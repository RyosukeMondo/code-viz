/**
 * Hook for dead code analysis execution
 *
 * This hook orchestrates calling the analyze_dead_code Tauri command
 * and synchronizing results with the global analysis store. It provides
 * a simple interface for triggering dead code analysis and tracking its state.
 *
 * @module hooks/useDeadCodeAnalysis
 */

import { useCallback, useRef } from 'react';
import { useTauriCommand } from './useTauriCommand';
import { useAnalysisStore } from '../store/analysisStore';
import type { DeadCodeResult } from '../types/bindings';

/**
 * State and actions returned by useDeadCodeAnalysis hook
 */
interface UseDeadCodeAnalysisResult {
  /** Current dead code analysis results (null if not yet analyzed) */
  results: DeadCodeResult | null;

  /** Loading state (true while analysis is running) */
  loading: boolean;

  /** Error message (null if no error) */
  error: string | null;

  /**
   * Execute dead code analysis on the specified repository path
   * @param path - Absolute path to the repository root directory
   * @param minConfidence - Minimum confidence score (0-100) for dead code inclusion (default: 80)
   */
  analyze: (path: string, minConfidence?: number) => Promise<void>;

  /**
   * Re-run the most recent analysis with the same parameters
   * @throws Error if no previous analysis parameters are available
   */
  refetch: () => Promise<void>;

  /** Clear all dead code analysis state and reset to initial values */
  reset: () => void;
}

/**
 * Hook for executing dead code analysis and managing state
 *
 * This hook provides a high-level interface for analyzing dead code in repositories.
 * It automatically syncs analysis results with the global Zustand store and provides
 * convenient methods for triggering and refreshing analysis. The hook handles cleanup
 * on unmount and concurrent request cancellation.
 *
 * @returns Dead code analysis state and control functions
 *
 * @example
 * ```typescript
 * function DeadCodePanel() {
 *   const { results, loading, error, analyze } = useDeadCodeAnalysis();
 *   const [path, setPath] = useState('/path/to/repo');
 *
 *   const handleAnalyze = () => {
 *     analyze(path, 80); // Analyze with 80% minimum confidence
 *   };
 *
 *   if (loading) return <div>Analyzing dead code...</div>;
 *   if (error) return <div>Error: {error}</div>;
 *   if (!results) return <div>Click analyze to start</div>;
 *
 *   return <div>Dead functions: {results.summary.deadFunctions}</div>;
 * }
 * ```
 *
 * @example
 * ```typescript
 * // With refetch for auto-refresh
 * function DeadCodeViewWithRefresh() {
 *   const { results, analyze, refetch } = useDeadCodeAnalysis();
 *
 *   useEffect(() => {
 *     if (results) {
 *       const interval = setInterval(() => refetch(), 60000); // Refresh every minute
 *       return () => clearInterval(interval);
 *     }
 *   }, [results, refetch]);
 *
 *   return <DeadCodeList results={results} />;
 * }
 * ```
 */
export function useDeadCodeAnalysis(): UseDeadCodeAnalysisResult {
  // Get store state and actions
  const deadCodeResults = useAnalysisStore((state) => state.deadCodeResults);
  const setDeadCodeResults = useAnalysisStore((state) => state.setDeadCodeResults);
  const setDeadCodeLoading = useAnalysisStore((state) => state.setDeadCodeLoading);
  const setDeadCodeError = useAnalysisStore((state) => state.setDeadCodeError);
  const resetDeadCode = useAnalysisStore((state) => state.resetDeadCode);

  // Track the last analyzed path and confidence for refetch functionality
  const lastParamsRef = useRef<{ path: string; minConfidence: number } | null>(null);

  // Set up Tauri command hook with callbacks
  const { execute } = useTauriCommand<DeadCodeResult>('analyze_dead_code_command', {
    onSuccess: (data) => {
      console.log('[useDeadCodeAnalysis] Analysis succeeded, syncing with store');
      // Sync with store on success
      setDeadCodeResults(data as DeadCodeResult);
      setDeadCodeLoading(false);
    },
    onError: (errorMessage) => {
      console.error('[useDeadCodeAnalysis] Analysis failed:', errorMessage);
      // Sync error state with store
      setDeadCodeError(errorMessage);
      setDeadCodeLoading(false);
    },
  });

  /**
   * Execute dead code analysis on the specified repository path
   */
  const analyze = useCallback(
    async (path: string, minConfidence: number = 80) => {
      console.log('[useDeadCodeAnalysis] analyze() called with path:', path, 'minConfidence:', minConfidence);

      // Validate path
      if (!path || typeof path !== 'string') {
        console.error('[useDeadCodeAnalysis] Invalid path:', path);
        setDeadCodeError('Invalid repository path');
        return;
      }

      // Validate minConfidence
      if (minConfidence < 0 || minConfidence > 100) {
        console.error('[useDeadCodeAnalysis] Invalid minConfidence:', minConfidence);
        setDeadCodeError('Confidence score must be between 0 and 100');
        return;
      }

      // Store parameters for refetch
      lastParamsRef.current = { path, minConfidence };

      // Update store loading state
      console.log('[useDeadCodeAnalysis] Setting loading state to true');
      setDeadCodeLoading(true);

      // Execute command (callbacks will handle success/error)
      console.log('[useDeadCodeAnalysis] Calling execute with args:', { path, min_confidence: minConfidence });
      try {
        await execute({ path, min_confidence: minConfidence });
        console.log('[useDeadCodeAnalysis] execute() completed');
      } catch (error) {
        console.error('[useDeadCodeAnalysis] execute() threw error:', error);
        setDeadCodeError(error instanceof Error ? error.message : String(error));
        setDeadCodeLoading(false);
      }
    },
    [execute, setDeadCodeLoading, setDeadCodeError]
  );

  /**
   * Re-run the most recent analysis with the same parameters
   */
  const refetch = useCallback(async () => {
    const lastParams = lastParamsRef.current;

    if (!lastParams) {
      setDeadCodeError('No previous analysis to refetch');
      return;
    }

    await analyze(lastParams.path, lastParams.minConfidence);
  }, [analyze, setDeadCodeError]);

  /**
   * Reset all dead code analysis state
   */
  const reset = useCallback(() => {
    resetDeadCode();
    lastParamsRef.current = null;
  }, [resetDeadCode]);

  return {
    results: deadCodeResults,
    loading: useAnalysisStore((state) => state.deadCodeLoading),
    error: useAnalysisStore((state) => state.deadCodeError),
    analyze,
    refetch,
    reset,
  };
}
