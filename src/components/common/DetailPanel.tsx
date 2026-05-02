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
import { formatRelativeDate } from '@/utils/formatting';
import { BasicMetricsSection } from './BasicMetricsSection';
import { CognitiveComplexitySection } from './CognitiveComplexitySection';
import { CouplingMetricsSection } from './CouplingMetricsSection';
import { CodeChurnSection } from './CodeChurnSection';
import { AiBloatIndexSection } from './AiBloatIndexSection';
import { TestCoverageSection } from './TestCoverageSection';

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
        <BasicMetricsSection node={node} />

        {/* Cognitive Complexity Section */}
        {node.cognitiveComplexity && (
          <CognitiveComplexitySection cognitiveComplexity={node.cognitiveComplexity} />
        )}

        {/* Coupling Metrics Section */}
        {node.coupling && (
          <CouplingMetricsSection coupling={node.coupling} />
        )}

        {/* Code Churn Section */}
        {node.codeChurn && (
          <CodeChurnSection codeChurn={node.codeChurn} />
        )}

        {/* AI Bloat Index Section */}
        {node.aiBloatIndex !== undefined && (
          <AiBloatIndexSection aiBloatIndex={node.aiBloatIndex} />
        )}

        {/* Test Coverage Section */}
        {node.testCoverage && (
          <TestCoverageSection testCoverage={node.testCoverage} />
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
