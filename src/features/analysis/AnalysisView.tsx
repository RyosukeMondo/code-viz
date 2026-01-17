/**
 * AnalysisView Feature Component
 *
 * Main feature component for displaying 2D treemap visualization.
 * Reads data from analysisStore populated by ConfigView.
 *
 * Features:
 * - Hierarchical treemap visualization with drill-down
 * - Breadcrumb navigation for drill-down path
 * - Detail panel for selected files
 * - Keyboard navigation support
 */

import { useState, useMemo } from 'react';
import Sunburst from '@/components/visualizations/Sunburst';
import { DetailPanel } from '@/components/common/DetailPanel';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { TreeView } from '@/components/common/TreeView';
import {
  useMetrics,
  useSelectedFile,
  useDrillDownPath,
  useAnalysisActions,
} from '@/store/analysisStore';
import { filterByPath } from '@/utils/treeTransform';

/**
 * AnalysisView - 2D treemap visualization component
 */
export function AnalysisView() {
  // Local state for view mode
  const [showTreeView, setShowTreeView] = useState(false);

  // Store state
  const data = useMetrics();
  const selectedFile = useSelectedFile();
  const drillDownPath = useDrillDownPath();
  const { setSelectedFile, setDrillDownPath } = useAnalysisActions();

  /**
   * Compute current tree node based on drill-down path
   */
  const currentTreeNode = useMemo(() => {
    if (!data) return null;
    if (drillDownPath.length === 0) return data;
    return filterByPath(data, drillDownPath);
  }, [data, drillDownPath]);

  // Handle node click for drill-down
  const handleNodeClick = (node: typeof data) => {
    if (!node) return;

    if (node.type === 'directory') {
      // Drill down into directory
      setDrillDownPath([...drillDownPath, node.id]);
    } else {
      // Select file
      setSelectedFile(node);
    }
  };

  // Handle node hover
  const handleNodeHover = () => {
    // Can add hover effects here if needed
  };

  // Handle breadcrumb navigation
  const handleBreadcrumbNavigate = (index: number) => {
    if (index === -1) {
      // Navigate to root
      setDrillDownPath([]);
    } else {
      // Navigate to specific level
      setDrillDownPath(drillDownPath.slice(0, index + 1));
    }
  };

  // Handle detail panel close
  const handleDetailPanelClose = () => {
    setSelectedFile(null);
  };

  // Handle navigate back
  const handleNavigateBack = () => {
    if (drillDownPath.length > 0) {
      setDrillDownPath(drillDownPath.slice(0, -1));
    }
  };

  return (
    <div className="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
      {/* Breadcrumb Navigation */}
      {data && drillDownPath.length > 0 && (
        <div className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-3">
          <nav className="flex items-center gap-2 text-sm">
            <button
              onClick={() => handleBreadcrumbNavigate(-1)}
              className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 font-medium transition-colors"
            >
              Root
            </button>
            {drillDownPath.map((_, index) => {
              const pathSegment = drillDownPath.slice(0, index + 1);
              const node = filterByPath(data, pathSegment);
              return (
                <div key={index} className="flex items-center gap-2">
                  <span className="text-gray-400 dark:text-gray-600">/</span>
                  <button
                    onClick={() => handleBreadcrumbNavigate(index)}
                    className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 font-medium transition-colors"
                  >
                    {node?.name || 'Unknown'}
                  </button>
                </div>
              );
            })}
          </nav>
        </div>
      )}

      {/* Main Content */}
      <main className="flex-1 overflow-hidden relative">
        {/* Empty State */}
        {!data && (
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
                No Repository Analyzed
              </h2>
              <p className="text-gray-600 dark:text-gray-400 mb-6">
                Go to the Config tab to analyze a repository first.
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
        {currentTreeNode && (
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
            <div data-testid="visualization-container" className="h-full p-6">
              <div className="h-full max-w-7xl mx-auto">
                <Sunburst
                  data={currentTreeNode}
                  onNodeClick={handleNodeClick}
                  onNodeHover={handleNodeHover}
                  onNavigateBack={handleNavigateBack}
                />
              </div>
            </div>
          </ErrorBoundary>
        )}

        {/* Detail Panel */}
        <DetailPanel node={selectedFile} onClose={handleDetailPanelClose} />

        {/* View Toggle Button (when data available) */}
        {data && (
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
      </main>
    </div>
  );
}
