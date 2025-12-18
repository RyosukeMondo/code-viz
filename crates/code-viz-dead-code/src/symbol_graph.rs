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
#[derive(Debug, Clone)]
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

impl SymbolGraphBuilder {
    /// Create a new symbol graph builder
    pub fn new() -> Self {
        todo!("Will implement in task 1.1.2")
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

    /// Build complete symbol graph from multiple files
    ///
    /// # Arguments
    /// * `files` - List of (file_path, source_code) tuples
    ///
    /// # Returns
    /// Complete symbol graph with all relationships
    pub fn build_graph(&mut self, _files: Vec<(PathBuf, String)>) -> Result<SymbolGraph, GraphError> {
        todo!("Will implement in task 1.1.2")
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
}
