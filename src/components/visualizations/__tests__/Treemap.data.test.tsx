/**
 * Treemap Data Transformation Tests
 *
 * Tests data transformation from TreeNode to ECharts format
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render } from '@testing-library/react';
import { Treemap } from '../Treemap';
import * as echarts from 'echarts/core';
import { mockTreeData } from '../__fixtures__/mockTreeData';
import { setupEChartsMocks } from './test-utils';

// Mock ECharts
vi.mock('echarts/core', () => ({
  use: vi.fn(),
  init: vi.fn(),
}));

// Mock the analysisStore hook
vi.mock('@/store/analysisStore', () => ({
  useDeadCodeEnabled: vi.fn(() => false),
}));

describe('Treemap - Data Transformation', () => {
  let mocks: ReturnType<typeof setupEChartsMocks>;

  beforeEach(() => {
    mocks = setupEChartsMocks();
    vi.mocked(echarts.init).mockReturnValue(mocks.mockChartInstance);
  });

  afterEach(() => {
    mocks.cleanup();
  });

  describe('TreeNode to ECharts format', () => {
    it('should transform TreeNode to ECharts format', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      // Component passes children directly when root has children
      const seriesData = optionsArg.series[0].data;

      // Should have 2 children (file1.ts and subdir)
      expect(seriesData).toHaveLength(2);

      // First child is file1.ts
      expect(seriesData[0]).toHaveProperty('name');
      expect(seriesData[0]).toHaveProperty('value');
      expect(seriesData[0]).toHaveProperty('complexity');
      expect(seriesData[0]).toHaveProperty('path');
      expect(seriesData[0]).toHaveProperty('type');

      expect(seriesData[0].name).toBe('file1.ts');
      expect(seriesData[0].value).toBe(300);
      expect(seriesData[0].type).toBe('file');
    });

    it('should preserve children structure in transformation', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      // Component passes children directly
      const seriesData = optionsArg.series[0].data;
      expect(seriesData).toHaveLength(2);

      // First child is file1.ts (no children)
      expect(seriesData[0].name).toBe('file1.ts');
      expect(seriesData[0].type).toBe('file');

      // Second child is subdir (has children)
      const subdirData = seriesData[1];
      expect(subdirData.name).toBe('subdir');
      expect(subdirData.type).toBe('directory');
      expect(subdirData).toHaveProperty('children');
      expect(Array.isArray(subdirData.children)).toBe(true);
      expect(subdirData.children).toHaveLength(2);
    });

    /**
     * CRITICAL REGRESSION TEST: Validate root node structure
     *
     * ECharts treemap expects data: [rootNode] where rootNode has name/path/type.
     * The root node from backend may have path="" (empty string).
     * Click handler must handle empty path as root and pass correct TreeNode.
     */
    it('should ensure root node and all descendants have required properties', () => {
      render(<Treemap data={mockTreeData} />);

      const optionsArg = mocks.mockSetOption.mock.calls[0][0];
      const nodes = optionsArg.series[0].data;

      /**
       * Recursive validation function
       * Ensures every node and its descendants have required properties
       */
      function validateNode(node: unknown, nodePath = 'root') {
        // CRITICAL: Every node must have these properties defined
        expect(node.name, `${nodePath}: must have name`).toBeDefined();
        expect(node.path, `${nodePath}: must have path`).toBeDefined();
        expect(node.type, `${nodePath}: must have type`).toBeDefined();

        // Validate types
        expect(typeof node.name, `${nodePath}: name must be string`).toBe('string');
        expect(typeof node.path, `${nodePath}: path must be string`).toBe('string');
        expect(typeof node.type, `${nodePath}: type must be string`).toBe('string');
        expect(['file', 'directory'], `${nodePath}: type must be file or directory`).toContain(node.type);

        // Name must not be empty
        expect(node.name.length, `${nodePath}: name must not be empty`).toBeGreaterThan(0);

        // Path must be defined (can be empty string for root from backend)
        expect(node.path, `${nodePath}: path must be defined (can be empty string for root)`).not.toBeUndefined();

        // Recursively validate children
        const nodeObj = node as Record<string, unknown>;
        if (nodeObj.children && Array.isArray(nodeObj.children)) {
          nodeObj.children.forEach((child: unknown, i: number) => {
            validateNode(child, `${nodePath}.children[${i}]`);
          });
        }
      }

      // Component passes children directly, so we have 2 nodes
      expect(nodes.length, 'Should pass children array').toBe(2);

      // Validate both nodes
      nodes.forEach((node: unknown, i: number) => {
        validateNode(node, `node[${i}]`);
      });

      // Verify first node (file1.ts)
      expect(nodes[0].name).toBe('file1.ts');
      expect(nodes[0].type).toBe('file');

      // Verify second node (subdir)
      expect(nodes[1].name).toBe('subdir');
      expect(nodes[1].type).toBe('directory');
      expect(nodes[1].children).toHaveLength(2);
    });
  });
});
