# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-15

### Added
- **Core Analysis Engine:**
  - Parallelized file scanning and metrics calculation using Rayon.
  - TypeScript (.ts, .tsx) and JavaScript (.js, .jsx) support using Tree-sitter.
  - Accurate LOC calculation excluding comments and blank lines.
  - Ignore pattern support (default: node_modules, target, etc.).

- **CLI:**
  - `analyze` command for repository analysis.
  - `watch` command for real-time monitoring.
  - `diff` command for comparing analysis reports.
  - `config init` command to generate configuration.
  - Output formats: Text (human-readable), JSON, CSV.
  - CI Integration: `--threshold` flag and `--baseline` comparison.

### Fixed
- N/A (Initial Release)

### Known Limitations
- Rust (.rs) parsing is partial/placeholder (full support planned for future).
- Python (.py) parsing is not yet implemented.
- Watch mode on some CI environments might be flaky (E2E tests skip watch mode).
