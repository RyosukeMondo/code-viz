/**
 * Tests for TreemapLayout.validateLayout() method
 * @module components/visualizations/3d/layout/TreemapLayout.validate.test
 */

import { describe, it, expect } from 'vitest';
import { TreemapLayout } from './TreemapLayout';
import { createTestHierarchy } from './test-helpers';
import type { LayoutNode } from '../types';

describe('TreemapLayout.validateLayout', () => {
  it('should validate correct layout with no errors', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();
    const nodes = layout.compute(hierarchy);

    const validation = layout.validateLayout(nodes);
    expect(validation.valid).toBe(true);
    expect(validation.errors).toHaveLength(0);
  });

  it('should detect nodes exceeding width bounds', () => {
    const layout = new TreemapLayout(1000, 1000);
    const invalidNode: LayoutNode = {
      id: 'test',
      name: 'test.ts',
      path: '/test.ts',
      x0: 0,
      x1: 1100,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([invalidNode]);
    expect(validation.valid).toBe(false);
    expect(validation.errors).toContain('Node test.ts exceeds width bounds: [0, 1100]');
  });

  it('should detect nodes exceeding depth bounds', () => {
    const layout = new TreemapLayout(1000, 1000);
    const invalidNode: LayoutNode = {
      id: 'test',
      name: 'test.ts',
      path: '/test.ts',
      x0: 0,
      x1: 100,
      y0: 0,
      y1: 1200,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([invalidNode]);
    expect(validation.valid).toBe(false);
    expect(validation.errors).toContain('Node test.ts exceeds depth bounds: [0, 1200]');
  });

  it('should detect negative coordinates', () => {
    const layout = new TreemapLayout(1000, 1000);
    const invalidNode: LayoutNode = {
      id: 'test',
      name: 'test.ts',
      path: '/test.ts',
      x0: -10,
      x1: 100,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([invalidNode]);
    expect(validation.valid).toBe(false);
    expect(validation.errors.length).toBeGreaterThan(0);
  });

  it('should detect invalid x coordinates (x0 >= x1)', () => {
    const layout = new TreemapLayout(1000, 1000);
    const invalidNode: LayoutNode = {
      id: 'test',
      name: 'test.ts',
      path: '/test.ts',
      x0: 100,
      x1: 100,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([invalidNode]);
    expect(validation.valid).toBe(false);
    expect(validation.errors).toContain('Node test.ts has invalid x coordinates: x0=100 >= x1=100');
  });

  it('should detect invalid y coordinates (y0 >= y1)', () => {
    const layout = new TreemapLayout(1000, 1000);
    const invalidNode: LayoutNode = {
      id: 'test',
      name: 'test.ts',
      path: '/test.ts',
      x0: 0,
      x1: 100,
      y0: 100,
      y1: 50,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([invalidNode]);
    expect(validation.valid).toBe(false);
    expect(validation.errors).toContain('Node test.ts has invalid y coordinates: y0=100 >= y1=50');
  });

  it('should detect overlapping nodes', () => {
    const layout = new TreemapLayout(1000, 1000);
    const node1: LayoutNode = {
      id: 'test1',
      name: 'test1.ts',
      path: '/test1.ts',
      x0: 0,
      x1: 100,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const node2: LayoutNode = {
      id: 'test2',
      name: 'test2.ts',
      path: '/test2.ts',
      x0: 50,
      x1: 150,
      y0: 50,
      y1: 150,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([node1, node2]);
    expect(validation.valid).toBe(false);
    expect(validation.errors).toContain('Overlap detected between test1.ts and test2.ts');
  });

  it('should not detect overlap for adjacent nodes', () => {
    const layout = new TreemapLayout(1000, 1000);
    const node1: LayoutNode = {
      id: 'test1',
      name: 'test1.ts',
      path: '/test1.ts',
      x0: 0,
      x1: 100,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const node2: LayoutNode = {
      id: 'test2',
      name: 'test2.ts',
      path: '/test2.ts',
      x0: 100,
      x1: 200,
      y0: 0,
      y1: 100,
      height: 10,
      color: '#cccccc',
      metrics: { loc: 100, complexity: 5, functions: 10, lastModified: '2025-01-01' }
    };

    const validation = layout.validateLayout([node1, node2]);
    expect(validation.valid).toBe(true);
    expect(validation.errors).toHaveLength(0);
  });

  it('should handle empty node array', () => {
    const layout = new TreemapLayout(1000, 1000);
    const validation = layout.validateLayout([]);
    expect(validation.valid).toBe(true);
    expect(validation.errors).toHaveLength(0);
  });
});
