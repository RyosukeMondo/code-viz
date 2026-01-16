/**
 * Tests for DataDebugger component
 */

import { render, screen, fireEvent } from '@testing-library/react';
import { describe, test, expect } from 'vitest';
import { DataDebugger } from './DataDebugger';
import { mockTreeData } from '../visualizations/__fixtures__/mockTreeData';

describe('DataDebugger', () => {
  describe('rendering', () => {
    test('renders debug panel when data is provided', () => {
      render(<DataDebugger data={mockTreeData} />);

      expect(screen.getByText('🐛 API Data Debug')).toBeInTheDocument();
    });

    test('does not render when data is null', () => {
      const { container } = render(<DataDebugger data={null} />);

      expect(container.firstChild).toBeNull();
    });

    test('is collapsed by default', () => {
      render(<DataDebugger data={mockTreeData} />);

      // Content should not be visible initially
      expect(screen.queryByText('Summary:')).not.toBeInTheDocument();
    });

    test('renders in fixed bottom-right position', () => {
      const { container } = render(<DataDebugger data={mockTreeData} />);

      const debugPanel = container.querySelector('.fixed.bottom-4.right-4');
      expect(debugPanel).toBeInTheDocument();
    });

    test('has high z-index for visibility', () => {
      const { container } = render(<DataDebugger data={mockTreeData} />);

      const debugPanel = container.querySelector('.z-50');
      expect(debugPanel).toBeInTheDocument();
    });
  });

  describe('interactions', () => {
    test('expands when toggle button is clicked', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      // Content should now be visible
      expect(screen.getByText('Summary:')).toBeInTheDocument();
      expect(screen.getByText('Full Data (first 100 lines):')).toBeInTheDocument();
    });

    test('collapses when toggle button is clicked again', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');

      // Expand
      fireEvent.click(toggleButton);
      expect(screen.getByText('Summary:')).toBeInTheDocument();

      // Collapse
      fireEvent.click(toggleButton);
      expect(screen.queryByText('Summary:')).not.toBeInTheDocument();
    });

    test('toggles expand/collapse icon', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');

      // Check initial icon
      expect(screen.getByText('▶')).toBeInTheDocument();

      // Expand
      fireEvent.click(toggleButton);
      expect(screen.getByText('▼')).toBeInTheDocument();

      // Collapse
      fireEvent.click(toggleButton);
      expect(screen.getByText('▶')).toBeInTheDocument();
    });
  });

  describe('data display', () => {
    test('displays summary data when expanded', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      // Summary should contain key information
      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain('rootId');
      expect(summaryPre.textContent).toContain('rootName');
      expect(summaryPre.textContent).toContain('rootPath');
      expect(summaryPre.textContent).toContain('childrenCount');
    });

    test('displays correct root ID in summary', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain(mockTreeData.id);
    });

    test('displays correct root name in summary', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain(mockTreeData.name);
    });

    test('displays correct children count', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain(
        String(mockTreeData.children?.length || 0)
      );
    });

    test('displays full data truncated to 100 lines', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      expect(screen.getByText('Full Data (first 100 lines):')).toBeInTheDocument();
    });

    test('formats data as JSON with indentation', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const fullDataPre = screen.getAllByRole('code')[1];
      // JSON should be formatted with indentation
      expect(fullDataPre.textContent).toContain('{');
      expect(fullDataPre.textContent).toContain('}');
    });
  });

  describe('edge cases', () => {
    test('handles data without children', () => {
      const dataWithoutChildren = {
        ...mockTreeData,
        children: undefined,
      };

      render(<DataDebugger data={dataWithoutChildren} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain('"childrenCount": 0');
      expect(summaryPre.textContent).toContain('"firstChildName": "none"');
    });

    test('handles empty children array', () => {
      const dataWithEmptyChildren = {
        ...mockTreeData,
        children: [],
      };

      render(<DataDebugger data={dataWithEmptyChildren} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const summaryPre = screen.getAllByRole('code')[0];
      expect(summaryPre.textContent).toContain('"childrenCount": 0');
    });

    test('handles data with missing optional fields', () => {
      const minimalData = {
        id: 'test',
        name: 'test-file',
        path: '/test',
        loc: 100,
        complexity: 10,
        type: 'file' as const,
        lastModified: '',
        children: [],
      };

      render(<DataDebugger data={minimalData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      expect(screen.getByText('Summary:')).toBeInTheDocument();
    });

    test('handles very large data structures', () => {
      const largeData = {
        ...mockTreeData,
        children: Array(200).fill(mockTreeData.children?.[0]),
      };

      render(<DataDebugger data={largeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      // Should still render without errors
      expect(screen.getByText('Summary:')).toBeInTheDocument();
    });
  });

  describe('styling', () => {
    test('uses monospace font', () => {
      const { container } = render(<DataDebugger data={mockTreeData} />);

      const debugPanel = container.querySelector('.font-mono');
      expect(debugPanel).toBeInTheDocument();
    });

    test('has dark terminal-like styling', () => {
      const { container } = render(<DataDebugger data={mockTreeData} />);

      const debugPanel = container.querySelector('.bg-gray-900');
      expect(debugPanel).toBeInTheDocument();

      const greenText = container.querySelector('.text-green-400');
      expect(greenText).toBeInTheDocument();
    });

    test('has scrollable content area when expanded', () => {
      render(<DataDebugger data={mockTreeData} />);

      const toggleButton = screen.getByText('🐛 API Data Debug');
      fireEvent.click(toggleButton);

      const contentArea = document.querySelector('.overflow-auto.max-h-96');
      expect(contentArea).toBeInTheDocument();
    });

    test('has border and shadow for visibility', () => {
      const { container } = render(<DataDebugger data={mockTreeData} />);

      const debugPanel = container.querySelector('.shadow-2xl');
      expect(debugPanel).toBeInTheDocument();

      const border = container.querySelector('.border-green-500');
      expect(border).toBeInTheDocument();
    });
  });
});
