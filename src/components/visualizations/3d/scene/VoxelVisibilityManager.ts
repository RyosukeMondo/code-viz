/**
 * VoxelVisibilityManager - Manages visibility, frustum culling, and LOD for voxel rendering
 * Handles dynamic visibility updates for performance optimization
 * @module components/visualizations/3d/scene/VoxelVisibilityManager
 */

import * as THREE from 'three';
import type { LayoutNode } from '../types';
import type { VoxelOptimizer } from './VoxelOptimizer';

/**
 * Tracks instance range for a building
 */
interface BuildingInstanceRange {
  start: number;
  count: number;
}

/**
 * VoxelVisibilityManager handles frustum culling and LOD for voxel instances
 */
export class VoxelVisibilityManager {
  private buildingInstanceRanges: Map<string, BuildingInstanceRange> = new Map();
  private lastCameraPosition = new THREE.Vector3();
  private lastCameraUpdate = 0;
  private frustumCullingEnabled = true;
  private lodEnabled = true;
  private voxelSize: number;

  private _tempMatrix = new THREE.Matrix4();
  private _zeroScale = new THREE.Vector3(0, 0, 0);

  constructor(voxelSize: number) {
    this.voxelSize = voxelSize;
  }

  /**
   * Registers a building's instance range
   * @param nodeId - Node ID
   * @param start - Starting instance ID
   * @param count - Number of instances
   */
  registerBuilding(nodeId: string, start: number, count: number): void {
    this.buildingInstanceRanges.set(nodeId, { start, count });
  }

  /**
   * Gets the instance range for a building
   * @param nodeId - Node ID
   * @returns Instance range or undefined
   */
  getBuildingRange(nodeId: string): BuildingInstanceRange | undefined {
    return this.buildingInstanceRanges.get(nodeId);
  }

  /**
   * Clears all registered buildings
   */
  clear(): void {
    this.buildingInstanceRanges.clear();
    this.lastCameraUpdate = 0;
  }

  /**
   * Enables or disables frustum culling
   * @param enabled - Whether frustum culling should be enabled
   */
  setFrustumCullingEnabled(enabled: boolean): void {
    this.frustumCullingEnabled = enabled;
  }

  /**
   * Enables or disables LOD system
   * @param enabled - Whether LOD should be enabled
   */
  setLODEnabled(enabled: boolean): void {
    this.lodEnabled = enabled;
  }

  /**
   * Updates visibility of buildings based on frustum culling and LOD
   * Throttled to avoid excessive calculations
   * @param camera - Camera to check visibility from
   * @param deltaTime - Time since last frame
   * @param layoutNodes - Layout nodes to check
   * @param instancedMesh - Instanced mesh to update
   * @param optimizer - VoxelOptimizer for culling checks
   */
  updateVisibility(
    camera: THREE.Camera,
    deltaTime: number,
    layoutNodes: LayoutNode[],
    instancedMesh: THREE.InstancedMesh,
    optimizer: VoxelOptimizer
  ): void {
    // Throttle visibility updates based on camera movement
    const cameraMoved = this.lastCameraPosition.distanceToSquared(camera.position) > 1.0;
    this.lastCameraUpdate += deltaTime;

    // Only update if camera moved significantly or 0.5 seconds passed
    if (!cameraMoved && this.lastCameraUpdate < 0.5) return;

    this.lastCameraUpdate = 0;
    this.lastCameraPosition.copy(camera.position);

    // Update frustum for culling checks
    optimizer.updateFrustum(camera);

    let culledCount = 0;
    let lodReducedCount = 0;
    let needsMatrixUpdate = false;

    // Check each building's visibility
    for (const node of layoutNodes) {
      const range = this.buildingInstanceRanges.get(node.id);
      if (!range) continue;

      // Check frustum culling
      let isVisible = true;
      if (this.frustumCullingEnabled) {
        isVisible = optimizer.isInFrustum(node);
      }

      if (!isVisible) {
        // Hide all instances of this building by setting scale to 0
        this.hideBuilding(instancedMesh, range.start, range.count);
        culledCount++;
        needsMatrixUpdate = true;
        continue;
      }

      // Check LOD for distant buildings
      if (this.lodEnabled) {
        const isFar = optimizer.isBeyondLODDistance(node, camera);
        if (isFar) {
          // Reduce detail for distant buildings - show only bottom half of voxels
          this.applyLODToBuilding(instancedMesh, node, range.start, range.count);
          lodReducedCount++;
          needsMatrixUpdate = true;
        } else {
          // Restore full detail
          this.restoreFullDetailToBuilding(instancedMesh, node, range.start, range.count);
          needsMatrixUpdate = true;
        }
      }
    }

    if (needsMatrixUpdate) {
      instancedMesh.instanceMatrix.needsUpdate = true;
    }

    // Log culling stats occasionally
    if (culledCount > 0 || lodReducedCount > 0) {
      console.debug(
        `VoxelVisibilityManager: Culled ${culledCount} buildings, ` +
        `LOD reduced ${lodReducedCount} buildings`
      );
    }
  }

  /**
   * Hides a range of instances by setting their scale to zero
   * @param instancedMesh - Instanced mesh
   * @param start - Starting instance ID
   * @param count - Number of instances to hide
   */
  private hideBuilding(instancedMesh: THREE.InstancedMesh, start: number, count: number): void {
    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();

    for (let i = 0; i < count; i++) {
      const instanceId = start + i;
      instancedMesh.getMatrixAt(instanceId, this._tempMatrix);
      this._tempMatrix.decompose(position, quaternion, this._zeroScale);

      // Set scale to zero to hide
      this._tempMatrix.compose(position, quaternion, this._zeroScale);
      instancedMesh.setMatrixAt(instanceId, this._tempMatrix);
    }
  }

  /**
   * Applies LOD to a building by showing only bottom half of voxels
   * @param instancedMesh - Instanced mesh
   * @param node - Layout node
   * @param start - Starting instance ID
   * @param count - Number of instances
   */
  private applyLODToBuilding(
    instancedMesh: THREE.InstancedMesh,
    node: LayoutNode,
    start: number,
    count: number
  ): void {
    const buildingWidth = node.x1 - node.x0;
    const buildingDepth = node.y1 - node.y0;
    const gapFactor = 0.94;

    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();

    // Show only bottom half for LOD
    const visibleCount = Math.ceil(count / 2);

    for (let i = 0; i < count; i++) {
      const instanceId = start + i;
      instancedMesh.getMatrixAt(instanceId, this._tempMatrix);
      this._tempMatrix.decompose(position, quaternion, scale);

      if (i < visibleCount) {
        // Keep bottom voxels visible with original scale
        scale.set(
          buildingWidth * gapFactor,
          this.voxelSize * gapFactor,
          buildingDepth * gapFactor
        );
      } else {
        // Hide top voxels
        scale.copy(this._zeroScale);
      }

      this._tempMatrix.compose(position, quaternion, scale);
      instancedMesh.setMatrixAt(instanceId, this._tempMatrix);
    }
  }

  /**
   * Restores full detail to a building
   * @param instancedMesh - Instanced mesh
   * @param node - Layout node
   * @param start - Starting instance ID
   * @param count - Number of instances
   */
  private restoreFullDetailToBuilding(
    instancedMesh: THREE.InstancedMesh,
    node: LayoutNode,
    start: number,
    count: number
  ): void {
    const buildingWidth = node.x1 - node.x0;
    const buildingDepth = node.y1 - node.y0;
    const gapFactor = 0.94;

    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();

    for (let i = 0; i < count; i++) {
      const instanceId = start + i;
      instancedMesh.getMatrixAt(instanceId, this._tempMatrix);
      this._tempMatrix.decompose(position, quaternion, scale);

      // Restore full scale
      scale.set(
        buildingWidth * gapFactor,
        this.voxelSize * gapFactor,
        buildingDepth * gapFactor
      );

      this._tempMatrix.compose(position, quaternion, scale);
      instancedMesh.setMatrixAt(instanceId, this._tempMatrix);
    }
  }
}
