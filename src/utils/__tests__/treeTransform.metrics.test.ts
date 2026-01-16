/**
 * Tree Transform Metrics Tests
 *
 * Tests metric aggregation, rollup calculations, and edge cases
 */

import { describe, it, expect } from 'vitest';
import {
  getTotalLOC,
  getFileCount,
} from '../treeTransform';
import { createMockTreeNode } from './test-utils';

describe('treeTransform - Metrics', () => {
  describe('getTotalLOC', () => {
    it('should return LOC for file node', () => {
      const node = createMockTreeNode({ loc: 500 });
      expect(getTotalLOC(node)).toBe(500);
    });

    it('should return total LOC for directory (already aggregated)', () => {
      const node = createMockTreeNode({
        type: 'directory',
        loc: 1000,
        children: [
          createMockTreeNode({ loc: 400 }),
          createMockTreeNode({ loc: 600 }),
        ],
      });

      expect(getTotalLOC(node)).toBe(1000);
    });

    it('should return 0 for null node', () => {
      expect(getTotalLOC(null)).toBe(0);
    });

    it('should return 0 for undefined node', () => {
      expect(getTotalLOC(undefined )).toBe(0);
    });

    it('should handle zero LOC', () => {
      const node = createMockTreeNode({ loc: 0 });
      expect(getTotalLOC(node)).toBe(0);
    });
  });

  describe('getFileCount', () => {
    it('should return 1 for file node', () => {
      const node = createMockTreeNode({ type: 'file' });
      expect(getFileCount(node)).toBe(1);
    });

    it('should return 0 for empty directory', () => {
      const node = createMockTreeNode({ type: 'directory', children: [] });
      expect(getFileCount(node)).toBe(0);
    });

    it('should count all files in directory', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [
          createMockTreeNode({ type: 'file' }),
          createMockTreeNode({ type: 'file' }),
          createMockTreeNode({ type: 'file' }),
        ],
      });

      expect(getFileCount(node)).toBe(3);
    });

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
      });

      expect(getFileCount(node)).toBe(4);
    });

    it('should not count directories', () => {
      const node = createMockTreeNode({
        type: 'directory',
        children: [
          createMockTreeNode({ type: 'directory', children: [] }),
          createMockTreeNode({ type: 'directory', children: [] }),
          createMockTreeNode({ type: 'file' }),
        ],
      });

      expect(getFileCount(node)).toBe(1);
    });

    it('should return 0 for null node', () => {
      expect(getFileCount(null)).toBe(0);
    });

    it('should return 0 for undefined node', () => {
      expect(getFileCount(undefined )).toBe(0);
    });
  });
});
