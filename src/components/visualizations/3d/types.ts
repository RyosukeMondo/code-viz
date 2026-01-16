/**
 * Type definitions for 3D visualization
 * @module components/visualizations/3d/types
 */

import type * as THREE from 'three';

/**
 * File metrics from analysis
 */
export interface FileMetrics {
  loc: number;
  complexity: number;
  functions: number;
  lastModified: string;
  churn?: number;
}

/**
 * Hierarchy node representing a file or directory
 */
export interface HierarchyNode {
  name: string;
  type: 'file' | 'directory';
  path: string;
  children?: HierarchyNode[];
  metrics?: FileMetrics;
}

/**
 * Layout node with 2D position and height calculated from treemap
 */
export interface LayoutNode {
  id: string;
  name: string;
  path: string;
  x0: number;
  x1: number;
  y0: number;
  y1: number;
  height: number;
  color: string;
  metrics: FileMetrics;
}

/**
 * Voxel instance mapping data
 */
export interface VoxelMapping {
  nodeId: string;
  node: LayoutNode;
  voxelLevel: number;
}

/**
 * Scene manager options
 */
export interface SceneManagerOptions {
  antialias?: boolean;
  shadowsEnabled?: boolean;
  targetFPS?: number;
}

/**
 * Voxel renderer options
 */
export interface VoxelRendererOptions {
  voxelSize?: number;
  maxHeight?: number;
  maxVoxels?: number;
}

/**
 * Rendering statistics
 */
export interface RenderStats {
  totalBuildings: number;
  totalVoxels: number;
  instancedMeshCount: number;
  voxelSize: number;
  maxHeight: number;
  memoryEstimate: number;
}

/**
 * Layout statistics
 */
export interface LayoutStats {
  nodeCount: number;
  totalArea: number;
  averageArea: number;
  minArea: number;
  maxArea: number;
  averageAspectRatio: number;
  coveragePercent?: number;
}

/**
 * Complexity thresholds configuration
 */
export interface ComplexityThresholds {
  LOW: number;
  MEDIUM: number;
  HIGH: number;
  VERY_HIGH?: number;
}

/**
 * Color legend entry for UI display
 */
export interface ColorLegendEntry {
  label: string;
  color: string;
  range: string;
}

/**
 * Camera state for persistence
 */
export interface CameraState {
  position: { x: number; y: number; z: number };
  target: { x: number; y: number; z: number };
  timestamp: number;
}

/**
 * Selection state
 */
export interface SelectionState {
  selectedNode: LayoutNode | null;
  hoveredNode: LayoutNode | null;
}
