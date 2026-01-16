//! Circular dependency and timeout error tests

use code_viz_core::error::CodeVizError;

#[test]
fn test_circular_dependency_detection() {
    let error = CodeVizError::analysis(
        "coupling",
        "circular dependency detected: module_a -> module_b -> module_c -> module_a"
    );

    let msg = error.to_string();
    assert!(msg.contains("circular dependency"));
    assert!(msg.contains("module_a"));
    assert!(msg.contains("->"));
}

#[test]
fn test_self_referential_dependency() {
    let error = CodeVizError::analysis(
        "dependency_check",
        "self-referential dependency: module imports itself"
    );

    let msg = error.to_string();
    assert!(msg.contains("self-referential"));
    assert!(msg.contains("imports itself"));
}
