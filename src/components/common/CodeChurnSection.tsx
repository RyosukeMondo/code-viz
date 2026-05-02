import type { CodeChurn } from '@/types/bindings';
import { formatNumber } from '@/utils/formatting';

interface CodeChurnSectionProps {
  codeChurn: CodeChurn;
}

export function CodeChurnSection({ codeChurn }: CodeChurnSectionProps) {
  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        Code Churn
      </h3>
      <div className="space-y-3">
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Lines Added</span>
          <span className="text-sm font-semibold text-green-600 dark:text-green-400">
            +{formatNumber(codeChurn.addedLines)}
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Lines Deleted</span>
          <span className="text-sm font-semibold text-red-600 dark:text-red-400">
            -{formatNumber(codeChurn.deletedLines)}
          </span>
        </div>
      </div>
    </section>
  );
}
