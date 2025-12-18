//! Symbol extraction utilities for parsing Tree-sitter nodes.

use std::path::Path;

/// Extract the name from a Tree-sitter node
pub(super) fn extract_symbol_name(node: &tree_sitter::Node, source: &str, kind: &str) -> String {
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
                            return subchild
                                .utf8_text(source.as_bytes())
                                .unwrap_or("")
                                .to_string();
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
pub(super) fn is_symbol_exported(node: &tree_sitter::Node, source: &str) -> bool {
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
pub(super) fn is_test_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains(".test.")
        || path_str.contains(".spec.")
        || path_str.contains("__tests__")
        || path_str.contains("/test/")
        || path_str.contains("/tests/")
        || path_str.starts_with("test/")
        || path_str.starts_with("tests/")
}
