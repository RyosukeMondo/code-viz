//! Reachability Analysis Module
//!
//! Performs reachability analysis from entry points to identify which symbols
//! are used in the codebase. Separates entry point detection from reachability analysis.

use crate::{entry_points, models, reachability, symbol_graph::SymbolGraph, AnalysisError};
use ahash::AHashSet;

/// Orchestrates reachability analysis from entry points
pub struct ReachabilityAnalyzer {
    graph: SymbolGraph,
}

impl ReachabilityAnalyzer {
    /// Create a new ReachabilityAnalyzer with the given symbol graph
    pub fn new(graph: SymbolGraph) -> Self {
        Self { graph }
    }

    /// Detect entry points in the symbol graph
    pub fn detect_entry_points(&self) -> Result<Vec<models::SymbolId>, AnalysisError> {
        tracing::info!("Detecting entry points");
        let entry_points = entry_points::detect_entry_points(&self.graph);

        if entry_points.is_empty() {
            tracing::error!("No entry points found in codebase");
            return Err(AnalysisError::NoEntryPoints);
        }

        tracing::info!(entry_point_count = entry_points.len(), "Entry points detected");
        Ok(entry_points)
    }

    /// Perform reachability analysis from the given entry points
    pub fn find_reachable(
        &self,
        entry_points: Vec<models::SymbolId>,
    ) -> Result<AHashSet<models::SymbolId>, AnalysisError> {
        tracing::info!("Performing reachability analysis");
        let mut analyzer = reachability::ReachabilityAnalyzer::new(self.graph.clone());
        let reachable = analyzer.analyze(entry_points)?;

        tracing::info!(
            reachable_count = reachable.len(),
            total_count = self.graph.symbols.len(),
            "Reachability analysis complete"
        );
        Ok(reachable)
    }

    /// Identify dead code by finding symbols not in the reachable set
    pub fn identify_dead_code(&self, reachable: &AHashSet<models::SymbolId>) -> Vec<models::Symbol> {
        let dead_symbols = reachability::identify_dead_code(&self.graph, reachable);
        tracing::info!(dead_symbol_count = dead_symbols.len(), "Dead code identified");
        dead_symbols
    }

    /// Get the underlying symbol graph
    pub fn graph(&self) -> &SymbolGraph {
        &self.graph
    }
}
