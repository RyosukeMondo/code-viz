/**
 * Tree Transform Structure Tests
 *
 * Tests tree building logic, path handling, node creation, and filtering
 */

import { describe, it, expect } from 'vitest';
import {
  treeNodeToECharts,
  filterByPath,
  flattenTree,
  findNodeByPath,
} from '../treeTransform';
import { createMockTreeNode, createTestTree } from './test-utils';

describe('treeTransform - Structure', () => {
  describe('treeNodeToECharts', () => {
    describe('basic transformation', () => {
      it('should transform TreeNode to ECharts format', () => {
        const node = createMockTreeNode({
          name: 'file.ts',
          loc: 200,
          complexity: 45,
          path: 'src/file.ts',
          type: 'file',
        });

        const result = treeNodeToECharts(node);

        expect(result).toEqual({
          name: 'file.ts',
          value: 200,
          complexity: 45,
          path: 'src/file.ts',
          type: 'file',
          itemStyle: {
            color: expect.any(String),
          },
        });
      });

      it('should map loc to value', () => {
        const node = createMockTreeNode({ loc: 500 });
        const result = treeNodeToECharts(node);

        expect(result.value).toBe(500);
        expect(result).not.toHaveProperty('loc');
      });

      it('should preserve complexity score', () => {
        const node = createMockTreeNode({ complexity: 75 });
        const result = treeNodeToECharts(node);

        expect(result.complexity).toBe(75);
      });

      it('should preserve path', () => {
        const node = createMockTreeNode({ path: 'src/components/Button.tsx' });
        const result = treeNodeToECharts(node);

        expect(result.path).toBe('src/components/Button.tsx');
      });

      it('should preserve type', () => {
        const fileNode = createMockTreeNode({ type: 'file' });
        const dirNode = createMockTreeNode({ type: 'directory' });

        expect(treeNodeToECharts(fileNode).type).toBe('file');
        expect(treeNodeToECharts(dirNode).type).toBe('directory');
      });
    });

    describe('color mapping', () => {
      it('should add color for file nodes', () => {
        const node = createMockTreeNode({ type: 'file', complexity: 50 });
        const result = treeNodeToECharts(node);

        expect(result.itemStyle).toBeDefined();
        expect(result.itemStyle?.color).toMatch(/^#[0-9a-f]{6}$/i);
      });

      it('should not add color for directory nodes', () => {
        const node = createMockTreeNode({ type: 'directory', complexity: 50 });
        const result = treeNodeToECharts(node);

        expect(result.itemStyle).toBeUndefined();
      });

      it('should map different complexity scores to different colors', () => {
        const lowComplexity = createMockTreeNode({ type: 'file', complexity: 10 });
        const highComplexity = createMockTreeNode({ type: 'file', complexity: 90 });

        const lowResult = treeNodeToECharts(lowComplexity);
        const highResult = treeNodeToECharts(highComplexity);

        expect(lowResult.itemStyle?.color).not.toBe(highResult.itemStyle?.color);
      });
    });

    describe('recursive transformation', () => {
      it('should recursively transform children', () => {
        const node = createMockTreeNode({
          name: 'src',
          type: 'directory',
          children: [
            createMockTreeNode({ name: 'file1.ts', path: 'src/file1.ts' }),
            createMockTreeNode({ name: 'file2.ts', path: 'src/file2.ts' }),
          ],
        });

        const result = treeNodeToECharts(node);

        expect(result.children).toBeDefined();
        expect(result.children).toHaveLength(2);
        expect(result.children?.[0].name).toBe('file1.ts');
        expect(result.children?.[1].name).toBe('file2.ts');
      });

      it('should handle deeply nested trees', () => {
        const node = createMockTreeNode({
          name: 'root',
          type: 'directory',
          children: [
            createMockTreeNode({
              name: 'level1',
              type: 'directory',
              children: [
                createMockTreeNode({
                  name: 'level2',
                  type: 'directory',
                  children: [
                    createMockTreeNode({ name: 'deep.ts' }),
                  ],
                }),
              ],
            }),
          ],
        });

        const result = treeNodeToECharts(node);

        expect(result.children?.[0].children?.[0].children?.[0].name).toBe('deep.ts');
      });

      it('should handle empty children arrays', () => {
        const node = createMockTreeNode({
          type: 'directory',
          children: [],
        });

        const result = treeNodeToECharts(node);

        expect(result.children === undefined || result.children.length === 0).toBe(true);
      });
    });

    describe('edge cases', () => {
      it('should throw error for null node', () => {
        expect(() => treeNodeToECharts(null )).toThrow();
      });

      it('should throw error for undefined node', () => {
        expect(() => treeNodeToECharts(undefined )).toThrow();
      });
    });
  });

  describe('filterByPath', () => {
    describe('successful filtering', () => {
      it('should return root for empty path', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, []);

        expect(result).toBe(tree);
      });

      it('should find direct child', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['src']);

        expect(result).not.toBeNull();
        expect(result?.name).toBe('src');
        expect(result?.path).toBe('src');
      });

      it('should find nested node', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['src', 'components']);

        expect(result).not.toBeNull();
        expect(result?.name).toBe('components');
        expect(result?.path).toBe('src/components');
      });

      it('should find deeply nested file', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['src', 'components', 'Button.tsx']);

        expect(result).not.toBeNull();
        expect(result?.name).toBe('Button.tsx');
        expect(result?.path).toBe('src/components/Button.tsx');
      });
    });

    describe('path not found', () => {
      it('should return null for non-existent path', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['nonexistent']);

        expect(result).toBeNull();
      });

      it('should return null for partially valid path', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['src', 'nonexistent']);

        expect(result).toBeNull();
      });

      it('should return null for path beyond leaf node', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['README.md', 'subfolder']);

        expect(result).toBeNull();
      });
    });

    describe('edge cases', () => {
      it('should return null for null node', () => {
        const result = filterByPath(null , ['src']);
        expect(result).toBeNull();
      });

      it('should return null for undefined node', () => {
        const result = filterByPath(undefined , ['src']);
        expect(result).toBeNull();
      });

      it('should handle empty segments in path', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['', 'src', '']);

        expect(result).not.toBeNull();
        expect(result?.name).toBe('src');
      });

      it('should return root for path with only empty segments', () => {
        const tree = createTestTree();
        const result = filterByPath(tree, ['', '', '']);

        expect(result).toBe(tree);
      });
    });
  });

  describe('flattenTree', () => {
    it('should return empty array for null node', () => {
      expect(flattenTree(null)).toEqual([]);
    });

    it('should return single node for leaf', () => {
      const node = createMockTreeNode({ name: 'file.ts' });
      const result = flattenTree(node);

      expect(result).toHaveLength(1);
      expect(result[0]).toBe(node);
    });

    it('should flatten tree in depth-first order', () => {
      const node = createMockTreeNode({
        name: 'root',
        type: 'directory',
        children: [
          createMockTreeNode({ name: 'file1.ts' }),
          createMockTreeNode({
            name: 'subdir',
            type: 'directory',
            children: [
              createMockTreeNode({ name: 'file2.ts' }),
            ],
          }),
        ],
      });

      const result = flattenTree(node);

      expect(result).toHaveLength(4);
      expect(result.map(n => n.name)).toEqual(['root', 'file1.ts', 'subdir', 'file2.ts']);
    });

    it('should handle deeply nested trees', () => {
      const node = createMockTreeNode({
        name: 'a',
        type: 'directory',
        children: [
          createMockTreeNode({
            name: 'b',
            type: 'directory',
            children: [
              createMockTreeNode({
                name: 'c',
                type: 'directory',
                children: [
                  createMockTreeNode({ name: 'd' }),
                ],
              }),
            ],
          }),
        ],
      });

      const result = flattenTree(node);

      expect(result).toHaveLength(4);
      expect(result.map(n => n.name)).toEqual(['a', 'b', 'c', 'd']);
    });

    it('should handle empty children arrays', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [],
      });

      const result = flattenTree(node);

      expect(result).toHaveLength(1);
      expect(result[0]).toBe(node);
    });
  });

  describe('findNodeByPath', () => {
    describe('successful finding', () => {
      it('should return null for empty path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, '');

        expect(result).toBeNull();
      });

      it('should find node by full path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'src/file1.ts');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('file1.ts');
        expect(result?.path).toBe('src/file1.ts');
      });

      it('should find deeply nested node', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'src/components/Button.tsx');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('Button.tsx');
        expect(result?.path).toBe('src/components/Button.tsx');
      });

      it('should find directory node', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'src/components');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('components');
        expect(result?.type).toBe('directory');
      });
    });

    describe('path normalization', () => {
      it('should handle leading slashes', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, '/src/file1.ts');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('file1.ts');
      });

      it('should handle trailing slashes', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'src/components/');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('components');
      });

      it('should handle multiple leading/trailing slashes', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, '///src/file1.ts///');

        expect(result).not.toBeNull();
        expect(result?.name).toBe('file1.ts');
      });
    });

    describe('path not found', () => {
      it('should return null for non-existent path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'nonexistent/path');

        expect(result).toBeNull();
      });

      it('should return null for partially valid path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, 'src/nonexistent.ts');

        expect(result).toBeNull();
      });
    });

    describe('edge cases', () => {
      it('should return null for null root', () => {
        const result = findNodeByPath(null, 'src/file.ts');
        expect(result).toBeNull();
      });

      it('should return null for undefined root', () => {
        const result = findNodeByPath(undefined , 'src/file.ts');
        expect(result).toBeNull();
      });

      it('should return null for null path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, null );
        expect(result).toBeNull();
      });

      it('should return null for undefined path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, undefined );
        expect(result).toBeNull();
      });

      it('should return null when path matches empty root path', () => {
        const tree = createTestTree();
        const result = findNodeByPath(tree, tree.path);
        expect(result).toBeNull();
      });
    });
  });
});
