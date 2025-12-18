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
import type { TreeNode, DeadCodeResult } from '../types/bindings';

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

  /** Dead code overlay enabled flag (opt-in feature) */
  deadCodeEnabled: boolean;

  /** Dead code analysis results (null if not yet analyzed) */
  deadCodeResults: DeadCodeResult | null;

  /** Loading state during dead code analysis */
  deadCodeLoading: boolean;

  /** Error message if dead code analysis failed (null if no error) */
  deadCodeError: string | null;
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

  /** Toggle dead code overlay on/off */
  toggleDeadCodeOverlay: () => void;

  /** Set dead code analysis results */
  setDeadCodeResults: (results: DeadCodeResult | null) => void;

  /** Set dead code loading state */
  setDeadCodeLoading: (loading: boolean) => void;

  /** Set dead code error message */
  setDeadCodeError: (error: string | null) => void;

  /** Reset dead code state */
  resetDeadCode: () => void;
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
  deadCodeEnabled: false,
  deadCodeResults: null,
  deadCodeLoading: false,
  deadCodeError: null,
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

    toggleDeadCodeOverlay: () =>
      set((state) => {
        state.deadCodeEnabled = !state.deadCodeEnabled;
      }),

    setDeadCodeResults: (results) =>
      set((state) => {
        state.deadCodeResults = results;
        state.deadCodeError = null;
      }),

    setDeadCodeLoading: (loading) =>
      set((state) => {
        state.deadCodeLoading = loading;
        if (loading) {
          state.deadCodeError = null;
        }
      }),

    setDeadCodeError: (error) =>
      set((state) => {
        state.deadCodeError = error;
        state.deadCodeLoading = false;
      }),

    resetDeadCode: () =>
      set((state) => {
        state.deadCodeEnabled = false;
        state.deadCodeResults = null;
        state.deadCodeLoading = false;
        state.deadCodeError = null;
      }),
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

/** Hook to get dead code enabled state */
export const useDeadCodeEnabled = () =>
  useAnalysisStore((state) => state.deadCodeEnabled);

/** Hook to get dead code results */
export const useDeadCodeResults = () =>
  useAnalysisStore((state) => state.deadCodeResults);

/** Hook to get dead code loading state */
export const useDeadCodeLoading = () =>
  useAnalysisStore((state) => state.deadCodeLoading);

/** Hook to get dead code error state */
export const useDeadCodeError = () =>
  useAnalysisStore((state) => state.deadCodeError);

/** Hook to get all actions */
export const useAnalysisActions = () =>
  useAnalysisStore((state) => ({
    setMetrics: state.setMetrics,
    setDrillDownPath: state.setDrillDownPath,
    setSelectedFile: state.setSelectedFile,
    setLoading: state.setLoading,
    setError: state.setError,
    reset: state.reset,
    toggleDeadCodeOverlay: state.toggleDeadCodeOverlay,
    setDeadCodeResults: state.setDeadCodeResults,
    setDeadCodeLoading: state.setDeadCodeLoading,
    setDeadCodeError: state.setDeadCodeError,
    resetDeadCode: state.resetDeadCode,
  }));
