/**
 * RaycasterHandler - Handles raycasting for building selection
 * Detects clicks on InstancedMesh voxels and emits selection events
 *
 * Features:
 * - Click detection with drag distinction (prevents selection during camera orbit)
 * - Hover detection with cursor feedback
 * - Throttled hover checks for performance optimization
 * - Type-safe integration with Three.js and VoxelRenderer
 *
 * @module components/visualizations/3d/scene/RaycasterHandler
 */

import * as THREE from 'three';
import type { LayoutNode } from '../types';

/**
 * VoxelRenderer interface (duck-typed to avoid circular dependency)
 * This should match the public API of VoxelRenderer
 */
interface IVoxelRenderer {
  voxelToNodeMap: Map<number, { nodeId: string; node: LayoutNode; voxelLevel: number }>;
  getNodeByInstanceId(instanceId: number): LayoutNode | undefined;
  highlightBuilding(nodeId: string, color: string): void;
  unhighlightBuilding(nodeId: string): void;
}

/**
 * Building data passed to selection callback
 */
export interface BuildingData {
  id: string;
  name: string;
  path: string;
  metrics: LayoutNode['metrics'];
  position: { x: number; y: number };
  dimensions: {
    width: number;
    depth: number;
    height: number;
  };
}

/**
 * Selection callback type
 */
export type SelectionCallback = (building: BuildingData | null) => void;

/**
 * Configuration options for RaycasterHandler
 */
export interface RaycasterHandlerOptions {
  /** Maximum raycasting checks per second for hover detection (default: 60) */
  maxChecksPerSecond?: number;
  /** Drag threshold in pixels to distinguish drag from click (default: 5) */
  dragThreshold?: number;
}

/**
 * RaycasterHandler class that manages raycasting for interactive building selection
 * Uses Three.js Raycaster to detect clicks on InstancedMesh and identify buildings
 */
export class RaycasterHandler {
  private canvas: HTMLCanvasElement | null;
  private camera: THREE.Camera;
  private instancedMesh: THREE.InstancedMesh;
  private voxelRenderer: IVoxelRenderer;
  private onSelect: SelectionCallback | null;

  // Throttling configuration
  private readonly maxChecksPerSecond: number;
  private readonly minInterval: number;
  private lastCheckTime = 0;

  // Raycasting components
  private readonly raycaster: THREE.Raycaster;
  private readonly mouse: THREE.Vector2;

  // Drag detection
  private readonly mouseDownPos: THREE.Vector2;
  private isDragging = false;
  private readonly dragThreshold: number;

  // Selection state
  private selectedNodeId: string | null = null;
  private hoveredNodeId: string | null = null;

  // Event handlers (bound methods)
  private readonly handleMouseDown: (event: MouseEvent) => void;
  private readonly handleMouseMove: (event: MouseEvent) => void;
  private readonly handleMouseUp: (event: MouseEvent) => void;

  /**
   * Creates a new RaycasterHandler instance
   * @param canvas - Canvas element to listen for clicks
   * @param camera - Three.js camera for raycasting
   * @param instancedMesh - The voxel instanced mesh to raycast against
   * @param voxelRenderer - VoxelRenderer instance for node lookups
   * @param onSelect - Callback function called with selected building data
   * @param options - Configuration options
   */
  constructor(
    canvas: HTMLCanvasElement,
    camera: THREE.Camera,
    instancedMesh: THREE.InstancedMesh,
    voxelRenderer: IVoxelRenderer,
    onSelect: SelectionCallback,
    options: RaycasterHandlerOptions = {}
  ) {
    this.canvas = canvas;
    this.camera = camera;
    this.instancedMesh = instancedMesh;
    this.voxelRenderer = voxelRenderer;
    this.onSelect = onSelect;

    // Throttling configuration
    this.maxChecksPerSecond = options.maxChecksPerSecond ?? 60;
    this.minInterval = 1000 / this.maxChecksPerSecond;

    // Drag detection
    this.dragThreshold = options.dragThreshold ?? 5;

    // Raycasting components
    this.raycaster = new THREE.Raycaster();
    this.mouse = new THREE.Vector2();
    this.mouseDownPos = new THREE.Vector2();

    // Bind event handlers
    this.handleMouseDown = this.onMouseDown.bind(this);
    this.handleMouseMove = this.onMouseMove.bind(this);
    this.handleMouseUp = this.onClick.bind(this);

    // Initialize event listeners
    this.initialize();
  }

  /**
   * Initializes event listeners
   * @private
   */
  private initialize(): void {
    if (!this.canvas) return;

    this.canvas.addEventListener('mousedown', this.handleMouseDown);
    this.canvas.addEventListener('mousemove', this.handleMouseMove);
    this.canvas.addEventListener('mouseup', this.handleMouseUp);
  }

  /**
   * Handles mouse down event to track drag start
   * @private
   */
  private onMouseDown(event: MouseEvent): void {
    this.mouseDownPos.x = event.clientX;
    this.mouseDownPos.y = event.clientY;
    this.isDragging = false;
  }

  /**
   * Handles mouse move event to detect dragging and hovering
   * @private
   */
  private onMouseMove(event: MouseEvent): void {
    if (!this.canvas) return;

    // Check if mouse has moved beyond threshold since mouse down
    const dx = event.clientX - this.mouseDownPos.x;
    const dy = event.clientY - this.mouseDownPos.y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    if (distance > this.dragThreshold) {
      this.isDragging = true;
    }

    // Throttle hover checks for performance
    const currentTime = performance.now();
    if (currentTime - this.lastCheckTime < this.minInterval) {
      return;
    }
    this.lastCheckTime = currentTime;

    // Update mouse position for raycasting
    const rect = this.canvas.getBoundingClientRect();
    this.mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
    this.mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

    // Perform raycasting for hover detection
    this.raycaster.setFromCamera(this.mouse, this.camera);
    const intersects = this.raycaster.intersectObject(this.instancedMesh);

    if (intersects.length > 0) {
      const instanceId = intersects[0].instanceId;

      if (instanceId !== undefined) {
        const voxelInfo = this.voxelRenderer.voxelToNodeMap.get(instanceId);

        if (voxelInfo) {
          const nodeId = voxelInfo.nodeId;
          if (this.hoveredNodeId !== nodeId) {
            this.hoveredNodeId = nodeId;
            this.canvas.style.cursor = 'pointer';
          }
        }
      }
    } else {
      if (this.hoveredNodeId !== null) {
        this.hoveredNodeId = null;
        this.canvas.style.cursor = 'default';
      }
    }
  }

  /**
   * Handles mouse up (click) event
   * @private
   */
  private onClick(event: MouseEvent): void {
    // Don't process if dragging (orbit controls)
    if (this.isDragging) {
      return;
    }

    // No throttling for clicks - they're infrequent user actions that should always be processed
    this.performRaycast(event);
  }

  /**
   * Performs raycasting to detect clicked building
   * @private
   */
  private performRaycast(event: MouseEvent): void {
    if (!this.canvas) return;

    // Get canvas bounding rect for coordinate conversion
    const rect = this.canvas.getBoundingClientRect();

    // Convert mouse coordinates to normalized device coordinates (-1 to +1)
    this.mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
    this.mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

    // Update raycaster with mouse position
    this.raycaster.setFromCamera(this.mouse, this.camera);

    // Check for intersections with instanced mesh
    const intersects = this.raycaster.intersectObject(this.instancedMesh);

    if (intersects.length > 0) {
      // Get the first intersection
      const intersection = intersects[0];

      // Get instance ID from intersection
      const instanceId = intersection.instanceId;

      if (instanceId !== undefined && instanceId !== null) {
        // Get the layout node associated with this voxel instance
        const node = this.voxelRenderer.getNodeByInstanceId(instanceId);

        if (node) {
          // Select the building
          this.selectBuilding(node);
        }
      }
    } else {
      // Click on empty space - deselect
      this.deselectBuilding();
    }
  }

  /**
   * Selects a building and calls the onSelect callback
   * @param node - The layout node to select
   */
  selectBuilding(node: LayoutNode): void {
    // Check if already selected
    if (this.selectedNodeId === node.id) {
      return;
    }

    // Unhighlight previous selection
    if (this.selectedNodeId) {
      this.voxelRenderer.unhighlightBuilding(this.selectedNodeId);
    }

    // Highlight new selection
    this.selectedNodeId = node.id;
    this.voxelRenderer.highlightBuilding(node.id, '#ffffff');

    // Call selection callback with building data
    if (this.onSelect) {
      const buildingData: BuildingData = {
        id: node.id,
        name: node.name,
        path: node.path,
        metrics: node.metrics,
        position: { x: node.x0, y: node.y0 },
        dimensions: {
          width: node.x1 - node.x0,
          depth: node.y1 - node.y0,
          height: node.height
        }
      };
      this.onSelect(buildingData);
    }
  }

  /**
   * Deselects the currently selected building
   */
  deselectBuilding(): void {
    if (this.selectedNodeId) {
      this.voxelRenderer.unhighlightBuilding(this.selectedNodeId);
      this.selectedNodeId = null;

      // Call callback with null to indicate deselection
      if (this.onSelect) {
        this.onSelect(null);
      }
    }
  }

  /**
   * Gets the currently selected node ID
   * @returns The selected node ID or null
   */
  getSelectedNodeId(): string | null {
    return this.selectedNodeId;
  }

  /**
   * Gets the currently hovered node ID
   * @returns The hovered node ID or null
   */
  getHoveredNodeId(): string | null {
    return this.hoveredNodeId;
  }

  /**
   * Updates the instanced mesh reference (useful when regenerating the scene)
   * @param instancedMesh - New instanced mesh
   */
  updateInstancedMesh(instancedMesh: THREE.InstancedMesh): void {
    this.instancedMesh = instancedMesh;
    this.selectedNodeId = null; // Clear selection when mesh changes
  }

  /**
   * Updates the voxel renderer reference
   * @param voxelRenderer - New voxel renderer
   */
  updateVoxelRenderer(voxelRenderer: IVoxelRenderer): void {
    this.voxelRenderer = voxelRenderer;
    this.selectedNodeId = null; // Clear selection when renderer changes
  }

  /**
   * Manually trigger a raycast at specific mouse coordinates
   * Useful for programmatic selection or testing
   * @param clientX - Client X coordinate
   * @param clientY - Client Y coordinate
   */
  raycastAt(clientX: number, clientY: number): void {
    this.performRaycast({ clientX, clientY } as MouseEvent);
  }

  /**
   * Disposes of resources and removes event listeners
   */
  dispose(): void {
    // Remove event listeners
    if (this.canvas) {
      this.canvas.removeEventListener('mousedown', this.handleMouseDown);
      this.canvas.removeEventListener('mousemove', this.handleMouseMove);
      this.canvas.removeEventListener('mouseup', this.handleMouseUp);
      this.canvas.style.cursor = 'default';
    }

    // Clear references
    this.canvas = null;
    this.onSelect = null;
    this.selectedNodeId = null;
    this.hoveredNodeId = null;

    console.log('RaycasterHandler: Disposed all resources');
  }
}

/**
 * Helper function to create and initialize a RaycasterHandler
 * @param canvas - Canvas element
 * @param camera - Three.js camera
 * @param instancedMesh - Voxel instanced mesh
 * @param voxelRenderer - VoxelRenderer instance
 * @param onSelect - Selection callback
 * @param options - Configuration options
 * @returns The created handler
 */
export function createRaycasterHandler(
  canvas: HTMLCanvasElement,
  camera: THREE.Camera,
  instancedMesh: THREE.InstancedMesh,
  voxelRenderer: IVoxelRenderer,
  onSelect: SelectionCallback,
  options?: RaycasterHandlerOptions
): RaycasterHandler {
  return new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect, options);
}
