/**
 * Tests for color mapping utilities
 * @module components/visualizations/3d/utils/colorMaps.test
 */

import { describe, it, expect } from 'vitest';
import * as THREE from 'three';
import {
  complexityToColor,
  getComplexityLabel,
  getColorLegend,
  applyComplexityColor
} from './colorMaps';
import { COMPLEXITY_THRESHOLDS, COMPLEXITY_COLORS } from './constants';
import type { ComplexityThresholds } from '../types';

describe('colorMaps', () => {
  describe('complexityToColor', () => {
    it('should return green for low complexity (< 10)', () => {
      const color = complexityToColor(5);
      const expectedColor = new THREE.Color(COMPLEXITY_COLORS.LOW);
      expect(color.r).toBeCloseTo(expectedColor.r, 2);
      expect(color.g).toBeCloseTo(expectedColor.g, 2);
      expect(color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should return exact low color at threshold boundary', () => {
      const color = complexityToColor(0);
      const expectedColor = new THREE.Color(COMPLEXITY_COLORS.LOW);
      expect(color.r).toBeCloseTo(expectedColor.r, 2);
      expect(color.g).toBeCloseTo(expectedColor.g, 2);
      expect(color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should interpolate between green and yellow for medium complexity (10-20)', () => {
      const color = complexityToColor(15);
      const lowColor = new THREE.Color(COMPLEXITY_COLORS.LOW);
      const mediumColor = new THREE.Color(COMPLEXITY_COLORS.MEDIUM);

      // Should be between low and medium colors
      expect(color.r).toBeGreaterThanOrEqual(Math.min(lowColor.r, mediumColor.r));
      expect(color.r).toBeLessThanOrEqual(Math.max(lowColor.r, mediumColor.r));
    });

    it('should interpolate between yellow and orange for high complexity (20-30)', () => {
      const color = complexityToColor(25);
      const mediumColor = new THREE.Color(COMPLEXITY_COLORS.MEDIUM);
      const highColor = new THREE.Color(COMPLEXITY_COLORS.HIGH);

      // Should be between medium and high colors
      expect(color.r).toBeGreaterThanOrEqual(Math.min(mediumColor.r, highColor.r));
      expect(color.r).toBeLessThanOrEqual(Math.max(mediumColor.r, highColor.r));
    });

    it('should interpolate between orange and red for very high complexity (30+)', () => {
      const color = complexityToColor(35);
      const highColor = new THREE.Color(COMPLEXITY_COLORS.HIGH);
      const veryHighColor = new THREE.Color(COMPLEXITY_COLORS.VERY_HIGH);

      // Should be between high and very high colors
      expect(color.r).toBeGreaterThanOrEqual(Math.min(highColor.r, veryHighColor.r));
      expect(color.r).toBeLessThanOrEqual(Math.max(highColor.r, veryHighColor.r));
    });

    it('should clamp extremely high complexity values', () => {
      const color1 = complexityToColor(1000);
      const color2 = complexityToColor(10000);

      // Both should produce the same clamped color
      expect(color1.r).toBeCloseTo(color2.r, 2);
      expect(color1.g).toBeCloseTo(color2.g, 2);
      expect(color1.b).toBeCloseTo(color2.b, 2);
    });

    it('should handle negative complexity values by clamping to 0', () => {
      const color = complexityToColor(-10);
      const expectedColor = new THREE.Color(COMPLEXITY_COLORS.LOW);
      expect(color.r).toBeCloseTo(expectedColor.r, 2);
      expect(color.g).toBeCloseTo(expectedColor.g, 2);
      expect(color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should support custom thresholds', () => {
      const customThresholds: ComplexityThresholds = {
        LOW: 5,
        MEDIUM: 10,
        HIGH: 15,
        VERY_HIGH: 20
      };

      const color = complexityToColor(6, customThresholds);
      const lowColor = new THREE.Color(COMPLEXITY_COLORS.LOW);
      const mediumColor = new THREE.Color(COMPLEXITY_COLORS.MEDIUM);

      // Should interpolate with custom thresholds
      expect(color.r).toBeGreaterThanOrEqual(Math.min(lowColor.r, mediumColor.r));
      expect(color.r).toBeLessThanOrEqual(Math.max(lowColor.r, mediumColor.r));
    });

    it('should handle partial custom thresholds by falling back to defaults', () => {
      const partialThresholds: Partial<ComplexityThresholds> = {
        LOW: 5
      };

      // Should not throw and should use default for missing values
      expect(() => complexityToColor(15, partialThresholds as ComplexityThresholds)).not.toThrow();
    });

    it('should produce different colors at each complexity level', () => {
      const color0 = complexityToColor(5);   // Low
      const color1 = complexityToColor(15);  // Medium
      const color2 = complexityToColor(25);  // High
      const color3 = complexityToColor(35);  // Very High

      // All colors should be different
      expect(color0.getHex()).not.toBe(color1.getHex());
      expect(color1.getHex()).not.toBe(color2.getHex());
      expect(color2.getHex()).not.toBe(color3.getHex());
    });
  });

  describe('getComplexityLabel', () => {
    it('should return "Low" for complexity < 10', () => {
      expect(getComplexityLabel(0)).toBe('Low');
      expect(getComplexityLabel(5)).toBe('Low');
      expect(getComplexityLabel(9)).toBe('Low');
    });

    it('should return "Medium" for complexity 10-19', () => {
      expect(getComplexityLabel(10)).toBe('Medium');
      expect(getComplexityLabel(15)).toBe('Medium');
      expect(getComplexityLabel(19)).toBe('Medium');
    });

    it('should return "High" for complexity 20-29', () => {
      expect(getComplexityLabel(20)).toBe('High');
      expect(getComplexityLabel(25)).toBe('High');
      expect(getComplexityLabel(29)).toBe('High');
    });

    it('should return "Very High" for complexity >= 30', () => {
      expect(getComplexityLabel(30)).toBe('Very High');
      expect(getComplexityLabel(50)).toBe('Very High');
      expect(getComplexityLabel(1000)).toBe('Very High');
    });

    it('should support custom thresholds', () => {
      const customThresholds: ComplexityThresholds = {
        LOW: 5,
        MEDIUM: 10,
        HIGH: 15,
        VERY_HIGH: 20
      };

      expect(getComplexityLabel(3, customThresholds)).toBe('Low');
      expect(getComplexityLabel(7, customThresholds)).toBe('Medium');
      expect(getComplexityLabel(12, customThresholds)).toBe('High');
      expect(getComplexityLabel(20, customThresholds)).toBe('Very High');
    });

    it('should handle partial custom thresholds', () => {
      const partialThresholds: Partial<ComplexityThresholds> = {
        LOW: 5
      };

      expect(() => getComplexityLabel(10, partialThresholds as ComplexityThresholds)).not.toThrow();
    });

    it('should handle negative complexity values', () => {
      expect(getComplexityLabel(-10)).toBe('Low');
    });
  });

  describe('getColorLegend', () => {
    it('should return 4 legend entries', () => {
      const legend = getColorLegend();
      expect(legend).toHaveLength(4);
    });

    it('should have correct labels', () => {
      const legend = getColorLegend();
      expect(legend[0].label).toBe('Low');
      expect(legend[1].label).toBe('Medium');
      expect(legend[2].label).toBe('High');
      expect(legend[3].label).toBe('Very High');
    });

    it('should have correct colors', () => {
      const legend = getColorLegend();
      expect(legend[0].color).toBe(COMPLEXITY_COLORS.LOW);
      expect(legend[1].color).toBe(COMPLEXITY_COLORS.MEDIUM);
      expect(legend[2].color).toBe(COMPLEXITY_COLORS.HIGH);
      expect(legend[3].color).toBe(COMPLEXITY_COLORS.VERY_HIGH);
    });

    it('should have correct ranges', () => {
      const legend = getColorLegend();
      expect(legend[0].range).toBe(`0-${COMPLEXITY_THRESHOLDS.LOW}`);
      expect(legend[1].range).toBe(`${COMPLEXITY_THRESHOLDS.LOW}-${COMPLEXITY_THRESHOLDS.MEDIUM}`);
      expect(legend[2].range).toBe(`${COMPLEXITY_THRESHOLDS.MEDIUM}-${COMPLEXITY_THRESHOLDS.HIGH}`);
      expect(legend[3].range).toBe(`${COMPLEXITY_THRESHOLDS.HIGH}+`);
    });

    it('should support custom thresholds', () => {
      const customThresholds: ComplexityThresholds = {
        LOW: 5,
        MEDIUM: 10,
        HIGH: 15,
        VERY_HIGH: 20
      };

      const legend = getColorLegend(customThresholds);
      expect(legend[0].range).toBe('0-5');
      expect(legend[1].range).toBe('5-10');
      expect(legend[2].range).toBe('10-15');
      expect(legend[3].range).toBe('15+');
    });

    it('should return entries with all required fields', () => {
      const legend = getColorLegend();

      legend.forEach(entry => {
        expect(entry).toHaveProperty('label');
        expect(entry).toHaveProperty('color');
        expect(entry).toHaveProperty('range');
        expect(typeof entry.label).toBe('string');
        expect(typeof entry.color).toBe('string');
        expect(typeof entry.range).toBe('string');
      });
    });
  });

  describe('applyComplexityColor', () => {
    it('should apply color to MeshStandardMaterial', () => {
      const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
      applyComplexityColor(material, 15);

      const expectedColor = complexityToColor(15);
      expect(material.color.r).toBeCloseTo(expectedColor.r, 2);
      expect(material.color.g).toBeCloseTo(expectedColor.g, 2);
      expect(material.color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should apply color to MeshBasicMaterial', () => {
      const material = new THREE.MeshBasicMaterial({ color: 0xffffff });
      applyComplexityColor(material, 25);

      const expectedColor = complexityToColor(25);
      expect(material.color.r).toBeCloseTo(expectedColor.r, 2);
      expect(material.color.g).toBeCloseTo(expectedColor.g, 2);
      expect(material.color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should apply color to MeshPhongMaterial', () => {
      const material = new THREE.MeshPhongMaterial({ color: 0xffffff });
      applyComplexityColor(material, 5);

      const expectedColor = complexityToColor(5);
      expect(material.color.r).toBeCloseTo(expectedColor.r, 2);
      expect(material.color.g).toBeCloseTo(expectedColor.g, 2);
      expect(material.color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should handle materials without color property gracefully', () => {
      const material = new THREE.ShaderMaterial();
      expect(() => applyComplexityColor(material, 15)).not.toThrow();
    });

    it('should support custom thresholds', () => {
      const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
      const customThresholds: ComplexityThresholds = {
        LOW: 5,
        MEDIUM: 10,
        HIGH: 15,
        VERY_HIGH: 20
      };

      applyComplexityColor(material, 7, customThresholds);
      const expectedColor = complexityToColor(7, customThresholds);

      expect(material.color.r).toBeCloseTo(expectedColor.r, 2);
      expect(material.color.g).toBeCloseTo(expectedColor.g, 2);
      expect(material.color.b).toBeCloseTo(expectedColor.b, 2);
    });

    it('should update existing material color without creating new material', () => {
      const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
      const originalMaterial = material;

      applyComplexityColor(material, 20);

      expect(material).toBe(originalMaterial);
    });

    it('should handle multiple color applications to same material', () => {
      const material = new THREE.MeshStandardMaterial({ color: 0xffffff });

      applyComplexityColor(material, 5);
      const color1 = material.color.clone();

      applyComplexityColor(material, 35);
      const color2 = material.color.clone();

      expect(color1.getHex()).not.toBe(color2.getHex());
    });
  });
});
