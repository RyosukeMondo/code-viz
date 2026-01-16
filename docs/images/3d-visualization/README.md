# 3D Visualization Screenshots

This directory contains screenshots and images for the 3D visualization documentation.

## Required Screenshots

To complete the documentation, add the following screenshots:

### 1. Overview Screenshot
**Filename**: `overview.png`
**Description**: Main 3D city view showing a medium-sized codebase
- Full scene with multiple buildings
- Good camera angle showing depth and perspective
- Various building heights and colors visible
- Statistics overlay in top-right corner

### 2. Info Panel Screenshot
**Filename**: `info-panel.png`
**Description**: Selected building with Info Panel visible
- Building highlighted with selection effect
- Info Panel in bottom-left showing file details
- Clear display of all metrics (LOC, complexity, etc.)

### 3. Configuration Panel Screenshot
**Filename**: `config-panel.png`
**Description**: Configuration panel open
- Show all configuration options
- Sliders and inputs visible
- Settings panel in foreground

### 4. Color Legend
**Filename**: `color-legend.png`
**Description**: Buildings showing color gradient
- Range from green (low complexity) to red (high)
- Multiple buildings at different complexity levels
- Clear color transitions

### 5. Heat Haze Effect
**Filename**: `heat-haze-effect.png`
**Description**: High-complexity file with heat haze shader
- Red/very complex building with shimmer effect
- Visible distortion/haze around building
- Comparison with normal building

### 6. Statistics Overlay
**Filename**: `statistics-overlay.png`
**Description**: Statistics overlay showing performance metrics
- FPS counter visible
- Voxel and instance counts
- Memory usage display
- Warning indicators if applicable

### 7. Layout Example
**Filename**: `treemap-layout.png`
**Description**: Top-down or angled view showing spatial layout
- Clear directory grouping visible
- Buildings organized by module/package
- Spacing between groups

## Image Guidelines

- **Format**: PNG preferred (lossless, good for UI screenshots)
- **Resolution**: Minimum 1920x1080, recommend 2560x1440
- **Compression**: Optimize with tools like ImageOptim or TinyPNG
- **Annotations**: Add callouts/arrows if needed using image editor
- **Consistency**: Use same theme/color scheme across all screenshots

## Taking Screenshots

1. Load sample data with good variety (50-100 files, complexity range 5-60)
2. Set camera to aesthetic angle (slightly above, 30-45° rotation)
3. Ensure good lighting (default Three.js lighting)
4. Capture at high resolution
5. Crop/resize to appropriate size
6. Annotate if needed
7. Optimize file size

## Usage in Documentation

Reference screenshots in markdown:

```markdown
![3D Visualization Overview](./images/3d-visualization/overview.png)
*The 3D city view showing your codebase structure*
```

Or with relative paths from docs root:

```markdown
![Info Panel](./images/3d-visualization/info-panel.png)
```

## Future Images

Consider adding:
- GIF/video of camera rotation
- GIF of selection interaction
- Comparison: before/after optimization settings
- Examples of different codebase sizes
- Mobile/responsive view examples
