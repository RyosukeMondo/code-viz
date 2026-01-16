/**
 * Tests for InfoPanel component
 * @module components/visualizations/3d/ui/InfoPanel.test
 */

import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { InfoPanel } from './InfoPanel';
import type { LayoutNode } from '../types';

describe('InfoPanel', () => {
  const mockNode: LayoutNode = {
    id: 'test-1',
    name: 'test.ts',
    path: '/src/components/test.ts',
    x0: 0,
    y0: 0,
    x1: 100,
    y1: 100,
    height: 200,
    color: '#00ff00',
    metrics: {
      loc: 1500,
      complexity: 15,
      functions: 25,
      lastModified: '2024-01-15T12:00:00Z'
    }
  };

  describe('rendering', () => {
    it('should not render when data is null', () => {
      const { container } = render(<InfoPanel data={null} />);
      expect(container.firstChild).toBeNull();
    });

    it('should render when data is provided', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });

    it('should display file path', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByText('/src/components/test.ts')).toBeInTheDocument();
    });

    it('should display lines of code with formatting', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByText('1,500')).toBeInTheDocument();
    });

    it('should display complexity', () => {
      render(<InfoPanel data={mockNode} />);
      const complexityText = screen.getByText('15');
      expect(complexityText).toBeInTheDocument();
      expect(screen.getByText(/Medium/)).toBeInTheDocument();
    });

    it('should display functions count when provided', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByText('25')).toBeInTheDocument();
    });

    it('should display last modified date when provided', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByText(/Jan 15, 2024/)).toBeInTheDocument();
    });

    it('should not display functions section when undefined', () => {
      const nodeWithoutFunctions: LayoutNode = {
        ...mockNode,
        metrics: {
          loc: 100,
          complexity: 5
        }
      };

      render(<InfoPanel data={nodeWithoutFunctions} />);
      expect(screen.queryByText('FUNCTIONS')).not.toBeInTheDocument();
    });

    it('should not display last modified section when undefined', () => {
      const nodeWithoutDate: LayoutNode = {
        ...mockNode,
        metrics: {
          loc: 100,
          complexity: 5
        }
      };

      render(<InfoPanel data={nodeWithoutDate} />);
      expect(screen.queryByText('LAST MODIFIED')).not.toBeInTheDocument();
    });

    it('should render with accessibility attributes', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByRole('dialog')).toHaveAttribute('aria-label', 'File information panel');
    });
  });

  describe('path truncation', () => {
    it('should truncate long paths', () => {
      const longPathNode: LayoutNode = {
        ...mockNode,
        path: '/very/long/path/to/some/nested/directory/structure/file.ts'
      };

      render(<InfoPanel data={longPathNode} maxPathLength={30} />);
      const pathElement = screen.getByTitle(longPathNode.path);
      expect(pathElement.textContent).toContain('...');
      expect(pathElement.textContent).toContain('file.ts');
    });

    it('should not truncate short paths', () => {
      const shortPathNode: LayoutNode = {
        ...mockNode,
        path: '/short.ts'
      };

      render(<InfoPanel data={shortPathNode} maxPathLength={50} />);
      expect(screen.getByText('/short.ts')).toBeInTheDocument();
    });

    it('should handle very long filename', () => {
      const longFilenameNode: LayoutNode = {
        ...mockNode,
        path: '/src/this-is-a-very-long-filename-that-exceeds-the-max-length.ts'
      };

      render(<InfoPanel data={longFilenameNode} maxPathLength={30} />);
      const pathElement = screen.getByTitle(longFilenameNode.path);
      expect(pathElement.textContent).toContain('...');
    });
  });

  describe('complexity styling', () => {
    it('should use green color for low complexity', () => {
      const lowComplexityNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 5 }
      };

      const { container } = render(<InfoPanel data={lowComplexityNode} />);
      expect(screen.getByText(/Low/)).toBeInTheDocument();
      // The color is applied to the parent div containing the complexity value
      const complexityDiv = container.querySelector('div[style*="color: rgb(34, 197, 94)"]');
      expect(complexityDiv).toBeInTheDocument();
      expect(complexityDiv?.textContent).toContain('5');
    });

    it('should use yellow color for medium complexity', () => {
      const mediumComplexityNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 15 }
      };

      const { container } = render(<InfoPanel data={mediumComplexityNode} />);
      expect(screen.getByText(/Medium/)).toBeInTheDocument();
      const complexityDiv = container.querySelector('div[style*="color: rgb(234, 179, 8)"]');
      expect(complexityDiv).toBeInTheDocument();
      expect(complexityDiv?.textContent).toContain('15');
    });

    it('should use red color for high complexity', () => {
      const highComplexityNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 25 }
      };

      const { container } = render(<InfoPanel data={highComplexityNode} />);
      expect(screen.getByText(/High/)).toBeInTheDocument();
      const complexityDiv = container.querySelector('div[style*="color: rgb(239, 68, 68)"]');
      expect(complexityDiv).toBeInTheDocument();
      expect(complexityDiv?.textContent).toContain('25');
    });

    it('should use very high label for complexity >= 30', () => {
      const veryHighComplexityNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 35 }
      };

      render(<InfoPanel data={veryHighComplexityNode} />);
      expect(screen.getByText(/Very High/)).toBeInTheDocument();
    });
  });

  describe('positioning', () => {
    it('should use bottom-left position by default', () => {
      const { container } = render(<InfoPanel data={mockNode} />);
      const panel = container.firstChild as HTMLElement;
      expect(panel.style.bottom).toBe('20px');
      expect(panel.style.left).toBe('20px');
    });

    it('should support top-left position', () => {
      const { container } = render(<InfoPanel data={mockNode} position="top-left" />);
      const panel = container.firstChild as HTMLElement;
      expect(panel.style.top).toBe('20px');
      expect(panel.style.left).toBe('20px');
    });

    it('should support top-right position', () => {
      const { container } = render(<InfoPanel data={mockNode} position="top-right" />);
      const panel = container.firstChild as HTMLElement;
      expect(panel.style.top).toBe('20px');
      expect(panel.style.right).toBe('20px');
    });

    it('should support bottom-right position', () => {
      const { container } = render(<InfoPanel data={mockNode} position="bottom-right" />);
      const panel = container.firstChild as HTMLElement;
      expect(panel.style.bottom).toBe('20px');
      expect(panel.style.right).toBe('20px');
    });
  });

  describe('close button', () => {
    it('should render close button when onClose is provided', () => {
      const onClose = vi.fn();
      render(<InfoPanel data={mockNode} onClose={onClose} />);
      expect(screen.getByRole('button', { name: /close/i })).toBeInTheDocument();
    });

    it('should not render close button when onClose is not provided', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.queryByRole('button', { name: /close/i })).not.toBeInTheDocument();
    });

    it('should call onClose when close button is clicked', () => {
      const onClose = vi.fn();
      render(<InfoPanel data={mockNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close/i });
      fireEvent.click(closeButton);

      expect(onClose).toHaveBeenCalledTimes(1);
    });

    it('should have hover effect on close button', () => {
      const onClose = vi.fn();
      render(<InfoPanel data={mockNode} onClose={onClose} />);

      const closeButton = screen.getByRole('button', { name: /close/i }) as HTMLButtonElement;
      const initialBackground = closeButton.style.background;

      fireEvent.mouseOver(closeButton);
      const hoverBackground = closeButton.style.background;
      expect(hoverBackground).not.toBe(initialBackground);

      fireEvent.mouseOut(closeButton);
      const finalBackground = closeButton.style.background;
      expect(finalBackground).toBe(initialBackground);
    });
  });

  describe('number formatting', () => {
    it('should format large numbers with commas', () => {
      const largeNumberNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 123456, complexity: 5 }
      };

      render(<InfoPanel data={largeNumberNode} />);
      expect(screen.getByText('123,456')).toBeInTheDocument();
    });

    it('should format functions count with commas', () => {
      const manyFunctionsNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 1000, complexity: 5, functions: 1234 }
      };

      render(<InfoPanel data={manyFunctionsNode} />);
      expect(screen.getByText('1,234')).toBeInTheDocument();
    });
  });

  describe('date formatting', () => {
    it('should format valid date strings', () => {
      const dateNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 5, lastModified: '2023-12-25T10:30:00Z' }
      };

      render(<InfoPanel data={dateNode} />);
      expect(screen.getByText(/Dec 25, 2023/)).toBeInTheDocument();
    });

    it('should handle invalid date strings', () => {
      const invalidDateNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 5, lastModified: 'invalid-date' }
      };

      render(<InfoPanel data={invalidDateNode} />);
      // Invalid dates should show "Invalid Date" (capital D) based on Date constructor behavior
      expect(screen.getByText(/Invalid Date/i)).toBeInTheDocument();
    });

    it('should handle undefined lastModified gracefully', () => {
      const noDateNode: LayoutNode = {
        ...mockNode,
        metrics: { loc: 100, complexity: 5, lastModified: undefined }
      };

      const { container } = render(<InfoPanel data={noDateNode} />);
      expect(container.textContent).not.toContain('Unknown');
      expect(screen.queryByText('LAST MODIFIED')).not.toBeInTheDocument();
    });
  });

  describe('footer', () => {
    it('should display footer hint', () => {
      render(<InfoPanel data={mockNode} />);
      expect(screen.getByText('Click elsewhere to deselect')).toBeInTheDocument();
    });
  });
});
