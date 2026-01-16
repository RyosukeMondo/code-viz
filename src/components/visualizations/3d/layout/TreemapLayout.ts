/**
 * TreemapLayout - Computes 2D spatial layout for voxel world using D3 treemap
 * @module components/visualizations/3d/layout/TreemapLayout
 */

import * as d3 from 'd3-hierarchy';
import { isFileNode } from '../utils/schemas';
import type { HierarchyNode, LayoutNode, LayoutStats } from '../types';

/**
 * TreemapLayout class that converts hierarchical data into 2D spatial layouts
 * Uses D3's squarified treemap algorithm for optimal aspect ratios
 */
export class TreemapLayout {
  private width: number;
  private depth: number;

  /**
   * Creates a new TreemapLayout instance
   * @param width - World width in units
   * @param depth - World depth in units
   */
  constructor(width: number, depth: number) {
    this.width = width;
    this.depth = depth;
  }

  /**
   * Computes treemap layout and returns array of LayoutNode objects
   * @param hierarchyNode - Root node from metrics data
   * @returns Array of layout nodes with 2D positions
   */
  compute(hierarchyNode: HierarchyNode): LayoutNode[] {
    if (!hierarchyNode) {
      throw new Error('TreemapLayout: hierarchyNode is required');
    }

    const root = d3.hierarchy(hierarchyNode)
      .sum((node) => {
        if (isFileNode(node)) {
          const loc = node.metrics?.loc || 0;
          return Math.max(1, loc);
        }
        return 0;
      })
      .sort((a, b) => {
        const aValue = a.value || 0;
        const bValue = b.value || 0;
        return bValue - aValue;
      });

    const treemapLayout = d3.treemap<HierarchyNode>()
      .size([this.width, this.depth])
      .tile(d3.treemapSquarify.ratio(1))
      .padding(4)
      .round(true);

    treemapLayout(root);

    const layoutNodes: LayoutNode[] = [];

    root.each((node) => {
      if (!node.children && isFileNode(node.data)) {
        const layoutNode = this._createLayoutNode(node as d3.HierarchyRectangularNode<HierarchyNode>);
        layoutNodes.push(layoutNode);
      }
    });

    return layoutNodes;
  }

  /**
   * Creates a LayoutNode from a D3 hierarchy node
   */
  private _createLayoutNode(d3Node: d3.HierarchyRectangularNode<HierarchyNode>): LayoutNode {
    const data = d3Node.data;
    const metrics = data.metrics!;

    const loc = metrics.loc || 1;
    const height = Math.log10(Math.max(1, loc)) * 10;

    const id = this._generateId(data.path);

    return {
      id,
      name: data.name,
      path: data.path,
      x0: d3Node.x0,
      x1: d3Node.x1,
      y0: d3Node.y0,
      y1: d3Node.y1,
      height,
      color: '#cccccc',
      metrics: {
        loc: metrics.loc,
        complexity: metrics.complexity,
        functions: metrics.functions,
        lastModified: metrics.lastModified,
        ...(metrics.churn && { churn: metrics.churn })
      }
    };
  }

  /**
   * Generates a unique ID from a file path
   */
  private _generateId(path: string): string {
    let hash = 0;
    for (let i = 0; i < path.length; i++) {
      const char = path.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash;
    }
    return `file-${Math.abs(hash).toString(36)}`;
  }

  /**
   * Validates the computed layout for overlaps and bounds
   * @param nodes - Layout nodes to validate
   * @returns Validation result
   */
  validateLayout(nodes: LayoutNode[]): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    for (const node of nodes) {
      if (node.x0 < 0 || node.x1 > this.width) {
        errors.push(`Node ${node.name} exceeds width bounds: [${node.x0}, ${node.x1}]`);
      }
      if (node.y0 < 0 || node.y1 > this.depth) {
        errors.push(`Node ${node.name} exceeds depth bounds: [${node.y0}, ${node.y1}]`);
      }
      if (node.x0 >= node.x1) {
        errors.push(`Node ${node.name} has invalid x coordinates: x0=${node.x0} >= x1=${node.x1}`);
      }
      if (node.y0 >= node.y1) {
        errors.push(`Node ${node.name} has invalid y coordinates: y0=${node.y0} >= y1=${node.y1}`);
      }
    }

    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const a = nodes[i];
        const b = nodes[j];

        const xOverlap = !(a.x1 <= b.x0 || b.x1 <= a.x0);
        const yOverlap = !(a.y1 <= b.y0 || b.y1 <= a.y0);

        if (xOverlap && yOverlap) {
          errors.push(`Overlap detected between ${a.name} and ${b.name}`);
        }
      }
    }

    return {
      valid: errors.length === 0,
      errors
    };
  }

  /**
   * Computes statistics about the layout
   * @param nodes - Layout nodes
   * @returns Layout statistics
   */
  computeStats(nodes: LayoutNode[]): LayoutStats {
    if (nodes.length === 0) {
      return {
        nodeCount: 0,
        totalArea: 0,
        averageArea: 0,
        minArea: 0,
        maxArea: 0,
        averageAspectRatio: 0
      };
    }

    let totalArea = 0;
    let minArea = Infinity;
    let maxArea = -Infinity;
    let totalAspectRatio = 0;

    for (const node of nodes) {
      const width = node.x1 - node.x0;
      const height = node.y1 - node.y0;
      const area = width * height;
      const aspectRatio = Math.max(width, height) / Math.min(width, height);

      totalArea += area;
      minArea = Math.min(minArea, area);
      maxArea = Math.max(maxArea, area);
      totalAspectRatio += aspectRatio;
    }

    return {
      nodeCount: nodes.length,
      totalArea,
      averageArea: totalArea / nodes.length,
      minArea,
      maxArea,
      averageAspectRatio: totalAspectRatio / nodes.length,
      coveragePercent: (totalArea / (this.width * this.depth)) * 100
    };
  }
}

/**
 * Helper function to create a TreemapLayout and compute in one call
 * @param hierarchyNode - Root node
 * @param worldWidth - World width
 * @param worldDepth - World depth
 * @returns Layout nodes
 */
export function computeTreemapLayout(
  hierarchyNode: HierarchyNode,
  worldWidth = 1000,
  worldDepth = 1000
): LayoutNode[] {
  const layout = new TreemapLayout(worldWidth, worldDepth);
  return layout.compute(hierarchyNode);
}
