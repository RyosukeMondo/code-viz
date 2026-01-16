use crate::language::provider::LanguageProvider;

/// Rust language provider.
///
/// Provides tree-sitter parsing and queries for Rust source files.
pub struct RustLanguageProvider;

impl LanguageProvider for RustLanguageProvider {
    fn name(&self) -> &str {
        "rust"
    }

    fn file_extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_rust::language()
    }

    fn coupling_query(&self) -> &str {
        include_str!("../queries/rust.scm")
    }

    fn symbol_query(&self) -> &str {
        // Placeholder for symbol extraction query
        // Can be expanded in the future for extracting function/struct definitions
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_provider_name() {
        let provider = RustLanguageProvider;
        assert_eq!(provider.name(), "rust");
    }

    #[test]
    fn test_rust_provider_extensions() {
        let provider = RustLanguageProvider;
        assert_eq!(provider.file_extensions(), &["rs"]);
    }

    #[test]
    fn test_rust_provider_query_not_empty() {
        let provider = RustLanguageProvider;
        assert!(!provider.coupling_query().is_empty());
    }

    #[test]
    fn test_rust_provider_language() {
        let provider = RustLanguageProvider;
        let language = provider.tree_sitter_language();
        assert_eq!(language.version(), tree_sitter_rust::language().version());
    }
}
