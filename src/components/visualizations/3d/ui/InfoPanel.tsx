/**
 * InfoPanel - React component for displaying building/file information
 * Shows detailed metrics when a building/file is selected in the 3D visualization
 * @module components/visualizations/3d/ui/InfoPanel
 */

import React, { useMemo } from 'react';
import type { LayoutNode } from '../types';

/**
 * Position for the info panel
 */
export type PanelPosition = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

/**
 * Props for InfoPanel component
 */
export interface InfoPanelProps {
  /** Selected node data to display */
  data: LayoutNode | null;
  /** Panel position on screen */
  position?: PanelPosition;
  /** Maximum path length before truncating */
  maxPathLength?: number;
  /** Callback when panel should close */
  onClose?: () => void;
}

/**
 * Gets CSS position styles based on panel position
 */
function getPositionStyles(position: PanelPosition): React.CSSProperties {
  const positions: Record<PanelPosition, React.CSSProperties> = {
    'top-left': { top: '20px', left: '20px' },
    'top-right': { top: '20px', right: '20px' },
    'bottom-left': { bottom: '20px', left: '20px' },
    'bottom-right': { bottom: '20px', right: '20px' },
  };
  return positions[position] || positions['bottom-left'];
}

/**
 * Truncates a file path if it exceeds maximum length
 */
function truncatePath(path: string, maxLength: number): string {
  if (path.length <= maxLength) {
    return path;
  }

  // Try to keep the filename and some parent directories
  const parts = path.split('/');
  const filename = parts[parts.length - 1];

  if (filename.length > maxLength - 3) {
    // Filename itself is too long
    return '...' + filename.slice(-(maxLength - 3));
  }

  // Keep filename and truncate the beginning
  const remainingLength = maxLength - filename.length - 4; // 4 for ".../"
  const pathStart = path.slice(0, remainingLength);
  return pathStart + '.../' + filename;
}

/**
 * Formats a number with commas (e.g., 1000 -> 1,000)
 */
function formatNumber(num: number): string {
  return num.toLocaleString();
}

/**
 * Formats a date string to readable format
 */
function formatDate(dateString: string | undefined): string {
  if (!dateString) {
    return 'Unknown';
  }

  try {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  } catch {
    return 'Invalid date';
  }
}

/**
 * Gets a color based on complexity score
 */
function getComplexityColor(complexity: number): string {
  if (complexity < 10) {
    return '#22c55e'; // Green
  } else if (complexity < 20) {
    return '#eab308'; // Yellow
  } else {
    return '#ef4444'; // Red
  }
}

/**
 * Gets a descriptive label for complexity
 */
function getComplexityLabel(complexity: number): string {
  if (complexity < 10) {
    return 'Low';
  } else if (complexity < 20) {
    return 'Medium';
  } else if (complexity < 30) {
    return 'High';
  } else {
    return 'Very High';
  }
}

/**
 * InfoPanel component that displays selected file/building information
 */
export const InfoPanel: React.FC<InfoPanelProps> = ({
  data,
  position = 'bottom-left',
  maxPathLength = 50,
  onClose,
}) => {
  // Memoize computed values (must be called before early return)
  const truncatedPath = useMemo(
    () => (data ? truncatePath(data.path, maxPathLength) : ''),
    [data, maxPathLength]
  );
  const complexityColor = useMemo(
    () => (data ? getComplexityColor(data.metrics.complexity) : '#ffffff'),
    [data]
  );
  const complexityLabel = useMemo(
    () => (data ? getComplexityLabel(data.metrics.complexity) : ''),
    [data]
  );
  const formattedDate = useMemo(
    () => (data ? formatDate(data.metrics.lastModified) : ''),
    [data]
  );

  // Don't render if no data
  if (!data) {
    return null;
  }

  const { path, metrics } = data;
  const { loc, complexity, lastModified, functions } = metrics;

  // Panel styles
  const panelStyles: React.CSSProperties = {
    position: 'fixed',
    backgroundColor: 'rgba(0, 0, 0, 0.85)',
    color: '#ffffff',
    padding: '16px 20px',
    borderRadius: '8px',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif',
    fontSize: '14px',
    lineHeight: '1.6',
    minWidth: '280px',
    maxWidth: '400px',
    boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
    border: '1px solid rgba(255, 255, 255, 0.1)',
    backdropFilter: 'blur(10px)',
    zIndex: 10000,
    pointerEvents: 'auto',
    ...getPositionStyles(position),
  };

  const headerStyles: React.CSSProperties = {
    marginBottom: '12px',
    borderBottom: '1px solid rgba(255, 255, 255, 0.2)',
    paddingBottom: '8px',
  };

  const labelStyles: React.CSSProperties = {
    fontSize: '11px',
    color: 'rgba(255, 255, 255, 0.6)',
    marginBottom: '4px',
  };

  const gridStyles: React.CSSProperties = {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '12px',
    marginBottom: '12px',
  };

  const metricValueStyles: React.CSSProperties = {
    fontSize: '20px',
    fontWeight: 600,
  };

  const sectionStyles: React.CSSProperties = {
    marginBottom: '12px',
  };

  const buttonStyles: React.CSSProperties = {
    display: 'inline-block',
    padding: '8px 12px',
    background: 'rgba(255, 255, 255, 0.1)',
    color: '#ffffff',
    textDecoration: 'none',
    borderRadius: '4px',
    fontSize: '12px',
    fontWeight: 500,
    border: 'none',
    cursor: 'pointer',
    transition: 'background 0.2s',
  };

  const footerStyles: React.CSSProperties = {
    marginTop: '12px',
    paddingTop: '8px',
    borderTop: '1px solid rgba(255, 255, 255, 0.1)',
    fontSize: '11px',
    color: 'rgba(255, 255, 255, 0.4)',
  };

  return (
    <div style={panelStyles} role="dialog" aria-label="File information panel">
      {/* File path header */}
      <div style={headerStyles}>
        <div style={labelStyles}>FILE</div>
        <div style={{ fontWeight: 500, wordBreak: 'break-all' }} title={path}>
          {truncatedPath}
        </div>
      </div>

      {/* Main metrics grid */}
      <div style={gridStyles}>
        <div>
          <div style={labelStyles}>LINES OF CODE</div>
          <div style={metricValueStyles}>{formatNumber(loc)}</div>
        </div>

        <div>
          <div style={labelStyles}>COMPLEXITY</div>
          <div style={{ ...metricValueStyles, color: complexityColor }}>
            {complexity}
            <span
              style={{
                fontSize: '11px',
                fontWeight: 400,
                color: 'rgba(255, 255, 255, 0.6)',
                marginLeft: '4px',
              }}
            >
              ({complexityLabel})
            </span>
          </div>
        </div>
      </div>

      {/* Functions count */}
      {functions !== undefined && (
        <div style={sectionStyles}>
          <div style={labelStyles}>FUNCTIONS</div>
          <div style={{ fontSize: '12px', color: 'rgba(255, 255, 255, 0.8)' }}>
            {formatNumber(functions)}
          </div>
        </div>
      )}

      {/* Last modified date */}
      {lastModified && (
        <div style={sectionStyles}>
          <div style={labelStyles}>LAST MODIFIED</div>
          <div style={{ fontSize: '12px', color: 'rgba(255, 255, 255, 0.8)' }}>
            {formattedDate}
          </div>
        </div>
      )}

      {/* Close button */}
      {onClose && (
        <div style={sectionStyles}>
          <button
            onClick={onClose}
            onMouseOver={(e) => {
              (e.target as HTMLButtonElement).style.background = 'rgba(255, 255, 255, 0.2)';
            }}
            onMouseOut={(e) => {
              (e.target as HTMLButtonElement).style.background = 'rgba(255, 255, 255, 0.1)';
            }}
            style={buttonStyles}
            aria-label="Close info panel"
          >
            ✕ Close
          </button>
        </div>
      )}

      {/* Footer hint */}
      <div style={footerStyles}>
        Click elsewhere to deselect
      </div>
    </div>
  );
};

export default InfoPanel;
