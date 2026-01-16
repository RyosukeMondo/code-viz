/**
 * Voxel3DView - Main component for 3D code visualization
 * Integrates Three.js scene, metrics data loading, and UI components
 * @module components/visualizations/Voxel3DView
 */

import React, { useEffect, useCallback, useState, useRef } from 'react';
import { useThreeScene } from './3d/hooks/useThreeScene';
import { useMetricsData, type DataSource } from './3d/hooks/useMetricsData';
import { useSelection } from './3d/hooks/useSelection';
import { InfoPanel } from './3d/ui/InfoPanel';
import { StatisticsOverlay } from './3d/ui/StatisticsOverlay';
import { ConfigPanel, loadSettings, type Config3DSettings } from './3d/ui/ConfigPanel';
import { VoxelRenderer } from './3d/scene/VoxelRenderer';
import { TreemapLayout } from './3d/layout/TreemapLayout';
import type { RenderStats } from './3d/types';

/**
 * Props for Voxel3DView component
 */
export interface Voxel3DViewProps {
  /** Data source for metrics */
  dataSource?: DataSource;
  /** Project key for camera persistence */
  projectKey?: string;
  /** Whether to show statistics overlay */
  showStatistics?: boolean;
  /** Whether to show info panel */
  showInfoPanel?: boolean;
  /** Target FPS for rendering */
  targetFPS?: number;
  /** Callback when data loads successfully */
  onDataLoaded?: () => void;
  /** Callback when an error occurs */
  onError?: (error: string) => void;
}

/**
 * Keyboard shortcuts
 */
const SHORTCUTS = {
  TOGGLE_INFO: 'i',
  TOGGLE_STATS: 's',
  TOGGLE_CONFIG: 'c',
  CLEAR_SELECTION: 'Escape',
} as const;

/**
 * Main 3D visualization component
 *
 * Integrates:
 * - Three.js scene via useThreeScene
 * - Metrics data loading via useMetricsData
 * - Selection state via useSelection
 * - VoxelRenderer for voxel rendering
 * - InfoPanel for selected file details
 * - StatisticsOverlay for performance stats
 *
 * Features:
 * - Automatic scene initialization and cleanup
 * - Loading and error states
 * - Keyboard shortcuts (I: info, S: stats, ESC: clear)
 * - Responsive rendering
 * - Performance monitoring
 */
export const Voxel3DView: React.FC<Voxel3DViewProps> = ({
  dataSource,
  projectKey = 'default',
  showStatistics = true,
  showInfoPanel = true,
  targetFPS = 60,
  onDataLoaded,
  onError,
}) => {
  // UI state
  const [infoPanelVisible, setInfoPanelVisible] = useState(showInfoPanel);
  const [statsVisible, setStatsVisible] = useState(showStatistics);
  const [configVisible, setConfigVisible] = useState(false);
  const [renderStats, setRenderStats] = useState<RenderStats | null>(null);

  // Config settings (load from localStorage on mount)
  const [configSettings, setConfigSettings] = useState<Config3DSettings>(() => loadSettings());

  // Refs for renderer and layout
  const voxelRendererRef = useRef<VoxelRenderer | null>(null);
  const layoutRef = useRef<TreemapLayout | null>(null);

  // Load metrics data
  const {
    data: metricsData,
    isLoading,
    isError,
    error: metricsError,
  } = useMetricsData({
    source: dataSource,
    onSuccess: () => {
      if (onDataLoaded) {
        onDataLoaded();
      }
    },
    onError: (errorMsg) => {
      if (onError) {
        onError(errorMsg);
      }
    },
  });

  // Selection state
  const {
    selectedNode,
    clearSelection,
  } = useSelection({
    onSelectionChange: (node) => {
      console.log('Selection changed:', node?.path);
    },
  });

  // Initialize Three.js scene
  const {
    canvasRef,
    sceneManager,
    isInitialized: sceneInitialized,
    error: sceneError,
  } = useThreeScene({
    projectKey,
    targetFPS,
    antialias: configSettings.antialias,
    shadowsEnabled: configSettings.shadowsEnabled,
    onInitialized: () => {
      console.log('Scene initialized successfully');

      // Create layout calculator with world dimensions
      // Default to 400x400 world size
      layoutRef.current = new TreemapLayout(400, 400);
    },
    onError: (err) => {
      console.error('Scene initialization error:', err);
      if (onError) {
        onError(err.message);
      }
    },
  });

  // Create voxel renderer when scene and data are ready, or recreate when settings change
  useEffect(() => {
    if (!sceneManager || !sceneInitialized || !metricsData || !layoutRef.current) {
      return;
    }

    console.log('Creating/updating voxel renderer...');

    try {
      // Dispose existing renderer
      if (voxelRendererRef.current) {
        voxelRendererRef.current.dispose();
        voxelRendererRef.current = null;
      }

      // Calculate layout
      const layoutNodes = layoutRef.current.compute(metricsData);

      // Get scene from scene manager
      const scene = sceneManager.getScene();
      if (!scene) {
        throw new Error('Scene not initialized');
      }

      // Create renderer with config settings
      const renderer = new VoxelRenderer(scene, {
        voxelSize: configSettings.voxelSize,
        maxHeight: configSettings.maxHeight,
        maxVoxels: 50000,
      });

      // Render voxels
      renderer.render(layoutNodes);

      // Update render stats
      const stats = renderer.getStats();
      setRenderStats(stats);

      // Store renderer ref
      voxelRendererRef.current = renderer;

      console.log('Voxel renderer created/updated:', stats);
    } catch (err) {
      console.error('Failed to create voxel renderer:', err);
      if (onError) {
        onError(err instanceof Error ? err.message : 'Failed to render visualization');
      }
    }
  }, [sceneManager, sceneInitialized, metricsData, configSettings.voxelSize, configSettings.maxHeight, onError]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyPress = (event: KeyboardEvent) => {
      // Ignore if typing in input
      if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
        return;
      }

      switch (event.key.toLowerCase()) {
        case SHORTCUTS.TOGGLE_INFO:
          setInfoPanelVisible((v) => !v);
          break;
        case SHORTCUTS.TOGGLE_STATS:
          setStatsVisible((v) => !v);
          break;
        case SHORTCUTS.TOGGLE_CONFIG:
          setConfigVisible((v) => !v);
          break;
        case SHORTCUTS.CLEAR_SELECTION:
          if (event.key === 'Escape') {
            // ESC also closes config panel if open
            if (configVisible) {
              setConfigVisible(false);
            } else {
              clearSelection();
            }
          }
          break;
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => {
      window.removeEventListener('keydown', handleKeyPress);
    };
  }, [clearSelection, configVisible]);

  // Cleanup renderer on unmount
  useEffect(() => {
    return () => {
      if (voxelRendererRef.current) {
        voxelRendererRef.current.dispose();
        voxelRendererRef.current = null;
      }
      layoutRef.current = null;
    };
  }, []);

  // Handle close info panel
  const handleCloseInfoPanel = useCallback(() => {
    clearSelection();
  }, [clearSelection]);

  // Handle config settings change
  const handleConfigChange = useCallback((newSettings: Config3DSettings) => {
    setConfigSettings(newSettings);
  }, []);

  // Toggle config panel
  const toggleConfigPanel = useCallback(() => {
    setConfigVisible((v) => !v);
  }, []);

  // Render loading state
  if (isLoading) {
    return (
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          backgroundColor: '#f5f5f5',
        }}
      >
        <div style={{ textAlign: 'center' }}>
          <div>Loading visualization...</div>
          <div style={{ marginTop: '8px', fontSize: '14px', color: '#666' }}>
            This may take a moment for large codebases
          </div>
        </div>
      </div>
    );
  }

  // Render error state
  if (isError || sceneError) {
    const errorMessage = sceneError?.message || metricsError || 'Unknown error occurred';
    return (
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          backgroundColor: '#f5f5f5',
        }}
      >
        <div
          style={{
            textAlign: 'center',
            maxWidth: '500px',
            padding: '20px',
            backgroundColor: '#fff',
            borderRadius: '8px',
            boxShadow: '0 2px 8px rgba(0,0,0,0.1)',
          }}
        >
          <div style={{ color: '#d32f2f', fontWeight: 'bold', marginBottom: '12px' }}>
            Visualization Error
          </div>
          <div style={{ color: '#666', fontSize: '14px' }}>{errorMessage}</div>
          {!sceneError && (
            <div style={{ marginTop: '16px', fontSize: '12px', color: '#999' }}>
              Please check your browser supports WebGL and try again.
            </div>
          )}
        </div>
      </div>
    );
  }

  // Render main view
  return (
    <div
      style={{
        width: '100%',
        height: '100%',
        position: 'relative',
        overflow: 'hidden',
      }}
    >
      {/* Canvas for Three.js rendering */}
      <canvas
        ref={canvasRef}
        style={{
          width: '100%',
          height: '100%',
          display: 'block',
        }}
      />

      {/* Info panel */}
      {infoPanelVisible && selectedNode && (
        <InfoPanel
          data={selectedNode}
          position="bottom-left"
          onClose={handleCloseInfoPanel}
        />
      )}

      {/* Statistics overlay */}
      {statsVisible && (
        <StatisticsOverlay
          stats={renderStats}
          position="top-right"
          showFPS={true}
          fpsWarningThreshold={30}
        />
      )}

      {/* Config panel */}
      <ConfigPanel
        settings={configSettings}
        onSettingsChange={handleConfigChange}
        visible={configVisible}
        onToggle={toggleConfigPanel}
      />

      {/* Keyboard shortcuts help */}
      <div
        style={{
          position: 'absolute',
          bottom: '10px',
          right: '10px',
          fontSize: '11px',
          color: '#999',
          backgroundColor: 'rgba(255, 255, 255, 0.8)',
          padding: '6px 10px',
          borderRadius: '4px',
          userSelect: 'none',
        }}
      >
        I: Info | S: Stats | C: Config | ESC: Clear
      </div>
    </div>
  );
};

export default Voxel3DView;
