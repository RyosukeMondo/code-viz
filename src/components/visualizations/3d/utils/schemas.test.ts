/**
 * Tests for schema validation utilities
 * @module components/visualizations/3d/utils/schemas.test
 */

import { describe, it, expect } from 'vitest';
import {
  isFileNode,
  isDirectoryNode,
  calculateTotalLOC,
  validateHierarchyNode
} from './schemas';
import type { HierarchyNode } from '../types';

describe('schemas', () => {
  describe('isFileNode', () => {
    it('should return true for file nodes', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(isFileNode(fileNode)).toBe(true);
    });

    it('should return false for directory nodes', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: []
      };

      expect(isFileNode(dirNode)).toBe(false);
    });
  });

  describe('isDirectoryNode', () => {
    it('should return true for directory nodes', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: []
      };

      expect(isDirectoryNode(dirNode)).toBe(true);
    });

    it('should return false for file nodes', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(isDirectoryNode(fileNode)).toBe(false);
    });
  });

  describe('calculateTotalLOC', () => {
    it('should return LOC for a file node', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 150, complexity: 10 }
      };

      expect(calculateTotalLOC(fileNode)).toBe(150);
    });

    it('should return 0 for file node without metrics', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts'
      };

      expect(calculateTotalLOC(fileNode)).toBe(0);
    });

    it('should return 0 for file node with undefined LOC', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { complexity: 5 }
      };

      expect(calculateTotalLOC(fileNode)).toBe(0);
    });

    it('should return 0 for directory node without children', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src'
      };

      expect(calculateTotalLOC(dirNode)).toBe(0);
    });

    it('should sum LOC for directory with file children', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: [
          {
            name: 'file1.ts',
            type: 'file',
            path: '/src/file1.ts',
            metrics: { loc: 100, complexity: 5 }
          },
          {
            name: 'file2.ts',
            type: 'file',
            path: '/src/file2.ts',
            metrics: { loc: 200, complexity: 10 }
          }
        ]
      };

      expect(calculateTotalLOC(dirNode)).toBe(300);
    });

    it('should recursively sum LOC for nested directories', () => {
      const rootNode: HierarchyNode = {
        name: 'root',
        type: 'directory',
        path: '/root',
        children: [
          {
            name: 'src',
            type: 'directory',
            path: '/root/src',
            children: [
              {
                name: 'file1.ts',
                type: 'file',
                path: '/root/src/file1.ts',
                metrics: { loc: 100, complexity: 5 }
              }
            ]
          },
          {
            name: 'tests',
            type: 'directory',
            path: '/root/tests',
            children: [
              {
                name: 'test1.ts',
                type: 'file',
                path: '/root/tests/test1.ts',
                metrics: { loc: 50, complexity: 2 }
              }
            ]
          }
        ]
      };

      expect(calculateTotalLOC(rootNode)).toBe(150);
    });

    it('should handle empty children array', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: []
      };

      expect(calculateTotalLOC(dirNode)).toBe(0);
    });

    it('should handle mixed file and directory children', () => {
      const rootNode: HierarchyNode = {
        name: 'root',
        type: 'directory',
        path: '/root',
        children: [
          {
            name: 'file.ts',
            type: 'file',
            path: '/root/file.ts',
            metrics: { loc: 75, complexity: 3 }
          },
          {
            name: 'subdir',
            type: 'directory',
            path: '/root/subdir',
            children: [
              {
                name: 'nested.ts',
                type: 'file',
                path: '/root/subdir/nested.ts',
                metrics: { loc: 25, complexity: 1 }
              }
            ]
          }
        ]
      };

      expect(calculateTotalLOC(rootNode)).toBe(100);
    });
  });

  describe('validateHierarchyNode', () => {
    it('should validate a valid file node', () => {
      const fileNode: HierarchyNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(fileNode)).toBe(true);
    });

    it('should validate a valid directory node', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: []
      };

      expect(validateHierarchyNode(dirNode)).toBe(true);
    });

    it('should validate directory node without children property', () => {
      const dirNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src'
      };

      expect(validateHierarchyNode(dirNode)).toBe(true);
    });

    it('should reject null or undefined', () => {
      expect(validateHierarchyNode(null)).toBe(false);
      expect(validateHierarchyNode(undefined)).toBe(false);
    });

    it('should reject non-object values', () => {
      expect(validateHierarchyNode('string')).toBe(false);
      expect(validateHierarchyNode(123)).toBe(false);
      expect(validateHierarchyNode(true)).toBe(false);
    });

    it('should reject node without name', () => {
      const invalidNode = {
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject node with non-string name', () => {
      const invalidNode = {
        name: 123,
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject node without type', () => {
      const invalidNode = {
        name: 'test.ts',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject node with invalid type', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'invalid',
        path: '/test.ts',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject node without path', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'file',
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject node with non-string path', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'file',
        path: 123,
        metrics: { loc: 100, complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject file node without metrics', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts'
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject file node with non-number LOC', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: '100', complexity: 5 }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should reject file node with non-number complexity', () => {
      const invalidNode = {
        name: 'test.ts',
        type: 'file',
        path: '/test.ts',
        metrics: { loc: 100, complexity: '5' }
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should validate directory with valid children', () => {
      const validNode: HierarchyNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: [
          {
            name: 'file.ts',
            type: 'file',
            path: '/src/file.ts',
            metrics: { loc: 100, complexity: 5 }
          }
        ]
      };

      expect(validateHierarchyNode(validNode)).toBe(true);
    });

    it('should reject directory with invalid children', () => {
      const invalidNode = {
        name: 'src',
        type: 'directory',
        path: '/src',
        children: [
          {
            name: 'invalid',
            type: 'file',
            path: '/src/invalid'
            // Missing required metrics
          }
        ]
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });

    it('should recursively validate nested children', () => {
      const validNode: HierarchyNode = {
        name: 'root',
        type: 'directory',
        path: '/root',
        children: [
          {
            name: 'src',
            type: 'directory',
            path: '/root/src',
            children: [
              {
                name: 'file.ts',
                type: 'file',
                path: '/root/src/file.ts',
                metrics: { loc: 100, complexity: 5 }
              }
            ]
          }
        ]
      };

      expect(validateHierarchyNode(validNode)).toBe(true);
    });

    it('should reject if any nested child is invalid', () => {
      const invalidNode = {
        name: 'root',
        type: 'directory',
        path: '/root',
        children: [
          {
            name: 'src',
            type: 'directory',
            path: '/root/src',
            children: [
              {
                name: 'invalid',
                type: 'file',
                path: '/root/src/invalid'
                // Missing metrics
              }
            ]
          }
        ]
      };

      expect(validateHierarchyNode(invalidNode)).toBe(false);
    });
  });
});
