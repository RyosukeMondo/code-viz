/**
 * Tests for Sunburst component
 */

import { render, screen, fireEvent } from '@testing-library/react';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import * as echarts from 'echarts/core';
import Sunburst from './Sunburst';
import { mockTreeData } from './__fixtures__/mockTreeData';
import { setupEChartsMocks, getEventHandler } from './__tests__/test-utils';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

describe('Sunburst', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    vi.clearAllMocks();
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  describe('rendering', () => {
    test('renders chart container', () => {
      const { container } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(chartDiv).toBeInTheDocument();
      expect(chartDiv).toHaveAttribute('aria-label', 'Sunburst chart visualization of code metrics');
    });

    test('renders depth control slider', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(screen.getByLabelText(/Depth:/)).toBeInTheDocument();
      expect(screen.getByRole('slider')).toBeInTheDocument();
    });

    test('shows default depth of 1', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(screen.getByText('Depth: 1')).toBeInTheDocument();
    });

    test('initializes ECharts instance', () => {
      const { container } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(echarts.init).toHaveBeenCalledWith(chartDiv);
    });

    test('does not set options when data is null', () => {
      render(
        <Sunburst
          data={null}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockSetOption).not.toHaveBeenCalled();
    });
  });

  describe('depth control', () => {
    test('updates depth when slider changed', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const slider = screen.getByRole('slider');
      fireEvent.change(slider, { target: { value: '3' } });

      expect(screen.getByText('Depth: 3')).toBeInTheDocument();
    });

    test('slider has min value of 1', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const slider = screen.getByRole('slider');
      expect(slider).toHaveAttribute('min', '1');
    });

    test('slider has max value of 4', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const slider = screen.getByRole('slider');
      expect(slider).toHaveAttribute('max', '4');
    });

    test('updates chart when depth changes', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const initialCalls = mocks.mockSetOption.mock.calls.length;

      const slider = screen.getByRole('slider');
      fireEvent.change(slider, { target: { value: '2' } });

      // Should call setOption again with new depth
      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(initialCalls);
    });
  });

  describe('chart configuration', () => {
    test('sets chart options with sunburst type', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series).toBeDefined();
      expect(options.series[0].type).toBe('sunburst');
    });

    test('configures radial layout', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].radius).toEqual([0, '90%']);
    });

    test('configures center back button', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].levels).toBeDefined();
      expect(options.series[0].levels[0].r).toBe('12%');
    });

    test('configures tooltip', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.tooltip).toBeDefined();
      expect(options.tooltip.trigger).toBe('item');
    });

    test('enables root to node click behavior', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].nodeClick).toBe('rootToNode');
    });

    test('sorts segments by descending size', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].sort).toBe('desc');
    });
  });

  describe('click interactions', () => {
    test('attaches click event handler', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockOn).toHaveBeenCalledWith('click', expect.any(Function));
    });

    test('calls onNavigateBack when center is clicked', () => {
      const onNavigateBack = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNavigateBack={onNavigateBack}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      // Click center (no data)
      clickHandler({});

      expect(onNavigateBack).toHaveBeenCalled();
    });

    test('calls onNavigateBack when root level is clicked', () => {
      const onNavigateBack = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNavigateBack={onNavigateBack}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      // Click root (dataIndex: 0)
      clickHandler({ dataIndex: 0, data: { name: 'root' } });

      expect(onNavigateBack).toHaveBeenCalled();
    });

    test('calls onNavigateBack when current directory is clicked', () => {
      const onNavigateBack = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNavigateBack={onNavigateBack}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      // Click the current directory (same as data)
      clickHandler({
        data: {
          name: mockTreeData.name,
          path: mockTreeData.path,
        },
      });

      expect(onNavigateBack).toHaveBeenCalled();
    });

    test('calls onNodeClick when child node is clicked', () => {
      const onNodeClick = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={onNodeClick}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      const childData = {
        name: 'file1.ts',
        path: '/root/file1.ts',
        value: 300,
        complexity: 40,
        type: 'file',
      };

      clickHandler({ data: childData });

      expect(onNodeClick).toHaveBeenCalled();
    });

    test('does not call onNodeClick when center is clicked', () => {
      const onNodeClick = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={onNodeClick}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      clickHandler({});

      expect(onNodeClick).not.toHaveBeenCalled();
    });

    test('removes click handler on unmount', () => {
      const { unmount } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      unmount();

      expect(mocks.mockOff).toHaveBeenCalledWith('click');
    });
  });

  describe('hover interactions', () => {
    test('attaches mouseover handler when onNodeHover provided', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNodeHover={vi.fn()}
        />
      );

      expect(mocks.mockOn).toHaveBeenCalledWith('mouseover', expect.any(Function));
    });

    test('does not attach mouseover handler when onNodeHover not provided', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const mouseoverCalls = mocks.mockOn.mock.calls.filter(call => call[0] === 'mouseover');
      expect(mouseoverCalls.length).toBe(0);
    });

    test('calls onNodeHover with correct data', () => {
      const onNodeHover = vi.fn();

      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNodeHover={onNodeHover}
        />
      );

      const hoverHandler = getEventHandler(mocks.mockOn, 'mouseover');

      const mockParams = {
        data: {
          name: 'hovered-file.ts',
          path: '/root/hovered-file.ts',
          value: 200,
          complexity: 30,
          type: 'file',
        },
      };

      hoverHandler(mockParams);

      expect(onNodeHover).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'hovered-file.ts',
          path: '/root/hovered-file.ts',
          loc: 200,
          complexity: 30,
        })
      );
    });
  });

  describe('window resize handling', () => {
    test('attaches resize event listener', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    test('calls chart.resize on window resize', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const resizeHandler = mocks.addEventListenerSpy.mock.calls.find(
        call => call[0] === 'resize'
      )?.[1];

      resizeHandler();

      expect(mocks.mockResize).toHaveBeenCalled();
    });

    test('removes resize event listener on unmount', () => {
      const { unmount } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      unmount();

      expect(mocks.removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });
  });

  describe('cleanup', () => {
    test('disposes chart instance on unmount', () => {
      const { unmount } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      unmount();

      expect(mocks.mockDispose).toHaveBeenCalled();
    });

    test('clears all event handlers on unmount', () => {
      const { unmount } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNodeHover={vi.fn()}
        />
      );

      unmount();

      expect(mocks.mockOff).toHaveBeenCalledWith('click');
      expect(mocks.mockOff).toHaveBeenCalledWith('mouseover');
    });
  });

  describe('data updates', () => {
    test('updates chart when data changes', () => {
      const { rerender } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const newData = {
        ...mockTreeData,
        name: 'updated-root',
        loc: 2000,
      };

      rerender(
        <Sunburst
          data={newData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(1);
    });

    test('reuses same chart instance on data update', () => {
      const { rerender } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const newData = {
        ...mockTreeData,
        loc: 3000,
      };

      rerender(
        <Sunburst
          data={newData}
          onNodeClick={vi.fn()}
        />
      );

      // init should only be called once
      expect(echarts.init).toHaveBeenCalledTimes(1);
    });
  });

  describe('depth control UI', () => {
    test('depth control is positioned top-left', () => {
      const { container } = render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const depthControl = container.querySelector('[style*="top: 10px"]');
      expect(depthControl).toBeInTheDocument();
    });

    test('depth control has proper styling', () => {
      render(
        <Sunburst
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const depthControl = screen.getByLabelText(/Depth:/).closest('div');
      expect(depthControl).toHaveStyle({
        position: 'absolute',
        zIndex: '1000',
      });
    });
  });
});
