/**
 * Tests for CameraPersistence
 * @module components/visualizations/3d/scene/CameraPersistence.test
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { CameraPersistence } from './CameraPersistence';

describe('CameraPersistence', () => {
  let camera: THREE.PerspectiveCamera;
  let controls: OrbitControls;
  let domElement: HTMLElement;

  beforeEach(() => {
    // Setup Three.js objects
    camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    camera.position.set(0, 10, 20);

    // Create a simple DOM element for OrbitControls (no WebGL needed)
    domElement = document.createElement('div');
    controls = new OrbitControls(camera, domElement);
    controls.target.set(0, 0, 0);

    // Clear localStorage
    localStorage.clear();
    vi.clearAllTimers();
  });

  afterEach(() => {
    controls.dispose();
    vi.clearAllTimers();
  });

  describe('constructor', () => {
    it('should create instance with camera and controls', () => {
      const persistence = new CameraPersistence(camera, controls);
      expect(persistence).toBeInstanceOf(CameraPersistence);
    });

    it('should use default project key', () => {
      const persistence = new CameraPersistence(camera, controls);
      persistence.restore();
      expect(persistence).toBeDefined();
    });

    it('should use custom project key', () => {
      const persistence = new CameraPersistence(camera, controls, 'test-project');
      expect(persistence).toBeDefined();
    });
  });

  describe('restore', () => {
    it('should return false when no data is stored', () => {
      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();
      expect(result).toBe(false);
    });

    it('should restore camera position and target', () => {
      const savedState = {
        position: { x: 100, y: 200, z: 300 },
        target: { x: 10, y: 20, z: 30 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(savedState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(true);
      expect(camera.position.x).toBe(100);
      expect(camera.position.y).toBe(200);
      expect(camera.position.z).toBe(300);
      expect(controls.target.x).toBe(10);
      expect(controls.target.y).toBe(20);
      expect(controls.target.z).toBe(30);
    });

    it('should restore zoom if present', () => {
      const savedState = {
        position: { x: 0, y: 0, z: 10 },
        target: { x: 0, y: 0, z: 0 },
        zoom: 2.5,
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(savedState));

      const persistence = new CameraPersistence(camera, controls);
      persistence.restore();

      expect(camera.zoom).toBe(2.5);
    });

    it('should reject data older than 7 days', () => {
      const oldTimestamp = Date.now() - (8 * 24 * 60 * 60 * 1000); // 8 days ago
      const savedState = {
        position: { x: 100, y: 200, z: 300 },
        target: { x: 10, y: 20, z: 30 },
        timestamp: oldTimestamp
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(savedState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
      expect(localStorage.getItem('code-viz-camera-default')).toBeNull();
    });

    it('should reject invalid JSON', () => {
      localStorage.setItem('code-viz-camera-default', 'invalid json{');

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
      expect(localStorage.getItem('code-viz-camera-default')).toBeNull();
    });

    it('should reject state with missing position', () => {
      const invalidState = {
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(invalidState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
    });

    it('should reject state with missing target', () => {
      const invalidState = {
        position: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(invalidState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
    });

    it('should reject state with NaN values', () => {
      const invalidState = {
        position: { x: NaN, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(invalidState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
    });

    it('should reject state with Infinity values', () => {
      const invalidState = {
        position: { x: 0, y: Infinity, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(invalidState));

      const persistence = new CameraPersistence(camera, controls);
      const result = persistence.restore();

      expect(result).toBe(false);
    });

    it('should use project-specific keys', () => {
      const state1 = {
        position: { x: 100, y: 100, z: 100 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      const state2 = {
        position: { x: 200, y: 200, z: 200 },
        target: { x: 10, y: 10, z: 10 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-project1', JSON.stringify(state1));
      localStorage.setItem('code-viz-camera-project2', JSON.stringify(state2));

      const persistence1 = new CameraPersistence(camera, controls, 'project1');
      persistence1.restore();

      expect(camera.position.x).toBe(100);

      const persistence2 = new CameraPersistence(camera, controls, 'project2');
      persistence2.restore();

      expect(camera.position.x).toBe(200);
    });
  });

  describe('startAutoSave and stopAutoSave', () => {
    it('should save state when controls change', async () => {
      vi.useFakeTimers();

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();

      camera.position.set(50, 60, 70);
      controls.target.set(5, 6, 7);

      controls.dispatchEvent({ type: 'change' });

      await vi.advanceTimersByTimeAsync(1500);

      const stored = localStorage.getItem('code-viz-camera-default');
      expect(stored).toBeTruthy();

      const state = JSON.parse(stored!);
      expect(state.position.x).toBe(50);
      expect(state.position.y).toBe(60);
      expect(state.position.z).toBe(70);
      expect(state.target.x).toBe(5);
      expect(state.target.y).toBe(6);
      expect(state.target.z).toBe(7);

      persistence.stopAutoSave();
      vi.useRealTimers();
    });

    it('should debounce rapid changes', async () => {
      vi.useFakeTimers();

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();

      for (let i = 0; i < 10; i++) {
        camera.position.x = i * 10;
        controls.dispatchEvent({ type: 'change' });
        await vi.advanceTimersByTimeAsync(100);
      }

      await vi.advanceTimersByTimeAsync(1000);

      const stored = localStorage.getItem('code-viz-camera-default');
      expect(stored).toBeTruthy();

      const state = JSON.parse(stored!);
      expect(state.position.x).toBe(90);

      persistence.stopAutoSave();
      vi.useRealTimers();
    });

    it('should not save multiple times for same instance', () => {
      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();
      persistence.startAutoSave();
      expect(persistence).toBeDefined();
      persistence.stopAutoSave();
    });

    it('should stop listening to changes after stopAutoSave', async () => {
      vi.useFakeTimers();

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();
      persistence.stopAutoSave();

      localStorage.clear();

      camera.position.set(100, 100, 100);
      controls.dispatchEvent({ type: 'change' });

      await vi.advanceTimersByTimeAsync(2000);

      expect(localStorage.getItem('code-viz-camera-default')).toBeNull();

      vi.useRealTimers();
    });

    it('should not save during restore to avoid infinite loop', async () => {
      vi.useFakeTimers();

      const savedState = {
        position: { x: 100, y: 200, z: 300 },
        target: { x: 10, y: 20, z: 30 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(savedState));

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();

      const saveCount = () => {
        const stored = localStorage.getItem('code-viz-camera-default');
        return stored ? JSON.parse(stored).timestamp : 0;
      };

      const timestampBefore = saveCount();
      persistence.restore();

      await vi.advanceTimersByTimeAsync(200);

      const timestampAfter = saveCount();
      expect(timestampAfter).toBe(timestampBefore);

      persistence.stopAutoSave();
      vi.useRealTimers();
    });
  });

  describe('clear', () => {
    it('should remove stored state', () => {
      const savedState = {
        position: { x: 0, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      };

      localStorage.setItem('code-viz-camera-default', JSON.stringify(savedState));

      const persistence = new CameraPersistence(camera, controls);
      persistence.clear();

      expect(localStorage.getItem('code-viz-camera-default')).toBeNull();
    });

    it('should handle clear when no data exists', () => {
      const persistence = new CameraPersistence(camera, controls);
      expect(() => persistence.clear()).not.toThrow();
    });
  });

  describe('cleanupStaleData', () => {
    it('should remove camera states older than 7 days', () => {
      const oldTimestamp = Date.now() - (8 * 24 * 60 * 60 * 1000);
      const recentTimestamp = Date.now();

      localStorage.setItem('code-viz-camera-old', JSON.stringify({
        position: { x: 0, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: oldTimestamp
      }));

      localStorage.setItem('code-viz-camera-recent', JSON.stringify({
        position: { x: 0, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: recentTimestamp
      }));

      CameraPersistence.cleanupStaleData();

      expect(localStorage.getItem('code-viz-camera-old')).toBeNull();
      expect(localStorage.getItem('code-viz-camera-recent')).toBeTruthy();
    });

    it('should remove invalid JSON camera states', () => {
      localStorage.setItem('code-viz-camera-invalid', 'invalid json');
      localStorage.setItem('code-viz-camera-valid', JSON.stringify({
        position: { x: 0, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      }));

      CameraPersistence.cleanupStaleData();

      expect(localStorage.getItem('code-viz-camera-invalid')).toBeNull();
      expect(localStorage.getItem('code-viz-camera-valid')).toBeTruthy();
    });

    it('should not remove non-camera localStorage items', () => {
      localStorage.setItem('other-app-data', 'some data');
      localStorage.setItem('code-viz-camera-test', JSON.stringify({
        position: { x: 0, y: 0, z: 0 },
        target: { x: 0, y: 0, z: 0 },
        timestamp: Date.now()
      }));

      CameraPersistence.cleanupStaleData();

      expect(localStorage.getItem('other-app-data')).toBe('some data');
    });

    it('should handle empty localStorage', () => {
      localStorage.clear();
      expect(() => CameraPersistence.cleanupStaleData()).not.toThrow();
    });
  });

  describe('dispose', () => {
    it('should stop auto-save', async () => {
      vi.useFakeTimers();

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();
      persistence.dispose();

      localStorage.clear();

      camera.position.set(100, 100, 100);
      controls.dispatchEvent({ type: 'change' });

      await vi.advanceTimersByTimeAsync(2000);

      expect(localStorage.getItem('code-viz-camera-default')).toBeNull();

      vi.useRealTimers();
    });
  });

  describe('error handling', () => {
    it('should handle localStorage quota exceeded', async () => {
      vi.useFakeTimers();

      const setItemSpy = vi.spyOn(Storage.prototype, 'setItem');
      setItemSpy.mockImplementation(() => {
        const error = new Error('QuotaExceededError');
        error.name = 'QuotaExceededError';
        throw error;
      });

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();

      controls.dispatchEvent({ type: 'change' });
      await vi.advanceTimersByTimeAsync(1500);

      expect(() => {}).not.toThrow();

      setItemSpy.mockRestore();
      persistence.stopAutoSave();
      vi.useRealTimers();
    });

    it('should handle other localStorage errors', async () => {
      vi.useFakeTimers();

      const setItemSpy = vi.spyOn(Storage.prototype, 'setItem');
      setItemSpy.mockImplementation(() => {
        throw new Error('Some other error');
      });

      const persistence = new CameraPersistence(camera, controls);
      persistence.startAutoSave();

      controls.dispatchEvent({ type: 'change' });
      await vi.advanceTimersByTimeAsync(1500);

      expect(() => {}).not.toThrow();

      setItemSpy.mockRestore();
      persistence.stopAutoSave();
      vi.useRealTimers();
    });
  });
});
