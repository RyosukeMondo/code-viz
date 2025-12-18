# code-viz-dead-code

Dead code detection and analysis engine for Code-Viz.

## Purpose

This crate provides semantic reachability analysis to identify unused (dead) code in TypeScript/JavaScript codebases. Unlike simple static analysis that only looks at local scopes, this engine builds a complete symbol graph from import/export relationships and performs depth-first traversal from entry points to identify truly unreachable code.

## Features

- **Symbol Graph Construction**: Extracts functions, classes, imports, and exports from TypeScript/JavaScript using Tree-sitter
- **Reachability Analysis**: DFS traversal from entry points (main files, exports, tests) to identify reachable symbols
- **Confidence Scoring**: Calculates deletion confidence (0-100) based on:
  - Export status (exported symbols have lower confidence)
  - Recent modifications (recently changed code has lower confidence)
  - Dynamic import patterns (code matching plugin/handler patterns has lower confidence)
  - Test coverage (code referenced in tests has lower confidence)
- **Incremental Analysis**: Caches symbol graph to disk using embedded sled database for fast re-analysis
- **Parallel Processing**: Leverages rayon for multi-threaded symbol extraction

## Architecture

The crate is organized into the following modules:

- `symbol_graph`: Symbol extraction from Tree-sitter AST and graph construction
- `reachability`: DFS-based reachability analysis algorithm
- `confidence`: Confidence score calculation using multiple heuristics
- `entry_points`: Entry point detection (main.ts, index.ts, test files, exports)
- `cache`: Symbol graph persistence using sled embedded database
- `models`: Core data structures (DeadCodeResult, Symbol, etc.)

## Integration

This crate is designed to be used by:

- `code-viz-cli`: Command-line interface for dead code analysis
- `code-viz-tauri`: Tauri IPC commands for GUI integration
- Direct library usage via `analyze_dead_code()` public API

## Development Status

This crate is currently under active development as part of the dead-code-detection specification.
