/**
 * Heat Haze Effect Manager
 * Applies heat haze shader effects to high-complexity buildings
 * Works alongside VoxelRenderer to add visual emphasis to problematic code
 * @module components/visualizations/3d/shaders/HeatHazeEffect
 */

import * as THREE from 'three';
import { HeatHazeShader, shouldUseHeatHaze } from './HeatHazeShader';
import { complexityToColor } from '../utils/colorMaps';
import { COMPLEXITY_THRESHOLDS, DEFAULT_VOXEL_SIZE } from '../utils/constants';
import type { LayoutNode } from '../types';

/**
 * Configuration options for HeatHazeEffect
 */
export interface HeatHazeEffectOptions {
  voxelSize?: number;
  complexityThreshold?: number;
}

/**
 * Mesh with user data for tracking
 */
interface EffectMesh extends THREE.Mesh {
  userData: {
    nodeId: string;
    complexity: number;
    node: LayoutNode;
  };
  material: THREE.ShaderMaterial | THREE.MeshStandardMaterial;
}

/**
 * Statistics about heat haze effects
 */
export interface HeatHazeEffectStats {
  effectMeshesCount: number;
  totalNodesCount: number;
  shaderStats: ReturnType<HeatHazeShader['getStats']>;
  effectCoverage: string;
}

/**
 * HeatHazeEffect manages visual effects for high-complexity buildings
 * Creates overlay meshes with shader materials on top of base voxel rendering
 */
export class HeatHazeEffect {
  private scene: THREE.Scene;
  private camera: THREE.Camera;
  private voxelSize: number;
  private shader: HeatHazeShader;
  private effectMeshes: EffectMesh[];
  private layoutNodes: LayoutNode[];
  private clock: THREE.Clock;

  /**
   * Creates a new HeatHazeEffect instance
   * @param scene - Three.js scene to add effects to
   * @param camera - Camera for LOD calculations
   * @param options - Effect options
   */
  constructor(scene: THREE.Scene, camera: THREE.Camera, options: HeatHazeEffectOptions = {}) {
    this.scene = scene;
    this.camera = camera;
    this.voxelSize = options.voxelSize ?? DEFAULT_VOXEL_SIZE;

    // Shader manager
    this.shader = new HeatHazeShader({
      complexityThreshold: options.complexityThreshold ?? COMPLEXITY_THRESHOLDS.MEDIUM
    });

    // Track effect meshes
    this.effectMeshes = [];
    this.layoutNodes = [];

    // Clock for animation
    this.clock = new THREE.Clock();
  }

  /**
   * Applies heat haze effects to high-complexity buildings
   * @param layoutNodes - Layout nodes from TreemapLayout
   */
  apply(layoutNodes: LayoutNode[]): void {
    if (!layoutNodes || layoutNodes.length === 0) {
      console.warn('HeatHazeEffect: No layout nodes provided');
      return;
    }

    this.layoutNodes = layoutNodes;

    // Clear any existing effects
    this.clear();

    // Filter for high-complexity buildings
    const highComplexityNodes = layoutNodes.filter(node => {
      const complexity = node.metrics?.complexity ?? 0;
      return shouldUseHeatHaze(complexity, this.shader.getStats().complexityThreshold);
    });

    console.log(
      `HeatHazeEffect: Applying effect to ${highComplexityNodes.length} high-complexity buildings ` +
      `(out of ${layoutNodes.length} total)`
    );

    // Create effect meshes for each high-complexity building
    for (const node of highComplexityNodes) {
      this._createEffectMesh(node);
    }
  }

  /**
   * Creates an effect mesh for a single building
   * @param node - Layout node to create effect for
   */
  private _createEffectMesh(node: LayoutNode): void {
    const complexity = node.metrics?.complexity ?? 0;
    const baseColor = complexityToColor(complexity);

    // Create material with shader
    const material = this.shader.createMaterial(complexity, baseColor);
    if (!material) return;

    // Calculate building dimensions
    const buildingWidth = node.x1 - node.x0;
    const buildingDepth = node.y1 - node.y0;
    const buildingHeight = node.height;

    // Create geometry that matches the building footprint
    // Use a slightly smaller box to avoid z-fighting with base voxels
    const geometry = new THREE.BoxGeometry(
      buildingWidth * 0.95,
      buildingHeight,
      buildingDepth * 0.95
    );

    // Create mesh
    const mesh = new THREE.Mesh(geometry, material) as EffectMesh;

    // Position at building center
    const centerX = (node.x0 + node.x1) / 2;
    const centerZ = (node.y0 + node.y1) / 2;
    const centerY = buildingHeight / 2;

    mesh.position.set(centerX, centerY, centerZ);

    // Store reference to node for later updates
    mesh.userData = {
      nodeId: node.id,
      complexity: complexity,
      node: node
    };

    // Add to scene
    this.scene.add(mesh);
    this.effectMeshes.push(mesh);
  }

  /**
   * Updates effect animation
   * Should be called in animation loop
   */
  update(): void {
    const deltaTime = this.clock.getDelta();

    // Update shader time
    this.shader.update(deltaTime);

    // LOD: Hide effects when camera is far away
    for (const mesh of this.effectMeshes) {
      const shouldShow = this.shader.shouldApplyEffect(mesh.position, this.camera);
      mesh.visible = shouldShow;
    }
  }

  /**
   * Highlights a specific building's effect
   * @param nodeId - ID of the node to highlight
   */
  highlightBuilding(nodeId: string): void {
    const mesh = this.effectMeshes.find(m => m.userData.nodeId === nodeId);
    if (!mesh) return;

    // Increase intensity for highlighting (only for shader materials)
    if ('uniforms' in mesh.material && mesh.material.uniforms.intensity) {
      const currentIntensity = mesh.material.uniforms.intensity.value as number;
      mesh.material.uniforms.intensity.value = Math.min(currentIntensity * 1.5, 1.0);
    }
  }

  /**
   * Removes highlighting from a building
   * @param nodeId - ID of the node to unhighlight
   */
  unhighlightBuilding(nodeId: string): void {
    const mesh = this.effectMeshes.find(m => m.userData.nodeId === nodeId);
    if (!mesh) return;

    // Restore original intensity
    const complexity = mesh.userData.complexity;
    const threshold = this.shader.getStats().complexityThreshold;
    const originalIntensity = Math.min((complexity - threshold) / 30, 1.0);

    if ('uniforms' in mesh.material && mesh.material.uniforms.intensity) {
      mesh.material.uniforms.intensity.value = originalIntensity;
    }
  }

  /**
   * Clears all effect meshes
   */
  clear(): void {
    for (const mesh of this.effectMeshes) {
      this.scene.remove(mesh);
      mesh.geometry.dispose();
      mesh.material.dispose();
    }
    this.effectMeshes = [];
  }

  /**
   * Gets statistics about the effect
   * @returns Statistics object
   */
  getStats(): HeatHazeEffectStats {
    return {
      effectMeshesCount: this.effectMeshes.length,
      totalNodesCount: this.layoutNodes.length,
      shaderStats: this.shader.getStats(),
      effectCoverage: this.layoutNodes.length > 0
        ? (this.effectMeshes.length / this.layoutNodes.length * 100).toFixed(1) + '%'
        : '0%'
    };
  }

  /**
   * Disposes of all resources
   */
  dispose(): void {
    this.clear();
    this.shader.dispose();
    console.log('HeatHazeEffect: Disposed all resources');
  }
}
