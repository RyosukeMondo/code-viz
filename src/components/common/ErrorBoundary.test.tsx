/**
 * Tests for ErrorBoundary component
 */

import { render, screen, fireEvent } from '@testing-library/react';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import { ErrorBoundary } from './ErrorBoundary';

// Component that throws an error
function ThrowError({ shouldThrow }: { shouldThrow: boolean }) {
  if (shouldThrow) {
    throw new Error('Test error message');
  }
  return <div>No error</div>;
}

describe('ErrorBoundary', () => {
  beforeEach(() => {
    // Suppress console.error for error boundary tests
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  describe('rendering', () => {
    test('renders children when no error occurs', () => {
      render(
        <ErrorBoundary>
          <div data-testid="child">Child content</div>
        </ErrorBoundary>
      );

      expect(screen.getByTestId('child')).toBeInTheDocument();
      expect(screen.getByText('Child content')).toBeInTheDocument();
    });

    test('renders default fallback UI when error is caught', () => {
      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something Went Wrong')).toBeInTheDocument();
      expect(screen.getByText(/An unexpected error occurred/)).toBeInTheDocument();
      expect(screen.getByText('Try Again')).toBeInTheDocument();
      expect(screen.getByText('Reload Page')).toBeInTheDocument();
    });

    test('renders custom fallback when provided', () => {
      const customFallback = (error: Error, reset: () => void) => (
        <div>
          <div data-testid="custom-error">Custom error: {error.message}</div>
          <button onClick={reset} data-testid="custom-reset">
            Custom Reset
          </button>
        </div>
      );

      render(
        <ErrorBoundary fallback={customFallback}>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      expect(screen.getByTestId('custom-error')).toHaveTextContent(
        'Custom error: Test error message'
      );
      expect(screen.getByTestId('custom-reset')).toBeInTheDocument();
    });

    test('shows error details in collapsible section', () => {
      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      const details = screen.getByText('Error Details');
      expect(details).toBeInTheDocument();

      // Error message should be in a pre tag
      const errorText = screen.getByText(/Test error message/);
      expect(errorText.tagName).toBe('PRE');
    });
  });

  describe('interactions', () => {
    test('resets error state when Try Again is clicked', () => {
      let shouldThrow = true;

      const { rerender } = render(
        <ErrorBoundary>
          <ThrowError shouldThrow={shouldThrow} />
        </ErrorBoundary>
      );

      // Error UI should be visible
      expect(screen.getByText('Something Went Wrong')).toBeInTheDocument();

      // Fix the error condition
      shouldThrow = false;

      // Click Try Again button
      const tryAgainButton = screen.getByText('Try Again');
      fireEvent.click(tryAgainButton);

      // Rerender with fixed component
      rerender(
        <ErrorBoundary>
          <ThrowError shouldThrow={shouldThrow} />
        </ErrorBoundary>
      );

      // Should show the child content now
      expect(screen.getByText('No error')).toBeInTheDocument();
    });

    test('reloads page when Reload Page is clicked', () => {
      const reloadMock = vi.fn();
      Object.defineProperty(window, 'location', {
        value: { reload: reloadMock },
        writable: true,
      });

      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      const reloadButton = screen.getByText('Reload Page');
      fireEvent.click(reloadButton);

      expect(reloadMock).toHaveBeenCalled();
    });

    test('custom fallback reset handler works', () => {
      const resetSpy = vi.fn();
      const customFallback = (_error: Error, reset: () => void) => (
        <button
          onClick={() => {
            resetSpy();
            reset();
          }}
          data-testid="custom-reset"
        >
          Reset
        </button>
      );

      render(
        <ErrorBoundary fallback={customFallback}>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      const resetButton = screen.getByTestId('custom-reset');
      fireEvent.click(resetButton);

      expect(resetSpy).toHaveBeenCalled();
    });
  });

  describe('error handling', () => {
    test('logs error to console', () => {
      const consoleErrorSpy = vi.spyOn(console, 'error');

      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    test('displays error message in UI', () => {
      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      expect(screen.getByText(/Test error message/)).toBeInTheDocument();
    });

    test('handles errors with stack traces', () => {
      render(
        <ErrorBoundary>
          <ThrowError shouldThrow={true} />
        </ErrorBoundary>
      );

      // Stack trace should be present in the error details
      const preElement = document.querySelector('pre');
      expect(preElement).toBeInTheDocument();
      expect(preElement?.textContent).toContain('Test error message');
    });
  });

  describe('edge cases', () => {
    test('handles multiple children', () => {
      render(
        <ErrorBoundary>
          <div>Child 1</div>
          <div>Child 2</div>
          <div>Child 3</div>
        </ErrorBoundary>
      );

      expect(screen.getByText('Child 1')).toBeInTheDocument();
      expect(screen.getByText('Child 2')).toBeInTheDocument();
      expect(screen.getByText('Child 3')).toBeInTheDocument();
    });

    test('handles nested error boundaries', () => {
      render(
        <ErrorBoundary>
          <ErrorBoundary>
            <ThrowError shouldThrow={true} />
          </ErrorBoundary>
        </ErrorBoundary>
      );

      // Inner boundary should catch the error
      expect(screen.getByText('Something Went Wrong')).toBeInTheDocument();
    });

    test('handles errors in nested components', () => {
      const NestedComponent = () => (
        <div>
          <div>
            <ThrowError shouldThrow={true} />
          </div>
        </div>
      );

      render(
        <ErrorBoundary>
          <NestedComponent />
        </ErrorBoundary>
      );

      expect(screen.getByText('Something Went Wrong')).toBeInTheDocument();
    });
  });
});
