import type { CouplingMetrics } from '@/types/bindings';

interface CouplingMetricsSectionProps {
  coupling: CouplingMetrics;
}

export function CouplingMetricsSection({ coupling }: CouplingMetricsSectionProps) {
  return (
    <section>
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
        Coupling Metrics
      </h3>
      <div className="space-y-3">
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Afferent (Incoming)</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {coupling.afferentCoupling}
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Efferent (Outgoing)</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {coupling.efferentCoupling}
          </span>
        </div>
        <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
          <span className="text-sm text-gray-600 dark:text-gray-400">Instability</span>
          <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
            {coupling.instability.toFixed(2)}
          </span>
        </div>
      </div>
    </section>
  );
}
