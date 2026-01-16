/**
 * VoxelOptimizer - Performance optimization for voxel rendering
 * Handles frustum culling, LOD, and adaptive quality
 * @module components/visualizations/3d/scene/VoxelOptimizer
 */

import * as THREE from 'three';
import { PERFORMANCE_THRESHOLDS } from '../utils/constants';
import type { LayoutNode } from '../types';

/**
 * VoxelOptimizer class handles performance optimizations for voxel rendering
 * including frustum culling, LOD system, and adaptive quality
 */
export class VoxelOptimizer {
  private frustum = new THREE.Frustum();
  private projScreenMatrix = new THREE.Matrix4();
  private lodEnabled = true;
  private lodDistance: number = PERFORMANCE_THRESHOLDS.LOD_DISTANCE;
  private adaptiveQualityEnabled = true;
  private currentQualityLevel = 1.0; // 1.0 = full quality, 0.5 = reduced
  private fpsHistory: number[] = [];
  private lastFpsCheck = 0;
  private centerOffsetX = 0;
  private centerOffsetZ = 0;
  private maxHeight: number;

  /**
   * Creates a VoxelOptimizer instance
   * @param maxHeight - Maximum building height for calculations
   */
  constructor(maxHeight: number) {
    this.maxHeight = maxHeight;
  }

  /**
   * Sets center offsets for position calculations
   * @param offsetX - X-axis center offset
   * @param offsetZ - Z-axis center offset
   */
  setCenterOffsets(offsetX: number, offsetZ: number): void {
    this.centerOffsetX = offsetX;
    this.centerOffsetZ = offsetZ;
  }

  /**
   * Updates frustum for culling calculations
   * Call this before culling checks when camera changes
   * @param camera - Camera to update frustum from
   */
  updateFrustum(camera: THREE.Camera): void {
    this.projScreenMatrix.multiplyMatrices(
      camera.projectionMatrix,
      camera.matrixWorldInverse
    );
    this.frustum.setFromProjectionMatrix(this.projScreenMatrix);
  }

  /**
   * Checks if a building is in camera frustum
   * @param node - Layout node to check
   * @returns true if building is visible
   */
  isInFrustum(node: LayoutNode): boolean {
    const centerX = (node.x0 + node.x1) / 2 - this.centerOffsetX;
    const centerZ = (node.y0 + node.y1) / 2 - this.centerOffsetZ;
    const buildingHeight = Math.min(node.height, this.maxHeight);
    const buildingWidth = node.x1 - node.x0;
    const buildingDepth = node.y1 - node.y0;

    const box = new THREE.Box3(
      new THREE.Vector3(
        centerX - buildingWidth / 2,
        0,
        centerZ - buildingDepth / 2
      ),
      new THREE.Vector3(
        centerX + buildingWidth / 2,
        buildingHeight,
        centerZ + buildingDepth / 2
      )
    );

    return this.frustum.intersectsBox(box);
  }

  /**
   * Calculates distance from camera to a building
   * @param node - Layout node
   * @param camera - Camera to measure distance from
   * @returns Distance in world units
   */
  getDistanceToCamera(node: LayoutNode, camera: THREE.Camera): number {
    const centerX = (node.x0 + node.x1) / 2 - this.centerOffsetX;
    const centerZ = (node.y0 + node.y1) / 2 - this.centerOffsetZ;
    const buildingHeight = Math.min(node.height, this.maxHeight);

    const buildingCenter = new THREE.Vector3(
      centerX,
      buildingHeight / 2,
      centerZ
    );

    return camera.position.distanceTo(buildingCenter);
  }

  /**
   * Checks if a building is beyond LOD distance
   * @param node - Layout node
   * @param camera - Camera to check distance from
   * @returns true if building is beyond LOD distance
   */
  isBeyondLODDistance(node: LayoutNode, camera: THREE.Camera): boolean {
    if (!this.lodEnabled) return false;
    return this.getDistanceToCamera(node, camera) > this.lodDistance;
  }

  /**
   * Updates adaptive quality based on FPS performance
   * @param currentFps - Current frames per second
   * @param deltaTime - Time since last update in seconds
   * @returns true if quality level changed
   */
  updateAdaptiveQuality(currentFps: number, deltaTime: number): boolean {
    if (!this.adaptiveQualityEnabled) return false;

    // Track FPS history (last 60 frames)
    this.fpsHistory.push(currentFps);
    if (this.fpsHistory.length > 60) {
      this.fpsHistory.shift();
    }

    // Check FPS every second
    this.lastFpsCheck += deltaTime;
    if (this.lastFpsCheck < 1.0) return false;
    this.lastFpsCheck = 0;

    // Calculate average FPS
    const avgFps = this.fpsHistory.reduce((a, b) => a + b, 0) / this.fpsHistory.length;
    const previousQuality = this.currentQualityLevel;

    // Adjust quality level based on performance
    if (avgFps < PERFORMANCE_THRESHOLDS.MIN_FPS) {
      // Performance is poor, reduce quality
      this.currentQualityLevel = Math.max(0.5, this.currentQualityLevel - 0.1);
      console.warn(`VoxelOptimizer: FPS below target (${avgFps.toFixed(1)}), reducing quality to ${(this.currentQualityLevel * 100).toFixed(0)}%`);
    } else if (avgFps > PERFORMANCE_THRESHOLDS.TARGET_FPS && this.currentQualityLevel < 1.0) {
      // Performance is good, increase quality back up
      this.currentQualityLevel = Math.min(1.0, this.currentQualityLevel + 0.05);
      console.log(`VoxelOptimizer: Performance good (${avgFps.toFixed(1)} fps), increasing quality to ${(this.currentQualityLevel * 100).toFixed(0)}%`);
    }

    return this.currentQualityLevel !== previousQuality;
  }

  /**
   * Applies quality settings to a material
   * @param material - Material to apply quality settings to
   */
  applyQualityToMaterial(material: THREE.MeshStandardMaterial): void {
    // At lower quality, reduce material complexity
    if (this.currentQualityLevel < 0.7) {
      material.flatShading = true;
      material.roughness = 1.0;
    } else {
      material.flatShading = false;
      material.roughness = 0.8;
    }
    material.needsUpdate = true;
  }

  /**
   * Enables or disables LOD system
   * @param enabled - Whether LOD should be enabled
   */
  setLODEnabled(enabled: boolean): void {
    this.lodEnabled = enabled;
  }

  /**
   * Sets LOD distance threshold
   * @param distance - Distance at which LOD kicks in
   */
  setLODDistance(distance: number): void {
    this.lodDistance = Math.max(50, distance);
  }

  /**
   * Enables or disables adaptive quality
   * @param enabled - Whether adaptive quality should be enabled
   */
  setAdaptiveQuality(enabled: boolean): void {
    this.adaptiveQualityEnabled = enabled;
    if (!enabled) {
      // Reset to full quality
      this.currentQualityLevel = 1.0;
    }
  }

  /**
   * Gets current quality level
   * @returns Quality level from 0.5 to 1.0
   */
  getQualityLevel(): number {
    return this.currentQualityLevel;
  }

  /**
   * Resets optimizer state
   */
  reset(): void {
    this.fpsHistory = [];
    this.lastFpsCheck = 0;
    this.currentQualityLevel = 1.0;
  }
}
