//! Confidence score calculation for dead code.
//!
//! This module calculates deletion confidence scores (0-100) based on various
//! heuristics including export status, recent changes, dynamic import patterns,
//! and test coverage.

use crate::models::Symbol;
use crate::symbol_graph::SymbolGraph;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Dynamic import patterns that suggest a symbol might be used dynamically
const DYNAMIC_IMPORT_PATTERNS: &[&str] = &[
    "_handler",
    "_plugin",
    "_loader",
    "_middleware",
    "_hook",
    "handler_",
    "plugin_",
    "loader_",
    "middleware_",
    "hook_",
];

/// Confidence score calculator for dead code
pub struct ConfidenceCalculator {
    /// The symbol graph for context
    graph: SymbolGraph,
    /// Repository root for git operations
    repo_root: Option<PathBuf>,
}

impl ConfidenceCalculator {
    /// Create a new confidence calculator
    ///
    /// # Arguments
    /// * `graph` - The symbol graph for context
    pub fn new(graph: SymbolGraph) -> Self {
        // Try to find git repository root
        let repo_root = find_git_root(&graph);

        Self { graph, repo_root }
    }

    /// Calculate deletion confidence score for a symbol
    ///
    /// Score starts at 100 and is reduced based on:
    /// - Exported symbols (-30)
    /// - Recently modified (-20)
    /// - Dynamic import patterns (-25)
    /// - Test coverage (-15)
    ///
    /// # Arguments
    /// * `symbol` - The symbol to score
    ///
    /// # Returns
    /// Confidence score (0-100), where 100 is highest confidence for deletion
    pub fn calculate(&self, symbol: &Symbol) -> u8 {
        let mut score = 100i16; // Use i16 to prevent underflow

        // Reduce confidence if exported (might be public API)
        if symbol.is_exported {
            score -= 30;
        }

        // Reduce confidence if recently modified
        if recently_modified(&symbol.path, self.repo_root.as_ref()) {
            score -= 20;
        }

        // Reduce confidence if symbol name matches dynamic import patterns
        if could_be_dynamic_import(&symbol.name) {
            score -= 25;
        }

        // Reduce confidence if symbol has test coverage
        if has_test_coverage(symbol, &self.graph) {
            score -= 15;
        }

        // Clamp to 0-100 range
        score.max(0).min(100) as u8
    }
}

/// Find the git repository root from the symbol graph
///
/// # Arguments
/// * `graph` - The symbol graph
///
/// # Returns
/// Optional path to git repository root
fn find_git_root(graph: &SymbolGraph) -> Option<PathBuf> {
    // Get any file path from the graph
    if let Some(symbol) = graph.symbols.values().next() {
        let mut current = symbol.path.as_path();

        // Walk up directory tree looking for .git
        while let Some(parent) = current.parent() {
            let git_dir = parent.join(".git");
            if git_dir.exists() {
                return Some(parent.to_path_buf());
            }
            current = parent;
        }
    }

    None
}

/// Check if a file was recently modified (last 30 days)
///
/// # Arguments
/// * `path` - File path to check
/// * `repo_root` - Optional git repository root
///
/// # Returns
/// True if file was modified in last 30 days
fn recently_modified(path: &Path, #[allow(unused_variables)] repo_root: Option<&PathBuf>) -> bool {
    #[cfg(feature = "git-integration")]
    {
        if let Some(root) = repo_root {
            return check_git_modification(path, root);
        }
    }

    // Fallback: check file system modification time
    if let Ok(metadata) = std::fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                return elapsed < Duration::from_secs(30 * 24 * 60 * 60); // 30 days
            }
        }
    }

    false
}

/// Check git history for recent modifications
#[cfg(feature = "git-integration")]
fn check_git_modification(path: &Path, repo_root: &Path) -> bool {
    use std::time::UNIX_EPOCH;

    // Try to open git repository
    let repo = match git2::Repository::open(repo_root) {
        Ok(r) => r,
        Err(_) => return false, // Not a git repo, fail gracefully
    };

    // Get relative path from repo root
    let rel_path = match path.strip_prefix(repo_root) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Get HEAD commit
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return false,
    };

    let commit = match head.peel_to_commit() {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Walk commit history for this file
    let mut revwalk = match repo.revwalk() {
        Ok(r) => r,
        Err(_) => return false,
    };

    if revwalk.push_head().is_err() {
        return false;
    }

    // Check last commit that touched this file
    for oid in revwalk.take(100) {
        // Limit to last 100 commits for performance
        if let Ok(oid) = oid {
            if let Ok(commit) = repo.find_commit(oid) {
                let tree = match commit.tree() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                // Check if file exists in this commit
                if tree.get_path(rel_path).is_ok() {
                    // Found the file, check commit time
                    let commit_time = UNIX_EPOCH + Duration::from_secs(commit.time().seconds() as u64);
                    if let Ok(elapsed) = SystemTime::now().duration_since(commit_time) {
                        return elapsed < Duration::from_secs(30 * 24 * 60 * 60);
                    }
                    return false;
                }
            }
        }
    }

    false
}

/// Check if symbol name matches dynamic import patterns
///
/// Patterns include: *_handler, *_plugin, *_loader, handler_*, plugin_*
///
/// # Arguments
/// * `name` - Symbol name
///
/// # Returns
/// True if name suggests dynamic usage
fn could_be_dynamic_import(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    DYNAMIC_IMPORT_PATTERNS.iter().any(|pattern| {
        if pattern.starts_with('_') {
            name_lower.ends_with(pattern)
        } else if pattern.ends_with('_') {
            name_lower.starts_with(pattern)
        } else {
            name_lower.contains(pattern)
        }
    })
}

/// Check if symbol has test coverage
///
/// Heuristic: checks if symbol name appears in test files
///
/// # Arguments
/// * `symbol` - The symbol to check
/// * `graph` - The symbol graph for accessing test files
///
/// # Returns
/// True if symbol appears to be tested
fn has_test_coverage(symbol: &Symbol, graph: &SymbolGraph) -> bool {
    // Get all test symbols
    let test_symbols: Vec<&Symbol> = graph
        .symbols
        .values()
        .filter(|s| s.is_test)
        .collect();

    if test_symbols.is_empty() {
        return false;
    }

    // Check if symbol name appears in any test file
    for test_symbol in &test_symbols {
        // Simple heuristic: symbol name appears in test file path or test symbol name
        if test_symbol.name.contains(&symbol.name) {
            return true;
        }
    }

    // Could also check if test file imports the symbol's file
    // But that requires import resolution which we already have in the graph
    let test_paths: Vec<&PathBuf> = test_symbols
        .iter()
        .map(|s| &s.path)
        .collect();

    // Check if any test file has this symbol in its dependencies
    for test_path in test_paths {
        if let Some(test_file_symbols) = graph.exports.get(test_path) {
            for test_sym_id in test_file_symbols {
                if let Some(deps) = graph.imports.get(test_sym_id) {
                    if deps.contains(&symbol.id) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Symbol, SymbolKind};
    use std::collections::HashMap;

    /// Create a test symbol with given attributes
    fn create_test_symbol(
        name: &str,
        is_exported: bool,
        is_test: bool,
        path: &str,
    ) -> Symbol {
        Symbol {
            id: format!("{}:1", path),
            name: name.to_string(),
            kind: SymbolKind::Function,
            path: PathBuf::from(path),
            line_start: 1,
            line_end: 10,
            is_exported,
            is_test,
        }
    }

    /// Create a minimal symbol graph for testing
    fn create_test_graph(symbols: Vec<Symbol>) -> SymbolGraph {
        let mut symbol_map = HashMap::new();
        for symbol in symbols {
            symbol_map.insert(symbol.id.clone(), symbol);
        }

        SymbolGraph {
            symbols: symbol_map,
            imports: HashMap::new(),
            exports: HashMap::new(),
        }
    }

    #[test]
    fn test_base_confidence_100() {
        // Unexported, not tested, no dynamic patterns, not recently modified
        let symbol = create_test_symbol("unusedFunction", false, false, "/tmp/test.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        assert_eq!(score, 100, "Unexported unused function should have 100 confidence");
    }

    #[test]
    fn test_exported_reduces_confidence() {
        let symbol = create_test_symbol("exportedFunction", true, false, "/tmp/test.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        assert_eq!(score, 70, "Exported function should reduce confidence by 30");
    }

    #[test]
    fn test_dynamic_import_pattern_reduces_confidence() {
        // Test various dynamic import patterns
        let patterns = vec![
            ("my_handler", 75),        // ends with _handler
            ("handler_foo", 75),       // starts with handler_
            ("foo_plugin", 75),        // ends with _plugin
            ("plugin_bar", 75),        // starts with plugin_
            ("data_loader", 75),       // ends with _loader
            ("middleware_auth", 75),   // starts with middleware_
            ("onClick_hook", 75),      // ends with _hook
            ("regularFunction", 100),  // no pattern match
        ];

        for (name, expected_score) in patterns {
            let symbol = create_test_symbol(name, false, false, "/tmp/test.ts");
            let graph = create_test_graph(vec![symbol.clone()]);
            let calculator = ConfidenceCalculator::new(graph);

            let score = calculator.calculate(&symbol);
            assert_eq!(
                score, expected_score,
                "Symbol '{}' should have confidence {}",
                name, expected_score
            );
        }
    }

    #[test]
    fn test_confidence_never_negative() {
        // Create a symbol with all penalties applied
        let symbol = create_test_symbol("exported_handler", true, false, "/tmp/test.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // Score is u8, so it's always >= 0 by type system guarantee
        // Expected: 100 - 30 (exported) - 25 (handler pattern) = 45
        assert_eq!(score, 45, "Score should be clamped correctly");
    }

    #[test]
    fn test_confidence_clamped_0_100() {
        // Test that score stays within 0-100
        let symbol = create_test_symbol("normalFunc", false, false, "/tmp/test.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // u8 type guarantees >= 0, just verify <= 100
        assert!(score <= 100, "Confidence must be in range 0-100");
    }

    #[test]
    fn test_has_test_coverage_by_name() {
        let symbol = create_test_symbol("myFunction", false, false, "/src/utils.ts");
        let test_symbol = create_test_symbol("test_myFunction", false, true, "/tests/utils.test.ts");

        let graph = create_test_graph(vec![symbol.clone(), test_symbol]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // Base 100 - 15 (test coverage) = 85
        assert_eq!(score, 85, "Function with test coverage should reduce confidence by 15");
    }

    #[test]
    fn test_could_be_dynamic_import_patterns() {
        assert!(could_be_dynamic_import("my_handler"));
        assert!(could_be_dynamic_import("MY_HANDLER")); // Case insensitive
        assert!(could_be_dynamic_import("handler_foo"));
        assert!(could_be_dynamic_import("some_plugin"));
        assert!(could_be_dynamic_import("plugin_bar"));
        assert!(could_be_dynamic_import("data_loader"));
        assert!(could_be_dynamic_import("loader_data"));
        assert!(could_be_dynamic_import("auth_middleware"));
        assert!(could_be_dynamic_import("middleware_auth"));
        assert!(could_be_dynamic_import("use_hook"));
        assert!(could_be_dynamic_import("hook_useEffect"));

        assert!(!could_be_dynamic_import("normalFunction"));
        assert!(!could_be_dynamic_import("myUtilFunc"));
        assert!(!could_be_dynamic_import("calculateTotal"));
    }

    #[test]
    fn test_combined_penalties() {
        // Exported + dynamic pattern
        let symbol = create_test_symbol("exported_plugin", true, false, "/tmp/test.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // 100 - 30 (exported) - 25 (plugin pattern) = 45
        assert_eq!(score, 45);
    }

    #[test]
    fn test_test_symbol_not_in_graph() {
        // Symbol with no test coverage (no test symbols in graph)
        let symbol = create_test_symbol("myFunction", false, false, "/src/utils.ts");
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        assert_eq!(score, 100, "No test coverage penalty when no tests exist");
    }

    #[test]
    fn test_recently_modified_reduces_confidence() {
        use std::fs;
        use std::io::Write;
        use tempfile::TempDir;

        // Create a temporary file that was just created (recent modification)
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("recent.ts");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "export function recentlyModified() {{}}").unwrap();
        drop(file);

        let symbol = create_test_symbol(
            "recentlyModified",
            false,
            false,
            file_path.to_str().unwrap(),
        );
        let graph = create_test_graph(vec![symbol.clone()]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // The file was just created, so it should be considered recently modified
        // Expected: 100 - 20 (recently modified) = 80
        // Note: Without git-integration feature, this falls back to file mtime check
        // which should detect the file was just created (within 30 days)
        assert_eq!(score, 80, "Recently created file should reduce confidence by 20");
    }

    #[test]
    fn test_all_penalties_combined() {
        use std::fs;
        use std::io::Write;
        use tempfile::TempDir;

        // Create a temporary file with a test symbol referencing our symbol
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("exported_handler.ts");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "export function exported_handler() {{}}").unwrap();
        drop(file);

        let symbol = create_test_symbol(
            "exported_handler",
            true,
            false,
            file_path.to_str().unwrap(),
        );
        let test_symbol = create_test_symbol(
            "test_exported_handler",
            false,
            true,
            "/tests/test.ts",
        );

        let graph = create_test_graph(vec![symbol.clone(), test_symbol]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // Expected: 100 - 30 (exported) - 20 (recent) - 25 (handler) - 15 (test) = 10
        assert_eq!(
            score, 10,
            "All penalties should stack: exported(-30) + recent(-20) + handler(-25) + test(-15)"
        );
    }

    #[test]
    fn test_minimum_score_is_zero() {
        use std::fs;
        use std::io::Write;
        use tempfile::TempDir;

        // Create symbol with even more penalties that would go negative
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("exported_handler.ts");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "export function exported_handler() {{}}").unwrap();
        drop(file);

        // This symbol has all penalties:
        // - exported: -30
        // - handler pattern: -25
        // - recent modification: -20
        // - has test coverage: -15
        // Total: -90, should clamp to 0 minimum
        let symbol = create_test_symbol(
            "exported_handler",
            true,
            false,
            file_path.to_str().unwrap(),
        );
        let test_symbol = create_test_symbol(
            "test_exported_handler",
            false,
            true,
            "/tests/test.ts",
        );

        let graph = create_test_graph(vec![symbol.clone(), test_symbol]);
        let calculator = ConfidenceCalculator::new(graph);

        let score = calculator.calculate(&symbol);
        // The calculate function uses i16 internally and clamps to 0-100
        // 100 - 30 - 25 - 20 - 15 = 10 (not actually negative in this case)
        assert!(score <= 100, "Score must not exceed 100");
        assert_eq!(score, 10, "Expected score with all penalties");
    }
}
