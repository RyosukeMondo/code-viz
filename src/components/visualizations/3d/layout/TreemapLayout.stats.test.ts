/**
 * Tests for TreemapLayout.computeStats() and constructor
 * @module components/visualizations/3d/layout/TreemapLayout.stats.test
 */

import { describe, it, expect } from 'vitest';
import { TreemapLayout, computeTreemapLayout } from './TreemapLayout';
import { createTestHierarchy } from './test-helpers';
import type { LayoutNode } from '../types';

describe('TreemapLayout', () => {
  describe('constructor', () => {
    it('should create instance with width and depth', () => {
      const layout = new TreemapLayout(1000, 1000);
      expect(layout).toBeInstanceOf(TreemapLayout);
    });

    it('should handle different dimensions', () => {
      const layout = new TreemapLayout(500, 800);
      expect(layout).toBeInstanceOf(TreemapLayout);
    });
  });

  describe('computeStats', () => {
    it('should return zero stats for empty array', () => {
      const layout = new TreemapLayout(1000, 1000);
      const stats = layout.computeStats([]);

      expect(stats.nodeCount).toBe(0);
      expect(stats.totalArea).toBe(0);
      expect(stats.averageArea).toBe(0);
      expect(stats.minArea).toBe(0);
      expect(stats.maxArea).toBe(0);
      expect(stats.averageAspectRatio).toBe(0);
    });

    it('should compute correct stats for single node', () => {
      const layout = new TreemapLayout(1000, 1000);
      const node: LayoutNode = {
        id: 'test',
        name: 'test.ts',
        path: '/test.ts',
        x0: 0,
        x1: 100,
        y0: 0,
        y1: 200,
        height: 10,
        color: '#cccccc',
        metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
      };

      const stats = layout.computeStats([node]);

      expect(stats.nodeCount).toBe(1);
      expect(stats.totalArea).toBe(20000);
      expect(stats.averageArea).toBe(20000);
      expect(stats.minArea).toBe(20000);
      expect(stats.maxArea).toBe(20000);
      expect(stats.averageAspectRatio).toBe(2);
      expect(stats.coveragePercent).toBeCloseTo(2, 1);
    });

    it('should compute correct stats for multiple nodes', () => {
      const layout = new TreemapLayout(1000, 1000);
      const hierarchy = createTestHierarchy();
      const nodes = layout.compute(hierarchy);

      const stats = layout.computeStats(nodes);

      expect(stats.nodeCount).toBe(3);
      expect(stats.totalArea).toBeGreaterThan(0);
      expect(stats.averageArea).toBe(stats.totalArea / 3);
      expect(stats.minArea).toBeLessThanOrEqual(stats.averageArea);
      expect(stats.maxArea).toBeGreaterThanOrEqual(stats.averageArea);
      expect(stats.averageAspectRatio).toBeGreaterThanOrEqual(1);
      expect(stats.coveragePercent).toBeGreaterThan(0);
      expect(stats.coveragePercent).toBeLessThanOrEqual(100);
    });

    it('should compute aspect ratio correctly', () => {
      const layout = new TreemapLayout(1000, 1000);
      const square: LayoutNode = {
        id: 'square',
        name: 'square.ts',
        path: '/square.ts',
        x0: 0,
        x1: 100,
        y0: 0,
        y1: 100,
        height: 10,
        color: '#cccccc',
        metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
      };

      const rectangle: LayoutNode = {
        id: 'rect',
        name: 'rect.ts',
        path: '/rect.ts',
        x0: 0,
        x1: 100,
        y0: 0,
        y1: 400,
        height: 10,
        color: '#cccccc',
        metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
      };

      const statsSquare = layout.computeStats([square]);
      const statsRect = layout.computeStats([rectangle]);

      expect(statsSquare.averageAspectRatio).toBeCloseTo(1, 1);
      expect(statsRect.averageAspectRatio).toBeCloseTo(4, 1);
    });

    it('should calculate coverage percentage correctly', () => {
      const layout = new TreemapLayout(1000, 1000);
      const hierarchy = createTestHierarchy();
      const nodes = layout.compute(hierarchy);

      const stats = layout.computeStats(nodes);

      const expectedCoverage = (stats.totalArea / (1000 * 1000)) * 100;
      expect(stats.coveragePercent).toBeCloseTo(expectedCoverage, 2);
    });
  });

  describe('computeTreemapLayout helper', () => {
    it('should compute layout with default dimensions', () => {
      const hierarchy = createTestHierarchy();
      const result = computeTreemapLayout(hierarchy);

      expect(result).toHaveLength(3);
      result.forEach(node => {
        expect(node.x0).toBeGreaterThanOrEqual(0);
        expect(node.x1).toBeLessThanOrEqual(1000);
        expect(node.y0).toBeGreaterThanOrEqual(0);
        expect(node.y1).toBeLessThanOrEqual(1000);
      });
    });

    it('should compute layout with custom dimensions', () => {
      const hierarchy = createTestHierarchy();
      const result = computeTreemapLayout(hierarchy, 500, 800);

      expect(result).toHaveLength(3);
      result.forEach(node => {
        expect(node.x0).toBeGreaterThanOrEqual(0);
        expect(node.x1).toBeLessThanOrEqual(500);
        expect(node.y0).toBeGreaterThanOrEqual(0);
        expect(node.y1).toBeLessThanOrEqual(800);
      });
    });

    it('should produce same result as TreemapLayout.compute', () => {
      const hierarchy = createTestHierarchy();
      const layout = new TreemapLayout(1000, 1000);

      const result1 = layout.compute(hierarchy);
      const result2 = computeTreemapLayout(hierarchy, 1000, 1000);

      expect(result1).toHaveLength(result2.length);
      result1.forEach((node, i) => {
        expect(node.name).toBe(result2[i].name);
        expect(node.path).toBe(result2[i].path);
      });
    });
  });
});
