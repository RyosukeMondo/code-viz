/// Trait for providing language-specific parsing capabilities.
///
/// Each language plugin implements this trait to provide tree-sitter
/// language configuration, file extension mappings, and queries for
/// extracting dependencies and symbols.
///
/// # Example
///
/// ```rust,ignore
/// use code_viz_core::language::LanguageProvider;
///
/// struct RustLanguageProvider;
///
/// impl LanguageProvider for RustLanguageProvider {
///     fn name(&self) -> &str {
///         "rust"
///     }
///
///     fn file_extensions(&self) -> &[&str] {
///         &["rs"]
///     }
///
///     fn tree_sitter_language(&self) -> tree_sitter::Language {
///         tree_sitter_rust::language()
///     }
///
///     fn coupling_query(&self) -> &str {
///         include_str!("../queries/rust.scm")
///     }
///
///     fn symbol_query(&self) -> &str {
///         // Return symbol extraction query
///         ""
///     }
/// }
/// ```
pub trait LanguageProvider: Send + Sync {
    /// Returns the canonical name of the language (e.g., "rust", "typescript").
    fn name(&self) -> &str;

    /// Returns the file extensions associated with this language (e.g., ["rs"], ["ts", "tsx"]).
    fn file_extensions(&self) -> &[&str];

    /// Returns the tree-sitter language instance for parsing.
    fn tree_sitter_language(&self) -> tree_sitter::Language;

    /// Returns the tree-sitter query string for extracting coupling dependencies.
    ///
    /// This query should capture import/use/require statements that represent
    /// dependencies between files.
    fn coupling_query(&self) -> &str;

    /// Returns the tree-sitter query string for extracting symbols.
    ///
    /// This query should capture function definitions, classes, and other
    /// top-level symbols for analysis.
    fn symbol_query(&self) -> &str;
}
