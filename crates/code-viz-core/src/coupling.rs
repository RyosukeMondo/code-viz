use crate::error::{CodeVizError, Result};
use crate::language::LanguageRegistry;
use crate::models::{CouplingMetrics, FileMetrics};
use crate::parser::{get_parser, LanguageParser};
use crate::traits::FileSystem;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn extract_dependencies(
    source: &str,
    parser: &dyn LanguageParser,
    query_str: &str,
) -> Result<Vec<String>> {
    let tree = parser.parse(source).map_err(|e| {
        CodeVizError::parse(
            PathBuf::from("<inline>"),
            parser.language_key(),
            None,
            e.to_string(),
        )
    })?;

    let mut cursor = tree_sitter::QueryCursor::new();
    let query = tree_sitter::Query::new(parser.get_language(), query_str).map_err(|e| {
        CodeVizError::parse(
            PathBuf::from("<query>"),
            parser.language_key(),
            None,
            format!("Invalid tree-sitter query: {}", e),
        )
    })?;

    let captures = cursor.matches(&query, tree.root_node(), source.as_bytes());

    let mut dependencies = Vec::new();
    for match_ in captures {
        for capture in match_.captures {
            let node = capture.node;
            let dep = &source[node.byte_range()];
            dependencies.push(dep.to_string());
        }
    }
    Ok(dependencies)
}

pub fn calculate_coupling(files: &mut [FileMetrics], fs: &impl FileSystem, base_path: &Path) {
    let mut dependency_graph: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let file_paths: Vec<PathBuf> = files.iter().map(|f| f.path.clone()).collect();
    let registry = LanguageRegistry::new();

    // First pass: build the dependency graph
    for file in files.iter() {
        let source = match fs.read_to_string(&file.path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let parser = match get_parser(&file.language) {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Get language provider from registry
        let provider = match registry.get_by_name(&file.language) {
            Some(p) => p,
            None => continue, // Skip unsupported languages
        };

        let query_str = provider.coupling_query();

        let dependencies = match extract_dependencies(&source, parser.as_ref(), query_str) {
            Ok(deps) => deps,
            Err(_) => {
                // Skip files that fail to parse - log warning in production but continue analysis
                continue;
            }
        };
        let resolved_deps = resolve_dependencies(
            base_path,
            &file.path,
            &dependencies,
            &file_paths,
            &file.language,
        );
        dependency_graph.insert(file.path.clone(), resolved_deps);
    }

    // Second pass: calculate metrics
    for file in files.iter_mut() {
        let efferent_coupling = dependency_graph
            .get(&file.path)
            .map_or(0, |deps| deps.len());

        let afferent_coupling = dependency_graph
            .values()
            .filter(|deps| deps.contains(&file.path))
            .count();

        let instability = if afferent_coupling + efferent_coupling == 0 {
            0.0
        } else {
            efferent_coupling as f64 / (afferent_coupling + efferent_coupling) as f64
        };

        file.coupling = Some(CouplingMetrics {
            afferent_coupling,
            efferent_coupling,
            instability,
        });
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::MockFileSystem;
    use crate::models::FileMetrics;
    use std::time::SystemTime;

    fn create_mock_file_metrics(path: &str, language: &str) -> FileMetrics {
        FileMetrics {
            path: PathBuf::from(path),
            language: language.to_string(),
            loc: 0,
            size_bytes: 0,
            function_count: 0,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: None,
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        }
    }

    #[test]
    fn test_typescript_coupling() {
        let fs = MockFileSystem::new()
            .with_file("a.ts", r#"import { B } from "./b";"#)
            .with_file("b.ts", r#"import { C } from "./c";"#)
            .with_file("c.ts", "");

        let mut files = vec![
            create_mock_file_metrics("a.ts", "typescript"),
            create_mock_file_metrics("b.ts", "typescript"),
            create_mock_file_metrics("c.ts", "typescript"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let a = files
            .iter()
            .find(|f| f.path == Path::new("a.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(a.efferent_coupling, 1);
        assert_eq!(a.afferent_coupling, 0);

        let b = files
            .iter()
            .find(|f| f.path == Path::new("b.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(b.efferent_coupling, 1);
        assert_eq!(b.afferent_coupling, 1);

        let c = files
            .iter()
            .find(|f| f.path == Path::new("c.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(c.efferent_coupling, 0);
        assert_eq!(c.afferent_coupling, 1);
    }

    #[test]
    fn test_rust_coupling() {
        let fs = MockFileSystem::new()
            .with_file("lib.rs", "mod a;")
            .with_file("a.rs", "use crate::b;")
            .with_file("b.rs", "");

        let mut files = vec![
            create_mock_file_metrics("lib.rs", "rust"),
            create_mock_file_metrics("a.rs", "rust"),
            create_mock_file_metrics("b.rs", "rust"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let main = files
            .iter()
            .find(|f| f.path == Path::new("lib.rs"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(main.efferent_coupling, 1);
        assert_eq!(main.afferent_coupling, 0);

        let a = files
            .iter()
            .find(|f| f.path == Path::new("a.rs"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(a.efferent_coupling, 1);
        assert_eq!(a.afferent_coupling, 1);

        let b = files
            .iter()
            .find(|f| f.path == Path::new("b.rs"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(b.efferent_coupling, 0);
        assert_eq!(b.afferent_coupling, 1);
    }

    #[test]
    fn test_python_coupling() {
        let fs = MockFileSystem::new()
            .with_file("main.py", "import a")
            .with_file("a.py", "from b import c")
            .with_file("b.py", "");

        let mut files = vec![
            create_mock_file_metrics("main.py", "python"),
            create_mock_file_metrics("a.py", "python"),
            create_mock_file_metrics("b.py", "python"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let main = files
            .iter()
            .find(|f| f.path == Path::new("main.py"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(main.efferent_coupling, 1);
        assert_eq!(main.afferent_coupling, 0);

        let a = files
            .iter()
            .find(|f| f.path == Path::new("a.py"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(a.efferent_coupling, 1);
        assert_eq!(a.afferent_coupling, 1);

        let b = files
            .iter()
            .find(|f| f.path == Path::new("b.py"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(b.efferent_coupling, 0);
        assert_eq!(b.afferent_coupling, 1);
    }

    #[test]
    fn test_python_relative_imports() {
        let fs = MockFileSystem::new()
            .with_file("pkg/a.py", "from .b import B")
            .with_file("pkg/b.py", "class B: pass");

        let mut files = vec![
            create_mock_file_metrics("pkg/a.py", "python"),
            create_mock_file_metrics("pkg/b.py", "python"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let a = files
            .iter()
            .find(|f| f.path == Path::new("pkg/a.py"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(a.efferent_coupling, 1);
        assert_eq!(a.afferent_coupling, 0);

        let b = files
            .iter()
            .find(|f| f.path == Path::new("pkg/b.py"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(b.efferent_coupling, 0);
        assert_eq!(b.afferent_coupling, 1);
    }

    #[test]
    fn test_instability_calculation() {
        let fs = MockFileSystem::new()
            .with_file("stable.ts", "") // No dependencies, only depended upon
            .with_file("unstable.ts", r#"import { Stable } from "./stable";"#); // Depends on others, nothing depends on it

        let mut files = vec![
            create_mock_file_metrics("stable.ts", "typescript"),
            create_mock_file_metrics("unstable.ts", "typescript"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let stable = files
            .iter()
            .find(|f| f.path == Path::new("stable.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(stable.instability, 0.0); // Maximally stable

        let unstable = files
            .iter()
            .find(|f| f.path == Path::new("unstable.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(unstable.instability, 1.0); // Maximally unstable
    }

    #[test]
    fn test_parse_error_handling() {
        let fs = MockFileSystem::new()
            .with_file("valid.ts", r#"import { B } from "./b";"#)
            .with_file("invalid.ts", "import {{{"); // Invalid syntax

        let mut files = vec![
            create_mock_file_metrics("valid.ts", "typescript"),
            create_mock_file_metrics("invalid.ts", "typescript"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        // Should handle parse errors gracefully and continue
        let valid = files
            .iter()
            .find(|f| f.path == Path::new("valid.ts"))
            .unwrap(); // Test-only unwrap: test data is guaranteed to have this file
        assert!(valid.coupling.is_some());
    }

    #[test]
    fn test_unsupported_language() {
        let fs = MockFileSystem::new().with_file("file.unknown", "some code");

        let mut files = vec![create_mock_file_metrics("file.unknown", "unknown")];

        calculate_coupling(&mut files, &fs, Path::new(""));

        // Should skip unsupported languages gracefully
        let file = &files[0];
        assert!(file.coupling.is_some()); // Gets default coupling of 0/0/0
    }

    #[test]
    fn test_circular_dependencies() {
        let fs = MockFileSystem::new()
            .with_file("a.ts", r#"import { B } from "./b";"#)
            .with_file("b.ts", r#"import { A } from "./a";"#);

        let mut files = vec![
            create_mock_file_metrics("a.ts", "typescript"),
            create_mock_file_metrics("b.ts", "typescript"),
        ];

        calculate_coupling(&mut files, &fs, Path::new(""));

        let a = files
            .iter()
            .find(|f| f.path == Path::new("a.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(a.efferent_coupling, 1);
        assert_eq!(a.afferent_coupling, 1);
        assert_eq!(a.instability, 0.5); // Balanced

        let b = files
            .iter()
            .find(|f| f.path == Path::new("b.ts"))
            .unwrap() // Test-only unwrap: test data is guaranteed to have this file
            .coupling
            .as_ref()
            .unwrap(); // Test-only unwrap: coupling is calculated for all test files
        assert_eq!(b.efferent_coupling, 1);
        assert_eq!(b.afferent_coupling, 1);
        assert_eq!(b.instability, 0.5); // Balanced
    }
}

/// A simple path normalization function to handle `.` and `..`.
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ std::path::Component::RootDir) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            std::path::Component::Normal(c) => {
                ret.push(c);
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                ret.pop();
            }
            std::path::Component::RootDir => {
                unreachable!();
            }
            std::path::Component::Prefix(..) => {
                unreachable!();
            }
        }
    }
    ret
}

fn resolve_dependencies(
    base_path: &Path,
    current_file_path: &Path,
    dependencies: &[String],
    all_files: &[PathBuf],
    language: &str,
) -> Vec<PathBuf> {
    let mut resolved_deps = Vec::new();
    let current_dir = current_file_path.parent().unwrap_or_else(|| Path::new(""));

    for dep in dependencies {
        let cleaned_dep = dep.trim_matches(|c| c == '"' || c == '\'');
        let mut found = false;

        if language == "python" {
            if let Some(relative_path) = cleaned_dep.strip_prefix('.') {
                let path_from_dir = relative_path.replace('.', "/");
                let mut potential_path = current_dir.join(path_from_dir);
                if potential_path.extension().is_none() {
                    potential_path.set_extension("py");
                }
                if all_files.contains(&potential_path) {
                    resolved_deps.push(potential_path);
                }
                continue;
            }
        }

        // Handle crate-relative paths for Rust
        if let Some(crate_relative) = cleaned_dep.strip_prefix("crate::") {
            let path_from_crate = crate_relative.replace("::", "/");
            let mut potential_path = base_path.join(path_from_crate);
            if potential_path.extension().is_none() {
                potential_path.set_extension("rs");
            }
            if all_files.contains(&potential_path) {
                resolved_deps.push(potential_path);
            }
            continue;
        }

        let dep_path = Path::new(cleaned_dep);
        let resolved_path = current_dir.join(dep_path);

        let mut potential_paths = Vec::new();
        if resolved_path.extension().is_none() {
            potential_paths.push(resolved_path.with_extension("ts"));
            potential_paths.push(resolved_path.with_extension("js"));
            potential_paths.push(resolved_path.join("mod.rs"));
            potential_paths.push(resolved_path.with_extension("rs"));
            potential_paths.push(resolved_path.with_extension("py"));
        } else {
            potential_paths.push(resolved_path);
        }

        for p in potential_paths {
            let normalized_p = normalize_path(&p);
            if all_files.contains(&normalized_p) {
                resolved_deps.push(normalized_p);
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
    }
    resolved_deps
}
