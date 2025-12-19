/**
 * Unit tests for Treemap component
 *
 * Tests rendering, ECharts integration, user interactions, and performance
 * for the treemap visualization component.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Treemap } from './Treemap';
import type { TreeNode } from '@/types/bindings';
import * as echarts from 'echarts/core';
import { useDeadCodeEnabled } from '@/store/analysisStore';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

// Mock the analysisStore hook
vi.mock('@/store/analysisStore', () => ({
  useDeadCodeEnabled: vi.fn(() => false),
}));

describe('Treemap', () => {
  // Mock TreeNode data
  const mockTreeData: TreeNode = {
    id: 'root',
    name: 'root',
    path: '/root',
    loc: 1000,
    complexity: 30,
    type: 'directory',
    lastModified: '2024-01-15T10:30:00Z',
    children: [
      {
        id: 'file-1',
        name: 'file1.ts',
        path: '/root/file1.ts',
        loc: 300,
        complexity: 40,
        type: 'file',
        children: [],
        lastModified: '2024-01-15T10:30:00Z',
      },
      {
        id: 'dir-1',
        name: 'subdir',
        path: '/root/subdir',
        loc: 700,
        complexity: 25,
        type: 'directory',
        lastModified: '2024-01-15T10:30:00Z',
        children: [
          {
            id: 'file-2',
            name: 'file2.ts',
            path: '/root/subdir/file2.ts',
            loc: 400,
            complexity: 20,
            type: 'file',
            children: [],
            lastModified: '2024-01-15T10:30:00Z',
          },
          {
            id: 'file-3',
            name: 'file3.ts',
            path: '/root/subdir/file3.ts',
            loc: 300,
            complexity: 30,
            type: 'file',
            children: [],
            lastModified: '2024-01-15T10:30:00Z',
          },
        ],
      },
    ],
  };

  // Mock ECharts instance
  let mockChartInstance: any;
  let mockSetOption: any;
  let mockOn: any;
  let mockOff: any;
  let mockResize: any;
  let mockDispose: any;

  beforeEach(() => {
    // Create mock chart instance methods
    mockSetOption = vi.fn();
    mockOn = vi.fn();
    mockOff = vi.fn();
    mockResize = vi.fn();
    mockDispose = vi.fn();

    mockChartInstance = {
      setOption: mockSetOption,
      on: mockOn,
      off: mockOff,
      resize: mockResize,
      dispose: mockDispose,
    };

    // Mock echarts.init to return our mock instance
    vi.mocked(echarts.init).mockReturnValue(mockChartInstance as any);

    // Mock window.addEventListener and removeEventListener
    vi.spyOn(window, 'addEventListener');
    vi.spyOn(window, 'removeEventListener');
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render treemap container', () => {
      const { container } = render(<Treemap data={mockTreeData} />);

      const treemapContainer = container.querySelector('.treemap-container');
      expect(treemapContainer).toBeInTheDocument();
    });

    it('should initialize ECharts instance', () => {
      const { container } = render(<Treemap data={mockTreeData} />);
      const treemapDiv = container.querySelector('.treemap-container');

      expect(echarts.init).toHaveBeenCalledWith(treemapDiv);
    });

    it('should call setOption with treemap configuration', () => {
      render(<Treemap data={mockTreeData} />);

      expect(mockSetOption).toHaveBeenCalled();
      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series).toBeDefined();
      expect(optionsArg.series[0].type).toBe('treemap');
    });

    it('should not render when data is null', () => {
      render(<Treemap data={null as any} />);

      expect(mockSetOption).not.toHaveBeenCalled();
    });

    it('should not render when data is undefined', () => {
      render(<Treemap data={undefined as any} />);

      expect(mockSetOption).not.toHaveBeenCalled();
    });
  });

  describe('Dimensions', () => {
    it('should use default width (100%) when not specified', () => {
      const { container } = render(<Treemap data={mockTreeData} />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.width).toBe('100%');
    });

    it('should use default height (600px) when not specified', () => {
      const { container } = render(<Treemap data={mockTreeData} />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.height).toBe('600px');
    });

    it('should accept custom width as string', () => {
      const { container } = render(<Treemap data={mockTreeData} width="500px" />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.width).toBe('500px');
    });

    it('should accept custom width as number', () => {
      const { container } = render(<Treemap data={mockTreeData} width={800} />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.width).toBe('800px');
    });

    it('should accept custom height as string', () => {
      const { container } = render(<Treemap data={mockTreeData} height="400px" />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.height).toBe('400px');
    });

    it('should accept custom height as number', () => {
      const { container } = render(<Treemap data={mockTreeData} height={500} />);
      const treemapDiv = container.querySelector('.treemap-container') as HTMLElement;

      expect(treemapDiv.style.height).toBe('500px');
    });
  });

  describe('Event handlers', () => {
    it('should register click event handler', () => {
      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      expect(mockOn).toHaveBeenCalledWith('click', expect.any(Function));
    });

    it('should register mouseover event handler', () => {
      render(<Treemap data={mockTreeData} onNodeHover={vi.fn()} />);

      expect(mockOn).toHaveBeenCalledWith('mouseover', expect.any(Function));
    });

    it('should register mouseout event handler', () => {
      render(<Treemap data={mockTreeData} onNodeHover={vi.fn()} />);

      expect(mockOn).toHaveBeenCalledWith('mouseout', expect.any(Function));
    });

    it('should call onNodeClick when node is clicked', () => {
      const onNodeClick = vi.fn();
      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      // Get the click handler
      const clickHandler = mockOn.mock.calls.find((call: any) => call[0] === 'click')?.[1];
      expect(clickHandler).toBeDefined();

      // Simulate click event with ECharts params
      const mockParams = {
        data: {
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
          children: [],
        },
      };

      clickHandler(mockParams);

      expect(onNodeClick).toHaveBeenCalledTimes(1);
      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'file1.ts',
          path: '/root/file1.ts',
          loc: 300,
          complexity: 40,
          type: 'file',
        })
      );
    });

    it('should call onNodeHover when node is hovered', () => {
      const onNodeHover = vi.fn();
      render(<Treemap data={mockTreeData} onNodeHover={onNodeHover} />);

      // Get the mouseover handler
      const mouseoverHandler = mockOn.mock.calls.find((call: any) => call[0] === 'mouseover')?.[1];
      expect(mouseoverHandler).toBeDefined();

      // Simulate mouseover event
      const mockParams = {
        data: {
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
          children: [],
        },
      };

      mouseoverHandler(mockParams);

      expect(onNodeHover).toHaveBeenCalledTimes(1);
      expect(onNodeHover).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'file1.ts',
          path: '/root/file1.ts',
        })
      );
    });

    it('should call onNodeHover with null when mouse leaves', () => {
      const onNodeHover = vi.fn();
      render(<Treemap data={mockTreeData} onNodeHover={onNodeHover} />);

      // Get the mouseout handler
      const mouseoutHandler = mockOn.mock.calls.find((call: any) => call[0] === 'mouseout')?.[1];
      expect(mouseoutHandler).toBeDefined();

      mouseoutHandler();

      expect(onNodeHover).toHaveBeenCalledWith(null);
    });

    it('should not call onNodeClick if not provided', () => {
      render(<Treemap data={mockTreeData} />);

      const clickHandler = mockOn.mock.calls.find((call: any) => call[0] === 'click')?.[1];
      const mockParams = {
        data: {
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
        },
      };

      // Should not throw
      expect(() => clickHandler(mockParams)).not.toThrow();
    });

    it('should not call onNodeHover if not provided', () => {
      render(<Treemap data={mockTreeData} />);

      const mouseoverHandler = mockOn.mock.calls.find((call: any) => call[0] === 'mouseover')?.[1];
      const mockParams = {
        data: {
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
        },
      };

      // Should not throw
      expect(() => mouseoverHandler(mockParams)).not.toThrow();
    });
  });

  describe('Drill-down filtering', () => {
    it('should render root data when drillDownPath is empty', () => {
      render(<Treemap data={mockTreeData} drillDownPath={[]} />);

      expect(mockSetOption).toHaveBeenCalled();
      const optionsArg = mockSetOption.mock.calls[0][0];
      // Pass root node wrapped in array
      expect(optionsArg.series[0].data).toHaveLength(1); // [root]
      expect(optionsArg.series[0].data[0].name).toBe('root');
      expect(optionsArg.series[0].data[0].children).toHaveLength(2); // file1.ts and subdir
    });

    it('should render pre-filtered data from parent', () => {
      // Parent component (AnalysisView) is responsible for filtering
      // Treemap receives already-filtered data
      const subdirNode = mockTreeData.children![1]; // The 'subdir' node (second child)
      render(<Treemap data={subdirNode} drillDownPath={['subdir']} />);

      expect(mockSetOption).toHaveBeenCalled();
      // When drilled down, we pass the filtered node
      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].data).toHaveLength(1); // [subdir]
      expect(optionsArg.series[0].data[0].name).toBe('subdir');
      expect(optionsArg.series[0].data[0].children).toHaveLength(2); // file2.ts and file3.ts
    });

    it('should handle deep drill-down paths', () => {
      render(<Treemap data={mockTreeData} drillDownPath={['subdir']} />);

      expect(mockSetOption).toHaveBeenCalled();
    });

    it('should handle invalid drill-down path gracefully', () => {
      // Component should not crash with invalid path
      // It will render an empty treemap since filteredData returns null
      const { container } = render(<Treemap data={mockTreeData} drillDownPath={['nonexistent', 'path']} />);

      // Verify component renders without throwing
      expect(container.querySelector('[data-testid="treemap-node"]')).toBeInTheDocument();

      // setOption might not be called since there's no valid data to display
      // This is the graceful handling - no crash, just no visualization
    });
  });

  describe('Window resize handling', () => {
    it('should register resize listener on window', () => {
      render(<Treemap data={mockTreeData} />);

      expect(window.addEventListener).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    it('should call chart.resize on window resize', () => {
      render(<Treemap data={mockTreeData} />);

      // Get the resize handler
      const addEventListenerCalls = vi.mocked(window.addEventListener).mock.calls;
      const resizeHandler = addEventListenerCalls.find((call) => call[0] === 'resize')?.[1];
      expect(resizeHandler).toBeDefined();

      // Trigger resize
      (resizeHandler as EventListener)(new Event('resize'));

      expect(mockResize).toHaveBeenCalled();
    });

    it('should remove resize listener on unmount', () => {
      const { unmount } = render(<Treemap data={mockTreeData} />);

      unmount();

      expect(window.removeEventListener).toHaveBeenCalledWith('resize', expect.any(Function));
    });
  });

  describe('Component lifecycle', () => {
    it('should dispose chart instance on unmount', () => {
      const { unmount } = render(<Treemap data={mockTreeData} />);

      unmount();

      expect(mockDispose).toHaveBeenCalled();
    });

    it('should unregister all event handlers on data change', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      // Change data to trigger cleanup
      const newData = { ...mockTreeData, name: 'new-root' };
      rerender(<Treemap data={newData} />);

      // Should have called off for each registered event
      expect(mockOff).toHaveBeenCalledWith('click', expect.any(Function));
      expect(mockOff).toHaveBeenCalledWith('mouseover', expect.any(Function));
      expect(mockOff).toHaveBeenCalledWith('mouseout', expect.any(Function));
    });

    it('should update chart when data changes', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      const initialCallCount = mockSetOption.mock.calls.length;

      const newData = { ...mockTreeData, name: 'new-root' };
      rerender(<Treemap data={newData} />);

      expect(mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });

    it('should update chart when data changes (simulating drill-down)', () => {
      const { rerender } = render(<Treemap data={mockTreeData} drillDownPath={[]} />);

      const initialCallCount = mockSetOption.mock.calls.length;

      // Parent component filters data and passes the filtered node
      const subdirNode = mockTreeData.children![1]; // The 'subdir' node (second child)
      rerender(<Treemap data={subdirNode} drillDownPath={['subdir']} />);

      expect(mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });
  });

  describe('ECharts configuration', () => {
    it('should configure treemap with proper series type', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].type).toBe('treemap');
    });

    it('should disable ECharts default breadcrumb', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].breadcrumb.show).toBe(false);
    });

    it('should disable default node click behavior', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].nodeClick).toBe(false);
    });

    it('should enable animations', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].animation).toBe(true);
    });

    it('should configure animation duration', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].animationDuration).toBe(500);
    });

    it('should configure tooltip', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.tooltip).toBeDefined();
      expect(optionsArg.tooltip.formatter).toBeInstanceOf(Function);
    });

    it('should configure labels for nodes', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].label).toBeDefined();
      expect(optionsArg.series[0].label.show).toBe(true);
    });

    it('should configure emphasis style', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].emphasis).toBeDefined();
      expect(optionsArg.series[0].emphasis.itemStyle).toBeDefined();
    });
  });

  describe('Memoization', () => {
    it('should not re-render when props are unchanged', () => {
      const onNodeClick = vi.fn();
      const { rerender } = render(
        <Treemap data={mockTreeData} onNodeClick={onNodeClick} />
      );

      const initialCallCount = mockSetOption.mock.calls.length;

      // Rerender with same props
      rerender(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      // Should use memoized version, no additional setOption calls
      expect(mockSetOption.mock.calls.length).toBe(initialCallCount);
    });

    it('should re-render when data reference changes', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      const initialCallCount = mockSetOption.mock.calls.length;

      // New data object with same content
      const newData = { ...mockTreeData };
      rerender(<Treemap data={newData} />);

      expect(mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });
  });

  describe('Edge cases', () => {
    it('should handle empty children array', () => {
      const emptyTreeData: TreeNode = {
        ...mockTreeData,
        children: [],
      };

      expect(() => {
        render(<Treemap data={emptyTreeData} />);
      }).not.toThrow();
    });

    it('should handle deeply nested tree structure', () => {
      const deepTree: TreeNode = {
        id: 'root',
        name: 'root',
        path: '/root',
        loc: 100,
        complexity: 10,
        type: 'directory',
        lastModified: '2024-01-15T10:30:00Z',
        children: [
          {
            id: 'level1',
            name: 'level1',
            path: '/root/level1',
            loc: 50,
            complexity: 10,
            type: 'directory',
            lastModified: '2024-01-15T10:30:00Z',
            children: [
              {
                id: 'level2',
                name: 'level2',
                path: '/root/level1/level2',
                loc: 25,
                complexity: 10,
                type: 'directory',
                lastModified: '2024-01-15T10:30:00Z',
                children: [
                  {
                    id: 'file',
                    name: 'deep.ts',
                    path: '/root/level1/level2/deep.ts',
                    loc: 25,
                    complexity: 10,
                    type: 'file',
                    lastModified: '2024-01-15T10:30:00Z',
                    children: [],
                  },
                ],
              },
            ],
          },
        ],
      };

      expect(() => {
        render(<Treemap data={deepTree} />);
      }).not.toThrow();
    });

    it('should handle node with zero LOC', () => {
      const zeroLocData: TreeNode = {
        ...mockTreeData,
        loc: 0,
        children: [],
      };

      expect(() => {
        render(<Treemap data={zeroLocData} />);
      }).not.toThrow();
    });

    it('should handle node with very high complexity', () => {
      const highComplexityData: TreeNode = {
        ...mockTreeData,
        complexity: 100,
      };

      expect(() => {
        render(<Treemap data={highComplexityData} />);
      }).not.toThrow();
    });
  });

  describe('Data transformation', () => {
    it('should transform TreeNode to ECharts format', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      // data contains root node wrapped in array
      const rootNode = optionsArg.series[0].data[0];

      // Should have transformed fields (loc → value)
      expect(rootNode).toHaveProperty('name');
      expect(rootNode).toHaveProperty('value'); // ECharts uses 'value' instead of 'loc'
      expect(rootNode).toHaveProperty('complexity');
      expect(rootNode).toHaveProperty('path');
      expect(rootNode).toHaveProperty('type');

      // Verify root values
      expect(rootNode.name).toBe('root');
      expect(rootNode.value).toBe(1000); // root LOC
      expect(rootNode.path).toBe('/root');
      expect(rootNode.type).toBe('directory');
      expect(rootNode.children).toHaveLength(2);
    });

    it('should preserve children structure in transformation', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      // data contains root node
      const rootData = optionsArg.series[0].data[0];
      expect(rootData.name).toBe('root');
      expect(rootData.children).toHaveLength(2);

      // First child is file1.ts (no children)
      expect(rootData.children[0].name).toBe('file1.ts');
      expect(rootData.children[0].type).toBe('file');

      // Second child is subdir (has children)
      const subdirData = rootData.children[1];
      expect(subdirData.name).toBe('subdir');
      expect(subdirData.type).toBe('directory');
      expect(subdirData).toHaveProperty('children');
      expect(Array.isArray(subdirData.children)).toBe(true);
      expect(subdirData.children).toHaveLength(2); // file2.ts and file3.ts
    });

    /**
     * CRITICAL REGRESSION TEST: Validate root node structure
     *
     * ECharts treemap expects data: [rootNode] where rootNode has name/path/type.
     * The root node from backend may have path="" (empty string).
     * Click handler must handle empty path as root and pass correct TreeNode.
     *
     * This test validates that:
     * - We pass a single root node wrapped in array
     * - Root node has all required properties (name, path, type)
     * - All descendants recursively have required properties
     */
    it('should ensure root node and all descendants have required properties', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const nodes = optionsArg.series[0].data;

      /**
       * Recursive validation function
       * Ensures every node and its descendants have required properties
       */
      function validateNode(node: any, nodePath = 'root') {
        // CRITICAL: Every node must have these properties defined
        expect(node.name, `${nodePath}: must have name`).toBeDefined();
        expect(node.path, `${nodePath}: must have path`).toBeDefined();
        expect(node.type, `${nodePath}: must have type`).toBeDefined();

        // Validate types
        expect(typeof node.name, `${nodePath}: name must be string`).toBe('string');
        expect(typeof node.path, `${nodePath}: path must be string`).toBe('string');
        expect(typeof node.type, `${nodePath}: type must be string`).toBe('string');
        expect(['file', 'directory'], `${nodePath}: type must be file or directory`).toContain(node.type);

        // Name must not be empty
        expect(node.name.length, `${nodePath}: name must not be empty`).toBeGreaterThan(0);

        // Path must be defined (can be empty string for root from backend)
        expect(node.path, `${nodePath}: path must be defined (can be empty string for root)`).not.toBeUndefined();

        // Recursively validate children
        if (node.children && Array.isArray(node.children)) {
          node.children.forEach((child: any, i: number) => {
            validateNode(child, `${nodePath}.children[${i}]`);
          });
        }
      }

      // Validate root node
      expect(nodes.length, 'Should pass [root] node').toBe(1);

      const rootNode = nodes[0];
      validateNode(rootNode, 'root');

      // Verify root properties
      expect(rootNode.name).toBe('root');
      expect(rootNode.type).toBe('directory');
      expect(rootNode.children).toHaveLength(2);
    });
  });

  describe('Dead code overlay', () => {
    const mockTreeDataWithDeadCode: TreeNode = {
      ...mockTreeData,
      children: [
        {
          id: 'file-1',
          name: 'file1.ts',
          path: '/root/file1.ts',
          loc: 300,
          complexity: 40,
          type: 'file',
          children: [],
          lastModified: '2024-01-15T10:30:00Z',
          deadCodeRatio: 0.6, // 60% dead code - high
        },
        {
          id: 'file-2',
          name: 'file2.ts',
          path: '/root/file2.ts',
          loc: 200,
          complexity: 30,
          type: 'file',
          children: [],
          lastModified: '2024-01-15T10:30:00Z',
          deadCodeRatio: 0.3, // 30% dead code - medium
        },
        {
          id: 'file-3',
          name: 'file3.ts',
          path: '/root/file3.ts',
          loc: 100,
          complexity: 20,
          type: 'file',
          children: [],
          lastModified: '2024-01-15T10:30:00Z',
          deadCodeRatio: 0.1, // 10% dead code - low
        },
      ],
    };

    beforeEach(() => {
      // Reset mocks before each test in this section
      vi.clearAllMocks();
    });

    it('should not render borders when deadCodeEnabled is false', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(false);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      // itemStyle should have borderColor and borderWidth functions
      expect(itemStyle.borderColor).toBeInstanceOf(Function);
      expect(itemStyle.borderWidth).toBeInstanceOf(Function);

      // When deadCodeEnabled is false, borders should be default
      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(2);
    });

    it('should render borders when deadCodeEnabled is true', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      // When deadCodeEnabled is true and node has dead code, border should be colored
      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).not.toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(3);
    });

    it('should not render borders for nodes without dead code even when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      // Nodes without deadCodeRatio should have default border
      const mockParams = { data: { name: 'clean.ts' } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(2);
    });

    it('should use thicker borders (width 3) for nodes with dead code when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      const highDeadCode = { data: { deadCodeRatio: 0.6 } };
      const mediumDeadCode = { data: { deadCodeRatio: 0.3 } };
      const lowDeadCode = { data: { deadCodeRatio: 0.1 } };

      expect(itemStyle.borderWidth(highDeadCode)).toBe(3);
      expect(itemStyle.borderWidth(mediumDeadCode)).toBe(3);
      expect(itemStyle.borderWidth(lowDeadCode)).toBe(3);
    });

    it('should show dead code percentage in tooltip when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const tooltipFormatter = optionsArg.tooltip.formatter;

      const mockParams = {
        data: {
          name: 'file1.ts',
          value: 300,
          complexity: 40,
          path: '/root/file1.ts',
          type: 'file',
          deadCodeRatio: 0.6,
        },
      };

      const tooltip = tooltipFormatter(mockParams);
      expect(tooltip).toContain('Dead Code');
      expect(tooltip).toContain('60.0%');
    });

    it('should not show dead code percentage in tooltip when overlay is disabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(false);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const tooltipFormatter = optionsArg.tooltip.formatter;

      const mockParams = {
        data: {
          name: 'file1.ts',
          value: 300,
          complexity: 40,
          path: '/root/file1.ts',
          type: 'file',
          deadCodeRatio: 0.6,
        },
      };

      const tooltip = tooltipFormatter(mockParams);
      expect(tooltip).not.toContain('Dead Code');
    });

    it('should not show dead code in tooltip for nodes without deadCodeRatio', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const tooltipFormatter = optionsArg.tooltip.formatter;

      const mockParams = {
        data: {
          name: 'file1.ts',
          value: 300,
          complexity: 40,
          path: '/root/file1.ts',
          type: 'file',
          // No deadCodeRatio
        },
      };

      const tooltip = tooltipFormatter(mockParams);
      expect(tooltip).not.toContain('Dead Code');
    });

    it('should not show dead code in tooltip when deadCodeRatio is 0', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const tooltipFormatter = optionsArg.tooltip.formatter;

      const mockParams = {
        data: {
          name: 'file1.ts',
          value: 300,
          complexity: 40,
          path: '/root/file1.ts',
          type: 'file',
          deadCodeRatio: 0,
        },
      };

      const tooltip = tooltipFormatter(mockParams);
      expect(tooltip).not.toContain('Dead Code');
    });

    it('should respect deadCodeEnabled state from store', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(false);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      // When deadCodeEnabled is false initially, borders should be default
      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');

      // Cleanup
      vi.clearAllMocks();

      // Now test with deadCodeEnabled true
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);
      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg2 = mockSetOption.mock.calls[0][0];
      const itemStyle2 = optionsArg2.series[0].itemStyle;

      // When deadCodeEnabled is true, borders should be colored
      expect(itemStyle2.borderColor(mockParams)).not.toBe('#ffffff');
    });
  });

  describe('Click Handler with Original TreeNode Data', () => {
    /**
     * REGRESSION TEST for serialization bug
     *
     * Bug: When clicking on a node, the component was reconstructing a TreeNode
     * from ECharts data, which had children in ECharts format (not TreeNode format).
     * This caused drill-down to fail because the children array was malformed.
     *
     * Fix: Use findNodeByPath to get the original TreeNode from source data instead
     * of reconstructing from ECharts data.
     */
    it('should pass original TreeNode with proper children to onNodeClick', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      // Simulate ECharts click event on 'subdir' directory
      const clickHandler = mockOn.mock.calls.find((call) => call[0] === 'click')?.[1];
      expect(clickHandler).toBeDefined();

      // Simulate ECharts data (which has 'value' instead of 'loc')
      const echartsClickData = {
        data: {
          name: 'subdir',
          path: '/root/subdir',
          value: 700, // ECharts format uses 'value', not 'loc'
          complexity: 25,
          type: 'directory',
          // ECharts children would also be in ECharts format
          children: [
            { name: 'file2.ts', value: 400, path: '/root/subdir/file2.ts' },
            { name: 'file3.ts', value: 300, path: '/root/subdir/file3.ts' },
          ],
        },
      };

      clickHandler(echartsClickData);

      // CRITICAL: onNodeClick should receive the ORIGINAL TreeNode, not reconstructed one
      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'subdir',
          path: '/root/subdir',
          loc: 700, // Should be 'loc', not 'value'
          complexity: 25,
          type: 'directory',
          // Children should be proper TreeNode objects with 'loc', not 'value'
          children: expect.arrayContaining([
            expect.objectContaining({
              name: 'file2.ts',
              path: '/root/subdir/file2.ts',
              loc: 400, // NOT 'value'
              complexity: 20,
              type: 'file',
              children: [],
            }),
            expect.objectContaining({
              name: 'file3.ts',
              path: '/root/subdir/file3.ts',
              loc: 300, // NOT 'value'
              complexity: 30,
              type: 'file',
              children: [],
            }),
          ]),
        })
      );
    });

    it('should find nested nodes correctly', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = mockOn.mock.calls.find((call) => call[0] === 'click')?.[1];

      // Click on deeply nested file
      const echartsClickData = {
        data: {
          name: 'file2.ts',
          path: '/root/subdir/file2.ts',
          value: 400,
          complexity: 20,
          type: 'file',
        },
      };

      clickHandler(echartsClickData);

      // Should find and return the original nested node
      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'file2.ts',
          path: '/root/subdir/file2.ts',
          loc: 400,
          complexity: 20,
          type: 'file',
          children: [], // Empty array for file
        })
      );
    });

    it('should log error when node path not found', () => {
      const onNodeClick = vi.fn();
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = mockOn.mock.calls.find((call) => call[0] === 'click')?.[1];

      // Click with invalid path
      const echartsClickData = {
        data: {
          name: 'nonexistent.ts',
          path: '/root/nonexistent.ts',
          value: 100,
        },
      };

      clickHandler(echartsClickData);

      // Should log error and NOT call onNodeClick
      expect(consoleSpy).toHaveBeenCalledWith(
        expect.stringContaining('[Treemap] Could not find original node for path:'),
        '/root/nonexistent.ts',
        'in data:',
        'root'
      );
      expect(onNodeClick).not.toHaveBeenCalled();

      consoleSpy.mockRestore();
    });

    it('should ignore clicks on root container (empty path)', () => {
      const onNodeClick = vi.fn();
      const consoleDebugSpy = vi.spyOn(console, 'debug').mockImplementation(() => {});

      // Create mock data with empty path (like backend root node)
      const rootWithEmptyPath = { ...mockTreeData, path: '' };

      render(<Treemap data={rootWithEmptyPath} onNodeClick={onNodeClick} />);

      const clickHandler = mockOn.mock.calls.find((call) => call[0] === 'click')?.[1];

      // Click on root container (empty path, undefined name)
      const echartsClickData = {
        data: {
          name: undefined,
          value: 1000,
          path: '', // Empty path - this is the root container
        },
      };

      clickHandler(echartsClickData);

      // Should log debug message about ignoring root container
      expect(consoleDebugSpy).toHaveBeenCalledWith(
        expect.stringContaining('[Treemap] Clicked on root container')
      );
      // Should NOT call onNodeClick - root container is not drillable
      expect(onNodeClick).not.toHaveBeenCalled();

      consoleDebugSpy.mockRestore();
    });

    it('should handle clicks when data prop changes', () => {
      const onNodeClick = vi.fn();

      const { rerender } = render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = mockOn.mock.calls.find((call) => call[0] === 'click')?.[1];

      // Click on subdir from first data set
      clickHandler({
        data: {
          name: 'subdir',
          path: '/root/subdir',
          value: 700,
        },
      });

      expect(onNodeClick).toHaveBeenCalledTimes(1);
      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'subdir',
          path: '/root/subdir',
        })
      );

      // Update data
      const newData: TreeNode = {
        ...mockTreeData,
        children: [
          {
            id: 'new-file',
            name: 'newfile.ts',
            path: '/root/newfile.ts',
            loc: 500,
            complexity: 35,
            type: 'file',
            children: [],
            lastModified: '2024-01-15T10:30:00Z',
          },
        ],
      };

      rerender(<Treemap data={newData} onNodeClick={onNodeClick} />);

      // After rerender, get the new click handler
      // Find the last 'click' handler registration (after rerender)
      const clickCalls = mockOn.mock.calls.filter((call) => call[0] === 'click');
      const clickHandler2 = clickCalls[clickCalls.length - 1][1];

      clickHandler2({
        data: {
          name: 'newfile.ts',
          path: '/root/newfile.ts',
          value: 500,
        },
      });

      expect(onNodeClick).toHaveBeenCalledTimes(2);
      expect(onNodeClick).toHaveBeenLastCalledWith(
        expect.objectContaining({
          name: 'newfile.ts',
          path: '/root/newfile.ts',
          loc: 500,
        })
      );
    });
  });
});
