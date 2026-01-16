/// Language plugins module.
///
/// Contains implementations of the LanguageProvider trait for different languages.
#[cfg(feature = "rust")]
pub mod rust;

#[cfg(feature = "typescript")]
pub mod typescript;

#[cfg(feature = "python")]
pub mod python;
