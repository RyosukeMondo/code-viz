/**
 * Tests for AnalysisSettings component
 */

import { render, screen, fireEvent } from '@testing-library/react';
import { describe, test, expect, vi } from 'vitest';
import { AnalysisSettings } from './AnalysisSettings';
import type { AnalysisOptions } from '@/types';

describe('AnalysisSettings', () => {
  const defaultOptions: AnalysisOptions = {
    enableDuplicates: false,
    minDuplicateLines: 5,
    enableHotspots: false,
    maxHotspots: 10,
    enableAiCommits: false,
    coverageReportPath: undefined,
  };

  const mockOnChange = vi.fn();
  const mockOnToggleDeadCode = vi.fn();

  describe('rendering', () => {
    test('renders toggle button', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      expect(screen.getByText(/Analysis Features/)).toBeInTheDocument();
    });

    test('is collapsed by default', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      expect(screen.queryByText('All Analysis Features')).not.toBeInTheDocument();
    });

    test('shows correct enabled count with defaults', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          deadCodeEnabled={false}
        />
      );

      // 5 always-on features + 0 optional = 5 total
      expect(screen.getByText(/5\/10 enabled/)).toBeInTheDocument();
    });

    test('shows correct enabled count with some options enabled', () => {
      const optionsWithSome = {
        ...defaultOptions,
        enableAiCommits: true,
        enableHotspots: true,
      };

      render(
        <AnalysisSettings
          options={optionsWithSome}
          onChange={mockOnChange}
          deadCodeEnabled={true}
        />
      );

      // 5 always-on + 2 optional + 1 dead code = 8 total
      expect(screen.getByText(/8\/10 enabled/)).toBeInTheDocument();
    });

    test('disables toggle button when disabled prop is true', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          disabled={true}
        />
      );

      const toggleButton = screen.getByRole('button');
      expect(toggleButton).toBeDisabled();
    });
  });

  describe('expand/collapse', () => {
    test('expands panel when button is clicked', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      const toggleButton = screen.getByRole('button');
      fireEvent.click(toggleButton);

      expect(screen.getByText('All Analysis Features')).toBeInTheDocument();
    });

    test('collapses panel when button is clicked again', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      const toggleButton = screen.getByRole('button');

      // Expand
      fireEvent.click(toggleButton);
      expect(screen.getByText('All Analysis Features')).toBeInTheDocument();

      // Collapse
      fireEvent.click(toggleButton);
      expect(screen.queryByText('All Analysis Features')).not.toBeInTheDocument();
    });

    test('rotates arrow icon when expanded', () => {
      const { container } = render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      const toggleButton = screen.getByRole('button');
      const arrow = container.querySelector('svg');

      // Initially not rotated
      expect(arrow).not.toHaveClass('rotate-90');

      // Expand
      fireEvent.click(toggleButton);
      expect(arrow).toHaveClass('rotate-90');
    });

    test('sets aria-expanded attribute correctly', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      const toggleButton = screen.getByRole('button');

      expect(toggleButton).toHaveAttribute('aria-expanded', 'false');

      fireEvent.click(toggleButton);
      expect(toggleButton).toHaveAttribute('aria-expanded', 'true');
    });
  });

  describe('always-on features display', () => {
    test('displays all 5 always-on features', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('Basic Metrics')).toBeInTheDocument();
      expect(screen.getByText('Coupling Metrics')).toBeInTheDocument();
      expect(screen.getByText('Code Churn')).toBeInTheDocument();
      expect(screen.getByText('AI Bloat Index')).toBeInTheDocument();
      expect(screen.getByText('Cognitive Complexity')).toBeInTheDocument();
    });

    test('always-on features have "Always On" badge', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const alwaysOnBadges = screen.getAllByText('Always On');
      expect(alwaysOnBadges.length).toBe(5);
    });

    test('shows section header for always-on features', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText(/Always Enabled \(5 features\)/)).toBeInTheDocument();
    });
  });

  describe('configurable features', () => {
    test('displays AI Commit Analysis toggle', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('AI Commit Analysis')).toBeInTheDocument();
    });

    test('AI Commits checkbox reflects state', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableAiCommits: true }}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const checkbox = screen.getByLabelText(/AI Commit Analysis/);
      expect(checkbox).toBeChecked();
    });

    test('toggles AI Commits when checkbox clicked', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const checkbox = screen.getByLabelText(/AI Commit Analysis/);
      fireEvent.click(checkbox);

      expect(mockOnChange).toHaveBeenCalledWith({
        ...defaultOptions,
        enableAiCommits: true,
      });
    });

    test('displays Dead Code Detection toggle', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          onToggleDeadCode={mockOnToggleDeadCode}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('Dead Code Detection')).toBeInTheDocument();
    });

    test('Dead Code checkbox reflects deadCodeEnabled prop', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          deadCodeEnabled={true}
          onToggleDeadCode={mockOnToggleDeadCode}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const checkbox = screen.getByLabelText(/Dead Code Detection/);
      expect(checkbox).toBeChecked();
    });

    test('calls onToggleDeadCode when Dead Code checkbox clicked', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          onToggleDeadCode={mockOnToggleDeadCode}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const checkbox = screen.getByLabelText(/Dead Code Detection/);
      fireEvent.click(checkbox);

      expect(mockOnToggleDeadCode).toHaveBeenCalled();
    });

    test('displays Git Hotspot Detection toggle', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('Git Hotspot Detection')).toBeInTheDocument();
    });

    test('toggles hotspots when checkbox clicked', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const checkbox = screen.getByLabelText(/Git Hotspot Detection/);
      fireEvent.click(checkbox);

      expect(mockOnChange).toHaveBeenCalledWith({
        ...defaultOptions,
        enableHotspots: true,
      });
    });

    test('shows maxHotspots slider when hotspots enabled', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableHotspots: true, maxHotspots: 10 }}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText(/Max hotspots to report: 10/)).toBeInTheDocument();
      expect(screen.getByRole('slider')).toBeInTheDocument();
    });

    test('hides maxHotspots slider when hotspots disabled', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableHotspots: false }}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.queryByRole('slider')).not.toBeInTheDocument();
    });

    test('updates maxHotspots when slider changed', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableHotspots: true, maxHotspots: 10 }}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const slider = screen.getByRole('slider');
      fireEvent.change(slider, { target: { value: '25' } });

      expect(mockOnChange).toHaveBeenCalledWith({
        ...defaultOptions,
        enableHotspots: true,
        maxHotspots: 25,
      });
    });

    test('displays Test Coverage Integration input', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('Test Coverage Integration')).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/Path to coverage.json/)).toBeInTheDocument();
    });

    test('updates coverageReportPath when input changed', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const input = screen.getByPlaceholderText(/Path to coverage.json/);
      fireEvent.change(input, { target: { value: '/path/to/coverage.json' } });

      expect(mockOnChange).toHaveBeenCalledWith({
        ...defaultOptions,
        coverageReportPath: '/path/to/coverage.json',
      });
    });

    test('clears coverageReportPath when input is emptied', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, coverageReportPath: '/some/path' }}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const input = screen.getByPlaceholderText(/Path to coverage.json/);
      fireEvent.change(input, { target: { value: '' } });

      expect(mockOnChange).toHaveBeenCalledWith({
        ...defaultOptions,
        coverageReportPath: undefined,
      });
    });

    test('shows Code Duplication as disabled with "in dev" badge', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText('Code Duplication Detection')).toBeInTheDocument();
      expect(screen.getByText('#in dev')).toBeInTheDocument();

      const duplicatesCheckbox = screen.getByRole('checkbox', { name: /Code Duplication Detection/ });
      expect(duplicatesCheckbox).toBeDisabled();
    });
  });

  describe('disabled state', () => {
    test('disables all checkboxes when disabled prop is true', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableAiCommits: true }}
          onChange={mockOnChange}
          onToggleDeadCode={mockOnToggleDeadCode}
          disabled={true}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const aiCommitsCheckbox = screen.getByLabelText(/AI Commit Analysis/);
      expect(aiCommitsCheckbox).toBeDisabled();

      const deadCodeCheckbox = screen.getByLabelText(/Dead Code Detection/);
      expect(deadCodeCheckbox).toBeDisabled();
    });

    test('disables coverage input when disabled prop is true', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
          disabled={true}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const input = screen.getByPlaceholderText(/Path to coverage.json/);
      expect(input).toBeDisabled();
    });

    test('disables hotspots slider when disabled prop is true', () => {
      render(
        <AnalysisSettings
          options={{ ...defaultOptions, enableHotspots: true }}
          onChange={mockOnChange}
          disabled={true}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      const slider = screen.getByRole('slider');
      expect(slider).toBeDisabled();
    });
  });

  describe('summary section', () => {
    test('displays summary information panel', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText(/How Features Appear in Visualization:/)).toBeInTheDocument();
    });

    test('explains how features appear in visualization', () => {
      render(
        <AnalysisSettings
          options={defaultOptions}
          onChange={mockOnChange}
        />
      );

      fireEvent.click(screen.getByRole('button'));

      expect(screen.getByText(/Always-on metrics.*affect file size and color/)).toBeInTheDocument();
      expect(screen.getByText(/Dead Code.*overlays purple tint/)).toBeInTheDocument();
    });
  });
});
