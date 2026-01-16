/**
 * Color mapping utilities for code complexity visualization
 * @module components/visualizations/3d/utils/colorMaps
 */

import * as THREE from 'three';
import { COMPLEXITY_THRESHOLDS, COMPLEXITY_COLORS } from './constants';
import type { ComplexityThresholds, ColorLegendEntry } from '../types';

/**
 * Linear interpolation helper function
 */
function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

/**
 * Clamp value between min and max
 */
function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

/**
 * Convert hex color string to RGB components
 */
function hexToRgb(hex: string): { r: number; g: number; b: number } {
  const color = new THREE.Color(hex);
  return { r: color.r, g: color.g, b: color.b };
}

/**
 * Interpolate between two colors
 */
function interpolateColor(colorA: string, colorB: string, t: number): THREE.Color {
  const rgbA = hexToRgb(colorA);
  const rgbB = hexToRgb(colorB);

  return new THREE.Color(
    lerp(rgbA.r, rgbB.r, t),
    lerp(rgbA.g, rgbB.g, t),
    lerp(rgbA.b, rgbB.b, t)
  );
}

/**
 * Maps cyclomatic complexity to a color gradient
 *
 * Color scale:
 * - 0-10: Green (low complexity, maintainable)
 * - 10-20: Green to Yellow (medium complexity)
 * - 20-30: Yellow to Orange (high complexity, needs attention)
 * - 30+: Orange to Red (very high complexity, refactor recommended)
 *
 * @param complexity - Cyclomatic complexity score
 * @param thresholds - Optional custom thresholds
 * @returns Color representing the complexity level
 *
 * @example
 * // Get color for low complexity code
 * const greenColor = complexityToColor(5); // Returns green
 *
 * @example
 * // Get color for high complexity code
 * const redColor = complexityToColor(35); // Returns red
 */
export function complexityToColor(
  complexity: number,
  thresholds: ComplexityThresholds = COMPLEXITY_THRESHOLDS
): THREE.Color {
  const low = thresholds.LOW ?? COMPLEXITY_THRESHOLDS.LOW;
  const medium = thresholds.MEDIUM ?? COMPLEXITY_THRESHOLDS.MEDIUM;
  const high = thresholds.HIGH ?? COMPLEXITY_THRESHOLDS.HIGH;
  const veryHigh = thresholds.VERY_HIGH ?? COMPLEXITY_THRESHOLDS.VERY_HIGH;

  const clampedComplexity = clamp(complexity, 0, veryHigh * 2);

  if (clampedComplexity < low) {
    return new THREE.Color(COMPLEXITY_COLORS.LOW);
  } else if (clampedComplexity < medium) {
    const t = (clampedComplexity - low) / (medium - low);
    return interpolateColor(COMPLEXITY_COLORS.LOW, COMPLEXITY_COLORS.MEDIUM, t);
  } else if (clampedComplexity < high) {
    const t = (clampedComplexity - medium) / (high - medium);
    return interpolateColor(COMPLEXITY_COLORS.MEDIUM, COMPLEXITY_COLORS.HIGH, t);
  } else {
    const t = clamp((clampedComplexity - high) / (veryHigh - high), 0, 1);
    return interpolateColor(COMPLEXITY_COLORS.HIGH, COMPLEXITY_COLORS.VERY_HIGH, t);
  }
}

/**
 * Get a descriptive label for a complexity score
 * @param complexity - Cyclomatic complexity score
 * @param thresholds - Optional custom thresholds
 * @returns Human-readable complexity level
 *
 * @example
 * getComplexityLabel(5); // Returns "Low"
 * getComplexityLabel(15); // Returns "Medium"
 */
export function getComplexityLabel(
  complexity: number,
  thresholds: ComplexityThresholds = COMPLEXITY_THRESHOLDS
): string {
  const low = thresholds.LOW ?? COMPLEXITY_THRESHOLDS.LOW;
  const medium = thresholds.MEDIUM ?? COMPLEXITY_THRESHOLDS.MEDIUM;
  const high = thresholds.HIGH ?? COMPLEXITY_THRESHOLDS.HIGH;

  if (complexity < low) return 'Low';
  if (complexity < medium) return 'Medium';
  if (complexity < high) return 'High';
  return 'Very High';
}

/**
 * Create a color legend array for UI display
 * @param thresholds - Optional custom thresholds
 * @returns Legend entries
 */
export function getColorLegend(
  thresholds: ComplexityThresholds = COMPLEXITY_THRESHOLDS
): ColorLegendEntry[] {
  const low = thresholds.LOW ?? COMPLEXITY_THRESHOLDS.LOW;
  const medium = thresholds.MEDIUM ?? COMPLEXITY_THRESHOLDS.MEDIUM;
  const high = thresholds.HIGH ?? COMPLEXITY_THRESHOLDS.HIGH;

  return [
    {
      label: 'Low',
      color: COMPLEXITY_COLORS.LOW,
      range: `0-${low}`
    },
    {
      label: 'Medium',
      color: COMPLEXITY_COLORS.MEDIUM,
      range: `${low}-${medium}`
    },
    {
      label: 'High',
      color: COMPLEXITY_COLORS.HIGH,
      range: `${medium}-${high}`
    },
    {
      label: 'Very High',
      color: COMPLEXITY_COLORS.VERY_HIGH,
      range: `${high}+`
    }
  ];
}

/**
 * Apply color to a Three.js material based on complexity
 * @param material - Material to update
 * @param complexity - Cyclomatic complexity score
 * @param thresholds - Optional custom thresholds
 */
export function applyComplexityColor(
  material: THREE.Material,
  complexity: number,
  thresholds: ComplexityThresholds = COMPLEXITY_THRESHOLDS
): void {
  const color = complexityToColor(complexity, thresholds);
  if ('color' in material) {
    (material as THREE.MeshStandardMaterial).color.copy(color);
  }
}
