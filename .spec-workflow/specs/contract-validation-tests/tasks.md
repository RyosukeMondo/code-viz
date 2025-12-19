# Tasks Document: Contract Validation Tests

- [x] 1. Create contract test file and validation error types
  - File: crates/code-viz-tauri/tests/contract_tests.rs
  - Define ValidationError enum with thiserror
  - Set up test module structure (specta_schema_tests, serialization_tests, echarts_compatibility_tests)
  - Purpose: Establish foundation for all contract validation tests
  - _Leverage: thiserror crate, existing test patterns in crates/code-viz-core/src/lib.rs_
  - _Requirements: Requirements 1, 2, 3_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Rust Test Engineer specializing in type-safe systems and error handling
Task: Create the contract_tests.rs file in crates/code-viz-tauri/tests/ with ValidationError enum using thiserror, and set up three test modules (specta_schema_tests, serialization_tests, echarts_compatibility_tests) following requirements 1, 2, and 3. Leverage existing error patterns from thiserror crate and test organization patterns from crates/code-viz-core tests.
Restrictions: Do not add test implementations yet (only structure), must use thiserror for error types, follow Rust test module naming conventions with mod tests syntax, do not create dependencies on Tauri runtime
Success: File created with ValidationError enum covering MissingField, InvalidValue, and SchemaMismatch variants; three empty test modules created with descriptive doc comments; compiles without errors; follows existing project test structure patterns
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After completing implementation and testing, use the log-implementation tool to record detailed implementation with artifacts (error types created, test modules added, file location). Then mark task as complete in tasks.md (change [-] to [x])._

- [x] 2. Create test fixtures and helper utilities
  - File: crates/code-viz-tauri/tests/helpers/validation_utils.rs
  - Implement create_test_tree() function generating sample TreeNode
  - Add assert_required_fields() helper for recursive validation
  - Purpose: Provide reusable test data and validation utilities
  - _Leverage: code-viz-core::TreeNode, existing test fixture patterns_
  - _Requirements: Requirements 2, 3_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Rust Test Engineer with expertise in test data generation and fixtures
Task: Create helpers/validation_utils.rs with test fixture functions following requirements 2 and 3, implementing create_test_tree() to generate realistic TreeNode structures with proper hierarchies (root with children, various file types), and assert_required_fields() helper for recursive JSON validation. Ensure test data never has empty string paths (prevents wrapper node bug regression).
Restrictions: Test fixtures must have non-empty paths for all nodes, must use realistic file names and structures, helpers should be reusable across all test modules, no business logic in helpers (pure test utilities only)
Success: create_test_tree() generates valid TreeNode with 3+ children, multiple depth levels, various node types; assert_required_fields() recursively validates JSON objects; fixtures are deterministic (same output every run); helpers compile and are usable from parent test module
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After completing implementation, use the log-implementation tool to record artifacts (functions created: create_test_tree signature, assert_required_fields signature, file location). Then mark task as complete in tasks.md (change [-] to [x])._

- [-] 3. Implement Specta schema validation tests
  - File: crates/code-viz-tauri/tests/contract_tests.rs (specta_schema_tests module)
  - Add test_validate_tree_node_schema() to validate TreeNode Specta schema
  - Add test_all_specta_types_coverage() to ensure 100% coverage
  - Purpose: Catch type definition changes at compile-time
  - _Leverage: specta::Type trait, code-viz-core types_
  - _Requirements: Requirement 1_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Rust Type System Expert specializing in Specta and TypeScript code generation
Task: Implement Specta schema validation tests in the specta_schema_tests module following requirement 1, creating test_validate_tree_node_schema() to extract and validate TreeNode's Specta schema contains all required fields (name, path, type, value, children), and test_all_specta_types_coverage() to iterate all #[specta::specta] annotated types. Use specta::Type::reference() to extract schemas.
Restrictions: Tests must not modify production code, must validate schema structure not serialization, should test both presence and types of fields, do not hardcode expected schemas (validate programmatically), tests must run in <1 second
Success: test_validate_tree_node_schema() passes and validates all required fields exist in schema; test_all_specta_types_coverage() identifies all types with #[specta::specta] annotation; tests fail if required field removed from TreeNode; clear error messages on failure; tests execute in milliseconds
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After implementing tests, verify they pass with cargo nextest run. Use log-implementation tool to record artifacts (test functions added, fields validated, execution time). Then mark task as complete in tasks.md (change [-] to [x])._

- [ ] 4. Implement serialization round-trip validation tests
  - File: crates/code-viz-tauri/tests/contract_tests.rs (serialization_tests module)
  - Add test_tree_node_serialization_round_trip() for Rust → JSON → Rust
  - Add test_no_empty_string_paths() to prevent wrapper node bug
  - Add test_recursive_children_validation() for nested structures
  - Purpose: Ensure data survives IPC without corruption
  - _Leverage: serde_json, helpers/validation_utils.rs_
  - _Requirements: Requirement 2_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Rust Serialization Expert with expertise in Serde and JSON validation
Task: Implement serialization round-trip tests in serialization_tests module following requirement 2, creating test_tree_node_serialization_round_trip() to serialize TreeNode to JSON and deserialize back verifying equality, test_no_empty_string_paths() that fails if any path field is empty string (regression test for wrapper node bug), and test_recursive_children_validation() to validate all descendants. Use serde_json::to_value() and from_value().
Restrictions: Must test all TreeNode variants (files and directories), must validate JSON intermediate format (not just Rust round-trip), empty string paths must trigger test failure with descriptive error, recursive validation must check all levels of nesting, tests must be deterministic
Success: Round-trip test preserves all TreeNode data exactly; empty string path test fails with error "path must not be empty string"; recursive validation checks all children and grandchildren; tests use fixtures from helpers module; all tests pass in <100ms total
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After implementation, run tests and verify the empty string path test correctly fails when given bad data, then fix test data and verify pass. Use log-implementation tool with artifacts (test functions, validation strategy, edge cases covered). Mark complete in tasks.md (change [-] to [x])._

- [ ] 5. Implement ECharts compatibility validation tests
  - File: crates/code-viz-tauri/tests/contract_tests.rs (echarts_compatibility_tests module)
  - Add test_echarts_treemap_format() to validate data structure
  - Add test_all_nodes_have_required_properties() for recursive validation
  - Purpose: Ensure frontend visualization libraries receive correct data
  - _Leverage: serde_json::Value, helpers/validation_utils.rs_
  - _Requirements: Requirement 3_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Full-Stack Integration Engineer with expertise in ECharts and data validation
Task: Implement ECharts compatibility tests in echarts_compatibility_tests module following requirement 3, creating test_echarts_treemap_format() to validate TreeNode JSON matches ECharts treemap expectations (name, value, children array structure), and test_all_nodes_have_required_properties() to recursively verify each node has name (string), path (string), type (string), value (number). Serialize test fixtures to JSON and validate structure.
Restrictions: Must validate JSON structure not Rust types, must check both root and all descendants recursively, must verify field types (string vs number) not just presence, undefined or null values must trigger failure, tests should validate actual ECharts requirements from docs
Success: test_echarts_treemap_format() validates root node has children array with valid elements; test_all_nodes_have_required_properties() recursively checks all descendants (3+ levels deep); tests fail with descriptive errors indicating which node and field failed; validation covers all ECharts required fields; tests complete in <200ms
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Review ECharts treemap documentation to confirm required fields. After implementation, use log-implementation tool to record artifacts (validation functions created, ECharts fields checked, recursive validation depth). Mark complete in tasks.md (change [-] to [x])._

- [ ] 6. Add CI integration and test execution validation
  - File: .github/workflows/test.yml (modify existing) OR create new workflow
  - Add contract test job running cargo nextest run --package code-viz-tauri --test contract_tests
  - Configure job to fail PR if contract tests fail
  - Add performance assertion (tests must complete in <5 seconds)
  - Purpose: Enforce contract validation on every commit
  - _Leverage: existing CI configuration, cargo-nextest_
  - _Requirements: Requirement 4_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: DevOps Engineer specializing in CI/CD pipelines and test automation
Task: Integrate contract tests into CI pipeline following requirement 4, modifying existing GitHub Actions workflow or creating new workflow file to run contract tests via cargo nextest, configure job to block PR merging on failure, add timeout assertion ensuring tests complete in <5 seconds. Ensure tests run in parallel with other test jobs for optimal CI performance.
Restrictions: Must not break existing CI jobs, must use cargo nextest (already in project), timeout must fail build if exceeded, contract tests should run early in pipeline (before E2E), do not run contract tests on every commit to feature branches (only on PR)
Success: CI job runs contract tests on all PRs; failed contract tests block PR merging; tests complete in <5 seconds (with assertion/timeout); job appears in GitHub PR status checks; CI configuration follows existing project patterns; documentation updated to mention contract tests in CI
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After implementation, create test PR to verify contract tests run and status check appears. Use log-implementation tool to record artifacts (CI job added, timeout configured, integration verified). Mark complete in tasks.md (change [-] to [x])._

- [ ] 7. Add regression test for wrapper node bug
  - File: crates/code-viz-tauri/tests/contract_tests.rs (add to serialization_tests module)
  - Create test_wrapper_node_bug_regression() specifically testing path: "" scenario
  - Document the bug in test comments with reference to WRAPPER_NODE_FIX.md
  - Purpose: Ensure the wrapper node bug never returns
  - _Leverage: WRAPPER_NODE_FIX.md documentation, existing test fixtures_
  - _Requirements: Requirement 2_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Quality Assurance Engineer specializing in regression testing and bug prevention
Task: Create specific regression test for the wrapper node bug following requirement 2, implementing test_wrapper_node_bug_regression() that validates TreeNode with path: "" (empty string) fails validation with clear error message. Add comprehensive doc comments referencing WRAPPER_NODE_FIX.md and explaining the historical bug (ECharts created wrapper with undefined properties from empty string path).
Restrictions: Test must fail when given path: "" but pass with path: "/" or any non-empty string, error message must mention "wrapper node" or "empty path", test should validate both root and child nodes, must not duplicate logic from test_no_empty_string_paths (different focus)
Success: test_wrapper_node_bug_regression() specifically tests the exact scenario from WRAPPER_NODE_FIX.md; test has comprehensive documentation explaining the bug; test fails with clear error when given empty path; test prevents regression by validating contract assumptions
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Read WRAPPER_NODE_FIX.md to understand the bug. After implementation, verify test fails with empty path and passes with valid path. Use log-implementation tool to record artifacts (regression test added, bug documentation, validation logic). Mark complete in tasks.md (change [-] to [x])._

- [ ] 8. Documentation and knowledge transfer
  - File: crates/code-viz-tauri/tests/README.md (create new)
  - Document contract test purpose, structure, and how to run
  - Add examples of common validation patterns
  - Update main README.md with contract testing section
  - Purpose: Enable team to understand and extend contract tests
  - _Leverage: existing documentation patterns, inline test comments_
  - _Requirements: All requirements (documentation)_
  - _Prompt: **Implement the task for spec contract-validation-tests, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Technical Writer with expertise in developer documentation and testing best practices
Task: Create comprehensive documentation for contract tests covering all requirements, writing crates/code-viz-tauri/tests/README.md explaining what contract tests are, why they exist (UAT cost reduction, wrapper node bug prevention), test structure (three modules), how to run tests (cargo nextest run --test contract_tests), how to add new validations. Update root README.md with Testing section mentioning contract tests in test pyramid.
Restrictions: Documentation must be accessible to developers unfamiliar with Specta, must include concrete examples with code snippets, should reference wrapper node bug as motivation, must explain when to add new contract tests (when adding #[specta::specta] types), keep concise (<500 lines total)
Success: README explains contract test purpose clearly; includes running instructions (cargo commands); shows examples of adding new validation tests; updated root README has test pyramid diagram with contract tests; documentation is clear and actionable; all test modules have descriptive doc comments
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After writing documentation, ask colleague to review for clarity or do self-review. Use log-implementation tool to record artifacts (README created, documentation sections added, examples provided). Mark complete in tasks.md (change [-] to [x])._
