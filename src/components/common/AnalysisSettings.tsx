/**
 * AnalysisSettings Component
 *
 * Collapsible settings panel for configuring analysis options.
 * Allows users to enable/disable features like duplicate detection,
 * hotspot analysis, AI commit analysis, and coverage integration.
 */

import { useState } from 'react';
import type { AnalysisOptions } from '@/types';

export interface AnalysisSettingsProps {
  /** Current options */
  options: AnalysisOptions;

  /** Callback when options change */
  onChange: (options: AnalysisOptions) => void;

  /** Whether analysis is currently running */
  disabled?: boolean;
}

/**
 * Collapsible settings panel for analysis options
 */
export function AnalysisSettings({ options, onChange, disabled = false }: AnalysisSettingsProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const handleToggle = (field: keyof AnalysisOptions) => {
    onChange({
      ...options,
      [field]: !options[field],
    });
  };

  const handleNumberChange = (field: keyof AnalysisOptions, value: number) => {
    onChange({
      ...options,
      [field]: value,
    });
  };

  const handleTextChange = (field: keyof AnalysisOptions, value: string) => {
    onChange({
      ...options,
      [field]: value || undefined,
    });
  };

  return (
    <div className="mt-4">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        disabled={disabled}
        className="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-300
                   hover:text-gray-900 dark:hover:text-gray-100
                   disabled:opacity-50 disabled:cursor-not-allowed
                   transition-colors"
        aria-expanded={isExpanded}
        aria-controls="analysis-settings-panel"
      >
        <svg
          className={`w-4 h-4 transition-transform ${isExpanded ? 'rotate-90' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9 5l7 7-7 7"
          />
        </svg>
        Advanced Options
      </button>

      {isExpanded && (
        <div
          id="analysis-settings-panel"
          className="mt-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700"
        >
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Duplicate Detection */}
            <div className="space-y-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.enableDuplicates || false}
                  onChange={() => handleToggle('enableDuplicates')}
                  disabled={disabled}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded
                           focus:ring-blue-500 focus:ring-2
                           disabled:opacity-50 disabled:cursor-not-allowed"
                />
                <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                  Duplicate Detection
                </span>
              </label>
              {options.enableDuplicates && (
                <div className="ml-6">
                  <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                    Min duplicate lines
                  </label>
                  <input
                    type="number"
                    min="2"
                    max="100"
                    value={options.minDuplicateLines || 5}
                    onChange={(e) => handleNumberChange('minDuplicateLines', parseInt(e.target.value))}
                    disabled={disabled}
                    className="w-24 px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded
                             bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                             focus:outline-none focus:ring-2 focus:ring-blue-500
                             disabled:opacity-50 disabled:cursor-not-allowed"
                  />
                </div>
              )}
            </div>

            {/* Hotspot Detection */}
            <div className="space-y-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.enableHotspots || false}
                  onChange={() => handleToggle('enableHotspots')}
                  disabled={disabled}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded
                           focus:ring-blue-500 focus:ring-2
                           disabled:opacity-50 disabled:cursor-not-allowed"
                />
                <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                  Git Hotspot Detection
                </span>
              </label>
              {options.enableHotspots && (
                <div className="ml-6">
                  <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                    Max hotspots to report
                  </label>
                  <input
                    type="number"
                    min="1"
                    max="50"
                    value={options.maxHotspots || 10}
                    onChange={(e) => handleNumberChange('maxHotspots', parseInt(e.target.value))}
                    disabled={disabled}
                    className="w-24 px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded
                             bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                             focus:outline-none focus:ring-2 focus:ring-blue-500
                             disabled:opacity-50 disabled:cursor-not-allowed"
                  />
                </div>
              )}
            </div>

            {/* AI Commit Analysis */}
            <div className="space-y-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={options.enableAiCommits || false}
                  onChange={() => handleToggle('enableAiCommits')}
                  disabled={disabled}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded
                           focus:ring-blue-500 focus:ring-2
                           disabled:opacity-50 disabled:cursor-not-allowed"
                />
                <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                  AI Commit Analysis
                </span>
              </label>
              <p className="ml-6 text-xs text-gray-500 dark:text-gray-400">
                Detect AI-generated commits and calculate AI contribution ratio
              </p>
            </div>

            {/* Test Coverage */}
            <div className="space-y-2">
              <label className="block text-sm font-medium text-gray-900 dark:text-gray-100 mb-1">
                Test Coverage Report
              </label>
              <input
                type="text"
                value={options.coverageReportPath || ''}
                onChange={(e) => handleTextChange('coverageReportPath', e.target.value)}
                disabled={disabled}
                placeholder="Path to coverage.json (llvm-cov or Tarpaulin)"
                className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded
                         bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                         placeholder-gray-400 dark:placeholder-gray-500
                         focus:outline-none focus:ring-2 focus:ring-blue-500
                         disabled:opacity-50 disabled:cursor-not-allowed"
              />
              <p className="text-xs text-gray-500 dark:text-gray-400">
                Optional: Path to llvm-cov or Tarpaulin JSON coverage report
              </p>
            </div>
          </div>

          {/* Info about enabled features */}
          {(options.enableDuplicates || options.enableHotspots || options.enableAiCommits || options.coverageReportPath) && (
            <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded border border-blue-200 dark:border-blue-800">
              <p className="text-xs text-blue-800 dark:text-blue-200">
                <strong>Enabled features:</strong>{' '}
                {[
                  options.enableDuplicates && 'Duplicates',
                  options.enableHotspots && 'Hotspots',
                  options.enableAiCommits && 'AI Commits',
                  options.coverageReportPath && 'Coverage',
                ]
                  .filter(Boolean)
                  .join(', ')}
              </p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
