/**
 * Treemap visualization component using ECharts
 *
 * This component renders a hierarchical treemap visualization of code metrics,
 * with color-coded complexity and interactive drill-down capabilities.
 *
 * Features:
 * - Color mapping by complexity score (green=low, yellow=medium, red=high)
 * - Click handlers for drill-down navigation
 * - Hover handlers for tooltips and selection
 * - Smooth animations for transitions
 * - Responsive to window resize
 * - Performance optimized with React.memo
 */

import React, { useEffect, useRef, memo } from 'react';
import * as echarts from 'echarts/core';
import type { EChartsCoreOption } from 'echarts/core';
import { TreemapChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { TreemapProps } from '../../types';
import { treeNodeToECharts, filterByPath } from '../../utils/treeTransform';
import { getComplexityLabel } from '../../utils/colors';
import { formatNumber, formatPath } from '../../utils/formatting';

// Register required ECharts components (tree-shaking)
echarts.use([
  TreemapChart,
  TitleComponent,
  TooltipComponent,
  GridComponent,
  CanvasRenderer,
]);

/**
 * Treemap component for visualizing hierarchical code metrics
 */
const TreemapComponent: React.FC<TreemapProps> = ({
  data,
  drillDownPath = [],
  onNodeClick,
  onNodeHover,
  width = '100%',
  height = 600,
}) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstanceRef = useRef<echarts.ECharts | null>(null);

  useEffect(() => {
    if (!chartRef.current || !data) {
      return;
    }

    // Initialize ECharts instance if not already created
    if (!chartInstanceRef.current) {
      chartInstanceRef.current = echarts.init(chartRef.current);
    }

    const chart = chartInstanceRef.current;

    // Filter data based on drill-down path
    const filteredData = drillDownPath.length > 0
      ? filterByPath(data, drillDownPath)
      : data;

    if (!filteredData) {
      console.warn('Filtered data is null, path not found:', drillDownPath);
      return;
    }

    // Transform TreeNode to ECharts format
    const echartsData = treeNodeToECharts(filteredData);

    // Configure ECharts treemap options
    const option: EChartsCoreOption = {
      tooltip: {
        formatter: (info: any) => {
          const { name, value, complexity, path, type } = info.data;
          const complexityLabel = getComplexityLabel(complexity);

          return `
            <div style="padding: 8px;">
              <div style="font-weight: 600; margin-bottom: 4px;">${formatPath(name)}</div>
              <div style="color: #64748b; font-size: 12px; margin-bottom: 8px;">${type}</div>
              <div style="display: flex; flex-direction: column; gap: 4px;">
                <div style="display: flex; justify-content: space-between; gap: 16px;">
                  <span style="color: #64748b;">Lines:</span>
                  <span style="font-weight: 500;">${formatNumber(value)}</span>
                </div>
                <div style="display: flex; justify-content: space-between; gap: 16px;">
                  <span style="color: #64748b;">Complexity:</span>
                  <span style="font-weight: 500;">${complexity.toFixed(1)} (${complexityLabel})</span>
                </div>
                <div style="color: #64748b; font-size: 11px; margin-top: 4px; max-width: 300px; word-break: break-all;">
                  ${path}
                </div>
              </div>
            </div>
          `;
        },
        backgroundColor: '#ffffff',
        borderColor: '#e2e8f0',
        borderWidth: 1,
        textStyle: {
          color: '#1e293b',
        },
      },
      series: [
        {
          type: 'treemap',
          data: [echartsData],
          leafDepth: 1,
          roam: false,
          nodeClick: false, // Disable ECharts default click (we handle it manually)
          breadcrumb: {
            show: false, // We use our custom Breadcrumb component
          },
          label: {
            show: true,
            formatter: (params: any) => {
              const { name, value } = params.data;
              // Show name and LOC for rectangles large enough
              if (params.rect.width > 60 && params.rect.height > 40) {
                return `{name|${name}}\n{loc|${formatNumber(value)} LOC}`;
              } else if (params.rect.width > 40 && params.rect.height > 30) {
                return `{name|${name}}`;
              }
              return '';
            },
            rich: {
              name: {
                fontSize: 14,
                fontWeight: 600,
                color: '#1e293b',
                lineHeight: 20,
              },
              loc: {
                fontSize: 11,
                color: '#64748b',
                lineHeight: 16,
              },
            },
            overflow: 'truncate',
            ellipsis: '...',
          },
          upperLabel: {
            show: true,
            height: 30,
            formatter: (params: any) => {
              return params.name;
            },
            textStyle: {
              fontSize: 13,
              fontWeight: 600,
              color: '#1e293b',
            },
          },
          itemStyle: {
            borderColor: '#ffffff',
            borderWidth: 2,
            gapWidth: 2,
          },
          emphasis: {
            itemStyle: {
              borderColor: '#3b82f6',
              borderWidth: 3,
              shadowBlur: 10,
              shadowColor: 'rgba(59, 130, 246, 0.3)',
            },
            label: {
              show: true,
            },
          },
          // Smooth animations
          animation: true,
          animationDuration: 500,
          animationEasing: 'cubicInOut',
        },
      ],
    };

    // Set options with merge to preserve animation state
    chart.setOption(option, true);

    // Handle click events
    const handleClick = (params: any) => {
      if (params.data && onNodeClick) {
        // Convert ECharts data back to TreeNode format for the callback
        const clickedNode = {
          id: params.data.path,
          name: params.data.name,
          path: params.data.path,
          loc: params.data.value,
          complexity: params.data.complexity,
          type: params.data.type,
          children: params.data.children || [],
          lastModified: '', // Not available in ECharts data
        };
        onNodeClick(clickedNode);
      }
    };

    // Handle hover events
    const handleMouseOver = (params: any) => {
      if (params.data && onNodeHover) {
        const hoveredNode = {
          id: params.data.path,
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
    };

    const handleMouseOut = () => {
      if (onNodeHover) {
        onNodeHover(null);
      }
    };

    // Register event handlers
    chart.on('click', handleClick);
    chart.on('mouseover', handleMouseOver);
    chart.on('mouseout', handleMouseOut);

    // Handle window resize
    const handleResize = () => {
      chart.resize();
    };

    window.addEventListener('resize', handleResize);

    // Cleanup function
    return () => {
      chart.off('click', handleClick);
      chart.off('mouseover', handleMouseOver);
      chart.off('mouseout', handleMouseOut);
      window.removeEventListener('resize', handleResize);
    };
  }, [data, drillDownPath, onNodeClick, onNodeHover]);

  // Dispose chart instance on unmount
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
        width: typeof width === 'number' ? `${width}px` : width,
        height: typeof height === 'number' ? `${height}px` : height,
      }}
      className="treemap-container"
    />
  );
};

// Memoize component to prevent unnecessary re-renders
export const Treemap = memo(TreemapComponent, (prevProps, nextProps) => {
  // Custom comparison to optimize re-renders
  return (
    prevProps.data === nextProps.data &&
    prevProps.drillDownPath === nextProps.drillDownPath &&
    prevProps.width === nextProps.width &&
    prevProps.height === nextProps.height &&
    prevProps.onNodeClick === nextProps.onNodeClick &&
    prevProps.onNodeHover === nextProps.onNodeHover
  );
});

Treemap.displayName = 'Treemap';
