/**
 * Treemap Interaction Tests
 *
 * Tests user interactions including clicks, hovers, drill-down, and lifecycle
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/react';
import { Treemap } from '../Treemap';
import type { TreeNode } from '@/types/bindings';
import * as echarts from 'echarts/core';
import { mockTreeData } from '../__fixtures__/mockTreeData';
import { setupEChartsMocks, getEventHandler } from './test-utils';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

// Mock the analysisStore hook
vi.mock('@/store/analysisStore', () => ({
  useDeadCodeEnabled: vi.fn(() => false),
}));

describe('Treemap - Interaction', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  afterEach(() => {
    mocks.cleanup();
  });

  describe('Event handlers', () => {
    it('should register click event handler', () => {
      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      expect(mocks.mockOn).toHaveBeenCalledWith('click', expect.any(Function));
    });

    it('should register mouseover event handler', () => {
      render(<Treemap data={mockTreeData} onNodeHover={vi.fn()} />);

      expect(mocks.mockOn).toHaveBeenCalledWith('mouseover', expect.any(Function));
    });

    it('should register mouseout event handler', () => {
      render(<Treemap data={mockTreeData} onNodeHover={vi.fn()} />);

      expect(mocks.mockOn).toHaveBeenCalledWith('mouseout', expect.any(Function));
    });

    it('should call onNodeClick when node is clicked', () => {
      const onNodeClick = vi.fn();
      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');
      expect(clickHandler).toBeDefined();

      // Simulate click event with ECharts params (using treePathInfo)
      const mockParams = {
        treePathInfo: [{
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
          children: [],
        }],
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

      const mouseoverHandler = getEventHandler(mocks.mockOn, 'mouseover');
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

      const mouseoutHandler = getEventHandler(mocks.mockOn, 'mouseout');
      expect(mouseoutHandler).toBeDefined();

      mouseoutHandler();

      expect(onNodeHover).toHaveBeenCalledWith(null);
    });

    it('should not call onNodeClick if not provided', () => {
      render(<Treemap data={mockTreeData} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');
      const mockParams = {
        treePathInfo: [{
          name: 'file1.ts',
          path: '/root/file1.ts',
          value: 300,
          complexity: 40,
          type: 'file',
        }],
      };

      // Should not throw
      expect(() => clickHandler(mockParams)).not.toThrow();
    });

    it('should not call onNodeHover if not provided', () => {
      render(<Treemap data={mockTreeData} />);

      const mouseoverHandler = getEventHandler(mocks.mockOn, 'mouseover');
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

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      // Component passes children directly
      expect(optionsArg.series[0].data).toHaveLength(2);
      expect(optionsArg.series[0].data[0].name).toBe('file1.ts');
      expect(optionsArg.series[0].data[1].name).toBe('subdir');
    });

    it('should render pre-filtered data from parent', () => {
      const subdirNode = mockTreeData.children![1];
      render(<Treemap data={subdirNode} drillDownPath={['subdir']} />);

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      // Component passes children directly
      expect(optionsArg.series[0].data).toHaveLength(2);
      expect(optionsArg.series[0].data[0].name).toBe('file2.ts');
      expect(optionsArg.series[0].data[1].name).toBe('file3.ts');
    });

    it('should handle deep drill-down paths', () => {
      render(<Treemap data={mockTreeData} drillDownPath={['subdir']} />);

      expect(mocks.mockSetOption).toHaveBeenCalled();
    });

    it('should handle invalid drill-down path gracefully', () => {
      const { container } = render(<Treemap data={mockTreeData} drillDownPath={['nonexistent', 'path']} />);

      expect(container.querySelector('[data-testid="treemap-node"]')).toBeInTheDocument();
    });
  });

  describe('Window resize handling', () => {
    it('should register resize listener on window', () => {
      render(<Treemap data={mockTreeData} />);

      expect(mocks.addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    it('should call chart.resize on window resize', () => {
      render(<Treemap data={mockTreeData} />);

      const resizeHandler = mocks.addEventListenerSpy.mock.calls.find(
        (call) => call[0] === 'resize'
      )?.[1];
      expect(resizeHandler).toBeDefined();

      (resizeHandler as EventListener)(new Event('resize'));

      expect(mocks.mockResize).toHaveBeenCalled();
    });

    it('should remove resize listener on unmount', () => {
      const { unmount } = render(<Treemap data={mockTreeData} />);

      unmount();

      expect(mocks.removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });
  });

  describe('Component lifecycle', () => {
    it('should dispose chart instance on unmount', () => {
      const { unmount } = render(<Treemap data={mockTreeData} />);

      unmount();

      expect(mocks.mockDispose).toHaveBeenCalled();
    });

    it('should unregister all event handlers on data change', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      const newData = { ...mockTreeData, name: 'new-root' };
      rerender(<Treemap data={newData} />);

      expect(mocks.mockOff).toHaveBeenCalledWith('click', expect.any(Function));
      expect(mocks.mockOff).toHaveBeenCalledWith('mouseover', expect.any(Function));
      expect(mocks.mockOff).toHaveBeenCalledWith('mouseout', expect.any(Function));
    });

    it('should update chart when data changes', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      const initialCallCount = mocks.mockSetOption.mock.calls.length;

      const newData = { ...mockTreeData, name: 'new-root' };
      rerender(<Treemap data={newData} />);

      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });

    it('should update chart when data changes (simulating drill-down)', () => {
      const { rerender } = render(<Treemap data={mockTreeData} drillDownPath={[]} />);

      const initialCallCount = mocks.mockSetOption.mock.calls.length;

      const subdirNode = mockTreeData.children![1];
      rerender(<Treemap data={subdirNode} drillDownPath={['subdir']} />);

      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });
  });

  describe('Click Handler with Original TreeNode Data', () => {
    it('should pass TreeNode data to onNodeClick', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');
      expect(clickHandler).toBeDefined();

      const echartsClickData = {
        treePathInfo: [{
          name: 'subdir',
          path: '/root/subdir',
          value: 700,
          complexity: 25,
          type: 'directory',
          children: [
            { name: 'file2.ts', value: 400, path: '/root/subdir/file2.ts' },
            { name: 'file3.ts', value: 300, path: '/root/subdir/file3.ts' },
          ],
        }],
      };

      clickHandler(echartsClickData);

      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'subdir',
          path: '/root/subdir',
          loc: 700,
          complexity: 25,
          type: 'directory',
        })
      );
    });

    it('should handle nested node clicks', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      const echartsClickData = {
        treePathInfo: [{
          name: 'file2.ts',
          path: '/root/subdir/file2.ts',
          value: 400,
          complexity: 20,
          type: 'file',
        }],
      };

      clickHandler(echartsClickData);

      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'file2.ts',
          path: '/root/subdir/file2.ts',
          loc: 400,
          complexity: 20,
          type: 'file',
        })
      );
    });

    it('should handle clicks with no treePathInfo gracefully', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      const echartsClickData = {
        treePathInfo: [],
      };

      clickHandler(echartsClickData);

      expect(onNodeClick).not.toHaveBeenCalled();
    });

    it('should handle clicks with undefined treePathInfo', () => {
      const onNodeClick = vi.fn();

      render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      const echartsClickData = {};

      clickHandler(echartsClickData);

      expect(onNodeClick).not.toHaveBeenCalled();
    });

    it('should handle clicks when data prop changes', () => {
      const onNodeClick = vi.fn();

      const { rerender } = render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      clickHandler({
        treePathInfo: [{
          name: 'subdir',
          path: '/root/subdir',
          value: 700,
        }],
      });

      expect(onNodeClick).toHaveBeenCalledTimes(1);
      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'subdir',
          path: '/root/subdir',
        })
      );

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

      const clickCalls = mocks.mockOn.mock.calls.filter((call: unknown[]) => call[0] === 'click');
      const clickHandler2 = clickCalls[clickCalls.length - 1][1] as (params: Record<string, unknown>) => void;

      clickHandler2({
        treePathInfo: [{
          name: 'newfile.ts',
          path: '/root/newfile.ts',
          value: 500,
        }],
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
