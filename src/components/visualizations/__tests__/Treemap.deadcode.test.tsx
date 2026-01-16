/**
 * Treemap Dead Code Overlay Tests
 *
 * Tests dead code visualization features including borders and tooltips
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/react';
import { Treemap } from '../Treemap';
import * as echarts from 'echarts/core';
import { useDeadCodeEnabled } from '@/store/analysisStore';
import { mockTreeDataWithDeadCode } from '../__fixtures__/mockTreeData';
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

describe('Treemap - Dead Code Overlay', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance );
  });

  afterEach(() => {
    mocks.cleanup();
  });

  describe('Border rendering', () => {
    it('should not render borders when deadCodeEnabled is false', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(false);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      expect(itemStyle.borderColor).toBeInstanceOf(Function);
      expect(itemStyle.borderWidth).toBeInstanceOf(Function);

      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(2);
    });

    it('should render borders when deadCodeEnabled is true', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).not.toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(3);
    });

    it('should not render borders for nodes without dead code even when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      const mockParams = { data: { name: 'clean.ts' } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');
      expect(itemStyle.borderWidth(mockParams)).toBe(2);
    });

    it('should use thicker borders (width 3) for nodes with dead code when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      const highDeadCode = { data: { deadCodeRatio: 0.6 } };
      const mediumDeadCode = { data: { deadCodeRatio: 0.3 } };
      const lowDeadCode = { data: { deadCodeRatio: 0.1 } };

      expect(itemStyle.borderWidth(highDeadCode)).toBe(3);
      expect(itemStyle.borderWidth(mediumDeadCode)).toBe(3);
      expect(itemStyle.borderWidth(lowDeadCode)).toBe(3);
    });
  });

  describe('Tooltip rendering', () => {
    it('should show dead code percentage in tooltip when overlay is enabled', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
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

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
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

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const tooltipFormatter = optionsArg.tooltip.formatter;

      const mockParams = {
        data: {
          name: 'file1.ts',
          value: 300,
          complexity: 40,
          path: '/root/file1.ts',
          type: 'file',
        },
      };

      const tooltip = tooltipFormatter(mockParams);
      expect(tooltip).not.toContain('Dead Code');
    });

    it('should not show dead code in tooltip when deadCodeRatio is 0', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
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
  });

  describe('Store integration', () => {
    it('should respect deadCodeEnabled state from store', () => {
      vi.mocked(useDeadCodeEnabled).mockReturnValue(false);

      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle = optionsArg.series[0].itemStyle;

      const mockParams = { data: { deadCodeRatio: 0.5 } };
      expect(itemStyle.borderColor(mockParams)).toBe('#ffffff');

      vi.clearAllMocks();

      vi.mocked(useDeadCodeEnabled).mockReturnValue(true);
      render(<Treemap data={mockTreeDataWithDeadCode} />);

      const optionsArg2 = mocks.mockSetOption.mock.calls[0][0];
      const itemStyle2 = optionsArg2.series[0].itemStyle;

      expect(itemStyle2.borderColor(mockParams)).not.toBe('#ffffff');
    });
  });
});
