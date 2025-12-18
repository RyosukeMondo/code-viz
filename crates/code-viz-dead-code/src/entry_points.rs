//! Entry point detection.
//!
//! This module identifies entry points in the codebase (main files,
//! exports, test files) from which reachability analysis begins.
//!
//! Entry points include:
//! - Main entry files (main.ts, index.ts, lib.rs)
//! - Functions named "main"
//! - All symbols in test files
//! - Exported symbols in entry files

use crate::models::{Symbol, SymbolId};
use crate::symbol_graph::SymbolGraph;
use std::path::Path;

/// Detect entry points in the symbol graph
///
/// Entry points are symbols where analysis should start. This includes:
/// - Main functions and entry files (main.ts, index.ts)
/// - All symbols in test files (*.test.ts, *.spec.ts)
/// - Exported symbols from entry files
///
/// # Arguments
/// * `graph` - The symbol graph to analyze
///
/// # Returns
/// List of symbol IDs that are entry points
pub fn detect_entry_points(_graph: &SymbolGraph) -> Vec<SymbolId> {
    todo!("Will implement in task 2.1.1")
}

/// Check if a symbol is an entry point based on heuristics
///
/// # Arguments
/// * `symbol` - The symbol to check
/// * `path` - File path containing the symbol
///
/// # Returns
/// True if symbol should be considered an entry point
fn is_entry_point(_symbol: &Symbol, _path: &Path) -> bool {
    todo!("Will implement in task 2.1.1")
}
