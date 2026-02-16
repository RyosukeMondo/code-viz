// Allow expect in static query initialization - these are programming errors (invalid hardcoded
// queries), not runtime errors. If these panic, it's a bug in the code, not user input.
#![allow(clippy::expect_used)]

use std::cell::RefCell;
use std::sync::OnceLock;
use thiserror::Error;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};

pub trait LanguageParser: Send + Sync {
    fn language_key(&self) -> &'static str;
    fn get_language(&self) -> Language;
    fn parse(&self, source: &str) -> Result<Tree, ParseError>;
    fn count_functions(&self, tree: &Tree) -> usize;
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range>;
}

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

#[tracing::instrument(skip(language, source), fields(source_len = source.len()))]
fn parse_with_language(language: Language, source: &str) -> Result<Tree, ParseError> {
    tracing::debug!("Parsing source code with tree-sitter");

    PARSER.with(|p| {
        let mut p = p.borrow_mut();
        p.set_language(language).map_err(|e| {
            tracing::error!(error = %e, "Failed to set language");
            ParseError::TreeSitterError(e.to_string())
        })?;
        let tree = p.parse(source, None).ok_or_else(|| {
            tracing::error!("Failed to parse source");
            ParseError::TreeSitterError("Failed to parse source".to_string())
        })?;

        tracing::debug!(has_error = tree.root_node().has_error(), "Parse completed");
        Ok(tree)
    })
}

pub struct TypeScriptParser;
impl LanguageParser for TypeScriptParser {
    fn language_key(&self) -> &'static str {
        "typescript"
    }
    fn get_language(&self) -> Language {
        tree_sitter_typescript::language_typescript()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_typescript::language_typescript(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f",
            )
            .expect(
                "BUG: Invalid TypeScript query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_typescript::language_typescript(),
                "(comment) @c"
            ).expect("BUG: Invalid TypeScript comment query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct TsxParser;
impl LanguageParser for TsxParser {
    fn language_key(&self) -> &'static str {
        "tsx"
    }
    fn get_language(&self) -> Language {
        tree_sitter_typescript::language_tsx()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_typescript::language_tsx(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f",
            )
            .expect("BUG: Invalid TSX query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_typescript::language_tsx(), "(comment) @c").expect(
                "BUG: Invalid TSX comment query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct JavaScriptParser;
impl LanguageParser for JavaScriptParser {
    fn language_key(&self) -> &'static str {
        "javascript"
    }
    fn get_language(&self) -> Language {
        tree_sitter_javascript::language()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_javascript::language(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f",
            )
            .expect(
                "BUG: Invalid JavaScript query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_javascript::language(),
                "(comment) @c"
            ).expect("BUG: Invalid JavaScript comment query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct RustParser;
impl LanguageParser for RustParser {
    fn language_key(&self) -> &'static str {
        "rust"
    }
    fn get_language(&self) -> Language {
        tree_sitter_rust::language()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_rust::language(), "(function_item) @f").expect(
                "BUG: Invalid Rust query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_rust::language(),
                "(line_comment) @c (block_comment) @c"
            ).expect("BUG: Invalid Rust comment query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct PythonParser;
impl LanguageParser for PythonParser {
    fn language_key(&self) -> &'static str {
        "python"
    }
    fn get_language(&self) -> Language {
        tree_sitter_python::language()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_python::language(), "(function_definition) @f").expect(
                "BUG: Invalid Python query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_python::language(),
                "(comment) @c"
            ).expect("BUG: Invalid Python comment query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct GoParser;
impl LanguageParser for GoParser {
    fn language_key(&self) -> &'static str {
        "go"
    }
    fn get_language(&self) -> Language {
        tree_sitter_go::language()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(
                tree_sitter_go::language(),
                "(function_declaration) @f (method_declaration) @f (func_literal) @f",
            )
            .expect("BUG: Invalid Go query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_go::language(), "(comment) @c").expect(
                "BUG: Invalid Go comment query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct CppParser;
impl LanguageParser for CppParser {
    fn language_key(&self) -> &'static str {
        "cpp"
    }
    fn get_language(&self) -> Language {
        tree_sitter_cpp::language()
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(self.get_language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_cpp::language(), "(function_definition) @f")
                .expect("BUG: Invalid C++ query - this is a programming error, not a runtime error")
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            // Programming error: hardcoded query string must be valid
            Query::new(tree_sitter_cpp::language(), "(comment) @c").expect(
                "BUG: Invalid C++ comment query - this is a programming error, not a runtime error",
            )
        });

        let mut cursor = QueryCursor::new();
        cursor
            .matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

#[tracing::instrument]
#[allow(clippy::cognitive_complexity)]
pub fn get_parser(language: &str) -> Result<Box<dyn LanguageParser>, ParseError> {
    tracing::debug!("Creating parser for language");

    let parser: Box<dyn LanguageParser> = match language {
        "typescript" | "ts" => Box::new(TypeScriptParser),
        "javascript" | "js" | "jsx" => Box::new(JavaScriptParser),
        "tsx" => Box::new(TsxParser),
        "rust" | "rs" => Box::new(RustParser),
        "python" | "py" => Box::new(PythonParser),
        "go" => Box::new(GoParser),
        "cpp" | "cxx" | "cc" | "hpp" | "h" => Box::new(CppParser),
        _ => {
            tracing::warn!(language = %language, "Unsupported language requested");
            return Err(ParseError::UnsupportedLanguage(language.to_string()));
        }
    };

    tracing::debug!(
        parser_language = parser.language_key(),
        "Parser created successfully"
    );
    Ok(parser)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Tree-sitter parse failed: {0}")]
    TreeSitterError(String),
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_typescript() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = "function hello() { console.log('world'); }";
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        assert!(tree.root_node().has_error() == false);
    }

    #[test]
    fn test_parse_syntax_error() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = "function hello() { return "; // Missing brace
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
                                                  // tree-sitter usually produces a tree even with errors, but has_error() should be true
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_count_functions_typescript() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            function a() {}
            const b = () => {};
            class C {
                m() {}
            }
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_javascript() {
        let parser = get_parser("javascript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            function a() {}
            const b = () => {};
            class C {
                m() {}
            }
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_tsx() {
        let parser = get_parser("tsx").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            const Component = () => <div></div>;
            function helper() {}
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_snapshot_typescript_ast() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            function greet(name: string) {
                console.log(`Hello, ${name}`);
            }
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        insta::assert_debug_snapshot!(tree.root_node());
    }

    #[test]
    fn test_snapshot_javascript_ast() {
        let parser = get_parser("javascript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            class Person {
                constructor(name) {
                    this.name = name;
                }
            }
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        insta::assert_debug_snapshot!(tree.root_node());
    }

    #[test]
    fn test_count_functions_rust() {
        let parser = get_parser("rust").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
            fn main() {}
            fn helper() {}
            impl MyStruct {
                fn method(&self) {}
            }
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_python() {
        let parser = get_parser("python").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
def main():
    pass

def helper():
    pass

class MyClass:
    def method(self):
        pass
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_go() {
        let parser = get_parser("go").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
package main

func main() {}
func helper() {}
func (s *MyStruct) method() {}
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_cpp() {
        let parser = get_parser("cpp").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
int main() { return 0; }
void helper() {}
class MyClass {
    void method() {}
};
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_parse_error_handling() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = "function incomplete("; // Severely malformed code
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: tree-sitter always produces a tree
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_unsupported_language() {
        let result = get_parser("fortran");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Unsupported"));
        }
    }

    #[test]
    fn test_empty_source() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = "";
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: empty source is valid
        assert!(!tree.root_node().has_error());
        let count = parser.count_functions(&tree);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_find_comment_ranges_typescript() {
        let parser = get_parser("typescript").unwrap(); // Test-only unwrap: test data is known to be valid
        let source = r#"
// Line comment
function foo() {
    /* Block comment */
    return 42;
}
        "#;
        let tree = parser.parse(source).unwrap(); // Test-only unwrap: test data is known to be valid
        let comments = parser.find_comment_ranges(&tree);
        assert!(comments.len() >= 2); // At least 2 comments
    }
}
