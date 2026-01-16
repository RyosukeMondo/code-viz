/**
 * Metrics loader for transforming code-viz JSON to 3D hierarchy format
 * @module components/visualizations/3d/utils/metricsLoader
 */

import type { HierarchyNode, FileMetrics } from '../types';
import { validateHierarchyNode } from './schemas';

/**
 * Code-viz analysis result format
 */
interface CodeVizFileMetrics {
  path: string;
  language: string;
  loc: number;
  size_bytes: number;
  function_count: number;
  last_modified?: string;
  cognitive_complexity?: {
    total_complexity: number;
    average_complexity: number;
    max_complexity: number;
  };
  code_churn?: {
    added_lines: number;
    deleted_lines: number;
  };
}

interface CodeVizAnalysisResult {
  summary: {
    total_files: number;
    total_loc: number;
    total_functions: number;
    largest_files: string[];
  };
  files: CodeVizFileMetrics[];
  timestamp?: string;
}

/**
 * Error class for metrics loading errors
 */
export class MetricsLoadError extends Error {
  constructor(
    message: string,
    public readonly code: string,
    public readonly details?: unknown
  ) {
    super(message);
    this.name = 'MetricsLoadError';
  }
}

/**
 * Transform code-viz file metrics to FileMetrics format
 */
function transformFileMetrics(file: CodeVizFileMetrics): FileMetrics {
  const complexity = file.cognitive_complexity?.total_complexity ?? 0;
  const churn = file.code_churn
    ? file.code_churn.added_lines + file.code_churn.deleted_lines
    : undefined;

  return {
    loc: file.loc,
    complexity,
    functions: file.function_count,
    lastModified: file.last_modified ?? new Date().toISOString(),
    churn,
  };
}

/**
 * Build directory hierarchy from flat file list
 */
function buildHierarchy(files: CodeVizFileMetrics[]): HierarchyNode {
  const root: HierarchyNode = {
    name: 'root',
    type: 'directory',
    path: '/',
    children: [],
  };

  for (const file of files) {
    const parts = file.path.split('/').filter(p => p.length > 0);
    let current = root;

    // Build directory structure
    for (let i = 0; i < parts.length - 1; i++) {
      const dirName = parts[i];
      const dirPath = '/' + parts.slice(0, i + 1).join('/');

      let dirNode = current.children?.find(
        c => c.name === dirName && c.type === 'directory'
      );

      if (!dirNode) {
        dirNode = {
          name: dirName,
          type: 'directory',
          path: dirPath,
          children: [],
        };
        current.children = current.children || [];
        current.children.push(dirNode);
      }

      current = dirNode;
    }

    // Add file node
    const fileName = parts[parts.length - 1];
    const fileNode: HierarchyNode = {
      name: fileName,
      type: 'file',
      path: '/' + parts.join('/'),
      metrics: transformFileMetrics(file),
    };

    current.children = current.children || [];
    current.children.push(fileNode);
  }

  return root;
}

/**
 * Validate code-viz analysis result structure
 */
function validateCodeVizFormat(data: unknown): data is CodeVizAnalysisResult {
  if (!data || typeof data !== 'object') {
    return false;
  }

  const result = data as CodeVizAnalysisResult;

  if (!result.summary || typeof result.summary !== 'object') {
    return false;
  }

  if (typeof result.summary.total_files !== 'number') {
    return false;
  }

  if (!Array.isArray(result.files)) {
    return false;
  }

  // Validate at least one file entry
  if (result.files.length > 0) {
    const file = result.files[0];
    if (
      typeof file.path !== 'string' ||
      typeof file.loc !== 'number' ||
      typeof file.function_count !== 'number'
    ) {
      return false;
    }
  }

  return true;
}

/**
 * Load and transform code-viz JSON to hierarchy format
 */
export async function loadMetricsFromJSON(
  jsonData: string | object
): Promise<HierarchyNode> {
  try {
    // Parse JSON if string
    const data =
      typeof jsonData === 'string' ? JSON.parse(jsonData) : jsonData;

    // Validate format
    if (!validateCodeVizFormat(data)) {
      throw new MetricsLoadError(
        'Invalid code-viz analysis format',
        'INVALID_FORMAT',
        { received: typeof data }
      );
    }

    // Check for empty results
    if (data.files.length === 0) {
      throw new MetricsLoadError(
        'No files found in analysis result',
        'EMPTY_RESULT',
        { summary: data.summary }
      );
    }

    // Build hierarchy
    const hierarchy = buildHierarchy(data.files);

    // Validate resulting hierarchy
    if (!validateHierarchyNode(hierarchy)) {
      throw new MetricsLoadError(
        'Failed to build valid hierarchy from analysis data',
        'INVALID_HIERARCHY',
        { nodeCount: data.files.length }
      );
    }

    return hierarchy;
  } catch (error) {
    if (error instanceof MetricsLoadError) {
      throw error;
    }

    if (error instanceof SyntaxError) {
      throw new MetricsLoadError(
        'Failed to parse JSON: ' + error.message,
        'JSON_PARSE_ERROR',
        { originalError: error }
      );
    }

    throw new MetricsLoadError(
      'Unknown error loading metrics: ' + String(error),
      'UNKNOWN_ERROR',
      { originalError: error }
    );
  }
}

/**
 * Load metrics from URL
 */
export async function loadMetricsFromURL(url: string): Promise<HierarchyNode> {
  try {
    const response = await fetch(url);

    if (!response.ok) {
      throw new MetricsLoadError(
        `HTTP ${response.status}: ${response.statusText}`,
        'HTTP_ERROR',
        { status: response.status, statusText: response.statusText }
      );
    }

    const data = await response.json();
    return loadMetricsFromJSON(data);
  } catch (error) {
    if (error instanceof MetricsLoadError) {
      throw error;
    }

    if (error instanceof TypeError) {
      throw new MetricsLoadError(
        'Network error: Failed to fetch from URL',
        'NETWORK_ERROR',
        { url, originalError: error }
      );
    }

    throw new MetricsLoadError(
      'Failed to load metrics from URL: ' + String(error),
      'URL_LOAD_ERROR',
      { url, originalError: error }
    );
  }
}

/**
 * Load metrics from file path (for development/testing)
 */
export async function loadMetricsFromFile(
  filePath: string
): Promise<HierarchyNode> {
  try {
    const response = await fetch(filePath);

    if (!response.ok) {
      throw new MetricsLoadError(
        'Failed to load file',
        'FILE_NOT_FOUND',
        { filePath }
      );
    }

    const data = await response.json();
    return loadMetricsFromJSON(data);
  } catch (error) {
    if (error instanceof MetricsLoadError) {
      throw error;
    }

    throw new MetricsLoadError(
      'Failed to load metrics from file: ' + String(error),
      'FILE_LOAD_ERROR',
      { filePath, originalError: error }
    );
  }
}

/**
 * Get user-friendly error message from MetricsLoadError
 */
export function getErrorMessage(error: unknown): string {
  if (error instanceof MetricsLoadError) {
    switch (error.code) {
      case 'JSON_PARSE_ERROR':
        return 'The file is not valid JSON. Please check the file format.';
      case 'INVALID_FORMAT':
        return 'The file does not match the expected code-viz analysis format.';
      case 'EMPTY_RESULT':
        return 'The analysis result contains no files to visualize.';
      case 'INVALID_HIERARCHY':
        return 'Failed to build a valid hierarchy from the analysis data.';
      case 'HTTP_ERROR':
        return `Failed to load from URL: ${error.message}`;
      case 'NETWORK_ERROR':
        return 'Network error. Please check your internet connection.';
      case 'FILE_NOT_FOUND':
        return 'File not found. Please check the file path.';
      default:
        return `Error loading metrics: ${error.message}`;
    }
  }

  return 'An unexpected error occurred while loading metrics.';
}
