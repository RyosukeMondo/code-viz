/**
 * ConfigPanel - React component for 3D visualization configuration settings
 * Allows users to customize voxel size, height, color scheme, and other visual settings
 * Settings are persisted to localStorage for convenience
 * @module components/visualizations/3d/ui/ConfigPanel
 */

import React, { useState, useCallback, useEffect } from 'react';
import type { ComplexityThresholds } from '../types';
import { getColorLegend } from '../utils/colorMaps';
import type { Config3DSettings } from './configSettings';
import { DEFAULT_CONFIG, saveSettings } from './configSettings';

/**
 * Props for ConfigPanel component
 */
export interface ConfigPanelProps {
  /** Current configuration */
  settings: Config3DSettings;
  /** Callback when settings change */
  onSettingsChange: (settings: Config3DSettings) => void;
  /** Whether panel is visible */
  visible: boolean;
  /** Callback to toggle panel visibility */
  onToggle: () => void;
}

/**
 * Validate a numeric value within bounds
 */
function validateNumber(value: number, min: number, max: number): number {
  const num = Number(value);
  if (isNaN(num)) return min;
  return Math.max(min, Math.min(max, num));
}

/**
 * ConfigPanel component
 */
export const ConfigPanel: React.FC<ConfigPanelProps> = ({
  settings,
  onSettingsChange,
  visible,
  onToggle,
}) => {
  const [localSettings, setLocalSettings] = useState<Config3DSettings>(settings);

  // Update local settings when props change
  useEffect(() => {
    setLocalSettings(settings);
  }, [settings]);

  const handleVoxelSizeChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = validateNumber(parseFloat(e.target.value), 0.1, 10);
    setLocalSettings((prev) => ({ ...prev, voxelSize: value }));
  }, []);

  const handleMaxHeightChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = validateNumber(parseFloat(e.target.value), 10, 500);
    setLocalSettings((prev) => ({ ...prev, maxHeight: value }));
  }, []);

  const handleThresholdChange = useCallback(
    (key: keyof ComplexityThresholds, value: string) => {
      const num = validateNumber(parseFloat(value), 1, 200);
      setLocalSettings((prev) => ({
        ...prev,
        thresholds: { ...prev.thresholds, [key]: num },
      }));
    },
    []
  );

  const handleAntialiasChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setLocalSettings((prev) => ({ ...prev, antialias: e.target.checked }));
  }, []);

  const handleShadowsChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setLocalSettings((prev) => ({ ...prev, shadowsEnabled: e.target.checked }));
  }, []);

  const handleApply = useCallback(() => {
    onSettingsChange(localSettings);
    saveSettings(localSettings);
  }, [localSettings, onSettingsChange]);

  const handleReset = useCallback(() => {
    setLocalSettings({ ...DEFAULT_CONFIG });
    onSettingsChange({ ...DEFAULT_CONFIG });
    saveSettings({ ...DEFAULT_CONFIG });
  }, [onSettingsChange]);

  if (!visible) {
    return (
      <button
        onClick={onToggle}
        style={toggleButtonStyles}
        aria-label="Open settings panel"
        title="Settings"
      >
        ⚙️
      </button>
    );
  }

  const colorLegend = getColorLegend(localSettings.thresholds);

  return (
    <>
      {/* Toggle button when open */}
      <button
        onClick={onToggle}
        style={{ ...toggleButtonStyles, backgroundColor: 'rgba(99, 102, 241, 0.9)' }}
        aria-label="Close settings panel"
        title="Close settings"
      >
        ✕
      </button>

      {/* Settings panel */}
      <div style={panelStyles} role="dialog" aria-label="3D Visualization Settings">
        <div style={headerStyles}>
          <h3 style={titleStyles}>3D Visualization Settings</h3>
        </div>

        {/* Rendering Settings */}
        <div style={sectionStyles}>
          <h4 style={sectionTitleStyles}>Rendering</h4>

          <div style={fieldStyles}>
            <label style={labelStyles} htmlFor="voxelSize">
              Voxel Size
            </label>
            <input
              id="voxelSize"
              type="number"
              min="0.1"
              max="10"
              step="0.1"
              value={localSettings.voxelSize}
              onChange={handleVoxelSizeChange}
              style={inputStyles}
              aria-describedby="voxelSizeHelp"
            />
            <div style={helpTextStyles} id="voxelSizeHelp">
              Size of each voxel unit (0.1 - 10)
            </div>
          </div>

          <div style={fieldStyles}>
            <label style={labelStyles} htmlFor="maxHeight">
              Max Building Height
            </label>
            <input
              id="maxHeight"
              type="number"
              min="10"
              max="500"
              step="10"
              value={localSettings.maxHeight}
              onChange={handleMaxHeightChange}
              style={inputStyles}
              aria-describedby="maxHeightHelp"
            />
            <div style={helpTextStyles} id="maxHeightHelp">
              Maximum height in voxels (10 - 500)
            </div>
          </div>

          <div style={fieldStyles}>
            <label style={checkboxLabelStyles}>
              <input
                type="checkbox"
                checked={localSettings.antialias}
                onChange={handleAntialiasChange}
                style={checkboxStyles}
              />
              Antialiasing
            </label>
            <div style={helpTextStyles}>Smoother edges (may impact performance)</div>
          </div>

          <div style={fieldStyles}>
            <label style={checkboxLabelStyles}>
              <input
                type="checkbox"
                checked={localSettings.shadowsEnabled}
                onChange={handleShadowsChange}
                style={checkboxStyles}
              />
              Shadows
            </label>
            <div style={helpTextStyles}>Enable shadows (impacts performance)</div>
          </div>
        </div>

        {/* Complexity Thresholds */}
        <div style={sectionStyles}>
          <h4 style={sectionTitleStyles}>Complexity Thresholds</h4>

          <div style={fieldStyles}>
            <label style={labelStyles} htmlFor="thresholdLow">
              Low
            </label>
            <input
              id="thresholdLow"
              type="number"
              min="1"
              max="200"
              step="1"
              value={localSettings.thresholds.LOW}
              onChange={(e) => handleThresholdChange('LOW', e.target.value)}
              style={inputStyles}
            />
          </div>

          <div style={fieldStyles}>
            <label style={labelStyles} htmlFor="thresholdMedium">
              Medium
            </label>
            <input
              id="thresholdMedium"
              type="number"
              min="1"
              max="200"
              step="1"
              value={localSettings.thresholds.MEDIUM}
              onChange={(e) => handleThresholdChange('MEDIUM', e.target.value)}
              style={inputStyles}
            />
          </div>

          <div style={fieldStyles}>
            <label style={labelStyles} htmlFor="thresholdHigh">
              High
            </label>
            <input
              id="thresholdHigh"
              type="number"
              min="1"
              max="200"
              step="1"
              value={localSettings.thresholds.HIGH}
              onChange={(e) => handleThresholdChange('HIGH', e.target.value)}
              style={inputStyles}
            />
          </div>

          {localSettings.thresholds.VERY_HIGH !== undefined && (
            <div style={fieldStyles}>
              <label style={labelStyles} htmlFor="thresholdVeryHigh">
                Very High
              </label>
              <input
                id="thresholdVeryHigh"
                type="number"
                min="1"
                max="200"
                step="1"
                value={localSettings.thresholds.VERY_HIGH}
                onChange={(e) => handleThresholdChange('VERY_HIGH', e.target.value)}
                style={inputStyles}
              />
            </div>
          )}

          {/* Color Legend */}
          <div style={legendContainerStyles}>
            <div style={legendTitleStyles}>Color Scale</div>
            {colorLegend.map((entry) => (
              <div key={entry.label} style={legendItemStyles}>
                <div
                  style={{
                    ...legendColorBoxStyles,
                    backgroundColor: entry.color,
                  }}
                  aria-hidden="true"
                />
                <div style={legendTextStyles}>
                  {entry.label}: {entry.range}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Action Buttons */}
        <div style={buttonGroupStyles}>
          <button onClick={handleApply} style={primaryButtonStyles} aria-label="Apply settings">
            Apply
          </button>
          <button onClick={handleReset} style={secondaryButtonStyles} aria-label="Reset to defaults">
            Reset to Defaults
          </button>
        </div>

        {/* Footer hint */}
        <div style={footerStyles}>
          Press ESC to close
        </div>
      </div>
    </>
  );
};

// Styles
const toggleButtonStyles: React.CSSProperties = {
  position: 'fixed',
  top: '20px',
  right: '20px',
  width: '44px',
  height: '44px',
  borderRadius: '50%',
  backgroundColor: 'rgba(0, 0, 0, 0.7)',
  color: '#ffffff',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  cursor: 'pointer',
  fontSize: '18px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.3)',
  zIndex: 10001,
  transition: 'background-color 0.2s',
};

const panelStyles: React.CSSProperties = {
  position: 'fixed',
  top: '80px',
  right: '20px',
  backgroundColor: 'rgba(0, 0, 0, 0.9)',
  color: '#ffffff',
  padding: '20px',
  borderRadius: '8px',
  fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif',
  fontSize: '14px',
  lineHeight: '1.5',
  width: '320px',
  maxHeight: 'calc(100vh - 120px)',
  overflowY: 'auto',
  boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
  border: '1px solid rgba(255, 255, 255, 0.1)',
  backdropFilter: 'blur(10px)',
  zIndex: 10000,
};

const headerStyles: React.CSSProperties = {
  marginBottom: '16px',
  paddingBottom: '12px',
  borderBottom: '1px solid rgba(255, 255, 255, 0.2)',
};

const titleStyles: React.CSSProperties = {
  margin: 0,
  fontSize: '16px',
  fontWeight: 600,
};

const sectionStyles: React.CSSProperties = {
  marginBottom: '20px',
};

const sectionTitleStyles: React.CSSProperties = {
  margin: '0 0 12px 0',
  fontSize: '13px',
  fontWeight: 600,
  color: 'rgba(255, 255, 255, 0.7)',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
};

const fieldStyles: React.CSSProperties = {
  marginBottom: '12px',
};

const labelStyles: React.CSSProperties = {
  display: 'block',
  marginBottom: '4px',
  fontSize: '12px',
  color: 'rgba(255, 255, 255, 0.8)',
};

const checkboxLabelStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  fontSize: '12px',
  color: 'rgba(255, 255, 255, 0.8)',
  cursor: 'pointer',
};

const inputStyles: React.CSSProperties = {
  width: '100%',
  padding: '8px',
  backgroundColor: 'rgba(255, 255, 255, 0.1)',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  borderRadius: '4px',
  color: '#ffffff',
  fontSize: '13px',
  boxSizing: 'border-box',
};

const checkboxStyles: React.CSSProperties = {
  marginRight: '8px',
  cursor: 'pointer',
};

const helpTextStyles: React.CSSProperties = {
  marginTop: '4px',
  fontSize: '11px',
  color: 'rgba(255, 255, 255, 0.5)',
};

const legendContainerStyles: React.CSSProperties = {
  marginTop: '12px',
  padding: '12px',
  backgroundColor: 'rgba(255, 255, 255, 0.05)',
  borderRadius: '4px',
};

const legendTitleStyles: React.CSSProperties = {
  fontSize: '11px',
  color: 'rgba(255, 255, 255, 0.6)',
  marginBottom: '8px',
  textTransform: 'uppercase',
  letterSpacing: '0.5px',
};

const legendItemStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  marginBottom: '6px',
};

const legendColorBoxStyles: React.CSSProperties = {
  width: '16px',
  height: '16px',
  borderRadius: '2px',
  marginRight: '8px',
  border: '1px solid rgba(255, 255, 255, 0.2)',
};

const legendTextStyles: React.CSSProperties = {
  fontSize: '11px',
  color: 'rgba(255, 255, 255, 0.7)',
};

const buttonGroupStyles: React.CSSProperties = {
  display: 'flex',
  gap: '8px',
  marginTop: '16px',
};

const primaryButtonStyles: React.CSSProperties = {
  flex: 1,
  padding: '10px 16px',
  backgroundColor: '#6366f1',
  color: '#ffffff',
  border: 'none',
  borderRadius: '4px',
  fontSize: '13px',
  fontWeight: 600,
  cursor: 'pointer',
  transition: 'background-color 0.2s',
};

const secondaryButtonStyles: React.CSSProperties = {
  flex: 1,
  padding: '10px 16px',
  backgroundColor: 'rgba(255, 255, 255, 0.1)',
  color: '#ffffff',
  border: '1px solid rgba(255, 255, 255, 0.2)',
  borderRadius: '4px',
  fontSize: '13px',
  fontWeight: 600,
  cursor: 'pointer',
  transition: 'background-color 0.2s',
};

const footerStyles: React.CSSProperties = {
  marginTop: '16px',
  paddingTop: '12px',
  borderTop: '1px solid rgba(255, 255, 255, 0.1)',
  fontSize: '11px',
  color: 'rgba(255, 255, 255, 0.4)',
  textAlign: 'center',
};

export default ConfigPanel;
