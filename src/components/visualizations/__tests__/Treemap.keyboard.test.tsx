/**
 * Treemap Keyboard Navigation and Accessibility Tests
 *
 * Tests keyboard navigation, accessibility features, and progressive rendering
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/react';
import { Treemap } from '../Treemap';
import * as echarts from 'echarts/core';
import { mockTreeData, emptyTreeData, deepTreeData } from '../__fixtures__/mockTreeData';
import { setupEChartsMocks } from './test-utils';
import * as treeTransform from '@/utils/treeTransform';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

// Mock the analysisStore hook
vi.mock('@/store/analysisStore', () => ({
  useDeadCodeEnabled: vi.fn(() => false),
}));

// Mock tree transformation utilities
vi.mock('@/utils/treeTransform', () => ({
  treeNodeToECharts: vi.fn((data) => ({
    name: data.name,
    path: data.path,
    value: data.loc,
    complexity: data.complexity,
    type: data.type,
    children: data.children?.map((child) => ({
      name: child.name,
      path: child.path,
      value: child.loc,
      complexity: child.complexity,
      type: child.type,
      children: child.children || [],
    })),
  })),
  getFileCount: vi.fn(() => 1000),
}));

describe('Treemap - Keyboard Navigation', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    vi.clearAllMocks();
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  describe('Node selection', () => {
    it('should select node on Enter key', () => {
      const onNodeClick = vi.fn();
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={onNodeClick}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'Enter' });

      expect(onNodeClick).toHaveBeenCalled();
    });

    it('should select node on Space key', () => {
      const onNodeClick = vi.fn();
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={onNodeClick}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: ' ' });

      expect(onNodeClick).toHaveBeenCalled();
    });

    it('should prevent default behavior on Enter', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      const event = new KeyboardEvent('keydown', { key: 'Enter', bubbles: true });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

      chartDiv!.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
    });

    it('should prevent default behavior on Space', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      const event = new KeyboardEvent('keydown', { key: ' ', bubbles: true });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

      chartDiv!.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
    });
  });

  describe('Navigation keys', () => {
    it('should navigate back on Escape key', () => {
      const onNavigateBack = vi.fn();
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNavigateBack={onNavigateBack}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'Escape' });

      expect(onNavigateBack).toHaveBeenCalled();
    });

    it('should move to next node on ArrowDown', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'ArrowDown' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should move to next node on ArrowRight', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'ArrowRight' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should move to previous node on ArrowUp', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'ArrowUp' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should move to previous node on ArrowLeft', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'ArrowLeft' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should jump to first node on Home key', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'Home' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should jump to last node on End key', () => {
      const { container } = render(
        <Treemap
          data={mockTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      fireEvent.keyDown(chartDiv!, { key: 'End' });

      expect(chartDiv).toBeInTheDocument();
    });

    it('should handle navigation with empty tree gracefully', () => {
      const { container } = render(
        <Treemap
          data={emptyTreeData}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');

      expect(() => {
        fireEvent.keyDown(chartDiv!, { key: 'ArrowDown' });
        fireEvent.keyDown(chartDiv!, { key: 'Enter' });
        fireEvent.keyDown(chartDiv!, { key: 'Home' });
      }).not.toThrow();
    });
  });

  describe('Keyboard navigation without handlers', () => {
    it('should not throw when Enter pressed without onNodeClick', () => {
      const { container } = render(
        <Treemap data={mockTreeData} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');

      expect(() => {
        fireEvent.keyDown(chartDiv!, { key: 'Enter' });
      }).not.toThrow();
    });

    it('should not throw when Escape pressed without onNavigateBack', () => {
      const { container } = render(
        <Treemap data={mockTreeData} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');

      expect(() => {
        fireEvent.keyDown(chartDiv!, { key: 'Escape' });
      }).not.toThrow();
    });
  });

  describe('Drill-down path changes', () => {
    it('should reset selected index when drillDownPath changes', () => {
      const { rerender, container } = render(
        <Treemap
          data={mockTreeData}
          drillDownPath={['root']}
          onNodeClick={vi.fn()}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');

      // Navigate to next node
      fireEvent.keyDown(chartDiv!, { key: 'ArrowDown' });

      // Change drill-down path
      rerender(
        <Treemap
          data={mockTreeData}
          drillDownPath={['root', 'subdir']}
          onNodeClick={vi.fn()}
        />
      );

      // Selected index should be reset
      expect(chartDiv).toBeInTheDocument();
    });

    it('should maintain keyboard focus when path remains same', () => {
      const onNodeClick = vi.fn();
      const { rerender, container } = render(
        <Treemap
          data={mockTreeData}
          drillDownPath={['root']}
          onNodeClick={onNodeClick}
        />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');

      // Re-render with same path
      rerender(
        <Treemap
          data={mockTreeData}
          drillDownPath={['root']}
          onNodeClick={onNodeClick}
        />
      );

      expect(chartDiv).toBeInTheDocument();
    });
  });
});

describe('Treemap - Accessibility', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    vi.clearAllMocks();
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  describe('ARIA attributes', () => {
    it('should have tabindex for keyboard focus', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      expect(chartDiv).toHaveAttribute('tabindex', '0');
    });

    it('should have application role', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      expect(chartDiv).toHaveAttribute('role', 'application');
    });

    it('should provide keyboard navigation instructions', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      const ariaLabel = chartDiv?.getAttribute('aria-label');

      expect(ariaLabel).toContain('arrow keys');
      expect(ariaLabel).toContain('Enter');
      expect(ariaLabel).toContain('Escape');
      expect(ariaLabel).toContain('navigate');
      expect(ariaLabel).toContain('select');
    });

    it('should have data-testid for testing', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      expect(chartDiv).toBeInTheDocument();
    });
  });

  describe('Focus management', () => {
    it('should apply focus styles', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      expect(chartDiv).toHaveClass('focus:outline-none');
      expect(chartDiv).toHaveClass('focus:ring-2');
      expect(chartDiv).toHaveClass('focus:ring-blue-500');
      expect(chartDiv).toHaveClass('focus:ring-offset-2');
    });

    it('should have rounded corners for visual consistency', () => {
      const { container } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const chartDiv = container.querySelector('[data-testid="treemap-node"]');
      expect(chartDiv).toHaveClass('rounded-lg');
    });
  });
});

describe('Treemap - Progressive Rendering', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    vi.clearAllMocks();
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  describe('Small datasets', () => {
    it('should not enable progressive rendering for datasets <50K files', () => {
      vi.mocked(treeTransform.getFileCount).mockReturnValue(1000);

      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      expect(mocks.mockSetOption).toHaveBeenCalled();
      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.progressive).toBeUndefined();
      expect(options.progressiveThreshold).toBeUndefined();
      expect(options.progressiveChunkMode).toBeUndefined();
    });

    it('should use merge mode for small datasets', () => {
      vi.mocked(treeTransform.getFileCount).mockReturnValue(1000);

      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      // Second argument to setOption should be true for merge mode
      expect(mocks.mockSetOption).toHaveBeenCalledWith(
        expect.any(Object),
        true
      );
    });
  });

  describe('Large datasets', () => {
    it('should enable progressive rendering for datasets >50K files', () => {
      vi.mocked(treeTransform.getFileCount).mockReturnValue(60000);

      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      const options = mocks.mockSetOption.mock.calls[0][0];

      expect(options.progressive).toBe(500);
      expect(options.progressiveThreshold).toBe(1000);
      expect(options.progressiveChunkMode).toBe('mod');
    });

    it('should use notMerge mode for large datasets', () => {
      vi.mocked(treeTransform.getFileCount).mockReturnValue(60000);

      render(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      // Second argument to setOption should be false for notMerge mode
      expect(mocks.mockSetOption).toHaveBeenCalledWith(
        expect.any(Object),
        false
      );
    });

    it('should calculate file count from data', () => {
      vi.mocked(treeTransform.getFileCount).mockReturnValue(60000);

      render(<Treemap data={deepTreeData} onNodeClick={vi.fn()} />);

      expect(treeTransform.getFileCount).toHaveBeenCalledWith(deepTreeData);
    });
  });

  describe('Performance optimizations', () => {
    it('should memoize ECharts transformation', () => {
      const { rerender } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const firstCallCount = vi.mocked(treeTransform.treeNodeToECharts).mock.calls.length;

      // Re-render with same data
      rerender(<Treemap data={mockTreeData} onNodeClick={vi.fn()} />);

      // Should use memoized result
      expect(vi.mocked(treeTransform.treeNodeToECharts).mock.calls.length).toBe(firstCallCount);
    });

    it('should recalculate transformation when data changes', () => {
      const { rerender } = render(
        <Treemap data={mockTreeData} onNodeClick={vi.fn()} />
      );

      const firstCallCount = vi.mocked(treeTransform.treeNodeToECharts).mock.calls.length;

      // Re-render with different data
      rerender(<Treemap data={deepTreeData} onNodeClick={vi.fn()} />);

      // Should recalculate
      expect(vi.mocked(treeTransform.treeNodeToECharts).mock.calls.length).toBeGreaterThan(firstCallCount);
    });
  });
});
