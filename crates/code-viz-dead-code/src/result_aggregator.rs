//! Result Aggregation Module
//!
//! Processes dead symbols into structured results with confidence scores
//! and generates summary statistics.

use crate::{confidence::ConfidenceCalculator, models, symbol_graph::SymbolGraph, DeadCodeResult, DeadCodeSummary, DeadSymbol, FileDeadCode};
use ahash::AHashMap as HashMap;
use std::path::PathBuf;

/// Internal statistics for dead code aggregation
struct DeadCodeStats {
    total_dead_loc: usize,
    dead_functions: usize,
    dead_classes: usize,
}

/// Aggregates dead code analysis results
pub struct ResultAggregator {
    graph: SymbolGraph,
    calculator: ConfidenceCalculator,
}

impl ResultAggregator {
    /// Create a new ResultAggregator with the given symbol graph
    pub fn new(graph: SymbolGraph) -> Self {
        let calculator = ConfidenceCalculator::new(graph.clone());
        Self { graph, calculator }
    }

    /// Aggregate dead symbols into a structured result
    pub fn aggregate(&self, dead_symbols: Vec<models::Symbol>) -> DeadCodeResult {
        tracing::info!("Calculating confidence scores");
        let (files, stats) = self.process_dead_symbols(dead_symbols);
        let ratio = self.calculate_dead_code_ratio(stats.total_dead_loc);
        self.build_result(files, stats, ratio)
    }

    /// Process dead symbols into file-grouped results with confidence scores
    fn process_dead_symbols(
        &self,
        dead_symbols: Vec<models::Symbol>,
    ) -> (Vec<FileDeadCode>, DeadCodeStats) {
        let mut files_map: HashMap<PathBuf, Vec<DeadSymbol>> = HashMap::new();
        let mut stats = DeadCodeStats {
            total_dead_loc: 0,
            dead_functions: 0,
            dead_classes: 0,
        };

        for symbol in dead_symbols {
            let confidence = self.calculator.calculate(&symbol);
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
    fn calculate_dead_code_ratio(&self, total_dead_loc: usize) -> f64 {
        let total_loc: usize = self
            .graph
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
        &self,
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
}
