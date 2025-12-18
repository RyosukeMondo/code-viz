//! Symbol graph construction from source code.
//!
//! This module handles extracting symbols (functions, classes, imports, exports)
//! from source files using Tree-sitter and building a dependency graph showing
//! import/export relationships.

mod builder;
mod extractors;
mod queries;
mod resolver;

#[cfg(test)]
mod tests;

pub use builder::SymbolGraphBuilder;

use crate::models::{Symbol, SymbolId};
use ahash::AHashMap as HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// Error type for symbol graph operations
#[derive(Debug, Error)]
pub enum GraphError {
    /// Failed to parse source code
    #[error("Parse error in {file}: {message}")]
    ParseError {
        /// File that failed to parse
        file: PathBuf,
        /// Error message
        message: String,
    },

    /// Failed to resolve import
    #[error("Failed to resolve import: {0}")]
    ImportResolutionError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Symbol graph containing all symbols and their relationships
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolGraph {
    /// All symbols indexed by their ID
    pub symbols: HashMap<SymbolId, Symbol>,

    /// Import relationships: symbol -> list of symbols it imports/depends on
    pub imports: HashMap<SymbolId, Vec<SymbolId>>,

    /// Exported symbols per file: file path -> list of exported symbol IDs
    pub exports: HashMap<PathBuf, Vec<SymbolId>>,
}
