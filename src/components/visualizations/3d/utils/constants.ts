/**
 * Shared constants for 3D visualization
 * @module components/visualizations/3d/utils/constants
 */

/**
 * Metric thresholds for complexity visualization
 * Used to determine color mapping and visual effects
 */
export const COMPLEXITY_THRESHOLDS = {
  LOW: 10,      // 0-10: Low complexity (green)
  MEDIUM: 20,   // 10-20: Medium complexity (yellow)
  HIGH: 30,     // 20-30: High complexity (orange)
  VERY_HIGH: 40 // 30+: Very high complexity (red)
} as const;

/**
 * Color palette for complexity visualization
 * Colors represent different levels of code complexity
 * Updated for high contrast and visibility against light background
 */
export const COMPLEXITY_COLORS = {
  LOW: '#00cc00',       // Bright green - simple, maintainable code
  MEDIUM: '#ffcc00',    // Bright yellow-orange - moderate complexity
  HIGH: '#ff6600',      // Vibrant orange - high complexity, needs attention
  VERY_HIGH: '#ff0000'  // Pure red - very high complexity, refactor recommended
} as const;

/**
 * Maximum building height in voxel units
 * Prevents excessively tall buildings from dominating the view
 */
export const MAX_HEIGHT = 100;

/**
 * Minimum building height in voxel units
 * Ensures even empty files are visible
 */
export const MIN_HEIGHT = 1;

/**
 * Default voxel size in world units
 */
export const DEFAULT_VOXEL_SIZE = 1;

/**
 * Default height scale multiplier
 * Applied to log10(LOC) to calculate building height
 */
export const DEFAULT_HEIGHT_SCALE = 10;

/**
 * Default world dimensions
 */
export const DEFAULT_WORLD_WIDTH = 1000;
export const DEFAULT_WORLD_DEPTH = 1000;

/**
 * Rendering performance thresholds
 */
export const PERFORMANCE_THRESHOLDS = {
  TARGET_FPS: 60,
  MIN_FPS: 30,
  MAX_VOXELS_PER_INSTANCE: 100000,  // GPU instance limit safety margin
  LOD_DISTANCE: 200  // Distance at which to disable expensive effects
} as const;

/**
 * UI constants
 */
export const UI_CONSTANTS = {
  INFO_PANEL_MAX_PATH_LENGTH: 60,  // Truncate paths longer than this
  STATS_UPDATE_INTERVAL: 100,       // Update stats every 100ms
  CAMERA_SAVE_DEBOUNCE: 1000,       // Save camera position max once per second
  CAMERA_STORAGE_KEY: 'code3d_camera_state',
  CAMERA_STORAGE_TTL: 604800000     // 7 days in milliseconds
} as const;

/**
 * Error messages
 */
export const ERROR_MESSAGES = {
  WEBGL_NOT_SUPPORTED: 'Your browser doesn\'t support WebGL. Please use Chrome 90+, Firefox 88+, or Safari 14+.',
  SHADER_COMPILATION_FAILED: 'Warning: Shader compilation failed. Falling back to standard materials.'
} as const;

export type ComplexityLevel = 'LOW' | 'MEDIUM' | 'HIGH' | 'VERY_HIGH';
