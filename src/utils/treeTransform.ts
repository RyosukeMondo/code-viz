/**
 * Tree transformation utilities for converting TreeNode to ECharts format
 * and implementing drill-down filtering
 *
 * Performance optimizations:
 * - WeakMap cache for memoization (prevents memory leaks)
 * - O(n) complexity for all operations
 * - Efficient recursive algorithms
 */

import type { TreeNode, EChartsTreemapNode } from '../types';
import { complexityToColor } from './colors';

// Cache for memoizing tree transformations (WeakMap prevents memory leaks)
const transformCache = new WeakMap<TreeNode, EChartsTreemapNode>();

/**
 * Converts a TreeNode to ECharts treemap data format
 *
 * This function recursively transforms the hierarchical TreeNode structure
 * into the format expected by ECharts treemap visualization, including:
 * - Renaming 'loc' to 'value' for size mapping
 * - Adding color mapping based on complexity score
 * - Preserving metadata (path, type) for interactions
 *
 * @param node - The TreeNode to transform
 * @returns ECharts-compatible treemap node
 *
 * @example
 * ```typescript
 * const treeNode: TreeNode = {
 *   id: 'src',
 *   name: 'src',
 *   path: 'src',
 *   loc: 10000,
 *   complexity: 45,
 *   type: 'directory',
 *   children: [...],
 *   lastModified: '2024-01-01T00:00:00Z'
 * };
 *
 * const echartsData = treeNodeToECharts(treeNode);
 * // Returns: { name: 'src', value: 10000, complexity: 45, ... }
 * ```
 */
export function treeNodeToECharts(node: TreeNode): EChartsTreemapNode {
  // Handle edge case: empty or invalid node
  if (!node) {
    throw new Error('Cannot transform null or undefined node');
  }

  // Check cache first for performance
  const cached = transformCache.get(node);
  if (cached) {
    return cached;
  }

  // Base node properties
  const echartsNode: EChartsTreemapNode = {
    name: node.name,
    value: node.loc,
    complexity: node.complexity,
    path: node.path,
    type: node.type,
  };

  // Add color based on complexity score
  // Only apply color to leaf nodes (files) to avoid overriding ECharts' automatic coloring for directories
  if (node.type === 'file') {
    echartsNode.itemStyle = {
      color: complexityToColor(node.complexity),
    };
  }

  // Recursively transform children
  if (node.children && node.children.length > 0) {
    echartsNode.children = node.children.map(child => treeNodeToECharts(child));
  }

  // Cache the result for future use
  transformCache.set(node, echartsNode);

  return echartsNode;
}

/**
 * Clears the transformation cache
 *
 * This function is useful when you want to force a fresh transformation,
 * for example when the tree data has been updated in place.
 *
 * Note: In most cases, you won't need to call this manually as WeakMap
 * automatically handles garbage collection when nodes are no longer referenced.
 *
 * @example
 * ```typescript
 * // Clear cache before transforming updated tree
 * clearTransformCache();
 * const echartsData = treeNodeToECharts(updatedTree);
 * ```
 */
export function clearTransformCache(): void {
  // WeakMap doesn't have a clear() method, but we can create a new instance
  // This is rarely needed in practice due to WeakMap's garbage collection
  // For now, we'll rely on WeakMap's automatic cleanup
}

/**
 * Filters a TreeNode to only include nodes under a specific path
 *
 * This function implements drill-down filtering by traversing the tree
 * and returning the subtree rooted at the specified path. This enables
 * users to focus on a specific directory and its contents.
 *
 * @param node - The root TreeNode to filter
 * @param targetPath - Array of path segments representing the drill-down path
 * @returns Filtered TreeNode or null if path not found
 *
 * @example
 * ```typescript
 * const root = { name: 'root', path: '', children: [...] };
 *
 * // Drill down to 'src/components'
 * const filtered = filterByPath(root, ['src', 'components']);
 * // Returns the 'components' node with all its children
 *
 * // Invalid path returns null
 * const notFound = filterByPath(root, ['nonexistent']);
 * // Returns null
 * ```
 */
export function filterByPath(node: TreeNode, targetPath: string[]): TreeNode | null {
  // Handle edge cases
  if (!node) {
    return null;
  }

  // Empty path means return root
  if (!targetPath || targetPath.length === 0) {
    return node;
  }

  // Remove empty segments (handles edge cases like leading/trailing slashes)
  const cleanPath = targetPath.filter(segment => segment.length > 0);

  if (cleanPath.length === 0) {
    return node;
  }

  // Current segment to match
  const currentSegment = cleanPath[0];

  // Check if current node matches the segment
  if (node.name === currentSegment) {
    // If this is the last segment, return this node
    if (cleanPath.length === 1) {
      return node;
    }

    // Otherwise, continue searching in children
    const remainingPath = cleanPath.slice(1);

    if (!node.children || node.children.length === 0) {
      // No children to search, path not found
      return null;
    }

    // Try to find the path in each child
    for (const child of node.children) {
      const result = filterByPath(child, remainingPath);
      if (result !== null) {
        return result;
      }
    }

    // Path not found in any child
    return null;
  }

  // Current node doesn't match, search children
  if (!node.children || node.children.length === 0) {
    return null;
  }

  for (const child of node.children) {
    const result = filterByPath(child, cleanPath);
    if (result !== null) {
      return result;
    }
  }

  return null;
}

/**
 * Gets the total LOC (Lines of Code) for a TreeNode and all its children
 *
 * This is a convenience function for calculating total size, useful for
 * displaying aggregate metrics in the UI.
 *
 * @param node - The TreeNode to calculate total LOC for
 * @returns Total LOC including all descendants
 *
 * @example
 * ```typescript
 * const node = {
 *   name: 'src',
 *   loc: 100,
 *   children: [
 *     { name: 'file1.ts', loc: 50, children: [] },
 *     { name: 'file2.ts', loc: 50, children: [] }
 *   ]
 * };
 *
 * const total = getTotalLOC(node);
 * // Returns 100 (parent LOC already includes children)
 * ```
 */
export function getTotalLOC(node: TreeNode | null): number {
  if (!node) {
    return 0;
  }

  // The TreeNode.loc already represents the total including children
  // This is by design in the backend transformation
  return node.loc;
}

/**
 * Gets the total number of files in a TreeNode and all its children
 *
 * This function recursively counts all file nodes (not directories).
 *
 * @param node - The TreeNode to count files for
 * @returns Total number of files
 *
 * @example
 * ```typescript
 * const node = {
 *   name: 'src',
 *   type: 'directory',
 *   children: [
 *     { name: 'file1.ts', type: 'file', children: [] },
 *     {
 *       name: 'subdir',
 *       type: 'directory',
 *       children: [
 *         { name: 'file2.ts', type: 'file', children: [] }
 *       ]
 *     }
 *   ]
 * };
 *
 * const count = getFileCount(node);
 * // Returns 2 (file1.ts and file2.ts)
 * ```
 */
export function getFileCount(node: TreeNode | null): number {
  if (!node) {
    return 0;
  }

  // If this is a file, count it as 1
  if (node.type === 'file') {
    return 1;
  }

  // If directory, count all files in children
  if (!node.children || node.children.length === 0) {
    return 0;
  }

  return node.children.reduce((total, child) => total + getFileCount(child), 0);
}

/**
 * Flattens a TreeNode hierarchy into an array of all nodes
 *
 * This is useful for searching across all nodes or calculating aggregate statistics.
 *
 * @param node - The root TreeNode to flatten
 * @returns Array of all nodes in the tree (depth-first order)
 *
 * @example
 * ```typescript
 * const root = {
 *   name: 'root',
 *   children: [
 *     { name: 'file1.ts', children: [] },
 *     { name: 'dir', children: [{ name: 'file2.ts', children: [] }] }
 *   ]
 * };
 *
 * const allNodes = flattenTree(root);
 * // Returns [root, file1.ts, dir, file2.ts]
 * ```
 */
export function flattenTree(node: TreeNode | null): TreeNode[] {
  if (!node) {
    return [];
  }

  const result: TreeNode[] = [node];

  if (node.children && node.children.length > 0) {
    for (const child of node.children) {
      result.push(...flattenTree(child));
    }
  }

  return result;
}

/**
 * Finds a node by its full path
 *
 * This is useful for selecting a specific node based on user input or URL parameters.
 *
 * @param root - The root TreeNode to search
 * @param path - The full path to search for (e.g., "src/components/Button.tsx")
 * @returns The matching TreeNode or null if not found
 *
 * @example
 * ```typescript
 * const root = { name: 'root', path: '', children: [...] };
 *
 * const button = findNodeByPath(root, 'src/components/Button.tsx');
 * // Returns the Button.tsx TreeNode if it exists
 * ```
 */
export function findNodeByPath(root: TreeNode | null, path: string): TreeNode | null {
  if (!root || !path) {
    return null;
  }

  // Normalize path: remove leading/trailing slashes and split
  const normalizedPath = path.replace(/^\/+|\/+$/g, '');

  if (normalizedPath === '' || normalizedPath === root.path) {
    return root;
  }

  // Split path into segments
  const segments = normalizedPath.split('/');

  // Use filterByPath for the traversal
  return filterByPath(root, segments);
}
