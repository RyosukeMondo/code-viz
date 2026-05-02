import type { TestCoverage } from '@/types/bindings';

interface TestCoverageSectionProps {
  testCoverage: TestCoverage;
}

export function TestCoverageSection({ testCoverage }: TestCoverageSectionProps) {
  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        Test Coverage
      </h3>
      <div className="space-y-3">
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Line Coverage</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {testCoverage.lineCoverage.toFixed(1)}%
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Lines Covered</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {testCoverage.linesCovered} / {testCoverage.totalLines}
          </span>
        </div>
        {testCoverage.branchCoverage !== undefined && (
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-600 dark:text-gray-400">Branch Coverage</span>
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
              {testCoverage.branchCoverage.toFixed(1)}%
            </span>
          </div>
        )}
      </div>
    </section>
  );
}
