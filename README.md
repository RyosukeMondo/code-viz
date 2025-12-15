# CodeViz

A high-performance code visualization and analysis tool.

## Overview

CodeViz is a CLI tool and desktop application for analyzing codebase metrics, visualizing structure, and tracking evolution over time.

## Development

### Prerequisites

- Rust 1.75+
- Just (command runner)
- Mold (linker, optional but recommended for Linux)

### Setup

```bash
# Clone repository
git clone https://github.com/rmondo/code-viz.git
cd code-viz

# Check environment
just check
```

## Structure

- `crates/code-viz-core`: Core analysis engine
- `crates/code-viz-cli`: CLI interface
