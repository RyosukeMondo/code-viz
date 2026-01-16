/**
 * Treemap Rendering Tests
 *
 * Tests visual rendering, dimensions, ECharts configuration, and memoization
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/react';
import { Treemap } from '../Treemap';
import * as echarts from 'echarts/core';
import { mockTreeData, emptyTreeData, deepTreeData, zeroLocData, highComplexityData } from '../__fixtures__/mockTreeData';
import { setupEChartsMocks } from './test-utils';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

// Mock the analysisStore hook
vi.mock('@/store/analysisStore', () => ({
  useDeadCodeEnabled: vi.fn(() => false),
}));

describe('Treemap - Rendering', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  afterEach(() => {
    mocks.cleanup();
  });

  describe('Basic rendering', () => {
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

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series).toBeDefined();
      expect(optionsArg.series[0].type).toBe('treemap');
    });

    it('should not render when data is null', () => {
      render(<Treemap data={null } />);

      expect(mocks.mockSetOption).not.toHaveBeenCalled();
    });

    it('should not render when data is undefined', () => {
      render(<Treemap data={undefined } />);

      expect(mocks.mockSetOption).not.toHaveBeenCalled();
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

  describe('ECharts configuration', () => {
    it('should configure treemap with proper series type', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].type).toBe('treemap');
    });

    it('should disable ECharts default breadcrumb', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].breadcrumb.show).toBe(false);
    });

    it('should disable default node click behavior', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].nodeClick).toBe(false);
    });

    it('should enable animations', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].animation).toBe(true);
    });

    it('should configure animation duration', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].animationDuration).toBe(500);
    });

    it('should configure tooltip', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.tooltip).toBeDefined();
      expect(optionsArg.tooltip.formatter).toBeInstanceOf(Function);
    });

    it('should configure labels for nodes', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      expect(optionsArg.series[0].label).toBeDefined();
      expect(optionsArg.series[0].label.show).toBe(true);
    });

    it('should configure emphasis style', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
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

      const initialCallCount = mocks.mockSetOption.mock.calls.length;

      // Rerender with same props
      rerender(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

      // Should use memoized version, no additional setOption calls
      expect(mocks.mockSetOption.mock.calls.length).toBe(initialCallCount);
    });

    it('should re-render when data reference changes', () => {
      const { rerender } = render(<Treemap data={mockTreeData} />);

      const initialCallCount = mocks.mockSetOption.mock.calls.length;

      // New data object with same content
      const newData = { ...mockTreeData };
      rerender(<Treemap data={newData} />);

      expect(mocks.mockSetOption.mock.calls.length).toBeGreaterThan(initialCallCount);
    });
  });

  describe('Edge cases', () => {
    it('should handle empty children array', () => {
      expect(() => {
        render(<Treemap data={emptyTreeData} />);
      }).not.toThrow();
    });

    it('should handle deeply nested tree structure', () => {
      expect(() => {
        render(<Treemap data={deepTreeData} />);
      }).not.toThrow();
    });

    it('should handle node with zero LOC', () => {
      expect(() => {
        render(<Treemap data={zeroLocData} />);
      }).not.toThrow();
    });

    it('should handle node with very high complexity', () => {
      expect(() => {
        render(<Treemap data={highComplexityData} />);
      }).not.toThrow();
    });
  });
});
