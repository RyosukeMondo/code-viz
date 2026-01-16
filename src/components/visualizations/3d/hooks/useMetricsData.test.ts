/**
 * Tests for useMetricsData hook
 * @module components/visualizations/3d/hooks/useMetricsData.test
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, waitFor, act } from '@testing-library/react';
import { useMetricsData, LoadingState } from './useMetricsData';
import type { DataSource } from './useMetricsData';

// Mock the metricsLoader module
vi.mock('../utils/metricsLoader', () => ({
  loadMetricsFromJSON: vi.fn(),
  loadMetricsFromURL: vi.fn(),
  loadMetricsFromFile: vi.fn(),
  MetricsLoadError: class MetricsLoadError extends Error {
    constructor(message: string, public code: string, public details?: unknown) {
      super(message);
      this.name = 'MetricsLoadError';
    }
  },
  getErrorMessage: vi.fn((error: Error) => error.message),
}));

const mockMetricsLoader = await import('../utils/metricsLoader');

const mockHierarchyData = {
  name: 'root',
  type: 'directory' as const,
  path: '/',
  children: [
    {
      name: 'test.ts',
      type: 'file' as const,
      path: '/test.ts',
      metrics: {
        loc: 100,
        complexity: 10,
        functions: 5,
        lastModified: '2025-01-01T00:00:00Z',
      },
    },
  ],
};

describe('useMetricsData', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('initialization', () => {
    it('should initialize with idle state when no source provided', () => {
      const { result } = renderHook(() => useMetricsData());

      expect(result.current.loadingState).toBe(LoadingState.IDLE);
      expect(result.current.data).toBeNull();
      expect(result.current.error).toBeNull();
      expect(result.current.isLoading).toBe(false);
      expect(result.current.isSuccess).toBe(false);
      expect(result.current.isError).toBe(false);
    });

    it('should start loading when initial source provided', async () => {
      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockResolvedValue(
        mockHierarchyData
      );

      const source: DataSource = {
        type: 'json',
        data: { summary: {}, files: [] },
      };

      const { result } = renderHook(() => useMetricsData({ source }));

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(result.current.data).toEqual(mockHierarchyData);
    });
  });

  describe('loadData', () => {
    it('should load data from JSON successfully', async () => {
      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockResolvedValue(
        mockHierarchyData
      );

      const { result } = renderHook(() => useMetricsData());

      const source: DataSource = {
        type: 'json',
        data: { summary: {}, files: [] },
      };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(result.current.data).toEqual(mockHierarchyData);
      expect(result.current.error).toBeNull();
    });

    it('should load data from URL successfully', async () => {
      vi.mocked(mockMetricsLoader.loadMetricsFromURL).mockResolvedValue(
        mockHierarchyData
      );

      const { result } = renderHook(() => useMetricsData());

      const source: DataSource = {
        type: 'url',
        url: 'https://example.com/data.json',
      };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(result.current.data).toEqual(mockHierarchyData);
    });

    it('should load data from file successfully', async () => {
      vi.mocked(mockMetricsLoader.loadMetricsFromFile).mockResolvedValue(
        mockHierarchyData
      );

      const { result } = renderHook(() => useMetricsData());

      const source: DataSource = { type: 'file', path: '/path/to/data.json' };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(result.current.data).toEqual(mockHierarchyData);
    });
  });

  describe('error handling', () => {
    it('should handle MetricsLoadError correctly', async () => {
      const error = new mockMetricsLoader.MetricsLoadError(
        'Invalid format',
        'INVALID_FORMAT'
      );

      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockRejectedValue(error);
      vi.mocked(mockMetricsLoader.getErrorMessage).mockReturnValue(
        'The file does not match the expected format'
      );

      const { result } = renderHook(() => useMetricsData());

      const source: DataSource = { type: 'json', data: {} };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isError).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(result.current.error).toBe(
        'The file does not match the expected format'
      );
      expect(result.current.data).toBeNull();
    });

    it('should call onError callback when loading fails', async () => {
      const onError = vi.fn();
      const error = new Error('Test error');

      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockRejectedValue(error);

      const { result } = renderHook(() => useMetricsData({ onError }));

      const source: DataSource = { type: 'json', data: {} };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isError).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(onError).toHaveBeenCalledWith('Test error', error);
    });
  });

  describe('callbacks', () => {
    it('should call onSuccess callback when data loads', async () => {
      const onSuccess = vi.fn();

      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockResolvedValue(
        mockHierarchyData
      );

      const { result } = renderHook(() => useMetricsData({ onSuccess }));

      const source: DataSource = { type: 'json', data: {} };

      result.current.loadData(source);

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      expect(onSuccess).toHaveBeenCalledWith(mockHierarchyData);
    });
  });

  describe('clear', () => {
    it('should clear data and reset state', async () => {
      vi.mocked(mockMetricsLoader.loadMetricsFromJSON).mockResolvedValue(
        mockHierarchyData
      );

      const source: DataSource = { type: 'json', data: {} };

      const { result } = renderHook(() => useMetricsData({ source }));

      await waitFor(
        () => {
          expect(result.current.isSuccess).toBe(true);
        },
        { timeout: 3000 }
      );

      act(() => {
        result.current.clear();
      });

      expect(result.current.data).toBeNull();
      expect(result.current.loadingState).toBe(LoadingState.IDLE);
      expect(result.current.error).toBeNull();
    });
  });
});
