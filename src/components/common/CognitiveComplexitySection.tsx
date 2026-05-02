import type { CognitiveComplexity } from '@/types/bindings';

interface CognitiveComplexitySectionProps {
  cognitiveComplexity: CognitiveComplexity;
}

export function CognitiveComplexitySection({ cognitiveComplexity }: CognitiveComplexitySectionProps) {
  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        Cognitive Complexity
      </h3>
      <div className="space-y-3">
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Total</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {cognitiveComplexity.totalComplexity}
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Average</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {cognitiveComplexity.averageComplexity.toFixed(1)}
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Maximum</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {cognitiveComplexity.maxComplexity}
          </span>
        </div>
      </div>
    </section>
  );
}
