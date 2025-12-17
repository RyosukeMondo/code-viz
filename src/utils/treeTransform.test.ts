import { describe, it, expect } from 'vitest'
import type { TreeNode, EChartsTreemapNode } from '../types'
import {
  treeNodeToECharts,
  filterByPath,
  getTotalLOC,
  getFileCount,
  flattenTree,
  findNodeByPath,
} from './treeTransform'

// Helper function to create mock TreeNode
function createMockTreeNode(overrides: Partial<TreeNode> = {}): TreeNode {
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
  }
}

describe('treeTransform utilities', () => {
  describe('treeNodeToECharts', () => {
    describe('basic transformation', () => {
      it('should transform TreeNode to ECharts format', () => {
        const node = createMockTreeNode({
          name: 'file.ts',
          loc: 200,
          complexity: 45,
          path: 'src/file.ts',
          type: 'file',
        })

        const result = treeNodeToECharts(node)

        expect(result).toEqual({
          name: 'file.ts',
          value: 200,
          complexity: 45,
          path: 'src/file.ts',
          type: 'file',
          itemStyle: {
            color: expect.any(String),
          },
        })
      })

      it('should map loc to value', () => {
        const node = createMockTreeNode({ loc: 500 })
        const result = treeNodeToECharts(node)

        expect(result.value).toBe(500)
        expect(result).not.toHaveProperty('loc')
      })

      it('should preserve complexity score', () => {
        const node = createMockTreeNode({ complexity: 75 })
        const result = treeNodeToECharts(node)

        expect(result.complexity).toBe(75)
      })

      it('should preserve path', () => {
        const node = createMockTreeNode({ path: 'src/components/Button.tsx' })
        const result = treeNodeToECharts(node)

        expect(result.path).toBe('src/components/Button.tsx')
      })

      it('should preserve type', () => {
        const fileNode = createMockTreeNode({ type: 'file' })
        const dirNode = createMockTreeNode({ type: 'directory' })

        expect(treeNodeToECharts(fileNode).type).toBe('file')
        expect(treeNodeToECharts(dirNode).type).toBe('directory')
      })
    })

    describe('color mapping', () => {
      it('should add color for file nodes', () => {
        const node = createMockTreeNode({ type: 'file', complexity: 50 })
        const result = treeNodeToECharts(node)

        expect(result.itemStyle).toBeDefined()
        expect(result.itemStyle?.color).toMatch(/^#[0-9a-f]{6}$/i)
      })

      it('should not add color for directory nodes', () => {
        const node = createMockTreeNode({ type: 'directory', complexity: 50 })
        const result = treeNodeToECharts(node)

        expect(result.itemStyle).toBeUndefined()
      })

      it('should map different complexity scores to different colors', () => {
        const lowComplexity = createMockTreeNode({ type: 'file', complexity: 10 })
        const highComplexity = createMockTreeNode({ type: 'file', complexity: 90 })

        const lowResult = treeNodeToECharts(lowComplexity)
        const highResult = treeNodeToECharts(highComplexity)

        expect(lowResult.itemStyle?.color).not.toBe(highResult.itemStyle?.color)
      })
    })

    describe('recursive transformation', () => {
      it('should recursively transform children', () => {
        const node = createMockTreeNode({
          name: 'src',
          type: 'directory',
          children: [
            createMockTreeNode({ name: 'file1.ts', path: 'src/file1.ts' }),
            createMockTreeNode({ name: 'file2.ts', path: 'src/file2.ts' }),
          ],
        })

        const result = treeNodeToECharts(node)

        expect(result.children).toBeDefined()
        expect(result.children).toHaveLength(2)
        expect(result.children?.[0].name).toBe('file1.ts')
        expect(result.children?.[1].name).toBe('file2.ts')
      })

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
        })

        const result = treeNodeToECharts(node)

        expect(result.children?.[0].children?.[0].children?.[0].name).toBe('deep.ts')
      })

      it('should handle empty children arrays', () => {
        const node = createMockTreeNode({
          type: 'directory',
          children: [],
        })

        const result = treeNodeToECharts(node)

        // Empty children array results in no children property (or empty array)
        expect(result.children === undefined || result.children.length === 0).toBe(true)
      })
    })

    describe('edge cases', () => {
      it('should throw error for null node', () => {
        expect(() => treeNodeToECharts(null as any)).toThrow()
      })

      it('should throw error for undefined node', () => {
        expect(() => treeNodeToECharts(undefined as any)).toThrow()
      })
    })
  })

  describe('filterByPath', () => {
    const createTestTree = (): TreeNode => {
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
      })
    }

    describe('successful filtering', () => {
      it('should return root for empty path', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, [])

        expect(result).toBe(tree)
      })

      it('should find direct child', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['src'])

        expect(result).not.toBeNull()
        expect(result?.name).toBe('src')
        expect(result?.path).toBe('src')
      })

      it('should find nested node', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['src', 'components'])

        expect(result).not.toBeNull()
        expect(result?.name).toBe('components')
        expect(result?.path).toBe('src/components')
      })

      it('should find deeply nested file', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['src', 'components', 'Button.tsx'])

        expect(result).not.toBeNull()
        expect(result?.name).toBe('Button.tsx')
        expect(result?.path).toBe('src/components/Button.tsx')
      })
    })

    describe('path not found', () => {
      it('should return null for non-existent path', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['nonexistent'])

        expect(result).toBeNull()
      })

      it('should return null for partially valid path', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['src', 'nonexistent'])

        expect(result).toBeNull()
      })

      it('should return null for path beyond leaf node', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['README.md', 'subfolder'])

        expect(result).toBeNull()
      })
    })

    describe('edge cases', () => {
      it('should return null for null node', () => {
        const result = filterByPath(null as any, ['src'])
        expect(result).toBeNull()
      })

      it('should return null for undefined node', () => {
        const result = filterByPath(undefined as any, ['src'])
        expect(result).toBeNull()
      })

      it('should handle empty segments in path', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['', 'src', ''])

        expect(result).not.toBeNull()
        expect(result?.name).toBe('src')
      })

      it('should return root for path with only empty segments', () => {
        const tree = createTestTree()
        const result = filterByPath(tree, ['', '', ''])

        expect(result).toBe(tree)
      })
    })
  })

  describe('getTotalLOC', () => {
    it('should return LOC for file node', () => {
      const node = createMockTreeNode({ loc: 500 })
      expect(getTotalLOC(node)).toBe(500)
    })

    it('should return total LOC for directory (already aggregated)', () => {
      const node = createMockTreeNode({
        type: 'directory',
        loc: 1000,
        children: [
          createMockTreeNode({ loc: 400 }),
          createMockTreeNode({ loc: 600 }),
        ],
      })

      expect(getTotalLOC(node)).toBe(1000)
    })

    it('should return 0 for null node', () => {
      expect(getTotalLOC(null)).toBe(0)
    })

    it('should return 0 for undefined node', () => {
      expect(getTotalLOC(undefined as any)).toBe(0)
    })

    it('should handle zero LOC', () => {
      const node = createMockTreeNode({ loc: 0 })
      expect(getTotalLOC(node)).toBe(0)
    })
  })

  describe('getFileCount', () => {
    it('should return 1 for file node', () => {
      const node = createMockTreeNode({ type: 'file' })
      expect(getFileCount(node)).toBe(1)
    })

    it('should return 0 for empty directory', () => {
      const node = createMockTreeNode({ type: 'directory', children: [] })
      expect(getFileCount(node)).toBe(0)
    })

    it('should count all files in directory', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [
          createMockTreeNode({ type: 'file' }),
          createMockTreeNode({ type: 'file' }),
          createMockTreeNode({ type: 'file' }),
        ],
      })

      expect(getFileCount(node)).toBe(3)
    })

    it('should count files recursively in nested directories', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [
          createMockTreeNode({ type: 'file' }),
          createMockTreeNode({
            type: 'directory',
            children: [
              createMockTreeNode({ type: 'file' }),
              createMockTreeNode({ type: 'file' }),
            ],
          }),
          createMockTreeNode({
            type: 'directory',
            children: [
              createMockTreeNode({ type: 'file' }),
            ],
          }),
        ],
      })

      expect(getFileCount(node)).toBe(4)
    })

    it('should not count directories', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [
          createMockTreeNode({ type: 'directory', children: [] }),
          createMockTreeNode({ type: 'directory', children: [] }),
          createMockTreeNode({ type: 'file' }),
        ],
      })

      expect(getFileCount(node)).toBe(1)
    })

    it('should return 0 for null node', () => {
      expect(getFileCount(null)).toBe(0)
    })

    it('should return 0 for undefined node', () => {
      expect(getFileCount(undefined as any)).toBe(0)
    })
  })

  describe('flattenTree', () => {
    it('should return empty array for null node', () => {
      expect(flattenTree(null)).toEqual([])
    })

    it('should return single node for leaf', () => {
      const node = createMockTreeNode({ name: 'file.ts' })
      const result = flattenTree(node)

      expect(result).toHaveLength(1)
      expect(result[0]).toBe(node)
    })

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
      })

      const result = flattenTree(node)

      expect(result).toHaveLength(4)
      expect(result.map(n => n.name)).toEqual(['root', 'file1.ts', 'subdir', 'file2.ts'])
    })

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
      })

      const result = flattenTree(node)

      expect(result).toHaveLength(4)
      expect(result.map(n => n.name)).toEqual(['a', 'b', 'c', 'd'])
    })

    it('should handle empty children arrays', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [],
      })

      const result = flattenTree(node)

      expect(result).toHaveLength(1)
      expect(result[0]).toBe(node)
    })
  })

  describe('findNodeByPath', () => {
    const createTestTree = (): TreeNode => {
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
        ],
      })
    }

    describe('successful finding', () => {
      it('should return null for empty path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, '')

        // Empty path is treated as invalid by the implementation
        expect(result).toBeNull()
      })

      it('should find node by full path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'src/file1.ts')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('file1.ts')
        expect(result?.path).toBe('src/file1.ts')
      })

      it('should find deeply nested node', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'src/components/Button.tsx')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('Button.tsx')
        expect(result?.path).toBe('src/components/Button.tsx')
      })

      it('should find directory node', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'src/components')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('components')
        expect(result?.type).toBe('directory')
      })
    })

    describe('path normalization', () => {
      it('should handle leading slashes', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, '/src/file1.ts')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('file1.ts')
      })

      it('should handle trailing slashes', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'src/components/')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('components')
      })

      it('should handle multiple leading/trailing slashes', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, '///src/file1.ts///')

        expect(result).not.toBeNull()
        expect(result?.name).toBe('file1.ts')
      })
    })

    describe('path not found', () => {
      it('should return null for non-existent path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'nonexistent/path')

        expect(result).toBeNull()
      })

      it('should return null for partially valid path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, 'src/nonexistent.ts')

        expect(result).toBeNull()
      })
    })

    describe('edge cases', () => {
      it('should return null for null root', () => {
        const result = findNodeByPath(null, 'src/file.ts')
        expect(result).toBeNull()
      })

      it('should return null for undefined root', () => {
        const result = findNodeByPath(undefined as any, 'src/file.ts')
        expect(result).toBeNull()
      })

      it('should return null for null path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, null as any)
        expect(result).toBeNull()
      })

      it('should return null for undefined path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, undefined as any)
        expect(result).toBeNull()
      })

      it('should return null when path matches empty root path', () => {
        const tree = createTestTree()
        const result = findNodeByPath(tree, tree.path)
        // Empty root path ('') is treated as invalid
        expect(result).toBeNull()
      })
    })
  })
})
