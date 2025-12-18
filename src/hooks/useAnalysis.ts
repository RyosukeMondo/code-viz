/**
 * High-level hook for repository analysis execution
 *
 * This hook orchestrates calling the analyze_repository Tauri command
 * and synchronizing results with the global analysis store. It provides
 * a simple interface for triggering analysis and tracking its state.
 *
 * @module hooks/useAnalysis
 */

import { useCallback } from 'react';
import { useTauriCommand } from './useTauriCommand';
import { useAnalysisStore } from '../store/analysisStore';
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
 * and provides convenient methods for triggering and refreshing analysis.
 *
 * @returns Analysis state and control functions
 *
 * @example
 * ```typescript
 * function AnalysisView() {
 *   const { data, loading, error, analyze } = useAnalysis();
 *   const [path, setPath] = useState('/path/to/repo');
 *
 *   const handleAnalyze = () => {
 *     analyze(path);
 *   };
 *
 *   if (loading) return <div>Analyzing repository...</div>;
 *   if (error) return <div>Error: {error}</div>;
 *   if (!data) return <div>Click analyze to start</div>;
 *
 *   return <div>Total LOC: {data.loc}</div>;
 * }
 * ```
 *
 * @example
 * ```typescript
 * // With refetch for auto-refresh
 * function AnalysisViewWithRefresh() {
 *   const { data, loading, analyze, refetch } = useAnalysis();
 *
 *   useEffect(() => {
 *     const interval = setInterval(() => {
 *       if (data) refetch();
 *     }, 60000); // Refresh every minute
 *
 *     return () => clearInterval(interval);
 *   }, [data, refetch]);
 *
 *   return <Treemap data={data} />;
 * }
 * ```
 */
export function useAnalysis(): UseAnalysisResult {
  // Get store state and actions
  const metrics = useAnalysisStore((state) => state.metrics);
  const setMetrics = useAnalysisStore((state) => state.setMetrics);
  const setLoading = useAnalysisStore((state) => state.setLoading);
  const setError = useAnalysisStore((state) => state.setError);
  const resetStore = useAnalysisStore((state) => state.reset);

  // Track the last analyzed path for refetch functionality
  const lastPathRef = useCallback(() => {
    let storedPath: string | null = null;
    return {
      get: () => storedPath,
      set: (path: string) => {
        storedPath = path;
      },
    };
  }, [])();

  // Set up Tauri command hook with callbacks
  const { execute } = useTauriCommand<TreeNode>('analyze_repository', {
    onSuccess: (data) => {
      // Sync with store on success
      setMetrics(data as TreeNode);
      setLoading(false);
    },
    onError: (errorMessage) => {
      // Sync error state with store
      setError(errorMessage);
      setLoading(false);
    },
  });

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
      lastPathRef.set(path);

      // Update store loading state
      console.log('[useAnalysis] Setting loading state to true');
      setLoading(true);

      // Execute command (callbacks will handle success/error)
      console.log('[useAnalysis] Calling execute with args:', { path });
      try {
        await execute({ path });
        console.log('[useAnalysis] execute() completed');
      } catch (error) {
        console.error('[useAnalysis] execute() threw error:', error);
        setError(error instanceof Error ? error.message : String(error));
        setLoading(false);
      }
    },
    [execute, setLoading, setError, lastPathRef]
  );

  /**
   * Re-run the most recent analysis
   */
  const refetch = useCallback(async () => {
    const lastPath = lastPathRef.get();

    if (!lastPath) {
      setError('No previous analysis to refetch');
      return;
    }

    await analyze(lastPath);
  }, [analyze, setError, lastPathRef]);

  /**
   * Reset all analysis state
   */
  const reset = useCallback(() => {
    resetStore();
    lastPathRef.set('');
  }, [resetStore, lastPathRef]);

  return {
    data: metrics,
    loading: useAnalysisStore((state) => state.loading),
    error: useAnalysisStore((state) => state.error),
    analyze,
    refetch,
    reset,
  };
}
