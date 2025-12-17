use std::cell::RefCell;
use std::sync::OnceLock;
use thiserror::Error;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};

pub trait LanguageParser: Send + Sync {
    fn language(&self) -> &str;
    fn parse(&self, source: &str) -> Result<Tree, ParseError>;
    fn count_functions(&self, tree: &Tree) -> usize;
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range>;
}

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

fn parse_with_language(language: Language, source: &str) -> Result<Tree, ParseError> {
    PARSER.with(|p| {
        let mut p = p.borrow_mut();
        p.set_language(language)
            .map_err(|e| ParseError::TreeSitterError(e.to_string()))?;
        p.parse(source, None)
            .ok_or_else(|| ParseError::TreeSitterError("Failed to parse source".to_string()))
    })
}

pub struct TypeScriptParser;
impl LanguageParser for TypeScriptParser {
    fn language(&self) -> &str {
        "typescript"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_typescript::language_typescript(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_typescript::language_typescript(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f"
            ).expect("Invalid TypeScript query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_typescript::language_typescript(),
                "(comment) @c"
            ).expect("Invalid TypeScript comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct TsxParser;
impl LanguageParser for TsxParser {
    fn language(&self) -> &str {
        "tsx"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_typescript::language_tsx(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_typescript::language_tsx(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f"
            ).expect("Invalid TSX query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_typescript::language_tsx(),
                "(comment) @c"
            ).expect("Invalid TSX comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct JavaScriptParser;
impl LanguageParser for JavaScriptParser {
    fn language(&self) -> &str {
        "javascript"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_javascript::language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_javascript::language(),
                "(function_declaration) @f (arrow_function) @f (method_definition) @f"
            ).expect("Invalid JavaScript query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_javascript::language(),
                "(comment) @c"
            ).expect("Invalid JavaScript comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct RustParser;
impl LanguageParser for RustParser {
    fn language(&self) -> &str {
        "rust"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_rust::language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_rust::language(),
                "(function_item) @f"
            ).expect("Invalid Rust query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_rust::language(),
                "(line_comment) @c (block_comment) @c"
            ).expect("Invalid Rust comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct PythonParser;
impl LanguageParser for PythonParser {
    fn language(&self) -> &str {
        "python"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_python::language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_python::language(),
                "(function_definition) @f"
            ).expect("Invalid Python query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_python::language(),
                "(comment) @c"
            ).expect("Invalid Python comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct GoParser;
impl LanguageParser for GoParser {
    fn language(&self) -> &str {
        "go"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_go::language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_go::language(),
                "(function_declaration) @f (method_declaration) @f (func_literal) @f"
            ).expect("Invalid Go query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_go::language(),
                "(comment) @c"
            ).expect("Invalid Go comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub struct CppParser;
impl LanguageParser for CppParser {
    fn language(&self) -> &str {
        "cpp"
    }
    fn parse(&self, source: &str) -> Result<Tree, ParseError> {
        parse_with_language(tree_sitter_cpp::language(), source)
    }
    fn count_functions(&self, tree: &Tree) -> usize {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_cpp::language(),
                "(function_declaration) @f"
            ).expect("Invalid C++ query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8]).count()
    }
    fn find_comment_ranges(&self, tree: &Tree) -> Vec<tree_sitter::Range> {
        static QUERY: OnceLock<Query> = OnceLock::new();
        let query = QUERY.get_or_init(|| {
            Query::new(
                tree_sitter_cpp::language(),
                "(comment) @c"
            ).expect("Invalid C++ comment query")
        });
        
        let mut cursor = QueryCursor::new();
        cursor.matches(query, tree.root_node(), &[] as &[u8])
            .map(|m| m.captures[0].node.range())
            .collect()
    }
}

pub fn get_parser(language: &str) -> Result<Box<dyn LanguageParser>, ParseError> {
    match language {
        "typescript" | "ts" => Ok(Box::new(TypeScriptParser)),
        "javascript" | "js" | "jsx" => Ok(Box::new(JavaScriptParser)),
        "tsx" => Ok(Box::new(TsxParser)),
        "rust" | "rs" => Ok(Box::new(RustParser)),
        "python" | "py" => Ok(Box::new(PythonParser)),
        "go" => Ok(Box::new(GoParser)),
        "cpp" | "cxx" | "cc" | "hpp" | "h" => Ok(Box::new(CppParser)),
        _ => Err(ParseError::UnsupportedLanguage(language.to_string())),
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Tree-sitter parse failed: {0}")]
    TreeSitterError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_typescript() {
        let parser = get_parser("typescript").unwrap();
        let source = "function hello() { console.log('world'); }";
        let tree = parser.parse(source).unwrap();
        assert!(tree.root_node().has_error() == false);
    }

    #[test]
    fn test_parse_syntax_error() {
        let parser = get_parser("typescript").unwrap();
        let source = "function hello() { return "; // Missing brace
        let tree = parser.parse(source).unwrap();
        // tree-sitter usually produces a tree even with errors, but has_error() should be true
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_count_functions_typescript() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            function a() {}
            const b = () => {};
            class C {
                m() {}
            }
        "#;
        let tree = parser.parse(source).unwrap();
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_functions_javascript() {
        let parser = get_parser("javascript").unwrap();
        let source = r#"
            function a() {}
            const b = () => {};
            class C {
                m() {}
            }
        "#;
        let tree = parser.parse(source).unwrap();
        let count = parser.count_functions(&tree);
        assert_eq!(count, 3);
    }
    
    #[test]
    fn test_count_functions_tsx() {
        let parser = get_parser("tsx").unwrap();
        let source = r#"
            const Component = () => <div></div>;
            function helper() {}
        "#;
        let tree = parser.parse(source).unwrap();
        let count = parser.count_functions(&tree);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_snapshot_typescript_ast() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            function greet(name: string) {
                console.log(`Hello, ${name}`);
            }
        "#;
        let tree = parser.parse(source).unwrap();
        insta::assert_debug_snapshot!(tree.root_node());
    }

    #[test]
    fn test_snapshot_javascript_ast() {
        let parser = get_parser("javascript").unwrap();
        let source = r#"
            class Person {
                constructor(name) {
                    this.name = name;
                }
            }
        "#;
        let tree = parser.parse(source).unwrap();
        insta::assert_debug_snapshot!(tree.root_node());
    }
}
