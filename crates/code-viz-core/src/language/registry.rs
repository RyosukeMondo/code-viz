use std::collections::HashMap;
use super::provider::LanguageProvider;

/// Registry for managing language providers.
///
/// The registry maps file extensions to language providers, enabling
/// dynamic language support without modifying core analysis code.
///
/// # Example
///
/// ```rust,ignore
/// use code_viz_core::language::{LanguageRegistry, RustLanguageProvider};
///
/// let registry = LanguageRegistry::new();
/// if let Some(provider) = registry.get_by_extension("rs") {
///     println!("Language: {}", provider.name());
/// }
/// ```
pub struct LanguageRegistry {
    providers: HashMap<String, Box<dyn LanguageProvider>>,
    extension_map: HashMap<String, String>,
}

impl LanguageRegistry {
    /// Creates a new registry with all built-in language providers registered.
    pub fn new() -> Self {
        #[cfg(any(feature = "rust", feature = "typescript", feature = "python"))]
        let mut registry = Self {
            providers: HashMap::new(),
            extension_map: HashMap::new(),
        };

        #[cfg(not(any(feature = "rust", feature = "typescript", feature = "python")))]
        let registry = Self {
            providers: HashMap::new(),
            extension_map: HashMap::new(),
        };

        // Register built-in languages
        #[cfg(feature = "rust")]
        registry.register(Box::new(super::plugins::rust::RustLanguageProvider));

        #[cfg(feature = "typescript")]
        registry.register(Box::new(super::plugins::typescript::TypeScriptLanguageProvider));

        #[cfg(feature = "python")]
        registry.register(Box::new(super::plugins::python::PythonLanguageProvider));

        registry
    }

    /// Registers a language provider.
    ///
    /// This maps all file extensions from the provider to the provider's name
    /// and stores the provider in the registry.
    pub fn register(&mut self, provider: Box<dyn LanguageProvider>) {
        let name = provider.name().to_string();

        // Map all extensions to this provider's name
        for ext in provider.file_extensions() {
            self.extension_map.insert(ext.to_string(), name.clone());
        }

        self.providers.insert(name, provider);
    }

    /// Retrieves a language provider by file extension.
    ///
    /// # Arguments
    ///
    /// * `extension` - File extension without the dot (e.g., "rs", "ts")
    ///
    /// # Returns
    ///
    /// `Some(&dyn LanguageProvider)` if a provider is registered for this extension,
    /// `None` otherwise.
    pub fn get_by_extension(&self, extension: &str) -> Option<&dyn LanguageProvider> {
        let name = self.extension_map.get(extension)?;
        self.providers.get(name).map(|b| b.as_ref())
    }

    /// Retrieves a language provider by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Language name (e.g., "rust", "typescript")
    ///
    /// # Returns
    ///
    /// `Some(&dyn LanguageProvider)` if a provider is registered with this name,
    /// `None` otherwise.
    pub fn get_by_name(&self, name: &str) -> Option<&dyn LanguageProvider> {
        self.providers.get(name).map(|b| b.as_ref())
    }

    /// Returns an iterator over all registered language names.
    pub fn language_names(&self) -> impl Iterator<Item = &str> {
        self.providers.keys().map(|s| s.as_str())
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    struct TestLanguageProvider {
        name: &'static str,
        extensions: &'static [&'static str],
    }

    impl LanguageProvider for TestLanguageProvider {
        fn name(&self) -> &str {
            self.name
        }

        fn file_extensions(&self) -> &[&str] {
            self.extensions
        }

        fn tree_sitter_language(&self) -> tree_sitter::Language {
            // Return a dummy language for testing
            tree_sitter_rust::language()
        }

        fn coupling_query(&self) -> &str {
            ""
        }

        fn symbol_query(&self) -> &str {
            ""
        }
    }

    #[test]
    fn test_register_and_get_by_extension() {
        let mut registry = LanguageRegistry {
            providers: HashMap::new(),
            extension_map: HashMap::new(),
        };

        let provider = Box::new(TestLanguageProvider {
            name: "test",
            extensions: &["test1", "test2"],
        });

        registry.register(provider);

        assert!(registry.get_by_extension("test1").is_some());
        assert!(registry.get_by_extension("test2").is_some());
        assert!(registry.get_by_extension("unknown").is_none());

        let retrieved = registry.get_by_extension("test1").unwrap(); // Test-only unwrap: test data guaranteed
        assert_eq!(retrieved.name(), "test");
    }

    #[test]
    fn test_get_by_name() {
        let mut registry = LanguageRegistry {
            providers: HashMap::new(),
            extension_map: HashMap::new(),
        };

        let provider = Box::new(TestLanguageProvider {
            name: "test",
            extensions: &["test"],
        });

        registry.register(provider);

        assert!(registry.get_by_name("test").is_some());
        assert!(registry.get_by_name("unknown").is_none());

        let retrieved = registry.get_by_name("test").unwrap(); // Test-only unwrap: test data guaranteed
        assert_eq!(retrieved.name(), "test");
    }

    #[test]
    fn test_language_names() {
        let mut registry = LanguageRegistry {
            providers: HashMap::new(),
            extension_map: HashMap::new(),
        };

        registry.register(Box::new(TestLanguageProvider {
            name: "lang1",
            extensions: &["l1"],
        }));
        registry.register(Box::new(TestLanguageProvider {
            name: "lang2",
            extensions: &["l2"],
        }));

        let names: Vec<&str> = registry.language_names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"lang1"));
        assert!(names.contains(&"lang2"));
    }
}
