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
use tree_sitter::{Query, QueryCursor, Tree};

// Type aliases to reduce complexity
type SymbolMap = HashMap<SymbolId, Symbol>;
type ExportMap = HashMap<PathBuf, Vec<SymbolId>>;
type ImportMap = HashMap<SymbolId, Vec<SymbolId>>;
type SymbolExtractionResult = (PathBuf, Vec<Symbol>, Vec<SymbolId>);
type ImportResolutionResult = Vec<(SymbolId, Vec<SymbolId>)>;

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

    fn parse_symbol_kind(capture_name: &str) -> Option<SymbolKind> {
        match capture_name {
            "function" => Some(SymbolKind::Function),
            "arrow" => Some(SymbolKind::ArrowFunction),
            "class" => Some(SymbolKind::Class),
            "method" => Some(SymbolKind::Method),
            "variable" => Some(SymbolKind::Variable),
            _ => None,
        }
    }

    fn build_symbol_from_capture(
        &self,
        node: tree_sitter::Node,
        capture_name: &str,
        source: &str,
        path: &Path,
        is_test: bool,
    ) -> Option<Symbol> {
        let kind = Self::parse_symbol_kind(capture_name)?;
        let name = extract_symbol_name(&node, source, capture_name);
        if name.is_empty() {
            return None;
        }

        let is_exported = is_symbol_exported(&node, source);
        let start_point = node.start_position();
        let end_point = node.end_position();
        let line_start = start_point.row + 1;
        let line_end = end_point.row + 1;
        let id = format!("{}:{}:{}", path.display(), line_start, name);

        Some(Symbol {
            id,
            name,
            kind,
            path: path.to_path_buf(),
            line_start,
            line_end,
            is_exported,
            is_test,
        })
    }

    fn process_query_matches(
        &self,
        query: &Query,
        tree: &Tree,
        source: &str,
        path: &Path,
        is_test: bool,
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(query, tree.root_node(), source.as_bytes());

        for m in matches {
            for capture in m.captures {
                let capture_name = &query.capture_names()[capture.index as usize];
                if let Some(symbol) = self.build_symbol_from_capture(
                    capture.node,
                    capture_name,
                    source,
                    path,
                    is_test,
                ) {
                    symbols.push(symbol);
                }
            }
        }

        symbols
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
        let tree = parser.parse(source).map_err(|e| GraphError::ParseError {
            file: path.to_path_buf(),
            message: e.to_string(),
        })?;

        let is_test = is_test_file(path);
        let query = get_symbol_query(parser.language_key())?;
        let symbols = self.process_query_matches(query, &tree, source, path, is_test);

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
        let query = get_import_query(parser.language_key())?;
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
        let file_count = files.len();
        let estimated_symbols = file_count * 20;

        let available_files: HashMap<PathBuf, bool> =
            files.iter().map(|(path, _)| (path.clone(), true)).collect();

        let (all_symbols, exports) = Self::extract_all_symbols(&files, estimated_symbols, file_count)?;
        let imports = Self::build_dependency_edges(&files, &all_symbols, &exports, &available_files, estimated_symbols)?;

        Ok(SymbolGraph {
            symbols: all_symbols,
            imports,
            exports,
        })
    }

    /// Extract all symbols from files in parallel
    fn extract_all_symbols(
        files: &[(PathBuf, String)],
        estimated_symbols: usize,
        file_count: usize,
    ) -> Result<(SymbolMap, ExportMap), GraphError> {
        let all_symbols = Mutex::new(HashMap::with_capacity(estimated_symbols));
        let exports = Mutex::new(HashMap::with_capacity(file_count));

        let symbol_results: Vec<Result<SymbolExtractionResult, GraphError>> = files
            .par_iter()
            .map(|(file_path, source)| {
                let parser = Self::create_parser(file_path);
                let mut builder = SymbolGraphBuilder::new();
                let symbols = builder.extract_symbols(file_path, source, parser.as_ref())?;

                let file_exports = Self::collect_exports(&symbols);
                Ok((file_path.clone(), symbols, file_exports))
            })
            .collect();

        Self::aggregate_symbols(symbol_results, all_symbols, exports)
    }

    /// Build dependency edges between symbols
    fn build_dependency_edges(
        files: &[(PathBuf, String)],
        all_symbols: &SymbolMap,
        exports: &ExportMap,
        available_files: &HashMap<PathBuf, bool>,
        estimated_symbols: usize,
    ) -> Result<ImportMap, GraphError> {
        let imports = Mutex::new(HashMap::with_capacity(estimated_symbols));

        let import_results: Vec<Result<_, GraphError>> = files
            .par_iter()
            .map(|(file_path, source)| {
                let parser = Self::create_parser(file_path);
                let builder = SymbolGraphBuilder::new();
                let import_sources = builder.extract_imports(file_path, source, parser.as_ref())?;

                let file_imports = Self::resolve_file_imports(
                    file_path,
                    &import_sources,
                    all_symbols,
                    exports,
                    available_files,
                );

                Ok(file_imports)
            })
            .collect();

        Self::aggregate_imports(import_results, imports)
    }

    /// Create appropriate parser based on file extension
    fn create_parser(file_path: &Path) -> Box<dyn LanguageParser> {
        if file_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s == "ts" || s == "tsx")
            .unwrap_or(false)
        {
            Box::new(code_viz_core::parser::TypeScriptParser)
        } else {
            Box::new(code_viz_core::parser::JavaScriptParser)
        }
    }

    /// Collect exported symbols from a symbol list
    fn collect_exports(symbols: &[Symbol]) -> Vec<SymbolId> {
        symbols
            .iter()
            .filter(|s| s.is_exported)
            .map(|s| s.id.clone())
            .collect()
    }

    /// Aggregate symbol extraction results
    fn aggregate_symbols(
        results: Vec<Result<SymbolExtractionResult, GraphError>>,
        all_symbols: Mutex<SymbolMap>,
        exports: Mutex<ExportMap>,
    ) -> Result<(SymbolMap, ExportMap), GraphError> {
        for result in results {
            let (file_path, symbols, file_exports) = result?;

            let mut all_symbols_guard = all_symbols.lock()
                .expect("BUG: Mutex poisoned - indicates panic in parallel processing");
            for symbol in symbols {
                all_symbols_guard.insert(symbol.id.clone(), symbol);
            }

            if !file_exports.is_empty() {
                let mut exports_guard = exports.lock()
                    .expect("BUG: Mutex poisoned - indicates panic in parallel processing");
                exports_guard.insert(file_path, file_exports);
            }
        }

        Ok((
            all_symbols.into_inner().expect("BUG: Mutex poisoned"),
            exports.into_inner().expect("BUG: Mutex poisoned")
        ))
    }

    /// Resolve imports for a single file
    fn resolve_file_imports(
        file_path: &Path,
        import_sources: &[String],
        all_symbols: &SymbolMap,
        exports: &ExportMap,
        available_files: &HashMap<PathBuf, bool>,
    ) -> Vec<(SymbolId, Vec<SymbolId>)> {
        let mut file_imports = Vec::new();

        for import_source in import_sources {
            if let Some(resolved_path) =
                resolve_import_path(file_path, import_source, available_files)
            {
                if let Some(exported_symbols) = exports.get(&resolved_path) {
                    let file_symbols: Vec<SymbolId> = all_symbols
                        .values()
                        .filter(|s| s.path == *file_path)
                        .map(|s| s.id.clone())
                        .collect();

                    for symbol_id in file_symbols {
                        file_imports.push((symbol_id, exported_symbols.clone()));
                    }
                }
            }
        }

        file_imports
    }

    /// Aggregate import resolution results
    fn aggregate_imports(
        results: Vec<Result<ImportResolutionResult, GraphError>>,
        imports: Mutex<ImportMap>,
    ) -> Result<ImportMap, GraphError> {
        for result in results {
            let file_imports = result?;
            let mut imports_guard = imports.lock()
                .expect("BUG: Mutex poisoned - indicates panic in parallel processing");
            for (symbol_id, deps) in file_imports {
                imports_guard
                    .entry(symbol_id)
                    .or_default()
                    .extend(deps);
            }
        }

        Ok(imports.into_inner().expect("BUG: Mutex poisoned"))
    }
}

impl Default for SymbolGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
