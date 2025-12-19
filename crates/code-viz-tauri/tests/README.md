# Contract Validation Tests

This directory contains contract validation tests that ensure data integrity and interface consistency between the Rust backend and the TypeScript frontend.

## Purpose

The primary goals of these tests are:
1.  **Type Safety**: Catch type definition changes in Rust that might break the TypeScript frontend.
2.  **Serialization Integrity**: Ensure that data (especially complex hierarchical structures like `TreeNode`) is correctly serialized and survives IPC without corruption.
3.  **Frontend Compatibility**: Validate that the JSON output matches the specific requirements of frontend libraries like ECharts.
4.  **Regression Prevention**: Prevent historical bugs (e.g., the "wrapper node bug") from returning.

## Test Structure

Tests are located in `contract_tests.rs` and organized into three main modules:

### 1. Specta Schema Validation (`specta_schema_tests`)
Validates that Rust types correctly generate the expected TypeScript schemas using the `Specta` library. This catches missing or renamed fields at compile-time/test-time.

### 2. Serialization Round-Trip (`serialization_tests`)
Validates that `TreeNode` objects can be serialized to JSON and deserialized back to Rust without loss of data. It also includes recursive validation of required fields and non-empty path assertions.

### 3. ECharts Compatibility (`echarts_compatibility_tests`)
Validates that the JSON structure matches ECharts treemap expectations (e.g., having `name` and `value` mapped from `loc`, and proper `children` arrays).

## Running Tests

You can run the contract tests using `cargo nextest` (recommended) or standard `cargo test`:

```bash
# Using nextest (fastest)
cargo nextest run --package code-viz-tauri --test contract_tests

# Using standard cargo test
cargo test --package code-viz-tauri --test contract_tests
```

## Adding New Tests

When adding new data models that are exposed to the frontend (via `#[specta::specta]`):

1.  Add a schema coverage test in `test_all_specta_types_coverage`.
2.  If it's a hierarchical or complex type, add a round-trip serialization test.
3.  If it's used directly by a frontend visualization library, add a compatibility test in `echarts_compatibility_tests`.

## Motivation: The Wrapper Node Bug

One of the key motivations for these tests was a bug where ECharts created an "undefined" wrapper node when passed a root node with an empty path. Contract tests now enforce that `path` must never be an empty string, ensuring the frontend always receives valid, clickable data.
