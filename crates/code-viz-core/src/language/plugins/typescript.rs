use crate::language::provider::LanguageProvider;

/// TypeScript/JavaScript language provider.
///
/// Provides tree-sitter parsing and queries for TypeScript and JavaScript source files.
pub struct TypeScriptLanguageProvider;

impl LanguageProvider for TypeScriptLanguageProvider {
    fn name(&self) -> &str {
        "typescript"
    }

    fn file_extensions(&self) -> &[&str] {
        &["ts", "tsx", "js", "jsx"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_typescript::language_typescript()
    }

    fn coupling_query(&self) -> &str {
        include_str!("../queries/typescript.scm")
    }

    fn symbol_query(&self) -> &str {
        // Placeholder for symbol extraction query
        // Can be expanded in the future for extracting function/class definitions
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_provider_name() {
        let provider = TypeScriptLanguageProvider;
        assert_eq!(provider.name(), "typescript");
    }

    #[test]
    fn test_typescript_provider_extensions() {
        let provider = TypeScriptLanguageProvider;
        let extensions = provider.file_extensions();
        assert!(extensions.contains(&"ts"));
        assert!(extensions.contains(&"tsx"));
        assert!(extensions.contains(&"js"));
        assert!(extensions.contains(&"jsx"));
    }

    #[test]
    fn test_typescript_provider_query_not_empty() {
        let provider = TypeScriptLanguageProvider;
        assert!(!provider.coupling_query().is_empty());
    }

    #[test]
    fn test_typescript_provider_language() {
        let provider = TypeScriptLanguageProvider;
        let language = provider.tree_sitter_language();
        assert_eq!(
            language.version(),
            tree_sitter_typescript::language_typescript().version()
        );
    }
}
