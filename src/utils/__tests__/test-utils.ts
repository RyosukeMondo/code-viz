/**
 * Shared test utilities for tree transform tests
 */

import type { TreeNode } from '@/types/bindings';

/**
 * Helper function to create mock TreeNode with defaults
 */
export function createMockTreeNode(overrides: Partial<TreeNode> = {}): TreeNode {
  return {
    id: 'test-id',
    name: 'test',
    path: 'test',
    loc: 100,
    complexity: 50,
    type: 'file',
    children: [],
    lastModified: '2024-01-01T00:00:00Z',
    ...overrides,
  };
}

/**
 * Creates a standard test tree structure for filtering tests
 */
export function createTestTree(): TreeNode {
  return createMockTreeNode({
    name: 'root',
    path: '',
    type: 'directory',
    children: [
      createMockTreeNode({
        name: 'src',
        path: 'src',
        type: 'directory',
        children: [
          createMockTreeNode({ name: 'file1.ts', path: 'src/file1.ts' }),
          createMockTreeNode({
            name: 'components',
            path: 'src/components',
            type: 'directory',
            children: [
              createMockTreeNode({ name: 'Button.tsx', path: 'src/components/Button.tsx' }),
            ],
          }),
        ],
      }),
      createMockTreeNode({ name: 'README.md', path: 'README.md' }),
    ],
  });
}
