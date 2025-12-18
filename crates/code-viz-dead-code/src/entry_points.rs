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
pub fn detect_entry_points(graph: &SymbolGraph) -> Vec<SymbolId> {
    let mut entry_points = Vec::new();

    // Iterate through all symbols and check if they are entry points
    for (symbol_id, symbol) in &graph.symbols {
        if is_entry_point(symbol, &symbol.path) {
            entry_points.push(symbol_id.clone());
        }
    }

    // Also include all exported symbols from entry files
    for (file_path, exported_symbols) in &graph.exports {
        if is_entry_file(file_path) {
            for symbol_id in exported_symbols {
                if !entry_points.contains(symbol_id) {
                    entry_points.push(symbol_id.clone());
                }
            }
        }
    }

    entry_points
}

/// Check if a symbol is an entry point based on heuristics
///
/// # Arguments
/// * `symbol` - The symbol to check
/// * `path` - File path containing the symbol
///
/// # Returns
/// True if symbol should be considered an entry point
fn is_entry_point(symbol: &Symbol, path: &Path) -> bool {
    // Heuristic 1: Functions named "main" are entry points
    if symbol.name == "main" {
        return true;
    }

    // Heuristic 2: All symbols in test files are entry points
    if is_test_file(path) {
        return true;
    }

    // Heuristic 3: Exported symbols in entry files are entry points
    if symbol.is_exported && is_entry_file(path) {
        return true;
    }

    false
}

/// Check if a file is a test file
///
/// Test files are identified by common patterns:
/// - *.test.ts, *.test.tsx, *.test.js, *.test.jsx
/// - *.spec.ts, *.spec.tsx, *.spec.js, *.spec.jsx
///
/// # Arguments
/// * `path` - File path to check
///
/// # Returns
/// True if the file is a test file
fn is_test_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        file_name.contains(".test.") || file_name.contains(".spec.")
    } else {
        false
    }
}

/// Check if a file is an entry file
///
/// Entry files are common entry points for applications and libraries:
/// - main.ts, main.tsx, main.js, main.jsx
/// - index.ts, index.tsx, index.js, index.jsx
/// - lib.rs (Rust)
/// - Files in src/ directory with these names
///
/// # Arguments
/// * `path` - File path to check
///
/// # Returns
/// True if the file is an entry file
fn is_entry_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        matches!(
            file_name,
            "main.ts"
                | "main.tsx"
                | "main.js"
                | "main.jsx"
                | "index.ts"
                | "index.tsx"
                | "index.js"
                | "index.jsx"
                | "lib.rs"
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SymbolKind;
    use std::path::PathBuf;

    fn create_test_symbol(name: &str, path: PathBuf, is_exported: bool) -> Symbol {
        // Use name as part of ID to ensure uniqueness
        Symbol {
            id: format!("{}:{}:1", path.display(), name),
            name: name.to_string(),
            kind: SymbolKind::Function,
            path,
            line_start: 1,
            line_end: 10,
            is_exported,
            is_test: false,
        }
    }

    #[test]
    fn test_detect_main_ts() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        let main_path = PathBuf::from("src/main.ts");
        let symbol = create_test_symbol("handleClick", main_path.clone(), true);
        graph.symbols.insert(symbol.id.clone(), symbol);

        let entry_points = detect_entry_points(&graph);
        assert_eq!(entry_points.len(), 1);
    }

    #[test]
    fn test_detect_index_ts() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        let index_path = PathBuf::from("src/index.ts");
        let symbol = create_test_symbol("init", index_path.clone(), true);
        graph.symbols.insert(symbol.id.clone(), symbol);

        let entry_points = detect_entry_points(&graph);
        assert_eq!(entry_points.len(), 1);
    }

    #[test]
    fn test_detect_test_files() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        // Add symbol in test file
        let test_path = PathBuf::from("src/utils.test.ts");
        let test_symbol = create_test_symbol("testHelper", test_path.clone(), false);
        graph.symbols.insert(test_symbol.id.clone(), test_symbol);

        // Add symbol in spec file
        let spec_path = PathBuf::from("src/app.spec.ts");
        let spec_symbol = create_test_symbol("specHelper", spec_path.clone(), false);
        graph.symbols.insert(spec_symbol.id.clone(), spec_symbol);

        let entry_points = detect_entry_points(&graph);
        assert_eq!(entry_points.len(), 2);
    }

    #[test]
    fn test_detect_exported_symbols() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        let index_path = PathBuf::from("src/index.ts");
        let exported_symbol = create_test_symbol("publicApi", index_path.clone(), true);
        let unexported_symbol = create_test_symbol("privateHelper", index_path.clone(), false);

        graph.symbols.insert(exported_symbol.id.clone(), exported_symbol.clone());
        graph.symbols.insert(unexported_symbol.id.clone(), unexported_symbol);
        graph.exports.insert(index_path, vec![exported_symbol.id.clone()]);

        let entry_points = detect_entry_points(&graph);
        // Only the exported symbol should be an entry point
        assert_eq!(entry_points.len(), 1);
        assert!(entry_points.contains(&exported_symbol.id));
    }

    #[test]
    fn test_no_entry_points() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        // Add a regular, unexported symbol in a non-entry file
        let regular_path = PathBuf::from("src/utils.ts");
        let symbol = create_test_symbol("helper", regular_path, false);
        graph.symbols.insert(symbol.id.clone(), symbol);

        let entry_points = detect_entry_points(&graph);
        assert_eq!(entry_points.len(), 0);
    }

    #[test]
    fn test_is_test_file() {
        assert!(is_test_file(&PathBuf::from("src/app.test.ts")));
        assert!(is_test_file(&PathBuf::from("src/utils.spec.ts")));
        assert!(is_test_file(&PathBuf::from("tests/integration.test.js")));
        assert!(is_test_file(&PathBuf::from("__tests__/unit.spec.tsx")));
        assert!(!is_test_file(&PathBuf::from("src/app.ts")));
        assert!(!is_test_file(&PathBuf::from("src/index.ts")));
    }

    #[test]
    fn test_is_entry_file() {
        assert!(is_entry_file(&PathBuf::from("src/main.ts")));
        assert!(is_entry_file(&PathBuf::from("src/index.ts")));
        assert!(is_entry_file(&PathBuf::from("index.js")));
        assert!(is_entry_file(&PathBuf::from("main.jsx")));
        assert!(is_entry_file(&PathBuf::from("lib.rs")));
        assert!(!is_entry_file(&PathBuf::from("src/app.ts")));
        assert!(!is_entry_file(&PathBuf::from("src/utils.ts")));
    }

    #[test]
    fn test_main_function_detection() {
        let mut graph = SymbolGraph {
            symbols: std::collections::HashMap::new(),
            imports: std::collections::HashMap::new(),
            exports: std::collections::HashMap::new(),
        };

        let regular_path = PathBuf::from("src/app.ts");
        let main_symbol = create_test_symbol("main", regular_path.clone(), false);
        let other_symbol = create_test_symbol("helper", regular_path, false);

        graph.symbols.insert(main_symbol.id.clone(), main_symbol.clone());
        graph.symbols.insert(other_symbol.id.clone(), other_symbol);

        let entry_points = detect_entry_points(&graph);
        // Only the main function should be an entry point
        assert_eq!(entry_points.len(), 1);
        assert!(entry_points.contains(&main_symbol.id));
    }
}
