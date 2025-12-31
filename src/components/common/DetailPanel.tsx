/**
 * DetailPanel Component
 *
 * Displays detailed metadata for a selected file or directory node.
 * Shows path, LOC, complexity, last modified date, and other relevant information.
 *
 * Features:
 * - Formatted display of file metadata
 * - Close button (click and keyboard Escape)
 * - Expandable sections for organization
 * - Null-safe handling for edge cases
 */

import { useEffect } from 'react';
import type { TreeNode } from '@/types/bindings';
import { formatNumber, formatRelativeDate } from '@/utils/formatting';
import { complexityToColor } from '@/utils/colors';

export interface DetailPanelProps {
  /** Selected file/directory node to display details for */
  node: TreeNode | null;
  /** Callback fired when user closes the panel */
  onClose: () => void;
}

/**
 * DetailPanel component for displaying file metadata
 *
 * @param props - DetailPanelProps
 * @returns Null if no node selected, otherwise the detail panel
 */
export function DetailPanel({ node, onClose }: DetailPanelProps) {
  // Handle Escape key to close panel
  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    // Only add listener if panel is open (node exists)
    if (node) {
      window.addEventListener('keydown', handleEscape);
      return () => {
        window.removeEventListener('keydown', handleEscape);
      };
    }
  }, [node, onClose]);

  // Don't render if no node selected
  if (!node) {
    return null;
  }

  // Get complexity color for visual indicator
  const complexityColor = complexityToColor(node.complexity);

  return (
    <div data-testid="detail-panel" className="fixed right-0 top-0 h-full w-96 bg-white dark:bg-gray-800 shadow-2xl border-l border-gray-200 dark:border-gray-700 overflow-y-auto">
      {/* Header */}
      <div className="sticky top-0 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-4 flex items-start justify-between">
        <div className="flex-1 min-w-0">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100 truncate">
            {node.name}
          </h2>
          <p className="text-sm text-gray-500 dark:text-gray-400 capitalize">
            {node.type}
          </p>
        </div>
        <button
          onClick={onClose}
          data-testid="detail-panel-close"
          className="ml-4 p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
          aria-label="Close detail panel"
          title="Close (Esc)"
        >
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      {/* Content */}
      <div className="p-4 space-y-6">
        {/* Path Section */}
        <section>
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
            Path
          </h3>
          <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-3 border border-gray-200 dark:border-gray-700">
            <p
              className="text-sm text-gray-900 dark:text-gray-100 font-mono break-all"
              title={node.path}
            >
              {node.path}
            </p>
          </div>
        </section>

        {/* Basic Metrics Section */}
        <section>
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
            Basic Metrics
          </h3>
          <div className="space-y-3">
            {/* Language (for files) */}
            {node.type === 'file' && node.language && (
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Language</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100 capitalize">
                  {node.language}
                </span>
              </div>
            )}

            {/* Lines of Code */}
            <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
              <span className="text-sm text-gray-600 dark:text-gray-400">Lines of Code</span>
              <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                {formatNumber(node.loc)}
              </span>
            </div>

            {/* File Size (for files) */}
            {node.type === 'file' && node.sizeBytes !== undefined && (
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">File Size</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {(node.sizeBytes / 1024).toFixed(2)} KB
                </span>
              </div>
            )}

            {/* Function Count (for files) */}
            {node.type === 'file' && node.functionCount !== undefined && (
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Functions</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {formatNumber(node.functionCount)}
                </span>
              </div>
            )}

            {/* Complexity */}
            <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
              <span className="text-sm text-gray-600 dark:text-gray-400">Complexity Score</span>
              <div className="flex items-center gap-2">
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {formatNumber(node.complexity)}
                </span>
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: complexityColor }}
                  title={`Complexity: ${node.complexity}`}
                  aria-label={`Complexity indicator: ${node.complexity}`}
                />
              </div>
            </div>

            {/* Children Count (for directories) */}
            {node.type === 'directory' && (
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {node.children.length === 1 ? 'Child' : 'Children'}
                </span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {formatNumber(node.children.length)}
                </span>
              </div>
            )}
          </div>
        </section>

        {/* Cognitive Complexity Section */}
        {node.cognitiveComplexity && (
          <section>
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
              Cognitive Complexity
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Total</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.cognitiveComplexity.totalComplexity}
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Average</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.cognitiveComplexity.averageComplexity.toFixed(1)}
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Maximum</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.cognitiveComplexity.maxComplexity}
                </span>
              </div>
            </div>
          </section>
        )}

        {/* Coupling Metrics Section */}
        {node.coupling && (
          <section>
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
              Coupling Metrics
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Afferent (Incoming)</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.coupling.afferentCoupling}
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Efferent (Outgoing)</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.coupling.efferentCoupling}
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Instability</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.coupling.instability.toFixed(2)}
                </span>
              </div>
            </div>
          </section>
        )}

        {/* Code Churn Section */}
        {node.codeChurn && (
          <section>
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
              Code Churn
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Lines Added</span>
                <span className="text-sm font-semibold text-green-600 dark:text-green-400">
                  +{formatNumber(node.codeChurn.addedLines)}
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Lines Deleted</span>
                <span className="text-sm font-semibold text-red-600 dark:text-red-400">
                  -{formatNumber(node.codeChurn.deletedLines)}
                </span>
              </div>
            </div>
          </section>
        )}

        {/* AI Bloat Index Section */}
        {node.aiBloatIndex !== undefined && (
          <section>
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
              AI Bloat Index
            </h3>
            <div className="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600 dark:text-gray-400">Bloat Score</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.aiBloatIndex.toFixed(1)}%
                </span>
              </div>
            </div>
          </section>
        )}

        {/* Test Coverage Section */}
        {node.testCoverage && (
          <section>
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
              Test Coverage
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Line Coverage</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.testCoverage.lineCoverage.toFixed(1)}%
                </span>
              </div>
              <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                <span className="text-sm text-gray-600 dark:text-gray-400">Lines Covered</span>
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {node.testCoverage.linesCovered} / {node.testCoverage.totalLines}
                </span>
              </div>
              {node.testCoverage.branchCoverage !== undefined && (
                <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                  <span className="text-sm text-gray-600 dark:text-gray-400">Branch Coverage</span>
                  <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                    {node.testCoverage.branchCoverage.toFixed(1)}%
                  </span>
                </div>
              )}
            </div>
          </section>
        )}

        {/* Timestamp Section */}
        <section>
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
            Last Modified
          </h3>
          <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-3 border border-gray-200 dark:border-gray-700">
            <p
              className="text-sm text-gray-900 dark:text-gray-100"
              title={node.lastModified}
            >
              {formatRelativeDate(node.lastModified)}
            </p>
          </div>
        </section>

        {/* ID Section (for debugging, can be collapsed in production) */}
        <details className="group">
          <summary className="cursor-pointer text-sm font-semibold text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 rounded px-1">
            <span className="inline-flex items-center gap-2">
              Technical Details
              <svg
                className="w-4 h-4 transition-transform group-open:rotate-90"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            </span>
          </summary>
          <div className="mt-2 bg-gray-50 dark:bg-gray-900 rounded-lg p-3 border border-gray-200 dark:border-gray-700">
            <p className="text-xs text-gray-600 dark:text-gray-400 mb-1">
              Node ID
            </p>
            <p className="text-xs text-gray-900 dark:text-gray-100 font-mono break-all">
              {node.id}
            </p>
          </div>
        </details>
      </div>
    </div>
  );
}
