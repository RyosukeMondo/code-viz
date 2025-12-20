/**
 * Data Debugger - Shows raw API response for troubleshooting
 */

import { useState } from 'react';
import type { TreeNode } from '@/types/bindings';

interface DataDebuggerProps {
  data: TreeNode | null;
}

export function DataDebugger({ data }: DataDebuggerProps) {
  const [expanded, setExpanded] = useState(false);

  if (!data) return null;

  const summary = {
    rootId: data.id,
    rootName: data.name,
    rootPath: data.path,
    rootType: data.type,
    rootLoc: data.loc,
    rootComplexity: data.complexity,
    childrenCount: data.children?.length || 0,
    firstChildName: data.children?.[0]?.name || 'none',
    firstChildType: data.children?.[0]?.type || 'none',
  };

  return (
    <div className="fixed bottom-4 right-4 z-50 bg-gray-900 text-green-400 font-mono text-xs rounded-lg shadow-2xl border border-green-500 max-w-md">
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full px-4 py-2 text-left flex items-center justify-between hover:bg-gray-800"
      >
        <span className="font-bold">🐛 API Data Debug</span>
        <span>{expanded ? '▼' : '▶'}</span>
      </button>

      {expanded && (
        <div className="px-4 py-3 max-h-96 overflow-auto border-t border-green-500">
          <div className="mb-3">
            <div className="text-yellow-400 font-bold mb-1">Summary:</div>
            <pre className="text-[10px] leading-relaxed whitespace-pre-wrap break-all">
              {JSON.stringify(summary, null, 2)}
            </pre>
          </div>

          <div>
            <div className="text-yellow-400 font-bold mb-1">Full Data (first 100 lines):</div>
            <pre className="text-[10px] leading-relaxed whitespace-pre-wrap break-all">
              {JSON.stringify(data, null, 2).split('\n').slice(0, 100).join('\n')}
            </pre>
          </div>
        </div>
      )}
    </div>
  );
}
