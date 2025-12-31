# Error Handling Migration Guide

This guide documents the error handling patterns for the code-viz project and provides practical examples for converting `unwrap()` calls to proper error handling.

## Table of Contents

1. [Overview](#overview)
2. [Error Type Hierarchy](#error-type-hierarchy)
3. [Common Migration Patterns](#common-migration-patterns)
4. [Language-Specific Examples](#language-specific-examples)
5. [Testing Error Paths](#testing-error-paths)
6. [Anti-Patterns to Avoid](#anti-patterns-to-avoid)

## Overview

The code-viz project uses a unified error handling approach based on the `CodeVizError` type defined in `crates/code-viz-core/src/error.rs`. This eliminates fragile `unwrap()` calls and provides:

- **Rich context**: Errors include file paths, line numbers, and operation details
- **Proper propagation**: Uses the `?` operator for clean error handling
- **Type safety**: All errors implement `std::error::Error`
- **Ergonomic helpers**: Extension traits and macros reduce boilerplate

## Error Type Hierarchy

The main error type is `CodeVizError` with the following variants:

### ParseError
Used when parsing source code fails (tree-sitter, malformed syntax).

**Fields:**
- `path`: File path that failed to parse
- `language`: Programming language
- `line`: Optional line number
- `message`: Error description

### FileSystem
Covers all file I/O errors (read, write, permissions, not found).

**Fields:**
- `path`: Path where the error occurred
- `message`: Human-readable description
- `source`: Original `std::io::Error`

### Git
Git operations failures (repository access, commit history).

**Fields:**
- `repository`: Optional repository path
- `message`: Error description

### Analysis
Analysis operation failures (metrics, coupling, dead code detection).

**Fields:**
- `operation`: Name of the analysis (e.g., "coupling", "metrics")
- `message`: Detailed error message
- `path`: Optional file path being analyzed

### CoverageDataMissing
Missing or malformed coverage data (often non-critical).

**Fields:**
- `message`: Description of the coverage issue
- `path`: Optional coverage file path

### Cache
Cache operations failures (serialization, storage).

**Fields:**
- `message`: Error description
- `source`: Optional underlying error

### Config
Invalid or missing configuration values.

**Fields:**
- `message`: What configuration is invalid

## Common Migration Patterns

### Pattern 1: Parser `unwrap()` → Result with ParseError

**Before:**
```rust
let tree = parser.parse(source_code, None).unwrap();
let root_node = tree.root_node();
```

**After:**
```rust
use code_viz_core::error::{CodeVizError, Result};

let tree = parser.parse(source_code, None)
    .ok_or_else(|| CodeVizError::parse(
        path.clone(),
        "rust",
        None,
        "tree-sitter parse failed"
    ))?;
let root_node = tree.root_node();
```

**With macro:**
```rust
use code_viz_core::parse_error;

let tree = parser.parse(source_code, None)
    .ok_or_else(|| parse_error!(
        path,
        "rust",
        None,
        "tree-sitter parse failed"
    ))?;
```

### Pattern 2: File I/O `unwrap()` → Result with Context

**Before:**
```rust
let content = std::fs::read_to_string(path).unwrap();
```

**After:**
```rust
use code_viz_core::error::CodeVizError;

let content = std::fs::read_to_string(path)
    .map_err(|e| CodeVizError::file_read(path, e))?;
```

**With macro:**
```rust
use code_viz_core::io_error;

let content = std::fs::read_to_string(path)
    .map_err(|e| io_error!(path, e))?;
```

**With ResultExt:**
```rust
use code_viz_core::error_ext::ResultExt;

let content = std::fs::read_to_string(path)
    .with_context(|| format!("reading file: {}", path.display()))?;
```

### Pattern 3: Coverage `unwrap()` → `unwrap_or` with Default

Coverage data is often optional and non-critical. Instead of failing, provide sensible defaults.

**Before:**
```rust
let hit_count = coverage_map.get(&line_number).unwrap();
```

**After:**
```rust
let hit_count = coverage_map.get(&line_number).unwrap_or(&0);
```

**Or with map:**
```rust
let hit_count = coverage_map
    .get(&line_number)
    .map(|c| c.hit_count)
    .unwrap_or(0);
```

### Pattern 4: Collection `unwrap()` → `ok_or` with Error

**Before:**
```rust
let first_import = imports.first().unwrap();
```

**After:**
```rust
use code_viz_core::analysis_error;

let first_import = imports.first()
    .ok_or_else(|| analysis_error!(
        "coupling",
        "no imports found in file"
    ))?;
```

### Pattern 5: Reducing `map_err` Duplication

**Before (duplicated error mapping):**
```rust
let content = std::fs::read_to_string(&path)
    .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

let parsed = parse_content(&content)
    .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;
```

**After (using ResultExt):**
```rust
use code_viz_core::error_ext::ResultExt;

let content = std::fs::read_to_string(&path)
    .with_context(|| format!("reading {}", path.display()))?;

let parsed = parse_content(&content)
    .with_context(|| format!("parsing {}", path.display()))?;
```

## Language-Specific Examples

### Parser Errors (tree-sitter integration)

**Scenario:** Parsing fails for a source file

```rust
use code_viz_core::error::{CodeVizError, Result};
use std::path::Path;

fn parse_rust_file(path: &Path, source: &str) -> Result<TreeNode> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_rust::language())
        .map_err(|e| CodeVizError::parse(
            path.to_path_buf(),
            "rust",
            None,
            format!("failed to set language: {}", e)
        ))?;

    let tree = parser.parse(source, None)
        .ok_or_else(|| CodeVizError::parse(
            path.to_path_buf(),
            "rust",
            None,
            "tree-sitter returned None"
        ))?;

    Ok(tree.root_node())
}
```

### Coverage Errors (graceful degradation)

**Scenario:** Coverage data might be missing for a file

```rust
use code_viz_core::error::Result;

fn get_coverage_percentage(
    file_path: &Path,
    coverage_data: &Option<CoverageMap>
) -> f64 {
    // Don't fail if coverage is missing - return 0.0 instead
    coverage_data
        .as_ref()
        .and_then(|data| data.get(file_path))
        .map(|cov| cov.percentage)
        .unwrap_or(0.0)
}
```

**Or with logging:**
```rust
use log::warn;

fn get_coverage_percentage(
    file_path: &Path,
    coverage_data: &Option<CoverageMap>
) -> f64 {
    match coverage_data.as_ref().and_then(|data| data.get(file_path)) {
        Some(cov) => cov.percentage,
        None => {
            warn!("No coverage data for {}", file_path.display());
            0.0
        }
    }
}
```

### File I/O Errors

**Scenario:** Reading a configuration file

```rust
use code_viz_core::error::{CodeVizError, Result};
use code_viz_core::error_ext::ResultExt;

fn load_config(config_path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(config_path)
        .with_context(|| format!("loading config from {}", config_path.display()))?;

    let config: Config = toml::from_str(&content)
        .map_err(|e| CodeVizError::config(
            format!("invalid TOML in {}: {}", config_path.display(), e)
        ))?;

    Ok(config)
}
```

### Git Errors

**Scenario:** Accessing git repository

```rust
use code_viz_core::error::{CodeVizError, Result};

fn get_commit_history(repo_path: &Path) -> Result<Vec<Commit>> {
    let repo = git2::Repository::open(repo_path)
        .map_err(|e| CodeVizError::git(
            Some(repo_path.to_path_buf()),
            format!("failed to open repository: {}", e)
        ))?;

    // ... process commits
    Ok(vec![])
}
```

### Analysis Errors

**Scenario:** Building dependency graph fails

```rust
use code_viz_core::error::{CodeVizError, Result};
use code_viz_core::analysis_error;

fn build_dependency_graph(files: &[PathBuf]) -> Result<DependencyGraph> {
    if files.is_empty() {
        return Err(analysis_error!(
            "coupling",
            "no files provided for dependency analysis"
        ));
    }

    let mut graph = DependencyGraph::new();

    for file in files {
        parse_and_add_to_graph(file, &mut graph)
            .map_err(|e| CodeVizError::analysis_with_path(
                "coupling",
                file,
                format!("failed to process file: {}", e)
            ))?;
    }

    Ok(graph)
}
```

## Testing Error Paths

Always test error scenarios, not just happy paths.

### Example: Test Parser Error

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_parse_malformed_rust() {
        let malformed = "fn incomplete(";
        let result = parse_rust_code(Path::new("test.rs"), malformed);

        assert!(result.is_err());
        match result.unwrap_err() {
            CodeVizError::ParseError { language, .. } => {
                assert_eq!(language, "rust");
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_file_not_found_returns_error() {
        let result = load_config(Path::new("nonexistent.toml"));

        assert!(result.is_err());
        match result.unwrap_err() {
            CodeVizError::FileSystem { .. } => (),
            _ => panic!("Expected FileSystem error"),
        }
    }

    #[test]
    fn test_missing_coverage_uses_default() {
        let percentage = get_coverage_percentage(
            Path::new("test.rs"),
            &None
        );

        assert_eq!(percentage, 0.0);
    }
}
```

### Example: Test Error Messages

```rust
#[test]
fn test_error_messages_include_context() {
    let error = CodeVizError::parse(
        PathBuf::from("src/main.rs"),
        "rust",
        Some(42),
        "unexpected EOF"
    );

    let message = error.to_string();
    assert!(message.contains("src/main.rs"));
    assert!(message.contains("rust"));
    assert!(message.contains("42"));
    assert!(message.contains("unexpected EOF"));
}
```

## Anti-Patterns to Avoid

### ❌ Anti-Pattern 1: "I know it won't fail"

**Don't:**
```rust
// "This unwrap is fine because we just checked the length"
if !vec.is_empty() {
    let first = vec.first().unwrap(); // Still wrong!
}
```

**Do:**
```rust
if let Some(first) = vec.first() {
    // Handle the value
}
```

### ❌ Anti-Pattern 2: Silently Ignoring Errors

**Don't:**
```rust
let _ = std::fs::remove_file(path); // Error silently ignored
```

**Do:**
```rust
if let Err(e) = std::fs::remove_file(path) {
    warn!("Failed to remove temporary file {}: {}", path.display(), e);
}
```

**Or:**
```rust
std::fs::remove_file(path)
    .with_context(|| format!("removing temporary file {}", path.display()))?;
```

### ❌ Anti-Pattern 3: Generic Error Messages

**Don't:**
```rust
let content = std::fs::read_to_string(path)
    .map_err(|e| "Failed to read file")?; // Lost all context!
```

**Do:**
```rust
let content = std::fs::read_to_string(path)
    .map_err(|e| CodeVizError::file_read(path, e))?;
```

### ❌ Anti-Pattern 4: Using `expect()` Without Good Reason

**Don't:**
```rust
let value = some_option.expect("this should never happen"); // Famous last words
```

**Do:**
```rust
let value = some_option.ok_or_else(|| {
    analysis_error!("operation", "expected value was None")
})?;
```

### ❌ Anti-Pattern 5: Panicking in Library Code

**Don't:**
```rust
fn analyze_file(path: &Path) -> Analysis {
    if !path.exists() {
        panic!("File doesn't exist!"); // Never panic in library code
    }
    // ...
}
```

**Do:**
```rust
fn analyze_file(path: &Path) -> Result<Analysis> {
    if !path.exists() {
        return Err(CodeVizError::file_read(
            path,
            io::Error::new(io::ErrorKind::NotFound, "file not found")
        ));
    }
    // ...
}
```

## Quick Reference

### When to Use Each Error Type

| Scenario | Error Type | Constructor Method |
|----------|------------|-------------------|
| Tree-sitter parsing fails | `ParseError` | `CodeVizError::parse()` |
| File read/write fails | `FileSystem` | `CodeVizError::file_read()` / `file_write()` |
| Git operation fails | `Git` | `CodeVizError::git()` |
| Analysis logic fails | `Analysis` | `CodeVizError::analysis()` |
| Coverage data missing | `CoverageDataMissing` | `CodeVizError::coverage_missing()` |
| Cache operation fails | `Cache` | `CodeVizError::cache()` |
| Invalid configuration | `Config` | `CodeVizError::config()` |

### Helper Macros

```rust
// I/O error with file path
io_error!(path, error)

// Parse error with full context
parse_error!(path, language, line, message)

// Analysis error with operation
analysis_error!(operation, message)
```

### Extension Trait

```rust
use code_viz_core::error_ext::ResultExt;

result.with_context(|| "additional context")?
```

## See Also

- [Error Handling Design Document](.spec-workflow/specs/error-handling-code-quality-remediation/design.md)
- [Error Handling Tasks](.spec-workflow/specs/error-handling-code-quality-remediation/tasks.md)
- [Rust Error Handling Best Practices](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
