/**
 * Tests for ConfigPanel component
 * @module components/visualizations/3d/ui/ConfigPanel.test
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ConfigPanel } from './ConfigPanel';
import { loadSettings, saveSettings, DEFAULT_CONFIG, type Config3DSettings } from './configSettings';

describe('ConfigPanel', () => {
  const mockOnSettingsChange = vi.fn();
  const mockOnToggle = vi.fn();

  const defaultProps = {
    settings: DEFAULT_CONFIG,
    onSettingsChange: mockOnSettingsChange,
    visible: true,
    onToggle: mockOnToggle,
  };

  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  describe('Rendering', () => {
    it('should render toggle button when not visible', () => {
      render(<ConfigPanel {...defaultProps} visible={false} />);
      const toggleButton = screen.getByLabelText('Open settings panel');
      expect(toggleButton).toBeDefined();
      expect(toggleButton.textContent).toBe('⚙️');
    });

    it('should render full panel when visible', () => {
      render(<ConfigPanel {...defaultProps} visible={true} />);
      expect(screen.getByText('3D Visualization Settings')).toBeDefined();
      expect(screen.getByLabelText('Voxel Size')).toBeDefined();
      expect(screen.getByLabelText('Max Building Height')).toBeDefined();
    });

    it('should render all form fields', () => {
      render(<ConfigPanel {...defaultProps} />);

      expect(screen.getByLabelText('Voxel Size')).toBeDefined();
      expect(screen.getByLabelText('Max Building Height')).toBeDefined();
      expect(screen.getByText('Antialiasing')).toBeDefined();
      expect(screen.getByText('Shadows')).toBeDefined();
      expect(screen.getByLabelText('Low')).toBeDefined();
      expect(screen.getByLabelText('Medium')).toBeDefined();
      expect(screen.getByLabelText('High')).toBeDefined();
    });

    it('should render action buttons', () => {
      render(<ConfigPanel {...defaultProps} />);
      expect(screen.getByLabelText('Apply settings')).toBeDefined();
      expect(screen.getByLabelText('Reset to defaults')).toBeDefined();
    });
  });

  describe('Form Interactions', () => {
    it('should update voxel size on input change', () => {
      render(<ConfigPanel {...defaultProps} />);
      const input = screen.getByLabelText('Voxel Size') as HTMLInputElement;

      fireEvent.change(input, { target: { value: '2.5' } });

      expect(input.value).toBe('2.5');
    });

    it('should update max height on input change', () => {
      render(<ConfigPanel {...defaultProps} />);
      const input = screen.getByLabelText('Max Building Height') as HTMLInputElement;

      fireEvent.change(input, { target: { value: '150' } });

      expect(input.value).toBe('150');
    });

    it('should toggle antialias checkbox', () => {
      render(<ConfigPanel {...defaultProps} />);
      const checkbox = screen.getByRole('checkbox', { name: /antialiasing/i }) as HTMLInputElement;

      expect(checkbox.checked).toBe(true);

      fireEvent.click(checkbox);

      expect(checkbox.checked).toBe(false);
    });

    it('should toggle shadows checkbox', () => {
      render(<ConfigPanel {...defaultProps} />);
      const checkbox = screen.getByRole('checkbox', { name: /shadows/i }) as HTMLInputElement;

      expect(checkbox.checked).toBe(false);

      fireEvent.click(checkbox);

      expect(checkbox.checked).toBe(true);
    });

    it('should update complexity thresholds', () => {
      render(<ConfigPanel {...defaultProps} />);
      const lowInput = screen.getByLabelText('Low') as HTMLInputElement;

      fireEvent.change(lowInput, { target: { value: '15' } });

      expect(lowInput.value).toBe('15');
    });
  });

  describe('Validation', () => {
    it('should validate voxel size within bounds', () => {
      render(<ConfigPanel {...defaultProps} />);
      const input = screen.getByLabelText('Voxel Size') as HTMLInputElement;

      // Test upper bound
      fireEvent.change(input, { target: { value: '100' } });
      expect(parseFloat(input.value)).toBeLessThanOrEqual(10);

      // Test lower bound
      fireEvent.change(input, { target: { value: '-5' } });
      expect(parseFloat(input.value)).toBeGreaterThanOrEqual(0.1);
    });

    it('should validate max height within bounds', () => {
      render(<ConfigPanel {...defaultProps} />);
      const input = screen.getByLabelText('Max Building Height') as HTMLInputElement;

      // Test upper bound
      fireEvent.change(input, { target: { value: '1000' } });
      expect(parseFloat(input.value)).toBeLessThanOrEqual(500);

      // Test lower bound
      fireEvent.change(input, { target: { value: '5' } });
      expect(parseFloat(input.value)).toBeGreaterThanOrEqual(10);
    });

    it('should validate complexity thresholds', () => {
      render(<ConfigPanel {...defaultProps} />);
      const input = screen.getByLabelText('Low') as HTMLInputElement;

      // Test upper bound
      fireEvent.change(input, { target: { value: '500' } });
      expect(parseFloat(input.value)).toBeLessThanOrEqual(200);

      // Test lower bound
      fireEvent.change(input, { target: { value: '-10' } });
      expect(parseFloat(input.value)).toBeGreaterThanOrEqual(1);
    });
  });

  describe('Actions', () => {
    it('should call onSettingsChange when Apply is clicked', () => {
      render(<ConfigPanel {...defaultProps} />);

      // Change a setting
      const input = screen.getByLabelText('Voxel Size') as HTMLInputElement;
      fireEvent.change(input, { target: { value: '2.0' } });

      // Click Apply
      const applyButton = screen.getByLabelText('Apply settings');
      fireEvent.click(applyButton);

      expect(mockOnSettingsChange).toHaveBeenCalledTimes(1);
      expect(mockOnSettingsChange).toHaveBeenCalledWith(
        expect.objectContaining({
          voxelSize: 2.0,
        })
      );
    });

    it('should reset to defaults when Reset is clicked', () => {
      render(<ConfigPanel {...defaultProps} />);

      // Change a setting
      const input = screen.getByLabelText('Voxel Size') as HTMLInputElement;
      fireEvent.change(input, { target: { value: '5.0' } });

      // Click Reset
      const resetButton = screen.getByLabelText('Reset to defaults');
      fireEvent.click(resetButton);

      expect(mockOnSettingsChange).toHaveBeenCalledWith(DEFAULT_CONFIG);
    });

    it('should call onToggle when toggle button is clicked', () => {
      render(<ConfigPanel {...defaultProps} visible={false} />);

      const toggleButton = screen.getByLabelText('Open settings panel');
      fireEvent.click(toggleButton);

      expect(mockOnToggle).toHaveBeenCalledTimes(1);
    });
  });

  describe('LocalStorage', () => {
    it('should save settings to localStorage on Apply', () => {
      render(<ConfigPanel {...defaultProps} />);

      // Change settings
      const voxelInput = screen.getByLabelText('Voxel Size') as HTMLInputElement;
      fireEvent.change(voxelInput, { target: { value: '3.0' } });

      // Apply
      const applyButton = screen.getByLabelText('Apply settings');
      fireEvent.click(applyButton);

      // Check localStorage
      const stored = localStorage.getItem('code3d_config_settings');
      expect(stored).toBeTruthy();

      const parsed = JSON.parse(stored!);
      expect(parsed.voxelSize).toBe(3.0);
    });

    it('should load settings from localStorage', () => {
      // Save custom settings
      const customSettings: Config3DSettings = {
        ...DEFAULT_CONFIG,
        voxelSize: 4.5,
        maxHeight: 200,
      };
      saveSettings(customSettings);

      // Load settings
      const loaded = loadSettings();

      expect(loaded.voxelSize).toBe(4.5);
      expect(loaded.maxHeight).toBe(200);
    });

    it('should return defaults if localStorage is empty', () => {
      const loaded = loadSettings();

      expect(loaded).toEqual(DEFAULT_CONFIG);
    });

    it('should handle corrupted localStorage data gracefully', () => {
      localStorage.setItem('code3d_config_settings', 'invalid json');

      const loaded = loadSettings();

      expect(loaded).toEqual(DEFAULT_CONFIG);
    });
  });

  describe('Color Legend', () => {
    it('should render color legend', () => {
      render(<ConfigPanel {...defaultProps} />);

      expect(screen.getByText('Color Scale')).toBeDefined();
      expect(screen.getByText(/^Low:/)).toBeDefined();
      expect(screen.getByText(/^Medium:/)).toBeDefined();
      expect(screen.getByText(/^High: 20-30$/)).toBeDefined();
      expect(screen.getByText(/^Very High:/)).toBeDefined();
    });

    it('should update color legend when thresholds change', () => {
      render(<ConfigPanel {...defaultProps} />);

      // Change LOW threshold
      const lowInput = screen.getByLabelText('Low') as HTMLInputElement;
      fireEvent.change(lowInput, { target: { value: '15' } });

      // Legend should update to show new range
      const legendText = screen.getByText(/Low:/).textContent;
      expect(legendText).toContain('0-15');
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels', () => {
      render(<ConfigPanel {...defaultProps} />);

      expect(screen.getByLabelText('Voxel Size')).toBeDefined();
      expect(screen.getByLabelText('Max Building Height')).toBeDefined();
      expect(screen.getByLabelText('Apply settings')).toBeDefined();
      expect(screen.getByLabelText('Reset to defaults')).toBeDefined();
    });

    it('should have proper dialog role', () => {
      render(<ConfigPanel {...defaultProps} />);

      const dialog = screen.getByRole('dialog');
      expect(dialog).toBeDefined();
      expect(dialog.getAttribute('aria-label')).toBe('3D Visualization Settings');
    });

    it('should have help text for inputs', () => {
      render(<ConfigPanel {...defaultProps} />);

      expect(screen.getByText('Size of each voxel unit (0.1 - 10)')).toBeDefined();
      expect(screen.getByText('Maximum height in voxels (10 - 500)')).toBeDefined();
    });
  });

  describe('Keyboard Shortcuts', () => {
    it('should show ESC hint in footer', () => {
      render(<ConfigPanel {...defaultProps} />);

      expect(screen.getByText('Press ESC to close')).toBeDefined();
    });
  });
});
