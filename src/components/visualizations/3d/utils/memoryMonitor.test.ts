/**
 * Unit tests for memory monitoring utility
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
  getMemoryInfo,
  estimateSceneMemory,
  checkMemoryBudget,
  formatMemorySize,
  MemoryMonitor,
  type MemoryBudget,
} from './memoryMonitor';

describe('memoryMonitor', () => {
  interface PerformanceWithMemory extends Performance {
    memory?: {
      totalJSHeapSize: number;
      usedJSHeapSize: number;
      jsHeapSizeLimit: number;
    };
  }

  describe('getMemoryInfo', () => {
    it('should return memory info when performance.memory is available', () => {
      // Mock performance.memory
      const mockMemory = {
        totalJSHeapSize: 50 * 1024 * 1024, // 50 MB
        usedJSHeapSize: 30 * 1024 * 1024, // 30 MB
        jsHeapSizeLimit: 100 * 1024 * 1024, // 100 MB
      };

      (global.performance as PerformanceWithMemory).memory = mockMemory;

      const info = getMemoryInfo();

      expect(info.isSupported).toBe(true);
      expect(info.totalHeapSize).toBe(mockMemory.totalJSHeapSize);
      expect(info.usedHeapSize).toBe(mockMemory.usedJSHeapSize);
      expect(info.heapSizeLimit).toBe(mockMemory.jsHeapSizeLimit);
      expect(info.usagePercent).toBe(30); // 30 / 100 * 100

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });

    it('should return fallback when performance.memory is not available', () => {
      // Ensure performance.memory is undefined
      delete (global.performance as PerformanceWithMemory).memory;

      const info = getMemoryInfo();

      expect(info.isSupported).toBe(false);
      expect(info.usagePercent).toBe(0);
      expect(info.heapSizeLimit).toBe(2 * 1024 * 1024 * 1024); // 2GB default
    });
  });

  describe('estimateSceneMemory', () => {
    it('should estimate memory for small scene', () => {
      const stats = {
        totalVoxels: 100,
        instancedMeshCount: 5,
        totalBuildings: 10,
      };

      const memory = estimateSceneMemory(stats);

      expect(memory).toBeGreaterThan(0);
      expect(memory).toBeLessThan(1024 * 1024); // Less than 1 MB for small scene
    });

    it('should estimate higher memory for larger scene', () => {
      const smallStats = {
        totalVoxels: 100,
        instancedMeshCount: 5,
        totalBuildings: 10,
      };

      const largeStats = {
        totalVoxels: 10000,
        instancedMeshCount: 50,
        totalBuildings: 1000,
      };

      const smallMemory = estimateSceneMemory(smallStats);
      const largeMemory = estimateSceneMemory(largeStats);

      expect(largeMemory).toBeGreaterThan(smallMemory * 50); // Should scale significantly
    });

    it('should account for instanced mesh optimization', () => {
      // More voxels but same instance count should use less memory per voxel
      const stats1 = {
        totalVoxels: 1000,
        instancedMeshCount: 10,
        totalBuildings: 100,
      };

      const stats2 = {
        totalVoxels: 1000,
        instancedMeshCount: 5, // Fewer unique geometries
        totalBuildings: 100,
      };

      const memory1 = estimateSceneMemory(stats1);
      const memory2 = estimateSceneMemory(stats2);

      expect(memory2).toBeLessThan(memory1); // Fewer instances = less geometry memory
    });
  });

  describe('checkMemoryBudget', () => {
    it('should return null when usage is below warning threshold', () => {
      const memoryInfo = {
        totalHeapSize: 100 * 1024 * 1024,
        usedHeapSize: 50 * 1024 * 1024,
        heapSizeLimit: 100 * 1024 * 1024,
        usagePercent: 50,
        isSupported: true,
      };

      const budget: MemoryBudget = {
        warningThreshold: 70,
        criticalThreshold: 85,
      };

      const warning = checkMemoryBudget(memoryInfo, budget);
      expect(warning).toBeNull();
    });

    it('should return warning when usage exceeds warning threshold', () => {
      const memoryInfo = {
        totalHeapSize: 100 * 1024 * 1024,
        usedHeapSize: 75 * 1024 * 1024,
        heapSizeLimit: 100 * 1024 * 1024,
        usagePercent: 75,
        isSupported: true,
      };

      const budget: MemoryBudget = {
        warningThreshold: 70,
        criticalThreshold: 85,
      };

      const warning = checkMemoryBudget(memoryInfo, budget);

      expect(warning).not.toBeNull();
      expect(warning?.level).toBe('warning');
      expect(warning?.message).toContain('75');
      expect(warning?.suggestions.length).toBeGreaterThan(0);
    });

    it('should return critical warning when usage exceeds critical threshold', () => {
      const memoryInfo = {
        totalHeapSize: 100 * 1024 * 1024,
        usedHeapSize: 90 * 1024 * 1024,
        heapSizeLimit: 100 * 1024 * 1024,
        usagePercent: 90,
        isSupported: true,
      };

      const budget: MemoryBudget = {
        warningThreshold: 70,
        criticalThreshold: 85,
      };

      const warning = checkMemoryBudget(memoryInfo, budget);

      expect(warning).not.toBeNull();
      expect(warning?.level).toBe('critical');
      expect(warning?.message).toContain('Critical');
      expect(warning?.suggestions.length).toBeGreaterThan(0);
    });

    it('should check max memory limit when configured', () => {
      const memoryInfo = {
        totalHeapSize: 100 * 1024 * 1024,
        usedHeapSize: 60 * 1024 * 1024, // 60 MB
        heapSizeLimit: 100 * 1024 * 1024,
        usagePercent: 60,
        isSupported: true,
      };

      const budget: MemoryBudget = {
        warningThreshold: 70,
        criticalThreshold: 85,
        maxMemoryMB: 50, // Max 50 MB
      };

      const warning = checkMemoryBudget(memoryInfo, budget);

      expect(warning).not.toBeNull();
      expect(warning?.level).toBe('critical');
      expect(warning?.message).toContain('exceeds configured limit');
    });

    it('should return null when memory API not supported and usage is 0', () => {
      const memoryInfo = {
        totalHeapSize: 0,
        usedHeapSize: 0,
        heapSizeLimit: 2 * 1024 * 1024 * 1024,
        usagePercent: 0,
        isSupported: false,
      };

      const warning = checkMemoryBudget(memoryInfo);
      expect(warning).toBeNull();
    });
  });

  describe('formatMemorySize', () => {
    it('should format bytes correctly', () => {
      expect(formatMemorySize(0)).toBe('0 B');
      expect(formatMemorySize(500)).toBe('500 B');
    });

    it('should format kilobytes correctly', () => {
      expect(formatMemorySize(1024)).toBe('1 KB');
      expect(formatMemorySize(1536)).toBe('2 KB'); // Rounded
    });

    it('should format megabytes correctly', () => {
      expect(formatMemorySize(1024 * 1024)).toBe('1.0 MB');
      expect(formatMemorySize(5.5 * 1024 * 1024)).toBe('5.5 MB');
    });

    it('should format gigabytes correctly', () => {
      expect(formatMemorySize(1024 * 1024 * 1024)).toBe('1.0 GB');
      expect(formatMemorySize(2.5 * 1024 * 1024 * 1024)).toBe('2.5 GB');
    });
  });

  describe('MemoryMonitor', () => {
    let monitor: MemoryMonitor;

    beforeEach(() => {
      vi.useFakeTimers();
      monitor = new MemoryMonitor();
    });

    afterEach(() => {
      monitor.stop();
      vi.restoreAllMocks();
    });

    it('should start and stop monitoring', () => {
      monitor.start(1000);
      expect(vi.getTimerCount()).toBeGreaterThan(0);

      monitor.stop();
      expect(vi.getTimerCount()).toBe(0);
    });

    it('should not start multiple monitoring intervals', () => {
      monitor.start(1000);
      const timerCount1 = vi.getTimerCount();

      monitor.start(1000); // Try starting again
      const timerCount2 = vi.getTimerCount();

      expect(timerCount1).toBe(timerCount2); // Should not create another timer
    });

    it('should register and unregister callbacks', () => {
      const callback = vi.fn();
      const unsubscribe = monitor.onWarning(callback);

      // Mock memory info to trigger warning
      const mockMemory = {
        totalJSHeapSize: 100 * 1024 * 1024,
        usedJSHeapSize: 80 * 1024 * 1024,
        jsHeapSizeLimit: 100 * 1024 * 1024,
      };
      (global.performance as PerformanceWithMemory).memory = mockMemory;

      monitor.start(100);
      vi.advanceTimersByTime(100);

      expect(callback).toHaveBeenCalled();

      // Unsubscribe and check callback not called again
      unsubscribe();
      callback.mockClear();
      vi.advanceTimersByTime(100);

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });

    it('should get current memory info', () => {
      const mockMemory = {
        totalJSHeapSize: 50 * 1024 * 1024,
        usedJSHeapSize: 30 * 1024 * 1024,
        jsHeapSizeLimit: 100 * 1024 * 1024,
      };
      (global.performance as PerformanceWithMemory).memory = mockMemory;

      const info = monitor.getCurrentInfo();

      expect(info.isSupported).toBe(true);
      expect(info.usagePercent).toBe(30);

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });

    it('should get current warning when threshold exceeded', () => {
      const mockMemory = {
        totalJSHeapSize: 100 * 1024 * 1024,
        usedJSHeapSize: 90 * 1024 * 1024,
        jsHeapSizeLimit: 100 * 1024 * 1024,
      };
      (global.performance as PerformanceWithMemory).memory = mockMemory;

      monitor.updateBudget({ warningThreshold: 70, criticalThreshold: 85 });
      const warning = monitor.getCurrentWarning();

      expect(warning).not.toBeNull();
      expect(warning?.level).toBe('critical');

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });

    it('should update budget configuration', () => {
      monitor.updateBudget({
        warningThreshold: 60,
        criticalThreshold: 80,
      });

      const mockMemory = {
        totalJSHeapSize: 100 * 1024 * 1024,
        usedJSHeapSize: 65 * 1024 * 1024,
        jsHeapSizeLimit: 100 * 1024 * 1024,
      };
      (global.performance as PerformanceWithMemory).memory = mockMemory;

      const warning = monitor.getCurrentWarning();
      expect(warning).not.toBeNull();
      expect(warning?.level).toBe('warning'); // 65% > 60% warning threshold

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });

    it('should only notify callbacks when warning level changes', () => {
      const callback = vi.fn();
      monitor.onWarning(callback);

      const mockMemory = {
        totalJSHeapSize: 100 * 1024 * 1024,
        usedJSHeapSize: 80 * 1024 * 1024,
        jsHeapSizeLimit: 100 * 1024 * 1024,
      };
      (global.performance as PerformanceWithMemory).memory = mockMemory;

      monitor.start(100);

      // First check should trigger callback
      vi.advanceTimersByTime(100);
      expect(callback).toHaveBeenCalledTimes(1);

      // Second check with same level should not trigger
      callback.mockClear();
      vi.advanceTimersByTime(100);
      expect(callback).not.toHaveBeenCalled();

      // Cleanup
      delete (global.performance as PerformanceWithMemory).memory;
    });
  });
});
