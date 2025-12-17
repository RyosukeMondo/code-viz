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

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
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
      expect(optionsArg.series[0].data[0].name).toBe('root');
    });

    it('should filter data based on drillDownPath', () => {
      render(<Treemap data={mockTreeData} drillDownPath={['subdir']} />);

      expect(mockSetOption).toHaveBeenCalled();
      // The filtered data should be the subdir node
      const optionsArg = mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].data[0].name).toBe('subdir');
    });

    it('should handle deep drill-down paths', () => {
      render(<Treemap data={mockTreeData} drillDownPath={['subdir']} />);

      expect(mockSetOption).toHaveBeenCalled();
    });

    it('should handle invalid drill-down path gracefully', () => {
      // Mock console.warn to suppress warning
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      render(<Treemap data={mockTreeData} drillDownPath={['nonexistent', 'path']} />);

      expect(consoleWarnSpy).toHaveBeenCalled();

      consoleWarnSpy.mockRestore();
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

    it('should update chart when drillDownPath changes', () => {
      const { rerender } = render(<Treemap data={mockTreeData} drillDownPath={[]} />);

      const initialCallCount = mockSetOption.mock.calls.length;

      rerender(<Treemap data={mockTreeData} drillDownPath={['subdir']} />);

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
      const echartsData = optionsArg.series[0].data[0];

      // Should have transformed fields
      expect(echartsData).toHaveProperty('name');
      expect(echartsData).toHaveProperty('value');
      expect(echartsData).toHaveProperty('complexity');
      expect(echartsData).toHaveProperty('path');
      expect(echartsData).toHaveProperty('type');
    });

    it('should preserve children structure in transformation', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mockSetOption.mock.calls[0][0];
      const echartsData = optionsArg.series[0].data[0];

      expect(echartsData).toHaveProperty('children');
      expect(Array.isArray(echartsData.children)).toBe(true);
    });
  });
});
