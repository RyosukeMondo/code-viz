/**
 * High-level hook for repository analysis execution
 *
 * This hook orchestrates calling the analysis API (Tauri IPC or HTTP REST)
 * and synchronizing results with the global analysis store. It automatically
 * detects whether to use Tauri or Web mode.
 *
 * @module hooks/useAnalysis
 */

import { useCallback, useState } from 'react';
import { useAnalysisStore } from '../store/analysisStore';
import { analyzeRepository } from '../api/client';
import type { TreeNode } from '../types/bindings';

/**
 * State and actions returned by useAnalysis hook
 */
interface UseAnalysisResult {
  /** Current analysis tree (null if not yet analyzed) */
  data: TreeNode | null;

  /** Loading state (true while analysis is running) */
  loading: boolean;

  /** Error message (null if no error) */
  error: string | null;

  /**
   * Execute analysis on the specified repository path
   * @param path - Absolute path to the repository root directory
   */
  analyze: (path: string) => Promise<void>;

  /**
   * Re-run the most recent analysis (useful for refreshing)
   * @throws Error if no previous path is available
   */
  refetch: () => Promise<void>;

  /** Clear all analysis state and reset to initial values */
  reset: () => void;
}

/**
 * Hook for executing repository analysis and managing state
 *
 * This hook provides a high-level interface for analyzing repositories.
 * It automatically syncs analysis results with the global Zustand store
 * and uses the appropriate backend (Tauri IPC or HTTP REST).
 *
 * @returns Analysis state and control functions
 */
export function useAnalysis(): UseAnalysisResult {
  // Get store state and actions
  const metrics = useAnalysisStore((state) => state.metrics);
  const setMetrics = useAnalysisStore((state) => state.setMetrics);
  const setLoading = useAnalysisStore((state) => state.setLoading);
  const setError = useAnalysisStore((state) => state.setError);
  const resetStore = useAnalysisStore((state) => state.reset);

  // Track the last analyzed path for refetch functionality
  const [lastPath, setLastPath] = useState<string | null>(null);

  /**
   * Execute analysis on the specified repository path
   */
  const analyze = useCallback(
    async (path: string) => {
      console.log('[useAnalysis] analyze() called with path:', path);

      // Validate path
      if (!path || typeof path !== 'string') {
        console.error('[useAnalysis] Invalid path:', path);
        setError('Invalid repository path');
        return;
      }

      // Store path for refetch
      setLastPath(path);

      // Update store loading state
      console.log('[useAnalysis] Setting loading state to true');
      setLoading(true);
      setError(null);

      try {
        console.log('[useAnalysis] Calling API client');
        // Generate request ID (optional)
        const requestId = typeof crypto !== 'undefined' && crypto.randomUUID
          ? crypto.randomUUID()
          : `req-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

        // Use unified API client (auto-detects Tauri vs Web)
        const data = await analyzeRepository(path, requestId);

        console.log('[useAnalysis] API returned data:', {
          hasData: !!data,
          name: data?.name,
          loc: data?.loc,
          childrenCount: data?.children?.length,
        });

        // Sync with store on success
        setMetrics(data);
        setLoading(false);
      } catch (error) {
        console.error('[useAnalysis] API call failed:', error);
        const errorMessage = error instanceof Error ? error.message : String(error);
        setError(errorMessage);
        setLoading(false);
      }
    },
    [setMetrics, setLoading, setError]
  );

  /**
   * Re-run the most recent analysis
   */
  const refetch = useCallback(async () => {
    if (!lastPath) {
      setError('No previous analysis to refetch');
      return;
    }

    await analyze(lastPath);
  }, [analyze, lastPath, setError]);

  /**
   * Reset all analysis state
   */
  const reset = useCallback(() => {
    resetStore();
    setLastPath(null);
  }, [resetStore]);

  return {
    data: metrics,
    loading: useAnalysisStore((state) => state.loading),
    error: useAnalysisStore((state) => state.error),
    analyze,
    refetch,
    reset,
  };
}
