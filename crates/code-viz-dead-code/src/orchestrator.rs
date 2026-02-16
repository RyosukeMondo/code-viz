//! Dead Code Analysis Orchestrator
//!
//! Coordinates the analysis pipeline by delegating to specialized modules.
//! This is the main coordination point that ties together graph management,
//! reachability analysis, and result aggregation.

use crate::{
    analyzer::ReachabilityAnalyzer, graph_manager::GraphManager,
    result_aggregator::ResultAggregator, AnalysisConfig, AnalysisError, DeadCodeResult,
};
use std::path::{Path, PathBuf};

/// Orchestrates the dead code analysis pipeline
pub struct DeadCodeOrchestrator {
    config: AnalysisConfig,
    root_path: PathBuf,
}

impl DeadCodeOrchestrator {
    /// Create a new orchestrator with the given configuration
    pub fn new(root_path: &Path, config: AnalysisConfig) -> Self {
        Self {
            config,
            root_path: root_path.to_path_buf(),
        }
    }

    /// Run the complete analysis pipeline
    pub fn analyze(&self) -> Result<DeadCodeResult, AnalysisError> {
        tracing::info!("Starting dead code analysis");

        // Step 1: Scan source files
        let files = GraphManager::scan_source_files(&self.root_path, &self.config)?;
        if files.is_empty() {
            tracing::warn!("No source files found in directory");
            return Ok(DeadCodeResult::empty());
        }

        // Step 2: Load or build symbol graph
        let graph_manager = GraphManager::new(self.config.clone(), self.root_path.clone());
        let graph = graph_manager.load_or_build(&files)?;

        // Step 3: Detect entry points and analyze reachability
        let analyzer = ReachabilityAnalyzer::new(graph.clone());
        let entry_points = analyzer.detect_entry_points()?;
        let reachable = analyzer.find_reachable(entry_points)?;

        // Step 4: Identify dead code
        let dead_symbols = analyzer.identify_dead_code(&reachable);

        // Step 5: Aggregate results with confidence scores
        let aggregator = ResultAggregator::new(graph);
        Ok(aggregator.aggregate(dead_symbols))
    }
}
