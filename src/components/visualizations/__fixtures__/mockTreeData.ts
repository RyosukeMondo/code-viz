/**
 * Shared test fixtures for tree visualization tests
 */

import type { TreeNode } from '@/types/bindings';

/**
 * Standard mock tree data used across multiple test suites
 */
export const mockTreeData: TreeNode = {
  id: 'root',
  name: 'root',
  path: '/root',
  loc: 1000,
  complexity: 30,
  type: 'directory',
  lastModified: '2024-01-15T10:30:00Z',
  children: [
    {
      id: 'file-1',
      name: 'file1.ts',
      path: '/root/file1.ts',
      loc: 300,
      complexity: 40,
      type: 'file',
      children: [],
      lastModified: '2024-01-15T10:30:00Z',
    },
    {
      id: 'dir-1',
      name: 'subdir',
      path: '/root/subdir',
      loc: 700,
      complexity: 25,
      type: 'directory',
      lastModified: '2024-01-15T10:30:00Z',
      children: [
        {
          id: 'file-2',
          name: 'file2.ts',
          path: '/root/subdir/file2.ts',
          loc: 400,
          complexity: 20,
          type: 'file',
          children: [],
          lastModified: '2024-01-15T10:30:00Z',
        },
        {
          id: 'file-3',
          name: 'file3.ts',
          path: '/root/subdir/file3.ts',
          loc: 300,
          complexity: 30,
          type: 'file',
          children: [],
          lastModified: '2024-01-15T10:30:00Z',
        },
      ],
    },
  ],
};

/**
 * Tree data with dead code metrics
 */
export const mockTreeDataWithDeadCode: TreeNode = {
  ...mockTreeData,
  children: [
    {
      id: 'file-1',
      name: 'file1.ts',
      path: '/root/file1.ts',
      loc: 300,
      complexity: 40,
      type: 'file',
      children: [],
      lastModified: '2024-01-15T10:30:00Z',
      deadCodeRatio: 0.6, // 60% dead code - high
    },
    {
      id: 'file-2',
      name: 'file2.ts',
      path: '/root/file2.ts',
      loc: 200,
      complexity: 30,
      type: 'file',
      children: [],
      lastModified: '2024-01-15T10:30:00Z',
      deadCodeRatio: 0.3, // 30% dead code - medium
    },
    {
      id: 'file-3',
      name: 'file3.ts',
      path: '/root/file3.ts',
      loc: 100,
      complexity: 20,
      type: 'file',
      children: [],
      lastModified: '2024-01-15T10:30:00Z',
      deadCodeRatio: 0.1, // 10% dead code - low
    },
  ],
};

/**
 * Empty tree data
 */
export const emptyTreeData: TreeNode = {
  ...mockTreeData,
  children: [],
};

/**
 * Deeply nested tree structure
 */
export const deepTreeData: TreeNode = {
  id: 'root',
  name: 'root',
  path: '/root',
  loc: 100,
  complexity: 10,
  type: 'directory',
  lastModified: '2024-01-15T10:30:00Z',
  children: [
    {
      id: 'level1',
      name: 'level1',
      path: '/root/level1',
      loc: 50,
      complexity: 10,
      type: 'directory',
      lastModified: '2024-01-15T10:30:00Z',
      children: [
        {
          id: 'level2',
          name: 'level2',
          path: '/root/level1/level2',
          loc: 25,
          complexity: 10,
          type: 'directory',
          lastModified: '2024-01-15T10:30:00Z',
          children: [
            {
              id: 'file',
              name: 'deep.ts',
              path: '/root/level1/level2/deep.ts',
              loc: 25,
              complexity: 10,
              type: 'file',
              lastModified: '2024-01-15T10:30:00Z',
              children: [],
            },
          ],
        },
      ],
    },
  ],
};

/**
 * Zero LOC node
 */
export const zeroLocData: TreeNode = {
  ...mockTreeData,
  loc: 0,
  children: [],
};

/**
 * High complexity node
 */
export const highComplexityData: TreeNode = {
  ...mockTreeData,
  complexity: 100,
};
