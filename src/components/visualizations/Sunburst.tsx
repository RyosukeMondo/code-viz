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

import React, { useEffect, useRef, memo, useMemo, useCallback, useState } from 'react';
import * as echarts from 'echarts/core';
import type { EChartsCoreOption } from 'echarts/core';
import { SunburstChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { TreemapProps, TreeNode } from '../../types';
import { treeNodeToEChartsWithDepth } from '../../utils/treeTransform';
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
  const [depth, setDepth] = useState(1);

  // Transform data to ECharts format with controlled depth
  const echartsData = useMemo(() => {
    if (!data) return null;
    return treeNodeToEChartsWithDepth(data, depth);
  }, [data, depth]);

  // Recursive helper to find a node by path/name anywhere in the tree
  const findNodeInTree = useCallback((root: TreeNode, targetPath: string, targetName: string): TreeNode | null => {
    if (root.path === targetPath || root.name === targetName) {
      return root;
    }
    if (root.children) {
      for (const child of root.children) {
        const found = findNodeInTree(child, targetPath, targetName);
        if (found) return found;
      }
    }
    return null;
  }, []);

  // Click handler - center always goes back, clicking current node goes up
  const handleClick = useCallback((params: any) => {
    console.log('[Sunburst] Click params:', params);
    console.log('[Sunburst] Current data:', { name: data?.name, path: data?.path });

    // ALWAYS navigate back if no data (center click) OR clicking on root level
    if (!params.data || params.dataIndex === 0) {
      console.log('[Sunburst] Center/root clicked - navigating back');
      if (onNavigateBack) {
        onNavigateBack();
      }
      return;
    }

    // Check if clicking on the current directory (same as data)
    // If so, navigate up instead of trying to drill into itself
    if (params.data && data &&
        (params.data.name === data.name && params.data.path === data.path)) {
      console.log('[Sunburst] Clicked current directory - navigating back');
      if (onNavigateBack) {
        onNavigateBack();
      }
      return;
    }

    // Normal drill-down for other nodes
    if (params.data && onNodeClick && data) {
      // Recursively search for the clicked node in the full tree (supports depth > 1)
      // The params.data is from ECharts (depth-limited), but we need the full node with all descendants
      const clickedNode = findNodeInTree(data, params.data.path, params.data.name);

      if (clickedNode) {
        console.log('[Sunburst] Calling onNodeClick with full node:', clickedNode);
        onNodeClick(clickedNode);
      } else {
        console.warn('[Sunburst] Could not find clicked node in original data');
      }
    }
  }, [data, onNodeClick, onNavigateBack, findNodeInTree]);

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
          radius: [0, '90%'], // Start from center (0) for back button visibility
          // Only show outer circle - zoom to clicked node for cleaner view
          nodeClick: 'rootToNode', // Click zooms to that node (shows only its children)
          sort: 'desc', // Sort by size (largest first)
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
              // Center circle - ALWAYS clickable (smaller, always visible)
              r0: 0,
              r: '12%', // Small center circle for back button
              label: {
                show: true,
                fontSize: 13,
                fontWeight: 'bold',
                formatter: () => '← Back',
                color: '#fff',
              },
              itemStyle: {
                color: '#4b5563',
                borderWidth: 3,
                borderColor: '#374151',
              },
              emphasis: {
                itemStyle: {
                  color: '#1f2937',
                },
              },
            },
            {
              // First data level - starts after center with gap
              r0: '15%',
              r: '35%',
              label: {
                rotate: 0,
                fontSize: 11,
              },
            },
            {
              r0: '35%',
              r: '65%',
              label: {
                fontSize: 10,
              },
            },
            {
              r0: '65%',
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
    <div style={{ position: 'relative', width: '100%', height: '100%' }}>
      {/* Depth control slider - positioned top-left to avoid covering DetailPanel */}
      <div
        style={{
          position: 'absolute',
          top: '10px',
          left: '10px',
          zIndex: 1000,
          background: 'rgba(255, 255, 255, 0.9)',
          padding: '8px 12px',
          borderRadius: '4px',
          boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
        }}
      >
        <label
          htmlFor="depth-slider"
          style={{
            fontSize: '12px',
            fontWeight: '500',
            color: '#374151',
          }}
        >
          Depth: {depth}
        </label>
        <input
          id="depth-slider"
          type="range"
          min="1"
          max="4"
          value={depth}
          onChange={(e) => setDepth(Number(e.target.value))}
          style={{
            width: '100px',
            cursor: 'pointer',
          }}
        />
      </div>

      {/* Chart container */}
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
    </div>
  );
});

Sunburst.displayName = 'Sunburst';

export default Sunburst;
