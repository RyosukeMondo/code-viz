/**
 * Unit tests for useAnalysis hook
 *
 * Tests the high-level analysis hook with mocked Tauri command
 * and Zustand store integration.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, waitFor, act } from '@testing-library/react';
import { useAnalysis } from './useAnalysis';
import { useAnalysisStore } from '../store/analysisStore';
import type { TreeNode } from '../types/bindings';

// Mock the API client
vi.mock('../api/client', () => ({
  analyzeRepository: vi.fn(),
}));

import { analyzeRepository } from '../api/client';

describe('useAnalysis', () => {
  // Mock analyze repository function
  let mockAnalyzeRepository: ReturnType<typeof vi.fn>;

  // Mock tree data
  const mockTreeNode: TreeNode = {
    id: 'root',
    name: 'project',
    path: '/test/project',
    loc: 1000,
    complexity: 50,
    type: 'Directory',
    children: [
      {
        id: 'file1',
        name: 'main.rs',
        path: '/test/project/main.rs',
        loc: 500,
        complexity: 25,
        type: 'File',
        children: null,
        last_modified: '2024-01-01T00:00:00Z',
      },
    ],
    last_modified: '2024-01-01T00:00:00Z',
  };

  beforeEach(() => {
    // Reset store before each test
    useAnalysisStore.getState().reset();

    // Create mock analyze repository function
    mockAnalyzeRepository = vi.mocked(analyzeRepository);
    mockAnalyzeRepository.mockResolvedValue(mockTreeNode);

    // Mock crypto.randomUUID
    vi.stubGlobal('crypto', {
      randomUUID: vi.fn(() => 'test-uuid-123'),
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
    vi.unstubAllGlobals();
  });

  describe('Initial state', () => {
    it('should initialize with null data, loading false, and no error', () => {
      const { result } = renderHook(() => useAnalysis());

      expect(result.current.data).toBeNull();
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.analyze).toBeInstanceOf(Function);
      expect(result.current.refetch).toBeInstanceOf(Function);
      expect(result.current.reset).toBeInstanceOf(Function);
    });
  });

  describe('analyze function', () => {
    it('should set loading state when analysis starts', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Start analysis
      const analyzePromise = act(async () => {
        await result.current.analyze('/test/repo');
      });

      // Loading should be set during the analysis
      await waitFor(() => {
        expect(result.current.loading).toBe(true);
      });

      await analyzePromise;

      expect(mockAnalyzeRepository).toHaveBeenCalledWith('/test/repo', {}, expect.any(String));
    });

    it('should update store with metrics on successful analysis', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(result.current.data).toEqual(mockTreeNode);
        expect(result.current.loading).toBe(false);
        expect(result.current.error).toBeNull();
      });
    });

    it('should set error on failed analysis', async () => {
      const { result } = renderHook(() => useAnalysis());
      const errorMessage = 'Failed to analyze repository';

      // Make the mock reject
      mockAnalyzeRepository.mockRejectedValueOnce(new Error(errorMessage));

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(result.current.error).toBe(errorMessage);
        expect(result.current.loading).toBe(false);
        expect(result.current.data).toBeNull();
      });
    });

    it('should reject invalid path', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.analyze('');
      });

      expect(result.current.error).toBe('Invalid repository path');
      expect(mockAnalyzeRepository).not.toHaveBeenCalled();
    });

    it('should reject non-string path', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        await result.current.analyze(null as any);
      });

      expect(result.current.error).toBe('Invalid repository path');
      expect(mockAnalyzeRepository).not.toHaveBeenCalled();
    });

    it('should call execute with correct path', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.analyze('/test/path');
      });

      expect(mockAnalyzeRepository).toHaveBeenCalledWith('/test/path', {}, expect.any(String));
    });

    it('should clear drill-down path when new metrics are set', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Set some drill-down path first
      act(() => {
        useAnalysisStore.getState().setDrillDownPath(['dir1', 'dir2']);
      });

      expect(useAnalysisStore.getState().drillDownPath).toEqual(['dir1', 'dir2']);

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(useAnalysisStore.getState().drillDownPath).toEqual([]);
      });
    });

    it('should clear selected file when new metrics are set', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Set a selected file first
      const selectedFile: TreeNode = {
        id: 'file1',
        name: 'test.rs',
        path: '/test/test.rs',
        loc: 100,
        complexity: 10,
        type: 'File',
        children: null,
        last_modified: '2024-01-01T00:00:00Z',
      };

      act(() => {
        useAnalysisStore.getState().setSelectedFile(selectedFile);
      });

      expect(useAnalysisStore.getState().selectedFile).toEqual(selectedFile);

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(useAnalysisStore.getState().selectedFile).toBeNull();
      });
    });
  });

  describe('refetch function', () => {
    it('should re-run analysis with last used path', async () => {
      const { result } = renderHook(() => useAnalysis());

      // First analysis
      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      // Wait for the first call to complete
      await waitFor(() => {
        expect(mockAnalyzeRepository).toHaveBeenCalledTimes(1);
      });

      expect(mockAnalyzeRepository).toHaveBeenCalledWith('/test/repo', {}, expect.any(String));

      // Clear the mock call count for clarity
      mockAnalyzeRepository.mockClear();

      // Refetch should call analyzeRepository again with the same path
      await act(async () => {
        await result.current.refetch();
      });

      await waitFor(() => {
        expect(mockAnalyzeRepository).toHaveBeenCalledTimes(1);
      });

      expect(mockAnalyzeRepository).toHaveBeenCalledWith('/test/repo', {}, expect.any(String));
    });

    it('should set error when refetch called without previous analysis', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.refetch();
      });

      expect(result.current.error).toBe('No previous analysis to refetch');
      expect(mockAnalyzeRepository).not.toHaveBeenCalled();
    });

    it('should use most recent path for refetch', async () => {
      const { result } = renderHook(() => useAnalysis());

      // First analysis
      await act(async () => {
        await result.current.analyze('/test/repo1');
      });

      // Second analysis with different path
      await act(async () => {
        await result.current.analyze('/test/repo2');
      });

      // Refetch should use second path
      await act(async () => {
        await result.current.refetch();
      });

      expect(mockAnalyzeRepository).toHaveBeenCalledTimes(3);
      expect(mockAnalyzeRepository).toHaveBeenLastCalledWith('/test/repo2', {}, expect.any(String));
    });
  });

  describe('reset function', () => {
    it('should reset all state to initial values', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Set up some state
      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(result.current.data).toEqual(mockTreeNode);
      });

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.data).toBeNull();
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
    });

    it('should clear stored path after reset', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Analyze once
      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      // Reset
      act(() => {
        result.current.reset();
      });

      // Try to refetch - should fail because path was cleared
      await act(async () => {
        await result.current.refetch();
      });

      expect(result.current.error).toBe('No previous analysis to refetch');
    });
  });

  describe('Store synchronization', () => {
    it('should sync loading state with store', async () => {
      const { result } = renderHook(() => useAnalysis());

      // Start analysis
      const analyzePromise = act(async () => {
        await result.current.analyze('/test/repo');
      });

      // Check loading state during analysis
      await waitFor(() => {
        expect(useAnalysisStore.getState().loading).toBe(true);
        expect(result.current.loading).toBe(true);
      });

      await analyzePromise;
    });

    it('should sync metrics with store on success', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(useAnalysisStore.getState().metrics).toEqual(mockTreeNode);
        expect(result.current.data).toEqual(mockTreeNode);
      });
    });

    it('should sync error with store on failure', async () => {
      const { result } = renderHook(() => useAnalysis());
      const errorMessage = 'Analysis failed';

      // Make the mock reject
      mockAnalyzeRepository.mockRejectedValueOnce(new Error(errorMessage));

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(useAnalysisStore.getState().error).toBe(errorMessage);
        expect(result.current.error).toBe(errorMessage);
      });
    });

    it('should reflect store updates in hook state', () => {
      const { result } = renderHook(() => useAnalysis());

      // Update store directly
      act(() => {
        useAnalysisStore.getState().setMetrics(mockTreeNode);
      });

      // Hook should reflect the update
      expect(result.current.data).toEqual(mockTreeNode);
    });
  });

  describe('API client integration', () => {
    it('should call analyzeRepository from API client', async () => {
      const { result } = renderHook(() => useAnalysis());

      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      expect(analyzeRepository).toHaveBeenCalledWith('/test/repo', {}, expect.any(String));
    });

    it('should pass options to analyzeRepository', async () => {
      const { result } = renderHook(() => useAnalysis());
      const options = { includeAiCommits: true, includeCoverage: true };

      await act(async () => {
        await result.current.analyze('/test/repo', options);
      });

      expect(analyzeRepository).toHaveBeenCalledWith('/test/repo', options, expect.any(String));
    });
  });

  describe('Error handling', () => {
    it('should handle multiple errors gracefully', async () => {
      const { result } = renderHook(() => useAnalysis());

      // First error
      mockAnalyzeRepository.mockRejectedValueOnce(new Error('First error'));
      await act(async () => {
        await result.current.analyze('/test/repo1');
      });

      await waitFor(() => {
        expect(result.current.error).toBe('First error');
      });

      // Second error should replace first
      mockAnalyzeRepository.mockRejectedValueOnce(new Error('Second error'));
      await act(async () => {
        await result.current.analyze('/test/repo2');
      });

      await waitFor(() => {
        expect(result.current.error).toBe('Second error');
      });
    });

    it('should clear error on successful analysis', async () => {
      const { result } = renderHook(() => useAnalysis());

      // First analysis fails
      mockAnalyzeRepository.mockRejectedValueOnce(new Error('Analysis failed'));
      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(result.current.error).toBe('Analysis failed');
      });

      // Second analysis succeeds
      mockAnalyzeRepository.mockResolvedValueOnce(mockTreeNode);
      await act(async () => {
        await result.current.analyze('/test/repo');
      });

      await waitFor(() => {
        expect(result.current.error).toBeNull();
      });
    });
  });
});
