//! Graph Management Module
//!
//! Handles loading symbol graphs from cache or building them from source files.
//! This module encapsulates all graph construction and caching logic.

use crate::{cache, symbol_graph, AnalysisConfig, AnalysisError};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// Manages symbol graph construction and caching
pub struct GraphManager {
    config: AnalysisConfig,
    root_path: PathBuf,
}

impl GraphManager {
    /// Create a new GraphManager with the given configuration
    pub fn new(config: AnalysisConfig, root_path: PathBuf) -> Self {
        Self { config, root_path }
    }

    /// Get a symbol graph, either from cache or by building from files
    pub fn load_or_build(
        &self,
        files: &[PathBuf],
    ) -> Result<symbol_graph::SymbolGraph, AnalysisError> {
        if self.config.enable_cache {
            self.load_or_build_cached(files)
        } else {
            Self::build_from_files(files)
        }
    }

    /// Load graph from cache or build it from files
    #[tracing::instrument(skip(self, files))]
    #[allow(clippy::cognitive_complexity)]
    fn load_or_build_cached(
        &self,
        files: &[PathBuf],
    ) -> Result<symbol_graph::SymbolGraph, AnalysisError> {
        let cache_dir = self
            .config
            .cache_dir
            .clone()
            .unwrap_or_else(|| self.root_path.join(".code-viz").join("cache"));

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
        let graph = Self::build_from_files(files)?;

        // Save to cache
        cache.save(&graph)?;
        tracing::info!("Saved symbol graph to cache");

        Ok(graph)
    }

    /// Build symbol graph from files using parallel processing
    #[tracing::instrument(skip(files))]
    pub fn build_from_files(files: &[PathBuf]) -> Result<symbol_graph::SymbolGraph, AnalysisError> {
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

        tracing::info!(
            symbol_count = graph.symbols.len(),
            "Symbol graph constructed"
        );
        Ok(graph)
    }

    /// Scan directory for source files
    pub fn scan_source_files(
        path: &Path,
        config: &AnalysisConfig,
    ) -> Result<Vec<PathBuf>, AnalysisError> {
        tracing::info!("Scanning directory for source files");
        let files = code_viz_core::scanner::scan_directory(path, &config.exclude_patterns)?;
        tracing::info!(file_count = files.len(), "Found source files");
        Ok(files)
    }
}
