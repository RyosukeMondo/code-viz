/**
 * VoxelSelection - Manages selection and hover state for voxel rendering
 * Handles highlighting and animations
 * @module components/visualizations/3d/scene/VoxelSelection
 */

import * as THREE from 'three';

/**
 * Animation state for color transitions
 */
interface ColorAnimation {
  target: THREE.Color;
  start: THREE.Color;
  progress: number;
}

/**
 * VoxelSelection class manages selection highlighting and hover effects
 */
export class VoxelSelection {
  private selectedNodeId: string | null = null;
  private hoveredNodeId: string | null = null;
  private originalColors: Map<number, THREE.Color> = new Map();
  private animationTargets: Map<number, ColorAnimation> = new Map();

  /**
   * Stores the original color for an instance
   * @param instanceId - Instance ID
   * @param color - Original color to store
   */
  storeOriginalColor(instanceId: number, color: THREE.Color): void {
    this.originalColors.set(instanceId, color.clone());
  }

  /**
   * Gets the original color for an instance
   * @param instanceId - Instance ID
   * @returns Original color or null
   */
  getOriginalColor(instanceId: number): THREE.Color | null {
    return this.originalColors.get(instanceId) || null;
  }

  /**
   * Sets the selected node ID
   * @param nodeId - Node ID to select
   */
  setSelectedNode(nodeId: string | null): void {
    this.selectedNodeId = nodeId;
  }

  /**
   * Gets the selected node ID
   * @returns Selected node ID or null
   */
  getSelectedNode(): string | null {
    return this.selectedNodeId;
  }

  /**
   * Sets the hovered node ID
   * @param nodeId - Node ID to hover
   */
  setHoveredNode(nodeId: string | null): void {
    this.hoveredNodeId = nodeId;
  }

  /**
   * Gets the hovered node ID
   * @returns Hovered node ID or null
   */
  getHoveredNode(): string | null {
    return this.hoveredNodeId;
  }

  /**
   * Checks if a node is selected
   * @param nodeId - Node ID to check
   * @returns true if node is selected
   */
  isSelected(nodeId: string): boolean {
    return this.selectedNodeId === nodeId;
  }

  /**
   * Checks if a node is hovered
   * @param nodeId - Node ID to check
   * @returns true if node is hovered
   */
  isHovered(nodeId: string): boolean {
    return this.hoveredNodeId === nodeId;
  }

  /**
   * Adds a color animation for an instance
   * @param instanceId - Instance ID
   * @param targetColor - Target color to animate to
   * @param currentColor - Current color of the instance
   */
  addAnimation(instanceId: number, targetColor: THREE.Color, currentColor: THREE.Color): void {
    this.animationTargets.set(instanceId, {
      target: targetColor,
      start: currentColor,
      progress: 0
    });
  }

  /**
   * Updates all color animations
   * @param deltaTime - Time since last update in seconds
   * @param mesh - Instanced mesh to apply colors to
   * @returns true if any animation updated
   */
  updateAnimations(deltaTime: number, mesh: THREE.InstancedMesh): boolean {
    if (this.animationTargets.size === 0) return false;

    let needsUpdate = false;

    for (const [instanceId, animation] of this.animationTargets.entries()) {
      animation.progress += deltaTime * 5; // 5x speed for quick transitions

      if (animation.progress >= 1) {
        // Animation complete
        mesh.setColorAt(instanceId, animation.target);
        this.animationTargets.delete(instanceId);
        needsUpdate = true;
      } else {
        // Interpolate color
        const interpolated = animation.start.clone().lerp(animation.target, animation.progress);
        mesh.setColorAt(instanceId, interpolated);
        needsUpdate = true;
      }
    }

    return needsUpdate;
  }

  /**
   * Creates a highlighted color by mixing with highlight color and brightening
   * @param originalColor - Original color
   * @param highlightColor - Color to mix in
   * @returns Highlighted color
   */
  createHighlightColor(originalColor: THREE.Color, highlightColor: THREE.Color): THREE.Color {
    return originalColor.clone()
      .lerp(highlightColor, 0.5)  // Mix with highlight color
      .multiplyScalar(1.5);        // Brighten
  }

  /**
   * Creates a hover color by brightening the original
   * @param originalColor - Original color
   * @returns Hover color
   */
  createHoverColor(originalColor: THREE.Color): THREE.Color {
    return originalColor.clone().multiplyScalar(1.2);
  }

  /**
   * Clears all selection state
   */
  clear(): void {
    this.selectedNodeId = null;
    this.hoveredNodeId = null;
    this.originalColors.clear();
    this.animationTargets.clear();
  }
}
