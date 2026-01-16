/**
 * Tests for useSelection hook
 * @module components/visualizations/3d/hooks/useSelection.test
 */

import { describe, it, expect, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useSelection } from './useSelection';
import type { BuildingData } from '../scene/RaycasterHandler';

describe('useSelection', () => {
  const mockBuildingData: BuildingData = {
    id: 'building-1',
    name: 'test.ts',
    path: '/src/test.ts',
    position: { x: 10, y: 20, z: 0 },
    dimensions: { width: 50, depth: 30, height: 100 },
    metrics: { loc: 200, complexity: 15 }
  };

  const mockBuildingData2: BuildingData = {
    id: 'building-2',
    name: 'other.ts',
    path: '/src/other.ts',
    position: { x: 100, y: 200, z: 0 },
    dimensions: { width: 40, depth: 25, height: 80 },
    metrics: { loc: 150, complexity: 10 }
  };

  describe('initialization', () => {
    it('should initialize with null selection and hover', () => {
      const { result } = renderHook(() => useSelection());

      expect(result.current.selectedNode).toBeNull();
      expect(result.current.hoveredNode).toBeNull();
      expect(result.current.selectionState.selectedNode).toBeNull();
      expect(result.current.selectionState.hoveredNode).toBeNull();
    });

    it('should initialize with provided initial selection', () => {
      const initialNode = {
        id: 'initial-1',
        name: 'initial.ts',
        path: '/initial.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#00ff00',
        metrics: { loc: 100, complexity: 5 }
      };

      const { result } = renderHook(() => useSelection({ initialSelected: initialNode }));

      expect(result.current.selectedNode).toEqual(initialNode);
      expect(result.current.selectionState.selectedNode).toEqual(initialNode);
    });
  });

  describe('selectBuilding', () => {
    it('should select a building', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.selectedNode).not.toBeNull();
      expect(result.current.selectedNode?.id).toBe('building-1');
      expect(result.current.selectedNode?.name).toBe('test.ts');
      expect(result.current.selectedNode?.path).toBe('/src/test.ts');
      expect(result.current.selectedNode?.x0).toBe(10);
      expect(result.current.selectedNode?.y0).toBe(20);
      expect(result.current.selectedNode?.height).toBe(100);
    });

    it('should convert BuildingData dimensions correctly', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      const node = result.current.selectedNode;
      expect(node?.x0).toBe(10);
      expect(node?.y0).toBe(20);
      expect(node?.x1).toBe(60); // x0 + width
      expect(node?.y1).toBe(50); // y0 + depth
    });

    it('should clear selection when null is passed', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.selectedNode).not.toBeNull();

      act(() => {
        result.current.selectBuilding(null);
      });

      expect(result.current.selectedNode).toBeNull();
    });

    it('should call onSelectionChange callback when selection changes', () => {
      const onSelectionChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onSelectionChange }));

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(onSelectionChange).toHaveBeenCalledTimes(1);
      expect(onSelectionChange).toHaveBeenCalledWith(expect.objectContaining({
        id: 'building-1',
        name: 'test.ts'
      }));
    });

    it('should not call onSelectionChange when selecting same building', () => {
      const onSelectionChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onSelectionChange }));

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      onSelectionChange.mockClear();

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(onSelectionChange).not.toHaveBeenCalled();
    });

    it('should handle rapid selection changes', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
        result.current.selectBuilding(mockBuildingData2);
        result.current.selectBuilding(null);
      });

      expect(result.current.selectedNode).toBeNull();
    });

    it('should update selectionState when selection changes', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.selectionState.selectedNode?.id).toBe('building-1');
    });
  });

  describe('clearSelection', () => {
    it('should clear the current selection', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.selectedNode).not.toBeNull();

      act(() => {
        result.current.clearSelection();
      });

      expect(result.current.selectedNode).toBeNull();
    });

    it('should call onSelectionChange callback when clearing', () => {
      const onSelectionChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onSelectionChange }));

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      onSelectionChange.mockClear();

      act(() => {
        result.current.clearSelection();
      });

      expect(onSelectionChange).toHaveBeenCalledTimes(1);
      expect(onSelectionChange).toHaveBeenCalledWith(null);
    });

    it('should not call callback when clearing already empty selection', () => {
      const onSelectionChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onSelectionChange }));

      act(() => {
        result.current.clearSelection();
      });

      expect(onSelectionChange).not.toHaveBeenCalled();
    });
  });

  describe('setHoveredNode', () => {
    it('should set hovered node', () => {
      const { result } = renderHook(() => useSelection());

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(result.current.hoveredNode).toEqual(hoverNode);
      expect(result.current.selectionState.hoveredNode).toEqual(hoverNode);
    });

    it('should clear hovered node when null is passed', () => {
      const { result } = renderHook(() => useSelection());

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(result.current.hoveredNode).not.toBeNull();

      act(() => {
        result.current.setHoveredNode(null);
      });

      expect(result.current.hoveredNode).toBeNull();
    });

    it('should call onHoverChange callback when hover changes', () => {
      const onHoverChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onHoverChange }));

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(onHoverChange).toHaveBeenCalledTimes(1);
      expect(onHoverChange).toHaveBeenCalledWith(hoverNode);
    });

    it('should not call onHoverChange when hovering same node', () => {
      const onHoverChange = vi.fn();
      const { result } = renderHook(() => useSelection({ onHoverChange }));

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      onHoverChange.mockClear();

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(onHoverChange).not.toHaveBeenCalled();
    });

    it('should handle rapid hover changes', () => {
      const { result } = renderHook(() => useSelection());

      const node1 = {
        id: 'hover-1',
        name: 'hover1.ts',
        path: '/hover1.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      const node2 = {
        id: 'hover-2',
        name: 'hover2.ts',
        path: '/hover2.ts',
        x0: 20,
        y0: 20,
        x1: 30,
        y1: 30,
        height: 60,
        color: '#00ff00',
        metrics: { loc: 90, complexity: 5 }
      };

      act(() => {
        result.current.setHoveredNode(node1);
        result.current.setHoveredNode(node2);
        result.current.setHoveredNode(null);
      });

      expect(result.current.hoveredNode).toBeNull();
    });
  });

  describe('isSelected', () => {
    it('should return true for selected node', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.isSelected('building-1')).toBe(true);
    });

    it('should return false for non-selected node', () => {
      const { result } = renderHook(() => useSelection());

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      expect(result.current.isSelected('building-2')).toBe(false);
    });

    it('should return false when nothing is selected', () => {
      const { result } = renderHook(() => useSelection());

      expect(result.current.isSelected('building-1')).toBe(false);
    });
  });

  describe('isHovered', () => {
    it('should return true for hovered node', () => {
      const { result } = renderHook(() => useSelection());

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(result.current.isHovered('hover-1')).toBe(true);
    });

    it('should return false for non-hovered node', () => {
      const { result } = renderHook(() => useSelection());

      const hoverNode = {
        id: 'hover-1',
        name: 'hover.ts',
        path: '/hover.ts',
        x0: 0,
        y0: 0,
        x1: 10,
        y1: 10,
        height: 50,
        color: '#0000ff',
        metrics: { loc: 80, complexity: 4 }
      };

      act(() => {
        result.current.setHoveredNode(hoverNode);
      });

      expect(result.current.isHovered('hover-2')).toBe(false);
    });

    it('should return false when nothing is hovered', () => {
      const { result } = renderHook(() => useSelection());

      expect(result.current.isHovered('hover-1')).toBe(false);
    });
  });

  describe('callback memoization', () => {
    it('should memoize selectBuilding callback', () => {
      const { result, rerender } = renderHook(() => useSelection());

      const callback1 = result.current.selectBuilding;
      rerender();
      const callback2 = result.current.selectBuilding;

      expect(callback1).toBe(callback2);
    });

    it('should memoize clearSelection callback', () => {
      const { result, rerender } = renderHook(() => useSelection());

      const callback1 = result.current.clearSelection;
      rerender();
      const callback2 = result.current.clearSelection;

      expect(callback1).toBe(callback2);
    });

    it('should memoize setHoveredNode callback', () => {
      const { result, rerender } = renderHook(() => useSelection());

      const callback1 = result.current.setHoveredNode;
      rerender();
      const callback2 = result.current.setHoveredNode;

      expect(callback1).toBe(callback2);
    });

    it('should update isSelected when selection changes', () => {
      const { result } = renderHook(() => useSelection());

      const isSelected1 = result.current.isSelected;

      act(() => {
        result.current.selectBuilding(mockBuildingData);
      });

      const isSelected2 = result.current.isSelected;

      // isSelected should be a new function with updated closure
      expect(isSelected1).not.toBe(isSelected2);
    });
  });
});
