/**
 * Tests for CirclePacking component
 */

import { render } from '@testing-library/react';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import * as echarts from 'echarts/core';
import CirclePacking from './CirclePacking';
import { mockTreeData } from './__fixtures__/mockTreeData';
import { setupEChartsMocks, getEventHandler } from './__tests__/test-utils';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

describe('CirclePacking', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    vi.clearAllMocks();
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  describe('rendering', () => {
    test('renders chart container with correct dimensions', () => {
      const { container } = render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(chartDiv).toBeInTheDocument();
      expect(chartDiv).toHaveAttribute('aria-label', 'Circle packing visualization of code metrics');
    });

    test('renders with minimum height', () => {
      const { container } = render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(chartDiv).toHaveStyle({ minHeight: '400px' });
    });

    test('initializes ECharts instance', () => {
      const { container } = render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(echarts.init).toHaveBeenCalledWith(chartDiv);
    });

    test('does not render when data is null', () => {
      const { container } = render(
        <CirclePacking
          data={null}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('div[role="img"]');
      expect(chartDiv).toBeInTheDocument();
      // Chart should not be initialized without data
      expect(mocks.mockSetOption).not.toHaveBeenCalled();
    });
  });

  describe('chart configuration', () => {
    test('sets chart options with tree layout', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series).toBeDefined();
      expect(options.series[0].type).toBe('tree');
      expect(options.series[0].layout).toBe('radial');
    });

    test('configures tooltip', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.tooltip).toBeDefined();
      expect(options.tooltip.trigger).toBe('item');
    });

    test('configures circle symbols', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].symbol).toBe('circle');
      expect(options.series[0].symbolSize).toBeInstanceOf(Function);
    });

    test('enables expand and collapse', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].expandAndCollapse).toBe(true);
    });

    test('configures animations', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.series[0].animationDuration).toBe(550);
      expect(options.series[0].animationDurationUpdate).toBe(750);
    });
  });

  describe('click interactions', () => {
    test('attaches click event handler', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.mockOn).toHaveBeenCalledWith('click', expect.any(Function));
    });

    test('calls onNodeClick with correct data when node clicked', () => {
      const onNodeClick = vi.fn();

      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={onNodeClick}
        />
      );

      const clickHandler = getEventHandler(mocks.mockOn, 'click');

      const mockParams = {
        data: {
          name: 'test-file.ts',
          path: '/root/test-file.ts',
          value: 100,
          complexity: 20,
          type: 'file',
        },
      };

      clickHandler(mockParams);

      expect(onNodeClick).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'test-file.ts',
          path: '/root/test-file.ts',
          loc: 100,
          complexity: 20,
          type: 'file',
        })
      );
    });

    test('handles click without crashing when data is missing', () => {
      const onNodeClick = vi.fn();

      render(
        <CirclePacking
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
        <CirclePacking
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
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNodeHover={vi.fn()}
        />
      );

      expect(mocks.mockOn).toHaveBeenCalledWith('mouseover', expect.any(Function));
    });

    test('does not attach mouseover handler when onNodeHover not provided', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const mouseoverCalls = mocks.mockOn.mock.calls.filter(call => call[0] === 'mouseover');
      expect(mouseoverCalls.length).toBe(0);
    });

    test('calls onNodeHover with correct data when node hovered', () => {
      const onNodeHover = vi.fn();

      render(
        <CirclePacking
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

    test('removes mouseover handler on unmount', () => {
      const { unmount } = render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
          onNodeHover={vi.fn()}
        />
      );

      unmount();

      expect(mocks.mockOff).toHaveBeenCalledWith('mouseover');
    });
  });

  describe('window resize handling', () => {
    test('attaches resize event listener', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      expect(mocks.addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    test('calls chart.resize on window resize', () => {
      render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const resizeHandler = mocks.addEventListenerSpy.mock.calls.find(
        call => call[0] === 'resize'
      )?.[1];

      expect(resizeHandler).toBeDefined();
      resizeHandler();

      expect(mocks.mockResize).toHaveBeenCalled();
    });

    test('removes resize event listener on unmount', () => {
      const { unmount } = render(
        <CirclePacking
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
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      unmount();

      expect(mocks.mockDispose).toHaveBeenCalled();
    });

    test('clears event handlers before unmount', () => {
      const { unmount } = render(
        <CirclePacking
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
        <CirclePacking
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
        <CirclePacking
          data={newData}
          onNodeClick={vi.fn()}
        />
      );

      // setOption should be called again with new data
      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(1);
    });

    test('reuses same chart instance on data update', () => {
      const { rerender } = render(
        <CirclePacking
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const newData = {
        ...mockTreeData,
        loc: 3000,
      };

      rerender(
        <CirclePacking
          data={newData}
          onNodeClick={vi.fn()}
        />
      );

      // init should only be called once
      expect(echarts.init).toHaveBeenCalledTimes(1);
    });
  });
});
