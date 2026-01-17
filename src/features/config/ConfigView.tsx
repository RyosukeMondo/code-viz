/**
 * ConfigView - Configuration and analysis component
 *
 * Provides interface for:
 * - Repository path input with file picker
 * - Analysis options configuration
 * - Analyze button to trigger repository analysis
 * - Loading states and error handling
 */

import { useState, useEffect } from 'react';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { ProgressBar } from '@/components/common/ProgressBar';
import { useAnalysis } from '@/hooks/useAnalysis';
import { useAnalysisActions } from '@/store/analysisStore';
import type { AnalysisOptions } from '@/types/bindings';

export function ConfigView() {
  // Local state for repository path input (load from localStorage if available)
  const [repoPath, setRepoPath] = useState<string>(() => {
    try {
      return localStorage.getItem('lastRepoPath') || '';
    } catch {
      return '';
    }
  });

  // Local state for analysis options (load from localStorage if available)
  const [analysisOptions, setAnalysisOptions] = useState<AnalysisOptions>(() => {
    try {
      const saved = localStorage.getItem('analysisOptions');
      const parsed = saved ? JSON.parse(saved) : {};
      return { ...parsed, enableDuplicates: false };
    } catch {
      return { enableDuplicates: false };
    }
  });

  // Analysis hook for executing repository analysis
  const { data, loading, error, analyze, reset } = useAnalysis();

  // Store actions to save data globally
  const { setMetrics, setError: setStoreError, setLoading: setStoreLoading } = useAnalysisActions();

  // Sync analysis results to global store
  useEffect(() => {
    if (data) {
      setMetrics(data);
    }
  }, [data, setMetrics]);

  // Sync loading state to global store
  useEffect(() => {
    setStoreLoading(loading);
  }, [loading, setStoreLoading]);

  // Sync error state to global store
  useEffect(() => {
    if (error) {
      setStoreError(error);
    }
  }, [error, setStoreError]);

  // Handle analyze button click
  const handleAnalyze = async () => {
    const path = repoPath.trim();
    if (!path) return;

    // Save to localStorage
    try {
      localStorage.setItem('lastRepoPath', path);
      localStorage.setItem('analysisOptions', JSON.stringify(analysisOptions));
    } catch (err) {
      console.warn('Failed to save to localStorage:', err);
    }

    await analyze(path, analysisOptions);
  };

  // Handle Enter key in path input
  const handlePathKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !loading && repoPath.trim()) {
      handleAnalyze();
    }
  };

  // Handle browse button click (Tauri only)
  const handleBrowse = async () => {
    const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;
    if (!isTauri) {
      alert('File picker is only available in desktop mode');
      return;
    }

    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Repository Directory',
      });

      if (selected) {
        setRepoPath(selected);
      }
    } catch (err) {
      console.error('Failed to open file picker:', err);
      alert('Failed to open file picker');
    }
  };

  // Handle reset button click
  const handleReset = () => {
    reset();
    setMetrics(null);
    setStoreError(null);
    setStoreLoading(false);
  };

  // Handle options change
  const handleOptionsChange = (newOptions: Partial<AnalysisOptions>) => {
    setAnalysisOptions((prev) => ({ ...prev, ...newOptions }));
  };

  return (
    <ErrorBoundary>
      <div className="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
        <div className="flex-1 overflow-auto">
          <div className="max-w-4xl mx-auto p-8">
            {/* Header */}
            <div className="mb-8">
              <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-2">
                Repository Analysis Configuration
              </h1>
              <p className="text-gray-600 dark:text-gray-400">
                Configure and analyze your codebase to visualize in 2D or 3D
              </p>
            </div>

            {/* Analysis Form */}
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-6">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Repository Path
              </h2>

              <div className="flex gap-2 mb-4">
                <input
                  type="text"
                  value={repoPath}
                  onChange={(e) => setRepoPath(e.target.value)}
                  onKeyDown={handlePathKeyDown}
                  placeholder="/path/to/repository"
                  disabled={loading}
                  className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg
                           bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100
                           placeholder-gray-400 dark:placeholder-gray-500
                           focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                           disabled:opacity-50 disabled:cursor-not-allowed"
                />
                <button
                  onClick={handleBrowse}
                  disabled={loading}
                  className="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300
                           border border-gray-300 dark:border-gray-600 rounded-lg
                           hover:bg-gray-200 dark:hover:bg-gray-600
                           focus:outline-none focus:ring-2 focus:ring-blue-500
                           disabled:opacity-50 disabled:cursor-not-allowed
                           transition-colors"
                  title="Browse for directory (desktop only)"
                >
                  Browse
                </button>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={handleAnalyze}
                  disabled={!repoPath.trim() || loading}
                  className="flex-1 px-6 py-3 bg-blue-600 text-white rounded-lg font-medium
                           hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                           dark:focus:ring-offset-gray-800
                           disabled:opacity-50 disabled:cursor-not-allowed
                           transition-colors"
                >
                  {loading ? 'Analyzing...' : 'Analyze Repository'}
                </button>
                {data && (
                  <button
                    onClick={handleReset}
                    disabled={loading}
                    className="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300
                             border border-gray-300 dark:border-gray-600 rounded-lg
                             hover:bg-gray-200 dark:hover:bg-gray-600
                             focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                             dark:focus:ring-offset-gray-800
                             disabled:opacity-50 disabled:cursor-not-allowed
                             transition-colors"
                  >
                    Reset
                  </button>
                )}
              </div>
            </div>

            {/* Analysis Options */}
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-6">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Analysis Options
              </h2>

              <div className="space-y-4">
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={analysisOptions.enableDuplicates || false}
                    onChange={(e) => handleOptionsChange({ enableDuplicates: e.target.checked })}
                    disabled={loading}
                    className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded
                             focus:ring-blue-500 focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed"
                  />
                  <div>
                    <div className="text-gray-900 dark:text-gray-100 font-medium">
                      Enable Duplicate Detection
                    </div>
                    <div className="text-sm text-gray-500 dark:text-gray-400">
                      Analyze code for duplicate blocks (experimental)
                    </div>
                  </div>
                </label>
              </div>
            </div>

            {/* Loading State */}
            {loading && (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-6">
                <div className="text-center mb-4">
                  <div className="inline-flex items-center justify-center w-12 h-12 rounded-full bg-blue-100 dark:bg-blue-900/30 mb-3">
                    <svg
                      className="animate-spin h-6 w-6 text-blue-600 dark:text-blue-400"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                    >
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                  </div>
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-1">
                    Analyzing Repository
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    This may take a moment for large codebases...
                  </p>
                </div>
                <ProgressBar progress={50} message="Processing files..." indeterminate={true} />
              </div>
            )}

            {/* Error State */}
            {error && !loading && (
              <div className="bg-red-50 dark:bg-red-900/20 rounded-lg border border-red-200 dark:border-red-800 p-6 mb-6">
                <div className="flex items-start gap-3">
                  <svg
                    className="w-6 h-6 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-red-800 dark:text-red-200 mb-1">
                      Analysis Failed
                    </h3>
                    <p className="text-sm text-red-700 dark:text-red-300 mb-3">
                      {error}
                    </p>
                    <div className="text-sm text-red-600 dark:text-red-400 mb-3">
                      <strong>Troubleshooting:</strong>
                      <ul className="list-disc list-inside mt-1 space-y-1">
                        <li>Verify the repository path exists and is accessible</li>
                        <li>Ensure you have read permissions for the directory</li>
                        <li>Check that the path contains valid source code files</li>
                      </ul>
                    </div>
                    <button
                      onClick={handleAnalyze}
                      disabled={!repoPath.trim()}
                      className="px-4 py-2 bg-red-600 text-white rounded-lg font-medium text-sm
                               hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500
                               disabled:opacity-50 disabled:cursor-not-allowed
                               transition-colors"
                    >
                      Retry Analysis
                    </button>
                  </div>
                </div>
              </div>
            )}

            {/* Success State */}
            {data && !loading && !error && (
              <div className="bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800 p-6">
                <div className="flex items-start gap-3">
                  <svg
                    className="w-6 h-6 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-green-800 dark:text-green-200 mb-1">
                      Analysis Complete
                    </h3>
                    <p className="text-sm text-green-700 dark:text-green-300 mb-3">
                      Repository analyzed successfully. Switch to 2D or 3D view to visualize the results.
                    </p>
                    <div className="text-sm text-green-600 dark:text-green-400">
                      <strong>Repository:</strong> {repoPath}
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </ErrorBoundary>
  );
}
