/**
 * StatisticsOverlay - React component for displaying rendering statistics and FPS
 * Shows real-time performance metrics and rendering information
 * @module components/visualizations/3d/ui/StatisticsOverlay
 */

import React, { useEffect, useRef, useMemo } from 'react';
import type { RenderStats } from '../types';
import Stats from 'stats.js';

/**
 * Position for the statistics overlay
 */
export type OverlayPosition = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

/**
 * Props for StatisticsOverlay component
 */
export interface StatisticsOverlayProps {
  /** Rendering statistics */
  stats: RenderStats | null;
  /** Overlay position on screen */
  position?: OverlayPosition;
  /** Whether to show FPS counter */
  showFPS?: boolean;
  /** FPS threshold for performance warnings */
  fpsWarningThreshold?: number;
}

/**
 * Gets CSS position styles based on overlay position
 */
function getPositionStyles(position: OverlayPosition): React.CSSProperties {
  const positions: Record<OverlayPosition, React.CSSProperties> = {
    'top-left': { top: '20px', left: '20px' },
    'top-right': { top: '20px', right: '20px' },
    'bottom-left': { bottom: '20px', left: '20px' },
    'bottom-right': { bottom: '20px', right: '20px' },
  };
  return positions[position] || positions['top-right'];
}

/**
 * Formats a number with commas (e.g., 1000 -> 1,000)
 */
function formatNumber(num: number): string {
  return num.toLocaleString();
}

/**
 * Formats memory in MB
 */
function formatMemory(bytes: number): string {
  const mb = bytes / (1024 * 1024);
  return mb < 10 ? mb.toFixed(2) : Math.round(mb).toString();
}

/**
 * StatisticsOverlay component that displays rendering statistics and FPS
 */
export const StatisticsOverlay: React.FC<StatisticsOverlayProps> = ({
  stats,
  position = 'top-right',
  showFPS = true,
  fpsWarningThreshold = 30,
}) => {
  const fpsCounterRef = useRef<Stats | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const lastUpdateRef = useRef<number>(0);
  const currentFPSRef = useRef<number>(60);

  // Initialize FPS counter
  useEffect(() => {
    if (!showFPS || !containerRef.current) {
      return;
    }

    const fpsCounter = new Stats();
    fpsCounter.showPanel(0); // 0: FPS, 1: MS, 2: MB

    // Style the stats panel
    fpsCounter.dom.style.position = 'relative';
    fpsCounter.dom.style.left = 'auto';
    fpsCounter.dom.style.top = 'auto';
    fpsCounter.dom.style.marginTop = '12px';
    fpsCounter.dom.style.border = '1px solid rgba(255, 255, 255, 0.15)';
    fpsCounter.dom.style.borderRadius = '4px';
    fpsCounter.dom.style.overflow = 'hidden';

    containerRef.current.appendChild(fpsCounter.dom);
    fpsCounterRef.current = fpsCounter;

    // Animation loop to update FPS
    let animationId: number;
    const animate = () => {
      if (fpsCounterRef.current) {
        fpsCounterRef.current.begin();
        // Store current FPS for warning display
        const now = performance.now();
        if (now - lastUpdateRef.current >= 16.67) {
          // ~60fps
          const delta = now - lastUpdateRef.current;
          currentFPSRef.current = Math.round(1000 / delta);
          lastUpdateRef.current = now;
        }
        fpsCounterRef.current.end();
      }
      animationId = requestAnimationFrame(animate);
    };
    animationId = requestAnimationFrame(animate);

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
      if (fpsCounter.dom && fpsCounter.dom.parentNode) {
        fpsCounter.dom.parentNode.removeChild(fpsCounter.dom);
      }
      fpsCounterRef.current = null;
    };
  }, [showFPS]);

  // Check for low FPS
  const showFPSWarning = useMemo(
    () => currentFPSRef.current < fpsWarningThreshold,
    [fpsWarningThreshold]
  );

  // Don't render if no stats
  if (!stats) {
    return null;
  }

  const {
    totalBuildings,
    totalVoxels,
    instancedMeshCount,
    voxelSize,
    maxHeight,
    memoryEstimate,
  } = stats;

  // Overlay styles
  const overlayStyles: React.CSSProperties = {
    position: 'fixed',
    backgroundColor: 'rgba(0, 0, 0, 0.75)',
    color: '#ffffff',
    padding: '16px 20px',
    borderRadius: '8px',
    fontFamily:
      '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, monospace',
    fontSize: '13px',
    lineHeight: '1.6',
    minWidth: '240px',
    maxWidth: '320px',
    boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
    border: '1px solid rgba(255, 255, 255, 0.15)',
    backdropFilter: 'blur(8px)',
    zIndex: 999,
    pointerEvents: 'none',
    ...getPositionStyles(position),
  };

  const headerStyles: React.CSSProperties = {
    fontSize: '10px',
    color: 'rgba(255, 255, 255, 0.5)',
    marginBottom: '12px',
    textTransform: 'uppercase',
    letterSpacing: '1px',
  };

  const labelStyles: React.CSSProperties = {
    fontSize: '10px',
    color: 'rgba(255, 255, 255, 0.6)',
    marginBottom: '2px',
  };

  const valueStyles: React.CSSProperties = {
    fontSize: '18px',
    fontWeight: 600,
    fontFamily: '"Courier New", monospace',
  };

  const gridStyles: React.CSSProperties = {
    display: 'grid',
    gridTemplateColumns: '1fr',
    gap: '8px',
    marginBottom: '12px',
  };

  return (
    <div style={overlayStyles} role="status" aria-label="Rendering statistics">
      <div style={headerStyles}>Rendering Statistics</div>

      <div style={gridStyles}>
        <div>
          <div style={labelStyles}>Total Buildings</div>
          <div style={valueStyles}>{formatNumber(totalBuildings)}</div>
        </div>

        <div>
          <div style={labelStyles}>Total Voxels</div>
          <div style={valueStyles}>{formatNumber(totalVoxels)}</div>
        </div>

        <div>
          <div style={labelStyles}>Instanced Meshes</div>
          <div style={valueStyles}>{formatNumber(instancedMeshCount)}</div>
        </div>

        <div>
          <div style={labelStyles}>Voxel Size</div>
          <div style={{ ...valueStyles, fontSize: '14px' }}>{voxelSize.toFixed(2)}</div>
        </div>

        <div>
          <div style={labelStyles}>Max Height</div>
          <div style={{ ...valueStyles, fontSize: '14px' }}>{maxHeight.toFixed(1)}</div>
        </div>

        <div>
          <div style={labelStyles}>Memory Estimate</div>
          <div style={{ ...valueStyles, fontSize: '14px' }}>
            {formatMemory(memoryEstimate)} MB
          </div>
        </div>
      </div>

      {/* FPS Warning */}
      {showFPSWarning && (
        <div
          style={{
            padding: '8px',
            marginBottom: '8px',
            backgroundColor: 'rgba(239, 68, 68, 0.2)',
            border: '1px solid rgba(239, 68, 68, 0.5)',
            borderRadius: '4px',
            fontSize: '11px',
            color: '#ff6b6b',
          }}
          role="alert"
        >
          ⚠ Performance Warning: FPS below {fpsWarningThreshold}
        </div>
      )}

      {/* FPS Counter Container */}
      <div ref={containerRef} />
    </div>
  );
};

export default StatisticsOverlay;
