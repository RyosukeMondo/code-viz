//! Tree-sitter query compilation for symbol and import extraction.

use super::GraphError;
use std::path::PathBuf;
use std::sync::OnceLock;
use tree_sitter::Query;

/// Get the Tree-sitter query for extracting symbols from a specific language
pub(super) fn get_symbol_query(language: &str) -> Result<&'static Query, GraphError> {
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
pub(super) fn get_import_query(language: &str) -> Result<&'static Query, GraphError> {
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
