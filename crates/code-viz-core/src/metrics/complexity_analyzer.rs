//! Cognitive Complexity Analysis Module
//!
//! Analyzes code complexity using cognitive complexity metrics.
//! Detects functions, calculates complexity per function, and aggregates statistics.

use crate::models::{CognitiveComplexity, FunctionComplexity};
use crate::parser::LanguageParser;
use tree_sitter::Node;

/// Calculate cognitive complexity for source code
///
/// # Arguments
/// * `source` - Source code text
/// * `parser` - Language parser implementing LanguageParser trait
///
/// # Returns
/// Cognitive complexity analysis including per-function metrics,
/// or None if no functions found
pub fn analyze_complexity(
    source: &str,
    parser: &dyn LanguageParser,
) -> Option<CognitiveComplexity> {
    let tree = parser.parse(source).ok()?;
    let root = tree.root_node();

    let mut functions = Vec::new();

    // Find all functions in the file
    let mut cursor = root.walk();
    let mut visited_children = false;

    loop {
        let node = cursor.node();

        // Check if this is a function node
        if is_function_node(&node) {
            let name = extract_function_name(&node, source).unwrap_or_else(|| "anonymous".to_string());
            let start_line = node.start_position().row + 1;
            let end_line = node.end_position().row + 1;
            let complexity = calculate_function_complexity(&node, source, 0);

            functions.push(FunctionComplexity {
                name,
                complexity,
                start_line,
                end_line,
            });
        }

        // Navigate tree
        if (!visited_children && cursor.goto_first_child()) || cursor.goto_next_sibling() {
            visited_children = false;
        } else if cursor.goto_parent() {
            visited_children = true;
        } else {
            break;
        }
    }

    if functions.is_empty() {
        return None;
    }

    let total_complexity: usize = functions.iter().map(|f| f.complexity).sum();
    let max_complexity = functions.iter().map(|f| f.complexity).max().unwrap_or(0);
    let average_complexity = total_complexity as f64 / functions.len() as f64;

    Some(CognitiveComplexity {
        total_complexity,
        average_complexity,
        max_complexity,
        functions,
    })
}

/// Check if node represents a function declaration
fn is_function_node(node: &Node) -> bool {
    matches!(
        node.kind(),
        "function_declaration" | "function" | "function_item" | "method_declaration" |
        "arrow_function" | "function_definition" | "method_definition"
    )
}

/// Extract function name from AST node
fn extract_function_name(node: &Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "property_identifier" {
            return child.utf8_text(source.as_bytes()).ok().map(|s| s.to_string());
        }
    }
    None
}

/// Calculate cognitive complexity for a function
///
/// Cognitive complexity increases with:
/// - Control flow structures (+1 + nesting level)
/// - Logical operators in conditions (+1 each)
/// - Jump statements (+1 each)
///
/// # Arguments
/// * `node` - Function AST node
/// * `source` - Source code text
/// * `nesting_level` - Current nesting depth
fn calculate_function_complexity(node: &Node, source: &str, nesting_level: usize) -> usize {
    let mut complexity = 0;
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();

        // Control flow structures (+1 + nesting level)
        if matches!(
            kind,
            "if_statement" | "else_clause" | "for_statement" | "while_statement" |
            "do_statement" | "switch_statement" | "case_clause" | "catch_clause" |
            "for_in_statement" | "for_of_statement" | "conditional_expression"
        ) {
            complexity += 1 + nesting_level;
            // Recursively calculate nested complexity
            complexity += calculate_function_complexity(&child, source, nesting_level + 1);
            continue;
        }

        // Logical operators in conditions (+1 each)
        if kind == "binary_expression" {
            if let Ok(text) = child.utf8_text(source.as_bytes()) {
                complexity += text.matches("&&").count() + text.matches("||").count();
            }
        }

        // Jump statements (+1): break, continue, goto, throw
        if matches!(kind, "break_statement" | "continue_statement" | "goto_statement" | "throw_statement") {
            complexity += 1;
        }

        // Recursively process children (except for nodes we already handled)
        complexity += calculate_function_complexity(&child, source, nesting_level);
    }

    complexity
}
