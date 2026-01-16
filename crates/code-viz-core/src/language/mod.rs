//! Language plugin system for extensible language support.
//!
//! This module provides a trait-based plugin system for adding language-specific
//! parsing and analysis capabilities without modifying core code.
//!
//! # Architecture
//!
//! - `LanguageProvider` trait: Interface for language plugins
//! - `LanguageRegistry`: Central registry for managing plugins
//! - `plugins/`: Built-in language implementations
//! - `queries/`: Tree-sitter query files for each language
//!
//! # Example
//!
//! ```rust,ignore
//! use code_viz_core::language::LanguageRegistry;
//!
//! let registry = LanguageRegistry::new();
//! if let Some(provider) = registry.get_by_extension("rs") {
//!     println!("Language: {}", provider.name());
//!     let query = provider.coupling_query();
//!     // Use query for parsing...
//! }
//! ```
//!
//! # Adding New Languages
//!
//! To add a new language:
//!
//! 1. Create a new module in `plugins/` (e.g., `plugins/go.rs`)
//! 2. Implement the `LanguageProvider` trait
//! 3. Add the tree-sitter query in `queries/` (e.g., `queries/go.scm`)
//! 4. Register the provider in `LanguageRegistry::new()`
//!
//! Zero changes to core analysis code are required.

pub mod provider;
pub mod registry;
pub mod plugins;

pub use provider::LanguageProvider;
pub use registry::LanguageRegistry;

#[cfg(feature = "rust")]
pub use plugins::rust::RustLanguageProvider;

#[cfg(feature = "typescript")]
pub use plugins::typescript::TypeScriptLanguageProvider;

#[cfg(feature = "python")]
pub use plugins::python::PythonLanguageProvider;
