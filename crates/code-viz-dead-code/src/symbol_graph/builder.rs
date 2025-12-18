//! Symbol graph builder implementation.

use super::extractors::{extract_symbol_name, is_symbol_exported, is_test_file};
use super::queries::{get_import_query, get_symbol_query};
use super::resolver::resolve_import_path;
use super::{GraphError, SymbolGraph};
use crate::models::{Symbol, SymbolId, SymbolKind};
use ahash::AHashMap as HashMap;
use code_viz_core::parser::LanguageParser;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tree_sitter::QueryCursor;

/// Builder for constructing symbol graphs
pub struct SymbolGraphBuilder {
    graph: HashMap<SymbolId, Symbol>,
    dependencies: HashMap<SymbolId, Vec<SymbolId>>,
}

impl SymbolGraphBuilder {
    /// Create a new symbol graph builder
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Extract symbols from a single file using Tree-sitter
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `source` - Source code content
    /// * `parser` - Language parser (from code-viz-core)
    ///
    /// # Returns
    /// List of symbols found in the file
    pub fn extract_symbols(
        &mut self,
        path: &Path,
        source: &str,
        parser: &dyn LanguageParser,
    ) -> Result<Vec<Symbol>, GraphError> {
        // Parse the source code
        let tree = parser.parse(source).map_err(|e| GraphError::ParseError {
            file: path.to_path_buf(),
            message: e.to_string(),
        })?;

        let mut symbols = Vec::new();
        let is_test = is_test_file(path);

        // Get the appropriate query based on language
        let query = get_symbol_query(parser.language())?;
        let mut cursor = QueryCursor::new();

        // Execute the query on the tree
        let matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                // Determine symbol kind based on capture name
                let kind = match capture_name.as_str() {
                    "function" => SymbolKind::Function,
                    "arrow" => SymbolKind::ArrowFunction,
                    "class" => SymbolKind::Class,
                    "method" => SymbolKind::Method,
                    "variable" => SymbolKind::Variable,
                    _ => continue,
                };

                // Extract symbol name from the node
                let name = extract_symbol_name(&node, source, capture_name);
                if name.is_empty() {
                    continue; // Skip anonymous functions
                }

                // Check if symbol is exported
                let is_exported = is_symbol_exported(&node, source);

                // Get line range
                let start_point = node.start_position();
                let end_point = node.end_position();
                let line_start = start_point.row + 1; // Convert to 1-indexed
                let line_end = end_point.row + 1;

                // Create unique symbol ID
                let id = format!("{}:{}:{}", path.display(), line_start, name);

                symbols.push(Symbol {
                    id,
                    name,
                    kind,
                    path: path.to_path_buf(),
                    line_start,
                    line_end,
                    is_exported,
                    is_test,
                });
            }
        }

        Ok(symbols)
    }

    /// Extract import paths from a file
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `source` - Source code content
    /// * `parser` - Language parser
    ///
    /// # Returns
    /// List of import source strings (e.g., "./utils", "@/components")
    pub(crate) fn extract_imports(
        &self,
        path: &Path,
        source: &str,
        parser: &dyn LanguageParser,
    ) -> Result<Vec<String>, GraphError> {
        // Parse the source code
        let tree = parser.parse(source).map_err(|e| GraphError::ParseError {
            file: path.to_path_buf(),
            message: e.to_string(),
        })?;

        let mut imports = Vec::new();

        // Get the appropriate query based on language
        let query = get_import_query(parser.language())?;
        let mut cursor = QueryCursor::new();

        // Execute the query on the tree
        let matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let import_source = node.utf8_text(source.as_bytes()).unwrap_or("");
                if !import_source.is_empty() {
                    imports.push(import_source.to_string());
                }
            }
        }

        Ok(imports)
    }

    /// Build complete symbol graph from multiple files
    ///
    /// # Arguments
    /// * `files` - List of (file_path, source_code) tuples
    ///
    /// # Returns
    /// Complete symbol graph with all relationships
    pub fn build_graph(
        &mut self,
        files: Vec<(PathBuf, String)>,
    ) -> Result<SymbolGraph, GraphError> {
        // Pre-allocate capacity more accurately (estimate 20 symbols per file)
        let file_count = files.len();
        let estimated_symbols = file_count * 20;

        // Build a map of available files for import resolution
        let available_files: HashMap<PathBuf, bool> =
            files.iter().map(|(path, _)| (path.clone(), true)).collect();

        // Use thread-safe containers for parallel processing
        let all_symbols = Mutex::new(HashMap::with_capacity(estimated_symbols));
        let exports = Mutex::new(HashMap::with_capacity(file_count));

        // First pass: Extract all symbols from all files IN PARALLEL
        let symbol_results: Vec<Result<_, GraphError>> = files
            .par_iter()
            .map(|(file_path, source)| {
                // Determine the parser based on file extension
                let parser: Box<dyn LanguageParser> = if file_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == "ts" || s == "tsx")
                    .unwrap_or(false)
                {
                    Box::new(code_viz_core::parser::TypeScriptParser)
                } else {
                    Box::new(code_viz_core::parser::JavaScriptParser)
                };

                // Extract symbols (each thread gets its own builder)
                let mut builder = SymbolGraphBuilder::new();
                let symbols = builder.extract_symbols(file_path, source, parser.as_ref())?;

                // Track exported symbols per file
                let mut file_exports = Vec::new();
                for symbol in &symbols {
                    if symbol.is_exported {
                        file_exports.push(symbol.id.clone());
                    }
                }

                Ok((file_path.clone(), symbols, file_exports))
            })
            .collect();

        // Collect results and handle errors
        for result in symbol_results {
            let (file_path, symbols, file_exports) = result?;

            let mut all_symbols_guard = all_symbols.lock().unwrap();
            for symbol in symbols {
                all_symbols_guard.insert(symbol.id.clone(), symbol);
            }

            if !file_exports.is_empty() {
                let mut exports_guard = exports.lock().unwrap();
                exports_guard.insert(file_path, file_exports);
            }
        }

        // Unwrap the Mutex to get the final HashMaps
        let all_symbols = all_symbols.into_inner().unwrap();
        let exports = exports.into_inner().unwrap();

        // Second pass: Build import relationships IN PARALLEL
        let imports = Mutex::new(HashMap::with_capacity(estimated_symbols));

        let import_results: Vec<Result<_, GraphError>> = files
            .par_iter()
            .map(|(file_path, source)| {
                let parser: Box<dyn LanguageParser> = if file_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == "ts" || s == "tsx")
                    .unwrap_or(false)
                {
                    Box::new(code_viz_core::parser::TypeScriptParser)
                } else {
                    Box::new(code_viz_core::parser::JavaScriptParser)
                };

                // Extract imports
                let builder = SymbolGraphBuilder::new();
                let import_sources = builder.extract_imports(file_path, source, parser.as_ref())?;

                // Collect import relationships for this file
                let mut file_imports: Vec<(SymbolId, Vec<SymbolId>)> = Vec::new();

                // Resolve import paths to actual files
                for import_source in import_sources {
                    if let Some(resolved_path) =
                        resolve_import_path(file_path, &import_source, &available_files)
                    {
                        // Find exported symbols from the imported file
                        if let Some(exported_symbols) = exports.get(&resolved_path) {
                            // Get all symbols in the current file that could depend on these imports
                            let file_symbols: Vec<SymbolId> = all_symbols
                                .values()
                                .filter(|s| s.path == *file_path)
                                .map(|s| s.id.clone())
                                .collect();

                            // For simplicity, mark all symbols in the importing file as depending
                            // on all exported symbols from the imported file
                            for symbol_id in file_symbols {
                                file_imports.push((symbol_id, exported_symbols.clone()));
                            }
                        }
                    }
                }

                Ok(file_imports)
            })
            .collect();

        // Collect import results
        for result in import_results {
            let file_imports = result?;
            let mut imports_guard = imports.lock().unwrap();
            for (symbol_id, deps) in file_imports {
                imports_guard
                    .entry(symbol_id)
                    .or_insert_with(Vec::new)
                    .extend(deps);
            }
        }

        let imports = imports.into_inner().unwrap();

        Ok(SymbolGraph {
            symbols: all_symbols,
            imports,
            exports,
        })
    }
}

impl Default for SymbolGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
