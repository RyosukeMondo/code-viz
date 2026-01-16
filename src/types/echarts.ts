/**
 * ECharts event parameter types
 *
 * These types define the structure of event parameters passed by ECharts
 * for various user interactions like clicks, hovers, and selections.
 */

/**
 * Base interface for ECharts event parameters
 */
export interface EChartsBaseParams {
  /** Component type (e.g., 'series') */
  componentType: string;

  /** Series type (e.g., 'treemap', 'sunburst') */
  seriesType?: string;

  /** Index of the series */
  seriesIndex?: number;

  /** Index of the data item */
  dataIndex?: number;

  /** Name of the data item */
  name: string;
}

/**
 * Tree path info for hierarchical visualizations
 */
export interface TreePathInfo {
  /** Node name */
  name: string;

  /** Data index */
  dataIndex: number;

  /** Node value (e.g., LOC) */
  value: number;

  /** Full path */
  path?: string;

  /** Node type */
  type?: 'file' | 'directory';

  /** Complexity score */
  complexity?: number;

  /** Child nodes */
  children?: unknown[];
}

/**
 * Click event parameters for ECharts treemap
 */
export interface EChartsTreemapClickParams extends EChartsBaseParams {
  /** Data object associated with the clicked item */
  data?: {
    name: string;
    value: number;
    path: string;
    type: 'file' | 'directory';
    complexity?: number;
    children?: unknown[];
    deadCodeRatio?: number;
  };

  /** Tree path information (path from root to clicked node) */
  treePathInfo?: TreePathInfo[];

  /** Rectangle dimensions */
  rect?: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
}

/**
 * Hover/mouseover event parameters
 */
export interface EChartsHoverParams extends EChartsBaseParams {
  /** Data object associated with the hovered item */
  data?: {
    name: string;
    value: number;
    path: string;
    type: 'file' | 'directory';
    complexity?: number;
    children?: unknown[];
  };
}

/**
 * Tooltip formatter parameters
 */
export interface EChartsTooltipParams {
  /** Item name */
  name: string;

  /** Item value */
  value: number;

  /** Data object */
  data: {
    name: string;
    value: number;
    path?: string;
    type?: 'file' | 'directory';
    complexity?: number;
    deadCodeRatio?: number;
  };

  /** Series name */
  seriesName?: string;

  /** Data index */
  dataIndex?: number;
}

/**
 * Label formatter parameters
 */
export interface EChartsLabelParams {
  /** Item name */
  name: string;

  /** Item value */
  value: number;

  /** Data object */
  data: {
    name: string;
    value: number;
    type?: 'file' | 'directory';
  };

  /** Rectangle dimensions (for treemap) */
  rect?: {
    x: number;
    y: number;
    width: number;
    height: number;
  };

  /** Symbol size (for circle packing) */
  symbolSize?: number;
}

/**
 * ItemStyle parameters (for conditional styling)
 */
export interface EChartsItemStyleParams {
  /** Data object */
  data?: {
    deadCodeRatio?: number;
    [key: string]: unknown;
  };
}
