import type { TreeNode } from '@/types/bindings';
import { formatNumber } from '@/utils/formatting';
import { complexityToColor } from '@/utils/colors';

interface BasicMetricsProps {
  node: TreeNode;
}

export function BasicMetricsSection({ node }: BasicMetricsProps) {
  const complexityColor = complexityToColor(node.complexity);

  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        Basic Metrics
      </h3>
      <div className="space-y-3">
        {node.type === 'file' && node.language && (
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-600 dark:text-gray-400">Language</span>
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100 capitalize">
              {node.language}
            </span>
          </div>
        )}

        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Lines of Code</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {formatNumber(node.loc)}
          </span>
        </div>

        {node.type === 'file' && node.sizeBytes !== undefined && (
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-600 dark:text-gray-400">File Size</span>
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
              {(node.sizeBytes / 1024).toFixed(2)} KB
            </span>
          </div>
        )}

        {node.type === 'file' && node.functionCount !== undefined && (
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-600 dark:text-gray-400">Functions</span>
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
              {formatNumber(node.functionCount)}
            </span>
          </div>
        )}

        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Complexity Score</span>
          <div className="flex items-center gap-2">
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
              {formatNumber(node.complexity)}
            </span>
            <div
              className="w-3 h-3 rounded-full"
              style={{ backgroundColor: complexityColor }}
              title={`Complexity: ${node.complexity}`}
              aria-label={`Complexity indicator: ${node.complexity}`}
            />
          </div>
        </div>

        {node.type === 'directory' && (
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
            <span className="text-sm text-gray-600 dark:text-gray-400">
              {node.children.length === 1 ? 'Child' : 'Children'}
            </span>
            <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
              {formatNumber(node.children.length)}
            </span>
          </div>
        )}
      </div>
    </section>
  );
}
