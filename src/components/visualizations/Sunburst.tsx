/**
 * Sunburst visualization component using ECharts
 *
 * This component renders a radial hierarchical sunburst visualization of code metrics,
 * with color-coded complexity and interactive drill-down capabilities.
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
import { SunburstChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { TreemapProps, TreeNode } from '../../types';
import { treeNodeToECharts } from '../../utils/treeTransform';
import { getComplexityLabel } from '../../utils/colors';
import { formatNumber, formatPath } from '../../utils/formatting';

// Register required ECharts components
echarts.use([
  SunburstChart,
  TitleComponent,
  TooltipComponent,
  CanvasRenderer,
]);

/**
 * Sunburst component for visualizing hierarchical code metrics
 */
const Sunburst: React.FC<TreemapProps> = memo(({
  data,
  onNodeClick,
  onNodeHover,
  onNavigateBack,
}) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstanceRef = useRef<echarts.ECharts | null>(null);

  // Transform data to ECharts format
  const echartsData = useMemo(() => {
    if (!data) return null;
    return treeNodeToECharts(data);
  }, [data]);

  // Click handler - center click goes back, other clicks drill down
  const handleClick = useCallback((params: any) => {
    console.log('[Sunburst] Click params:', params);

    // Check if clicking on center (root/first level)
    // If dataIndex is 0 or undefined, it's the center
    if (!params.data || params.dataIndex === undefined || params.dataIndex === 0) {
      console.log('[Sunburst] Center clicked - navigating back');
      if (onNavigateBack) {
        onNavigateBack();
      }
      return;
    }

    if (params.data && onNodeClick) {
      const clickedNode: TreeNode = {
        id: params.data.path || params.data.name,
        name: params.data.name,
        path: params.data.path,
        loc: params.data.value,
        complexity: params.data.complexity,
        type: params.data.type,
        children: params.data.children || [],
        lastModified: '',
      };

      console.log('[Sunburst] Calling onNodeClick with:', clickedNode);
      onNodeClick(clickedNode);
    }
  }, [onNodeClick, onNavigateBack]);

  // Hover handler
  const handleMouseOver = useCallback((params: any) => {
    if (params.data && onNodeHover) {
      const hoveredNode: TreeNode = {
        id: params.data.path || params.data.name,
        name: params.data.name,
        path: params.data.path,
        loc: params.data.value,
        complexity: params.data.complexity,
        type: params.data.type,
        children: params.data.children || [],
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

    // Configure chart options
    const option: EChartsCoreOption = {
      tooltip: {
        trigger: 'item',
        formatter: (params: any) => {
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
          type: 'sunburst',
          data: [echartsData],
          radius: ['20%', '90%'],
          label: {
            rotate: 'radial',
            fontSize: 10,
            minAngle: 15, // Only show label if segment is large enough (15 degrees)
            formatter: (params: any) => {
              // Only show labels for directories or larger files
              if (params.data && params.data.type === 'directory') {
                return params.name;
              }
              // For files, only show if they're reasonably large
              if (params.data && params.data.value > 500) {
                return params.name;
              }
              return ''; // Hide small file labels
            },
          },
          itemStyle: {
            borderWidth: 2,
            borderColor: '#fff',
          },
          emphasis: {
            focus: 'ancestor',
            label: {
              show: true,
              fontSize: 12,
            },
          },
          levels: [
            {
              // Center circle - clickable to go back
              label: {
                show: true,
                fontSize: 14,
                fontWeight: 'bold',
                formatter: () => '← Back',
              },
              itemStyle: {
                color: '#4b5563',
                borderWidth: 3,
              },
            },
            {
              r0: '20%',
              r: '40%',
              label: {
                rotate: 0,
                fontSize: 11,
              },
            },
            {
              r0: '40%',
              r: '70%',
              label: {
                fontSize: 10,
              },
            },
            {
              r0: '70%',
              r: '90%',
              label: {
                position: 'outside',
                silent: false,
                fontSize: 9,
              },
            },
          ],
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
      aria-label="Sunburst chart visualization of code metrics"
    />
  );
});

Sunburst.displayName = 'Sunburst';

export default Sunburst;
