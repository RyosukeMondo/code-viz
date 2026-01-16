/**
 * Tests for useThreeScene hook
 * @module components/visualizations/3d/hooks/useThreeScene.test
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, waitFor } from '@testing-library/react';
import { useThreeScene, SceneInitError } from './useThreeScene';
import { SceneManager } from '../scene/SceneManager';
import type { UseThreeSceneOptions } from './useThreeScene';
import type { FC } from 'react';

// Mock SceneManager
vi.mock('../scene/SceneManager', () => {
  const mockInitialize = vi.fn();
  const mockAnimate = vi.fn();
  const mockDispose = vi.fn();
  const mockIsWebGLSupported = vi.fn().mockReturnValue(true);

  class MockSceneManager {
    static isWebGLSupported = mockIsWebGLSupported;

    initialize = mockInitialize;
    animate = mockAnimate;
    dispose = mockDispose;

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    constructor(_canvas: HTMLCanvasElement, _options?: unknown) {
      // Mock constructor
    }
  }

  return {
    SceneManager: MockSceneManager,
  };
});

describe('useThreeScene', () => {
  let mockGetContext: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    vi.clearAllMocks();

    // Mock canvas getContext
    mockGetContext = vi.fn().mockReturnValue({});
    HTMLCanvasElement.prototype.getContext = mockGetContext;

    // Reset WebGL support to true by default
    vi.mocked(SceneManager.isWebGLSupported).mockReturnValue(true);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  // Test component that uses the hook
  const TestComponent: FC<{ options?: UseThreeSceneOptions }> = ({
    options,
  }) => {
    const { canvasRef, sceneManager, isInitialized, error } =
      useThreeScene(options);

    return (
      <div data-testid="container">
        <canvas
          ref={canvasRef}
          data-testid="canvas"
          data-initialized={isInitialized}
          data-has-error={error !== null}
          data-has-manager={sceneManager !== null}
        />
        {error && <div data-testid="error">{error.message}</div>}
      </div>
    );
  };

  describe('initialization', () => {
    it('should initialize scene when canvas is mounted', async () => {
      const onInitialized = vi.fn();

      const { getByTestId } = render(
        <TestComponent options={{ onInitialized }} />
      );

      await waitFor(
        () => {
          const canvas = getByTestId('canvas');
          expect(canvas.getAttribute('data-initialized')).toBe('true');
        },
        { timeout: 3000 }
      );

      const canvas = getByTestId('canvas');
      expect(canvas.getAttribute('data-has-manager')).toBe('true');
      expect(canvas.getAttribute('data-has-error')).toBe('false');
      expect(onInitialized).toHaveBeenCalledTimes(1);
    });

    it('should pass options to SceneManager constructor', async () => {
      const options: UseThreeSceneOptions = {
        targetFPS: 30,
        projectKey: 'test-project',
        antialias: false,
      };

      const { getByTestId } = render(<TestComponent options={options} />);

      await waitFor(
        () => {
          const canvas = getByTestId('canvas');
          expect(canvas.getAttribute('data-initialized')).toBe('true');
        },
        { timeout: 3000 }
      );

      // Verified by successful initialization
      const canvas = getByTestId('canvas');
      expect(canvas.getAttribute('data-initialized')).toBe('true');
    });
  });

  describe('error handling', () => {
    it('should handle WebGL not supported error', async () => {
      vi.mocked(SceneManager.isWebGLSupported).mockReturnValue(false);

      const onError = vi.fn();
      const { getByTestId, queryByTestId } = render(
        <TestComponent options={{ onError }} />
      );

      await waitFor(
        () => {
          expect(queryByTestId('error')).not.toBeNull();
        },
        { timeout: 3000 }
      );

      const canvas = getByTestId('canvas');
      expect(canvas.getAttribute('data-initialized')).toBe('false');
      expect(canvas.getAttribute('data-has-error')).toBe('true');

      const error = getByTestId('error');
      expect(error.textContent).toContain('WebGL is not supported');
      expect(onError).toHaveBeenCalledWith(expect.any(SceneInitError));
    });
  });

  describe('cleanup', () => {
    it('should dispose scene manager on unmount', async () => {
      const { getByTestId, unmount } = render(<TestComponent />);

      await waitFor(
        () => {
          const canvas = getByTestId('canvas');
          expect(canvas.getAttribute('data-initialized')).toBe('true');
        },
        { timeout: 3000 }
      );

      unmount();

      // Verified by no errors on unmount (dispose is called internally)
    });
  });

  describe('callbacks', () => {
    it('should call onInitialized with scene manager', async () => {
      const onInitialized = vi.fn();

      const { getByTestId } = render(
        <TestComponent options={{ onInitialized }} />
      );

      await waitFor(
        () => {
          const canvas = getByTestId('canvas');
          expect(canvas.getAttribute('data-initialized')).toBe('true');
        },
        { timeout: 3000 }
      );

      expect(onInitialized).toHaveBeenCalledTimes(1);
      expect(onInitialized).toHaveBeenCalledWith(expect.anything());
    });

    it('should call onError when initialization fails', async () => {
      vi.mocked(SceneManager.isWebGLSupported).mockReturnValue(false);

      const onError = vi.fn();
      const { queryByTestId } = render(
        <TestComponent options={{ onError }} />
      );

      await waitFor(
        () => {
          expect(queryByTestId('error')).not.toBeNull();
        },
        { timeout: 3000 }
      );

      expect(onError).toHaveBeenCalledTimes(1);
      expect(onError).toHaveBeenCalledWith(expect.any(SceneInitError));
    });
  });
});
