/**
 * useThreeScene - React hook for managing Three.js scene lifecycle
 * Handles initialization, animation loop, and cleanup
 * @module components/visualizations/3d/hooks/useThreeScene
 */

import { useRef, useEffect, useCallback, useState } from 'react';
import { SceneManager } from '../scene/SceneManager';
import type { SceneManagerOptions } from '../types';

/**
 * Scene initialization error type
 */
export class SceneInitError extends Error {
  constructor(message: string, public readonly cause?: Error) {
    super(message);
    this.name = 'SceneInitError';
  }
}

/**
 * Return type for useThreeScene hook
 */
export interface UseThreeSceneReturn {
  /** Canvas ref to attach to canvas element */
  canvasRef: React.RefObject<HTMLCanvasElement>;
  /** Scene manager instance (null if not initialized) */
  sceneManager: SceneManager | null;
  /** Whether scene is initialized */
  isInitialized: boolean;
  /** Initialization error if any */
  error: SceneInitError | null;
  /** Manually trigger reinitialization */
  reinitialize: () => void;
}

/**
 * Options for useThreeScene hook
 */
export interface UseThreeSceneOptions extends SceneManagerOptions {
  /** Callback when scene initializes successfully */
  onInitialized?: (sceneManager: SceneManager) => void;
  /** Callback when initialization fails */
  onError?: (error: SceneInitError) => void;
  /** Enable automatic resize handling (default: true) */
  autoResize?: boolean;
}

/**
 * Custom hook for managing Three.js scene lifecycle
 *
 * Features:
 * - Manages SceneManager initialization and cleanup
 * - Handles canvas ref properly
 * - Uses RAF for smooth animation loop
 * - Automatic window resize handling
 * - Proper cleanup to prevent memory leaks
 * - Error handling for WebGL support issues
 *
 * @example
 * ```tsx
 * function Voxel3DView() {
 *   const { canvasRef, sceneManager, isInitialized, error } = useThreeScene({
 *     targetFPS: 60,
 *     projectKey: 'my-project',
 *     onInitialized: (manager) => {
 *       console.log('Scene ready!');
 *       // Add objects to scene
 *     },
 *     onError: (err) => {
 *       console.error('Scene init failed:', err);
 *     }
 *   });
 *
 *   if (error) {
 *     return <div>WebGL Error: {error.message}</div>;
 *   }
 *
 *   return (
 *     <canvas
 *       ref={canvasRef}
 *       style={{ width: '100%', height: '100%' }}
 *     />
 *   );
 * }
 * ```
 */
export function useThreeScene(
  options: UseThreeSceneOptions = {}
): UseThreeSceneReturn {
  const {
    onInitialized,
    onError,
    ...sceneManagerOptions
  } = options;

  // Canvas ref
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // Scene manager ref - persists across re-renders
  const sceneManagerRef = useRef<SceneManager | null>(null);

  // Initialization state
  const [isInitialized, setIsInitialized] = useState(false);
  const [error, setError] = useState<SceneInitError | null>(null);

  // Track if component is mounted to prevent state updates after unmount
  const isMountedRef = useRef(true);

  // Initialization flag ref to prevent double initialization
  const initializingRef = useRef(false);

  // Store options in ref to avoid dependency issues
  const optionsRef = useRef(sceneManagerOptions);
  optionsRef.current = sceneManagerOptions;

  /**
   * Initialize the scene manager
   */
  const initialize = useCallback(() => {
    // Prevent double initialization
    if (initializingRef.current) {
      return;
    }

    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }

    initializingRef.current = true;

    try {
      // Check WebGL support before creating scene manager
      if (!SceneManager.isWebGLSupported()) {
        throw new SceneInitError(
          'WebGL is not supported in this browser. Please use a modern browser with WebGL support.'
        );
      }

      // Dispose existing scene manager if any
      if (sceneManagerRef.current) {
        sceneManagerRef.current.dispose();
        sceneManagerRef.current = null;
      }

      // Create and initialize scene manager
      const manager = new SceneManager(canvas, optionsRef.current);
      manager.initialize();
      sceneManagerRef.current = manager;

      // Start animation loop
      manager.animate();

      // Update state if still mounted
      if (isMountedRef.current) {
        setIsInitialized(true);
        setError(null);

        // Call success callback
        if (onInitialized) {
          onInitialized(manager);
        }
      }
    } catch (err) {
      const sceneError = err instanceof SceneInitError
        ? err
        : new SceneInitError(
            err instanceof Error ? err.message : 'Unknown initialization error',
            err instanceof Error ? err : undefined
          );

      if (isMountedRef.current) {
        setError(sceneError);
        setIsInitialized(false);

        // Call error callback
        if (onError) {
          onError(sceneError);
        }
      }

      console.error('Scene initialization failed:', sceneError);
    } finally {
      initializingRef.current = false;
    }
  }, [onInitialized, onError]);

  /**
   * Manually trigger reinitialization
   */
  const reinitialize = useCallback(() => {
    setIsInitialized(false);
    setError(null);
    initialize();
  }, [initialize]);

  /**
   * Initialize scene when canvas is available
   */
  useEffect(() => {
    if (canvasRef.current && !isInitialized && !error) {
      initialize();
    }
  }, [initialize, isInitialized, error]);

  /**
   * Cleanup on unmount
   */
  useEffect(() => {
    return () => {
      isMountedRef.current = false;

      // Dispose scene manager
      if (sceneManagerRef.current) {
        sceneManagerRef.current.dispose();
        sceneManagerRef.current = null;
      }
    };
  }, []);

  return {
    canvasRef,
    sceneManager: sceneManagerRef.current,
    isInitialized,
    error,
    reinitialize,
  };
}

export default useThreeScene;
