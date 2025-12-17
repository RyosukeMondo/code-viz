/**
 * Zustand store for managing global analysis state
 *
 * This store manages the state of the code analysis visualization, including:
 * - The hierarchical tree of code metrics (TreeNode)
 * - Current drill-down path for treemap navigation
 * - Selected file for detail panel display
 * - Loading and error states
 *
 * The store uses Zustand's immer middleware for immutable state updates.
 */

import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type { TreeNode } from '../types/bindings';

/**
 * Analysis state interface
 */
interface AnalysisState {
  /** Root TreeNode from repository analysis (null if not yet analyzed) */
  metrics: TreeNode | null;

  /** Current drill-down path as array of node IDs (empty = root view) */
  drillDownPath: string[];

  /** Currently selected file for detail panel (null if none selected) */
  selectedFile: TreeNode | null;

  /** Loading state during analysis */
  loading: boolean;

  /** Error message if analysis failed (null if no error) */
  error: string | null;
}

/**
 * Analysis actions interface
 */
interface AnalysisActions {
  /** Set the root metrics tree after successful analysis */
  setMetrics: (metrics: TreeNode | null) => void;

  /** Update the drill-down path for treemap navigation */
  setDrillDownPath: (path: string[]) => void;

  /** Set the selected file for detail panel display */
  setSelectedFile: (file: TreeNode | null) => void;

  /** Set loading state */
  setLoading: (loading: boolean) => void;

  /** Set error message */
  setError: (error: string | null) => void;

  /** Reset all state to initial values */
  reset: () => void;
}

/**
 * Complete store type combining state and actions
 */
type AnalysisStore = AnalysisState & AnalysisActions;

/**
 * Initial state values
 */
const initialState: AnalysisState = {
  metrics: null,
  drillDownPath: [],
  selectedFile: null,
  loading: false,
  error: null,
};

/**
 * Zustand store for analysis state management
 *
 * Uses immer middleware for immutable state updates without spread operators.
 * All state updates are handled through actions to maintain consistency.
 *
 * @example
 * ```typescript
 * // In a React component
 * import { useAnalysisStore } from './store/analysisStore';
 *
 * function MyComponent() {
 *   const metrics = useAnalysisStore(state => state.metrics);
 *   const setMetrics = useAnalysisStore(state => state.setMetrics);
 *   const loading = useAnalysisStore(state => state.loading);
 *
 *   // Use the store...
 * }
 * ```
 */
export const useAnalysisStore = create<AnalysisStore>()(
  immer((set) => ({
    // Initial state
    ...initialState,

    // Actions
    setMetrics: (metrics) =>
      set((state) => {
        state.metrics = metrics;
        // Clear drill-down path and selected file when new metrics loaded
        state.drillDownPath = [];
        state.selectedFile = null;
        state.error = null;
      }),

    setDrillDownPath: (path) =>
      set((state) => {
        state.drillDownPath = path;
        // Clear selected file when navigating
        state.selectedFile = null;
      }),

    setSelectedFile: (file) =>
      set((state) => {
        state.selectedFile = file;
      }),

    setLoading: (loading) =>
      set((state) => {
        state.loading = loading;
        // Clear error when starting new load
        if (loading) {
          state.error = null;
        }
      }),

    setError: (error) =>
      set((state) => {
        state.error = error;
        state.loading = false;
      }),

    reset: () => set(initialState),
  }))
);

/**
 * Selector hooks for common state selections
 *
 * These hooks provide optimized access to specific parts of the store,
 * minimizing unnecessary re-renders.
 */

/** Hook to get metrics state */
export const useMetrics = () => useAnalysisStore((state) => state.metrics);

/** Hook to get drill-down path */
export const useDrillDownPath = () =>
  useAnalysisStore((state) => state.drillDownPath);

/** Hook to get selected file */
export const useSelectedFile = () =>
  useAnalysisStore((state) => state.selectedFile);

/** Hook to get loading state */
export const useLoading = () => useAnalysisStore((state) => state.loading);

/** Hook to get error state */
export const useError = () => useAnalysisStore((state) => state.error);

/** Hook to get all actions */
export const useAnalysisActions = () =>
  useAnalysisStore((state) => ({
    setMetrics: state.setMetrics,
    setDrillDownPath: state.setDrillDownPath,
    setSelectedFile: state.setSelectedFile,
    setLoading: state.setLoading,
    setError: state.setError,
    reset: state.reset,
  }));
