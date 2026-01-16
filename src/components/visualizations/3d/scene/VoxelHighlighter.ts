/**
 * VoxelHighlighter - Manages highlighting and hover effects for voxel rendering
 * Handles color transitions and selection state
 * @module components/visualizations/3d/scene/VoxelHighlighter
 */

import * as THREE from 'three';
import type { VoxelMapping } from '../types';
import { VoxelSelection } from './VoxelSelection';

/**
 * VoxelHighlighter manages highlighting and hover effects for buildings
 */
export class VoxelHighlighter {
  private selection: VoxelSelection;
  private voxelToNodeMap: Map<number, VoxelMapping>;
  private instancedMesh: THREE.InstancedMesh | null = null;

  constructor() {
    this.selection = new VoxelSelection();
    this.voxelToNodeMap = new Map();
  }

  /**
   * Sets the instanced mesh to operate on
   * @param mesh - Instanced mesh
   */
  setInstancedMesh(mesh: THREE.InstancedMesh | null): void {
    this.instancedMesh = mesh;
  }

  /**
   * Sets the voxel to node mapping
   * @param map - Voxel to node mapping
   */
  setVoxelToNodeMap(map: Map<number, VoxelMapping>): void {
    this.voxelToNodeMap = map;
  }

  /**
   * Gets the selection manager
   * @returns VoxelSelection instance
   */
  getSelection(): VoxelSelection {
    return this.selection;
  }

  /**
   * Updates animations
   * @param deltaTime - Time since last frame
   * @returns true if mesh needs update
   */
  updateAnimations(deltaTime: number): boolean {
    if (!this.instancedMesh) return false;
    const needsUpdate = this.selection.updateAnimations(deltaTime, this.instancedMesh);
    if (needsUpdate) {
      this.instancedMesh.instanceColor!.needsUpdate = true;
    }
    return needsUpdate;
  }

  /**
   * Highlights a specific building
   * @param nodeId - ID of the node to highlight
   * @param highlightColor - Color to use for highlighting
   */
  highlightBuilding(nodeId: string, highlightColor: THREE.Color | string = '#ffff00'): void {
    if (!this.instancedMesh) return;

    this.selection.setSelectedNode(nodeId);

    const color = typeof highlightColor === 'string'
      ? new THREE.Color(highlightColor)
      : highlightColor;

    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.selection.getOriginalColor(instanceId);
        if (originalColor) {
          const highlightedColor = this.selection.createHighlightColor(originalColor, color);
          this._animateColorTransition(instanceId, highlightedColor);
        }
      }
    }
  }

  /**
   * Removes highlighting from a building
   * @param nodeId - ID of the node to unhighlight
   */
  unhighlightBuilding(nodeId: string): void {
    if (!this.instancedMesh) return;

    if (this.selection.isSelected(nodeId)) {
      this.selection.setSelectedNode(null);
    }

    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.selection.getOriginalColor(instanceId);
        if (originalColor) {
          this._animateColorTransition(instanceId, originalColor.clone());
        }
      }
    }
  }

  /**
   * Applies hover effect to a building
   * @param nodeId - ID of the node to hover
   */
  hoverBuilding(nodeId: string): void {
    if (!this.instancedMesh || this.selection.isHovered(nodeId)) return;
    if (this.selection.isSelected(nodeId)) return;

    this.selection.setHoveredNode(nodeId);

    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.selection.getOriginalColor(instanceId);
        if (originalColor) {
          const hoverColor = this.selection.createHoverColor(originalColor);
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
    if (!this.instancedMesh || !this.selection.isHovered(nodeId)) return;
    if (this.selection.isSelected(nodeId)) return;

    this.selection.setHoveredNode(null);

    for (const [instanceId, mapping] of this.voxelToNodeMap.entries()) {
      if (mapping.nodeId === nodeId) {
        const originalColor = this.selection.getOriginalColor(instanceId);
        if (originalColor) {
          this._animateColorTransition(instanceId, originalColor.clone());
        }
      }
    }
  }

  /**
   * Animates a smooth color transition
   * @param instanceId - Instance to animate
   * @param targetColor - Target color
   */
  private _animateColorTransition(instanceId: number, targetColor: THREE.Color): void {
    if (!this.instancedMesh) return;

    const currentColor = new THREE.Color();
    this.instancedMesh.getColorAt(instanceId, currentColor);

    this.selection.addAnimation(instanceId, targetColor, currentColor);

    this.instancedMesh.setColorAt(instanceId, targetColor);
    this.instancedMesh.instanceColor!.needsUpdate = true;
  }

  /**
   * Clears all state
   */
  clear(): void {
    this.selection.clear();
    this.voxelToNodeMap.clear();
  }
}
