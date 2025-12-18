//! Symbol graph construction from source code.
//!
//! This module handles extracting symbols (functions, classes, imports, exports)
//! from source files using Tree-sitter and building a dependency graph showing
//! import/export relationships.

use crate::models::{Symbol, SymbolId, SymbolKind};
use code_viz_core::parser::LanguageParser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use thiserror::Error;
use tree_sitter::{Query, QueryCursor};

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

/// Builder for constructing symbol graphs
pub struct SymbolGraphBuilder {
    graph: HashMap<SymbolId, Symbol>,
    dependencies: HashMap<SymbolId, Vec<SymbolId>>,
}

/// Get the Tree-sitter query for extracting symbols from a specific language
fn get_symbol_query(language: &str) -> Result<&'static Query, GraphError> {
    match language {
        "typescript" | "tsx" => {
            static TS_QUERY: OnceLock<Query> = OnceLock::new();
            Ok(TS_QUERY.get_or_init(|| {
                Query::new(
                    tree_sitter_typescript::language_typescript(),
                    r#"
                    (function_declaration) @function
                    (lexical_declaration
                        (variable_declarator
                            value: (arrow_function))) @arrow
                    (variable_declaration
                        (variable_declarator
                            value: (arrow_function))) @arrow
                    (class_declaration) @class
                    (method_definition) @method
                    "#,
                )
                .expect("Invalid TypeScript symbol query")
            }))
        }
        "javascript" | "jsx" => {
            static JS_QUERY: OnceLock<Query> = OnceLock::new();
            Ok(JS_QUERY.get_or_init(|| {
                Query::new(
                    tree_sitter_javascript::language(),
                    r#"
                    (function_declaration) @function
                    (lexical_declaration
                        (variable_declarator
                            value: (arrow_function))) @arrow
                    (variable_declaration
                        (variable_declarator
                            value: (arrow_function))) @arrow
                    (class_declaration) @class
                    (method_definition) @method
                    "#,
                )
                .expect("Invalid JavaScript symbol query")
            }))
        }
        _ => Err(GraphError::ParseError {
            file: PathBuf::new(),
            message: format!("Unsupported language for dead code analysis: {}", language),
        }),
    }
}

/// Get the Tree-sitter query for extracting imports from a specific language
fn get_import_query(language: &str) -> Result<&'static Query, GraphError> {
    match language {
        "typescript" | "tsx" => {
            static TS_QUERY: OnceLock<Query> = OnceLock::new();
            Ok(TS_QUERY.get_or_init(|| {
                Query::new(
                    tree_sitter_typescript::language_typescript(),
                    r#"
                    (import_statement
                        source: (string) @import_source)
                    "#,
                )
                .expect("Invalid TypeScript import query")
            }))
        }
        "javascript" | "jsx" => {
            static JS_QUERY: OnceLock<Query> = OnceLock::new();
            Ok(JS_QUERY.get_or_init(|| {
                Query::new(
                    tree_sitter_javascript::language(),
                    r#"
                    (import_statement
                        source: (string) @import_source)
                    "#,
                )
                .expect("Invalid JavaScript import query")
            }))
        }
        _ => Err(GraphError::ParseError {
            file: PathBuf::new(),
            message: format!("Unsupported language for imports: {}", language),
        }),
    }
}

/// Extract the name from a Tree-sitter node
fn extract_symbol_name(node: &tree_sitter::Node, source: &str, kind: &str) -> String {
    let mut cursor = node.walk();

    match kind {
        "function" => {
            // For function_declaration, look for name field
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    return child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                }
            }
        }
        "arrow" => {
            // For arrow functions in variable declarations, get the variable name
            // The node is the lexical_declaration, need to find the identifier
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    let mut child_cursor = child.walk();
                    for subchild in child.children(&mut child_cursor) {
                        if subchild.kind() == "identifier" {
                            return subchild.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                        }
                    }
                }
            }
        }
        "class" => {
            // For class_declaration, look for type_identifier or identifier
            for child in node.children(&mut cursor) {
                if child.kind() == "type_identifier" || child.kind() == "identifier" {
                    return child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                }
            }
        }
        "method" => {
            // For method_definition, look for property_identifier
            for child in node.children(&mut cursor) {
                if child.kind() == "property_identifier" {
                    return child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                }
            }
        }
        _ => {}
    }

    String::new()
}

/// Check if a symbol is exported by examining parent nodes
fn is_symbol_exported(node: &tree_sitter::Node, source: &str) -> bool {
    let mut current = *node;

    // Walk up the tree to find export declarations
    while let Some(parent) = current.parent() {
        let kind = parent.kind();

        // Check for export declarations
        if kind == "export_statement" {
            return true;
        }

        // Check for "export default"
        if kind == "export_statement" {
            let text = parent.utf8_text(source.as_bytes()).unwrap_or("");
            if text.starts_with("export default") || text.starts_with("export {") {
                return true;
            }
        }

        current = parent;
    }

    false
}

/// Check if a file is a test file based on its path
fn is_test_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains(".test.")
        || path_str.contains(".spec.")
        || path_str.contains("__tests__")
        || path_str.contains("/test/")
        || path_str.contains("/tests/")
        || path_str.starts_with("test/")
        || path_str.starts_with("tests/")
}

/// Resolve an import path relative to the importing file
///
/// Handles:
/// - Relative imports: "./utils" -> "../src/utils.ts"
/// - Package imports: "@/utils" or "~/utils" (TypeScript path aliases)
/// - Extension-less imports: "./utils" could be "./utils.ts" or "./utils/index.ts"
fn resolve_import_path(
    importer_path: &Path,
    import_source: &str,
    available_files: &HashMap<PathBuf, bool>,
) -> Option<PathBuf> {
    // Remove quotes from import source
    let import_source = import_source.trim_matches(|c| c == '"' || c == '\'');

    // Skip node_modules and package imports (e.g., "react", "lodash")
    if !import_source.starts_with('.') && !import_source.starts_with('/')
        && !import_source.starts_with("@/") && !import_source.starts_with("~/") {
        return None;
    }

    // Get the directory of the importing file
    let importer_dir = importer_path.parent()?;

    // Handle TypeScript path aliases (@/ and ~/ typically map to src/)
    let import_path_str = if import_source.starts_with("@/") || import_source.starts_with("~/") {
        import_source[2..].to_string()
    } else {
        import_source.to_string()
    };

    // Resolve the path relative to the importer
    let base_path = if import_path_str.starts_with("./") || import_path_str.starts_with("../") {
        importer_dir.join(&import_path_str)
    } else {
        // Assume path alias points to project root (simplified)
        PathBuf::from(&import_path_str)
    };

    // Try to resolve with common extensions
    let extensions = ["", ".ts", ".tsx", ".js", ".jsx"];
    for ext in &extensions {
        let candidate = if ext.is_empty() {
            base_path.clone()
        } else {
            base_path.with_extension(&ext[1..]) // Remove the leading dot
        };

        if available_files.contains_key(&candidate) {
            return Some(candidate);
        }
    }

    // Try index file resolution (import "./dir" -> "./dir/index.ts")
    for ext in &[".ts", ".tsx", ".js", ".jsx"] {
        let index_path = base_path.join(format!("index{}", ext));
        if available_files.contains_key(&index_path) {
            return Some(index_path);
        }
    }

    // Log warning for unresolved import but don't fail
    None
}

impl SymbolGraphBuilder {
    /// Create a new symbol graph builder
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Extract symbols from a single file using Tree-sitter
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `source` - Source code content
    /// * `parser` - Language parser (from code-viz-core)
    ///
    /// # Returns
    /// List of symbols found in the file
    pub fn extract_symbols(
        &mut self,
        path: &Path,
        source: &str,
        parser: &dyn LanguageParser,
    ) -> Result<Vec<Symbol>, GraphError> {
        // Parse the source code
        let tree = parser.parse(source).map_err(|e| GraphError::ParseError {
            file: path.to_path_buf(),
            message: e.to_string(),
        })?;

        let mut symbols = Vec::new();
        let is_test = is_test_file(path);

        // Get the appropriate query based on language
        let query = get_symbol_query(parser.language())?;
        let mut cursor = QueryCursor::new();

        // Execute the query on the tree
        let matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                // Determine symbol kind based on capture name
                let kind = match capture_name.as_str() {
                    "function" => SymbolKind::Function,
                    "arrow" => SymbolKind::ArrowFunction,
                    "class" => SymbolKind::Class,
                    "method" => SymbolKind::Method,
                    "variable" => SymbolKind::Variable,
                    _ => continue,
                };

                // Extract symbol name from the node
                let name = extract_symbol_name(&node, source, capture_name);
                if name.is_empty() {
                    continue; // Skip anonymous functions
                }

                // Check if symbol is exported
                let is_exported = is_symbol_exported(&node, source);

                // Get line range
                let start_point = node.start_position();
                let end_point = node.end_position();
                let line_start = start_point.row + 1; // Convert to 1-indexed
                let line_end = end_point.row + 1;

                // Create unique symbol ID
                let id = format!("{}:{}:{}", path.display(), line_start, name);

                symbols.push(Symbol {
                    id,
                    name,
                    kind,
                    path: path.to_path_buf(),
                    line_start,
                    line_end,
                    is_exported,
                    is_test,
                });
            }
        }

        Ok(symbols)
    }

    /// Extract import paths from a file
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `source` - Source code content
    /// * `parser` - Language parser
    ///
    /// # Returns
    /// List of import source strings (e.g., "./utils", "@/components")
    fn extract_imports(
        &self,
        path: &Path,
        source: &str,
        parser: &dyn LanguageParser,
    ) -> Result<Vec<String>, GraphError> {
        // Parse the source code
        let tree = parser.parse(source).map_err(|e| GraphError::ParseError {
            file: path.to_path_buf(),
            message: e.to_string(),
        })?;

        let mut imports = Vec::new();

        // Get the appropriate query based on language
        let query = get_import_query(parser.language())?;
        let mut cursor = QueryCursor::new();

        // Execute the query on the tree
        let matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let import_source = node.utf8_text(source.as_bytes()).unwrap_or("");
                if !import_source.is_empty() {
                    imports.push(import_source.to_string());
                }
            }
        }

        Ok(imports)
    }

    /// Build complete symbol graph from multiple files
    ///
    /// # Arguments
    /// * `files` - List of (file_path, source_code) tuples
    ///
    /// # Returns
    /// Complete symbol graph with all relationships
    pub fn build_graph(&mut self, files: Vec<(PathBuf, String)>) -> Result<SymbolGraph, GraphError> {
        // Pre-allocate capacity
        let file_count = files.len();
        let mut all_symbols: HashMap<SymbolId, Symbol> = HashMap::with_capacity(file_count * 10);
        let mut imports: HashMap<SymbolId, Vec<SymbolId>> = HashMap::new();
        let mut exports: HashMap<PathBuf, Vec<SymbolId>> = HashMap::with_capacity(file_count);

        // Build a map of available files for import resolution
        let available_files: HashMap<PathBuf, bool> = files.iter()
            .map(|(path, _)| (path.clone(), true))
            .collect();

        // First pass: Extract all symbols from all files
        for (file_path, source) in &files {
            // Determine the parser based on file extension
            let parser: Box<dyn LanguageParser> = if file_path.extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "ts" || s == "tsx")
                .unwrap_or(false)
            {
                Box::new(code_viz_core::parser::TypeScriptParser)
            } else {
                Box::new(code_viz_core::parser::JavaScriptParser)
            };

            // Extract symbols
            let symbols = self.extract_symbols(file_path, source, parser.as_ref())?;

            // Track exported symbols per file
            let mut file_exports = Vec::new();

            for symbol in symbols {
                if symbol.is_exported {
                    file_exports.push(symbol.id.clone());
                }
                all_symbols.insert(symbol.id.clone(), symbol);
            }

            if !file_exports.is_empty() {
                exports.insert(file_path.clone(), file_exports);
            }
        }

        // Second pass: Build import relationships
        for (file_path, source) in &files {
            let parser: Box<dyn LanguageParser> = if file_path.extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "ts" || s == "tsx")
                .unwrap_or(false)
            {
                Box::new(code_viz_core::parser::TypeScriptParser)
            } else {
                Box::new(code_viz_core::parser::JavaScriptParser)
            };

            // Extract imports
            let import_sources = self.extract_imports(file_path, source, parser.as_ref())?;

            // Resolve import paths to actual files
            for import_source in import_sources {
                if let Some(resolved_path) = resolve_import_path(
                    file_path,
                    &import_source,
                    &available_files,
                ) {
                    // Find exported symbols from the imported file
                    if let Some(exported_symbols) = exports.get(&resolved_path) {
                        // Get all symbols in the current file that could depend on these imports
                        let file_symbols: Vec<&Symbol> = all_symbols.values()
                            .filter(|s| s.path == *file_path)
                            .collect();

                        // For simplicity, mark all symbols in the importing file as depending
                        // on all exported symbols from the imported file
                        // (More sophisticated analysis would parse which specific imports are used where)
                        for symbol in file_symbols {
                            imports
                                .entry(symbol.id.clone())
                                .or_insert_with(Vec::new)
                                .extend(exported_symbols.clone());
                        }
                    }
                }
            }
        }

        Ok(SymbolGraph {
            symbols: all_symbols,
            imports,
            exports,
        })
    }
}

impl Default for SymbolGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::parser::TypeScriptParser;

    #[test]
    fn test_extract_typescript_functions() {
        let source = r#"
            function regularFunction() {
                return 42;
            }

            const arrowFunc = () => {
                return 'hello';
            };
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        assert_eq!(symbols.len(), 2);
        assert!(symbols.iter().any(|s| s.name == "regularFunction" && s.kind == SymbolKind::Function));
        assert!(symbols.iter().any(|s| s.name == "arrowFunc" && s.kind == SymbolKind::ArrowFunction));
    }

    #[test]
    fn test_extract_classes_and_methods() {
        let source = r#"
            class MyClass {
                myMethod() {
                    return 'test';
                }
            }
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        assert_eq!(symbols.len(), 2);
        assert!(symbols.iter().any(|s| s.name == "MyClass" && s.kind == SymbolKind::Class));
        assert!(symbols.iter().any(|s| s.name == "myMethod" && s.kind == SymbolKind::Method));
    }

    #[test]
    fn test_exported_symbols() {
        let source = r#"
            export function exportedFunc() {
                return 1;
            }

            function privateFunc() {
                return 2;
            }

            export default class ExportedClass {
            }
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        let exported_func = symbols.iter().find(|s| s.name == "exportedFunc").unwrap();
        assert!(exported_func.is_exported);

        let private_func = symbols.iter().find(|s| s.name == "privateFunc").unwrap();
        assert!(!private_func.is_exported);

        let exported_class = symbols.iter().find(|s| s.name == "ExportedClass").unwrap();
        assert!(exported_class.is_exported);
    }

    #[test]
    fn test_test_file_detection() {
        assert!(is_test_file(Path::new("src/utils.test.ts")));
        assert!(is_test_file(Path::new("src/components.spec.tsx")));
        assert!(is_test_file(Path::new("src/__tests__/utils.ts")));
        assert!(is_test_file(Path::new("tests/integration.ts")));
        assert!(!is_test_file(Path::new("src/utils.ts")));
    }

    #[test]
    fn test_symbol_line_numbers() {
        let source = r#"
function first() {
    return 1;
}

const second = () => {
    return 2;
};
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        let first = symbols.iter().find(|s| s.name == "first").unwrap();
        assert_eq!(first.line_start, 2); // 1-indexed

        let second = symbols.iter().find(|s| s.name == "second").unwrap();
        assert_eq!(second.line_start, 6);
    }

    #[test]
    fn test_skip_anonymous_functions() {
        let source = r#"
            [1, 2, 3].map(() => {
                return 42;
            });
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        // Anonymous functions should be skipped
        assert_eq!(symbols.len(), 0);
    }

    #[test]
    fn test_symbol_id_generation() {
        let source = r#"
            function testFunc() {}
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("/home/user/project/test.ts");
        let mut builder = SymbolGraphBuilder {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        };

        let symbols = builder.extract_symbols(path, source, &parser).unwrap();

        assert_eq!(symbols.len(), 1);
        let symbol = &symbols[0];
        assert!(symbol.id.contains("/home/user/project/test.ts"));
        assert!(symbol.id.contains("testFunc"));
        assert!(symbol.id.contains(":2:")); // Line number
    }

    #[test]
    fn test_build_graph_simple() {
        let mut builder = SymbolGraphBuilder::new();

        let files = vec![
            (
                PathBuf::from("src/utils.ts"),
                r#"
                export function helper() {
                    return 42;
                }
                "#.to_string(),
            ),
            (
                PathBuf::from("src/main.ts"),
                r#"
                import { helper } from "./utils";

                function main() {
                    helper();
                }
                "#.to_string(),
            ),
        ];

        let graph = builder.build_graph(files).unwrap();

        // Check that symbols were extracted
        assert!(graph.symbols.len() >= 2);

        // Check that exports were tracked
        assert!(graph.exports.contains_key(&PathBuf::from("src/utils.ts")));

        // Check that helper function was found
        let helper_symbol = graph.symbols.values()
            .find(|s| s.name == "helper");
        assert!(helper_symbol.is_some());
    }

    #[test]
    fn test_build_graph_multi_file() {
        let mut builder = SymbolGraphBuilder::new();

        let files = vec![
            (
                PathBuf::from("a.ts"),
                r#"
                export function funcA() {}
                "#.to_string(),
            ),
            (
                PathBuf::from("b.ts"),
                r#"
                import { funcA } from "./a";
                export function funcB() {
                    funcA();
                }
                "#.to_string(),
            ),
            (
                PathBuf::from("c.ts"),
                r#"
                import { funcB } from "./b";
                function funcC() {
                    funcB();
                }
                "#.to_string(),
            ),
        ];

        let graph = builder.build_graph(files).unwrap();

        // All three functions should be in the graph
        assert!(graph.symbols.values().any(|s| s.name == "funcA"));
        assert!(graph.symbols.values().any(|s| s.name == "funcB"));
        assert!(graph.symbols.values().any(|s| s.name == "funcC"));

        // Check exports
        assert!(graph.exports.contains_key(&PathBuf::from("a.ts")));
        assert!(graph.exports.contains_key(&PathBuf::from("b.ts")));
    }

    #[test]
    fn test_build_graph_circular_imports() {
        let mut builder = SymbolGraphBuilder::new();

        let files = vec![
            (
                PathBuf::from("a.ts"),
                r#"
                import { funcB } from "./b";
                export function funcA() {
                    funcB();
                }
                "#.to_string(),
            ),
            (
                PathBuf::from("b.ts"),
                r#"
                import { funcA } from "./a";
                export function funcB() {
                    funcA();
                }
                "#.to_string(),
            ),
        ];

        // Should not panic or infinite loop on circular imports
        let graph = builder.build_graph(files).unwrap();

        assert!(graph.symbols.values().any(|s| s.name == "funcA"));
        assert!(graph.symbols.values().any(|s| s.name == "funcB"));
    }

    #[test]
    fn test_extract_imports() {
        let source = r#"
            import { foo } from "./foo";
            import * as bar from "../bar";
            import type { Baz } from "@/types";
        "#;

        let parser = TypeScriptParser;
        let path = Path::new("test.ts");
        let builder = SymbolGraphBuilder::new();

        let imports = builder.extract_imports(path, source, &parser).unwrap();

        assert_eq!(imports.len(), 3);
        assert!(imports.iter().any(|i| i.contains("./foo")));
        assert!(imports.iter().any(|i| i.contains("../bar")));
        assert!(imports.iter().any(|i| i.contains("@/types")));
    }

    #[test]
    fn test_resolve_relative_imports() {
        let mut available = HashMap::new();
        available.insert(PathBuf::from("src/utils.ts"), true);
        available.insert(PathBuf::from("src/components/Button.tsx"), true);

        let importer = Path::new("src/main.ts");

        // Resolve "./utils" to "src/utils.ts"
        let resolved = resolve_import_path(importer, "\"./utils\"", &available);
        assert_eq!(resolved, Some(PathBuf::from("src/utils.ts")));

        // Resolve "./components/Button" to "src/components/Button.tsx"
        let resolved = resolve_import_path(importer, "\"./components/Button\"", &available);
        assert_eq!(resolved, Some(PathBuf::from("src/components/Button.tsx")));
    }
}
