use crate::language::provider::LanguageProvider;

/// Python language provider.
///
/// Provides tree-sitter parsing and queries for Python source files.
pub struct PythonLanguageProvider;

impl LanguageProvider for PythonLanguageProvider {
    fn name(&self) -> &str {
        "python"
    }

    fn file_extensions(&self) -> &[&str] {
        &["py"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_python::language()
    }

    fn coupling_query(&self) -> &str {
        include_str!("../queries/python.scm")
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
    fn test_python_provider_name() {
        let provider = PythonLanguageProvider;
        assert_eq!(provider.name(), "python");
    }

    #[test]
    fn test_python_provider_extensions() {
        let provider = PythonLanguageProvider;
        assert_eq!(provider.file_extensions(), &["py"]);
    }

    #[test]
    fn test_python_provider_query_not_empty() {
        let provider = PythonLanguageProvider;
        assert!(!provider.coupling_query().is_empty());
    }

    #[test]
    fn test_python_provider_language() {
        let provider = PythonLanguageProvider;
        let language = provider.tree_sitter_language();
        assert_eq!(language.version(), tree_sitter_python::language().version());
    }
}
