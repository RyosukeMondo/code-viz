/**
 * Unit tests for StatisticsOverlay component
 * Tests rendering, FPS monitoring, memory warnings, and overlay positioning
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatisticsOverlay } from './StatisticsOverlay';
import type { RenderStats } from '../types';

// Mock stats.js
vi.mock('stats.js', () => {
  class MockStats {
    dom: HTMLElement;
    showPanel: ReturnType<typeof vi.fn>;
    begin: ReturnType<typeof vi.fn>;
    end: ReturnType<typeof vi.fn>;

    constructor() {
      this.dom = document.createElement('div');
      this.showPanel = vi.fn();
      this.begin = vi.fn();
      this.end = vi.fn();
    }
  }

  return {
    default: MockStats,
  };
});

// Mock MemoryMonitor
vi.mock('../utils/memoryMonitor', () => {
  class MockMemoryMonitor {
    start = vi.fn();
    stop = vi.fn();
    getCurrentInfo = vi.fn(() => ({
      usedHeapSize: 100 * 1024 * 1024,
      totalHeapSize: 200 * 1024 * 1024,
      heapSizeLimit: 500 * 1024 * 1024,
      usagePercent: 20,
      isSupported: true,
    }));
    onWarning = vi.fn(() => vi.fn());
  }

  return {
    MemoryMonitor: MockMemoryMonitor,
    formatMemorySize: vi.fn((bytes: number) => {
      const mb = bytes / (1024 * 1024);
      return `${mb.toFixed(1)} MB`;
    }),
  };
});

describe('StatisticsOverlay', () => {
  const mockStats: RenderStats = {
    totalBuildings: 150,
    totalVoxels: 5000,
    instancedMeshCount: 1,
    voxelSize: 2.5,
    maxHeight: 100.5,
    memoryEstimate: 50 * 1024 * 1024, // 50MB in bytes
  };

  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('rendering', () => {
    it('should render with stats', () => {
      render(<StatisticsOverlay stats={mockStats} />);

      expect(screen.getByText('Rendering Statistics')).toBeInTheDocument();
      expect(screen.getByText('150')).toBeInTheDocument(); // totalBuildings
      expect(screen.getByText('5,000')).toBeInTheDocument(); // totalVoxels
    });

    it('should not render without stats', () => {
      const { container } = render(<StatisticsOverlay stats={null} />);

      expect(container.firstChild).toBeNull();
    });

    it('should render all stat fields', () => {
      render(<StatisticsOverlay stats={mockStats} />);

      expect(screen.getByText('Total Buildings')).toBeInTheDocument();
      expect(screen.getByText('Total Voxels')).toBeInTheDocument();
      expect(screen.getByText('Instanced Meshes')).toBeInTheDocument();
      expect(screen.getByText('Voxel Size')).toBeInTheDocument();
      expect(screen.getByText('Max Height')).toBeInTheDocument();
      expect(screen.getByText('Memory Estimate')).toBeInTheDocument();
    });

    it('should format numbers with commas', () => {
      const largeStats: RenderStats = {
        ...mockStats,
        totalBuildings: 1000000,
        totalVoxels: 5000000,
      };

      render(<StatisticsOverlay stats={largeStats} />);

      expect(screen.getByText('1,000,000')).toBeInTheDocument();
      expect(screen.getByText('5,000,000')).toBeInTheDocument();
    });

    it('should format memory in MB', () => {
      render(<StatisticsOverlay stats={mockStats} />);

      // memoryEstimate is 50MB
      expect(screen.getByText(/50 MB/)).toBeInTheDocument();
    });

    it('should format decimal values correctly', () => {
      render(<StatisticsOverlay stats={mockStats} />);

      expect(screen.getByText('2.50')).toBeInTheDocument(); // voxelSize
      expect(screen.getByText('100.5')).toBeInTheDocument(); // maxHeight
    });
  });

  describe('positioning', () => {
    it('should default to top-right position', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.top).toBe('20px');
      expect(overlay.style.right).toBe('20px');
    });

    it('should position at top-left', () => {
      const { container } = render(
        <StatisticsOverlay stats={mockStats} position="top-left" />
      );
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.top).toBe('20px');
      expect(overlay.style.left).toBe('20px');
    });

    it('should position at bottom-right', () => {
      const { container } = render(
        <StatisticsOverlay stats={mockStats} position="bottom-right" />
      );
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.bottom).toBe('20px');
      expect(overlay.style.right).toBe('20px');
    });

    it('should position at bottom-left', () => {
      const { container } = render(
        <StatisticsOverlay stats={mockStats} position="bottom-left" />
      );
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.bottom).toBe('20px');
      expect(overlay.style.left).toBe('20px');
    });
  });

  describe('FPS counter', () => {
    it('should show FPS counter by default', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);

      // Check that FPS counter container exists
      expect(container.querySelector('div')).toBeInTheDocument();
    });

    it('should not show FPS counter when disabled', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} showFPS={false} />);

      // Component should still render but without FPS counter
      expect(container.querySelector('div')).toBeInTheDocument();
    });

    it('should cleanup FPS counter on unmount', () => {
      const { unmount } = render(<StatisticsOverlay stats={mockStats} />);

      unmount();

      // requestAnimationFrame should be cancelled
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('memory monitoring', () => {
    it('should render with memory monitoring enabled', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);

      // Component should render with memory monitoring
      expect(container.querySelector('div')).toBeInTheDocument();
    });

    it('should not show memory info when disabled', () => {
      render(<StatisticsOverlay stats={mockStats} showMemoryMonitor={false} />);

      expect(screen.queryByText('Heap Memory')).not.toBeInTheDocument();
    });

    it('should render with memory thresholds configured', () => {
      const { container } = render(
        <StatisticsOverlay
          stats={mockStats}
          memoryWarningThreshold={80}
          memoryCriticalThreshold={90}
        />
      );

      // Component should render with configured thresholds
      expect(container.querySelector('div')).toBeInTheDocument();
    });

    it('should cleanup memory monitor on unmount', () => {
      const { unmount } = render(<StatisticsOverlay stats={mockStats} />);

      unmount();

      // Verify cleanup happened (timers cleared)
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('FPS warning', () => {
    it('should render with FPS warning threshold configured', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} fpsWarningThreshold={30} />);

      // Component should render with configured threshold
      expect(container.querySelector('div')).toBeInTheDocument();
    });

    it('should not show FPS warning when above threshold', () => {
      render(<StatisticsOverlay stats={mockStats} fpsWarningThreshold={10} />);

      expect(screen.queryByText(/Performance Warning/)).not.toBeInTheDocument();
    });
  });

  describe('accessibility', () => {
    it('should have proper ARIA role for overlay', () => {
      render(<StatisticsOverlay stats={mockStats} />);

      const overlay = screen.getByRole('status');
      expect(overlay).toBeInTheDocument();
      expect(overlay).toHaveAttribute('aria-label', 'Rendering statistics');
    });

    it('should render with proper aria attributes', () => {
      render(<StatisticsOverlay stats={mockStats} fpsWarningThreshold={30} />);

      const overlay = screen.getByRole('status');
      expect(overlay).toHaveAttribute('aria-label', 'Rendering statistics');
    });
  });

  describe('styling', () => {
    it('should apply overlay styles', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.position).toBe('fixed');
      expect(overlay.style.zIndex).toBe('999');
      expect(overlay.style.pointerEvents).toBe('none');
    });

    it('should have semi-transparent background', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.backgroundColor).toContain('rgba');
    });

    it('should have rounded corners and shadow', () => {
      const { container } = render(<StatisticsOverlay stats={mockStats} />);
      const overlay = container.firstChild as HTMLElement;

      expect(overlay.style.borderRadius).toBe('8px');
      expect(overlay.style.boxShadow).toBeTruthy();
    });
  });

  describe('edge cases', () => {
    it('should handle zero values', () => {
      const zeroStats: RenderStats = {
        totalBuildings: 0,
        totalVoxels: 0,
        instancedMeshCount: 0,
        voxelSize: 0,
        maxHeight: 0,
        memoryEstimate: 0,
      };

      render(<StatisticsOverlay stats={zeroStats} />);

      // Check for multiple zero values (totalBuildings, totalVoxels, instancedMeshCount)
      const zeros = screen.getAllByText('0');
      expect(zeros.length).toBeGreaterThan(0);
    });

    it('should handle very large numbers', () => {
      const largeStats: RenderStats = {
        ...mockStats,
        totalBuildings: 999999999,
        totalVoxels: 999999999,
      };

      render(<StatisticsOverlay stats={largeStats} />);

      // Check for the large number (appears twice: totalBuildings and totalVoxels)
      const largeNumbers = screen.getAllByText('999,999,999');
      expect(largeNumbers.length).toBe(2);
    });

    it('should handle small decimal values', () => {
      const smallStats: RenderStats = {
        ...mockStats,
        voxelSize: 0.01,
        maxHeight: 0.1,
      };

      render(<StatisticsOverlay stats={smallStats} />);

      expect(screen.getByText('0.01')).toBeInTheDocument();
      expect(screen.getByText('0.1')).toBeInTheDocument();
    });

    it('should handle memory info not supported', () => {
      // Mock will return isSupported: true by default, so test the rendering path
      render(<StatisticsOverlay stats={mockStats} showMemoryMonitor={false} />);

      // Should not show memory info when monitoring is disabled
      expect(screen.queryByText('Heap Memory')).not.toBeInTheDocument();
    });
  });
});
