/**
 * Progress Bar Component
 *
 * Shows analysis progress with percentage and status message
 */

interface ProgressBarProps {
  /** Progress percentage (0-100) */
  progress: number;
  /** Status message */
  message?: string;
  /** Show indeterminate progress */
  indeterminate?: boolean;
}

export function ProgressBar({ progress, message, indeterminate = false }: ProgressBarProps) {
  return (
    <div className="w-full space-y-2">
      {/* Progress bar */}
      <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-300 ${
            indeterminate
              ? 'bg-blue-500 animate-pulse w-full'
              : 'bg-blue-600 dark:bg-blue-500'
          }`}
          style={{ width: indeterminate ? '100%' : `${Math.min(100, Math.max(0, progress))}%` }}
        />
      </div>

      {/* Status text */}
      <div className="flex justify-between text-sm">
        <span className="text-gray-600 dark:text-gray-400">
          {message || 'Processing...'}
        </span>
        {!indeterminate && (
          <span className="text-gray-700 dark:text-gray-300 font-medium">
            {Math.round(progress)}%
          </span>
        )}
      </div>
    </div>
  );
}
