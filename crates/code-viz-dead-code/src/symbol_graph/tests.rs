//! Tests for symbol graph construction.

use super::builder::SymbolGraphBuilder;
use super::extractors::is_test_file;
use super::resolver::resolve_import_path;
use ahash::AHashMap as HashMap;
use crate::models::SymbolKind;
use code_viz_core::parser::TypeScriptParser;
use std::path::{Path, PathBuf};

#[test]
fn test_extract_typescript_functions() {
    let source = r#"
        function regularFunction() {
            return 42;
        }

        const arrowFunc = () => {
            return 'hello';
        };
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    assert_eq!(symbols.len(), 2);
    assert!(symbols
        .iter()
        .any(|s| s.name == "regularFunction" && s.kind == SymbolKind::Function));
    assert!(symbols
        .iter()
        .any(|s| s.name == "arrowFunc" && s.kind == SymbolKind::ArrowFunction));
}

#[test]
fn test_extract_classes_and_methods() {
    let source = r#"
        class MyClass {
            myMethod() {
                return 'test';
            }
        }
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    assert_eq!(symbols.len(), 2);
    assert!(symbols
        .iter()
        .any(|s| s.name == "MyClass" && s.kind == SymbolKind::Class));
    assert!(symbols
        .iter()
        .any(|s| s.name == "myMethod" && s.kind == SymbolKind::Method));
}

#[test]
fn test_exported_symbols() {
    let source = r#"
        export function exportedFunc() {
            return 1;
        }

        function privateFunc() {
            return 2;
        }

        export default class ExportedClass {
        }
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    let exported_func = symbols.iter().find(|s| s.name == "exportedFunc").unwrap();
    assert!(exported_func.is_exported);

    let private_func = symbols.iter().find(|s| s.name == "privateFunc").unwrap();
    assert!(!private_func.is_exported);

    let exported_class = symbols.iter().find(|s| s.name == "ExportedClass").unwrap();
    assert!(exported_class.is_exported);
}

#[test]
fn test_test_file_detection() {
    assert!(is_test_file(Path::new("src/utils.test.ts")));
    assert!(is_test_file(Path::new("src/components.spec.tsx")));
    assert!(is_test_file(Path::new("src/__tests__/utils.ts")));
    assert!(is_test_file(Path::new("tests/integration.ts")));
    assert!(!is_test_file(Path::new("src/utils.ts")));
}

#[test]
fn test_symbol_line_numbers() {
    let source = r#"
function first() {
    return 1;
}

const second = () => {
    return 2;
};
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    let first = symbols.iter().find(|s| s.name == "first").unwrap();
    assert_eq!(first.line_start, 2); // 1-indexed

    let second = symbols.iter().find(|s| s.name == "second").unwrap();
    assert_eq!(second.line_start, 6);
}

#[test]
fn test_skip_anonymous_functions() {
    let source = r#"
        [1, 2, 3].map(() => {
            return 42;
        });
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    // Anonymous functions should be skipped
    assert_eq!(symbols.len(), 0);
}

#[test]
fn test_symbol_id_generation() {
    let source = r#"
        function testFunc() {}
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("/home/user/project/test.ts");
    let mut builder = SymbolGraphBuilder::new();

    let symbols = builder.extract_symbols(path, source, &parser).unwrap();

    assert_eq!(symbols.len(), 1);
    let symbol = &symbols[0];
    assert!(symbol.id.contains("/home/user/project/test.ts"));
    assert!(symbol.id.contains("testFunc"));
    assert!(symbol.id.contains(":2:")); // Line number
}

#[test]
fn test_build_graph_simple() {
    let mut builder = SymbolGraphBuilder::new();

    let files = vec![
        (
            PathBuf::from("src/utils.ts"),
            r#"
            export function helper() {
                return 42;
            }
            "#
            .to_string(),
        ),
        (
            PathBuf::from("src/main.ts"),
            r#"
            import { helper } from "./utils";

            function main() {
                helper();
            }
            "#
            .to_string(),
        ),
    ];

    let graph = builder.build_graph(files).unwrap();

    // Check that symbols were extracted
    assert!(graph.symbols.len() >= 2);

    // Check that exports were tracked
    assert!(graph.exports.contains_key(&PathBuf::from("src/utils.ts")));

    // Check that helper function was found
    let helper_symbol = graph.symbols.values().find(|s| s.name == "helper");
    assert!(helper_symbol.is_some());
}

#[test]
fn test_build_graph_multi_file() {
    let mut builder = SymbolGraphBuilder::new();

    let files = vec![
        (
            PathBuf::from("a.ts"),
            r#"
            export function funcA() {}
            "#
            .to_string(),
        ),
        (
            PathBuf::from("b.ts"),
            r#"
            import { funcA } from "./a";
            export function funcB() {
                funcA();
            }
            "#
            .to_string(),
        ),
        (
            PathBuf::from("c.ts"),
            r#"
            import { funcB } from "./b";
            function funcC() {
                funcB();
            }
            "#
            .to_string(),
        ),
    ];

    let graph = builder.build_graph(files).unwrap();

    // All three functions should be in the graph
    assert!(graph.symbols.values().any(|s| s.name == "funcA"));
    assert!(graph.symbols.values().any(|s| s.name == "funcB"));
    assert!(graph.symbols.values().any(|s| s.name == "funcC"));

    // Check exports
    assert!(graph.exports.contains_key(&PathBuf::from("a.ts")));
    assert!(graph.exports.contains_key(&PathBuf::from("b.ts")));
}

#[test]
fn test_build_graph_circular_imports() {
    let mut builder = SymbolGraphBuilder::new();

    let files = vec![
        (
            PathBuf::from("a.ts"),
            r#"
            import { funcB } from "./b";
            export function funcA() {
                funcB();
            }
            "#
            .to_string(),
        ),
        (
            PathBuf::from("b.ts"),
            r#"
            import { funcA } from "./a";
            export function funcB() {
                funcA();
            }
            "#
            .to_string(),
        ),
    ];

    // Should not panic or infinite loop on circular imports
    let graph = builder.build_graph(files).unwrap();

    assert!(graph.symbols.values().any(|s| s.name == "funcA"));
    assert!(graph.symbols.values().any(|s| s.name == "funcB"));
}

#[test]
fn test_extract_imports() {
    let source = r#"
        import { foo } from "./foo";
        import * as bar from "../bar";
        import type { Baz } from "@/types";
    "#;

    let parser = TypeScriptParser;
    let path = Path::new("test.ts");
    let builder = SymbolGraphBuilder::new();

    let imports = builder.extract_imports(path, source, &parser).unwrap();

    assert_eq!(imports.len(), 3);
    assert!(imports.iter().any(|i| i.contains("./foo")));
    assert!(imports.iter().any(|i| i.contains("../bar")));
    assert!(imports.iter().any(|i| i.contains("@/types")));
}

#[test]
fn test_resolve_relative_imports() {
    let mut available = HashMap::new();
    available.insert(PathBuf::from("src/utils.ts"), true);
    available.insert(PathBuf::from("src/components/Button.tsx"), true);

    let importer = Path::new("src/main.ts");

    // Resolve "./utils" to "src/utils.ts"
    let resolved = resolve_import_path(importer, "\"./utils\"", &available);
    assert_eq!(resolved, Some(PathBuf::from("src/utils.ts")));

    // Resolve "./components/Button" to "src/components/Button.tsx"
    let resolved = resolve_import_path(importer, "\"./components/Button\"", &available);
    assert_eq!(resolved, Some(PathBuf::from("src/components/Button.tsx")));
}
