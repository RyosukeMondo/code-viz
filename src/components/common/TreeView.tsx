/**
 * TreeView Component - Simple tree visualization for debugging
 *
 * Shows hierarchical data in a simple collapsible tree format
 */

import { useState } from 'react';
import type { TreeNode } from '@/types/bindings';

interface TreeViewProps {
  data: TreeNode | null;
  maxDepth?: number;
}

interface TreeNodeItemProps {
  node: TreeNode;
  depth: number;
  maxDepth: number;
}

function TreeNodeItem({ node, depth, maxDepth }: TreeNodeItemProps) {
  const [expanded, setExpanded] = useState(depth < 2); // Auto-expand first 2 levels
  const hasChildren = node.children && node.children.length > 0;
  const indent = depth * 20;

  if (depth > maxDepth) {
    return null;
  }

  return (
    <div className="text-sm font-mono">
      <div
        className="flex items-center gap-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded px-2 cursor-pointer"
        style={{ paddingLeft: `${indent}px` }}
        onClick={() => hasChildren && setExpanded(!expanded)}
      >
        {/* Expand/collapse icon */}
        {hasChildren ? (
          <span className="w-4 text-gray-500 dark:text-gray-400">
            {expanded ? '▼' : '▶'}
          </span>
        ) : (
          <span className="w-4"></span>
        )}

        {/* Node type icon */}
        <span className="text-blue-600 dark:text-blue-400">
          {node.type === 'directory' || node.children?.length > 0 ? '📁' : '📄'}
        </span>

        {/* Node name */}
        <span className="flex-1 text-gray-900 dark:text-gray-100">
          {node.name || '(unnamed)'}
        </span>

        {/* LOC */}
        <span className="text-gray-600 dark:text-gray-400 text-xs">
          {node.loc.toLocaleString()} LOC
        </span>

        {/* Complexity */}
        {node.complexity > 0 && (
          <span className="text-orange-600 dark:text-orange-400 text-xs">
            C:{node.complexity}
          </span>
        )}
      </div>

      {/* Children */}
      {expanded && hasChildren && (
        <div>
          {node.children.map((child, index) => (
            <TreeNodeItem
              key={child.id || `${child.name}-${index}`}
              node={child}
              depth={depth + 1}
              maxDepth={maxDepth}
            />
          ))}
        </div>
      )}
    </div>
  );
}

export function TreeView({ data, maxDepth = 10 }: TreeViewProps) {
  if (!data) {
    return (
      <div className="text-center text-gray-500 dark:text-gray-400 py-8">
        No data to display
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 overflow-auto max-h-[600px]">
      <div className="mb-4 text-sm text-gray-600 dark:text-gray-400">
        <div className="font-semibold text-gray-900 dark:text-gray-100 mb-2">Tree Structure</div>
        <div>Total: {data.loc.toLocaleString()} LOC</div>
        <div className="text-xs mt-1 text-gray-500 dark:text-gray-500">
          Click to expand/collapse • Max depth: {maxDepth}
        </div>
      </div>
      <TreeNodeItem node={data} depth={0} maxDepth={maxDepth} />
    </div>
  );
}
