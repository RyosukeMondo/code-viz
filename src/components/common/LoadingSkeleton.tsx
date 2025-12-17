/**
 * LoadingSkeleton Components
 *
 * Provides skeleton loading UI components for different parts of the application.
 * Skeletons show placeholders while content is loading, improving perceived performance.
 *
 * Features:
 * - TreemapSkeleton for visualization loading state
 * - Animated pulse effect
 * - Dark mode support
 */

/**
 * TreemapSkeleton - Loading skeleton for treemap visualization
 */
export function TreemapSkeleton() {
  return (
    <div className="h-full w-full flex flex-col gap-4 p-6 animate-pulse">
      {/* Breadcrumb skeleton */}
      <div className="flex gap-2 items-center">
        <div className="h-8 w-8 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div className="h-4 w-4 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
        <div className="h-8 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div className="h-4 w-4 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
        <div className="h-8 w-32 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>

      {/* Main treemap skeleton - grid of rectangles simulating treemap layout */}
      <div className="flex-1 bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden">
        <div className="h-full grid grid-cols-3 gap-1 p-1">
          {/* Large rectangle - top left */}
          <div className="col-span-2 row-span-2 bg-gray-200 dark:bg-gray-700 rounded"></div>

          {/* Medium rectangles - right side */}
          <div className="bg-gray-300 dark:bg-gray-600 rounded"></div>
          <div className="bg-gray-300 dark:bg-gray-600 rounded"></div>

          {/* Small rectangles - bottom */}
          <div className="bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div className="bg-gray-300 dark:bg-gray-600 rounded"></div>
          <div className="bg-gray-200 dark:bg-gray-700 rounded"></div>

          {/* More small rectangles */}
          <div className="bg-gray-300 dark:bg-gray-600 rounded"></div>
          <div className="bg-gray-200 dark:bg-gray-700 rounded"></div>
          <div className="bg-gray-300 dark:bg-gray-600 rounded"></div>
        </div>
      </div>

      {/* Stats skeleton - bottom */}
      <div className="flex gap-4">
        <div className="flex-1 h-16 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
        <div className="flex-1 h-16 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
        <div className="flex-1 h-16 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
      </div>
    </div>
  );
}

/**
 * AnalysisLoadingSkeleton - Loading skeleton for the entire analysis view
 */
export function AnalysisLoadingSkeleton() {
  return (
    <div className="absolute inset-0 bg-white dark:bg-gray-900">
      <div className="h-full flex flex-col">
        {/* Header skeleton is not needed as the real header stays visible */}

        {/* Main content with centered spinner and message */}
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <div className="inline-block animate-spin rounded-full h-16 w-16 border-b-4 border-blue-600 mb-6"></div>
            <h3 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2">
              Analyzing Repository
            </h3>
            <p className="text-gray-600 dark:text-gray-400 mb-4">
              Scanning files and calculating complexity metrics...
            </p>

            {/* Progress indicators */}
            <div className="space-y-3 max-w-md mx-auto">
              <div className="flex items-center gap-3 animate-pulse">
                <div className="h-2 flex-1 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                  <div className="h-full w-2/3 bg-blue-500 rounded-full"></div>
                </div>
                <span className="text-sm text-gray-500 dark:text-gray-500">
                  Scanning...
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

/**
 * Skeleton - Generic skeleton box component
 */
interface SkeletonProps {
  className?: string;
}

export function Skeleton({ className = '' }: SkeletonProps) {
  return (
    <div
      className={`animate-pulse bg-gray-200 dark:bg-gray-700 rounded ${className}`}
    ></div>
  );
}
