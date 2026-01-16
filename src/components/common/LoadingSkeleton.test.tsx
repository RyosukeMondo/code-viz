/**
 * Tests for LoadingSkeleton components
 */

import { render, screen } from '@testing-library/react';
import { describe, test, expect } from 'vitest';
import {
  TreemapSkeleton,
  AnalysisLoadingSkeleton,
  Skeleton,
} from './LoadingSkeleton';

describe('TreemapSkeleton', () => {
  describe('rendering', () => {
    test('renders with correct structure', () => {
      const { container } = render(<TreemapSkeleton />);

      // Should have main container with pulse animation
      const mainContainer = container.querySelector('.animate-pulse');
      expect(mainContainer).toBeInTheDocument();
    });

    test('renders breadcrumb skeleton', () => {
      const { container } = render(<TreemapSkeleton />);

      // Should have breadcrumb elements (multiple rounded elements in a row)
      const breadcrumbElements = container.querySelectorAll(
        '.bg-gray-200.dark\\:bg-gray-700.rounded'
      );
      expect(breadcrumbElements.length).toBeGreaterThan(0);
    });

    test('renders treemap grid skeleton', () => {
      const { container } = render(<TreemapSkeleton />);

      // Should have grid container
      const gridContainer = container.querySelector('.grid');
      expect(gridContainer).toBeInTheDocument();
      expect(gridContainer).toHaveClass('grid-cols-3');
    });

    test('renders stats skeleton at bottom', () => {
      const { container } = render(<TreemapSkeleton />);

      // Should have three stat boxes
      const statBoxes = container.querySelectorAll('.h-16.bg-gray-200');
      expect(statBoxes.length).toBe(3);
    });

    test('has flex-1 for full height', () => {
      const { container } = render(<TreemapSkeleton />);

      const mainContainer = container.firstChild;
      expect(mainContainer).toHaveClass('flex-1');
    });
  });

  describe('dark mode', () => {
    test('includes dark mode classes', () => {
      const { container } = render(<TreemapSkeleton />);

      // Check for dark mode classes
      const darkElements = container.querySelectorAll('.dark\\:bg-gray-700');
      expect(darkElements.length).toBeGreaterThan(0);
    });
  });
});

describe('AnalysisLoadingSkeleton', () => {
  describe('rendering', () => {
    test('renders with full screen container', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      const mainContainer = container.querySelector('.absolute.inset-0');
      expect(mainContainer).toBeInTheDocument();
    });

    test('renders spinning loader', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      const spinner = container.querySelector('.animate-spin');
      expect(spinner).toBeInTheDocument();
      expect(spinner).toHaveClass('rounded-full');
      expect(spinner).toHaveClass('border-b-4');
    });

    test('renders analysis message', () => {
      render(<AnalysisLoadingSkeleton />);

      expect(screen.getByText('Analyzing Repository')).toBeInTheDocument();
      expect(
        screen.getByText(/Scanning files and calculating complexity metrics/)
      ).toBeInTheDocument();
    });

    test('renders progress indicator', () => {
      render(<AnalysisLoadingSkeleton />);

      expect(screen.getByText('Scanning...')).toBeInTheDocument();
    });

    test('renders progress bar', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      // Should have progress bar with blue fill
      const progressBar = container.querySelector('.bg-blue-500');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveClass('rounded-full');
    });

    test('centers content vertically and horizontally', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      const flexContainer = container.querySelector('.flex.items-center.justify-center');
      expect(flexContainer).toBeInTheDocument();
    });
  });

  describe('dark mode', () => {
    test('includes dark mode background', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      const mainContainer = container.querySelector('.dark\\:bg-gray-900');
      expect(mainContainer).toBeInTheDocument();
    });

    test('includes dark mode text colors', () => {
      const { container } = render(<AnalysisLoadingSkeleton />);

      const darkText = container.querySelectorAll('.dark\\:text-gray-100');
      expect(darkText.length).toBeGreaterThan(0);
    });
  });
});

describe('Skeleton', () => {
  describe('rendering', () => {
    test('renders with default classes', () => {
      const { container } = render(<Skeleton />);

      const skeleton = container.firstChild;
      expect(skeleton).toBeInTheDocument();
      expect(skeleton).toHaveClass('animate-pulse');
      expect(skeleton).toHaveClass('bg-gray-200');
      expect(skeleton).toHaveClass('dark:bg-gray-700');
      expect(skeleton).toHaveClass('rounded');
    });

    test('applies custom className', () => {
      const { container } = render(<Skeleton className="w-20 h-10" />);

      const skeleton = container.firstChild;
      expect(skeleton).toHaveClass('w-20');
      expect(skeleton).toHaveClass('h-10');
    });

    test('combines default and custom classes', () => {
      const { container } = render(<Skeleton className="custom-class" />);

      const skeleton = container.firstChild;
      expect(skeleton).toHaveClass('animate-pulse');
      expect(skeleton).toHaveClass('custom-class');
    });

    test('renders as a div element', () => {
      const { container } = render(<Skeleton />);

      expect(container.firstChild?.nodeName).toBe('DIV');
    });
  });

  describe('customization', () => {
    test('can be customized with various dimensions', () => {
      const { container } = render(<Skeleton className="w-full h-64" />);

      const skeleton = container.firstChild;
      expect(skeleton).toHaveClass('w-full');
      expect(skeleton).toHaveClass('h-64');
    });

    test('can be customized with margin/padding', () => {
      const { container } = render(<Skeleton className="m-4 p-2" />);

      const skeleton = container.firstChild;
      expect(skeleton).toHaveClass('m-4');
      expect(skeleton).toHaveClass('p-2');
    });
  });
});
