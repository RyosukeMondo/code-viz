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

## Features

CodeViz provides 13 powerful code analysis and visualization features:

### 1. **Basic Metrics** 📊
Accurate LOC calculation excluding comments and blank lines, function counts, file sizes.
```bash
code-viz analyze ./src --format json
```

### 2. **Code Duplication Detection** 🔄
Identifies duplicate code blocks across files with configurable similarity thresholds.
```bash
code-viz analyze ./src --duplicates --min-duplicate-lines 5
```

### 3. **Git Diff Comparison** 📈
Compare analysis reports between commits or branches.
```bash
code-viz diff report-old.json report-new.json
```

### 4. **AI Commit Analysis** 🤖
Detects AI-generated commits and calculates AI contribution ratio.
```bash
code-viz analyze ./src --ai-commits
```

### 5. **Coupling Metrics** 🔗
Measures afferent/efferent coupling and instability for each module.
*Automatically included in standard analysis*

### 6. **Code Churn** 📉
Tracks file modification frequency and change volume from Git history.
*Automatically included in standard analysis*

### 7. **Dead Code Detection** 💀
Semantic analysis using stack-graphs to identify unused functions and classes.
```bash
code-viz dead-code ./src --min-confidence 90
```
*See dedicated section below for details*

### 8. **AI Bloat Index** 🎈
Measures ratio of comments/documentation to code (detects over-documentation).
*Automatically included in standard analysis*

### 9. **Cognitive Complexity** 🧠
Per-function complexity analysis using SonarSource algorithm.
*Automatically included in standard analysis*

### 10. **Git Hotspot Detection** 🔥
Identifies high-risk files combining churn, complexity, and size.
```bash
code-viz analyze ./src --hotspots --max-hotspots 10
```

### 11. **Test Coverage Integration** ✅
Integrates llvm-cov or Tarpaulin coverage reports.
```bash
code-viz analyze ./src --coverage-report coverage.json
```

### 12. **Watch Mode** 👁️
Real-time monitoring with incremental re-analysis on file changes.
```bash
code-viz watch ./src --format text
```

### 13. **3D Code Visualization** 🏙️
Interactive 3D city visualization where files are buildings with GPU-accelerated rendering.
*Available in desktop app and web interface*

See the [3D Visualization Guide](docs/3d-visualization.md) for detailed documentation.

## 3D Code Visualization

Transform your codebase into an interactive 3D city where files become buildings in a virtual world. This powerful visualization makes complexity issues immediately visible and provides an intuitive way to explore code structure.

### Key Features

- **Buildings as Files**: Height represents LOC, color shows complexity
- **Interactive Exploration**: Click to select, hover for info, pan/zoom/rotate camera
- **GPU-Accelerated**: 60fps performance even for 100K+ LOC codebases
- **Visual Effects**: Heat haze shader for high-complexity files
- **Real-Time Stats**: Monitor FPS, memory usage, and rendering performance
- **Persistent Camera**: Your view position saves between sessions
- **Configurable**: Customize voxel size, height, colors, and performance settings

### Quick Start

Generate analysis data and open in 3D view:

```bash
# Analyze your codebase
code-viz analyze ./src --format json --output metrics.json

# Open in desktop app with 3D visualization
code-viz-gui --load metrics.json
```

### Controls

| Mouse | Action |
|-------|--------|
| Left-click + drag | Rotate camera |
| Right-click + drag | Pan camera |
| Mouse wheel | Zoom in/out |
| Click building | Select and show details |

| Keyboard | Action |
|----------|--------|
| **I** | Toggle info panel |
| **S** | Toggle statistics |
| **C** | Open configuration |
| **ESC** | Clear selection |

### Understanding the Visualization

**Height**: Logarithmic scale based on Lines of Code
- Small files: Short buildings
- Large files: Tall buildings (capped at max height)

**Colors**: Complexity gradient
- 🟢 Green: Low complexity (0-10)
- 🟡 Yellow: Moderate (10-30)
- 🔴 Red: High complexity (30+)

**Effects**: Heat haze shader on very complex files (>50 complexity)

### Performance Tips

For large codebases (100K+ LOC):
1. Reduce voxel size to 0.5 or lower
2. Disable shadows in configuration
3. Lower max height to 100-150
4. Monitor memory usage in statistics overlay

### Documentation

See the comprehensive [3D Visualization Guide](docs/3d-visualization.md) for:
- Detailed controls and keyboard shortcuts
- Configuration options and customization
- Performance optimization strategies
- Troubleshooting common issues
- Technical architecture and API reference

## Documentation

Comprehensive documentation is available in the [`docs/`](docs/) directory:

- **[3D Visualization](docs/3d-visualization.md)** - Interactive 3D code city visualization guide
- **[Architecture](docs/architecture/)** - System architecture, design decisions, and diagrams
  - [Architecture Overview](docs/architecture/ARCHITECTURE.md) - Trait-based DI architecture
  - [Frontend-Backend Architecture](docs/architecture/diagrams/FRONTEND_BACKEND_ARCHITECTURE.md) - Full stack architecture with React + Tauri
  - [Component Interaction](docs/architecture/diagrams/COMPONENT_INTERACTION.md) - Component flow and data flow diagrams
- **[Guides](docs/guides/)** - Development workflows and feature guides
  - [Fast Iteration Guide](docs/guides/development/fast-iteration.md) - Quick development workflow
  - [Tauri CLI Guide](docs/guides/development/tauri-cli.md) - Tauri command reference
- **[Testing](docs/testing/)** - Testing strategy and coverage reports
- **[Troubleshooting](docs/troubleshooting/)** - Common issues and fixes

See the [documentation index](docs/README.md) for the complete documentation structure.

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

## Dead Code Detection

CodeViz includes semantic dead code analysis to identify unused functions, classes, and modules across your codebase using stack-graphs for cross-file reachability analysis.

### Overview

The dead code analyzer builds a symbol graph from import/export relationships and performs depth-first search from entry points (main files, exports, tests) to identify unreachable code. Each dead symbol receives a confidence score (0-100) indicating how safe it is to delete.

**Key Features:**
- 🎯 **Semantic Analysis**: Uses stack-graphs to track cross-file dependencies
- 📊 **Confidence Scores**: 0-100 score based on exports, test coverage, recent changes
- 🔍 **Function-Level Granularity**: Identifies dead functions, classes, and methods
- ⚡ **Incremental Analysis**: Cached symbol graphs for fast re-analysis
- 🎨 **Visual Overlay**: GUI treemap highlights files with dead code

### Quick Start

Analyze a project for dead code:

```bash
code-viz dead-code ./src
```

Output:
```text
Dead Code Analysis Summary
===========================
Total Files Analyzed: 42
Files with Dead Code: 12
Dead Functions:       28
Dead Classes:         4
Total Dead LOC:       856 (12.3% of codebase)

High-Confidence Deletions (score > 90):
  1. src/utils/deprecated.ts: oldHelper() - 95% confidence
  2. src/legacy/converter.ts: convertLegacy() - 92% confidence

Top Files by Dead Code:
  1. src/utils/deprecated.ts (245 LOC, 89% dead)
  2. src/legacy/converter.ts (180 LOC, 65% dead)
```

### CLI Usage

#### Standalone Command

```bash
# Basic analysis
code-viz dead-code ./src

# JSON output for CI integration
code-viz dead-code ./src --format json > dead-code-report.json

# Filter by minimum confidence (only show high-confidence dead code)
code-viz dead-code ./src --min-confidence 90

# Write results to file
code-viz dead-code ./src --output dead-code.json

# Fail CI if dead code ratio exceeds threshold (exit code 3)
code-viz dead-code ./src --threshold dead_code_ratio=0.15
```

#### Integrated with Analyze Command

Enable dead code detection alongside standard analysis:

```bash
# Add dead code analysis to standard metrics
code-viz analyze ./src --dead-code

# JSON output includes dead code fields
code-viz analyze ./src --dead-code --format json
```

This extends the JSON output with dead code fields:
```json
{
  "files": [
    {
      "path": "src/utils/helper.ts",
      "loc": 150,
      "dead_function_count": 3,
      "dead_code_loc": 45,
      "dead_code_ratio": 0.30
    }
  ]
}
```

### GUI Usage

The Tauri desktop app provides visual dead code detection:

1. **Enable Overlay**: Click the "Dead Code" toggle in the toolbar
2. **Visual Indicators**: Files with dead code show colored borders on the treemap:
   - 🔴 **Red**: >50% dead code
   - 🟠 **Orange**: 20-50% dead code
   - 🟡 **Yellow**: <20% dead code
3. **Hover Details**: Tooltips show dead code ratio and symbol counts
4. **Click for Details**: Select a file to see the list of dead symbols with confidence scores and line numbers

### Confidence Scores

Each dead symbol receives a confidence score indicating deletion safety:

| Score | Meaning | Action |
|-------|---------|--------|
| 90-100 | **High Confidence** | Safe to delete - unexported, untested, no dynamic usage |
| 70-89 | **Medium Confidence** | Likely safe - may have test coverage or be exported |
| 0-69 | **Low Confidence** | Review carefully - exported, recently modified, or matches dynamic import patterns |

**Score Calculation:**
- Base score: 100
- Exported to public API: -30
- Modified in last 30 days: -20
- Matches dynamic import pattern (`*_handler`, `*_plugin`): -25
- Has test coverage: -15

### False Positives

To reduce false positives, use the ignore file:

Create `.code-viz-ignore-dead` in your project root:
```
# Ignore specific symbols (exact match)
myDynamicHandler
legacyPluginLoader

# Ignore by pattern (glob)
*_plugin
handle*

# Ignore entire files
src/plugins/**
```

Common false positive cases:
- **Dynamic imports**: `import('./handlers/' + name)` - scored with low confidence
- **Reflection/eval**: Code loaded dynamically at runtime
- **Public API exports**: Exported for external consumers (lower confidence)
- **Framework conventions**: Files loaded by convention (e.g., Next.js pages)

### Troubleshooting

**"No entry points found" error:**
- Ensure your project has `main.ts`, `index.ts`, or test files
- Check that files are not excluded by `.code-viz.toml` patterns
- For libraries, all exports are treated as entry points

**High false positive rate:**
- Use `--min-confidence 80` to filter out low-confidence results
- Add patterns to `.code-viz-ignore-dead`
- Check for dynamic import patterns in your codebase

**Slow analysis on large codebases:**
- First run builds symbol graph cache (may take 1-2 minutes)
- Subsequent runs use incremental analysis (<1 second)
- Cache stored in `.code-viz/cache/symbols.db`

## Contract Testing

CodeViz uses contract validation tests to ensure data integrity and interface consistency between the Rust backend and the TypeScript frontend.

### Overview

Contract tests validate that the data models shared between Rust and TypeScript (via Specta) are consistent and that the serialized JSON output matches the expectations of frontend visualization libraries.

**Key Validations:**
- 🛡️ **Type Safety**: Automatic TypeScript type generation with Specta.
- 🔄 **Serialization**: Round-trip validation for complex hierarchical data (`TreeNode`).
- 📈 **Library Compatibility**: Specific validation for ECharts treemap requirements.
- 🐛 **Regression Testing**: Ensures historical bugs like the "wrapper node bug" don't return.

### Running Contract Tests

```bash
cargo nextest run --package code-viz-tauri --test contract_tests
```

For more details, see [crates/code-viz-tauri/tests/README.md](./crates/code-viz-tauri/tests/README.md).

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

[dead_code]
# Minimum confidence score to report (0-100)
min_confidence = 80
# Patterns to ignore (in addition to .code-viz-ignore-dead)
ignore_patterns = ["*_plugin", "*_handler"]
```

## CI/CD Integration

### GitHub Actions Examples

#### Code Size Monitoring

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

#### Dead Code Detection

Fail CI if dead code ratio exceeds 15% or if new dead code is introduced:

```yaml
name: Dead Code Check
on: [pull_request]

jobs:
  dead-code:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install CodeViz
        run: cargo install --git https://github.com/rmondo/code-viz crates/code-viz-cli

      - name: Check Dead Code
        run: |
          # Fail if dead code ratio exceeds 15%
          code-viz dead-code ./src --threshold dead_code_ratio=0.15

          # Generate report for PR comment
          code-viz dead-code ./src --format json --output dead-code.json

      - name: Comment Dead Code Report
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = JSON.parse(fs.readFileSync('dead-code.json'));
            const summary = report.summary;

            const comment = `## 🔍 Dead Code Analysis

            - **Dead Functions**: ${summary.dead_functions}
            - **Dead Classes**: ${summary.dead_classes}
            - **Total Dead LOC**: ${summary.total_dead_loc} (${(summary.dead_code_ratio * 100).toFixed(1)}%)

            ${summary.dead_code_ratio > 0.15 ? '⚠️ Dead code ratio exceeds 15% threshold!' : '✅ Within acceptable threshold'}
            `;

            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
```

## Contributing

1. Clone repo
2. Run tests: `cargo test`
3. Run lints: `cargo clippy`

## License

MIT