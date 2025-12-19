# Tasks Document: CLI Integration Testing

- [x] 1. Create test directory structure and basic fixtures
  - Files: crates/code-viz-cli/tests/fixtures/{simple-repo, empty-repo}/
  - Create simple-repo with 3 files, 1 subdirectory (known structure)
  - Create empty-repo with no files (edge case)
  - Purpose: Establish foundation for CLI integration testing
  - _Leverage: Git for version control of fixtures_
  - _Requirements: Requirement 3_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Test Engineer with expertise in test fixture design and repository structures
Task: Create tests/fixtures/ directory in crates/code-viz-cli with two fixture repositories following requirement 3: simple-repo (3 Rust files totaling ~500 LOC, 1 src/ subdirectory) and empty-repo (empty directory for edge case testing). Structure simple-repo as a minimal but realistic Rust project (lib.rs, mod.rs, helper.rs). Commit fixtures to Git so they're versioned.
Restrictions: Fixtures must be deterministic (no timestamps, no random data), simple-repo must be analyzable by CLI, files should have realistic Rust code (not placeholders), keep total size <10KB, fixtures should not depend on external files
Success: tests/fixtures/simple-repo/ created with 3 .rs files and src/ subdirectory; empty-repo/ exists as empty directory; fixtures committed to Git; can manually run CLI on simple-repo without errors; file structures are deterministic
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). After creating fixtures, verify CLI can analyze simple-repo with cargo run --bin code-viz -- analyze tests/fixtures/simple-repo. Use log-implementation tool to record artifacts (fixture paths, file counts, size). Mark complete in tasks.md (change [-] to [x])._

- [x] 2. Create expected JSON outputs for fixtures
  - Files: crates/code-viz-cli/tests/expected/{simple-repo.json, empty-repo.json}
  - Generate expected outputs by running current CLI on fixtures
  - Validate JSON structure matches TreeNode schema
  - Purpose: Establish baseline expected outputs for regression testing
  - _Leverage: jq for JSON validation, current CLI binary_
  - _Requirements: Requirement 2_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: QA Engineer with expertise in golden file testing and JSON validation
Task: Generate and validate expected JSON outputs for fixtures following requirement 2. Run CLI on each fixture, capture JSON output to tests/expected/{fixture}.json, validate with jq (check for required fields: name, path, type, value, children). Ensure outputs are deterministic (no timestamps) and pretty-printed for diff-friendliness.
Restrictions: Expected outputs must match actual CLI output exactly (byte-for-byte), must validate JSON is parseable and has required TreeNode fields, outputs should be pretty-printed (jq .) for readability, no manual editing of expected files (always generated from CLI)
Success: simple-repo.json and empty-repo.json created in tests/expected/; files are valid JSON parseable by jq; contain TreeNode structure with all required fields; simple-repo.json has 3+ file nodes; empty-repo.json represents empty analysis result; files are formatted for easy diffs
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Generate expected outputs with: cargo run --bin code-viz -- analyze tests/fixtures/simple-repo --format json | jq . > tests/expected/simple-repo.json. Validate with jq -e '.name' tests/expected/simple-repo.json. Use log-implementation tool to record artifacts (expected files created, validation commands). Mark complete in tasks.md (change [-] to [x])._

- [ ] 3. Create shell test utilities and framework
  - File: crates/code-viz-cli/tests/helpers/test_utils.sh
  - Implement build_cli(), run_cli_test(), validate_json() functions
  - Add compare_json_files() for diffing actual vs expected
  - Purpose: Provide reusable shell utilities for CLI testing
  - _Leverage: bash, jq, diff commands_
  - _Requirements: Requirement 1_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: DevOps Engineer with expertise in bash scripting and test automation
Task: Create helpers/test_utils.sh with reusable shell functions following requirement 1: build_cli() builds CLI if not present and returns path, run_cli_test(fixture, expected) executes CLI and compares output, validate_json(file) checks JSON is valid, compare_json_files(actual, expected) shows diff if mismatch. Functions should have error handling and descriptive output.
Restrictions: Must use portable bash (no bashisms requiring bash 5+), must check command availability (jq, cargo), functions should return proper exit codes (0 = success, non-zero = failure), error messages should be descriptive, utilities must work in CI environment (non-interactive)
Success: test_utils.sh created with 4+ utility functions; build_cli() compiles CLI and caches result; run_cli_test() executes CLI and compares with expected output; validate_json() uses jq to check JSON validity; compare_json_files() shows clear diff on mismatch; all functions have error handling
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Test each utility function manually. Use log-implementation tool to record artifacts (functions created, utilities functionality, error handling). Mark complete in tasks.md (change [-] to [x])._

- [ ] 4. Create shell-based integration test script
  - File: crates/code-viz-cli/tests/cli_analysis_tests.sh
  - Implement test_simple_analysis(), test_empty_repository(), test_invalid_path()
  - Add test runner that executes all tests and reports results
  - Purpose: Fast validation of CLI behavior via shell tests
  - _Leverage: helpers/test_utils.sh, fixtures/_, expected/_
  - _Requirements: Requirements 2, 4_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Test Automation Engineer with expertise in shell scripting and integration testing
Task: Create cli_analysis_tests.sh with shell-based CLI tests following requirements 2 and 4: test_simple_analysis() runs CLI on simple-repo and validates output matches expected, test_empty_repository() handles empty-repo gracefully, test_invalid_path() verifies error handling for non-existent paths. Include test runner that executes all tests, counts passes/failures, exits with proper code. Source test_utils.sh for utilities.
Restrictions: Tests must be independent (isolated temp directories), must not modify fixtures, must clean up temporary files, should complete in <30 seconds total, test names must start with test_, runner must show summary (X/Y tests passed)
Success: cli_analysis_tests.sh executable with test runner; test_simple_analysis() passes with valid fixture; test_empty_repository() handles empty case without crash; test_invalid_path() verifies exit code non-zero; runner shows clear pass/fail summary; all tests complete in <10 seconds; tests can run in any order
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Run tests with bash tests/cli_analysis_tests.sh and verify all pass. Use log-implementation tool to record artifacts (test functions added, test runner logic, pass/fail results). Mark complete in tasks.md (change [-] to [x])._

- [ ] 5. Create Rust integration test with detailed validation
  - File: crates/code-viz-cli/tests/integration_tests.rs
  - Implement test_cli_output_format() with serde_json parsing
  - Add test_cli_all_required_fields() for TreeNode validation
  - Add test_cli_performance() with timeout assertions
  - Purpose: Detailed programmatic validation of CLI outputs
  - _Leverage: std::process::Command, serde_json, code-viz-core::TreeNode_
  - _Requirements: Requirements 2, 4, 5_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Rust Integration Test Engineer with expertise in std::process and JSON validation
Task: Create integration_tests.rs with Rust-based CLI tests following requirements 2, 4, and 5: test_cli_output_format() executes CLI via Command, parses JSON output with serde_json, deserializes to TreeNode type; test_cli_all_required_fields() validates all required fields present recursively; test_cli_performance() runs CLI on large fixture with timeout (<30s). Use std::process::Command for CLI execution.
Restrictions: Must build CLI before tests (build script or manual cargo build), must deserialize to actual TreeNode type (not generic JSON), must validate JSON structure programmatically (not string comparison), performance test must fail if timeout exceeded, tests must use fixture paths relative to workspace root
Success: integration_tests.rs compiles and runs; test_cli_output_format() successfully parses CLI JSON into TreeNode; test_cli_all_required_fields() recursively validates all nodes have name/path/type/value; test_cli_performance() completes in <30s; tests discoverable by cargo nextest; all tests pass
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Run tests with cargo nextest run --package code-viz-cli --test integration_tests. Use log-implementation tool to record artifacts (test functions, TreeNode validation, performance benchmarks). Mark complete in tasks.md (change [-] to [x])._

- [ ] 6. Add complex fixture repositories (nested, large)
  - Files: tests/fixtures/{nested-repo, large-repo}/
  - Create nested-repo with 5+ levels of directory nesting
  - Create large-repo with 100+ files (generated, not manual)
  - Generate expected outputs for new fixtures
  - Purpose: Comprehensive edge case and performance testing
  - _Leverage: Fixture generation scripts, existing test patterns_
  - _Requirements: Requirements 3, 4_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Test Data Engineer with expertise in test fixture generation and edge case design
Task: Create advanced fixture repositories following requirements 3 and 4: nested-repo with 5+ directory levels testing deep hierarchy handling, large-repo with 100+ files testing performance and scalability. Generate large-repo programmatically (script to create files) for maintainability. Generate expected outputs using CLI. Add performance assertions for large-repo (<30s analysis time).
Restrictions: nested-repo must be realistic project structure (not artificial deep nesting), large-repo should be generated from template (not 100 manual files), fixtures must remain deterministic, total fixture size should be <1MB, expected outputs must be validated (parseable JSON with correct structure)
Success: nested-repo created with 5+ directory levels and realistic file distribution; large-repo generated with 100+ files via script; expected outputs for both fixtures created and validated; CLI handles nested-repo without stack overflow; CLI analyzes large-repo in <30s; fixtures committed to Git
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Create generation script for large-repo (e.g., Rust/bash script). Verify CLI performance on large-repo. Use log-implementation tool to record artifacts (fixtures created, file counts, performance metrics, generation approach). Mark complete in tasks.md (change [-] to [x])._

- [ ] 7. Integrate CLI tests into CI pipeline
  - File: .github/workflows/test.yml (modify) OR create new workflow
  - Add job for shell tests: bash tests/cli_analysis_tests.sh
  - Add job for Rust integration tests: cargo nextest run --test integration_tests
  - Configure jobs to fail PR if any CLI test fails
  - Purpose: Enforce CLI validation on every commit
  - _Leverage: existing CI configuration, cargo-nextest_
  - _Requirements: Requirement 5_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: CI/CD Engineer with expertise in GitHub Actions and test automation
Task: Integrate CLI tests into CI pipeline following requirement 5, adding jobs to run shell tests (bash tests/cli_analysis_tests.sh) and Rust integration tests (cargo nextest run --package code-viz-cli --test integration_tests). Configure 30-second timeout for full suite, ensure jobs block PR on failure, enable parallel execution with contract tests. Install jq if needed.
Restrictions: Must not break existing CI jobs, shell tests require jq (install if missing), CLI must be built before tests run, timeout should fail build if exceeded, jobs should run in parallel for speed, cache CLI build between test runs
Success: CI runs shell and Rust CLI tests on all PRs; failed tests block PR merging; tests complete in <30 seconds with timeout enforcement; jq available in CI environment; CLI build cached for performance; job status appears in GitHub PR checks; tests run in parallel with other test jobs
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Create test PR to verify CLI tests run in CI and appear in status checks. Use log-implementation tool to record artifacts (CI jobs added, timeout configured, caching strategy, integration verified). Mark complete in tasks.md (change [-] to [x])._

- [ ] 8. Documentation and migration plan
  - Files: crates/code-viz-cli/tests/README.md, docs/UAT_COST_REDUCTION_PLAN.md (update)
  - Document CLI test framework usage and how to add new tests
  - Update UAT plan with CLI testing results (6x speedup achieved)
  - Create migration plan for converting E2E tests to CLI tests
  - Purpose: Enable team to leverage CLI testing and track UAT savings
  - _Leverage: existing test documentation patterns_
  - _Requirements: All requirements (documentation)_
  - _Prompt: **Implement the task for spec cli-integration-testing, first run spec-workflow-guide to get the workflow guide then implement the task:**
Role: Technical Writer and QA Strategist with expertise in testing documentation and process improvement
Task: Create comprehensive documentation covering all requirements: write tests/README.md explaining CLI test framework (shell + Rust), how to run tests (cargo nextest, bash), how to add fixtures and expected outputs; update UAT_COST_REDUCTION_PLAN.md with implementation results (baseline E2E time vs CLI time, speedup factor, cost savings); create migration guide for converting E2E tests to CLI format.
Restrictions: Documentation must be actionable with concrete examples, must include before/after performance metrics, migration plan should prioritize which E2E tests to convert first, keep concise (<800 lines total), include troubleshooting section for common issues
Success: README explains CLI testing framework clearly with examples; includes commands to run tests and add new fixtures; UAT plan updated with actual measurements (e.g., "CLI tests: 25s vs E2E: 180s = 7.2x speedup"); migration plan identifies 2-3 E2E tests to convert first; troubleshooting section covers jq installation, fixture generation, JSON validation
**Instructions**: First mark this task as in-progress in tasks.md (change [ ] to [-]). Measure actual test execution times for documentation. Use log-implementation tool to record artifacts (documentation created, performance measurements, migration recommendations). Mark complete in tasks.md (change [-] to [x])._
