/**
 * VoxelRenderer - Efficient voxel rendering using GPU instancing
 * Renders 100K+ voxels at 60fps using THREE.InstancedMesh
 * @module components/visualizations/3d/scene/VoxelRenderer
 */

import * as THREE from 'three';
import { complexityToColor } from '../utils/colorMaps';
import { MAX_HEIGHT, DEFAULT_VOXEL_SIZE, PERFORMANCE_THRESHOLDS } from '../utils/constants';
import type { LayoutNode, VoxelMapping, RenderStats, VoxelRendererOptions } from '../types';

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

  // Selection state tracking
  private selectedNodeId: string | null = null;
  private hoveredNodeId: string | null = null;
  private originalColors: Map<number, THREE.Color> = new Map();

  // Animation state for smooth transitions
  private animationTargets: Map<number, { target: THREE.Color; start: THREE.Color; progress: number }> = new Map();

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
    if (!layoutNodes || layoutNodes.length === 0) {
      console.warn('VoxelRenderer: No layout nodes provided');
      return null;
    }

    this.layoutNodes = layoutNodes;

    this.centerOffsetX = worldWidth / 2;
    this.centerOffsetZ = worldDepth / 2;

    const totalVoxels = this._calculateTotalVoxels(layoutNodes);

    if (totalVoxels > this.maxVoxels) {
      console.warn(
        `VoxelRenderer: Total voxels (${totalVoxels}) exceeds max (${this.maxVoxels}). ` +
        'Some buildings will be capped.'
      );
    }

    const geometry = new THREE.BoxGeometry(this.voxelSize, this.voxelSize, this.voxelSize);

    const positionAttribute = geometry.getAttribute('position');
    const colors: number[] = [];
    for (let i = 0; i < positionAttribute.count; i++) {
      colors.push(1, 1, 1);
    }
    geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));

    const material = new THREE.MeshStandardMaterial({
      vertexColors: true,
      roughness: 0.8,
      metalness: 0.0,
      flatShading: false,
      envMapIntensity: 0.3
    });

    const instanceCount = Math.min(totalVoxels, this.maxVoxels);
    this.instancedMesh = new THREE.InstancedMesh(geometry, material, instanceCount);

    if (!this.instancedMesh.instanceColor) {
      const colors = new Float32Array(instanceCount * 3);
      for (let i = 0; i < instanceCount; i++) {
        colors[i * 3 + 0] = 1.0;
        colors[i * 3 + 1] = 1.0;
        colors[i * 3 + 2] = 1.0;
      }
      this.instancedMesh.instanceColor = new THREE.InstancedBufferAttribute(colors, 3);
      console.log('✓ Manually initialized instanceColor buffer');
    }

    this.instancedMesh.castShadow = true;
    this.instancedMesh.receiveShadow = true;

    let currentInstanceId = 0;

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

        // Store original color for restoration
        this.originalColors.set(currentInstanceId, voxelColor.clone());

        currentInstanceId++;
      }
    }

    this.instancedMesh.instanceMatrix.needsUpdate = true;
    this.instancedMesh.instanceColor!.needsUpdate = true;

    // Handle material update (may be array or single material)
    if (Array.isArray(this.instancedMesh.material)) {
      this.instancedMesh.material.forEach(mat => mat.needsUpdate = true);
    } else {
      this.instancedMesh.material.needsUpdate = true;
    }

    this.scene.add(this.instancedMesh);

    console.log(`VoxelRenderer: Created ${currentInstanceId} voxel instances from ${layoutNodes.length} buildings`);

    const testColor = new THREE.Color();
    this.instancedMesh.getColorAt(0, testColor);
    console.log('First instance color:', testColor);

    return this.instancedMesh;
  }

  /**
   * Updates animation state for smooth color transitions
   * Should be called in the animation loop for smooth effects
   * @param deltaTime - Time elapsed since last frame in seconds
   */
  update(deltaTime: number): void {
    if (!this.instancedMesh || this.animationTargets.size === 0) return;

    let needsUpdate = false;

    // Update all ongoing animations
    for (const [instanceId, animation] of this.animationTargets.entries()) {
      animation.progress += deltaTime * 5; // 5x speed for quick transitions

      if (animation.progress >= 1) {
        // Animation complete
        this.instancedMesh.setColorAt(instanceId, animation.target);
        this.animationTargets.delete(instanceId);
        needsUpdate = true;
      } else {
        // Interpolate color
        const interpolated = animation.start.clone().lerp(animation.target, animation.progress);
        this.instancedMesh.setColorAt(instanceId, interpolated);
        needsUpdate = true;
      }
    }

    if (needsUpdate) {
      this.instancedMesh.instanceColor!.needsUpdate = true;
    }
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
      const buildingHeight = Math.min(node.height, this.maxHeight);
      const numVoxelsHeight = Math.ceil(buildingHeight / this.voxelSize);

      const complexity = node.metrics?.complexity || 0;
      const color = complexityToColor(complexity, thresholds);

      for (let y = 0; y < numVoxelsHeight; y++) {
        if (instanceId >= this.instancedMesh.count) break;

        this.instancedMesh.setColorAt(instanceId, color);
        instanceId++;
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
    if (!this.instancedMesh) return;

    this.selectedNodeId = nodeId;

    const color = typeof highlightColor === 'string'
      ? new THREE.Color(highlightColor)
      : highlightColor;

    // Apply selection highlight with brightness boost
    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.originalColors.get(instanceId);
        if (originalColor) {
          // Create a highlighted color by brightening and adding the highlight color
          const highlightedColor = originalColor.clone()
            .lerp(color, 0.5)  // Mix with highlight color
            .multiplyScalar(1.5);  // Brighten

          this._animateColorTransition(instanceId, highlightedColor);
        }
      }
    }
  }

  /**
   * Removes highlighting from a building by restoring original colors with smooth transition
   * @param nodeId - ID of the node to unhighlight
   */
  unhighlightBuilding(nodeId: string): void {
    if (!this.instancedMesh) return;

    if (this.selectedNodeId === nodeId) {
      this.selectedNodeId = null;
    }

    // Restore original colors with smooth transition
    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.originalColors.get(instanceId);
        if (originalColor) {
          this._animateColorTransition(instanceId, originalColor.clone());
        }
      }
    }
  }

  /**
   * Applies hover effect to a building with subtle brightness increase
   * @param nodeId - ID of the node to hover
   */
  hoverBuilding(nodeId: string): void {
    if (!this.instancedMesh || this.hoveredNodeId === nodeId) return;

    // Don't apply hover if already selected
    if (this.selectedNodeId === nodeId) return;

    this.hoveredNodeId = nodeId;

    // Apply subtle hover effect (brightness increase)
    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.originalColors.get(instanceId);
        if (originalColor) {
          const hoverColor = originalColor.clone().multiplyScalar(1.2);
          this._animateColorTransition(instanceId, hoverColor);
        }
      }
    }
  }

  /**
   * Removes hover effect from a building
   * @param nodeId - ID of the node to unhover
   */
  unhoverBuilding(nodeId: string): void {
    if (!this.instancedMesh || this.hoveredNodeId !== nodeId) return;

    // Don't remove hover if building is selected
    if (this.selectedNodeId === nodeId) return;

    this.hoveredNodeId = null;

    // Restore original colors
    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.originalColors.get(instanceId);
        if (originalColor) {
          this._animateColorTransition(instanceId, originalColor.clone());
        }
      }
    }
  }

  /**
   * Animates a smooth color transition for an instance
   * @param instanceId - Instance to animate
   * @param targetColor - Target color to transition to
   */
  private _animateColorTransition(instanceId: number, targetColor: THREE.Color): void {
    if (!this.instancedMesh) return;

    const currentColor = new THREE.Color();
    this.instancedMesh.getColorAt(instanceId, currentColor);

    // Store animation target
    this.animationTargets.set(instanceId, {
      target: targetColor,
      start: currentColor,
      progress: 0
    });

    // Immediate update for now (smooth animation would require update loop integration)
    // For now, apply the color immediately for instant feedback
    this.instancedMesh.setColorAt(instanceId, targetColor);
    this.instancedMesh.instanceColor!.needsUpdate = true;
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
   * Disposes of all resources
   * Call this when removing the visualization to free GPU memory
   */
  dispose(): void {
    if (this.instancedMesh) {
      this.scene.remove(this.instancedMesh);

      this.instancedMesh.geometry.dispose();

      // Handle material disposal (may be array or single material)
      if (Array.isArray(this.instancedMesh.material)) {
        this.instancedMesh.material.forEach(mat => mat.dispose());
      } else {
        this.instancedMesh.material.dispose();
      }

      this.instancedMesh = null;
      this.voxelToNodeMap.clear();
      this.layoutNodes = [];
      this.originalColors.clear();
      this.animationTargets.clear();
      this.selectedNodeId = null;
      this.hoveredNodeId = null;

      console.log('VoxelRenderer: Disposed all resources');
    }
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
