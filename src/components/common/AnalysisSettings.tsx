/**
 * AnalysisSettings Component
 *
 * Comprehensive settings panel showing all 10 analysis features.
 * Features are organized by:
 * - Always enabled (auto-included in analysis)
 * - Configurable (user can toggle on/off)
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

  /** Dead code enabled state (from store) */
  deadCodeEnabled?: boolean;

  /** Callback to toggle dead code */
  onToggleDeadCode?: () => void;
}

/**
 * Comprehensive settings panel for all analysis features
 */
export function AnalysisSettings({
  options,
  onChange,
  disabled = false,
  deadCodeEnabled = false,
  onToggleDeadCode,
}: AnalysisSettingsProps) {
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

  // Count enabled features
  const enabledCount = [
    true, // Basic Metrics (always on)
    false, // Duplication (disabled - in dev)
    options.enableAiCommits,
    true, // Coupling (always on)
    true, // Code Churn (always on)
    deadCodeEnabled,
    true, // AI Bloat (always on)
    true, // Cognitive Complexity (always on)
    options.enableHotspots,
    !!options.coverageReportPath,
  ].filter(Boolean).length;

  return (
    <div className="mt-4">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        disabled={disabled}
        className="flex items-center gap-2 px-4 py-2 text-sm font-medium
                   bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100
                   border border-gray-300 dark:border-gray-600 rounded-lg
                   hover:bg-gray-200 dark:hover:bg-gray-700
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
        <span>Analysis Features ({enabledCount}/10 enabled)</span>
      </button>

      {isExpanded && (
        <div
          id="analysis-settings-panel"
          className="mt-3 p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm"
        >
          <div className="mb-4">
            <h3 className="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-1">
              All Analysis Features
            </h3>
            <p className="text-xs text-gray-600 dark:text-gray-400">
              Configure which features to enable. Always-enabled features are included automatically.
            </p>
          </div>

          <div className="space-y-4">
            {/* Always Enabled Features */}
            <div className="pb-4 border-b border-gray-200 dark:border-gray-700">
              <h4 className="text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-3">
                Always Enabled (5 features)
              </h4>
              <div className="space-y-2">
                <FeatureItem
                  icon="📊"
                  name="Basic Metrics"
                  description="LOC, file size, function count (excludes comments/blanks)"
                  enabled={true}
                  alwaysOn
                />
                <FeatureItem
                  icon="🔗"
                  name="Coupling Metrics"
                  description="Afferent/efferent coupling, instability index"
                  enabled={true}
                  alwaysOn
                />
                <FeatureItem
                  icon="📈"
                  name="Code Churn"
                  description="Git commit frequency, lines added/deleted over time"
                  enabled={true}
                  alwaysOn
                />
                <FeatureItem
                  icon="🤖"
                  name="AI Bloat Index"
                  description="Detects verbose AI-generated code patterns"
                  enabled={true}
                  alwaysOn
                />
                <FeatureItem
                  icon="🧠"
                  name="Cognitive Complexity"
                  description="SonarSource algorithm measuring code understanding difficulty"
                  enabled={true}
                  alwaysOn
                />
              </div>
            </div>

            {/* Configurable Features */}
            <div>
              <h4 className="text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-3">
                Configurable (4 features)
              </h4>
              <div className="space-y-3">
                {/* Duplicate Detection - IN DEV */}
                <div className="p-3 bg-gray-50 dark:bg-gray-900/50 rounded-lg opacity-60">
                  <label className="flex items-start gap-3 cursor-not-allowed">
                    <input
                      type="checkbox"
                      checked={false}
                      onChange={() => {}}
                      disabled={true}
                      className="mt-0.5 w-4 h-4 text-blue-600 border-gray-300 rounded
                               focus:ring-blue-500 focus:ring-2
                               opacity-50 cursor-not-allowed"
                    />
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-lg">🔄</span>
                        <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                          Code Duplication Detection
                        </span>
                        <span className="text-xs px-2 py-0.5 bg-yellow-100 dark:bg-yellow-900/50 text-yellow-800 dark:text-yellow-200 rounded-full font-semibold">
                          #in dev
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        Finds duplicate code blocks (currently disabled - performance issues)
                      </p>
                    </div>
                  </label>
                </div>

                {/* AI Commit Analysis */}
                <div className="p-3 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
                  <label className="flex items-start gap-3 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={options.enableAiCommits || false}
                      onChange={() => handleToggle('enableAiCommits')}
                      disabled={disabled}
                      className="mt-0.5 w-4 h-4 text-blue-600 border-gray-300 rounded
                               focus:ring-blue-500 focus:ring-2
                               disabled:opacity-50 disabled:cursor-not-allowed"
                    />
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-lg">🤖</span>
                        <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                          AI Commit Analysis
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        Detects AI-generated commits, calculates AI contribution ratio
                      </p>
                    </div>
                  </label>
                </div>

                {/* Dead Code Detection */}
                <div className="p-3 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
                  <label className="flex items-start gap-3 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={deadCodeEnabled}
                      onChange={onToggleDeadCode}
                      disabled={disabled}
                      className="mt-0.5 w-4 h-4 text-blue-600 border-gray-300 rounded
                               focus:ring-blue-500 focus:ring-2
                               disabled:opacity-50 disabled:cursor-not-allowed"
                    />
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-lg">⚡</span>
                        <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                          Dead Code Detection
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        Finds unreachable functions/classes (overlays purple in visualization)
                      </p>
                    </div>
                  </label>
                </div>

                {/* Git Hotspot Detection */}
                <div className="p-3 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
                  <label className="flex items-start gap-3 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={options.enableHotspots || false}
                      onChange={() => handleToggle('enableHotspots')}
                      disabled={disabled}
                      className="mt-0.5 w-4 h-4 text-blue-600 border-gray-300 rounded
                               focus:ring-blue-500 focus:ring-2
                               disabled:opacity-50 disabled:cursor-not-allowed"
                    />
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-lg">🔥</span>
                        <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                          Git Hotspot Detection
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                        Identifies high-risk files (high churn + complexity + size)
                      </p>
                      {options.enableHotspots && (
                        <div className="mt-2">
                          <label className="block text-xs text-gray-600 dark:text-gray-400 mb-1">
                            Max hotspots to report: {options.maxHotspots || 10}
                          </label>
                          <input
                            type="range"
                            min="1"
                            max="50"
                            value={options.maxHotspots || 10}
                            onChange={(e) => handleNumberChange('maxHotspots', parseInt(e.target.value))}
                            disabled={disabled}
                            className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer"
                          />
                        </div>
                      )}
                    </div>
                  </label>
                </div>

                {/* Test Coverage */}
                <div className="p-3 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
                  <div className="flex items-start gap-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-lg">✅</span>
                        <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                          Test Coverage Integration
                        </span>
                      </div>
                      <p className="text-xs text-gray-600 dark:text-gray-400 mb-2">
                        Integrates llvm-cov or Tarpaulin coverage reports
                      </p>
                      <input
                        type="text"
                        value={options.coverageReportPath || ''}
                        onChange={(e) => handleTextChange('coverageReportPath', e.target.value)}
                        disabled={disabled}
                        placeholder="Path to coverage.json (optional)"
                        className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded
                                 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                                 placeholder-gray-400 dark:placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-blue-500
                                 disabled:opacity-50 disabled:cursor-not-allowed"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Summary */}
          <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
            <div className="flex items-start gap-3">
              <svg className="w-5 h-5 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <div className="flex-1 text-xs text-blue-900 dark:text-blue-100">
                <p className="font-semibold mb-1">How Features Appear in Visualization:</p>
                <ul className="space-y-1 list-disc list-inside">
                  <li><strong>Always-on metrics</strong> (LOC, coupling, churn, etc.) affect file size and color</li>
                  <li><strong>Dead Code</strong> overlays purple tint on affected files</li>
                  <li><strong>Duplicates/Hotspots</strong> appear in detail panel when selecting files</li>
                  <li><strong>Coverage</strong> shows percentage in detail panel</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Feature item component (read-only display)
 */
function FeatureItem({
  icon,
  name,
  description,
  enabled,
  alwaysOn = false,
}: {
  icon: string;
  name: string;
  description: string;
  enabled: boolean;
  alwaysOn?: boolean;
}) {
  return (
    <div className="flex items-start gap-3 p-2 rounded">
      <span className="text-lg flex-shrink-0">{icon}</span>
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
            {name}
          </span>
          {alwaysOn && (
            <span className="px-1.5 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded">
              Always On
            </span>
          )}
        </div>
        <p className="text-xs text-gray-600 dark:text-gray-400 mt-0.5">
          {description}
        </p>
      </div>
      {enabled && (
        <svg className="w-4 h-4 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
          <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
        </svg>
      )}
    </div>
  );
}
