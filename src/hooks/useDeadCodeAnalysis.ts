/**
 * Hook for dead code analysis execution
 *
 * This hook orchestrates calling the dead code analysis API (Tauri IPC or HTTP REST)
 * and synchronizing results with the global analysis store. It automatically
 * detects whether to use Tauri or Web mode.
 *
 * @module hooks/useDeadCodeAnalysis
 */

import { useCallback, useState } from 'react';
import { useAnalysisStore } from '../store/analysisStore';
import { analyzeDeadCode } from '../api/client';
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
   * @param minConfidence - Minimum confidence score (0-100) for dead code inclusion (default: 70)
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
 * It automatically syncs analysis results with the global Zustand store and uses
 * the appropriate backend (Tauri IPC or HTTP REST).
 *
 * @returns Dead code analysis state and control functions
 */
export function useDeadCodeAnalysis(): UseDeadCodeAnalysisResult {
  // Get store state and actions
  const deadCodeResults = useAnalysisStore((state) => state.deadCodeResults);
  const setDeadCodeResults = useAnalysisStore((state) => state.setDeadCodeResults);
  const setDeadCodeLoading = useAnalysisStore((state) => state.setDeadCodeLoading);
  const setDeadCodeError = useAnalysisStore((state) => state.setDeadCodeError);
  const resetStore = useAnalysisStore((state) => state.reset);

  // Track the last analyzed parameters for refetch functionality
  const [lastParams, setLastParams] = useState<{
    path: string;
    minConfidence: number;
  } | null>(null);

  /**
   * Execute dead code analysis
   */
  const analyze = useCallback(
    async (path: string, minConfidence: number = 70) => {
      console.log('[useDeadCodeAnalysis] analyze() called', { path, minConfidence });

      // Validate inputs
      if (!path || typeof path !== 'string') {
        console.error('[useDeadCodeAnalysis] Invalid path:', path);
        setDeadCodeError('Invalid repository path');
        return;
      }

      if (minConfidence < 0 || minConfidence > 100) {
        console.error('[useDeadCodeAnalysis] Invalid minConfidence:', minConfidence);
        setDeadCodeError('Confidence must be between 0 and 100');
        return;
      }

      // Store parameters for refetch
      setLastParams({ path, minConfidence });

      // Update store loading state
      setDeadCodeLoading(true);
      setDeadCodeError(null);

      try {
        console.log('[useDeadCodeAnalysis] Calling API client');
        // Generate request ID (optional)
        const requestId = typeof crypto !== 'undefined' && crypto.randomUUID
          ? crypto.randomUUID()
          : `req-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

        // Use unified API client (auto-detects Tauri vs Web)
        const results = await analyzeDeadCode(path, minConfidence, requestId);

        console.log('[useDeadCodeAnalysis] API returned results:', {
          hasResults: !!results,
          deadFunctions: results?.summary?.deadFunctions,
          filesCount: results?.files?.length,
        });

        // Sync with store on success
        setDeadCodeResults(results);
        setDeadCodeLoading(false);
      } catch (error) {
        console.error('[useDeadCodeAnalysis] API call failed:', error);
        const errorMessage = error instanceof Error ? error.message : String(error);
        setDeadCodeError(errorMessage);
        setDeadCodeLoading(false);
      }
    },
    [setDeadCodeResults, setDeadCodeLoading, setDeadCodeError]
  );

  /**
   * Re-run the most recent analysis
   */
  const refetch = useCallback(async () => {
    if (!lastParams) {
      setDeadCodeError('No previous analysis to refetch');
      return;
    }

    await analyze(lastParams.path, lastParams.minConfidence);
  }, [analyze, lastParams, setDeadCodeError]);

  /**
   * Reset all dead code analysis state
   */
  const reset = useCallback(() => {
    resetStore();
    setLastParams(null);
  }, [resetStore]);

  return {
    results: deadCodeResults,
    loading: useAnalysisStore((state) => state.deadCodeLoading),
    error: useAnalysisStore((state) => state.deadCodeError),
    analyze,
    refetch,
    reset,
  };
}
