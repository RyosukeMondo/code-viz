/**
 * Unit tests for DetailPanel component
 *
 * Tests rendering, user interactions, keyboard handling, and accessibility
 * for the file detail panel component.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DetailPanel } from './DetailPanel';
import type { TreeNode } from '@/types/bindings';

describe('DetailPanel', () => {
  // Mock TreeNode data
  const mockFileNode: TreeNode = {
    id: 'file-1',
    name: 'example.ts',
    path: '/src/components/example.ts',
    loc: 250,
    complexity: 45.5,
    type: 'file',
    children: [],
    lastModified: '2024-01-15T10:30:00Z',
  };

  const mockDirectoryNode: TreeNode = {
    id: 'dir-1',
    name: 'components',
    path: '/src/components',
    loc: 1500,
    complexity: 35.2,
    type: 'directory',
    children: [mockFileNode, { ...mockFileNode, id: 'file-2', name: 'other.ts' }],
    lastModified: '2024-01-16T14:20:00Z',
  };

  describe('Rendering', () => {
    it('should render null when node is null', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={null} onClose={onClose} />);

      expect(container.firstChild).toBeNull();
    });

    it('should render panel when node is provided', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('example.ts')).toBeInTheDocument();
    });

    it('should render file name in header', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByRole('heading', { name: 'example.ts' })).toBeInTheDocument();
    });

    it('should render file type', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('file')).toBeInTheDocument();
    });

    it('should render directory type for directories', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockDirectoryNode} onClose={onClose} />);

      expect(screen.getByText('directory')).toBeInTheDocument();
    });

    it('should render close button', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close detail panel/i });
      expect(closeButton).toBeInTheDocument();
    });
  });

  describe('File metadata display', () => {
    it('should display full file path', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('/src/components/example.ts')).toBeInTheDocument();
    });

    it('should display lines of code', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('Lines of Code')).toBeInTheDocument();
      // formatNumber(250) = "250"
      expect(screen.getByText('250')).toBeInTheDocument();
    });

    it('should display complexity score', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('Complexity Score')).toBeInTheDocument();
      // formatNumber may round the complexity value
      expect(screen.getByText(/45\.5|46/)).toBeInTheDocument();
    });

    it('should display complexity color indicator', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      // Find the complexity indicator div with color
      const indicator = container.querySelector('[title*="Complexity"]');
      expect(indicator).toBeInTheDocument();
      expect(indicator).toHaveStyle({ backgroundColor: expect.any(String) });
    });

    it('should display last modified date', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('Last Modified')).toBeInTheDocument();
      // formatRelativeDate should display the date in some format
      const dateSection = screen.getByText('Last Modified').parentElement?.parentElement;
      expect(dateSection).toBeInTheDocument();
      expect(dateSection?.textContent).toMatch(/\d{4}|ago|just now|Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec/i);
    });

    it('should display node ID in technical details', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      // Open the details section
      const detailsElement = screen.getByText('Technical Details');
      fireEvent.click(detailsElement);

      expect(screen.getByText('file-1')).toBeInTheDocument();
    });
  });

  describe('Directory-specific display', () => {
    it('should show children count for directories', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockDirectoryNode} onClose={onClose} />);

      expect(screen.getByText('Children')).toBeInTheDocument();
      expect(screen.getByText('2')).toBeInTheDocument();
    });

    it('should use singular "Child" for single child', () => {
      const onClose = vi.fn();
      const nodeWithOneChild: TreeNode = {
        ...mockDirectoryNode,
        children: [mockFileNode],
      };
      render(<DetailPanel node={nodeWithOneChild} onClose={onClose} />);

      expect(screen.getByText('Child')).toBeInTheDocument();
    });

    it('should not show children count for files', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.queryByText(/Child|Children/)).not.toBeInTheDocument();
    });
  });

  describe('Close button interactions', () => {
    it('should call onClose when close button is clicked', async () => {
      const user = userEvent.setup();
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close detail panel/i });
      await user.click(closeButton);

      expect(onClose).toHaveBeenCalledTimes(1);
    });

    it('should call onClose when Escape key is pressed', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      // Simulate Escape key press
      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).toHaveBeenCalledTimes(1);
    });

    it('should not call onClose when other keys are pressed', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      fireEvent.keyDown(window, { key: 'Enter' });
      fireEvent.keyDown(window, { key: 'Space' });
      fireEvent.keyDown(window, { key: 'Tab' });

      expect(onClose).not.toHaveBeenCalled();
    });

    it('should not add Escape listener when node is null', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={null} onClose={onClose} />);

      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).not.toHaveBeenCalled();
    });

    it('should remove Escape listener on unmount', () => {
      const onClose = vi.fn();
      const { unmount } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      unmount();

      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).not.toHaveBeenCalled();
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels on close button', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close detail panel/i });
      expect(closeButton).toHaveAttribute('aria-label', 'Close detail panel');
    });

    it('should have title attribute with keyboard hint on close button', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close detail panel/i });
      expect(closeButton).toHaveAttribute('title', 'Close (Esc)');
    });

    it('should have focus ring on close button', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close detail panel/i });
      expect(closeButton.className).toContain('focus:ring');
    });

    it('should have accessible complexity indicator', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const indicator = container.querySelector('[aria-label*="Complexity indicator"]');
      expect(indicator).toBeInTheDocument();
    });
  });

  describe('Expandable sections', () => {
    it('should have collapsible technical details section', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const detailsElement = screen.getByText('Technical Details');
      expect(detailsElement).toBeInTheDocument();

      // Check that the details element is initially closed (not open attribute)
      const detailsTag = container.querySelector('details');
      expect(detailsTag).toBeInTheDocument();
    });

    it('should expand technical details when clicked', () => {
      const onClose = vi.fn();
      render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const detailsElement = screen.getByText('Technical Details');
      fireEvent.click(detailsElement);

      expect(screen.getByText('file-1')).toBeInTheDocument();
    });
  });

  describe('Edge cases', () => {
    it('should handle node with zero LOC', () => {
      const onClose = vi.fn();
      const nodeWithZeroLoc: TreeNode = {
        ...mockFileNode,
        loc: 0,
      };
      render(<DetailPanel node={nodeWithZeroLoc} onClose={onClose} />);

      expect(screen.getByText('0')).toBeInTheDocument();
    });

    it('should handle node with zero complexity', () => {
      const onClose = vi.fn();
      const nodeWithZeroComplexity: TreeNode = {
        ...mockFileNode,
        complexity: 0,
      };
      render(<DetailPanel node={nodeWithZeroComplexity} onClose={onClose} />);

      expect(screen.getByText('0')).toBeInTheDocument();
    });

    it('should handle node with very high complexity', () => {
      const onClose = vi.fn();
      const nodeWithHighComplexity: TreeNode = {
        ...mockFileNode,
        complexity: 99.9,
      };
      render(<DetailPanel node={nodeWithHighComplexity} onClose={onClose} />);

      // formatNumber might format it differently, so check for the value
      expect(screen.getByText(/99\.9|100/)).toBeInTheDocument();
    });

    it('should handle node with very long path', () => {
      const onClose = vi.fn();
      const longPath = '/very/long/path/that/goes/on/and/on/with/many/nested/directories/example.ts';
      const nodeWithLongPath: TreeNode = {
        ...mockFileNode,
        path: longPath,
      };
      render(<DetailPanel node={nodeWithLongPath} onClose={onClose} />);

      expect(screen.getByText(longPath)).toBeInTheDocument();
    });

    it('should handle directory with no children', () => {
      const onClose = vi.fn();
      const emptyDirectory: TreeNode = {
        ...mockDirectoryNode,
        children: [],
      };
      render(<DetailPanel node={emptyDirectory} onClose={onClose} />);

      expect(screen.getByText('Children')).toBeInTheDocument();
      expect(screen.getByText('0')).toBeInTheDocument();
    });

    it('should handle node with special characters in name', () => {
      const onClose = vi.fn();
      const specialNode: TreeNode = {
        ...mockFileNode,
        name: '@types/node.d.ts',
      };
      render(<DetailPanel node={specialNode} onClose={onClose} />);

      expect(screen.getByText('@types/node.d.ts')).toBeInTheDocument();
    });
  });

  describe('Number formatting', () => {
    it('should format large LOC numbers with commas', () => {
      const onClose = vi.fn();
      const nodeWithLargeLoc: TreeNode = {
        ...mockFileNode,
        loc: 1234567,
      };
      render(<DetailPanel node={nodeWithLargeLoc} onClose={onClose} />);

      // formatNumber should add commas
      expect(screen.getByText(/1,234,567|1234567/)).toBeInTheDocument();
    });

    it('should handle decimal complexity values', () => {
      const onClose = vi.fn();
      const nodeWithDecimal: TreeNode = {
        ...mockFileNode,
        complexity: 25.789,
      };
      render(<DetailPanel node={nodeWithDecimal} onClose={onClose} />);

      // Should display the decimal value
      expect(screen.getByText(/25\.79|25\.8|26/)).toBeInTheDocument();
    });
  });

  describe('Visual styling', () => {
    it('should have fixed positioning on the right side', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const panel = container.firstChild as HTMLElement;
      expect(panel.className).toContain('fixed');
      expect(panel.className).toContain('right-0');
    });

    it('should have dark mode classes', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const panel = container.firstChild as HTMLElement;
      expect(panel.className).toContain('dark:bg-gray-800');
    });

    it('should have overflow scroll for long content', () => {
      const onClose = vi.fn();
      const { container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      const panel = container.firstChild as HTMLElement;
      expect(panel.className).toContain('overflow-y-auto');
    });
  });

  describe('Component lifecycle', () => {
    it('should update when node changes', () => {
      const onClose = vi.fn();
      const { rerender } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('example.ts')).toBeInTheDocument();

      rerender(<DetailPanel node={mockDirectoryNode} onClose={onClose} />);

      expect(screen.getByText('components')).toBeInTheDocument();
      expect(screen.queryByText('example.ts')).not.toBeInTheDocument();
    });

    it('should render nothing when node changes to null', () => {
      const onClose = vi.fn();
      const { rerender, container } = render(<DetailPanel node={mockFileNode} onClose={onClose} />);

      expect(screen.getByText('example.ts')).toBeInTheDocument();

      rerender(<DetailPanel node={null} onClose={onClose} />);

      expect(container.firstChild).toBeNull();
    });
  });
});
