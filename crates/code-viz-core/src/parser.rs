use thiserror::Error;
use tree_sitter::Tree;

pub trait LanguageParser: Send + Sync {
    fn language(&self) -> &str;
    fn parse(&self, source: &str) -> Result<Tree, ParseError>;
    fn count_functions(&self, tree: &Tree) -> usize;
}

pub struct TypeScriptParser;
impl LanguageParser for TypeScriptParser {
    fn language(&self) -> &str { "typescript" }
    fn parse(&self, _source: &str) -> Result<Tree, ParseError> {
        todo!("Implement Tree-sitter parsing")
    }
    fn count_functions(&self, _tree: &Tree) -> usize {
        todo!("Implement function counting via queries")
    }
}

pub struct JavaScriptParser;
impl LanguageParser for JavaScriptParser {
    fn language(&self) -> &str { "javascript" }
    fn parse(&self, _source: &str) -> Result<Tree, ParseError> {
        todo!("Implement Tree-sitter parsing")
    }
    fn count_functions(&self, _tree: &Tree) -> usize {
        todo!("Implement function counting via queries")
    }
}

pub fn get_parser(_language: &str) -> Result<Box<dyn LanguageParser>, ParseError> {
    todo!("Return correct parser for language")
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Tree-sitter parse failed: {0}")]
    TreeSitterError(String),
}
