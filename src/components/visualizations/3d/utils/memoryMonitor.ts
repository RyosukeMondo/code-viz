/**
 * Memory monitoring utility for 3D visualization
 * Estimates memory usage and provides warnings when approaching limits
 * @module components/visualizations/3d/utils/memoryMonitor
 */

/**
 * Memory usage information
 */
export interface MemoryInfo {
  /** Total JS heap size in bytes */
  totalHeapSize: number;
  /** Used JS heap size in bytes */
  usedHeapSize: number;
  /** JS heap size limit in bytes */
  heapSizeLimit: number;
  /** Percentage of heap used (0-100) */
  usagePercent: number;
  /** Whether browser supports memory API */
  isSupported: boolean;
}

/**
 * Memory warning with severity level
 */
export interface MemoryWarning {
  /** Warning severity level */
  level: 'info' | 'warning' | 'critical';
  /** Warning message */
  message: string;
  /** Suggested actions to reduce memory usage */
  suggestions: string[];
}

/**
 * Memory budget configuration
 */
export interface MemoryBudget {
  /** Warning threshold as percentage (default: 70) */
  warningThreshold: number;
  /** Critical threshold as percentage (default: 85) */
  criticalThreshold: number;
  /** Maximum allowed memory in MB (optional) */
  maxMemoryMB?: number;
}

/**
 * Default memory budget configuration
 */
const DEFAULT_BUDGET: MemoryBudget = {
  warningThreshold: 70,
  criticalThreshold: 85,
};

/**
 * Performance memory interface for browsers that support it
 */
interface PerformanceMemory {
  totalJSHeapSize: number;
  usedJSHeapSize: number;
  jsHeapSizeLimit: number;
}

/**
 * Extended performance interface with optional memory property
 */
interface PerformanceWithMemory extends Performance {
  memory?: PerformanceMemory;
}

/**
 * Gets current memory information from browser
 * Falls back to estimation if performance.memory is not available
 */
export function getMemoryInfo(): MemoryInfo {
  // Check if performance.memory is available (Chrome/Edge only)
  const perfMemory = (performance as PerformanceWithMemory).memory;

  if (perfMemory) {
    const usagePercent = (perfMemory.usedJSHeapSize / perfMemory.jsHeapSizeLimit) * 100;

    return {
      totalHeapSize: perfMemory.totalJSHeapSize,
      usedHeapSize: perfMemory.usedJSHeapSize,
      heapSizeLimit: perfMemory.jsHeapSizeLimit,
      usagePercent: Math.round(usagePercent * 100) / 100,
      isSupported: true,
    };
  }

  // Fallback for browsers without performance.memory
  // Estimate based on typical heap limits
  const estimatedLimit = 2 * 1024 * 1024 * 1024; // 2GB default for most browsers
  const estimatedUsed = 0; // Can't estimate without API

  return {
    totalHeapSize: 0,
    usedHeapSize: estimatedUsed,
    heapSizeLimit: estimatedLimit,
    usagePercent: 0,
    isSupported: false,
  };
}

/**
 * Estimates Three.js scene memory usage based on geometry and materials
 */
export function estimateSceneMemory(stats: {
  totalVoxels: number;
  instancedMeshCount: number;
  totalBuildings: number;
}): number {
  const { totalVoxels, instancedMeshCount, totalBuildings } = stats;

  // Estimate geometry memory (vertices, indices, UVs, normals)
  // BoxGeometry has 24 vertices (4 per face * 6 faces)
  const bytesPerVertex = 4 * 3; // 3 floats (x, y, z) * 4 bytes per float
  const bytesPerNormal = 4 * 3; // Same as vertex
  const bytesPerUV = 4 * 2; // 2 floats (u, v) * 4 bytes
  const bytesPerIndex = 2; // Uint16 typically
  const indicesPerBox = 36; // 6 faces * 2 triangles * 3 vertices

  const geometryMemoryPerVoxel =
    24 * bytesPerVertex + 24 * bytesPerNormal + 24 * bytesPerUV + indicesPerBox * bytesPerIndex;

  // For instanced meshes, geometry is shared but matrices are per-instance
  const bytesPerMatrix = 4 * 16; // 4x4 matrix * 4 bytes per float
  const instancedGeometryMemory = geometryMemoryPerVoxel * instancedMeshCount;
  const instanceMatrixMemory = totalVoxels * bytesPerMatrix;

  // Material memory (relatively small, shared across instances)
  const bytesPerMaterial = 1024; // Rough estimate for material data
  const materialMemory = instancedMeshCount * bytesPerMaterial;

  // Additional overhead for scene graph, objects, etc.
  const sceneOverhead = totalBuildings * 512; // ~512 bytes per building object

  const totalMemory =
    instancedGeometryMemory + instanceMatrixMemory + materialMemory + sceneOverhead;

  return totalMemory;
}

/**
 * Checks memory usage and returns warning if thresholds exceeded
 */
export function checkMemoryBudget(
  memoryInfo: MemoryInfo,
  budget: MemoryBudget = DEFAULT_BUDGET
): MemoryWarning | null {
  // Can't warn if memory API not supported and usage is 0
  if (!memoryInfo.isSupported && memoryInfo.usagePercent === 0) {
    return null;
  }

  const { usagePercent } = memoryInfo;
  const { warningThreshold, criticalThreshold, maxMemoryMB } = budget;

  // Check if we've exceeded max allowed memory
  if (maxMemoryMB) {
    const usedMB = memoryInfo.usedHeapSize / (1024 * 1024);
    if (usedMB > maxMemoryMB) {
      return {
        level: 'critical',
        message: `Memory usage (${usedMB.toFixed(0)}MB) exceeds configured limit (${maxMemoryMB}MB)`,
        suggestions: [
          'Reduce voxel count by increasing voxel size',
          'Lower the maximum building height setting',
          'Filter out smaller files from visualization',
          'Consider analyzing a subset of your codebase',
        ],
      };
    }
  }

  // Check percentage thresholds
  if (usagePercent >= criticalThreshold) {
    return {
      level: 'critical',
      message: `Critical memory usage: ${usagePercent.toFixed(1)}% of heap limit`,
      suggestions: [
        'Immediately reduce voxel count by increasing voxel size',
        'Lower maximum building height to reduce voxel count',
        'Close other browser tabs to free memory',
        'Restart browser if experiencing slowness',
      ],
    };
  }

  if (usagePercent >= warningThreshold) {
    return {
      level: 'warning',
      message: `High memory usage: ${usagePercent.toFixed(1)}% of heap limit`,
      suggestions: [
        'Consider increasing voxel size to reduce memory usage',
        'Lower maximum building height if not needed',
        'Monitor performance for potential slowdowns',
      ],
    };
  }

  return null;
}

/**
 * Formats memory size in bytes to human-readable string
 */
export function formatMemorySize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const units = ['B', 'KB', 'MB', 'GB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = bytes / Math.pow(k, i);

  return `${size.toFixed(i >= 2 ? 1 : 0)} ${units[i]}`;
}

/**
 * Memory monitor class for continuous monitoring
 */
export class MemoryMonitor {
  private budget: MemoryBudget;
  private callbacks: Set<(warning: MemoryWarning | null) => void>;
  private intervalId: number | null;
  private lastWarningLevel: string | null;

  constructor(budget: MemoryBudget = DEFAULT_BUDGET) {
    this.budget = budget;
    this.callbacks = new Set();
    this.intervalId = null;
    this.lastWarningLevel = null;
  }

  /**
   * Start monitoring memory at specified interval
   * @param intervalMs Monitoring interval in milliseconds (default: 5000)
   */
  start(intervalMs: number = 5000): void {
    if (this.intervalId !== null) {
      return; // Already monitoring
    }

    this.intervalId = window.setInterval(() => {
      this.check();
    }, intervalMs);

    // Initial check
    this.check();
  }

  /**
   * Stop monitoring memory
   */
  stop(): void {
    if (this.intervalId !== null) {
      clearInterval(this.intervalId);
      this.intervalId = null;
    }
  }

  /**
   * Register callback for memory warnings
   */
  onWarning(callback: (warning: MemoryWarning | null) => void): () => void {
    this.callbacks.add(callback);
    return () => this.callbacks.delete(callback);
  }

  /**
   * Check current memory status and notify callbacks
   */
  private check(): void {
    const memoryInfo = getMemoryInfo();
    const warning = checkMemoryBudget(memoryInfo, this.budget);

    // Only notify if warning level changed or became null
    const currentLevel = warning?.level || null;
    if (currentLevel !== this.lastWarningLevel) {
      this.lastWarningLevel = currentLevel;
      this.callbacks.forEach((callback) => callback(warning));
    }
  }

  /**
   * Update memory budget configuration
   */
  updateBudget(budget: Partial<MemoryBudget>): void {
    this.budget = { ...this.budget, ...budget };
  }

  /**
   * Get current memory info
   */
  getCurrentInfo(): MemoryInfo {
    return getMemoryInfo();
  }

  /**
   * Get current warning if any
   */
  getCurrentWarning(): MemoryWarning | null {
    const memoryInfo = getMemoryInfo();
    return checkMemoryBudget(memoryInfo, this.budget);
  }
}
