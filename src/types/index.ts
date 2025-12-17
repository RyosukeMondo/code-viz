/**
 * TypeScript type definitions for code-viz frontend
 *
 * This file contains supplementary types for component props, state management,
 * and UI-specific interfaces that complement the auto-generated Tauri bindings.
 */

// Import types from bindings for internal use
import type { TreeNode } from './bindings';

// Re-export all types from bindings for convenient single import
export type { TreeNode } from './bindings';
export { analyzeRepository } from './bindings';

/**
 * ECharts-compatible treemap data format
 *
 * This format is used to transform TreeNode data for ECharts consumption.
 * ECharts expects a slightly different structure with 'value' instead of 'loc'
 * and some additional metadata for visualization.
 */
export interface EChartsTreemapNode {
  /** Node name for display */
  name: string;

  /** Numeric value determining rectangle size (maps to LOC) */
  value: number;

  /** Complexity score for color mapping (0-100) */
  complexity: number;

  /** Full path for identification and drill-down */
  path: string;

  /** Node type: "file" or "directory" */
  type: "file" | "directory";

  /** Child nodes (recursive structure) */
  children?: EChartsTreemapNode[];

  /** Additional item style for color override */
  itemStyle?: {
    color?: string;
  };
}

/**
 * Component Props Types
 */

/** Props for the Breadcrumb component */
export interface BreadcrumbProps {
  /** Array of path segments representing the drill-down path */
  path: string[];

  /** Callback fired when a breadcrumb segment is clicked */
  onNavigate: (index: number) => void;
}

/** Props for the DetailPanel component */
export interface DetailPanelProps {
  /** Selected file/directory node to display details for (null to hide panel) */
  file: TreeNode | null;

  /** Callback fired when the close button is clicked */
  onClose: () => void;
}

/** Props for the Treemap component */
export interface TreemapProps {
  /** Root node data to visualize */
  data: TreeNode;

  /** Current drill-down path for filtering */
  drillDownPath?: string[];

  /** Callback fired when a node is clicked */
  onNodeClick?: (node: TreeNode) => void;

  /** Callback fired when a node is hovered */
  onNodeHover?: (node: TreeNode | null) => void;

  /** Callback fired when Escape key is pressed (navigate back) */
  onNavigateBack?: () => void;

  /** Width of the chart (defaults to 100%) */
  width?: string | number;

  /** Height of the chart (defaults to 600px) */
  height?: string | number;
}

/** Props for the AnalysisView feature component */
export interface AnalysisViewProps {
  /** Optional initial repository path */
  initialPath?: string;
}

/**
 * State Management Types (for Zustand store)
 */

/** Analysis state interface */
export interface AnalysisState {
  /** Root tree node data from analysis */
  metrics: TreeNode | null;

  /** Current drill-down path (array of directory names) */
  drillDownPath: string[];

  /** Currently selected file/directory node */
  selectedFile: TreeNode | null;

  /** Loading state during analysis */
  loading: boolean;

  /** Error message if analysis fails */
  error: string | null;
}

/** Analysis state actions */
export interface AnalysisActions {
  /** Set the metrics data */
  setMetrics: (metrics: TreeNode | null) => void;

  /** Set the drill-down path */
  setDrillDownPath: (path: string[]) => void;

  /** Set the selected file */
  setSelectedFile: (file: TreeNode | null) => void;

  /** Set loading state */
  setLoading: (loading: boolean) => void;

  /** Set error state */
  setError: (error: string | null) => void;

  /** Reset all state to initial values */
  reset: () => void;
}

/** Combined store type */
export type AnalysisStore = AnalysisState & AnalysisActions;

/**
 * Hook Return Types
 */

/** Return type for useTauriCommand hook */
export interface UseTauriCommandResult<T> {
  /** Execute the command */
  execute: (...args: unknown[]) => Promise<T>;

  /** Loading state */
  loading: boolean;

  /** Error state */
  error: string | null;

  /** Result data */
  data: T | null;
}

/** Return type for useAnalysis hook */
export interface UseAnalysisResult {
  /** Execute analysis for a given path */
  analyze: (path: string) => Promise<void>;

  /** Re-fetch the current analysis */
  refetch: () => Promise<void>;

  /** Loading state */
  loading: boolean;

  /** Error state */
  error: string | null;

  /** Current repository path being analyzed */
  currentPath: string | null;
}

/**
 * Utility Types
 */

/** Color gradient stop for complexity visualization */
export interface ColorStop {
  /** Score threshold (0-100) */
  score: number;

  /** Hex color code */
  color: string;
}

/** Format options for path truncation */
export interface PathFormatOptions {
  /** Maximum length before truncation */
  maxLength?: number;

  /** Truncation position: 'start' | 'middle' | 'end' */
  position?: 'start' | 'middle' | 'end';

  /** Ellipsis string (defaults to '...') */
  ellipsis?: string;
}

/** Number format options */
export interface NumberFormatOptions {
  /** Use compact notation (K, M, B) */
  compact?: boolean;

  /** Number of decimal places */
  decimals?: number;

  /** Locale for formatting (defaults to 'en-US') */
  locale?: string;
}
