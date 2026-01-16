/**
 * Test helpers for TreemapLayout tests
 * @module components/visualizations/3d/layout/test-helpers
 */

import type { HierarchyNode } from '../types';

export const createTestNode = (name: string, loc: number, complexity: number): HierarchyNode => ({
  name,
  type: 'file' as const,
  path: `/test/${name}`,
  metrics: {
    loc,
    complexity,
    functions: 10,
    lastModified: '2025-01-01'
  }
});

export const createTestHierarchy = (): HierarchyNode => ({
  name: 'root',
  type: 'directory' as const,
  path: '/test',
  children: [
    createTestNode('file1.ts', 100, 5),
    createTestNode('file2.ts', 200, 10),
    createTestNode('file3.ts', 300, 15)
  ]
});
