# 3D Code Visualization

A powerful 3D voxel-based visualization feature that transforms code metrics into an interactive 3D city where files are represented as buildings. This provides an intuitive, spatial understanding of codebase structure, complexity, and relationships.

<!-- TODO: Add overview screenshot here
![3D Visualization Overview](./images/3d-visualization/overview.png)
*The 3D city view showing your codebase structure with buildings representing files*
-->

## Overview

The 3D visualization uses GPU-accelerated rendering with Three.js to display your codebase as a virtual city:

- **Buildings represent files**: Height indicates Lines of Code (LOC), position shows file hierarchy
- **Colors indicate complexity**: Smooth gradients from green (simple) to red (complex)
- **Interactive exploration**: Click to select files, hover for quick info, pan/zoom/rotate the camera
- **Real-time statistics**: Monitor FPS, voxel count, memory usage, and rendering performance
- **Persistent camera**: Your view position is saved and restored between sessions

### Key Features

- **GPU-Accelerated Rendering**: Maintains 60fps even for 100K+ LOC codebases
- **Instanced Mesh Optimization**: Efficient rendering using THREE.InstancedMesh
- **Frustum Culling**: Automatically hides off-screen buildings to boost performance
- **LOD System**: Reduces detail for distant buildings (shows only bottom 50% of voxels)
- **Interactive Selection**: Click buildings to see detailed file metrics
- **Heat Haze Effect**: High-complexity files shimmer with a shader effect
- **Camera Persistence**: Your camera position is saved to localStorage
- **Memory Monitoring**: Warns when approaching browser memory limits
- **Configurable Settings**: Customize voxel size, height, colors, and performance options

## Getting Started

### Loading the 3D Visualization

The 3D visualization can be accessed through the CodeViz desktop app or web interface:

1. **From Analysis Results**: After running `code-viz analyze`, click the "3D View" button
2. **From File Menu**: Select "Visualizations" → "3D Code City"
3. **Direct URL**: Navigate to `/3d-view` in the web interface

### Sample Data

To try the 3D visualization with sample data:

```bash
# Generate analysis data for your project
code-viz analyze ./src --format json --output metrics.json

# Open the desktop app and load the metrics.json file
code-viz-gui --load metrics.json
```

## Controls

### Mouse Controls

| Action | Control |
|--------|---------|
| **Rotate Camera** | Left-click + drag |
| **Pan Camera** | Right-click + drag (or Shift + left-click + drag) |
| **Zoom In/Out** | Mouse wheel scroll |
| **Select Building** | Left-click on building |
| **Hover Info** | Move mouse over building |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **I** | Toggle Info Panel (shows selected file details) |
| **S** | Toggle Statistics Overlay (FPS, voxel count, memory) |
| **C** | Toggle Configuration Panel (adjust settings) |
| **ESC** | Clear selection / Close panels |

### Camera Tips

- **Reset View**: Use the configuration panel (press C) and click "Reset Camera"
- **Focus on Area**: Zoom in close to focus on specific modules
- **Overview Mode**: Zoom out to see the entire codebase structure
- **Smooth Navigation**: Camera movements are smoothed for comfortable exploration

## Understanding the Visualization

### Building Height

Building height represents the **Lines of Code (LOC)** of each file using logarithmic scaling:

```
height = ln(1 + LOC / scale) * heightScale
```

This ensures that:
- Small files (10-50 LOC) are visible as small buildings
- Medium files (100-500 LOC) have proportional height
- Large files (1000+ LOC) don't dominate the view

### Color Coding

Colors represent **Cyclomatic Complexity** with smooth gradients:

| Complexity | Color | Meaning |
|------------|-------|---------|
| 0-10 | **Green** | Simple, easy to maintain |
| 10-20 | **Yellow-Green** | Moderate complexity |
| 20-30 | **Yellow-Orange** | Getting complex |
| 30-50 | **Orange-Red** | High complexity, review needed |
| 50+ | **Red** | Very complex, refactor recommended |

You can customize these thresholds in the Configuration Panel (press C).

### Spatial Layout

Files are arranged using a **treemap layout** algorithm:

- **Position**: Files are grouped by directory structure
- **Adjacent buildings**: Often belong to the same module/directory
- **Size**: Building footprint reflects the relative importance (LOC ratio)
- **Spacing**: Small gaps between buildings for visual clarity

### Visual Effects

#### Heat Haze Shader
High-complexity files (complexity > 50 by default) have a heat haze effect - a subtle shader that makes them appear to shimmer. This helps quickly identify problem areas.

#### Selection Highlighting
- **Selected building**: Highlighted with a brighter material and slight emissive glow
- **Hover effect**: Buildings brighten when you hover over them
- **Smooth transitions**: All visual changes use smooth animations

## Info Panel

The Info Panel displays detailed metrics for the selected file. Press **I** to toggle visibility or click a building to show it.

<!-- TODO: Add info panel screenshot here
![Info Panel](./images/3d-visualization/info-panel.png)
*Info Panel showing detailed metrics for a selected file*
-->

### Displayed Information

- **File Path**: Full path relative to project root
- **Lines of Code**: Total LOC (excluding comments and blank lines)
- **Cyclomatic Complexity**: Function-level complexity score
- **Function Count**: Number of functions in the file
- **Cognitive Complexity**: SonarSource cognitive complexity score
- **Code Churn**: Number of Git commits modifying this file
- **Coupling Metrics**: Afferent/efferent coupling and instability

### Panel Controls

- **Close**: Click the X button or press ESC
- **Position**: Fixed to bottom-left corner
- **Resizable**: Panel adjusts based on content length

## Statistics Overlay

The Statistics Overlay shows real-time rendering performance. Press **S** to toggle visibility.

### Displayed Statistics

- **FPS (Frames Per Second)**: Current rendering frame rate
  - Green: 60fps (optimal)
  - Yellow: 30-60fps (acceptable)
  - Red: <30fps (performance issue)
- **Voxel Count**: Total number of voxels rendered
- **Instance Count**: Number of instanced meshes (optimized rendering)
- **Memory Usage**: Estimated heap memory usage
  - Warning at 70% of available heap
  - Critical at 85% of available heap

### Performance Warnings

When FPS drops below 30fps, the overlay displays suggestions:

- Reduce voxel size in configuration
- Lower max building height
- Disable shadows (if enabled)
- Close other browser tabs
- Try a smaller codebase subset

### Memory Warnings

When memory usage exceeds thresholds:

- **70% (Warning)**: "High memory usage. Consider reducing voxel size."
- **85% (Critical)**: "Critical memory usage! Reduce voxel size or max height immediately."

Memory estimates include:
- Geometry buffers (vertices, normals, UVs)
- Material instances
- Texture memory
- Three.js scene graph overhead

## Configuration Panel

Open the Configuration Panel by pressing **C** to customize visualization settings.

<!-- TODO: Add configuration panel screenshot here
![Configuration Panel](./images/3d-visualization/config-panel.png)
*Configuration panel with customizable visualization settings*
-->

### Voxel Settings

**Voxel Size** (0.1 - 2.0)
- Default: 1.0
- Controls the size of individual voxel cubes
- Smaller values = more detailed buildings, more voxels, higher memory
- Larger values = blockier buildings, fewer voxels, better performance

**Max Height** (50 - 500)
- Default: 200
- Maximum height buildings can reach in the scene
- Prevents extremely large files from dominating the view
- Adjust based on your codebase size distribution

### Complexity Thresholds

Customize the color gradient thresholds:

- **Low Threshold**: Green to yellow-green transition (default: 10)
- **Medium Threshold**: Yellow-green to yellow-orange (default: 20)
- **High Threshold**: Yellow-orange to orange-red (default: 30)
- **Very High Threshold**: Orange-red to red transition (default: 50)

These control when buildings change color based on complexity.

### Performance Settings

**Antialiasing**
- Default: Enabled
- Smooths jagged edges for better visual quality
- Disable to improve performance on lower-end hardware

**Shadows**
- Default: Disabled
- Adds realistic shadow casting from buildings
- Enable for more realistic visuals (requires GPU support)
- Warning: Can significantly impact performance

### Saving Settings

All settings are automatically saved to `localStorage` when changed and restored on next session.

**Reset to Defaults**: Click the "Reset to Defaults" button to restore factory settings.

## Performance Optimization

### For Large Codebases (100K+ LOC)

If you experience performance issues with large codebases:

1. **Reduce Voxel Size**: Lower to 0.5 or 0.3 in configuration
2. **Lower Max Height**: Reduce to 100-150 to decrease voxel count
3. **Disable Shadows**: Shadows are expensive, keep disabled for large projects
4. **Close Other Tabs**: Free up browser memory and GPU resources
5. **Use Chrome/Edge**: Best WebGL performance compared to other browsers

### Optimization Features

The 3D visualization includes automatic optimizations:

- **Frustum Culling**: Buildings outside camera view are hidden
- **LOD System**: Distant buildings show fewer voxels (50% reduction)
- **Instanced Rendering**: All voxels share geometry for efficiency
- **Throttled Updates**: Visibility checks throttled to camera movement
- **Memory Budget**: Warns before reaching browser limits

### Performance Metrics

Target performance on reference hardware:

| Codebase Size | Target FPS | Typical Voxel Count | Memory Usage |
|---------------|------------|---------------------|--------------|
| 1K-10K LOC | 60fps | 1K-5K voxels | <50MB |
| 10K-50K LOC | 60fps | 5K-20K voxels | 50-150MB |
| 50K-100K LOC | 60fps | 20K-40K voxels | 150-300MB |
| 100K+ LOC | 45-60fps | 40K-50K voxels | 300-500MB |

Reference: Modern laptop with dedicated GPU (GTX 1650 or better)

## Troubleshooting

### Visualization Won't Load

**Symptoms**: Blank screen, error message, or infinite loading

**Solutions**:
1. Check browser supports WebGL: Visit https://get.webgl.org/
2. Update graphics drivers if WebGL test fails
3. Try different browser (Chrome/Edge recommended)
4. Check browser console for error messages (F12 → Console tab)
5. Verify metrics data is valid JSON format

### Low FPS / Choppy Rendering

**Symptoms**: Frame rate below 30fps, stuttering camera movement

**Solutions**:
1. Reduce voxel size in configuration (try 0.5)
2. Lower max height to 100-150
3. Disable shadows in configuration
4. Close other browser tabs/applications
5. Check Statistics Overlay memory warnings
6. Try smaller codebase subset for testing

### Memory Errors / Crashes

**Symptoms**: Browser tab crashes, "Out of memory" errors

**Solutions**:
1. Reduce voxel size significantly (try 0.3 or 0.2)
2. Lower max height to 50-100
3. Analyze smaller portions of codebase
4. Close other tabs to free memory
5. Restart browser to clear memory
6. Check available system RAM (8GB+ recommended)

### Buildings Not Visible

**Symptoms**: Empty scene or missing buildings

**Solutions**:
1. Zoom out using mouse wheel (might be too close)
2. Reset camera in configuration panel
3. Check data was loaded successfully (no error message)
4. Verify JSON metrics file contains file data
5. Check browser console for warnings

### Selection Not Working

**Symptoms**: Clicking buildings doesn't show info panel

**Solutions**:
1. Press **I** to ensure info panel not hidden
2. Try clicking center of building (more reliable hit detection)
3. Zoom in closer for better selection accuracy
4. Check browser console for JavaScript errors
5. Reload page to reset state

### Camera Position Not Saving

**Symptoms**: Camera resets to default view on page reload

**Solutions**:
1. Check browser localStorage is enabled (not in private/incognito mode)
2. Verify no browser extensions blocking localStorage
3. Check browser console for localStorage errors
4. Try clearing browser cache and reloading
5. Ensure same `projectKey` is used across sessions

## Technical Architecture

### Component Structure

```
Voxel3DView (main component)
├── useThreeScene (Three.js lifecycle)
│   └── SceneManager (scene, camera, renderer)
│       ├── CameraPersistence (localStorage save/restore)
│       └── RaycasterHandler (mouse interaction)
├── useMetricsData (data loading/validation)
│   └── metricsLoader (JSON parsing)
├── useSelection (selection state)
│   └── SelectionState (selected/hovered nodes)
├── VoxelRenderer (voxel rendering)
│   ├── TreemapLayout (spatial layout calculation)
│   ├── VoxelOptimizer (geometry merging)
│   ├── VoxelVisibilityManager (frustum culling + LOD)
│   └── VoxelHighlighter (selection effects)
├── InfoPanel (file details UI)
├── StatisticsOverlay (performance stats UI)
└── ConfigPanel (settings UI)
```

### Data Flow

1. **Load Metrics**: `useMetricsData` loads and validates JSON data
2. **Calculate Layout**: `TreemapLayout` computes building positions
3. **Render Voxels**: `VoxelRenderer` creates instanced meshes
4. **Handle Interaction**: `RaycasterHandler` detects mouse clicks
5. **Update Selection**: `useSelection` manages selected state
6. **Display Info**: `InfoPanel` shows selected file details
7. **Monitor Performance**: `StatisticsOverlay` tracks FPS and memory

### Performance Optimizations

**Instanced Rendering**
- All voxels share single geometry buffer
- GPU renders thousands of instances efficiently
- Minimizes draw calls and state changes

**Frustum Culling**
- Buildings outside camera frustum are hidden
- Achieved by setting instance matrix scale to zero
- Updated on camera movement (throttled)

**LOD System**
- Distant buildings (>100 units) show only bottom 50% of voxels
- Reduces total voxel count without visual impact
- Smoothly transitions based on camera distance

**Memory Management**
- Estimates total memory usage (geometry + textures + scene)
- Warns at 70% and 85% of available heap
- Provides actionable suggestions to reduce usage

## API Reference

### Voxel3DView Component

```typescript
interface Voxel3DViewProps {
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
```

### DataSource Types

```typescript
type DataSource =
  | { type: 'url'; url: string }           // Load from URL
  | { type: 'file'; file: File }           // Load from File object
  | { type: 'json'; data: object }         // Use provided JSON data
```

### Config3DSettings

```typescript
interface Config3DSettings {
  voxelSize: number;                       // 0.1 - 2.0
  maxHeight: number;                       // 50 - 500
  thresholds: ComplexityThresholds;        // Color thresholds
  antialias: boolean;                      // Enable antialiasing
  shadowsEnabled: boolean;                 // Enable shadows
}

interface ComplexityThresholds {
  low: number;      // Default: 10
  medium: number;   // Default: 20
  high: number;     // Default: 30
  veryHigh: number; // Default: 50
}
```

## Future Enhancements

Planned features for future releases:

- **Time-travel visualization**: Animate codebase evolution over Git history
- **Diff mode**: Highlight changed files between commits
- **Module boundaries**: Visual dividers between major modules
- **Dependency arrows**: Show import/export relationships
- **Test coverage overlay**: Color by test coverage percentage
- **Minimap**: Small overview map for navigation
- **VR support**: Explore codebase in virtual reality
- **Collaboration mode**: Multiple users exploring together
- **Custom metrics**: Configure color/height based on any metric
- **Export images**: Save camera view as PNG/SVG

## Related Documentation

- [Architecture Overview](./architecture/ARCHITECTURE.md) - Full system architecture
- [Frontend-Backend Architecture](./architecture/diagrams/FRONTEND_BACKEND_ARCHITECTURE.md) - React + Tauri integration
- [Testing Strategy](./testing/README.md) - Test coverage and E2E tests
- [Fast Iteration Guide](./guides/development/fast-iteration.md) - Development workflow

## Feedback and Issues

Found a bug or have a feature request? Please report it:

- **GitHub Issues**: https://github.com/rmondo/code-viz/issues
- **Discussion**: https://github.com/rmondo/code-viz/discussions

## Credits

The 3D visualization feature was migrated and enhanced from the original [code-3d](https://github.com/rmondo/code-3d) project. Major improvements include:

- Full TypeScript conversion with strict typing
- React integration with custom hooks
- Memory monitoring and optimization
- Frustum culling and LOD system
- Enhanced UI with configuration panel
- Comprehensive test coverage (>90%)
- E2E test suite with Playwright
