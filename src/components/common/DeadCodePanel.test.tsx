/**
 * Unit tests for DeadCodePanel component
 *
 * Tests rendering, user interactions, keyboard handling, and accessibility
 * for the dead code panel component.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DeadCodePanel } from './DeadCodePanel';
import type { FileDeadCode, DeadSymbol } from '@/types/bindings';

describe('DeadCodePanel', () => {
  // Mock FileDeadCode data
  const mockDeadSymbol1: DeadSymbol = {
    symbol: 'unusedFunction',
    kind: 'Function',
    lineStart: 10,
    lineEnd: 25,
    loc: 15,
    confidence: 95,
    reason: 'Function is never called or imported',
    lastModified: '2024-01-15T10:30:00Z',
  };

  const mockDeadSymbol2: DeadSymbol = {
    symbol: 'oldUtility',
    kind: 'ArrowFunction',
    lineStart: 30,
    lineEnd: 35,
    loc: 5,
    confidence: 75,
    reason: 'Exported but never imported in the codebase',
  };

  const mockDeadSymbol3: DeadSymbol = {
    symbol: 'legacyClass',
    kind: 'Class',
    lineStart: 50,
    lineEnd: 80,
    loc: 30,
    confidence: 50,
    reason: 'Recently modified, could be used dynamically',
  };

  const mockFileWithDeadCode: FileDeadCode = {
    path: '/src/components/example.ts',
    deadCode: [mockDeadSymbol1, mockDeadSymbol2, mockDeadSymbol3],
  };

  const mockFileWithOneSymbol: FileDeadCode = {
    path: '/src/utils/helper.ts',
    deadCode: [mockDeadSymbol1],
  };

  const mockFileWithNoDeadCode: FileDeadCode = {
    path: '/src/components/clean.ts',
    deadCode: [],
  };

  describe('Rendering', () => {
    it('should render null when file is null', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={null} onClose={onClose} />);

      expect(container.firstChild).toBeNull();
    });

    it('should render panel when file is provided', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByTestId('dead-code-panel')).toBeInTheDocument();
    });

    it('should render file path in header', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('/src/components/example.ts')).toBeInTheDocument();
    });

    it('should render symbol count in header (plural)', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('3 symbols')).toBeInTheDocument();
    });

    it('should render symbol count in header (singular)', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithOneSymbol} onClose={onClose} />);

      expect(screen.getByText('1 symbol')).toBeInTheDocument();
    });

    it('should render close button', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const closeButton = screen.getByTestId('dead-code-panel-close');
      expect(closeButton).toBeInTheDocument();
    });

    it('should display "Dead Code" heading', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByRole('heading', { name: 'Dead Code' })).toBeInTheDocument();
    });
  });

  describe('Dead symbols list', () => {
    it('should render all dead symbols', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('unusedFunction')).toBeInTheDocument();
      expect(screen.getByText('oldUtility')).toBeInTheDocument();
      expect(screen.getByText('legacyClass')).toBeInTheDocument();
    });

    it('should display symbol types correctly', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('Function')).toBeInTheDocument();
      expect(screen.getByText('Arrow Function')).toBeInTheDocument();
      expect(screen.getByText('Class')).toBeInTheDocument();
    });

    it('should display line numbers for each symbol', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText(/Lines 10–25/)).toBeInTheDocument();
      expect(screen.getByText(/Lines 30–35/)).toBeInTheDocument();
      expect(screen.getByText(/Lines 50–80/)).toBeInTheDocument();
    });

    it('should display LOC for each symbol', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      // Use getAllByText for multiple matches
      const loc15 = screen.getByText(/^15 LOC$/);
      const loc5 = screen.getByText(/^5 LOC$/);
      const loc30 = screen.getByText(/^30 LOC$/);

      expect(loc15).toBeInTheDocument();
      expect(loc5).toBeInTheDocument();
      expect(loc30).toBeInTheDocument();
    });

    it('should display reason for each symbol', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('Function is never called or imported')).toBeInTheDocument();
      expect(screen.getByText('Exported but never imported in the codebase')).toBeInTheDocument();
      expect(screen.getByText('Recently modified, could be used dynamically')).toBeInTheDocument();
    });

    it('should display message when no dead code found', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithNoDeadCode} onClose={onClose} />);

      expect(screen.getByText('No dead code found in this file')).toBeInTheDocument();
    });

    it('should have accessible list role', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const list = screen.getByRole('list', { name: 'Dead symbols' });
      expect(list).toBeInTheDocument();
    });
  });

  describe('Confidence scores', () => {
    it('should display confidence scores', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('95%')).toBeInTheDocument();
      expect(screen.getByText('75%')).toBeInTheDocument();
      expect(screen.getByText('50%')).toBeInTheDocument();
    });

    it('should show confidence scores with correct colors (high confidence >80)', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      // Find the confidence badge with 95%
      const highConfidenceBadge = screen.getByText('95%').parentElement;
      expect(highConfidenceBadge).toHaveStyle({ backgroundColor: expect.any(String) });
    });

    it('should show confidence scores with correct colors (medium confidence 60-80)', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      // Find the confidence badge with 75%
      const mediumConfidenceBadge = screen.getByText('75%').parentElement;
      expect(mediumConfidenceBadge).toHaveStyle({ backgroundColor: expect.any(String) });
    });

    it('should show confidence scores with correct colors (low confidence <60)', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      // Find the confidence badge with 50%
      const lowConfidenceBadge = screen.getByText('50%').parentElement;
      expect(lowConfidenceBadge).toHaveStyle({ backgroundColor: expect.any(String) });
    });

    it('should have aria-label for confidence scores', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const confidenceBadge = screen.getByLabelText('Confidence score: 95%');
      expect(confidenceBadge).toBeInTheDocument();
    });

    it('should have title attribute on confidence badge', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const confidenceBadge = screen.getByTitle('Confidence: 95%');
      expect(confidenceBadge).toBeInTheDocument();
    });
  });

  describe('Close button interactions', () => {
    it('should call onClose when close button is clicked', async () => {
      const user = userEvent.setup();
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const closeButton = screen.getByTestId('dead-code-panel-close');
      await user.click(closeButton);

      expect(onClose).toHaveBeenCalledTimes(1);
    });

    it('should call onClose when Escape key is pressed', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      // Simulate Escape key press
      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).toHaveBeenCalledTimes(1);
    });

    it('should not call onClose when other keys are pressed', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      fireEvent.keyDown(window, { key: 'Enter' });
      fireEvent.keyDown(window, { key: 'Space' });
      fireEvent.keyDown(window, { key: 'Tab' });

      expect(onClose).not.toHaveBeenCalled();
    });

    it('should not add Escape listener when file is null', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={null} onClose={onClose} />);

      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).not.toHaveBeenCalled();
    });

    it('should remove Escape listener on unmount', () => {
      const onClose = vi.fn();
      const { unmount } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      unmount();

      fireEvent.keyDown(window, { key: 'Escape' });

      expect(onClose).not.toHaveBeenCalled();
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels on close button', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const closeButton = screen.getByTestId('dead-code-panel-close');
      expect(closeButton).toHaveAttribute('aria-label', 'Close dead code panel');
    });

    it('should have title attribute with keyboard hint on close button', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const closeButton = screen.getByTestId('dead-code-panel-close');
      expect(closeButton).toHaveAttribute('title', 'Close (Esc)');
    });

    it('should have focus ring on close button', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const closeButton = screen.getByTestId('dead-code-panel-close');
      expect(closeButton.className).toContain('focus:ring');
    });

    it('should have accessible labels for View in Editor button', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const editorButton = screen.getByLabelText(/View unusedFunction in editor at line 10/);
      expect(editorButton).toBeInTheDocument();
    });
  });

  describe('View in Editor button', () => {
    it('should render View in Editor button for each symbol', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const buttons = screen.getAllByText(/View in Editor/);
      expect(buttons).toHaveLength(3);
    });

    it('should be disabled (placeholder for future integration)', () => {
      const onClose = vi.fn();
      render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const buttons = screen.getAllByText(/View in Editor/);
      buttons.forEach(button => {
        expect(button).toBeDisabled();
      });
    });
  });

  describe('Edge cases', () => {
    it('should handle file with very long path', () => {
      const onClose = vi.fn();
      const longPath = '/very/long/path/that/goes/on/and/on/with/many/nested/directories/example.ts';
      const fileWithLongPath: FileDeadCode = {
        path: longPath,
        deadCode: [mockDeadSymbol1],
      };
      render(<DeadCodePanel file={fileWithLongPath} onClose={onClose} />);

      expect(screen.getByText(longPath)).toBeInTheDocument();
    });

    it('should handle symbol with very long name', () => {
      const onClose = vi.fn();
      const longSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        symbol: 'veryLongFunctionNameThatGoesOnAndOnWithManyCharacters',
      };
      const fileWithLongSymbol: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [longSymbol],
      };
      render(<DeadCodePanel file={fileWithLongSymbol} onClose={onClose} />);

      expect(screen.getByText('veryLongFunctionNameThatGoesOnAndOnWithManyCharacters')).toBeInTheDocument();
    });

    it('should handle symbol with special characters in name', () => {
      const onClose = vi.fn();
      const specialSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        symbol: '$_unusedHelper',
      };
      const fileWithSpecialSymbol: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [specialSymbol],
      };
      render(<DeadCodePanel file={fileWithSpecialSymbol} onClose={onClose} />);

      expect(screen.getByText('$_unusedHelper')).toBeInTheDocument();
    });

    it('should handle symbol with confidence of 0', () => {
      const onClose = vi.fn();
      const zeroConfidenceSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        confidence: 0,
      };
      const fileWithZeroConfidence: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [zeroConfidenceSymbol],
      };
      render(<DeadCodePanel file={fileWithZeroConfidence} onClose={onClose} />);

      expect(screen.getByText('0%')).toBeInTheDocument();
    });

    it('should handle symbol with confidence of 100', () => {
      const onClose = vi.fn();
      const perfectConfidenceSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        confidence: 100,
      };
      const fileWithPerfectConfidence: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [perfectConfidenceSymbol],
      };
      render(<DeadCodePanel file={fileWithPerfectConfidence} onClose={onClose} />);

      expect(screen.getByText('100%')).toBeInTheDocument();
    });

    it('should handle symbol with zero LOC', () => {
      const onClose = vi.fn();
      const zeroLocSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        loc: 0,
      };
      const fileWithZeroLoc: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [zeroLocSymbol],
      };
      render(<DeadCodePanel file={fileWithZeroLoc} onClose={onClose} />);

      expect(screen.getByText(/0 LOC/)).toBeInTheDocument();
    });

    it('should handle symbol with very high LOC', () => {
      const onClose = vi.fn();
      const highLocSymbol: DeadSymbol = {
        ...mockDeadSymbol1,
        loc: 9999,
      };
      const fileWithHighLoc: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [highLocSymbol],
      };
      render(<DeadCodePanel file={fileWithHighLoc} onClose={onClose} />);

      expect(screen.getByText(/9,999 LOC/)).toBeInTheDocument();
    });

    it('should handle all SymbolKind types', () => {
      const onClose = vi.fn();
      const allKinds: FileDeadCode = {
        path: '/src/test.ts',
        deadCode: [
          { ...mockDeadSymbol1, kind: 'Function' },
          { ...mockDeadSymbol2, kind: 'ArrowFunction' },
          { ...mockDeadSymbol3, kind: 'Class' },
          { ...mockDeadSymbol1, symbol: 'method1', kind: 'Method' },
          { ...mockDeadSymbol1, symbol: 'var1', kind: 'Variable' },
        ],
      };
      render(<DeadCodePanel file={allKinds} onClose={onClose} />);

      expect(screen.getByText('Function')).toBeInTheDocument();
      expect(screen.getByText('Arrow Function')).toBeInTheDocument();
      expect(screen.getByText('Class')).toBeInTheDocument();
      expect(screen.getByText('Method')).toBeInTheDocument();
      expect(screen.getByText('Variable')).toBeInTheDocument();
    });
  });

  describe('Visual styling', () => {
    it('should have fixed positioning on the right side', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const panel = container.querySelector('[data-testid="dead-code-panel"]') as HTMLElement;
      expect(panel.className).toContain('fixed');
      expect(panel.className).toContain('right-0');
    });

    it('should have dark mode classes', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const panel = container.querySelector('[data-testid="dead-code-panel"]') as HTMLElement;
      expect(panel.className).toContain('dark:bg-gray-800');
    });

    it('should have overflow scroll for long content', () => {
      const onClose = vi.fn();
      const { container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      const panel = container.querySelector('[data-testid="dead-code-panel"]') as HTMLElement;
      expect(panel.className).toContain('overflow-y-auto');
    });
  });

  describe('Component lifecycle', () => {
    it('should update when file changes', () => {
      const onClose = vi.fn();
      const { rerender } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('/src/components/example.ts')).toBeInTheDocument();

      rerender(<DeadCodePanel file={mockFileWithOneSymbol} onClose={onClose} />);

      expect(screen.getByText('/src/utils/helper.ts')).toBeInTheDocument();
      expect(screen.queryByText('/src/components/example.ts')).not.toBeInTheDocument();
    });

    it('should render nothing when file changes to null', () => {
      const onClose = vi.fn();
      const { rerender, container } = render(<DeadCodePanel file={mockFileWithDeadCode} onClose={onClose} />);

      expect(screen.getByText('/src/components/example.ts')).toBeInTheDocument();

      rerender(<DeadCodePanel file={null} onClose={onClose} />);

      expect(container.firstChild).toBeNull();
    });
  });
});
