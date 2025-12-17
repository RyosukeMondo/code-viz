# CodeViz GUI - Tauri Application

This directory contains the Tauri v2 desktop application for CodeViz's interactive treemap visualization.

## System Prerequisites

Before building the Tauri application, you need to install system dependencies:

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf
```

### Fedora/RHEL
```bash
sudo dnf install \
    webkit2gtk4.1-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

### Arch Linux
```bash
sudo pacman -S \
    webkit2gtk-4.1 \
    gtk3 \
    libappindicator-gtk3 \
    librsvg
```

### macOS
```bash
# Xcode Command Line Tools (if not already installed)
xcode-select --install
```

### Windows
Download and install:
- Microsoft Visual Studio C++ Build Tools
- WebView2 (usually pre-installed on Windows 11)

## Building the Application

Once system dependencies are installed:

```bash
# From the project root
cd src-tauri
cargo build

# Or build with frontend integration
npm run tauri:dev    # Development
npm run tauri:build  # Production
```

## Project Structure

- `Cargo.toml` - Rust dependencies and project metadata
- `tauri.conf.json` - Tauri application configuration
- `src/main.rs` - Application entry point
- `build.rs` - Build script for Tauri

## Dependencies

This crate depends on:
- `code-viz-core` - Analysis engine (from workspace)
- `tauri` v2.0 - Desktop application framework
- `tauri-plugin-shell` - Shell command support

## Development

The app will be extended with:
- Tauri commands for repository analysis
- IPC bindings for frontend communication
- TypeScript type generation via specta
