/**
 * DeadCodePanel Component
 *
 * Displays a list of dead (unreachable) symbols for a selected file.
 * Shows symbol name, type, confidence score, reason, and line numbers.
 *
 * Features:
 * - Color-coded confidence scores (green >80, yellow 60-80, red <60)
 * - Displays reason for marking symbol as dead
 * - Line number information for navigation
 * - Close button (click and keyboard Escape)
 * - Scrollable list for files with many dead symbols
 * - Accessible keyboard navigation
 */

import { useEffect } from 'react';
import type { FileDeadCode, SymbolKind } from '@/types/bindings';
import { formatNumber } from '@/utils/formatting';
import { confidenceToColor } from '@/utils/colors';

export interface DeadCodePanelProps {
  /** Selected file with dead code information */
  file: FileDeadCode | null;
  /** Callback fired when user closes the panel */
  onClose: () => void;
}

/**
 * Maps SymbolKind enum to human-readable label
 */
function symbolKindToLabel(kind: SymbolKind): string {
  switch (kind) {
    case 'Function':
      return 'Function';
    case 'ArrowFunction':
      return 'Arrow Function';
    case 'Class':
      return 'Class';
    case 'Method':
      return 'Method';
    case 'Variable':
      return 'Variable';
    default:
      return 'Unknown';
  }
}

/**
 * DeadCodePanel component for displaying dead symbols
 *
 * @param props - DeadCodePanelProps
 * @returns Null if no file selected, otherwise the dead code panel
 */
export function DeadCodePanel({ file, onClose }: DeadCodePanelProps) {
  // Handle Escape key to close panel
  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    // Only add listener if panel is open (file exists)
    if (file) {
      window.addEventListener('keydown', handleEscape);
      return () => {
        window.removeEventListener('keydown', handleEscape);
      };
    }
  }, [file, onClose]);

  // Don't render if no file selected
  if (!file) {
    return null;
  }

  return (
    <div
      data-testid="dead-code-panel"
      className="fixed right-0 top-0 h-full w-96 bg-white dark:bg-gray-800 shadow-2xl border-l border-gray-200 dark:border-gray-700 overflow-y-auto"
    >
      {/* Header */}
      <div className="sticky top-0 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-4 flex items-start justify-between z-10">
        <div className="flex-1 min-w-0">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-1">
            Dead Code
          </h2>
          <p
            className="text-sm text-gray-500 dark:text-gray-400 font-mono break-all"
            title={file.path}
          >
            {file.path}
          </p>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
            {file.deadCode.length} {file.deadCode.length === 1 ? 'symbol' : 'symbols'}
          </p>
        </div>
        <button
          onClick={onClose}
          data-testid="dead-code-panel-close"
          className="ml-4 p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
          aria-label="Close dead code panel"
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

      {/* Content - Dead Symbols List */}
      <div className="p-4">
        {file.deadCode.length === 0 ? (
          <div className="text-center py-8">
            <p className="text-gray-500 dark:text-gray-400">
              No dead code found in this file
            </p>
          </div>
        ) : (
          <ul className="space-y-3" role="list" aria-label="Dead symbols">
            {file.deadCode.map((symbol, index) => {
              const confidenceColor = confidenceToColor(symbol.confidence);

              return (
                <li
                  key={`${symbol.symbol}-${symbol.lineStart}-${index}`}
                  className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-colors"
                >
                  {/* Symbol Name and Type */}
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex-1 min-w-0">
                      <h3
                        className="text-sm font-semibold text-gray-900 dark:text-gray-100 font-mono truncate"
                        title={symbol.symbol}
                      >
                        {symbol.symbol}
                      </h3>
                      <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
                        {symbolKindToLabel(symbol.kind)}
                      </p>
                    </div>

                    {/* Confidence Badge */}
                    <div
                      className="ml-2 px-2 py-1 rounded text-xs font-semibold text-white flex items-center gap-1.5 flex-shrink-0"
                      style={{ backgroundColor: confidenceColor }}
                      title={`Confidence: ${symbol.confidence}%`}
                      aria-label={`Confidence score: ${symbol.confidence}%`}
                    >
                      <div
                        className="w-2 h-2 rounded-full bg-white opacity-75"
                        aria-hidden="true"
                      />
                      {symbol.confidence}%
                    </div>
                  </div>

                  {/* Line Numbers */}
                  <div className="flex items-center gap-2 text-xs text-gray-600 dark:text-gray-400 mb-2">
                    <svg
                      className="w-3.5 h-3.5 flex-shrink-0"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"
                      />
                    </svg>
                    <span className="font-mono">
                      Lines {formatNumber(symbol.lineStart)}–{formatNumber(symbol.lineEnd)}
                    </span>
                    <span className="text-gray-400 dark:text-gray-500">•</span>
                    <span>{formatNumber(symbol.loc)} LOC</span>
                  </div>

                  {/* Reason */}
                  <div className="mb-3">
                    <p className="text-xs text-gray-700 dark:text-gray-300 leading-relaxed">
                      {symbol.reason}
                    </p>
                  </div>

                  {/* View in Editor Button (placeholder for future integration) */}
                  <button
                    onClick={() => {
                      // Placeholder for future editor integration
                      console.log(`Navigate to ${file.path}:${symbol.lineStart}`);
                    }}
                    className="w-full px-3 py-2 text-xs font-medium text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20 rounded hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled
                    title="Editor integration coming soon"
                    aria-label={`View ${symbol.symbol} in editor at line ${symbol.lineStart}`}
                  >
                    View in Editor (Coming Soon)
                  </button>
                </li>
              );
            })}
          </ul>
        )}
      </div>
    </div>
  );
}
