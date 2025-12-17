/**
 * Unit tests for Breadcrumb component
 *
 * Tests rendering, click handlers, keyboard navigation, and accessibility
 * for the breadcrumb navigation component.
 */

import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Breadcrumb } from './Breadcrumb';

describe('Breadcrumb', () => {
  describe('Rendering', () => {
    it('should render home button', () => {
      const onNavigate = vi.fn();
      render(<Breadcrumb path={[]} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      expect(homeButton).toBeInTheDocument();
      expect(homeButton).toHaveTextContent('Home');
    });

    it('should render "Root directory" text when path is empty', () => {
      const onNavigate = vi.fn();
      render(<Breadcrumb path={[]} onNavigate={onNavigate} />);

      expect(screen.getByText('Root directory')).toBeInTheDocument();
    });

    it('should render path segments correctly', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components', 'common'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      expect(screen.getByRole('button', { name: 'Navigate to src' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to components' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to common' })).toBeInTheDocument();
    });

    it('should render separators between segments', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      const { container } = render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      // Check for separator characters
      const separators = container.querySelectorAll('[aria-hidden="true"]');
      // Should have at least 2 separators for 2 path segments
      expect(separators.length).toBeGreaterThanOrEqual(2);
    });

    it('should highlight the last segment as current page', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components', 'common'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const lastSegment = screen.getByRole('button', { name: 'Navigate to common' });
      expect(lastSegment).toHaveAttribute('aria-current', 'page');
    });

    it('should not set aria-current on non-last segments', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const firstSegment = screen.getByRole('button', { name: 'Navigate to src' });
      expect(firstSegment).not.toHaveAttribute('aria-current');
    });
  });

  describe('Click interactions', () => {
    it('should call onNavigate with -1 when home button is clicked', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      render(<Breadcrumb path={['src', 'components']} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      await user.click(homeButton);

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(-1);
    });

    it('should call onNavigate with correct index when path segment is clicked', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components', 'common'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      // Click the second segment (index 1)
      const componentsButton = screen.getByRole('button', { name: 'Navigate to components' });
      await user.click(componentsButton);

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(1);
    });

    it('should call onNavigate with 0 for first path segment', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const srcButton = screen.getByRole('button', { name: 'Navigate to src' });
      await user.click(srcButton);

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(0);
    });

    it('should call onNavigate when clicking last segment', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const lastButton = screen.getByRole('button', { name: 'Navigate to components' });
      await user.click(lastButton);

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(1);
    });
  });

  describe('Keyboard navigation', () => {
    it('should call onNavigate when Enter is pressed on home button', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      render(<Breadcrumb path={['src']} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      homeButton.focus();
      await user.keyboard('{Enter}');

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(-1);
    });

    it('should call onNavigate when Space is pressed on home button', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      render(<Breadcrumb path={['src']} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      homeButton.focus();
      await user.keyboard(' ');

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(-1);
    });

    it('should call onNavigate when Enter is pressed on path segment', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const srcButton = screen.getByRole('button', { name: 'Navigate to src' });
      srcButton.focus();
      await user.keyboard('{Enter}');

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(0);
    });

    it('should call onNavigate when Space is pressed on path segment', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const componentsButton = screen.getByRole('button', { name: 'Navigate to components' });
      componentsButton.focus();
      await user.keyboard(' ');

      expect(onNavigate).toHaveBeenCalledTimes(1);
      expect(onNavigate).toHaveBeenCalledWith(1);
    });

    it('should support Tab navigation between buttons', async () => {
      const user = userEvent.setup();
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      const srcButton = screen.getByRole('button', { name: 'Navigate to src' });

      homeButton.focus();
      expect(document.activeElement).toBe(homeButton);

      await user.keyboard('{Tab}');
      expect(document.activeElement).toBe(srcButton);
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      expect(screen.getByRole('navigation', { name: 'Breadcrumb navigation' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to root' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to src' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to components' })).toBeInTheDocument();
    });

    it('should have visible focus indicators', () => {
      const onNavigate = vi.fn();
      const path = ['src'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const homeButton = screen.getByRole('button', { name: /navigate to root/i });
      // Check for focus ring classes
      expect(homeButton.className).toContain('focus:ring');
    });

    it('should mark last segment with aria-current', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'components', 'common'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const lastButton = screen.getByRole('button', { name: 'Navigate to common' });
      expect(lastButton).toHaveAttribute('aria-current', 'page');
    });
  });

  describe('Edge cases', () => {
    it('should handle single path segment', () => {
      const onNavigate = vi.fn();
      render(<Breadcrumb path={['src']} onNavigate={onNavigate} />);

      expect(screen.getByRole('button', { name: 'Navigate to src' })).toBeInTheDocument();
      expect(screen.queryByText('Root directory')).not.toBeInTheDocument();
    });

    it('should handle long path with many segments', () => {
      const onNavigate = vi.fn();
      const path = ['src', 'very', 'long', 'path', 'with', 'many', 'segments'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      path.forEach((segment) => {
        expect(screen.getByRole('button', { name: `Navigate to ${segment}` })).toBeInTheDocument();
      });
    });

    it('should handle path segments with special characters', () => {
      const onNavigate = vi.fn();
      const path = ['@types', 'node_modules', 'my-component'];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      expect(screen.getByRole('button', { name: 'Navigate to @types' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to node_modules' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Navigate to my-component' })).toBeInTheDocument();
    });

    it('should not crash with empty string segments', () => {
      const onNavigate = vi.fn();
      const path = ['', 'src', ''];

      expect(() => {
        render(<Breadcrumb path={path} onNavigate={onNavigate} />);
      }).not.toThrow();
    });
  });

  describe('Path formatting', () => {
    it('should truncate long segment names', () => {
      const onNavigate = vi.fn();
      const longName = 'a'.repeat(50);
      const path = [longName];
      render(<Breadcrumb path={path} onNavigate={onNavigate} />);

      const button = screen.getByRole('button', { name: `Navigate to ${longName}` });
      expect(button).toBeInTheDocument();
      // The formatPath utility should truncate it (maxLength: 30)
      expect(button.textContent?.length).toBeLessThanOrEqual(35); // Some buffer for ellipsis
    });
  });
});
