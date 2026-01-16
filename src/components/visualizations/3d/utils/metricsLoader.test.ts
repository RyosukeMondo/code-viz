/**
 * Tests for metricsLoader
 * @module components/visualizations/3d/utils/metricsLoader.test
 */

import { describe, it, expect } from 'vitest';
import {
  loadMetricsFromJSON,
  MetricsLoadError,
  getErrorMessage,
} from './metricsLoader';

describe('metricsLoader', () => {
  describe('loadMetricsFromJSON', () => {
    it('should load valid code-viz JSON', async () => {
      const validData = {
        summary: {
          total_files: 2,
          total_loc: 20,
          total_functions: 4,
          largest_files: ['src/main.rs', 'src/lib.rs'],
        },
        files: [
          {
            path: 'src/main.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
            last_modified: '2025-01-01T00:00:00Z',
          },
          {
            path: 'src/lib.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
        ],
      };

      const result = await loadMetricsFromJSON(validData);

      expect(result).toBeDefined();
      expect(result.type).toBe('directory');
      expect(result.name).toBe('root');
      expect(result.children).toBeDefined();
      expect(result.children?.length).toBeGreaterThan(0);
    });

    it('should build correct hierarchy structure', async () => {
      const data = {
        summary: {
          total_files: 3,
          total_loc: 30,
          total_functions: 6,
          largest_files: [],
        },
        files: [
          {
            path: 'src/main.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
          {
            path: 'src/utils/helper.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
          {
            path: 'tests/test_main.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
        ],
      };

      const result = await loadMetricsFromJSON(data);

      // Should have src and tests directories
      expect(result.children?.length).toBe(2);

      const srcDir = result.children?.find(c => c.name === 'src');
      expect(srcDir).toBeDefined();
      expect(srcDir?.type).toBe('directory');
      expect(srcDir?.children?.length).toBe(2); // main.rs and utils directory
    });

    it('should transform metrics correctly', async () => {
      const data = {
        summary: {
          total_files: 1,
          total_loc: 10,
          total_functions: 2,
          largest_files: [],
        },
        files: [
          {
            path: 'src/main.rs',
            language: 'rust',
            loc: 100,
            size_bytes: 2000,
            function_count: 5,
            cognitive_complexity: {
              total_complexity: 25,
              average_complexity: 5,
              max_complexity: 10,
            },
            code_churn: {
              added_lines: 50,
              deleted_lines: 30,
            },
          },
        ],
      };

      const result = await loadMetricsFromJSON(data);

      const srcDir = result.children?.find(c => c.name === 'src');
      const mainFile = srcDir?.children?.find(c => c.name === 'main.rs');

      expect(mainFile?.metrics).toBeDefined();
      expect(mainFile?.metrics?.loc).toBe(100);
      expect(mainFile?.metrics?.functions).toBe(5);
      expect(mainFile?.metrics?.complexity).toBe(25);
      expect(mainFile?.metrics?.churn).toBe(80); // 50 + 30
    });

    it('should handle files without complexity metrics', async () => {
      const data = {
        summary: {
          total_files: 1,
          total_loc: 10,
          total_functions: 2,
          largest_files: [],
        },
        files: [
          {
            path: 'src/simple.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
        ],
      };

      const result = await loadMetricsFromJSON(data);

      const srcDir = result.children?.find(c => c.name === 'src');
      const file = srcDir?.children?.find(c => c.name === 'simple.rs');

      expect(file?.metrics?.complexity).toBe(0);
      expect(file?.metrics?.churn).toBeUndefined();
    });

    it('should parse JSON string', async () => {
      const jsonString = JSON.stringify({
        summary: {
          total_files: 1,
          total_loc: 10,
          total_functions: 2,
          largest_files: [],
        },
        files: [
          {
            path: 'test.rs',
            language: 'rust',
            loc: 10,
            size_bytes: 200,
            function_count: 2,
          },
        ],
      });

      const result = await loadMetricsFromJSON(jsonString);

      expect(result).toBeDefined();
      expect(result.type).toBe('directory');
    });

    it('should throw error for invalid JSON string', async () => {
      const invalidJSON = '{ invalid json }';

      await expect(loadMetricsFromJSON(invalidJSON)).rejects.toThrow(
        MetricsLoadError
      );

      try {
        await loadMetricsFromJSON(invalidJSON);
      } catch (error) {
        expect(error).toBeInstanceOf(MetricsLoadError);
        expect((error as MetricsLoadError).code).toBe('JSON_PARSE_ERROR');
      }
    });

    it('should throw error for invalid format', async () => {
      const invalidData = {
        not_summary: {},
        not_files: [],
      };

      await expect(loadMetricsFromJSON(invalidData)).rejects.toThrow(
        MetricsLoadError
      );

      try {
        await loadMetricsFromJSON(invalidData);
      } catch (error) {
        expect(error).toBeInstanceOf(MetricsLoadError);
        expect((error as MetricsLoadError).code).toBe('INVALID_FORMAT');
      }
    });

    it('should throw error for empty results', async () => {
      const emptyData = {
        summary: {
          total_files: 0,
          total_loc: 0,
          total_functions: 0,
          largest_files: [],
        },
        files: [],
      };

      await expect(loadMetricsFromJSON(emptyData)).rejects.toThrow(
        MetricsLoadError
      );

      try {
        await loadMetricsFromJSON(emptyData);
      } catch (error) {
        expect(error).toBeInstanceOf(MetricsLoadError);
        expect((error as MetricsLoadError).code).toBe('EMPTY_RESULT');
      }
    });
  });

  describe('getErrorMessage', () => {
    it('should return user-friendly message for JSON parse error', () => {
      const error = new MetricsLoadError(
        'Parse failed',
        'JSON_PARSE_ERROR'
      );
      const message = getErrorMessage(error);

      expect(message).toContain('not valid JSON');
    });

    it('should return user-friendly message for invalid format', () => {
      const error = new MetricsLoadError(
        'Invalid format',
        'INVALID_FORMAT'
      );
      const message = getErrorMessage(error);

      expect(message).toContain('does not match');
    });

    it('should return user-friendly message for empty result', () => {
      const error = new MetricsLoadError(
        'No files',
        'EMPTY_RESULT'
      );
      const message = getErrorMessage(error);

      expect(message).toContain('no files');
    });

    it('should return generic message for unknown errors', () => {
      const error = new Error('Unknown error');
      const message = getErrorMessage(error);

      expect(message).toContain('unexpected error');
    });
  });
});
