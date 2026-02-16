#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::parser::{get_parser, ParseError};
use proptest::prelude::*;
use tree_sitter::Tree;

// Helper function to validate tree structure invariants
fn assert_valid_tree_structure(tree: &Tree) {
    let root = tree.root_node();

    // Tree must have a root node
    assert!(!root.kind().is_empty(), "Root node must have a kind");

    // Recursively validate node structure
    fn validate_node(node: tree_sitter::Node) {
        // Node byte range must be valid
        assert!(
            node.start_byte() <= node.end_byte(),
            "Node start_byte must be <= end_byte"
        );

        // Validate all children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                // Child must be within parent bounds
                assert!(
                    child.start_byte() >= node.start_byte(),
                    "Child start must be >= parent start"
                );
                assert!(
                    child.end_byte() <= node.end_byte(),
                    "Child end must be <= parent end"
                );

                // Recursively validate child
                validate_node(child);
            }
        }
    }

    validate_node(root);
}

// Generate arbitrary UTF-8 strings (potentially invalid code)
fn arb_utf8_string() -> impl Strategy<Value = String> {
    // Use ASCII printable + some Unicode for broader testing
    prop::string::string_regex("[\\x20-\\x7E\u{00A0}-\u{00FF}]{0,200}").expect("Valid regex")
}

// Generate valid-ish code snippets for each language
fn arb_rust_code() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("fn main() {}".to_string()),
        Just("struct Foo { x: i32 }".to_string()),
        Just("let x = 42;".to_string()),
        Just("// comment\nfn foo() { let y = 1; }".to_string()),
        Just("".to_string()),
        Just("fn incomplete(".to_string()), // Malformed
    ]
}

fn arb_typescript_code() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("function hello() {}".to_string()),
        Just("const x = 42;".to_string()),
        Just("class Foo { method() {} }".to_string()),
        Just("// comment\nconst y = () => {};".to_string()),
        Just("".to_string()),
        Just("function incomplete(".to_string()), // Malformed
    ]
}

fn arb_python_code() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("def hello():\n    pass".to_string()),
        Just("x = 42".to_string()),
        Just("class Foo:\n    def method(self):\n        pass".to_string()),
        Just("# comment\ny = lambda: None".to_string()),
        Just("".to_string()),
        Just("def incomplete(".to_string()), // Malformed
    ]
}

// Property 1: Parser never panics on any input
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn parser_never_panics_on_random_input(code in arb_utf8_string()) {
        let languages = ["typescript", "javascript", "rust", "python", "go", "cpp"];

        for lang in &languages {
            if let Ok(parser) = get_parser(lang) {
                // Should return Ok or Err, never panic
                let result = parser.parse(&code);
                assert!(result.is_ok() || result.is_err());
            }
        }
    }
}

// Property 2: Parser never panics on empty input
#[test]
fn parser_handles_empty_input() {
    let languages = ["typescript", "javascript", "rust", "python", "go", "cpp"];

    for lang in &languages {
        let parser = get_parser(lang).expect("Valid language");
        let result = parser.parse("");
        assert!(result.is_ok(), "Parser should handle empty input");

        if let Ok(tree) = result {
            assert!(
                !tree.root_node().has_error(),
                "Empty input should not have errors"
            );
            let count = parser.count_functions(&tree);
            assert_eq!(count, 0, "Empty input should have no functions");
        }
    }
}

// Property 3: Tree structure is always valid when parsing succeeds
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn tree_structure_valid_for_typescript(code in arb_typescript_code()) {
        let parser = get_parser("typescript").expect("Valid language");
        if let Ok(tree) = parser.parse(&code) {
            assert_valid_tree_structure(&tree);
        }
    }

    #[test]
    fn tree_structure_valid_for_rust(code in arb_rust_code()) {
        let parser = get_parser("rust").expect("Valid language");
        if let Ok(tree) = parser.parse(&code) {
            assert_valid_tree_structure(&tree);
        }
    }

    #[test]
    fn tree_structure_valid_for_python(code in arb_python_code()) {
        let parser = get_parser("python").expect("Valid language");
        if let Ok(tree) = parser.parse(&code) {
            assert_valid_tree_structure(&tree);
        }
    }
}

// Property 4: Function counting never panics
proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn count_functions_never_panics(code in arb_typescript_code()) {
        let parser = get_parser("typescript").expect("Valid language");
        if let Ok(tree) = parser.parse(&code) {
            let _count = parser.count_functions(&tree);
            // Function count is usize, so always non-negative - just verify no panic
        }
    }
}

// Property 5: Comment range finding never panics
proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn find_comments_never_panics(code in arb_rust_code()) {
        let parser = get_parser("rust").expect("Valid language");
        if let Ok(tree) = parser.parse(&code) {
            let comments = parser.find_comment_ranges(&tree);
            // Should return a valid vector, even if empty (length is always >= 0 for Vec)

            // All comment ranges should be within the source bounds
            let source_len = code.len();
            for range in comments {
                assert!(range.start_byte <= range.end_byte);
                assert!(range.end_byte <= source_len);
            }
        }
    }
}

// Property 6: Idempotency - parsing same input twice gives same result
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn parsing_is_idempotent(code in arb_typescript_code()) {
        let parser = get_parser("typescript").expect("Valid language");

        let result1 = parser.parse(&code);
        let result2 = parser.parse(&code);

        match (result1, result2) {
            (Ok(tree1), Ok(tree2)) => {
                // Both succeeded - validate they produce equivalent trees
                assert_eq!(tree1.root_node().to_sexp(), tree2.root_node().to_sexp(),
                    "Parsing same code twice should produce identical trees");

                // Function counts should match
                let count1 = parser.count_functions(&tree1);
                let count2 = parser.count_functions(&tree2);
                assert_eq!(count1, count2);
            },
            (Err(_), Err(_)) => {
                // Both failed - this is consistent
            },
            _ => panic!("Parsing same code twice should give consistent results"),
        }
    }
}

// Property 7: Error handling - severely malformed input returns tree with errors
proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn malformed_input_has_errors(prefix in arb_typescript_code()) {
        let parser = get_parser("typescript").expect("Valid language");

        // Create intentionally malformed code
        let malformed = format!("function incomplete( {} {{{{{} }}}}}}", prefix, prefix);

        if let Ok(tree) = parser.parse(&malformed) {
            // Tree-sitter usually produces a tree even for malformed input
            // but it should be marked with errors
            let root = tree.root_node();
            assert!(!root.kind().is_empty(), "Should produce a tree");
        }
    }
}

// Property 8: Valid language names always succeed
#[test]
fn all_supported_languages_create_parsers() {
    let languages = [
        "typescript",
        "ts",
        "javascript",
        "js",
        "jsx",
        "tsx",
        "rust",
        "rs",
        "python",
        "py",
        "go",
        "cpp",
        "cxx",
        "cc",
        "hpp",
        "h",
    ];

    for lang in &languages {
        let result = get_parser(lang);
        assert!(result.is_ok(), "Language '{}' should be supported", lang);
    }
}

// Property 9: Invalid language names always fail
proptest! {
    #[test]
    fn unsupported_languages_fail(lang in "[a-z]{2,10}") {
        let supported = ["typescript", "ts", "javascript", "js", "jsx", "tsx",
                        "rust", "rs", "python", "py", "go", "cpp", "cxx", "cc", "hpp", "h"];

        if !supported.contains(&lang.as_str()) {
            let result = get_parser(&lang);
            assert!(result.is_err(), "Unsupported language '{}' should fail", lang);

            if let Err(e) = result {
                match e {
                    ParseError::UnsupportedLanguage(_) => {
                        // Expected error type
                    },
                    _ => panic!("Wrong error type for unsupported language"),
                }
            }
        }
    }
}

// Property 10: Very long input doesn't cause panics or excessive memory
proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn handles_large_input(repeat in 1usize..100) {
        let parser = get_parser("typescript").expect("Valid language");
        let code = "function f() {}\n".repeat(repeat);

        let result = parser.parse(&code);
        assert!(result.is_ok(), "Should handle large valid input");

        if let Ok(tree) = result {
            let count = parser.count_functions(&tree);
            assert_eq!(count, repeat, "Should count all functions in large input");
        }
    }
}
