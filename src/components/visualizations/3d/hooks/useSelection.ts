/**
 * useSelection - React hook for managing 3D visualization selection state
 * Provides centralized selection and hover state management
 * @module components/visualizations/3d/hooks/useSelection
 */

import { useState, useCallback, useMemo } from 'react';
import type { LayoutNode, SelectionState } from '../types';
import type { BuildingData } from '../scene/RaycasterHandler';

/**
 * Selection change callback type
 */
export type SelectionChangeCallback = (node: LayoutNode | null) => void;

/**
 * Hover change callback type
 */
export type HoverChangeCallback = (node: LayoutNode | null) => void;

/**
 * Return type for useSelection hook
 */
export interface UseSelectionReturn {
  /** Current selection state */
  selectionState: SelectionState;
  /** Selected node (convenience accessor) */
  selectedNode: LayoutNode | null;
  /** Hovered node (convenience accessor) */
  hoveredNode: LayoutNode | null;
  /** Select a building/node */
  selectBuilding: (building: BuildingData | null) => void;
  /** Clear current selection */
  clearSelection: () => void;
  /** Set hovered node */
  setHoveredNode: (node: LayoutNode | null) => void;
  /** Check if a node is selected */
  isSelected: (nodeId: string) => boolean;
  /** Check if a node is hovered */
  isHovered: (nodeId: string) => boolean;
}

/**
 * Options for useSelection hook
 */
export interface UseSelectionOptions {
  /** Callback when selection changes */
  onSelectionChange?: SelectionChangeCallback;
  /** Callback when hover state changes */
  onHoverChange?: HoverChangeCallback;
  /** Initial selected node */
  initialSelected?: LayoutNode | null;
}

/**
 * Converts BuildingData to LayoutNode
 * RaycasterHandler returns BuildingData, but we work with LayoutNode internally
 */
function buildingDataToLayoutNode(building: BuildingData): LayoutNode {
  return {
    id: building.id,
    name: building.name,
    path: building.path,
    x0: building.position.x,
    y0: building.position.y,
    x1: building.position.x + building.dimensions.width,
    y1: building.position.y + building.dimensions.depth,
    height: building.dimensions.height,
    color: '#ffffff', // Default color - will be overridden by actual color mapping
    metrics: building.metrics,
  };
}

/**
 * Custom hook for managing selection state in 3D visualization
 *
 * Features:
 * - Manages selected and hovered node state
 * - Provides memoized callbacks for performance
 * - Integrates with RaycasterHandler via selectBuilding callback
 * - Handles edge cases (null selection, rapid changes)
 * - Type-safe API
 *
 * @example
 * ```tsx
 * const {
 *   selectedNode,
 *   hoveredNode,
 *   selectBuilding,
 *   clearSelection,
 *   isSelected
 * } = useSelection({
 *   onSelectionChange: (node) => console.log('Selected:', node?.path),
 *   onHoverChange: (node) => console.log('Hovering:', node?.path)
 * });
 *
 * // Pass selectBuilding to RaycasterHandler constructor
 * const handler = new RaycasterHandler(canvas, camera, mesh, renderer, selectBuilding);
 * ```
 */
export function useSelection(options: UseSelectionOptions = {}): UseSelectionReturn {
  const { onSelectionChange, onHoverChange, initialSelected = null } = options;

  // Selection state
  const [selectedNode, setSelectedNodeInternal] = useState<LayoutNode | null>(initialSelected);
  const [hoveredNode, setHoveredNodeInternal] = useState<LayoutNode | null>(null);

  // Memoized selection state object
  const selectionState = useMemo<SelectionState>(
    () => ({
      selectedNode,
      hoveredNode,
    }),
    [selectedNode, hoveredNode]
  );

  /**
   * Selects a building - compatible with RaycasterHandler callback
   * This is the main callback that should be passed to RaycasterHandler
   */
  const selectBuilding = useCallback(
    (building: BuildingData | null) => {
      const newNode = building ? buildingDataToLayoutNode(building) : null;

      // Only update if selection actually changed
      setSelectedNodeInternal((prev) => {
        // Check if selection changed
        const selectionChanged = prev?.id !== newNode?.id;

        if (!selectionChanged) {
          return prev;
        }

        // Call change callback
        if (onSelectionChange) {
          onSelectionChange(newNode);
        }

        return newNode;
      });
    },
    [onSelectionChange]
  );

  /**
   * Clears the current selection
   */
  const clearSelection = useCallback(() => {
    setSelectedNodeInternal((prev) => {
      if (prev === null) {
        return prev;
      }

      // Call change callback
      if (onSelectionChange) {
        onSelectionChange(null);
      }

      return null;
    });
  }, [onSelectionChange]);

  /**
   * Sets the hovered node
   */
  const setHoveredNode = useCallback(
    (node: LayoutNode | null) => {
      setHoveredNodeInternal((prev) => {
        // Only update if hover changed
        const hoverChanged = prev?.id !== node?.id;

        if (!hoverChanged) {
          return prev;
        }

        // Call change callback
        if (onHoverChange) {
          onHoverChange(node);
        }

        return node;
      });
    },
    [onHoverChange]
  );

  /**
   * Checks if a node is currently selected
   */
  const isSelected = useCallback(
    (nodeId: string): boolean => {
      return selectedNode?.id === nodeId;
    },
    [selectedNode]
  );

  /**
   * Checks if a node is currently hovered
   */
  const isHovered = useCallback(
    (nodeId: string): boolean => {
      return hoveredNode?.id === nodeId;
    },
    [hoveredNode]
  );

  return {
    selectionState,
    selectedNode,
    hoveredNode,
    selectBuilding,
    clearSelection,
    setHoveredNode,
    isSelected,
    isHovered,
  };
}

export default useSelection;
