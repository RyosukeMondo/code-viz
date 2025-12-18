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

The analysis pipeline consists of six stages:

```
Source Files → Symbol Extraction → Graph Building → Entry Point Detection
                                         ↓
                         Dead Code Results ← Confidence Scoring ← Reachability Analysis
```

### Modules

- `symbol_graph`: Symbol extraction from Tree-sitter AST and graph construction
- `reachability`: DFS-based reachability analysis algorithm
- `confidence`: Confidence score calculation using multiple heuristics
- `entry_points`: Entry point detection (main.ts, index.ts, test files, exports)
- `cache`: Symbol graph persistence using sled embedded database
- `models`: Core data structures (DeadCodeResult, Symbol, etc.)

## Public API

### Main Entry Point

```rust
pub fn analyze_dead_code(
    path: &Path,
    config: Option<AnalysisConfig>,
) -> Result<DeadCodeResult, AnalysisError>
```

Orchestrates the full analysis pipeline and returns dead code results.

### Configuration

```rust
pub struct AnalysisConfig {
    pub exclude_patterns: Vec<String>,
    pub enable_cache: bool,
    pub cache_dir: Option<PathBuf>,
}
```

Control analysis behavior including file exclusions and caching.

### Data Models

#### `DeadCodeResult`
Main analysis result containing summary statistics and per-file dead code details.

```rust
pub struct DeadCodeResult {
    pub summary: DeadCodeSummary,
    pub files: Vec<FileDeadCode>,
}
```

**Methods:**
- `filter_by_confidence(min_confidence: u8) -> DeadCodeResult` - Filter results by confidence threshold

#### `DeadCodeSummary`
Aggregated statistics across the entire analysis.

```rust
pub struct DeadCodeSummary {
    pub total_files: usize,
    pub files_with_dead_code: usize,
    pub dead_functions: usize,
    pub dead_classes: usize,
    pub total_dead_loc: usize,
    pub dead_code_ratio: f64,
}
```

#### `FileDeadCode`
Dead code found in a single file.

```rust
pub struct FileDeadCode {
    pub path: PathBuf,
    pub dead_code: Vec<DeadSymbol>,
}
```

#### `DeadSymbol`
An individual unreachable symbol with metadata.

```rust
pub struct DeadSymbol {
    pub symbol: String,
    pub kind: SymbolKind,
    pub line_start: usize,
    pub line_end: usize,
    pub loc: usize,
    pub confidence: u8,
    pub reason: String,
    pub last_modified: Option<SystemTime>,
}
```

#### `SymbolKind`
Type of symbol detected.

```rust
pub enum SymbolKind {
    Function,
    ArrowFunction,
    Class,
    Method,
    Variable,
}
```

## Usage Example

```rust
use code_viz_dead_code::{analyze_dead_code, AnalysisConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Analyze with default configuration
    let result = analyze_dead_code(Path::new("./src"), None)?;

    println!("Analysis Results:");
    println!("  Total files: {}", result.summary.total_files);
    println!("  Dead functions: {}", result.summary.dead_functions);
    println!("  Dead classes: {}", result.summary.dead_classes);
    println!("  Dead code ratio: {:.2}%", result.summary.dead_code_ratio * 100.0);

    // Filter by high confidence (90%+)
    let high_confidence = result.filter_by_confidence(90);
    println!("\nHigh-confidence deletions: {}", high_confidence.summary.dead_functions);

    // Iterate through files with dead code
    for file in &high_confidence.files {
        println!("\nFile: {}", file.path.display());
        for symbol in &file.dead_code {
            println!("  {} {} (lines {}-{}, confidence: {}%)",
                match symbol.kind {
                    code_viz_dead_code::SymbolKind::Function => "function",
                    code_viz_dead_code::SymbolKind::Class => "class",
                    code_viz_dead_code::SymbolKind::Method => "method",
                    code_viz_dead_code::SymbolKind::ArrowFunction => "arrow fn",
                    code_viz_dead_code::SymbolKind::Variable => "variable",
                },
                symbol.symbol,
                symbol.line_start,
                symbol.line_end,
                symbol.confidence
            );
        }
    }

    Ok(())
}
```

### With Custom Configuration

```rust
use code_viz_dead_code::{analyze_dead_code, AnalysisConfig};
use std::path::{Path, PathBuf};

let config = AnalysisConfig {
    exclude_patterns: vec![
        "node_modules/**".to_string(),
        "dist/**".to_string(),
        "**/*.test.ts".to_string(),  // Exclude test files
    ],
    enable_cache: true,
    cache_dir: Some(PathBuf::from(".cache/dead-code")),
};

let result = analyze_dead_code(Path::new("./src"), Some(config))?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Performance

- **Large codebases**: Designed to handle 100K+ files in under 2 minutes
- **Incremental analysis**: Cached re-analysis completes in under 1 second for single file changes
- **Parallel processing**: Utilizes all CPU cores via rayon for symbol extraction
- **Memory efficient**: Uses embedded sled database for persistent caching

## Integration

This crate is designed to be used by:

- **`code-viz-cli`**: Command-line interface for dead code analysis
- **`code-viz-tauri`**: Tauri IPC commands for GUI integration
- **Direct library usage**: Via the `analyze_dead_code()` public API

## Error Handling

All errors are represented by the `AnalysisError` enum:

```rust
pub enum AnalysisError {
    ScanError(code_viz_core::scanner::ScanError),
    GraphError(symbol_graph::GraphError),
    ReachabilityError(reachability::ReachabilityError),
    CacheError(cache::CacheError),
    Io(std::io::Error),
    NoEntryPoints,
}
```

Errors use `thiserror` for clear error messages and proper error propagation.

## Development

### Building Documentation

```bash
cargo doc --open
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run integration tests only
cargo test --test integration_test
```

### Coverage

The crate maintains >90% test coverage for critical paths (symbol graph and reachability analysis).

## License

See the main Code-Viz repository for licensing information.
