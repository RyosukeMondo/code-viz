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
 *
 * Performance optimizations:
 * - React.memo with custom comparison to prevent unnecessary re-renders
 * - useMemo for expensive filtering and transformation operations
 * - useCallback for stable event handler references
 * - Progressive/lazy rendering for large datasets (>50K files)
 * - WeakMap-based caching in transformation utilities
 * - Optimized ECharts rendering settings for large trees
 */

import React, { useEffect, useRef, memo, useMemo, useCallback } from 'react';
import * as echarts from 'echarts/core';
import type { EChartsCoreOption } from 'echarts/core';
import { TreemapChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { TreemapProps, TreeNode } from '../../types';
import { treeNodeToECharts, getFileCount } from '../../utils/treeTransform';
import { getComplexityLabel, deadCodeBorderColor } from '../../utils/colors';
import { formatNumber, formatPath } from '../../utils/formatting';
import { useDeadCodeEnabled } from '../../store/analysisStore';

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
  onNavigateBack,
  width = '100%',
  height = 600,
}) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstanceRef = useRef<echarts.ECharts | null>(null);
  const [selectedNodeIndex, setSelectedNodeIndex] = React.useState<number>(0);

  // Subscribe to dead code overlay state
  const deadCodeEnabled = useDeadCodeEnabled();

  // The data prop already comes pre-filtered from AnalysisView
  // No need to filter again here

  // Memoize ECharts transformation (expensive for large datasets)
  const echartsData = useMemo(() => {
    if (!data) return null;
    const transformed = treeNodeToECharts(data);
    console.log('[Treemap] Transformed echartsData root:', {
      name: transformed.name,
      path: transformed.path,
      type: transformed.type,
      value: transformed.value,
      complexity: transformed.complexity,
      childrenCount: transformed.children?.length
    });
    return transformed;
  }, [data]);

  // Calculate file count to determine if we need lazy rendering
  const fileCount = useMemo(() => {
    if (!data) return 0;
    return getFileCount(data);
  }, [data]);

  // Determine if we should use lazy rendering (for datasets > 50K files)
  const shouldUseLazyRendering = fileCount > 50000;

  // Memoize event handlers to prevent unnecessary re-renders
  const handleClick = useCallback((params: any) => {
    console.log('[Treemap] FULL params object:', params);
    console.log('[Treemap] params.treePathInfo:', params.treePathInfo);

    // ECharts treemap stores actual node data in the last item of treePathInfo array
    const actualNode = params.treePathInfo?.[params.treePathInfo.length - 1];

    if (actualNode && onNodeClick) {
      const clickedNode: TreeNode = {
        id: actualNode.path || actualNode.name,
        name: actualNode.name,
        path: actualNode.path,
        loc: actualNode.value,
        complexity: actualNode.complexity,
        type: actualNode.type,
        children: actualNode.children || [],
        lastModified: '',
      };

      console.log('[Treemap] ✅ Extracted node from treePathInfo:', clickedNode);
      onNodeClick(clickedNode);
    }
  }, [onNodeClick]);

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

  const handleMouseOut = useCallback(() => {
    if (onNodeHover) {
      onNodeHover(null);
    }
  }, [onNodeHover]);

  /**
   * Get flattened list of visible nodes for keyboard navigation
   */
  const getVisibleNodes = useCallback((): TreeNode[] => {
    if (!data) return [];

    const nodes: TreeNode[] = [];
    const traverse = (node: TreeNode) => {
      if (node.type === 'file' || (node.type === 'directory' && node.children.length > 0)) {
        nodes.push(node);
      }
      if (node.children && node.children.length > 0) {
        node.children.forEach(traverse);
      }
    };

    traverse(data);
    return nodes;
  }, [data]);

  /**
   * Handle keyboard navigation
   */
  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent<HTMLDivElement>) => {
      const visibleNodes = getVisibleNodes();

      if (visibleNodes.length === 0) return;

      switch (event.key) {
        case 'Enter':
        case ' ': // Space key
          event.preventDefault();
          // Select the currently focused node
          if (visibleNodes[selectedNodeIndex] && onNodeClick) {
            onNodeClick(visibleNodes[selectedNodeIndex]);
          }
          break;

        case 'Escape':
          event.preventDefault();
          // Navigate back
          if (onNavigateBack) {
            onNavigateBack();
          }
          break;

        case 'ArrowRight':
        case 'ArrowDown':
          event.preventDefault();
          // Move to next node
          setSelectedNodeIndex((prev) =>
            prev < visibleNodes.length - 1 ? prev + 1 : prev
          );
          break;

        case 'ArrowLeft':
        case 'ArrowUp':
          event.preventDefault();
          // Move to previous node
          setSelectedNodeIndex((prev) => (prev > 0 ? prev - 1 : prev));
          break;

        case 'Home':
          event.preventDefault();
          // Jump to first node
          setSelectedNodeIndex(0);
          break;

        case 'End':
          event.preventDefault();
          // Jump to last node
          setSelectedNodeIndex(visibleNodes.length - 1);
          break;

        default:
          break;
      }
    },
    [selectedNodeIndex, getVisibleNodes, onNodeClick, onNavigateBack]
  );

  // Reset selected index when drill-down path changes
  useEffect(() => {
    setSelectedNodeIndex(0);
  }, [drillDownPath]);

  useEffect(() => {
    if (!chartRef.current || !data || !echartsData) {
      return;
    }

    // Initialize ECharts instance if not already created
    if (!chartInstanceRef.current) {
      chartInstanceRef.current = echarts.init(chartRef.current);
    }

    const chart = chartInstanceRef.current;

    if (!data) {
      console.warn('Data is null');
      return;
    }

    // Configure ECharts treemap options
    const option: EChartsCoreOption = {
      // Enable progressive rendering for large datasets
      progressive: shouldUseLazyRendering ? 500 : undefined,
      progressiveThreshold: shouldUseLazyRendering ? 1000 : undefined,
      progressiveChunkMode: shouldUseLazyRendering ? 'mod' : undefined,

      tooltip: {
        formatter: (info: any) => {
          const { name, value, complexity, path, type, deadCodeRatio } = info.data;
          const complexityValue = complexity ?? 0;
          const complexityLabel = getComplexityLabel(complexityValue);

          let deadCodeSection = '';
          if (deadCodeEnabled && deadCodeRatio !== undefined && deadCodeRatio > 0) {
            const percentage = (deadCodeRatio * 100).toFixed(1);
            deadCodeSection = `
              <div style="display: flex; justify-content: space-between; gap: 16px;">
                <span style="color: #64748b;">Dead Code:</span>
                <span style="font-weight: 500; color: ${deadCodeBorderColor(deadCodeRatio)};">${percentage}%</span>
              </div>
            `;
          }

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
                  <span style="font-weight: 500;">${complexityValue.toFixed(1)} (${complexityLabel})</span>
                </div>
                ${deadCodeSection}
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
          // Show children directly to avoid extra root wrapper level
          // This prevents clicking on children from returning the root node
          data: echartsData?.children && echartsData.children.length > 0
            ? echartsData.children
            : [echartsData],
          leafDepth: 1,
          roam: false,
          nodeClick: false, // Disable ECharts default click (we handle it manually)
          breadcrumb: {
            show: false, // We use our custom Breadcrumb component
          },
          label: {
            show: true,
            formatter: (params: any) => {
              if (!params.data || !params.rect) return '';
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
            borderColor: (params: any) => {
              // Apply dead code border color if overlay is enabled and node has dead code
              if (deadCodeEnabled && params.data?.deadCodeRatio) {
                return deadCodeBorderColor(params.data.deadCodeRatio);
              }
              return '#ffffff';
            },
            borderWidth: (params: any) => {
              // Use thicker border for nodes with dead code when overlay is enabled
              if (deadCodeEnabled && params.data?.deadCodeRatio) {
                return 3;
              }
              return 2;
            },
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
    // For large datasets, use notMerge=true to improve performance
    chart.setOption(option, shouldUseLazyRendering ? false : true);

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
  }, [data, echartsData, shouldUseLazyRendering, deadCodeEnabled, handleClick, handleMouseOver, handleMouseOut]);

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
      data-testid="treemap-node"
      tabIndex={0}
      role="application"
      aria-label="Interactive treemap visualization - use arrow keys to navigate, Enter to select, Escape to go back"
      onKeyDown={handleKeyDown}
      style={{
        width: typeof width === 'number' ? `${width}px` : width,
        height: typeof height === 'number' ? `${height}px` : height,
      }}
      className="treemap-container focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 rounded-lg"
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
    prevProps.onNodeHover === nextProps.onNodeHover &&
    prevProps.onNavigateBack === nextProps.onNavigateBack
  );
});

Treemap.displayName = 'Treemap';
