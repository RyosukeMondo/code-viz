/**
 * Configuration settings utilities for 3D visualization
 * Handles persistence and default configuration
 * @module components/visualizations/3d/ui/configSettings
 */

import type { ComplexityThresholds } from '../types';
import { COMPLEXITY_THRESHOLDS, DEFAULT_VOXEL_SIZE, MAX_HEIGHT } from '../utils/constants';

/**
 * Configuration settings for 3D visualization
 */
export interface Config3DSettings {
  voxelSize: number;
  maxHeight: number;
  thresholds: ComplexityThresholds;
  antialias: boolean;
  shadowsEnabled: boolean;
}

/**
 * Default configuration settings
 */
export const DEFAULT_CONFIG: Config3DSettings = {
  voxelSize: DEFAULT_VOXEL_SIZE,
  maxHeight: MAX_HEIGHT,
  thresholds: { ...COMPLEXITY_THRESHOLDS },
  antialias: true,
  shadowsEnabled: false,
};

/**
 * Storage key for localStorage persistence
 */
const STORAGE_KEY = 'code3d_config_settings';

/**
 * Load settings from localStorage
 */
export function loadSettings(): Config3DSettings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return {
        ...DEFAULT_CONFIG,
        ...parsed,
        thresholds: {
          ...DEFAULT_CONFIG.thresholds,
          ...parsed.thresholds,
        },
      };
    }
  } catch (error) {
    console.warn('Failed to load 3D config settings:', error);
  }
  return { ...DEFAULT_CONFIG };
}

/**
 * Save settings to localStorage
 */
export function saveSettings(settings: Config3DSettings): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    console.warn('Failed to save 3D config settings:', error);
  }
}
