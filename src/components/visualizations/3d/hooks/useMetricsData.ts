/**
 * useMetricsData - React hook for loading and managing metrics data
 * Handles data loading lifecycle with loading/error states
 * @module components/visualizations/3d/hooks/useMetricsData
 */

import { useState, useEffect, useCallback, useMemo } from 'react';
import type { HierarchyNode } from '../types';
import {
  loadMetricsFromJSON,
  loadMetricsFromURL,
  loadMetricsFromFile,
  MetricsLoadError,
  getErrorMessage,
} from '../utils/metricsLoader';

/**
 * Loading state enum
 */
export enum LoadingState {
  IDLE = 'idle',
  LOADING = 'loading',
  SUCCESS = 'success',
  ERROR = 'error',
}

/**
 * Data source type
 */
export type DataSource =
  | { type: 'json'; data: string | object }
  | { type: 'url'; url: string }
  | { type: 'file'; path: string };

/**
 * Return type for useMetricsData hook
 */
export interface UseMetricsDataReturn {
  /** Loaded hierarchy data */
  data: HierarchyNode | null;
  /** Current loading state */
  loadingState: LoadingState;
  /** Error if loading failed */
  error: string | null;
  /** Original error object */
  errorDetails: MetricsLoadError | Error | null;
  /** Whether data is currently loading */
  isLoading: boolean;
  /** Whether data loaded successfully */
  isSuccess: boolean;
  /** Whether an error occurred */
  isError: boolean;
  /** Reload the data from the same source */
  reload: () => Promise<void>;
  /** Load data from a new source */
  loadData: (source: DataSource) => Promise<void>;
  /** Clear current data and reset state */
  clear: () => void;
}

/**
 * Options for useMetricsData hook
 */
export interface UseMetricsDataOptions {
  /** Initial data source to load on mount */
  source?: DataSource;
  /** Enable automatic reload in development mode (hot-reload) */
  enableHotReload?: boolean;
  /** Hot reload interval in milliseconds (default: 2000ms) */
  hotReloadInterval?: number;
  /** Callback when data loads successfully */
  onSuccess?: (data: HierarchyNode) => void;
  /** Callback when loading fails */
  onError?: (error: string, details: MetricsLoadError | Error) => void;
}

/**
 * Custom hook for loading and managing metrics data
 *
 * Features:
 * - Manages loading/error states properly
 * - Supports JSON, URL, and file path sources
 * - Development hot-reload support
 * - Memoizes data to prevent unnecessary re-renders
 * - Error boundaries integration ready
 * - Progress indication
 *
 * @example
 * ```tsx
 * // Load from URL parameter
 * const searchParams = new URLSearchParams(window.location.search);
 * const dataUrl = searchParams.get('data');
 *
 * const {
 *   data,
 *   isLoading,
 *   isError,
 *   error,
 *   reload
 * } = useMetricsData({
 *   source: dataUrl ? { type: 'url', url: dataUrl } : undefined,
 *   enableHotReload: import.meta.env.DEV,
 *   onError: (error) => console.error('Failed to load:', error)
 * });
 *
 * if (isLoading) return <LoadingSpinner />;
 * if (isError) return <ErrorDisplay message={error} onRetry={reload} />;
 * if (!data) return <EmptyState />;
 *
 * return <Visualization data={data} />;
 * ```
 */
export function useMetricsData(
  options: UseMetricsDataOptions = {}
): UseMetricsDataReturn {
  const {
    source: initialSource,
    enableHotReload = false,
    hotReloadInterval = 2000,
    onSuccess,
    onError,
  } = options;

  // State
  const [data, setData] = useState<HierarchyNode | null>(null);
  const [loadingState, setLoadingState] = useState<LoadingState>(LoadingState.IDLE);
  const [error, setError] = useState<string | null>(null);
  const [errorDetails, setErrorDetails] = useState<MetricsLoadError | Error | null>(null);
  const [currentSource, setCurrentSource] = useState<DataSource | undefined>(initialSource);

  /**
   * Load data from a source
   */
  const loadFromSource = useCallback(
    async (source: DataSource): Promise<void> => {
      setLoadingState(LoadingState.LOADING);
      setError(null);
      setErrorDetails(null);

      try {
        let result: HierarchyNode;

        switch (source.type) {
          case 'json':
            result = await loadMetricsFromJSON(source.data);
            break;
          case 'url':
            result = await loadMetricsFromURL(source.url);
            break;
          case 'file':
            result = await loadMetricsFromFile(source.path);
            break;
          default:
            throw new Error('Invalid data source type');
        }

        setData(result);
        setLoadingState(LoadingState.SUCCESS);

        if (onSuccess) {
          onSuccess(result);
        }
      } catch (err) {
        const errorMessage = err instanceof MetricsLoadError
          ? getErrorMessage(err)
          : err instanceof Error
          ? err.message
          : 'Unknown error occurred';

        const errorObj = err instanceof Error ? err : new Error(String(err));

        setError(errorMessage);
        setErrorDetails(errorObj);
        setLoadingState(LoadingState.ERROR);

        if (onError) {
          onError(errorMessage, errorObj);
        }
      }
    },
    [onSuccess, onError]
  );

  /**
   * Load data from a new source
   */
  const loadData = useCallback(
    async (source: DataSource): Promise<void> => {
      setCurrentSource(source);
      await loadFromSource(source);
    },
    [loadFromSource]
  );

  /**
   * Reload from the current source
   */
  const reload = useCallback(async (): Promise<void> => {
    if (currentSource) {
      await loadFromSource(currentSource);
    }
  }, [currentSource, loadFromSource]);

  /**
   * Clear data and reset state
   */
  const clear = useCallback(() => {
    setData(null);
    setLoadingState(LoadingState.IDLE);
    setError(null);
    setErrorDetails(null);
    setCurrentSource(undefined);
  }, []);

  // Load initial source on mount
  useEffect(() => {
    if (initialSource) {
      loadFromSource(initialSource);
    }
  }, []); // Only run on mount

  // Hot reload in development
  useEffect(() => {
    if (!enableHotReload || !currentSource) {
      return;
    }

    const interval = setInterval(() => {
      // Only reload if not currently loading
      if (loadingState !== LoadingState.LOADING) {
        loadFromSource(currentSource);
      }
    }, hotReloadInterval);

    return () => clearInterval(interval);
  }, [enableHotReload, currentSource, hotReloadInterval, loadingState, loadFromSource]);

  // Derived state
  const isLoading = loadingState === LoadingState.LOADING;
  const isSuccess = loadingState === LoadingState.SUCCESS;
  const isError = loadingState === LoadingState.ERROR;

  // Memoize return object to prevent unnecessary re-renders
  return useMemo(
    () => ({
      data,
      loadingState,
      error,
      errorDetails,
      isLoading,
      isSuccess,
      isError,
      reload,
      loadData,
      clear,
    }),
    [data, loadingState, error, errorDetails, isLoading, isSuccess, isError, reload, loadData, clear]
  );
}

export default useMetricsData;
