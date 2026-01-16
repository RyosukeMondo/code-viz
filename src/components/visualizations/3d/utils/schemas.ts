/**
 * Schema utilities for hierarchy node validation
 * @module components/visualizations/3d/utils/schemas
 */

import type { HierarchyNode, FileMetrics } from '../types';

/**
 * Check if a node is a file node
 */
export function isFileNode(node: HierarchyNode): boolean {
  return node.type === 'file';
}

/**
 * Check if a node is a directory node
 */
export function isDirectoryNode(node: HierarchyNode): boolean {
  return node.type === 'directory';
}

/**
 * Calculate total LOC for a hierarchy node and its children
 */
export function calculateTotalLOC(node: HierarchyNode): number {
  if (isFileNode(node)) {
    return node.metrics?.loc || 0;
  }

  if (!node.children) {
    return 0;
  }

  return node.children.reduce((total, child) => {
    return total + calculateTotalLOC(child);
  }, 0);
}

/**
 * Validate a hierarchy node structure
 */
export function validateHierarchyNode(node: unknown): node is HierarchyNode {
  if (!node || typeof node !== 'object') {
    return false;
  }

  const n = node as HierarchyNode;

  if (!n.name || typeof n.name !== 'string') return false;
  if (!n.type || (n.type !== 'file' && n.type !== 'directory')) return false;
  if (!n.path || typeof n.path !== 'string') return false;

  if (n.type === 'file') {
    if (!n.metrics) return false;
    if (typeof n.metrics.loc !== 'number') return false;
    if (typeof n.metrics.complexity !== 'number') return false;
  }

  if (n.children && Array.isArray(n.children)) {
    return n.children.every(child => validateHierarchyNode(child));
  }

  return true;
}
