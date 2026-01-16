/**
 * AnalysisHeader - Header section of the AnalysisView
 *
 * Contains repository path input, analyze button, settings, and breadcrumb navigation.
 */

import { Breadcrumb } from '@/components/common/Breadcrumb';
import { AnalysisSettings } from '@/components/common/AnalysisSettings';
import type { AnalysisOptions } from '@/types/bindings';

interface AnalysisHeaderProps {
  repoPath: string;
  onRepoPathChange: (path: string) => void;
  onPathKeyDown: (e: React.KeyboardEvent<HTMLInputElement>) => void;
  onBrowse: () => void;
  onAnalyze: () => void;
  onReset: () => void;
  loading: boolean;
  hasData: boolean;
  analysisOptions: AnalysisOptions;
  onOptionsChange: (options: AnalysisOptions) => void;
  deadCodeEnabled: boolean;
  onToggleDeadCode: () => void;
  drillDownPath: string[];
  onBreadcrumbNavigate: (index: number) => void;
}

export function AnalysisHeader({
  repoPath,
  onRepoPathChange,
  onPathKeyDown,
  onBrowse,
  onAnalyze,
  onReset,
  loading,
  hasData,
  analysisOptions,
  onOptionsChange,
  deadCodeEnabled,
  onToggleDeadCode,
  drillDownPath,
  onBreadcrumbNavigate,
}: AnalysisHeaderProps) {
  return (
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
              onChange={(e) => onRepoPathChange(e.target.value)}
              onKeyDown={onPathKeyDown}
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
              onClick={onBrowse}
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
            onClick={onAnalyze}
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

          {hasData && !loading && (
            <button
              onClick={onReset}
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
          )}
        </div>

        {/* Analysis Settings */}
        <AnalysisSettings
          options={analysisOptions}
          onChange={onOptionsChange}
          disabled={loading}
          deadCodeEnabled={deadCodeEnabled}
          onToggleDeadCode={onToggleDeadCode}
        />

        {/* Breadcrumb Navigation */}
        {hasData && !loading && (
          <div className="mt-4">
            <Breadcrumb path={drillDownPath} onNavigate={onBreadcrumbNavigate} />
          </div>
        )}
      </div>
    </header>
  );
}
