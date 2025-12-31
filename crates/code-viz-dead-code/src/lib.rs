//! Dead Code Detection and Analysis Engine
//!
//! This crate provides semantic reachability analysis using symbol graphs to identify
//! unused functions, classes, and modules across a codebase. It builds a symbol graph
//! from import/export relationships and performs depth-first traversal from entry points
//! to identify unreachable (dead) code.
//!
//! # Architecture
//!
//! The analysis pipeline consists of the following stages:
//!
//! 1. **Symbol Extraction**: Parse source files using Tree-sitter to extract symbols
//!    (functions, classes, imports, exports)
//! 2. **Graph Building**: Construct a bidirectional dependency graph from symbols
//! 3. **Entry Point Detection**: Identify entry points (main files, exports, tests)
//! 4. **Reachability Analysis**: DFS traversal from entry points to mark reachable symbols
//! 5. **Dead Code Identification**: Symbols not reached = dead code
//! 6. **Confidence Scoring**: Calculate deletion confidence based on heuristics
//!
//! # Example
//!
//! ```rust,no_run
//! use code_viz_dead_code::analyze_dead_code;
//! use std::path::Path;
//!
//! // Analyze a TypeScript project for dead code
//! let result = analyze_dead_code(Path::new("./src"), None)?;
//! println!("Found {} dead symbols", result.summary.dead_functions);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![allow(dead_code)]

pub mod cache;
pub mod confidence;
pub mod entry_points;
pub mod models;
pub mod reachability;
pub mod symbol_graph;

// Re-export main types for convenience
pub use models::{DeadCodeResult, DeadCodeSummary, DeadSymbol, FileDeadCode};

pub use cache::{CacheError, SymbolGraphCache};
pub use confidence::ConfidenceCalculator;
pub use entry_points::detect_entry_points;
pub use reachability::{ReachabilityAnalyzer, ReachabilityError};
pub use symbol_graph::{GraphError, SymbolGraph, SymbolGraphBuilder};

use ahash::AHashMap as HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Configuration options for dead code analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Patterns to exclude from analysis (glob patterns)
    pub exclude_patterns: Vec<String>,

    /// Enable caching for incremental analysis
    pub enable_cache: bool,

    /// Cache directory path (defaults to .code-viz/cache)
    pub cache_dir: Option<PathBuf>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "node_modules/**".to_string(),
                "dist/**".to_string(),
                "build/**".to_string(),
                ".git/**".to_string(),
            ],
            enable_cache: true,
            cache_dir: None,
        }
    }
}

/// Error type for analysis operations
#[derive(Debug, Error)]
pub enum AnalysisError {
    /// Failed to scan directory
    #[error("Directory scan failed: {0}")]
    ScanError(#[from] code_viz_core::scanner::ScanError),

    /// Symbol graph construction failed
    #[error("Symbol graph construction failed: {0}")]
    GraphError(#[from] symbol_graph::GraphError),

    /// Reachability analysis failed
    #[error("Reachability analysis failed: {0}")]
    ReachabilityError(#[from] reachability::ReachabilityError),

    /// Cache operation failed
    #[error("Cache operation failed: {0}")]
    CacheError(#[from] cache::CacheError),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// No entry points found
    #[error("No entry points found in the codebase")]
    NoEntryPoints,
}

/// Main entry point for dead code analysis
///
/// This function orchestrates the full analysis pipeline:
/// 1. Scans the directory for source files
/// 2. Builds or loads cached symbol graph
/// 3. Detects entry points
/// 4. Performs reachability analysis
/// 5. Calculates confidence scores
/// 6. Returns aggregated results
///
/// # Arguments
///
/// * `path` - Root directory to analyze
/// * `config` - Analysis configuration options (uses defaults if None)
///
/// # Returns
///
/// A `DeadCodeResult` containing the summary and per-file dead code details
///
/// # Example
///
/// ```rust,no_run
/// use code_viz_dead_code::{analyze_dead_code, AnalysisConfig};
/// use std::path::Path;
///
/// let result = analyze_dead_code(Path::new("./src"), None)?;
/// println!("Found {} dead symbols", result.summary.dead_functions);
/// # Ok::<(), code_viz_dead_code::AnalysisError>(())
/// ```
#[tracing::instrument(skip(config), fields(path = %path.display()))]
/// Scan directory for source files
fn scan_source_files(path: &Path, config: &AnalysisConfig) -> Result<Vec<PathBuf>, AnalysisError> {
    tracing::info!("Scanning directory for source files");
    let files = code_viz_core::scanner::scan_directory(path, &config.exclude_patterns)?;
    tracing::info!(file_count = files.len(), "Found source files");
    Ok(files)
}

/// Build or load symbol graph based on configuration
fn get_symbol_graph(
    files: &[PathBuf],
    config: &AnalysisConfig,
    path: &Path,
) -> Result<SymbolGraph, AnalysisError> {
    let graph = if config.enable_cache {
        load_or_build_graph(files, config, path)?
    } else {
        build_graph_from_files(files)?
    };
    tracing::info!(symbol_count = graph.symbols.len(), "Symbol graph constructed");
    Ok(graph)
}

/// Detect and validate entry points
fn get_entry_points(graph: &SymbolGraph) -> Result<Vec<models::SymbolId>, AnalysisError> {
    tracing::info!("Detecting entry points");
    let entry_points = entry_points::detect_entry_points(graph);

    if entry_points.is_empty() {
        tracing::error!("No entry points found in codebase");
        return Err(AnalysisError::NoEntryPoints);
    }

    tracing::info!(entry_point_count = entry_points.len(), "Entry points detected");
    Ok(entry_points)
}

/// Perform reachability analysis from entry points
fn analyze_reachability(
    graph: &SymbolGraph,
    entry_points: Vec<models::SymbolId>,
) -> Result<ahash::AHashSet<models::SymbolId>, AnalysisError> {
    tracing::info!("Performing reachability analysis");
    let mut analyzer = reachability::ReachabilityAnalyzer::new(graph.clone());
    let reachable = analyzer.analyze(entry_points)?;

    tracing::info!(
        reachable_count = reachable.len(),
        total_count = graph.symbols.len(),
        "Reachability analysis complete"
    );
    Ok(reachable)
}

/// Aggregation statistics for dead code
struct DeadCodeStats {
    total_dead_loc: usize,
    dead_functions: usize,
    dead_classes: usize,
}

/// Process dead symbols into file-grouped results with confidence scores
fn process_dead_symbols(
    dead_symbols: Vec<models::Symbol>,
    calculator: &confidence::ConfidenceCalculator,
) -> (Vec<FileDeadCode>, DeadCodeStats) {
    let mut files_map: HashMap<PathBuf, Vec<DeadSymbol>> = HashMap::new();
    let mut stats = DeadCodeStats {
        total_dead_loc: 0,
        dead_functions: 0,
        dead_classes: 0,
    };

    for symbol in dead_symbols {
        let confidence = calculator.calculate(&symbol);
        let loc = symbol.line_end.saturating_sub(symbol.line_start) + 1;
        stats.total_dead_loc += loc;

        match symbol.kind {
            models::SymbolKind::Function
            | models::SymbolKind::ArrowFunction
            | models::SymbolKind::Method => stats.dead_functions += 1,
            models::SymbolKind::Class => stats.dead_classes += 1,
            _ => {}
        }

        let dead_symbol = DeadSymbol {
            symbol: symbol.name.clone(),
            kind: symbol.kind,
            line_start: symbol.line_start,
            line_end: symbol.line_end,
            loc,
            confidence,
            reason: "Unreachable from entry points".to_string(),
            last_modified: None,
        };

        files_map.entry(symbol.path.clone()).or_default().push(dead_symbol);
    }

    let mut files: Vec<FileDeadCode> = files_map
        .into_iter()
        .map(|(path, dead_code)| FileDeadCode { path, dead_code })
        .collect();

    files.sort_by(|a, b| a.path.cmp(&b.path));
    (files, stats)
}

/// Calculate dead code ratio from total and dead LOC
fn calculate_dead_code_ratio(graph: &SymbolGraph, total_dead_loc: usize) -> f64 {
    let total_loc: usize = graph
        .symbols
        .values()
        .map(|s| s.line_end.saturating_sub(s.line_start) + 1)
        .sum();

    if total_loc > 0 {
        total_dead_loc as f64 / total_loc as f64
    } else {
        0.0
    }
}

/// Build final result from processed dead code data
fn build_result(
    files: Vec<FileDeadCode>,
    stats: DeadCodeStats,
    dead_code_ratio: f64,
) -> DeadCodeResult {
    tracing::info!(
        dead_functions = stats.dead_functions,
        dead_classes = stats.dead_classes,
        total_dead_loc = stats.total_dead_loc,
        dead_code_ratio = format!("{:.2}%", dead_code_ratio * 100.0),
        "Analysis complete"
    );

    DeadCodeResult {
        summary: DeadCodeSummary {
            total_files: files.len(),
            files_with_dead_code: files.len(),
            dead_functions: stats.dead_functions,
            dead_classes: stats.dead_classes,
            total_dead_loc: stats.total_dead_loc,
            dead_code_ratio,
        },
        files,
    }
}

pub fn analyze_dead_code(
    path: &Path,
    config: Option<AnalysisConfig>,
) -> Result<DeadCodeResult, AnalysisError> {
    let config = config.unwrap_or_else(AnalysisConfig::default);
    tracing::info!("Starting dead code analysis");

    // Step 1: Scan source files
    let files = scan_source_files(path, &config)?;
    if files.is_empty() {
        tracing::warn!("No source files found in directory");
        return Ok(DeadCodeResult::empty());
    }

    // Step 2-4: Build graph, detect entry points, and analyze reachability
    let graph = get_symbol_graph(&files, &config, path)?;
    let entry_points = get_entry_points(&graph)?;
    let reachable = analyze_reachability(&graph, entry_points)?;

    // Step 5: Identify dead code
    let dead_symbols = reachability::identify_dead_code(&graph, &reachable);
    tracing::info!(dead_symbol_count = dead_symbols.len(), "Dead code identified");

    // Step 6: Calculate confidence and build result
    tracing::info!("Calculating confidence scores");
    let calculator = confidence::ConfidenceCalculator::new(graph.clone());
    let (files, stats) = process_dead_symbols(dead_symbols, &calculator);
    let ratio = calculate_dead_code_ratio(&graph, stats.total_dead_loc);

    Ok(build_result(files, stats, ratio))
}

/// Load graph from cache or build it from files
#[tracing::instrument(skip(files, config))]
fn load_or_build_graph(
    files: &[PathBuf],
    config: &AnalysisConfig,
    root_path: &Path,
) -> Result<symbol_graph::SymbolGraph, AnalysisError> {
    let cache_dir = config
        .cache_dir
        .clone()
        .unwrap_or_else(|| root_path.join(".code-viz").join("cache"));

    let cache = cache::SymbolGraphCache::new(&cache_dir)?;

    // Check if cache is stale
    let is_stale = cache.invalidate_if_stale(files)?;

    if !is_stale {
        // Try to load from cache
        if let Some(graph) = cache.load()? {
            tracing::info!("Loaded symbol graph from cache");
            return Ok(graph);
        }
    }

    tracing::info!("Building fresh symbol graph");
    let graph = build_graph_from_files(files)?;

    // Save to cache
    cache.save(&graph)?;
    tracing::info!("Saved symbol graph to cache");

    Ok(graph)
}

/// Build symbol graph from files using parallel processing
#[tracing::instrument(skip(files))]
fn build_graph_from_files(files: &[PathBuf]) -> Result<symbol_graph::SymbolGraph, AnalysisError> {
    use rayon::prelude::*;

    // Read all files in parallel
    tracing::info!(file_count = files.len(), "Reading source files");

    let file_contents: Result<Vec<_>, _> = files
        .par_iter()
        .map(|path| {
            std::fs::read_to_string(path)
                .map(|content| (path.clone(), content))
                .map_err(|e| {
                    tracing::error!(path = %path.display(), error = %e, "Failed to read file");
                    e
                })
        })
        .collect();

    let file_contents = file_contents?;

    // Build the graph
    let mut builder = symbol_graph::SymbolGraphBuilder::new();
    let graph = builder.build_graph(file_contents)?;

    Ok(graph)
}

#[cfg(test)]
mod tests {
    //! Integration tests for dead code analysis
    //!
    //! Note: unwrap() is acceptable in test code because test panics indicate
    //! test fixture or assertion failures, which is the expected behavior.

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_analyze_dead_code_integration() {
        // Create temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        // Create main.ts (entry point)
        fs::write(
            src_dir.join("main.ts"),
            r#"
import { usedFunction } from './used';

function main() {
    usedFunction();
}

main();
"#,
        )
        .unwrap();

        // Create used.ts (partially used)
        fs::write(
            src_dir.join("used.ts"),
            r#"
export function usedFunction() {
    console.log('I am used');
}

export function deadFunction() {
    console.log('I am never called');
}
"#,
        )
        .unwrap();

        // Create dead.ts (completely unused)
        fs::write(
            src_dir.join("dead.ts"),
            r#"
export function completelyUnused() {
    console.log('Nobody uses me');
}

export class UnusedClass {
    method() {
        console.log('Never called');
    }
}
"#,
        )
        .unwrap();

        // Run analysis
        let result = analyze_dead_code(&src_dir, None).unwrap();

        // Verify results
        assert!(result.summary.total_files > 0, "Should analyze files");
        assert!(
            result.summary.files_with_dead_code > 0,
            "Should find dead code"
        );
        assert!(
            result.summary.dead_functions >= 2,
            "Should find at least 2 dead functions"
        );

        // Print results for debugging
        eprintln!("Dead functions: {}", result.summary.dead_functions);
        eprintln!("Dead classes: {}", result.summary.dead_classes);
        eprintln!(
            "Files with dead code: {}",
            result.summary.files_with_dead_code
        );
    }

    #[test]
    fn test_filter_by_confidence() {
        let result = DeadCodeResult {
            summary: DeadCodeSummary {
                total_files: 2,
                files_with_dead_code: 2,
                dead_functions: 3,
                dead_classes: 0,
                total_dead_loc: 30,
                dead_code_ratio: 0.5,
            },
            files: vec![FileDeadCode {
                path: PathBuf::from("test.ts"),
                dead_code: vec![
                    DeadSymbol {
                        symbol: "highConfidence".to_string(),
                        kind: models::SymbolKind::Function,
                        line_start: 1,
                        line_end: 10,
                        loc: 10,
                        confidence: 95,
                        reason: "Test".to_string(),
                        last_modified: None,
                    },
                    DeadSymbol {
                        symbol: "lowConfidence".to_string(),
                        kind: models::SymbolKind::Function,
                        line_start: 11,
                        line_end: 20,
                        loc: 10,
                        confidence: 50,
                        reason: "Test".to_string(),
                        last_modified: None,
                    },
                ],
            }],
        };

        let filtered = result.filter_by_confidence(80);

        assert_eq!(filtered.summary.dead_functions, 1);
        assert_eq!(filtered.summary.total_dead_loc, 10);
        assert_eq!(filtered.files.len(), 1);
        assert_eq!(filtered.files[0].dead_code.len(), 1);
        assert_eq!(filtered.files[0].dead_code[0].symbol, "highConfidence");
    }
}
