/**
 * VoxelRenderer - Efficient voxel rendering using GPU instancing
 * Renders 100K+ voxels at 60fps using THREE.InstancedMesh
 * Includes frustum culling, LOD system, and adaptive quality
 * @module components/visualizations/3d/scene/VoxelRenderer
 */

import * as THREE from 'three';
import { complexityToColor } from '../utils/colorMaps';
import { MAX_HEIGHT, DEFAULT_VOXEL_SIZE, PERFORMANCE_THRESHOLDS } from '../utils/constants';
import type { LayoutNode, VoxelMapping, RenderStats, VoxelRendererOptions } from '../types';
import { VoxelOptimizer } from './VoxelOptimizer';
import { VoxelVisibilityManager } from './VoxelVisibilityManager';
import { VoxelHighlighter } from './VoxelHighlighter';

/**
 * VoxelRenderer class that generates and renders voxel buildings using InstancedMesh
 * for optimal GPU performance
 */
export class VoxelRenderer {
  private scene: THREE.Scene;
  private voxelSize: number;
  private maxHeight: number;
  private maxVoxels: number;

  private layoutNodes: LayoutNode[] = [];
  private instancedMesh: THREE.InstancedMesh | null = null;
  private voxelToNodeMap: Map<number, VoxelMapping> = new Map();

  private centerOffsetX = 0;
  private centerOffsetZ = 0;

  private _tempMatrix = new THREE.Matrix4();

  // Performance optimizer and managers
  private optimizer: VoxelOptimizer;
  private visibilityManager: VoxelVisibilityManager;
  private highlighter: VoxelHighlighter;

  /**
   * Creates a new VoxelRenderer instance
   * @param scene - Three.js scene to add voxels to
   * @param options - Rendering options
   */
  constructor(scene: THREE.Scene, options: VoxelRendererOptions = {}) {
    this.scene = scene;
    this.voxelSize = options.voxelSize || DEFAULT_VOXEL_SIZE;
    this.maxHeight = options.maxHeight || MAX_HEIGHT;
    this.maxVoxels = options.maxVoxels || PERFORMANCE_THRESHOLDS.MAX_VOXELS_PER_INSTANCE;
    this.optimizer = new VoxelOptimizer(this.maxHeight);
    this.visibilityManager = new VoxelVisibilityManager(this.voxelSize);
    this.highlighter = new VoxelHighlighter();
  }

  /**
   * Renders voxel buildings from layout nodes
   * Creates a single InstancedMesh with all voxels for optimal performance
   * @param layoutNodes - Array of layout nodes from TreemapLayout
   * @param worldWidth - World width (for centering)
   * @param worldDepth - World depth (for centering)
   * @returns The created instanced mesh
   */
  render(layoutNodes: LayoutNode[], worldWidth = 1000, worldDepth = 1000): THREE.InstancedMesh | null {
    if (!layoutNodes?.length) {
      console.warn('VoxelRenderer: No layout nodes provided');
      return null;
    }

    this.layoutNodes = layoutNodes;
    this.centerOffsetX = worldWidth / 2;
    this.centerOffsetZ = worldDepth / 2;
    this.optimizer.setCenterOffsets(this.centerOffsetX, this.centerOffsetZ);

    const totalVoxels = this._calculateTotalVoxels(layoutNodes);
    if (totalVoxels > this.maxVoxels) {
      console.warn(`VoxelRenderer: Total voxels (${totalVoxels}) exceeds max (${this.maxVoxels})`);
    }

    const instanceCount = Math.min(totalVoxels, this.maxVoxels);
    this._createInstancedMesh(instanceCount);

    if (!this.instancedMesh) {
      console.error('VoxelRenderer: Failed to create instanced mesh');
      return null;
    }

    const currentInstanceId = this._renderBuildings(layoutNodes, instanceCount);
    this.instancedMesh.instanceMatrix.needsUpdate = true;
    this.instancedMesh.instanceColor!.needsUpdate = true;

    if (Array.isArray(this.instancedMesh.material)) {
      this.instancedMesh.material.forEach(mat => mat.needsUpdate = true);
    } else {
      this.instancedMesh.material.needsUpdate = true;
    }

    this.scene.add(this.instancedMesh);
    console.log(`VoxelRenderer: Created ${currentInstanceId} instances from ${layoutNodes.length} buildings`);

    return this.instancedMesh;
  }

  /**
   * Creates the instanced mesh with geometry and material
   * @param instanceCount - Number of instances to create
   * @private
   */
  private _createInstancedMesh(instanceCount: number): void {
    const geometry = new THREE.BoxGeometry(this.voxelSize, this.voxelSize, this.voxelSize);
    const posAttr = geometry.getAttribute('position');
    geometry.setAttribute('color', new THREE.Float32BufferAttribute(
      new Array(posAttr.count * 3).fill(1), 3
    ));

    const material = new THREE.MeshStandardMaterial({
      vertexColors: true, roughness: 0.8, metalness: 0.0,
      flatShading: false, envMapIntensity: 0.3
    });

    this.instancedMesh = new THREE.InstancedMesh(geometry, material, instanceCount);
    this.instancedMesh.instanceColor = new THREE.InstancedBufferAttribute(
      new Float32Array(instanceCount * 3).fill(1), 3
    );
    this.instancedMesh.castShadow = true;
    this.instancedMesh.receiveShadow = true;

    this.highlighter.setInstancedMesh(this.instancedMesh);
    this.highlighter.setVoxelToNodeMap(this.voxelToNodeMap);
  }

  /**
   * Renders buildings as voxel instances
   * @param layoutNodes - Layout nodes to render
   * @param instanceCount - Maximum instance count
   * @returns Number of instances created
   * @private
   */
  private _renderBuildings(layoutNodes: LayoutNode[], instanceCount: number): number {
    if (!this.instancedMesh) return 0;

    let currentInstanceId = 0;
    this.visibilityManager.clear();

    for (const node of layoutNodes) {
      if (currentInstanceId >= instanceCount) {
        console.warn('VoxelRenderer: Reached maximum voxel instance limit');
        break;
      }

      const buildingWidth = node.x1 - node.x0;
      const buildingDepth = node.y1 - node.y0;
      const buildingHeight = Math.min(node.height, this.maxHeight);
      const numVoxelsHeight = Math.ceil(buildingHeight / this.voxelSize);

      const complexity = node.metrics?.complexity || 0;
      const baseColor = complexityToColor(complexity);
      const gray = new THREE.Color(0.5, 0.5, 0.5);
      const color = baseColor.clone().lerp(gray, 0.15);

      const seed = (node.x0 * 1000 + node.y0) % 100;
      const buildingVariation = (seed / 100) * 0.1 - 0.05;
      color.multiplyScalar(1.0 + buildingVariation);

      const centerX = (node.x0 + node.x1) / 2 - this.centerOffsetX;
      const centerZ = (node.y0 + node.y1) / 2 - this.centerOffsetZ;

      // Track where this building's instances start
      const buildingStartInstance = currentInstanceId;

      for (let y = 0; y < numVoxelsHeight; y++) {
        if (currentInstanceId >= instanceCount) break;

        const posX = centerX;
        const posY = y * this.voxelSize + this.voxelSize / 2;
        const posZ = centerZ;

        const gapFactor = 0.94;
        const scaledWidth = buildingWidth * gapFactor;
        const scaledHeight = this.voxelSize * gapFactor;
        const scaledDepth = buildingDepth * gapFactor;

        this._tempMatrix.compose(
          new THREE.Vector3(posX, posY, posZ),
          new THREE.Quaternion(),
          new THREE.Vector3(scaledWidth, scaledHeight, scaledDepth)
        );
        this.instancedMesh.setMatrixAt(currentInstanceId, this._tempMatrix);

        const heightFactor = (y / numVoxelsHeight);
        const darkenBottom = 0.7 + (heightFactor * 0.5);
        const voxelColor = color.clone().multiplyScalar(darkenBottom);
        this.instancedMesh.setColorAt(currentInstanceId, voxelColor);

        this.voxelToNodeMap.set(currentInstanceId, {
          nodeId: node.id,
          node: node,
          voxelLevel: y
        });

        this.highlighter.getSelection().storeOriginalColor(currentInstanceId, voxelColor);
        currentInstanceId++;
      }

      // Register building instance range with visibility manager
      this.visibilityManager.registerBuilding(
        node.id,
        buildingStartInstance,
        currentInstanceId - buildingStartInstance
      );
    }

    return currentInstanceId;
  }

  /**
   * Updates animation state for smooth color transitions
   * Should be called in the animation loop for smooth effects
   * @param deltaTime - Time elapsed since last frame in seconds
   * @param camera - Camera for frustum culling and LOD
   * @param currentFps - Current FPS for adaptive quality (optional)
   */
  update(deltaTime: number, camera?: THREE.Camera, currentFps?: number): void {
    if (!this.instancedMesh) return;

    // Update adaptive quality based on FPS
    if (currentFps !== undefined) {
      const qualityChanged = this.optimizer.updateAdaptiveQuality(currentFps, deltaTime);
      if (qualityChanged) {
        this._applyQualitySettings();
      }
    }

    // Apply frustum culling and LOD if camera provided
    if (camera) {
      this.visibilityManager.updateVisibility(
        camera,
        deltaTime,
        this.layoutNodes,
        this.instancedMesh,
        this.optimizer
      );
    }

    // Update animations
    this.highlighter.updateAnimations(deltaTime);
  }

  /**
   * Updates colors of all voxels based on new complexity mapping
   * @param thresholds - Optional custom complexity thresholds
   */
  updateColors(thresholds?: { LOW: number; MEDIUM: number; HIGH: number; VERY_HIGH?: number }): void {
    if (!this.instancedMesh) {
      console.warn('VoxelRenderer: No instanced mesh to update');
      return;
    }

    let instanceId = 0;
    for (const node of this.layoutNodes) {
      const numVoxels = Math.ceil(Math.min(node.height, this.maxHeight) / this.voxelSize);
      const color = complexityToColor(node.metrics?.complexity || 0, thresholds);
      for (let y = 0; y < numVoxels && instanceId < this.instancedMesh.count; y++, instanceId++) {
        this.instancedMesh.setColorAt(instanceId, color);
      }
    }

    this.instancedMesh.instanceColor!.needsUpdate = true;
    console.log('VoxelRenderer: Updated colors for all voxel instances');
  }

  /**
   * Gets the layout node associated with a voxel instance ID
   * @param instanceId - Instance ID from raycaster intersection
   * @returns Associated layout node or null if not found
   */
  getNodeByInstanceId(instanceId: number): LayoutNode | null {
    const mapping = this.voxelToNodeMap.get(instanceId);
    return mapping ? mapping.node : null;
  }

  /**
   * Highlights a specific building by changing its voxels' colors with selection effect
   * Uses a brighter, more saturated color with emissive-like glow
   * @param nodeId - ID of the node to highlight
   * @param highlightColor - Color to use for highlighting (default: bright yellow)
   */
  highlightBuilding(nodeId: string, highlightColor: THREE.Color | string = '#ffff00'): void {
    this.highlighter.highlightBuilding(nodeId, highlightColor);
  }

  /**
   * Removes highlighting from a building by restoring original colors with smooth transition
   * @param nodeId - ID of the node to unhighlight
   */
  unhighlightBuilding(nodeId: string): void {
    this.highlighter.unhighlightBuilding(nodeId);
  }

  /**
   * Applies hover effect to a building with subtle brightness increase
   * @param nodeId - ID of the node to hover
   */
  hoverBuilding(nodeId: string): void {
    this.highlighter.hoverBuilding(nodeId);
  }

  /**
   * Removes hover effect from a building
   * @param nodeId - ID of the node to unhover
   */
  unhoverBuilding(nodeId: string): void {
    this.highlighter.unhoverBuilding(nodeId);
  }


  /**
   * Calculates total number of voxels needed for all buildings
   */
  private _calculateTotalVoxels(layoutNodes: LayoutNode[]): number {
    let total = 0;

    for (const node of layoutNodes) {
      const buildingHeight = Math.min(node.height, this.maxHeight);
      const numVoxelsHeight = Math.ceil(buildingHeight / this.voxelSize);
      total += numVoxelsHeight;
    }

    return total;
  }

  /**
   * Computes rendering statistics
   * @returns Statistics about the rendering
   */
  getStats(): RenderStats {
    return {
      totalBuildings: this.layoutNodes.length,
      totalVoxels: this.instancedMesh ? this.instancedMesh.count : 0,
      instancedMeshCount: this.instancedMesh ? 1 : 0,
      voxelSize: this.voxelSize,
      maxHeight: this.maxHeight,
      memoryEstimate: this._estimateMemoryUsage()
    };
  }

  /**
   * Estimates memory usage in bytes
   */
  private _estimateMemoryUsage(): number {
    if (!this.instancedMesh) return 0;

    const instanceCount = this.instancedMesh.count;

    const bytesPerInstance = 64 + 12;

    const geometryBytes = 24 * 3 * 4;

    return instanceCount * bytesPerInstance + geometryBytes;
  }

  /**
   * Updates frustum for culling calculations
   * Call this before culling checks when camera changes
   * @param camera - Camera to update frustum from
   */
  updateFrustum(camera: THREE.Camera): void {
    this.optimizer.updateFrustum(camera);
  }

  /**
   * Checks if a building is in camera frustum
   * @param node - Layout node to check
   * @returns true if building is visible
   */
  isInFrustum(node: LayoutNode): boolean {
    return this.optimizer.isInFrustum(node);
  }

  /**
   * Calculates distance from camera to a building
   * @param node - Layout node
   * @param camera - Camera to measure distance from
   * @returns Distance in world units
   */
  getDistanceToCamera(node: LayoutNode, camera: THREE.Camera): number {
    return this.optimizer.getDistanceToCamera(node, camera);
  }

  /**
   * Applies LOD (Level of Detail) based on camera distance
   * Reduces visual quality for distant buildings
   * @param camera - Camera to calculate distances from
   * @deprecated This method is now called automatically within update()
   */
  applyLOD(camera: THREE.Camera): void {
    // This method is now handled internally by _updateVisibility in update()
    // Kept for backward compatibility
    if (!this.instancedMesh || !camera) return;
  }

  /**
   * Applies current quality level to rendering settings
   */
  private _applyQualitySettings(): void {
    if (!this.instancedMesh) return;

    const material = this.instancedMesh.material;

    if (!Array.isArray(material) && material instanceof THREE.MeshStandardMaterial) {
      this.optimizer.applyQualityToMaterial(material);
    }
  }

  /**
   * Enables or disables frustum culling
   * @param enabled - Whether frustum culling should be enabled
   */
  setFrustumCullingEnabled(enabled: boolean): void {
    this.visibilityManager.setFrustumCullingEnabled(enabled);
  }

  /**
   * Enables or disables LOD system
   * @param enabled - Whether LOD should be enabled
   */
  setLODEnabled(enabled: boolean): void {
    this.visibilityManager.setLODEnabled(enabled);
    this.optimizer.setLODEnabled(enabled);
  }

  /**
   * Sets LOD distance threshold
   * @param distance - Distance at which LOD kicks in
   */
  setLODDistance(distance: number): void {
    this.optimizer.setLODDistance(distance);
  }

  /**
   * Enables or disables adaptive quality
   * @param enabled - Whether adaptive quality should be enabled
   */
  setAdaptiveQuality(enabled: boolean): void {
    this.optimizer.setAdaptiveQuality(enabled);
    if (!enabled) {
      this._applyQualitySettings();
    }
  }

  /**
   * Gets current quality level
   * @returns Quality level from 0.5 to 1.0
   */
  getQualityLevel(): number {
    return this.optimizer.getQualityLevel();
  }

  /**
   * Disposes of all resources
   * Call this when removing the visualization to free GPU memory
   */
  dispose(): void {
    if (!this.instancedMesh) return;

    this.scene.remove(this.instancedMesh);
    this.instancedMesh.geometry.dispose();

    if (Array.isArray(this.instancedMesh.material)) {
      this.instancedMesh.material.forEach(mat => mat.dispose());
    } else {
      this.instancedMesh.material.dispose();
    }

    this.instancedMesh = null;
    this.voxelToNodeMap.clear();
    this.layoutNodes = [];
    this.highlighter.clear();
    this.optimizer.reset();
    console.log('VoxelRenderer: Disposed all resources');
  }
}

/**
 * Helper function to quickly render voxels in one call
 * @param scene - Three.js scene
 * @param layoutNodes - Layout nodes to render
 * @param options - Rendering options
 * @returns The renderer instance
 */
export function renderVoxels(
  scene: THREE.Scene,
  layoutNodes: LayoutNode[],
  options: VoxelRendererOptions = {}
): VoxelRenderer {
  const renderer = new VoxelRenderer(scene, options);
  renderer.render(layoutNodes);
  return renderer;
}
