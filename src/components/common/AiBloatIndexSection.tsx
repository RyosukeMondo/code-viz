interface AiBloatIndexSectionProps {
  aiBloatIndex: number;
}

export function AiBloatIndexSection({ aiBloatIndex }: AiBloatIndexSectionProps) {
  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        AI Bloat Index
      </h3>
      <div className="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <span className="text-sm text-gray-600 dark:text-gray-400">Bloat Score</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {aiBloatIndex.toFixed(1)}%
          </span>
        </div>
      </div>
    </section>
  );
}
