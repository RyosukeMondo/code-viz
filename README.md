# CodeViz

A high-performance code visualization and analysis tool.

![CI Status](https://github.com/rmondo/code-viz/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Overview

CodeViz is a fast, parallelized CLI tool for analyzing codebase metrics (LOC, function counts, file sizes) with support for modern web languages (TypeScript, JavaScript, TSX). It is designed for CI/CD integration and rapid local development loops.

**Key Features:**
- ⚡ **Fast**: Parallelized analysis engine using Rayon.
- 📊 **Metrics**: Accurate Lines of Code (LOC) calculation excluding comments/blanks.
- 🛠️ **Multi-Language**: Built-in support for TypeScript (.ts, .tsx) and JavaScript (.js, .jsx).
- 🔄 **Watch Mode**: Real-time monitoring and incremental analysis.
- ⚙️ **CI Integration**: Threshold enforcement and baseline comparison (fail CI on regression).
- 📈 **Formats**: Output to JSON, CSV, or human-readable text.

## Installation

### From Source

```bash
cargo install --path crates/code-viz-cli
```

Or build from source:

```bash
git clone https://github.com/rmondo/code-viz.git
cd code-viz
cargo build --release
./target/release/code-viz-cli --help
```

## Quick Start

Analyze the current directory:

```bash
code-viz analyze .
```

Output:
```text
Code Analysis Summary
=====================
Total Files: 42
Total LOC:   3500
Functions:   120

Largest Files:
  1. src/core/engine.ts (450 LOC)
  2. src/ui/App.tsx (320 LOC)
  ...
```

## Commands

### `analyze`

Analyze a directory and generate metrics.

```bash
# Basic usage
code-viz analyze ./src

# Output as JSON
code-viz analyze ./src --format json > report.json

# Output as CSV
code-viz analyze ./src --format csv > report.csv

# Fail if any file exceeds 500 LOC (useful for CI)
code-viz analyze ./src --threshold loc=500

# Compare against a baseline report
code-viz analyze ./src --baseline report-old.json
```

### `watch`

Monitor a directory for changes and re-analyze incrementally.

```bash
code-viz watch ./src
```

### `diff`

Compare two analysis reports.

```bash
code-viz diff report-old.json report-new.json
```

Output:
```text
+2 files added
-1 files deleted
0 files modified (LOC changed)
Total LOC: 3500 -> 3650 (+150)
Largest growth: src/new-feature.ts (+120 LOC)
```

### `config init`

Initialize a default configuration file.

```bash
code-viz config init
```

## Configuration

CodeViz looks for a `.code-viz.toml` file in the project root.

```toml
[analysis]
# Glob patterns to exclude
exclude = ["node_modules/**", "dist/**", "**/*.test.ts"]

[output]
# Default output format
format = "text"

[cache]
enabled = true
```

## CI/CD Integration

### GitHub Actions Example

Fail CI if total LOC increases by more than 10% compared to main branch:

```yaml
name: Code Quality
on: [pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install CodeViz
        run: cargo install --git https://github.com/rmondo/code-viz crates/code-viz-cli

      - name: Download Baseline
        # (Implementation depends on artifact storage)
        run: curl -o baseline.json https://artifacts.example.com/main/report.json

      - name: Analyze & Compare
        run: |
          code-viz analyze . --format json --output report.json --baseline baseline.json
```

## Contributing

1. Clone repo
2. Run tests: `cargo test`
3. Run lints: `cargo clippy`

## License

MIT