/**
 * CameraPersistence handles saving and restoring camera position and target
 * to localStorage for improved UX across sessions.
 *
 * Features:
 * - Debounced saves (max once per second) to avoid performance issues
 * - Project-specific keys to isolate different visualizations
 * - Validation of stored data to prevent crashes from corrupted data
 * - Automatic cleanup of stale data (>7 days old)
 *
 * @class CameraPersistence
 * @example
 * const persistence = new CameraPersistence(camera, controls, 'my-project');
 * persistence.restore(); // Restore saved position
 * persistence.startAutoSave(); // Begin auto-saving on changes
 */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import type { CameraState } from '../types';

/**
 * Extended camera state with zoom support
 */
interface ExtendedCameraState extends CameraState {
  zoom?: number;
}

export class CameraPersistence {
  private readonly camera: THREE.PerspectiveCamera;
  private readonly controls: OrbitControls;
  private readonly storageKey: string;
  private saveTimeout: NodeJS.Timeout | null = null;
  private readonly debounceDelay = 1000; // Save max once per second
  private readonly maxAge = 7 * 24 * 60 * 60 * 1000; // 7 days in milliseconds
  private changeHandler: (() => void) | null = null;
  private isRestoring = false;

  /**
   * Creates a CameraPersistence instance
   * @param camera - The camera to persist
   * @param controls - The orbit controls to listen to
   * @param projectKey - Unique key for this project/visualization
   */
  constructor(camera: THREE.PerspectiveCamera, controls: OrbitControls, projectKey = 'default') {
    this.camera = camera;
    this.controls = controls;
    this.storageKey = `code-viz-camera-${projectKey}`;
  }

  /**
   * Generates a storage key with timestamp for cleanup
   * @returns The storage key
   * @private
   */
  private getStorageKey(): string {
    return this.storageKey;
  }

  /**
   * Saves camera state to localStorage (debounced)
   * @private
   */
  private save(): void {
    // Prevent saving during restore to avoid infinite loops
    if (this.isRestoring) {
      return;
    }

    // Clear existing timeout
    if (this.saveTimeout) {
      clearTimeout(this.saveTimeout);
    }

    // Debounce: save after delay
    this.saveTimeout = setTimeout(() => {
      try {
        const state: ExtendedCameraState = {
          position: {
            x: this.camera.position.x,
            y: this.camera.position.y,
            z: this.camera.position.z
          },
          target: {
            x: this.controls.target.x,
            y: this.controls.target.y,
            z: this.controls.target.z
          },
          zoom: this.camera.zoom,
          timestamp: Date.now()
        };

        localStorage.setItem(this.getStorageKey(), JSON.stringify(state));
      } catch (error) {
        // Handle localStorage quota exceeded or other errors
        if (error instanceof Error && error.name === 'QuotaExceededError') {
          console.warn('CameraPersistence: localStorage quota exceeded, cannot save camera state');
        } else {
          console.error('CameraPersistence: Failed to save camera state:', error);
        }
      }
    }, this.debounceDelay);
  }

  /**
   * Restores camera state from localStorage
   * @returns True if state was restored, false otherwise
   */
  restore(): boolean {
    try {
      const stored = localStorage.getItem(this.getStorageKey());

      if (!stored) {
        return false;
      }

      const state = JSON.parse(stored) as ExtendedCameraState;

      // Validate state structure
      if (!this.validateState(state)) {
        console.warn('CameraPersistence: Invalid stored state, ignoring');
        localStorage.removeItem(this.getStorageKey());
        return false;
      }

      // Check if data is too old (>7 days)
      const age = Date.now() - state.timestamp;
      if (age > this.maxAge) {
        console.log('CameraPersistence: Stored state is too old, clearing');
        localStorage.removeItem(this.getStorageKey());
        return false;
      }

      // Set flag to prevent save during restore
      this.isRestoring = true;

      // Restore camera position
      this.camera.position.set(
        state.position.x,
        state.position.y,
        state.position.z
      );

      // Restore target
      this.controls.target.set(
        state.target.x,
        state.target.y,
        state.target.z
      );

      // Restore zoom if present
      if (state.zoom !== undefined) {
        this.camera.zoom = state.zoom;
        this.camera.updateProjectionMatrix();
      }

      // Update controls to apply changes
      this.controls.update();

      // Reset flag after a short delay to allow controls to settle
      setTimeout(() => {
        this.isRestoring = false;
      }, 100);

      console.log('CameraPersistence: Camera state restored');
      return true;
    } catch (error) {
      console.error('CameraPersistence: Failed to restore camera state:', error);
      // Clean up corrupted data
      try {
        localStorage.removeItem(this.getStorageKey());
      } catch {
        // Ignore cleanup errors
      }
      return false;
    }
  }

  /**
   * Validates the structure of stored camera state
   * @param state - The state to validate
   * @returns True if state is valid
   * @private
   */
  private validateState(state: unknown): state is ExtendedCameraState {
    if (!state || typeof state !== 'object') {
      return false;
    }

    const s = state as Record<string, unknown>;

    // Check required fields exist
    if (!s.position || !s.target || !s.timestamp) {
      return false;
    }

    const position = s.position as Record<string, unknown>;
    const target = s.target as Record<string, unknown>;

    // Validate position
    if (typeof position.x !== 'number' ||
        typeof position.y !== 'number' ||
        typeof position.z !== 'number') {
      return false;
    }

    // Validate target
    if (typeof target.x !== 'number' ||
        typeof target.y !== 'number' ||
        typeof target.z !== 'number') {
      return false;
    }

    // Validate timestamp
    if (typeof s.timestamp !== 'number' || s.timestamp <= 0) {
      return false;
    }

    // Check for NaN or Infinity values
    const values = [
      position.x, position.y, position.z,
      target.x, target.y, target.z
    ];

    if (values.some(v => !isFinite(v as number))) {
      return false;
    }

    return true;
  }

  /**
   * Starts auto-saving camera state on controls change
   */
  startAutoSave(): void {
    if (this.changeHandler) {
      // Already started
      return;
    }

    this.changeHandler = () => this.save();
    this.controls.addEventListener('change', this.changeHandler);
    console.log('CameraPersistence: Auto-save enabled');
  }

  /**
   * Stops auto-saving camera state
   */
  stopAutoSave(): void {
    if (this.changeHandler) {
      this.controls.removeEventListener('change', this.changeHandler);
      this.changeHandler = null;

      // Clear any pending save
      if (this.saveTimeout) {
        clearTimeout(this.saveTimeout);
        this.saveTimeout = null;
      }

      console.log('CameraPersistence: Auto-save disabled');
    }
  }

  /**
   * Clears stored camera state for this project
   */
  clear(): void {
    try {
      localStorage.removeItem(this.getStorageKey());
      console.log('CameraPersistence: Cleared stored state');
    } catch (error) {
      console.error('CameraPersistence: Failed to clear state:', error);
    }
  }

  /**
   * Cleans up all stale camera states across all projects
   * @static
   */
  static cleanupStaleData(): void {
    try {
      const maxAge = 7 * 24 * 60 * 60 * 1000; // 7 days
      const now = Date.now();
      const keysToRemove: string[] = [];

      // Scan localStorage for code-viz-camera-* keys
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key && key.startsWith('code-viz-camera-')) {
          try {
            const value = localStorage.getItem(key);
            if (value) {
              const state = JSON.parse(value) as ExtendedCameraState;

              if (state.timestamp && (now - state.timestamp) > maxAge) {
                keysToRemove.push(key);
              }
            }
          } catch {
            // Invalid JSON, mark for removal
            keysToRemove.push(key);
          }
        }
      }

      // Remove stale keys
      keysToRemove.forEach(key => {
        localStorage.removeItem(key);
      });

      if (keysToRemove.length > 0) {
        console.log(`CameraPersistence: Cleaned up ${keysToRemove.length} stale camera states`);
      }
    } catch (error) {
      console.error('CameraPersistence: Failed to cleanup stale data:', error);
    }
  }

  /**
   * Disposes of the persistence handler and stops auto-save
   */
  dispose(): void {
    this.stopAutoSave();
  }
}
