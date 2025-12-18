/**
 * Color mapping utilities for complexity visualization
 * Maps complexity scores (0-100) to WCAG AA compliant colors
 */

/**
 * Interpolates between two values
 */
function lerp(start: number, end: number, t: number): number {
  return start + (end - start) * t;
}

/**
 * Converts RGB values to hex color code
 */
function rgbToHex(r: number, g: number, b: number): string {
  const toHex = (n: number) => {
    const hex = Math.round(n).toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  };
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

/**
 * Maps a complexity score (0-100) to a color on a green-yellow-red gradient
 *
 * Color stops (WCAG AA compliant):
 * - 0-30: Green (#22c55e) - Low complexity
 * - 30-60: Yellow (#eab308) - Medium complexity
 * - 60-100: Red (#ef4444) - High complexity
 *
 * @param score - Complexity score from 0 to 100
 * @returns Hex color code (e.g., "#22c55e")
 */
export function complexityToColor(score: number): string {
  // Handle edge cases
  if (typeof score !== 'number' || isNaN(score)) {
    return '#94a3b8'; // gray-400 for invalid values
  }

  // Clamp score to 0-100 range
  const clampedScore = Math.max(0, Math.min(100, score));

  // Define color stops (WCAG AA compliant)
  // Green: rgb(34, 197, 94) - #22c55e
  // Yellow: rgb(234, 179, 8) - #eab308
  // Red: rgb(239, 68, 68) - #ef4444

  if (clampedScore <= 30) {
    // Green to Yellow transition (0-30)
    const t = clampedScore / 30;
    const r = lerp(34, 234, t);
    const g = lerp(197, 179, t);
    const b = lerp(94, 8, t);
    return rgbToHex(r, g, b);
  } else if (clampedScore <= 60) {
    // Yellow to Red transition (30-60)
    const t = (clampedScore - 30) / 30;
    const r = lerp(234, 239, t);
    const g = lerp(179, 68, t);
    const b = lerp(8, 68, t);
    return rgbToHex(r, g, b);
  } else {
    // Red gradient (60-100)
    const t = (clampedScore - 60) / 40;
    const r = lerp(239, 220, t);
    const g = lerp(68, 38, t);
    const b = lerp(68, 38, t);
    return rgbToHex(r, g, b);
  }
}

/**
 * Returns an array of color stops for the complexity gradient
 * Useful for creating legends or color scales
 *
 * @returns Array of objects with score and color
 */
export function getComplexityGradient(): Array<{ score: number; color: string; label: string }> {
  return [
    { score: 0, color: '#22c55e', label: 'Low' },
    { score: 15, color: '#84cc16', label: '' },
    { score: 30, color: '#eab308', label: 'Medium' },
    { score: 45, color: '#f59e0b', label: '' },
    { score: 60, color: '#ef4444', label: 'High' },
    { score: 80, color: '#dc2626', label: '' },
    { score: 100, color: '#dc2626', label: 'Very High' },
  ];
}

/**
 * Gets a complexity level label for a given score
 *
 * @param score - Complexity score from 0 to 100
 * @returns Label string (Low, Medium, High, Very High)
 */
export function getComplexityLabel(score: number): string {
  if (typeof score !== 'number' || isNaN(score)) {
    return 'Unknown';
  }

  const clampedScore = Math.max(0, Math.min(100, score));

  if (clampedScore < 30) return 'Low';
  if (clampedScore < 60) return 'Medium';
  if (clampedScore < 80) return 'High';
  return 'Very High';
}

/**
 * Maps a dead code ratio (0.0 to 1.0) to a border color
 *
 * Color mapping:
 * - >0.5: Red (#ef4444) - High amount of dead code
 * - 0.2-0.5: Orange (#f97316) - Medium amount of dead code
 * - <0.2: Yellow (#eab308) - Low amount of dead code
 *
 * @param ratio - Dead code ratio from 0.0 to 1.0
 * @returns Hex color code for border (e.g., "#ef4444")
 */
export function deadCodeBorderColor(ratio: number): string {
  if (typeof ratio !== 'number' || isNaN(ratio)) {
    return '#eab308'; // yellow for invalid values
  }

  const clampedRatio = Math.max(0, Math.min(1, ratio));

  if (clampedRatio > 0.5) return '#ef4444'; // red
  if (clampedRatio > 0.2) return '#f97316'; // orange
  return '#eab308'; // yellow
}

/**
 * Maps a confidence score (0-100) to a color for dead code visualization
 *
 * Color mapping:
 * - >80: Green (#22c55e) - High confidence it's safe to delete
 * - 60-80: Yellow (#eab308) - Medium confidence, review needed
 * - <60: Red (#ef4444) - Low confidence, likely false positive
 *
 * @param confidence - Confidence score from 0 to 100
 * @returns Hex color code (e.g., "#22c55e")
 */
export function confidenceToColor(confidence: number): string {
  if (typeof confidence !== 'number' || isNaN(confidence)) {
    return '#94a3b8'; // gray-400 for invalid values
  }

  const clampedScore = Math.max(0, Math.min(100, confidence));

  if (clampedScore > 80) return '#22c55e'; // green
  if (clampedScore >= 60) return '#eab308'; // yellow
  return '#ef4444'; // red
}
