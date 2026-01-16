/**
 * Tests for ProgressBar component
 */

import { render, screen } from '@testing-library/react';
import { describe, test, expect } from 'vitest';
import { ProgressBar } from './ProgressBar';

describe('ProgressBar', () => {
  describe('rendering', () => {
    test('renders with required props', () => {
      const { container } = render(<ProgressBar progress={50} />);

      expect(container.querySelector('.w-full')).toBeInTheDocument();
    });

    test('renders progress message', () => {
      render(<ProgressBar progress={50} message="Loading data..." />);

      expect(screen.getByText('Loading data...')).toBeInTheDocument();
    });

    test('renders default message when no message provided', () => {
      render(<ProgressBar progress={50} />);

      expect(screen.getByText('Processing...')).toBeInTheDocument();
    });

    test('displays progress percentage', () => {
      render(<ProgressBar progress={75} />);

      expect(screen.getByText('75%')).toBeInTheDocument();
    });

    test('rounds progress percentage', () => {
      render(<ProgressBar progress={75.7} />);

      expect(screen.getByText('76%')).toBeInTheDocument();
    });

    test('renders progress bar with correct width', () => {
      const { container } = render(<ProgressBar progress={60} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveStyle({ width: '60%' });
    });
  });

  describe('progress clamping', () => {
    test('clamps progress above 100 to 100%', () => {
      const { container } = render(<ProgressBar progress={150} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveStyle({ width: '100%' });
      expect(screen.getByText('100%')).toBeInTheDocument();
    });

    test('clamps progress below 0 to 0%', () => {
      const { container } = render(<ProgressBar progress={-20} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveStyle({ width: '0%' });
      expect(screen.getByText('0%')).toBeInTheDocument();
    });

    test('handles 0% progress', () => {
      const { container } = render(<ProgressBar progress={0} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveStyle({ width: '0%' });
      expect(screen.getByText('0%')).toBeInTheDocument();
    });

    test('handles 100% progress', () => {
      const { container } = render(<ProgressBar progress={100} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveStyle({ width: '100%' });
      expect(screen.getByText('100%')).toBeInTheDocument();
    });
  });

  describe('indeterminate mode', () => {
    test('renders in indeterminate mode when enabled', () => {
      const { container } = render(
        <ProgressBar progress={50} indeterminate={true} />
      );

      const progressFill = container.querySelector('.animate-pulse');
      expect(progressFill).toBeInTheDocument();
      expect(progressFill).toHaveStyle({ width: '100%' });
    });

    test('hides percentage in indeterminate mode', () => {
      render(<ProgressBar progress={50} indeterminate={true} />);

      expect(screen.queryByText('50%')).not.toBeInTheDocument();
    });

    test('shows message in indeterminate mode', () => {
      render(
        <ProgressBar
          progress={50}
          indeterminate={true}
          message="Loading..."
        />
      );

      expect(screen.getByText('Loading...')).toBeInTheDocument();
    });

    test('applies pulse animation in indeterminate mode', () => {
      const { container } = render(
        <ProgressBar progress={50} indeterminate={true} />
      );

      const progressFill = container.querySelector('.bg-blue-500');
      expect(progressFill).toHaveClass('animate-pulse');
    });
  });

  describe('determinate mode', () => {
    test('shows percentage when not indeterminate', () => {
      render(<ProgressBar progress={45} indeterminate={false} />);

      expect(screen.getByText('45%')).toBeInTheDocument();
    });

    test('uses transition for smooth progress changes', () => {
      const { container } = render(<ProgressBar progress={30} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).toHaveClass('transition-all');
      expect(progressFill).toHaveClass('duration-300');
    });

    test('does not apply pulse animation in determinate mode', () => {
      const { container } = render(<ProgressBar progress={50} />);

      const progressFill = container.querySelector('.bg-blue-600');
      expect(progressFill).not.toHaveClass('animate-pulse');
    });
  });

  describe('styling', () => {
    test('has full width container', () => {
      const { container } = render(<ProgressBar progress={50} />);

      const outerContainer = container.querySelector('.w-full.space-y-2');
      expect(outerContainer).toBeInTheDocument();
    });

    test('includes dark mode classes for background', () => {
      const { container } = render(<ProgressBar progress={50} />);

      const progressTrack = container.querySelector('.dark\\:bg-gray-700');
      expect(progressTrack).toBeInTheDocument();
    });

    test('includes dark mode classes for text', () => {
      const { container } = render(<ProgressBar progress={50} />);

      const darkText = container.querySelector('.dark\\:text-gray-400');
      expect(darkText).toBeInTheDocument();
    });

    test('uses rounded-full for progress bar', () => {
      const { container } = render(<ProgressBar progress={50} />);

      const progressTrack = container.querySelector('.rounded-full');
      expect(progressTrack).toBeInTheDocument();
    });
  });

  describe('edge cases', () => {
    test('handles decimal progress values', () => {
      render(<ProgressBar progress={33.33} />);

      expect(screen.getByText('33%')).toBeInTheDocument();
    });

    test('handles very small progress values', () => {
      render(<ProgressBar progress={0.1} />);

      expect(screen.getByText('0%')).toBeInTheDocument();
    });

    test('handles very large progress values', () => {
      render(<ProgressBar progress={999} />);

      expect(screen.getByText('100%')).toBeInTheDocument();
    });

    test('handles empty message string', () => {
      render(<ProgressBar progress={50} message="" />);

      expect(screen.getByText('Processing...')).toBeInTheDocument();
    });

    test('handles long message strings', () => {
      const longMessage =
        'This is a very long message that might wrap to multiple lines';
      render(<ProgressBar progress={50} message={longMessage} />);

      expect(screen.getByText(longMessage)).toBeInTheDocument();
    });
  });
});
