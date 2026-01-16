/**
 * Circle Packing visualization component using ECharts
 *
 * This component renders a circle packing visualization of code metrics,
 * with nested circles representing the hierarchical structure.
 *
 * Features:
 * - Color mapping by complexity score (green=low, yellow=medium, red=high)
 * - Click handlers for drill-down navigation
 * - Hover handlers for tooltips
 * - Smooth animations for transitions
 * - Responsive to window resize
 */

import React, { useEffect, useRef, memo, useMemo, useCallback } from 'react';
import * as echarts from 'echarts/core';
import type { EChartsCoreOption } from 'echarts/core';
import { TreeChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type {
  TreemapProps,
  TreeNode,
  EChartsHoverParams,
  EChartsTooltipParams,
  EChartsLabelParams,
} from '../../types';
import { treeNodeToECharts } from '../../utils/treeTransform';
import { getComplexityLabel } from '../../utils/colors';
import { formatNumber, formatPath } from '../../utils/formatting';

// Register required ECharts components
echarts.use([
  TreeChart,
  TitleComponent,
  TooltipComponent,
  CanvasRenderer,
]);

/**
 * CirclePacking component for visualizing hierarchical code metrics
 */
const CirclePacking: React.FC<TreemapProps> = memo(({
  data,
  onNodeClick,
  onNodeHover,
}) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstanceRef = useRef<echarts.ECharts | null>(null);

  // Transform data to ECharts format
  const echartsData = useMemo(() => {
    if (!data) return null;
    return treeNodeToECharts(data);
  }, [data]);

  // Click handler
  const handleClick = useCallback((params: Record<string, unknown>) => {
    const typedParams = params as unknown as EChartsHoverParams;
    console.log('[CirclePacking] Click params:', typedParams);

    if (typedParams.data && onNodeClick) {
      const clickedNode: TreeNode = {
        id: typedParams.data.path || typedParams.data.name,
        name: typedParams.data.name,
        path: typedParams.data.path,
        loc: typedParams.data.value,
        complexity: typedParams.data.complexity ?? 0,
        type: typedParams.data.type,
        children: (typedParams.data.children || []) as TreeNode[],
        lastModified: '',
      };

      console.log('[CirclePacking] Calling onNodeClick with:', clickedNode);
      onNodeClick(clickedNode);
    }
  }, [onNodeClick]);

  // Hover handler
  const handleMouseOver = useCallback((params: Record<string, unknown>) => {
    const typedParams = params as unknown as EChartsHoverParams;
    if (typedParams.data && onNodeHover) {
      const hoveredNode: TreeNode = {
        id: typedParams.data.path || typedParams.data.name,
        name: typedParams.data.name,
        path: typedParams.data.path,
        loc: typedParams.data.value,
        complexity: typedParams.data.complexity ?? 0,
        type: typedParams.data.type,
        children: (typedParams.data.children || []) as TreeNode[],
        lastModified: '',
      };
      onNodeHover(hoveredNode);
    }
  }, [onNodeHover]);

  useEffect(() => {
    if (!chartRef.current || !echartsData) {
      return;
    }

    // Initialize chart
    if (!chartInstanceRef.current) {
      chartInstanceRef.current = echarts.init(chartRef.current);
    }

    const chart = chartInstanceRef.current;

    // Configure chart options for circle packing layout
    const option: EChartsCoreOption = {
      tooltip: {
        trigger: 'item',
        formatter: (params: EChartsTooltipParams) => {
          const { name, value, data } = params;
          const complexity = data?.complexity ?? 0;
          const type = data?.type ?? 'unknown';
          const path = data?.path ?? '';

          return `
            <div style="padding: 8px;">
              <div style="font-weight: bold; margin-bottom: 4px;">${formatPath(name)}</div>
              <div>Type: ${type}</div>
              <div>LOC: ${formatNumber(value)}</div>
              <div>Complexity: ${complexity}% (${getComplexityLabel(complexity)})</div>
              ${path ? `<div style="font-size: 11px; color: #888; margin-top: 4px;">${formatPath(path)}</div>` : ''}
            </div>
          `;
        },
        backgroundColor: 'rgba(255, 255, 255, 0.95)',
        borderColor: '#ddd',
        borderWidth: 1,
        textStyle: {
          color: '#333',
          fontSize: 12,
        },
      },
      series: [
        {
          type: 'tree',
          data: [echartsData],
          layout: 'radial',
          symbol: 'circle',
          symbolSize: (value: number) => {
            // Scale circle size based on LOC (value)
            // Use logarithmic scale to handle large variations
            return Math.max(8, Math.min(100, Math.log(value + 1) * 8));
          },
          label: {
            show: true,
            position: 'inside',
            fontSize: 10,
            formatter: (params: EChartsLabelParams) => {
              const name = params.name || '';
              // Show label only if circle is large enough
              return (params.symbolSize && params.symbolSize > 30) ? name : '';
            },
          },
          itemStyle: {
            borderWidth: 2,
            borderColor: '#fff',
          },
          lineStyle: {
            color: '#ccc',
            width: 1,
          },
          emphasis: {
            focus: 'descendant',
            scale: 1.2,
            itemStyle: {
              borderColor: '#333',
              borderWidth: 3,
            },
          },
          expandAndCollapse: true,
          animationDuration: 550,
          animationDurationUpdate: 750,
        },
      ],
    };

    chart.setOption(option);

    // Attach event handlers
    chart.off('click');
    chart.on('click', handleClick);

    if (onNodeHover) {
      chart.off('mouseover');
      chart.on('mouseover', handleMouseOver);
    }

    // Handle resize
    const handleResize = () => {
      chart.resize();
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      chart.off('click');
      chart.off('mouseover');
    };
  }, [echartsData, handleClick, handleMouseOver, onNodeHover]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (chartInstanceRef.current) {
        chartInstanceRef.current.dispose();
        chartInstanceRef.current = null;
      }
    };
  }, []);

  return (
    <div
      ref={chartRef}
      style={{
        width: '100%',
        height: '100%',
        minHeight: '400px',
      }}
      role="img"
      aria-label="Circle packing visualization of code metrics"
    />
  );
});

CirclePacking.displayName = 'CirclePacking';

export default CirclePacking;
