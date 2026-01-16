/**
 * Tests for TreemapLayout.compute() method
 * @module components/visualizations/3d/layout/TreemapLayout.compute.test
 */

import { describe, it, expect } from 'vitest';
import { TreemapLayout } from './TreemapLayout';
import { createTestNode, createTestHierarchy } from './test-helpers';
import type { HierarchyNode } from '../types';

describe('TreemapLayout.compute', () => {
  it('should throw error for null/undefined input', () => {
    const layout = new TreemapLayout(1000, 1000);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => layout.compute(null as any)).toThrow('TreemapLayout: hierarchyNode is required');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(() => layout.compute(undefined as any)).toThrow('TreemapLayout: hierarchyNode is required');
  });

  it('should return empty array for directory with no file children', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: []
    };

    const result = layout.compute(hierarchy);
    expect(result).toHaveLength(0);
  });

  it('should compute layout for single file', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: [createTestNode('file1.ts', 100, 5)]
    };

    const result = layout.compute(hierarchy);
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe('file1.ts');
    expect(result[0].path).toBe('/test/file1.ts');
  });

  it('should compute layout for multiple files', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);
    expect(result).toHaveLength(3);
    expect(result.map(n => n.name)).toContain('file1.ts');
    expect(result.map(n => n.name)).toContain('file2.ts');
    expect(result.map(n => n.name)).toContain('file3.ts');
  });

  it('should generate unique IDs for each node', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);
    const ids = result.map(n => n.id);
    const uniqueIds = new Set(ids);

    expect(uniqueIds.size).toBe(ids.length);
    ids.forEach(id => expect(id).toMatch(/^file-[a-z0-9]+$/));
  });

  it('should set bounds within world dimensions', () => {
    const width = 1000;
    const depth = 1000;
    const layout = new TreemapLayout(width, depth);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);

    result.forEach(node => {
      expect(node.x0).toBeGreaterThanOrEqual(0);
      expect(node.x1).toBeLessThanOrEqual(width);
      expect(node.y0).toBeGreaterThanOrEqual(0);
      expect(node.y1).toBeLessThanOrEqual(depth);
      expect(node.x0).toBeLessThan(node.x1);
      expect(node.y0).toBeLessThan(node.y1);
    });
  });

  it('should calculate height based on LOC using log scale', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: [
        createTestNode('small.ts', 10, 5),
        createTestNode('large.ts', 10000, 5)
      ]
    };

    const result = layout.compute(hierarchy);
    const small = result.find(n => n.name === 'small.ts');
    const large = result.find(n => n.name === 'large.ts');

    expect(small?.height).toBeLessThan(large?.height || 0);
    expect(small?.height).toBeCloseTo(10, 1);
    expect(large?.height).toBeCloseTo(40, 1);
  });

  it('should handle files with 0 LOC by using minimum of 1', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: [createTestNode('empty.ts', 0, 0)]
    };

    const result = layout.compute(hierarchy);
    expect(result).toHaveLength(1);
    expect(result[0].height).toBe(0);
  });

  it('should preserve all metrics in layout nodes', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: [
        {
          name: 'test.ts',
          type: 'file',
          path: '/test/test.ts',
          metrics: {
            loc: 100,
            complexity: 15,
            functions: 5,
            lastModified: '2025-01-17',
            churn: 10
          }
        }
      ]
    };

    const result = layout.compute(hierarchy);
    expect(result[0].metrics.loc).toBe(100);
    expect(result[0].metrics.complexity).toBe(15);
    expect(result[0].metrics.functions).toBe(5);
    expect(result[0].metrics.lastModified).toBe('2025-01-17');
    expect(result[0].metrics.churn).toBe(10);
  });

  it('should handle files without churn metric', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);
    result.forEach(node => {
      expect(node.metrics).toHaveProperty('loc');
      expect(node.metrics).toHaveProperty('complexity');
      expect(node.metrics).not.toHaveProperty('churn');
    });
  });

  it('should assign default color to all nodes', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);
    result.forEach(node => {
      expect(node.color).toBe('#cccccc');
    });
  });

  it('should sort nodes by LOC (largest first)', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy = createTestHierarchy();

    const result = layout.compute(hierarchy);

    const file1 = result.find(n => n.name === 'file1.ts');
    const file3 = result.find(n => n.name === 'file3.ts');

    const area1 = (file1!.x1 - file1!.x0) * (file1!.y1 - file1!.y0);
    const area3 = (file3!.x1 - file3!.x0) * (file3!.y1 - file3!.y0);

    expect(area3).toBeGreaterThan(area1);
  });

  it('should handle nested directory structure', () => {
    const layout = new TreemapLayout(1000, 1000);
    const hierarchy: HierarchyNode = {
      name: 'root',
      type: 'directory',
      path: '/test',
      children: [
        {
          name: 'src',
          type: 'directory',
          path: '/test/src',
          children: [
            createTestNode('nested.ts', 100, 5)
          ]
        },
        createTestNode('root.ts', 200, 10)
      ]
    };

    const result = layout.compute(hierarchy);
    expect(result).toHaveLength(2);
    expect(result.map(n => n.name)).toContain('nested.ts');
    expect(result.map(n => n.name)).toContain('root.ts');
  });
});
