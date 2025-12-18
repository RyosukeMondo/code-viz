//! Reachability analysis via depth-first search.
//!
//! This module performs DFS traversal from entry points to identify
//! all reachable symbols in the codebase. Unreachable symbols are
//! considered dead code.

use crate::models::{Symbol, SymbolId};
use crate::symbol_graph::SymbolGraph;
use ahash::AHashSet as HashSet;
use thiserror::Error;

/// Error type for reachability analysis
#[derive(Debug, Error)]
pub enum ReachabilityError {
    /// Invalid symbol ID referenced
    #[error("Invalid symbol ID: {0}")]
    InvalidSymbolId(SymbolId),

    /// No entry points provided
    #[error("No entry points provided for analysis")]
    NoEntryPoints,
}

/// Reachability analyzer that performs DFS from entry points
pub struct ReachabilityAnalyzer {
    /// The symbol graph to analyze
    graph: SymbolGraph,

    /// Set of visited symbols during DFS
    visited: HashSet<SymbolId>,
}

impl ReachabilityAnalyzer {
    /// Create a new reachability analyzer
    ///
    /// # Arguments
    /// * `graph` - The symbol graph to analyze
    pub fn new(graph: SymbolGraph) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
        }
    }

    /// Perform reachability analysis from given entry points
    ///
    /// # Arguments
    /// * `entry_points` - List of entry point symbol IDs to start DFS from
    ///
    /// # Returns
    /// Set of all reachable symbol IDs
    ///
    /// # Errors
    /// Returns error if no entry points provided
    pub fn analyze(
        &mut self,
        entry_points: Vec<SymbolId>,
    ) -> Result<HashSet<SymbolId>, ReachabilityError> {
        if entry_points.is_empty() {
            return Err(ReachabilityError::NoEntryPoints);
        }

        // Clear visited set for fresh analysis
        self.visited.clear();

        // Perform DFS from each entry point
        for entry_point in entry_points {
            self.dfs(&entry_point);
        }

        tracing::info!(
            "Reachability analysis complete: {} reachable symbols out of {} total",
            self.visited.len(),
            self.graph.symbols.len()
        );

        Ok(self.visited.clone())
    }

    /// Perform depth-first search from a symbol
    ///
    /// # Arguments
    /// * `symbol_id` - Starting symbol ID for DFS
    ///
    /// This method uses an iterative approach (instead of recursive)
    /// to avoid stack overflow on deep dependency chains or circular imports.
    fn dfs(&mut self, symbol_id: &SymbolId) {
        // Check if the symbol exists in the graph
        if !self.graph.symbols.contains_key(symbol_id) {
            // Entry point doesn't exist in graph, skip it
            return;
        }

        // Use a stack for iterative DFS (prevents stack overflow)
        let mut stack = vec![symbol_id.clone()];

        while let Some(current_id) = stack.pop() {
            // Skip if already visited (handles circular imports)
            if self.visited.contains(&current_id) {
                continue;
            }

            // Mark as visited
            self.visited.insert(current_id.clone());

            // Add all dependencies to the stack
            if let Some(dependencies) = self.graph.imports.get(&current_id) {
                for dep_id in dependencies {
                    if !self.visited.contains(dep_id) {
                        stack.push(dep_id.clone());
                    }
                }
            }
        }
    }
}

/// Identify dead code (unreachable symbols) in the symbol graph
///
/// # Arguments
/// * `graph` - The symbol graph
/// * `reachable` - Set of reachable symbol IDs
///
/// # Returns
/// List of dead (unreachable) symbols
pub fn identify_dead_code(graph: &SymbolGraph, reachable: &HashSet<SymbolId>) -> Vec<Symbol> {
    let mut dead_symbols = Vec::new();

    // Iterate through all symbols and find those not in reachable set
    for (symbol_id, symbol) in &graph.symbols {
        if !reachable.contains(symbol_id) {
            dead_symbols.push(symbol.clone());
        }
    }

    tracing::info!(
        "Dead code identification complete: {} dead symbols found out of {} total",
        dead_symbols.len(),
        graph.symbols.len()
    );

    dead_symbols
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SymbolKind;
    use ahash::AHashMap as HashMap;
    use std::path::PathBuf;

    /// Helper function to create a test symbol
    fn create_symbol(id: &str, name: &str, path: &str) -> Symbol {
        Symbol {
            id: id.to_string(),
            name: name.to_string(),
            kind: SymbolKind::Function,
            path: PathBuf::from(path),
            line_start: 1,
            line_end: 5,
            is_exported: false,
            is_test: false,
        }
    }

    /// Helper function to create a test symbol graph
    fn create_test_graph() -> SymbolGraph {
        let mut symbols = HashMap::new();
        let mut imports = HashMap::new();
        let exports = HashMap::new();

        // Create symbols: A -> B -> C, D is isolated (dead)
        symbols.insert("A".to_string(), create_symbol("A", "funcA", "a.ts"));
        symbols.insert("B".to_string(), create_symbol("B", "funcB", "b.ts"));
        symbols.insert("C".to_string(), create_symbol("C", "funcC", "c.ts"));
        symbols.insert("D".to_string(), create_symbol("D", "funcD", "d.ts"));

        // A imports B, B imports C
        imports.insert("A".to_string(), vec!["B".to_string()]);
        imports.insert("B".to_string(), vec!["C".to_string()]);

        SymbolGraph {
            symbols,
            imports,
            exports,
        }
    }

    #[test]
    fn test_dfs_simple_graph() {
        let graph = create_test_graph();
        let mut analyzer = ReachabilityAnalyzer::new(graph);

        // Start from A (entry point)
        let entry_points = vec!["A".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        // A, B, C should be reachable
        assert!(reachable.contains("A"), "A should be reachable");
        assert!(reachable.contains("B"), "B should be reachable");
        assert!(reachable.contains("C"), "C should be reachable");

        // D should not be reachable
        assert!(!reachable.contains("D"), "D should not be reachable");

        assert_eq!(
            reachable.len(),
            3,
            "Should have exactly 3 reachable symbols"
        );
    }

    #[test]
    fn test_circular_dependency_handling() {
        let mut symbols = HashMap::new();
        let mut imports = HashMap::new();
        let exports = HashMap::new();

        // Create circular dependency: A -> B -> C -> A
        symbols.insert("A".to_string(), create_symbol("A", "funcA", "a.ts"));
        symbols.insert("B".to_string(), create_symbol("B", "funcB", "b.ts"));
        symbols.insert("C".to_string(), create_symbol("C", "funcC", "c.ts"));

        imports.insert("A".to_string(), vec!["B".to_string()]);
        imports.insert("B".to_string(), vec!["C".to_string()]);
        imports.insert("C".to_string(), vec!["A".to_string()]); // Circular!

        let graph = SymbolGraph {
            symbols,
            imports,
            exports,
        };

        let mut analyzer = ReachabilityAnalyzer::new(graph);

        // Start from A
        let entry_points = vec!["A".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        // All three should be reachable (circular is ok)
        assert_eq!(
            reachable.len(),
            3,
            "All symbols in circular dependency should be reachable"
        );
        assert!(reachable.contains("A"));
        assert!(reachable.contains("B"));
        assert!(reachable.contains("C"));
    }

    #[test]
    fn test_identify_dead_code() {
        let graph = create_test_graph();
        let mut analyzer = ReachabilityAnalyzer::new(graph.clone());

        // Start from A
        let entry_points = vec!["A".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        // Identify dead code
        let dead = identify_dead_code(&graph, &reachable);

        // D should be dead
        assert_eq!(dead.len(), 1, "Should have exactly 1 dead symbol");
        assert_eq!(dead[0].id, "D", "Dead symbol should be D");
        assert_eq!(dead[0].name, "funcD", "Dead symbol name should be funcD");
    }

    #[test]
    fn test_entry_points_reachability() {
        let mut symbols = HashMap::new();
        let mut imports = HashMap::new();
        let exports = HashMap::new();

        // Create two separate dependency chains
        // Chain 1: Entry1 -> A -> B
        // Chain 2: Entry2 -> C -> D
        // E is isolated (dead)
        symbols.insert(
            "Entry1".to_string(),
            create_symbol("Entry1", "main1", "main1.ts"),
        );
        symbols.insert(
            "Entry2".to_string(),
            create_symbol("Entry2", "main2", "main2.ts"),
        );
        symbols.insert("A".to_string(), create_symbol("A", "funcA", "a.ts"));
        symbols.insert("B".to_string(), create_symbol("B", "funcB", "b.ts"));
        symbols.insert("C".to_string(), create_symbol("C", "funcC", "c.ts"));
        symbols.insert("D".to_string(), create_symbol("D", "funcD", "d.ts"));
        symbols.insert("E".to_string(), create_symbol("E", "funcE", "e.ts"));

        imports.insert("Entry1".to_string(), vec!["A".to_string()]);
        imports.insert("A".to_string(), vec!["B".to_string()]);
        imports.insert("Entry2".to_string(), vec!["C".to_string()]);
        imports.insert("C".to_string(), vec!["D".to_string()]);

        let graph = SymbolGraph {
            symbols,
            imports,
            exports,
        };

        let mut analyzer = ReachabilityAnalyzer::new(graph.clone());

        // Multiple entry points
        let entry_points = vec!["Entry1".to_string(), "Entry2".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        // All except E should be reachable
        assert_eq!(reachable.len(), 6, "Should have 6 reachable symbols");
        assert!(reachable.contains("Entry1"));
        assert!(reachable.contains("Entry2"));
        assert!(reachable.contains("A"));
        assert!(reachable.contains("B"));
        assert!(reachable.contains("C"));
        assert!(reachable.contains("D"));
        assert!(!reachable.contains("E"), "E should not be reachable");

        // Verify E is identified as dead
        let dead = identify_dead_code(&graph, &reachable);
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0].id, "E");
    }

    #[test]
    fn test_no_entry_points_error() {
        let graph = create_test_graph();
        let mut analyzer = ReachabilityAnalyzer::new(graph);

        // Empty entry points should error
        let entry_points = vec![];
        let result = analyzer.analyze(entry_points);

        assert!(
            result.is_err(),
            "Should return error for empty entry points"
        );
        match result {
            Err(ReachabilityError::NoEntryPoints) => {
                // Expected error
            }
            _ => panic!("Expected NoEntryPoints error"),
        }
    }

    #[test]
    fn test_multiple_calls_to_analyze() {
        let graph = create_test_graph();
        let mut analyzer = ReachabilityAnalyzer::new(graph);

        // First analysis
        let entry_points1 = vec!["A".to_string()];
        let reachable1 = analyzer
            .analyze(entry_points1)
            .expect("First analysis should succeed");
        assert_eq!(reachable1.len(), 3);

        // Second analysis should clear previous state
        let entry_points2 = vec!["D".to_string()];
        let reachable2 = analyzer
            .analyze(entry_points2)
            .expect("Second analysis should succeed");

        // Only D should be reachable now
        assert_eq!(reachable2.len(), 1);
        assert!(reachable2.contains("D"));
    }

    #[test]
    fn test_empty_graph() {
        let graph = SymbolGraph {
            symbols: HashMap::new(),
            imports: HashMap::new(),
            exports: HashMap::new(),
        };

        let mut analyzer = ReachabilityAnalyzer::new(graph.clone());

        // Even with entry point, should handle gracefully
        let entry_points = vec!["NonExistent".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        // No symbols should be reachable (entry point doesn't exist)
        assert_eq!(
            reachable.len(),
            0,
            "No symbols should be reachable in empty graph"
        );

        // No dead code either
        let dead = identify_dead_code(&graph, &reachable);
        assert_eq!(dead.len(), 0);
    }

    #[test]
    fn test_symbol_with_no_imports() {
        let mut symbols = HashMap::new();
        let imports = HashMap::new();
        let exports = HashMap::new();

        // Single symbol with no dependencies
        symbols.insert(
            "Isolated".to_string(),
            create_symbol("Isolated", "isolated", "isolated.ts"),
        );

        let graph = SymbolGraph {
            symbols,
            imports,
            exports,
        };

        let mut analyzer = ReachabilityAnalyzer::new(graph.clone());

        // If Isolated is an entry point, it should be reachable
        let entry_points = vec!["Isolated".to_string()];
        let reachable = analyzer
            .analyze(entry_points)
            .expect("Analysis should succeed");

        assert_eq!(reachable.len(), 1);
        assert!(reachable.contains("Isolated"));

        // No dead code
        let dead = identify_dead_code(&graph, &reachable);
        assert_eq!(dead.len(), 0);
    }
}
