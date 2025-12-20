/**
 * AnalysisView Feature Component
 *
 * Main feature component that orchestrates the complete repository analysis workflow.
 * Integrates Treemap visualization, Breadcrumb navigation, and DetailPanel components
 * with drill-down state management.
 *
 * Features:
 * - Repository path input with file picker
 * - Analyze button to trigger repository analysis
 * - Hierarchical treemap visualization with drill-down
 * - Breadcrumb navigation for drill-down path
 * - Detail panel for selected files
 * - Loading states and error handling
 * - Keyboard navigation support
 */

import { useState, useCallback, useMemo, useEffect } from 'react';
import { Treemap } from '@/components/visualizations/Treemap';
import { Breadcrumb } from '@/components/common/Breadcrumb';
import { DetailPanel } from '@/components/common/DetailPanel';
import { DeadCodePanel } from '@/components/common/DeadCodePanel';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { DataDebugger } from '@/components/common/DataDebugger';
import { ProgressBar } from '@/components/common/ProgressBar';
import { TreeView } from '@/components/common/TreeView';
import { useAnalysis } from '@/hooks/useAnalysis';
import { useDeadCodeAnalysis } from '@/hooks/useDeadCodeAnalysis';
import {
  useSelectedFile,
  useDrillDownPath,
  useAnalysisActions,
  useDeadCodeEnabled,
} from '@/store/analysisStore';
import type { TreeNode } from '@/types/bindings';
import { filterByPath } from '@/utils/treeTransform';

/**
 * AnalysisView - Main feature component for code analysis and visualization
 */
export function AnalysisView() {
  // Local state for repository path input (load from localStorage if available)
  const [repoPath, setRepoPath] = useState<string>(() => {
    try {
      return localStorage.getItem('lastRepoPath') || '';
    } catch {
      return '';
    }
  });

  // Local state for view mode
  const [showTreeView, setShowTreeView] = useState(false);

  // Analysis hook for executing repository analysis
  const { data, loading, error, analyze, reset } = useAnalysis();

  // Dead code analysis hook
  const {
    results: deadCodeResults,
    loading: deadCodeLoading,
    error: deadCodeError,
    analyze: analyzeDeadCode
  } = useDeadCodeAnalysis();

  // Store state and actions
  const selectedFile = useSelectedFile();
  const drillDownPath = useDrillDownPath();
  const deadCodeEnabled = useDeadCodeEnabled();
  const { setSelectedFile, setDrillDownPath, toggleDeadCodeOverlay } = useAnalysisActions();

  /**
   * Trigger dead code analysis when main analysis completes and overlay is enabled
   */
  useEffect(() => {
    if (data && deadCodeEnabled && repoPath.trim() && !deadCodeResults && !deadCodeLoading) {
      console.log('[AnalysisView] Auto-triggering dead code analysis');
      analyzeDeadCode(repoPath.trim(), 80);
    }
  }, [data, deadCodeEnabled, repoPath, deadCodeResults, deadCodeLoading, analyzeDeadCode]);

  /**
   * Handle analyze button click
   */
  const handleAnalyze = useCallback(async () => {
    console.log('[AnalysisView] handleAnalyze called, repoPath:', repoPath);
    if (!repoPath.trim()) {
      console.warn('[AnalysisView] Empty path, aborting');
      return;
    }

    // Save path to localStorage for next time
    try {
      localStorage.setItem('lastRepoPath', repoPath.trim());
    } catch (error) {
      console.warn('[AnalysisView] Failed to save path to localStorage:', error);
    }

    console.log('[AnalysisView] Calling analyze() with:', repoPath.trim());
    try {
      await analyze(repoPath.trim());
      console.log('[AnalysisView] analyze() returned successfully');
      // If dead code overlay is enabled, also run dead code analysis
      if (deadCodeEnabled) {
        console.log('[AnalysisView] Running dead code analysis');
        await analyzeDeadCode(repoPath.trim(), 80);
      }
    } catch (error) {
      console.error('[AnalysisView] analyze() threw error:', error);
    }
  }, [repoPath, analyze, deadCodeEnabled, analyzeDeadCode]);

  /**
   * Handle Enter key in path input
   */
  const handlePathKeyDown = useCallback(
    (event: React.KeyboardEvent<HTMLInputElement>) => {
      if (event.key === 'Enter') {
        handleAnalyze();
      }
    },
    [handleAnalyze]
  );

  /**
   * Handle folder picker dialog
   */
  const handleBrowse = useCallback(async () => {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Repository Directory',
      });

      if (selected && typeof selected === 'string') {
        setRepoPath(selected);
      }
    } catch (error) {
      console.error('Failed to open folder picker:', error);
    }
  }, []);

  /**
   * Handle treemap node click for drill-down
   */
  const handleNodeClick = useCallback(
    (node: TreeNode) => {
      console.log('[AnalysisView] Node clicked:', {
        name: node.name,
        type: node.type,
        path: node.path,
        currentDrillDownPath: JSON.stringify(drillDownPath),
      });

      // If it's a directory, drill down
      if (node.type === 'directory' && node.children.length > 0) {
        // Build new drill-down path
        const newPath = [...drillDownPath, node.name];
        console.log('[AnalysisView] Drilling down, new path:', JSON.stringify(newPath));
        setDrillDownPath(newPath);
        setSelectedFile(null);
      } else {
        // If it's a file, show details
        console.log('[AnalysisView] Selecting file');
        setSelectedFile(node);
      }
    },
    [drillDownPath, setDrillDownPath, setSelectedFile]
  );

  /**
   * Handle treemap node hover
   */
  const handleNodeHover = useCallback(
    (_node: TreeNode | null) => {
      // Could implement tooltip or hover effects here
      // For now, we rely on ECharts built-in tooltip
    },
    []
  );

  /**
   * Handle breadcrumb navigation
   * Index -1 means navigate to root, otherwise navigate to specific segment
   */
  const handleBreadcrumbNavigate = useCallback(
    (index: number) => {
      if (index === -1) {
        // Navigate to root
        setDrillDownPath([]);
      } else {
        // Navigate to specific segment (inclusive)
        setDrillDownPath(drillDownPath.slice(0, index + 1));
      }
      setSelectedFile(null);
    },
    [drillDownPath, setDrillDownPath, setSelectedFile]
  );

  /**
   * Handle detail panel close
   */
  const handleDetailPanelClose = useCallback(() => {
    setSelectedFile(null);
  }, [setSelectedFile]);

  /**
   * Handle navigate back (Escape key in treemap)
   */
  const handleNavigateBack = useCallback(() => {
    if (drillDownPath.length > 0) {
      // Navigate up one level
      setDrillDownPath(drillDownPath.slice(0, -1));
      setSelectedFile(null);
    }
  }, [drillDownPath, setDrillDownPath, setSelectedFile]);

  /**
   * Handle reset button click
   */
  const handleReset = useCallback(() => {
    reset();
    setRepoPath('');
  }, [reset]);

  /**
   * Handle dead code panel close
   */
  const handleDeadCodePanelClose = useCallback(() => {
    setSelectedFile(null);
  }, [setSelectedFile]);

  /**
   * Compute current tree node based on drill-down path
   */
  const currentTreeNode = useMemo(() => {
    if (!data) return null;
    if (drillDownPath.length === 0) return data;
    return filterByPath(data, drillDownPath);
  }, [data, drillDownPath]);

  /**
   * Find dead code for the currently selected file
   */
  const selectedFileDeadCode = useMemo(() => {
    if (!selectedFile || !deadCodeResults || selectedFile.type !== 'file') {
      return null;
    }

    // Find the file in dead code results by path
    const fileDeadCode = deadCodeResults.files.find(
      (f) => f.path === selectedFile.path
    );

    return fileDeadCode || null;
  }, [selectedFile, deadCodeResults]);

  return (
    <div className="h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4">
        <div className="max-w-7xl mx-auto">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4">
            Code Visualization
          </h1>

          {/* Repository Path Input */}
          <div className="flex gap-3">
            <div className="flex-1 flex gap-2">
              <input
                type="text"
                value={repoPath}
                onChange={(e) => setRepoPath(e.target.value)}
                onKeyDown={handlePathKeyDown}
                placeholder="Enter repository path..."
                disabled={loading}
                data-testid="repository-path-input"
                className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg
                         bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                         placeholder-gray-400 dark:placeholder-gray-500
                         focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                         disabled:opacity-50 disabled:cursor-not-allowed"
                aria-label="Repository path"
              />
              <button
                onClick={handleBrowse}
                disabled={loading}
                className="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300
                         border border-gray-300 dark:border-gray-600 rounded-lg
                         hover:bg-gray-200 dark:hover:bg-gray-600
                         focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                         dark:focus:ring-offset-gray-800
                         disabled:opacity-50 disabled:cursor-not-allowed
                         transition-colors"
                aria-label="Browse for directory"
              >
                Browse
              </button>
            </div>

            <button
              onClick={handleAnalyze}
              disabled={loading || !repoPath.trim()}
              data-testid="analyze-button"
              className="px-6 py-2 bg-blue-600 text-white rounded-lg font-medium
                       hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                       dark:focus:ring-offset-gray-800
                       disabled:opacity-50 disabled:cursor-not-allowed
                       transition-colors"
              aria-label="Analyze repository"
            >
              {loading ? 'Analyzing...' : 'Analyze'}
            </button>

            {data && !loading && (
              <>
                <button
                  onClick={toggleDeadCodeOverlay}
                  className={`px-4 py-2 rounded-lg font-medium border transition-colors
                           focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                           dark:focus:ring-offset-gray-800
                           ${
                             deadCodeEnabled
                               ? 'bg-blue-600 text-white border-blue-600 hover:bg-blue-700'
                               : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600'
                           }`}
                  aria-label="Toggle dead code overlay"
                  title="Toggle dead code visualization overlay"
                >
                  <span className="flex items-center gap-2">
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M13 10V3L4 14h7v7l9-11h-7z"
                      />
                    </svg>
                    Dead Code
                  </span>
                </button>
                <button
                  onClick={handleReset}
                  className="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300
                         border border-gray-300 dark:border-gray-600 rounded-lg
                         hover:bg-gray-200 dark:hover:bg-gray-600
                         focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                         dark:focus:ring-offset-gray-800
                         transition-colors"
                  aria-label="Reset analysis"
                >
                  Reset
                </button>
              </>
            )}
          </div>

          {/* Breadcrumb Navigation */}
          {data && !loading && (
            <div className="mt-4">
              <Breadcrumb
                path={drillDownPath}
                onNavigate={handleBreadcrumbNavigate}
              />
            </div>
          )}
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 overflow-hidden relative">
        {/* Dead Code Loading State */}
        {deadCodeLoading && deadCodeEnabled && !loading && (
          <div className="absolute top-4 right-4 z-50 bg-white dark:bg-gray-800 rounded-lg shadow-lg px-4 py-3 border border-gray-200 dark:border-gray-700">
            <div className="flex items-center gap-3">
              <svg
                className="animate-spin h-5 w-5 text-blue-600 dark:text-blue-400"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                ></circle>
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
              <p className="text-sm font-medium text-gray-900 dark:text-gray-100">
                Analyzing dead code...
              </p>
            </div>
          </div>
        )}

        {/* Dead Code Error State */}
        {deadCodeError && deadCodeEnabled && !loading && (
          <div className="absolute top-4 right-4 z-50 bg-red-50 dark:bg-red-900/20 rounded-lg shadow-lg px-4 py-3 border border-red-200 dark:border-red-800 max-w-md">
            <div className="flex items-start gap-3">
              <svg
                className="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
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
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-red-800 dark:text-red-200 mb-1">
                  Dead Code Analysis Failed
                </p>
                <p className="text-sm text-red-700 dark:text-red-300 break-words">
                  {deadCodeError}
                </p>
                <button
                  onClick={() => analyzeDeadCode(repoPath.trim(), 80)}
                  className="mt-2 text-sm font-medium text-red-700 dark:text-red-300 hover:text-red-900 dark:hover:text-red-100 underline"
                >
                  Retry
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Loading State - Enhanced Skeleton with Progress */}
        {loading && (
          <div data-testid="loading-state" className="absolute inset-0 flex items-center justify-center bg-white dark:bg-gray-900">
            <div className="max-w-2xl w-full px-8">
              <div className="text-center mb-8">
                <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-blue-100 dark:bg-blue-900/30 mb-4">
                  <svg
                    className="animate-spin h-8 w-8 text-blue-600 dark:text-blue-400"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                </div>
                <h2 className="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">
                  Analyzing Repository
                </h2>
                <p className="text-gray-600 dark:text-gray-400">
                  This may take a few moments for large codebases...
                </p>
              </div>

              {/* Progress Bar */}
              <ProgressBar progress={50} message="Processing files..." indeterminate={true} />

              <div className="mt-6 text-center text-sm text-gray-500 dark:text-gray-400">
                Check the browser console for detailed progress
              </div>
            </div>
          </div>
        )}

        {/* Error State - Enhanced with Retry */}
        {error && !loading && (
          <div data-testid="error-state" className="absolute inset-0 flex items-center justify-center bg-white dark:bg-gray-900 p-4">
            <div className="max-w-lg w-full bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
              <div className="flex items-center justify-center w-16 h-16 mx-auto rounded-full bg-red-100 dark:bg-red-900/30 mb-4">
                <svg
                  className="w-8 h-8 text-red-600 dark:text-red-400"
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
              </div>
              <h2 className="text-2xl font-bold text-gray-900 dark:text-gray-100 text-center mb-3">
                Analysis Failed
              </h2>
              <p data-testid="error-message" className="text-gray-600 dark:text-gray-400 text-center mb-6">
                {error}
              </p>

              <div className="bg-gray-50 dark:bg-gray-900/50 rounded-lg p-4 mb-6">
                <h3 className="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-2">
                  Troubleshooting Tips:
                </h3>
                <ul className="text-sm text-gray-600 dark:text-gray-400 space-y-1 list-disc list-inside">
                  <li>Verify the repository path exists and is accessible</li>
                  <li>Ensure you have read permissions for the directory</li>
                  <li>Check that the path contains valid source code files</li>
                  <li>Try using an absolute path instead of a relative one</li>
                </ul>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={handleAnalyze}
                  disabled={!repoPath.trim()}
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg font-medium
                           hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                           dark:focus:ring-offset-gray-800
                           disabled:opacity-50 disabled:cursor-not-allowed
                           transition-colors"
                >
                  Retry Analysis
                </button>
                <button
                  onClick={handleReset}
                  className="flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300
                           border border-gray-300 dark:border-gray-600 rounded-lg
                           hover:bg-gray-200 dark:hover:bg-gray-600
                           focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                           dark:focus:ring-offset-gray-800
                           transition-colors"
                >
                  Start Over
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Empty State */}
        {!data && !loading && !error && (
          <div className="absolute inset-0 flex items-center justify-center bg-white dark:bg-gray-900">
            <div className="text-center max-w-md">
              <div className="inline-flex items-center justify-center w-20 h-20 rounded-full bg-blue-100 dark:bg-blue-900/30 mb-6">
                <svg
                  className="w-10 h-10 text-blue-600 dark:text-blue-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                  />
                </svg>
              </div>
              <h2 className="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-3">
                Welcome to Code Visualization
              </h2>
              <p className="text-gray-600 dark:text-gray-400 mb-6">
                Enter a repository path above and click "Analyze" to visualize your codebase structure and complexity.
              </p>
              <div className="text-sm text-gray-500 dark:text-gray-500 space-y-2">
                <p>Interactive treemap visualization</p>
                <p>Drill-down navigation</p>
                <p>Complexity analysis</p>
              </div>
            </div>
          </div>
        )}

        {/* Treemap Visualization - Wrapped in Error Boundary */}
        {currentTreeNode && !loading && !error && (
          <ErrorBoundary
            fallback={(error, reset) => (
              <div className="h-full flex items-center justify-center p-6">
                <div className="max-w-md text-center">
                  <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-red-100 dark:bg-red-900/30 mb-4">
                    <svg
                      className="w-8 h-8 text-red-600 dark:text-red-400"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                  </div>
                  <h3 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2">
                    Visualization Error
                  </h3>
                  <p className="text-gray-600 dark:text-gray-400 mb-4">
                    The treemap visualization encountered an error.
                  </p>
                  <details className="mb-4 text-left">
                    <summary className="text-sm text-gray-500 dark:text-gray-500 cursor-pointer hover:text-gray-700 dark:hover:text-gray-300">
                      Error Details
                    </summary>
                    <pre className="text-xs bg-gray-100 dark:bg-gray-900 p-3 rounded mt-2 overflow-auto max-h-32 text-red-600 dark:text-red-400">
                      {error.message}
                    </pre>
                  </details>
                  <button
                    onClick={reset}
                    className="px-6 py-2 bg-blue-600 text-white rounded-lg font-medium
                             hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500
                             transition-colors"
                  >
                    Retry
                  </button>
                </div>
              </div>
            )}
          >
            <div data-testid="treemap-container" className="h-full p-6">
              <div className="h-full max-w-7xl mx-auto">
                <Treemap
                  data={currentTreeNode}
                  drillDownPath={drillDownPath}
                  onNodeClick={handleNodeClick}
                  onNodeHover={handleNodeHover}
                  onNavigateBack={handleNavigateBack}
                  width="100%"
                  height="100%"
                />
              </div>
            </div>
          </ErrorBoundary>
        )}

        {/* Detail Panel */}
        <DetailPanel node={selectedFile} onClose={handleDetailPanelClose} />

        {/* Dead Code Panel - Only show when dead code enabled and file has dead code */}
        {deadCodeEnabled && selectedFileDeadCode && (
          <DeadCodePanel
            file={selectedFileDeadCode}
            onClose={handleDeadCodePanelClose}
          />
        )}

        {/* View Toggle Button (when data available) */}
        {data && !loading && !error && (
          <button
            onClick={() => setShowTreeView(!showTreeView)}
            className="fixed bottom-4 left-4 z-40 px-4 py-2 bg-gray-800 dark:bg-gray-700 text-white rounded-lg shadow-lg
                     hover:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500
                     transition-colors flex items-center gap-2"
            title="Toggle between Treemap and Tree View"
          >
            <span>{showTreeView ? '📊 Treemap' : '🌳 Tree View'}</span>
          </button>
        )}

        {/* Tree View Modal */}
        {showTreeView && data && (
          <div className="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4">
            <div className="bg-white dark:bg-gray-900 rounded-lg shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col">
              <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
                <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                  Tree View - Debugging
                </h3>
                <button
                  onClick={() => setShowTreeView(false)}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  ✕
                </button>
              </div>
              <div className="flex-1 overflow-auto p-6">
                <TreeView data={currentTreeNode} maxDepth={10} />
              </div>
            </div>
          </div>
        )}

        {/* Data Debugger */}
        <DataDebugger data={data} />
      </main>
    </div>
  );
}
