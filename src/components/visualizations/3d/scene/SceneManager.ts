/**
 * SceneManager manages the Three.js scene, camera, renderer, lights, and controls
 * @module components/visualizations/3d/scene/SceneManager
 */

import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import type { SceneManagerOptions, SelectionState, LayoutNode, VoxelMapping } from '../types';
import { CameraPersistence } from './CameraPersistence';
import { RaycasterHandler, type SelectionCallback, type BuildingData } from './RaycasterHandler';

/**
 * VoxelRenderer interface for raycaster integration and optimization
 * Duck-typed to avoid circular dependency
 */
interface IVoxelRenderer {
  voxelToNodeMap: Map<number, VoxelMapping>;
  getNodeByInstanceId(instanceId: number): LayoutNode | undefined;
  highlightBuilding(nodeId: string, color: string): void;
  unhighlightBuilding(nodeId: string): void;
  update?(deltaTime: number, camera?: THREE.Camera, currentFps?: number): void;
  updateFrustum?(camera: THREE.Camera): void;
  applyLOD?(camera: THREE.Camera): void;
}

/**
 * SceneManager manages the Three.js scene, camera, renderer, lights, and controls.
 * Provides initialization, animation loop, and resource cleanup functionality.
 */
export class SceneManager {
  private canvas: HTMLCanvasElement;
  private scene: THREE.Scene | null = null;
  private camera: THREE.PerspectiveCamera | null = null;
  private renderer: THREE.WebGLRenderer | null = null;
  private controls: OrbitControls | null = null;
  private animationId: number | null = null;
  private lastFrameTime = 0;
  private targetFPS: number;
  private frameInterval: number;
  private handleResize: (() => void) | null = null;
  private cameraPersistence: CameraPersistence | null = null;
  private raycasterHandler: RaycasterHandler | null = null;
  private selectionState: SelectionState = {
    selectedNode: null,
    hoveredNode: null
  };
  private selectionCallbacks: Set<SelectionCallback> = new Set();
  private projectKey: string;
  private voxelRenderer: IVoxelRenderer | null = null;

  // Performance tracking
  private fpsFrameTimes: number[] = [];
  private currentFps = 60;

  /**
   * Creates a SceneManager instance
   * @param canvas - The canvas element to render to
   * @param options - Scene manager options
   */
  constructor(canvas: HTMLCanvasElement, options: SceneManagerOptions = {}) {
    this.canvas = canvas;
    this.targetFPS = options.targetFPS || 60;
    this.frameInterval = 1000 / this.targetFPS;
    this.projectKey = options.projectKey || 'default';
  }

  /**
   * Checks if WebGL is supported in the current browser
   * @returns True if WebGL is supported
   */
  static isWebGLSupported(): boolean {
    try {
      const canvas = document.createElement('canvas');
      const context = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
      return !!context;
    } catch {
      return false;
    }
  }

  /**
   * Initializes the Three.js scene, camera, renderer, lights, and controls
   * @throws Error if WebGL is not supported
   */
  initialize(): void {
    if (!SceneManager.isWebGLSupported()) {
      throw new Error('WebGL is not supported in this browser. Please use a modern browser with WebGL support.');
    }

    this.scene = new THREE.Scene();
    this.scene.background = new THREE.Color(0xf5f5f5);

    const aspect = this.canvas.clientWidth / this.canvas.clientHeight;
    this.camera = new THREE.PerspectiveCamera(75, aspect, 0.1, 2000);
    this.camera.position.set(100, 160, 100);
    this.camera.lookAt(0, 0, 0);

    this.renderer = new THREE.WebGLRenderer({
      canvas: this.canvas,
      antialias: true,
      alpha: false
    });
    this.renderer.setSize(this.canvas.clientWidth, this.canvas.clientHeight);
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    console.log('Renderer capabilities:', {
      maxAttributes: this.renderer.capabilities.maxAttributes,
      maxVertexUniforms: this.renderer.capabilities.maxVertexUniforms,
      maxTextureSize: this.renderer.capabilities.maxTextureSize
    });

    const hemisphereLight = new THREE.HemisphereLight(0xffffff, 0x444444, 0.4);
    this.scene.add(hemisphereLight);

    const ambientLight = new THREE.AmbientLight(0xffffff, 0.3);
    this.scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 1.5);
    directionalLight.position.set(100, 200, 100);
    directionalLight.castShadow = true;

    directionalLight.shadow.mapSize.width = 4096;
    directionalLight.shadow.mapSize.height = 4096;
    directionalLight.shadow.camera.near = 0.5;
    directionalLight.shadow.camera.far = 1000;
    directionalLight.shadow.camera.left = -600;
    directionalLight.shadow.camera.right = 600;
    directionalLight.shadow.camera.top = 600;
    directionalLight.shadow.camera.bottom = -600;
    directionalLight.shadow.bias = -0.0001;
    directionalLight.shadow.normalBias = 0.02;

    this.scene.add(directionalLight);

    const rimLight = new THREE.DirectionalLight(0xaaccff, 0.8);
    rimLight.position.set(-100, 150, -100);
    this.scene.add(rimLight);

    const fillLight = new THREE.DirectionalLight(0xffffff, 0.4);
    fillLight.position.set(-150, 100, 150);
    this.scene.add(fillLight);

    const groundSize = 1000;
    const groundGeometry = new THREE.PlaneGeometry(groundSize, groundSize);
    const groundMaterial = new THREE.MeshStandardMaterial({
      color: 0xffffff,
      roughness: 0.8,
      metalness: 0.0,
      side: THREE.DoubleSide
    });
    const ground = new THREE.Mesh(groundGeometry, groundMaterial);
    ground.rotation.x = -Math.PI / 2;
    ground.position.y = -0.5;
    ground.receiveShadow = true;
    this.scene.add(ground);

    const gridHelper = new THREE.GridHelper(groundSize, 100, 0xcccccc, 0xe0e0e0);
    gridHelper.position.y = -0.4;
    this.scene.add(gridHelper);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.05;
    this.controls.minDistance = 10;
    this.controls.maxDistance = 500;
    this.controls.maxPolarAngle = Math.PI / 2;

    // Initialize camera persistence
    this.cameraPersistence = new CameraPersistence(this.camera, this.controls, this.projectKey);
    this.cameraPersistence.restore();
    this.cameraPersistence.startAutoSave();

    this.handleResize = this.onResize.bind(this);
    window.addEventListener('resize', this.handleResize);
  }

  /**
   * Handles window resize events
   */
  private onResize(): void {
    if (!this.camera || !this.renderer) return;

    this.camera.aspect = this.canvas.clientWidth / this.canvas.clientHeight;
    this.camera.updateProjectionMatrix();

    this.renderer.setSize(this.canvas.clientWidth, this.canvas.clientHeight);
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  }

  /**
   * Animation loop that renders the scene at target FPS
   * @param currentTime - Current timestamp from requestAnimationFrame
   */
  animate(currentTime = 0): void {
    this.animationId = requestAnimationFrame((time) => this.animate(time));

    const deltaTime = currentTime - this.lastFrameTime;

    if (deltaTime < this.frameInterval) {
      return;
    }

    this.lastFrameTime = currentTime - (deltaTime % this.frameInterval);

    // Calculate FPS
    this._updateFPS(deltaTime);

    // Update controls
    if (this.controls) {
      this.controls.update();
    }

    // Update voxel renderer with optimization data
    if (this.voxelRenderer && this.camera) {
      const deltaSeconds = deltaTime / 1000;

      // Update frustum for culling
      if (this.voxelRenderer.updateFrustum) {
        this.voxelRenderer.updateFrustum(this.camera);
      }

      // Apply LOD based on camera distance
      if (this.voxelRenderer.applyLOD) {
        this.voxelRenderer.applyLOD(this.camera);
      }

      // Update renderer with FPS for adaptive quality
      if (this.voxelRenderer.update) {
        this.voxelRenderer.update(deltaSeconds, this.camera, this.currentFps);
      }
    }

    // Render scene
    if (this.renderer && this.scene && this.camera) {
      this.renderer.render(this.scene, this.camera);
    }
  }

  /**
   * Updates FPS calculation
   * @param deltaTime - Time since last frame in milliseconds
   * @private
   */
  private _updateFPS(deltaTime: number): void {
    // Track frame times for accurate FPS calculation
    this.fpsFrameTimes.push(deltaTime);

    // Keep only last 60 frames
    if (this.fpsFrameTimes.length > 60) {
      this.fpsFrameTimes.shift();
    }

    // Calculate average FPS
    if (this.fpsFrameTimes.length > 0) {
      const avgDelta = this.fpsFrameTimes.reduce((a, b) => a + b, 0) / this.fpsFrameTimes.length;
      this.currentFps = avgDelta > 0 ? 1000 / avgDelta : 60;
    }
  }

  /**
   * Gets current FPS
   * @returns Current frames per second
   */
  getCurrentFPS(): number {
    return this.currentFps;
  }

  /**
   * Adds an object to the scene
   * @param object - The object to add
   */
  add(object: THREE.Object3D): void {
    if (this.scene) {
      this.scene.add(object);
    }
  }

  /**
   * Removes an object from the scene
   * @param object - The object to remove
   */
  remove(object: THREE.Object3D): void {
    if (this.scene) {
      this.scene.remove(object);
    }
  }

  /**
   * Gets the current scene
   * @returns The scene
   */
  getScene(): THREE.Scene | null {
    return this.scene;
  }

  /**
   * Gets the current camera
   * @returns The camera
   */
  getCamera(): THREE.PerspectiveCamera | null {
    return this.camera;
  }

  /**
   * Gets the renderer
   * @returns The renderer
   */
  getRenderer(): THREE.WebGLRenderer | null {
    return this.renderer;
  }

  /**
   * Gets the controls
   * @returns The orbit controls
   */
  getControls(): OrbitControls | null {
    return this.controls;
  }

  /**
   * Sets up raycaster handler for interactive selection
   * @param instancedMesh - The voxel instanced mesh to raycast against
   * @param voxelRenderer - VoxelRenderer instance for node lookups
   */
  setupRaycaster(instancedMesh: THREE.InstancedMesh, voxelRenderer: IVoxelRenderer): void {
    if (!this.camera) {
      throw new Error('SceneManager: Camera not initialized. Call initialize() first.');
    }

    // Store reference to voxel renderer for optimization updates
    this.voxelRenderer = voxelRenderer;

    // Dispose existing raycaster if any
    if (this.raycasterHandler) {
      this.raycasterHandler.dispose();
    }

    // Create internal selection callback that updates state and notifies listeners
    const onSelect: SelectionCallback = (building: BuildingData | null) => {
      this.handleSelection(building);
    };

    // Create raycaster handler
    this.raycasterHandler = new RaycasterHandler(
      this.canvas,
      this.camera,
      instancedMesh,
      voxelRenderer,
      onSelect
    );
  }

  /**
   * Handles selection changes from raycaster
   * @param building - Selected building data or null
   * @private
   */
  private handleSelection(building: BuildingData | null): void {
    // Update selection state
    if (building) {
      // Convert BuildingData to LayoutNode for state
      const layoutNode: LayoutNode = {
        id: building.id,
        name: building.name,
        path: building.path,
        x0: building.position.x,
        x1: building.position.x + building.dimensions.width,
        y0: building.position.y,
        y1: building.position.y + building.dimensions.depth,
        height: building.dimensions.height,
        color: '', // Color not needed for selection state
        metrics: building.metrics
      };
      this.selectionState.selectedNode = layoutNode;
    } else {
      this.selectionState.selectedNode = null;
    }

    // Notify all registered callbacks
    this.selectionCallbacks.forEach(callback => {
      callback(building);
    });
  }

  /**
   * Registers a callback to be called when selection changes
   * @param callback - Callback function to register
   */
  onSelectionChange(callback: SelectionCallback): void {
    this.selectionCallbacks.add(callback);
  }

  /**
   * Unregisters a selection change callback
   * @param callback - Callback function to unregister
   */
  offSelectionChange(callback: SelectionCallback): void {
    this.selectionCallbacks.delete(callback);
  }

  /**
   * Gets the current selection state
   * @returns The current selection state
   */
  getSelectionState(): SelectionState {
    return { ...this.selectionState };
  }

  /**
   * Gets the selected node
   * @returns The selected layout node or null
   */
  getSelectedNode(): LayoutNode | null {
    return this.selectionState.selectedNode;
  }

  /**
   * Programmatically select a building by node
   * @param node - The layout node to select
   */
  selectNode(node: LayoutNode | null): void {
    if (this.raycasterHandler) {
      if (node) {
        this.raycasterHandler.selectBuilding(node);
      } else {
        this.raycasterHandler.deselectBuilding();
      }
    }
  }

  /**
   * Clears the current selection
   */
  clearSelection(): void {
    if (this.raycasterHandler) {
      this.raycasterHandler.deselectBuilding();
    }
  }

  /**
   * Updates the raycaster's instanced mesh reference
   * @param instancedMesh - New instanced mesh
   */
  updateInstancedMesh(instancedMesh: THREE.InstancedMesh): void {
    if (this.raycasterHandler) {
      this.raycasterHandler.updateInstancedMesh(instancedMesh);
    }
  }

  /**
   * Disposes of all Three.js resources and stops animation
   */
  dispose(): void {
    if (this.animationId !== null) {
      cancelAnimationFrame(this.animationId);
      this.animationId = null;
    }

    if (this.handleResize) {
      window.removeEventListener('resize', this.handleResize);
    }

    // Dispose camera persistence
    if (this.cameraPersistence) {
      this.cameraPersistence.dispose();
      this.cameraPersistence = null;
    }

    // Dispose raycaster handler
    if (this.raycasterHandler) {
      this.raycasterHandler.dispose();
      this.raycasterHandler = null;
    }

    // Clear selection callbacks
    this.selectionCallbacks.clear();

    // Clear voxel renderer reference
    this.voxelRenderer = null;
    this.fpsFrameTimes = [];

    if (this.controls) {
      this.controls.dispose();
      this.controls = null;
    }

    if (this.renderer) {
      this.renderer.dispose();
      this.renderer = null;
    }

    if (this.scene) {
      this.scene.traverse((object) => {
        if (object instanceof THREE.Mesh) {
          if (object.geometry) {
            object.geometry.dispose();
          }
          if (object.material) {
            if (Array.isArray(object.material)) {
              object.material.forEach(material => material.dispose());
            } else {
              object.material.dispose();
            }
          }
        }
      });
      this.scene.clear();
      this.scene = null;
    }

    this.camera = null;
  }
}
