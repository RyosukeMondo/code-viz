/**
 * Unit tests for RaycasterHandler
 * Tests raycasting, selection, hover detection, and event handling
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import * as THREE from 'three';
import { RaycasterHandler, createRaycasterHandler, type SelectionCallback } from './RaycasterHandler';
import type { LayoutNode } from '../types';

// Mock LayoutNode
const createMockNode = (id: string, path: string): LayoutNode => ({
  id,
  name: path.split('/').pop() || path,
  path,
  type: 'file',
  x0: 0,
  y0: 0,
  x1: 100,
  y1: 100,
  height: 50,
  metrics: {
    loc: 100,
    complexity: 10,
    functions: 5,
    classes: 2
  },
  children: []
});

// Mock VoxelRenderer
const createMockVoxelRenderer = () => {
  const voxelToNodeMap = new Map<number, { nodeId: string; node: LayoutNode; voxelLevel: number }>();
  const highlightedBuildings = new Set<string>();

  const mockNode = createMockNode('node-1', 'src/test.ts');
  voxelToNodeMap.set(0, { nodeId: 'node-1', node: mockNode, voxelLevel: 0 });

  return {
    voxelToNodeMap,
    highlightedBuildings,
    getNodeByInstanceId: vi.fn((instanceId: number) => {
      const voxelInfo = voxelToNodeMap.get(instanceId);
      return voxelInfo?.node;
    }),
    highlightBuilding: vi.fn((nodeId: string) => {
      highlightedBuildings.add(nodeId);
    }),
    unhighlightBuilding: vi.fn((nodeId: string) => {
      highlightedBuildings.delete(nodeId);
    })
  };
};

// Mock canvas element
const createMockCanvas = () => {
  const listeners: Record<string, EventListener[]> = {};

  const canvas = {
    width: 800,
    height: 600,
    style: { cursor: 'default' },
    getBoundingClientRect: vi.fn(() => ({
      left: 0,
      top: 0,
      width: 800,
      height: 600,
      right: 800,
      bottom: 600,
      x: 0,
      y: 0,
      toJSON: () => ({})
    })),
    addEventListener: vi.fn((event: string, listener: EventListener) => {
      if (!listeners[event]) listeners[event] = [];
      listeners[event].push(listener);
    }),
    removeEventListener: vi.fn((event: string, listener: EventListener) => {
      if (listeners[event]) {
        listeners[event] = listeners[event].filter(l => l !== listener);
      }
    }),
    dispatchEvent: (event: Event) => {
      const eventListeners = listeners[event.type] || [];
      eventListeners.forEach(listener => listener(event));
      return true;
    }
  };

  return { canvas: canvas as unknown as HTMLCanvasElement, listeners };
};

describe('RaycasterHandler', () => {
  let canvas: HTMLCanvasElement;
  let camera: THREE.Camera;
  let instancedMesh: THREE.InstancedMesh;
  let voxelRenderer: ReturnType<typeof createMockVoxelRenderer>;
  let onSelect: SelectionCallback;
  let handler: RaycasterHandler;

  beforeEach(() => {
    // Setup Three.js objects
    camera = new THREE.PerspectiveCamera(75, 800 / 600, 0.1, 1000);
    camera.position.z = 5;

    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial();
    instancedMesh = new THREE.InstancedMesh(geometry, material, 1);

    // Setup mock objects
    const mockCanvas = createMockCanvas();
    canvas = mockCanvas.canvas;
    voxelRenderer = createMockVoxelRenderer();
    onSelect = vi.fn();

    // Mock Raycaster intersections
    vi.spyOn(THREE.Raycaster.prototype, 'intersectObject').mockImplementation(function(this: THREE.Raycaster, object: THREE.Object3D) {
      if (object === instancedMesh) {
        // Return intersection with instanceId 0 for testing
        return [{
          distance: 1,
          point: new THREE.Vector3(0, 0, 0),
          instanceId: 0,
          object: instancedMesh,
          face: null,
          faceIndex: 0,
          uv: undefined,
          uv1: undefined
        }];
      }
      return [];
    });
  });

  afterEach(() => {
    if (handler) {
      handler.dispose();
    }
    vi.restoreAllMocks();
  });

  describe('constructor', () => {
    it('should create handler with default options', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);

      expect(handler).toBeDefined();
      expect(handler.getSelectedNodeId()).toBeNull();
      expect(handler.getHoveredNodeId()).toBeNull();
    });

    it('should create handler with custom options', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect, {
        maxChecksPerSecond: 30,
        dragThreshold: 10
      });

      expect(handler).toBeDefined();
    });

    it('should attach event listeners to canvas', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);

      expect(canvas.addEventListener).toHaveBeenCalledWith('mousedown', expect.any(Function));
      expect(canvas.addEventListener).toHaveBeenCalledWith('mousemove', expect.any(Function));
      expect(canvas.addEventListener).toHaveBeenCalledWith('mouseup', expect.any(Function));
    });
  });

  describe('selection', () => {
    beforeEach(() => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
    });

    it('should select building on click', () => {
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      expect(handler.getSelectedNodeId()).toBe('node-1');
      expect(voxelRenderer.highlightBuilding).toHaveBeenCalledWith('node-1', '#ffffff');
      expect(onSelect).toHaveBeenCalledWith(expect.objectContaining({
        id: 'node-1',
        name: 'test.ts',
        path: 'src/test.ts'
      }));
    });

    it('should not re-select already selected building', () => {
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      vi.clearAllMocks();
      handler.selectBuilding(mockNode);

      expect(voxelRenderer.highlightBuilding).not.toHaveBeenCalled();
      expect(onSelect).not.toHaveBeenCalled();
    });

    it('should deselect previous building when selecting new one', () => {
      const mockNode1 = createMockNode('node-1', 'src/test1.ts');
      const mockNode2 = createMockNode('node-2', 'src/test2.ts');

      handler.selectBuilding(mockNode1);
      vi.clearAllMocks();
      handler.selectBuilding(mockNode2);

      expect(voxelRenderer.unhighlightBuilding).toHaveBeenCalledWith('node-1');
      expect(voxelRenderer.highlightBuilding).toHaveBeenCalledWith('node-2', '#ffffff');
      expect(handler.getSelectedNodeId()).toBe('node-2');
    });

    it('should deselect building', () => {
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      vi.clearAllMocks();
      handler.deselectBuilding();

      expect(voxelRenderer.unhighlightBuilding).toHaveBeenCalledWith('node-1');
      expect(handler.getSelectedNodeId()).toBeNull();
      expect(onSelect).toHaveBeenCalledWith(null);
    });

    it('should not error when deselecting with no selection', () => {
      expect(() => handler.deselectBuilding()).not.toThrow();
      expect(onSelect).not.toHaveBeenCalled();
    });
  });

  describe('mouse events', () => {
    beforeEach(() => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
    });

    it('should detect click without drag', () => {
      const mouseDownEvent = new MouseEvent('mousedown', {
        clientX: 400,
        clientY: 300
      });
      const mouseUpEvent = new MouseEvent('mouseup', {
        clientX: 400,
        clientY: 300
      });

      canvas.dispatchEvent(mouseDownEvent);
      canvas.dispatchEvent(mouseUpEvent);

      expect(onSelect).toHaveBeenCalled();
    });

    it('should ignore click if dragging', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect, {
        dragThreshold: 5
      });

      const mouseDownEvent = new MouseEvent('mousedown', {
        clientX: 400,
        clientY: 300
      });
      const mouseMoveEvent = new MouseEvent('mousemove', {
        clientX: 410,
        clientY: 310
      });
      const mouseUpEvent = new MouseEvent('mouseup', {
        clientX: 410,
        clientY: 310
      });

      canvas.dispatchEvent(mouseDownEvent);
      canvas.dispatchEvent(mouseMoveEvent);
      canvas.dispatchEvent(mouseUpEvent);

      expect(onSelect).not.toHaveBeenCalled();
    });

    it('should update cursor on hover', () => {
      const mouseMoveEvent = new MouseEvent('mousemove', {
        clientX: 400,
        clientY: 300
      });

      canvas.dispatchEvent(mouseMoveEvent);

      expect(canvas.style.cursor).toBe('pointer');
      expect(handler.getHoveredNodeId()).toBe('node-1');
    });

    it('should reset cursor when not hovering', async () => {
      // First set up hover state
      canvas.style.cursor = 'pointer';
      const mouseMoveEvent1 = new MouseEvent('mousemove', {
        clientX: 400,
        clientY: 300
      });
      canvas.dispatchEvent(mouseMoveEvent1);

      // Wait for throttle interval (default is 1000/60 = ~16.67ms)
      await new Promise(resolve => setTimeout(resolve, 20));

      // Now mock no intersection for next move
      vi.spyOn(THREE.Raycaster.prototype, 'intersectObject').mockReturnValue([]);

      const mouseMoveEvent2 = new MouseEvent('mousemove', {
        clientX: 450,
        clientY: 350
      });

      canvas.dispatchEvent(mouseMoveEvent2);

      expect(canvas.style.cursor).toBe('default');
      expect(handler.getHoveredNodeId()).toBeNull();
    });
  });

  describe('raycasting', () => {
    beforeEach(() => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
    });

    it('should raycast at specific coordinates', () => {
      handler.raycastAt(400, 300);

      expect(onSelect).toHaveBeenCalled();
    });

    it('should handle no intersection', () => {
      // Mock no intersection
      vi.spyOn(THREE.Raycaster.prototype, 'intersectObject').mockReturnValue([]);

      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      vi.clearAllMocks();
      handler.raycastAt(400, 300);

      expect(handler.getSelectedNodeId()).toBeNull();
      expect(onSelect).toHaveBeenCalledWith(null);
    });

    it('should handle intersection without instanceId', () => {
      // First select a building
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);
      vi.clearAllMocks();

      // Mock intersection without instanceId
      vi.spyOn(THREE.Raycaster.prototype, 'intersectObject').mockReturnValue([{
        distance: 1,
        point: new THREE.Vector3(0, 0, 0),
        object: instancedMesh,
        face: null,
        faceIndex: 0,
        uv: undefined,
        uv1: undefined
      }]);

      handler.raycastAt(400, 300);

      // Should not change selection when intersection has no instanceId
      expect(onSelect).not.toHaveBeenCalled();
      expect(handler.getSelectedNodeId()).toBe('node-1');
    });
  });

  describe('updates', () => {
    beforeEach(() => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
    });

    it('should update instanced mesh', () => {
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      const newMesh = new THREE.InstancedMesh(
        new THREE.BoxGeometry(1, 1, 1),
        new THREE.MeshStandardMaterial(),
        1
      );

      handler.updateInstancedMesh(newMesh);

      expect(handler.getSelectedNodeId()).toBeNull();
    });

    it('should update voxel renderer', () => {
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      const newRenderer = createMockVoxelRenderer();
      handler.updateVoxelRenderer(newRenderer);

      expect(handler.getSelectedNodeId()).toBeNull();
    });
  });

  describe('dispose', () => {
    it('should remove event listeners', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);

      handler.dispose();

      expect(canvas.removeEventListener).toHaveBeenCalledWith('mousedown', expect.any(Function));
      expect(canvas.removeEventListener).toHaveBeenCalledWith('mousemove', expect.any(Function));
      expect(canvas.removeEventListener).toHaveBeenCalledWith('mouseup', expect.any(Function));
    });

    it('should reset cursor', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
      canvas.style.cursor = 'pointer';

      handler.dispose();

      expect(canvas.style.cursor).toBe('default');
    });

    it('should clear selection state', () => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
      const mockNode = createMockNode('node-1', 'src/test.ts');
      handler.selectBuilding(mockNode);

      handler.dispose();

      expect(handler.getSelectedNodeId()).toBeNull();
      expect(handler.getHoveredNodeId()).toBeNull();
    });
  });

  describe('createRaycasterHandler', () => {
    it('should create handler using factory function', () => {
      handler = createRaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);

      expect(handler).toBeInstanceOf(RaycasterHandler);
      expect(handler.getSelectedNodeId()).toBeNull();
    });

    it('should create handler with options', () => {
      handler = createRaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect, {
        maxChecksPerSecond: 30,
        dragThreshold: 10
      });

      expect(handler).toBeInstanceOf(RaycasterHandler);
    });
  });

  describe('BuildingData format', () => {
    beforeEach(() => {
      handler = new RaycasterHandler(canvas, camera, instancedMesh, voxelRenderer, onSelect);
    });

    it('should provide complete building data in callback', () => {
      const mockNode = createMockNode('node-1', 'src/components/test.ts');
      mockNode.x0 = 10;
      mockNode.y0 = 20;
      mockNode.x1 = 110;
      mockNode.y1 = 120;
      mockNode.height = 50;

      handler.selectBuilding(mockNode);

      expect(onSelect).toHaveBeenCalledWith({
        id: 'node-1',
        name: 'test.ts',
        path: 'src/components/test.ts',
        metrics: mockNode.metrics,
        position: { x: 10, y: 20 },
        dimensions: {
          width: 100,
          depth: 100,
          height: 50
        }
      });
    });
  });
});
