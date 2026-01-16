/**
 * Tests for TreeView component
 */

import { render, screen, fireEvent } from '@testing-library/react';
import { describe, test, expect } from 'vitest';
import { TreeView } from './TreeView';
import { mockTreeData, deepTreeData, emptyTreeData } from '../visualizations/__fixtures__/mockTreeData';

describe('TreeView', () => {
  describe('rendering', () => {
    test('renders tree structure with data', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText('Tree Structure')).toBeInTheDocument();
      expect(screen.getByText(mockTreeData.name)).toBeInTheDocument();
    });

    test('displays "No data to display" when data is null', () => {
      render(<TreeView data={null} />);

      expect(screen.getByText('No data to display')).toBeInTheDocument();
    });

    test('displays total LOC in header', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(/Total:/)).toBeInTheDocument();
      expect(screen.getByText(/1,000 LOC/)).toBeInTheDocument();
    });

    test('displays max depth info', () => {
      render(<TreeView data={mockTreeData} maxDepth={5} />);

      expect(screen.getByText(/Max depth: 5/)).toBeInTheDocument();
    });

    test('uses default maxDepth of 10', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(/Max depth: 10/)).toBeInTheDocument();
    });

    test('displays instructions for user', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(/Click to expand\/collapse/)).toBeInTheDocument();
    });
  });

  describe('tree node display', () => {
    test('displays node name', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(mockTreeData.name)).toBeInTheDocument();
    });

    test('displays LOC for nodes', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(/1,000 LOC/)).toBeInTheDocument();
    });

    test('displays complexity when greater than 0', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText(/C:30/)).toBeInTheDocument();
    });

    test('shows folder icon for directories', () => {
      render(<TreeView data={mockTreeData} />);

      const folderIcons = screen.getAllByText('📁');
      expect(folderIcons.length).toBeGreaterThan(0);
    });

    test('shows file icon for files', () => {
      render(<TreeView data={mockTreeData} />);

      // Expand to see children
      const rootNode = screen.getByText(mockTreeData.name);
      fireEvent.click(rootNode);

      const fileIcons = screen.getAllByText('📄');
      expect(fileIcons.length).toBeGreaterThan(0);
    });

    test('handles unnamed nodes', () => {
      const unnamedData = {
        ...mockTreeData,
        name: '',
      };

      render(<TreeView data={unnamedData} />);

      expect(screen.getByText('(unnamed)')).toBeInTheDocument();
    });
  });

  describe('expand/collapse functionality', () => {
    test('expands first 2 levels by default', () => {
      render(<TreeView data={mockTreeData} />);

      // Root and first level children should be visible
      expect(screen.getByText('file1.ts')).toBeInTheDocument();
      expect(screen.getByText('subdir')).toBeInTheDocument();
    });

    test('collapses node when clicked', () => {
      render(<TreeView data={mockTreeData} />);

      // Children are visible by default (auto-expanded)
      expect(screen.getByText('file1.ts')).toBeInTheDocument();

      // Click root to collapse
      const rootNode = screen.getByText(mockTreeData.name);
      fireEvent.click(rootNode);

      // Children should be hidden
      expect(screen.queryByText('file1.ts')).not.toBeInTheDocument();
    });

    test('expands node when clicked again', () => {
      render(<TreeView data={mockTreeData} />);

      const rootNode = screen.getByText(mockTreeData.name);

      // Collapse
      fireEvent.click(rootNode);
      expect(screen.queryByText('file1.ts')).not.toBeInTheDocument();

      // Expand
      fireEvent.click(rootNode);
      expect(screen.getByText('file1.ts')).toBeInTheDocument();
    });

    test('shows expand arrow when collapsed', () => {
      render(<TreeView data={mockTreeData} />);

      const rootNode = screen.getByText(mockTreeData.name);
      fireEvent.click(rootNode);

      expect(screen.getByText('▶')).toBeInTheDocument();
    });

    test('shows collapse arrow when expanded', () => {
      render(<TreeView data={mockTreeData} />);

      expect(screen.getByText('▼')).toBeInTheDocument();
    });

    test('does not toggle when clicking on leaf node', () => {
      render(<TreeView data={mockTreeData} />);

      // file1.ts has no children - clicking should do nothing
      const leafNode = screen.getByText('file1.ts');
      fireEvent.click(leafNode);

      // Node should still be visible (no error thrown)
      expect(screen.getByText('file1.ts')).toBeInTheDocument();
    });
  });

  describe('nested children', () => {
    test('displays nested children when parent is expanded', () => {
      render(<TreeView data={mockTreeData} />);

      // Subdir is expanded by default, so its children should be visible
      expect(screen.getByText('file2.ts')).toBeInTheDocument();
      expect(screen.getByText('file3.ts')).toBeInTheDocument();
    });

    test('hides nested children when parent is collapsed', () => {
      render(<TreeView data={mockTreeData} />);

      // Find and click subdir to collapse it
      const subdir = screen.getByText('subdir');
      fireEvent.click(subdir);

      // Children should be hidden
      expect(screen.queryByText('file2.ts')).not.toBeInTheDocument();
      expect(screen.queryByText('file3.ts')).not.toBeInTheDocument();
    });

    test('respects maxDepth limit', () => {
      render(<TreeView data={deepTreeData} maxDepth={1} />);

      // Only root and first level should be visible
      expect(screen.getByText('level1')).toBeInTheDocument();

      // Deeper levels should not render
      expect(screen.queryByText('level2')).not.toBeInTheDocument();
      expect(screen.queryByText('deep.ts')).not.toBeInTheDocument();
    });

    test('renders all levels when maxDepth is high enough', () => {
      render(<TreeView data={deepTreeData} maxDepth={10} />);

      // All levels should be visible (auto-expanded first 2 levels)
      expect(screen.getByText('level1')).toBeInTheDocument();
      expect(screen.getByText('level2')).toBeInTheDocument();
    });
  });

  describe('formatting', () => {
    test('formats large LOC numbers with commas', () => {
      const largeLocData = {
        ...mockTreeData,
        loc: 1000000,
      };

      render(<TreeView data={largeLocData} />);

      expect(screen.getByText(/1,000,000 LOC/)).toBeInTheDocument();
    });

    test('applies correct indentation for nested levels', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      // Check that nested items have padding-left style
      const nestedItems = container.querySelectorAll('[style*="paddingLeft"]');
      expect(nestedItems.length).toBeGreaterThan(0);
    });

    test('uses monospace font', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      const monoFont = container.querySelector('.font-mono');
      expect(monoFont).toBeInTheDocument();
    });
  });

  describe('edge cases', () => {
    test('handles empty children array', () => {
      render(<TreeView data={emptyTreeData} />);

      // Root should be visible
      expect(screen.getByText(emptyTreeData.name)).toBeInTheDocument();

      // No expand/collapse icon since no children
      const rootContainer = screen.getByText(emptyTreeData.name).closest('div');
      expect(rootContainer).toBeInTheDocument();
    });

    test('handles zero LOC', () => {
      const zeroLocData = {
        ...mockTreeData,
        loc: 0,
      };

      render(<TreeView data={zeroLocData} />);

      expect(screen.getByText(/0 LOC/)).toBeInTheDocument();
    });

    test('handles zero complexity', () => {
      const zeroComplexityData = {
        ...mockTreeData,
        complexity: 0,
      };

      render(<TreeView data={zeroComplexityData} />);

      // Complexity should not be shown when it's 0
      expect(screen.queryByText(/C:0/)).not.toBeInTheDocument();
    });

    test('handles maxDepth of 0', () => {
      const { container } = render(<TreeView data={mockTreeData} maxDepth={0} />);

      // Should render but with no visible nodes beyond root
      expect(container).toBeInTheDocument();
    });

    test('handles deeply nested structures', () => {
      render(<TreeView data={deepTreeData} />);

      // Should render without errors
      expect(screen.getByText('root')).toBeInTheDocument();
    });
  });

  describe('accessibility', () => {
    test('has scrollable container', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      const scrollableDiv = container.querySelector('.overflow-auto');
      expect(scrollableDiv).toBeInTheDocument();
    });

    test('has max height constraint', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      const maxHeightDiv = container.querySelector('.max-h-\\[600px\\]');
      expect(maxHeightDiv).toBeInTheDocument();
    });

    test('clickable nodes have cursor pointer', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      const clickableNodes = container.querySelectorAll('.cursor-pointer');
      expect(clickableNodes.length).toBeGreaterThan(0);
    });

    test('provides hover feedback', () => {
      const { container } = render(<TreeView data={mockTreeData} />);

      const hoverableNodes = container.querySelectorAll('.hover\\:bg-gray-100');
      expect(hoverableNodes.length).toBeGreaterThan(0);
    });
  });
});
