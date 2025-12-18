/**
 * Unit tests for useTauriCommand hook
 *
 * Tests the generic Tauri command hook with mocked invoke function
 * covering loading states, error handling, and cleanup.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, waitFor, act } from '@testing-library/react';
import { useTauriCommand } from './useTauriCommand';

// Mock the Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('useTauriCommand', () => {
  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();
    // Mock crypto.randomUUID
    vi.stubGlobal('crypto', {
      randomUUID: vi.fn(() => 'test-uuid-123'),
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  describe('Initial state', () => {
    it('should initialize with null data, loading false, and no error', () => {
      const { result } = renderHook(() => useTauriCommand('test_command'));

      expect(result.current.data).toBeNull();
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.requestId).toBeNull();
      expect(result.current.execute).toBeInstanceOf(Function);
      expect(result.current.reset).toBeInstanceOf(Function);
    });
  });

  describe('execute function', () => {
    it('should set loading to true when command is executing', async () => {
      vi.mocked(invoke).mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve('result'), 100))
      );

      const { result } = renderHook(() => useTauriCommand('test_command'));

      result.current.execute('arg1');

      // Should be loading immediately after execute
      await waitFor(() => {
        expect(result.current.loading).toBe(true);
      });
    });

    it('should set data and clear loading on successful command execution', async () => {
      const mockData = { id: '1', name: 'test' };
      vi.mocked(invoke).mockResolvedValue(mockData);

      const { result } = renderHook(() => useTauriCommand<typeof mockData>('test_command'));

      await result.current.execute('arg1');

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
        expect(result.current.data).toEqual(mockData);
        expect(result.current.error).toBeNull();
      });
    });

    it('should set error and clear loading on failed command execution', async () => {
      const errorMessage = 'Command failed';
      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage));

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute('arg1');

      await waitFor(() => {
        expect(result.current.loading).toBe(false);
        expect(result.current.error).toBe(errorMessage);
        expect(result.current.data).toBeNull();
      });
    });

    it('should generate and set a request ID', async () => {
      vi.mocked(invoke).mockResolvedValue('success');

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute();

      await waitFor(() => {
        expect(result.current.requestId).toBe('test-uuid-123');
      });
    });

    it('should pass single argument correctly to invoke', async () => {
      vi.mocked(invoke).mockResolvedValue('success');

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute('/test/path');

      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('test_command', {
          value: '/test/path',
        });
      });
    });

    it('should pass object argument correctly to invoke', async () => {
      vi.mocked(invoke).mockResolvedValue('success');

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute({ path: '/test/path', recursive: true });

      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('test_command', {
          path: '/test/path',
          recursive: true,
        });
      });
    });

    it('should pass undefined when no arguments provided', async () => {
      vi.mocked(invoke).mockResolvedValue('success');

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute();

      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('test_command', undefined);
      });
    });

    it('should handle string error types', async () => {
      vi.mocked(invoke).mockRejectedValue('String error message');

      const { result } = renderHook(() => useTauriCommand('test_command'));

      await result.current.execute();

      await waitFor(() => {
        expect(result.current.error).toBe('String error message');
      });
    });
  });

  describe('Callbacks', () => {
    it('should call onSuccess callback when command succeeds', async () => {
      const mockData = { success: true };
      const onSuccess = vi.fn();

      vi.mocked(invoke).mockResolvedValue(mockData);

      const { result } = renderHook(() =>
        useTauriCommand('test_command', { onSuccess })
      );

      await result.current.execute();

      await waitFor(() => {
        expect(onSuccess).toHaveBeenCalledWith(mockData);
      });
    });

    it('should call onError callback when command fails', async () => {
      const errorMessage = 'Test error';
      const onError = vi.fn();

      vi.mocked(invoke).mockRejectedValue(new Error(errorMessage));

      const { result } = renderHook(() =>
        useTauriCommand('test_command', { onError })
      );

      await result.current.execute();

      await waitFor(() => {
        expect(onError).toHaveBeenCalledWith(errorMessage);
      });
    });
  });

  describe('Immediate execution', () => {
    it('should execute immediately on mount when immediate is true', async () => {
      vi.mocked(invoke).mockResolvedValue('immediate result');

      const { result } = renderHook(() =>
        useTauriCommand('test_command', {
          immediate: true,
          immediateArgs: ['/test/path'],
        })
      );

      await waitFor(() => {
        expect(invoke).toHaveBeenCalled();
        expect(result.current.data).toBe('immediate result');
      });
    });

    it('should not execute immediately when immediate is false', () => {
      vi.mocked(invoke).mockResolvedValue('result');

      renderHook(() =>
        useTauriCommand('test_command', {
          immediate: false,
        })
      );

      expect(invoke).not.toHaveBeenCalled();
    });
  });

  describe('reset function', () => {
    it('should reset all state to initial values', async () => {
      const mockData = { id: '1' };
      vi.mocked(invoke).mockResolvedValue(mockData);

      const { result } = renderHook(() => useTauriCommand('test_command'));

      // Execute command to populate state
      await act(async () => {
        await result.current.execute();
      });

      await waitFor(() => {
        expect(result.current.data).toEqual(mockData);
      });

      // Reset state
      act(() => {
        result.current.reset();
      });

      expect(result.current.data).toBeNull();
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.requestId).toBeNull();
    });
  });

  describe('Cleanup', () => {
    it('should not update state after unmount', async () => {
      let resolveInvoke: (value: string) => void;
      const invokePromise = new Promise<string>((resolve) => {
        resolveInvoke = resolve;
      });

      vi.mocked(invoke).mockReturnValue(invokePromise);

      const { result, unmount } = renderHook(() =>
        useTauriCommand('test_command')
      );

      result.current.execute();

      // Wait for loading to be true
      await waitFor(() => {
        expect(result.current.loading).toBe(true);
      });

      // Unmount before the promise resolves
      unmount();

      // Resolve the promise after unmount
      resolveInvoke!('late result');

      // Give it a moment to potentially update (which it shouldn't)
      await new Promise((resolve) => setTimeout(resolve, 50));

      // State should remain as it was before unmount (loading: true)
      expect(result.current.loading).toBe(true);
    });

    it('should cancel ongoing requests on unmount', async () => {
      const abortSpy = vi.spyOn(AbortController.prototype, 'abort');

      const { result, unmount } = renderHook(() =>
        useTauriCommand('test_command')
      );

      result.current.execute();

      unmount();

      expect(abortSpy).toHaveBeenCalled();
    });
  });

  describe('Concurrent requests', () => {
    it('should cancel previous request when new request starts', async () => {
      let resolveFirst: (value: string) => void;
      let resolveSecond: (value: string) => void;

      const firstPromise = new Promise<string>((resolve) => {
        resolveFirst = resolve;
      });

      const secondPromise = new Promise<string>((resolve) => {
        resolveSecond = resolve;
      });

      vi.mocked(invoke)
        .mockReturnValueOnce(firstPromise)
        .mockReturnValueOnce(secondPromise);

      const { result } = renderHook(() => useTauriCommand('test_command'));

      // Start first request
      result.current.execute('first');

      await waitFor(() => {
        expect(result.current.loading).toBe(true);
      });

      // Start second request before first completes
      result.current.execute('second');

      // Resolve first request (should be ignored)
      resolveFirst!('first result');

      // Resolve second request
      resolveSecond!('second result');

      await waitFor(() => {
        expect(result.current.data).toBe('second result');
        expect(result.current.loading).toBe(false);
      });
    });
  });

  describe('TypeScript generics', () => {
    it('should infer correct type for data', async () => {
      interface TestData {
        id: string;
        value: number;
      }

      const mockData: TestData = { id: '123', value: 42 };
      vi.mocked(invoke).mockResolvedValue(mockData);

      const { result } = renderHook(() => useTauriCommand<TestData>('test_command'));

      await result.current.execute();

      await waitFor(() => {
        // TypeScript should infer data as TestData | null
        const data = result.current.data;
        if (data) {
          expect(data.id).toBe('123');
          expect(data.value).toBe(42);
        }
      });
    });
  });
});
