# Coverage Report - Task 31

**Generated:** 2026-01-17
**Spec:** error-handling-code-quality-remediation

## Executive Summary

- **Overall Rust Coverage:** 66.58% (2114/3175 lines covered)
- **TypeScript Coverage:** Not generated (test failures - 50 failed, 555 passed)
- **Critical Finding:** 18 files with 0% coverage
- **Files Below 80% Threshold:** 32 files

## Coverage by Crate

| Crate | Coverage | Lines Covered | Total Lines | Status |
|-------|----------|---------------|-------------|--------|
| code-viz-commands | 87.00% | 168/192 | ✅ Above threshold |
| code-viz-dead-code | 87.00% | 500/570 | ✅ Above threshold |
| code-viz-core | 78.00% | 1046/1332 | ⚠️ Below threshold |
| code-viz-tauri | 70.00% | 108/153 | ⚠️ Below threshold |
| code-viz-api | 51.00% | 54/105 | ❌ Critical |
| code-viz-cli | 31.00% | 235/749 | ❌ Critical |
| code-viz-web | 4.00% | 3/67 | ❌ Critical |

## Critical Gaps (0% Coverage)

### CLI Commands (442 uncovered lines)
- `crates/code-viz-cli/src/commands/analyze.rs` - 149 lines
- `crates/code-viz-cli/src/commands/dead_code.rs` - 104 lines
- `crates/code-viz-cli/src/commands/watch.rs` - 95 lines
- `crates/code-viz-cli/src/commands/diff.rs` - 49 lines
- `crates/code-viz-cli/src/commands/compare.rs` - 32 lines
- `crates/code-viz-cli/src/commands/config.rs` - 8 lines
- `crates/code-viz-cli/src/commands/timeline.rs` - 5 lines

### Application Entry Points (83 uncovered lines)
- `crates/code-viz-cli/src/main.rs` - 36 lines
- `crates/code-viz-web/src/main.rs` - 30 lines
- `crates/code-viz-web/src/routes.rs` - 29 lines
- `crates/code-viz-api/src/error.rs` - 8 lines
- `src-tauri/src/main.rs` - 7 lines

### Context Implementations (30 uncovered lines)
- `crates/code-viz-tauri/src/commands.rs` - 16 lines
- `crates/code-viz-tauri/src/logging.rs` - 12 lines
- `crates/code-viz-cli/src/context/cli_context.rs` - 10 lines
- `crates/code-viz-tauri/src/context/tauri_context.rs` - 10 lines

### Core Utilities (6 uncovered lines)
- `crates/code-viz-core/src/cache.rs` - 4 lines
- `crates/code-viz-core/src/models.rs` - 2 lines

## Files Below 80% Coverage (Non-Zero)

| File | Coverage | Covered | Total | Gap |
|------|----------|---------|-------|-----|
| `crates/code-viz-core/src/context/real_git.rs` | 25% | 49/191 | 142 lines |
| `crates/code-viz-core/src/analyzer.rs` | 31% | 11/35 | 24 lines |
| `crates/code-viz-api/src/handlers.rs` | 34% | 23/66 | 43 lines |
| `crates/code-viz-web/src/context.rs` | 37% | 3/8 | 5 lines |
| `crates/code-viz-core/src/coverage.rs` | 51% | 32/62 | 30 lines |
| `crates/code-viz-core/src/context/real_filesystem.rs` | 57% | 8/14 | 6 lines |
| `crates/code-viz-cli/src/output/text.rs` | 62% | 18/29 | 11 lines |
| `crates/code-viz-core/src/mocks/mock_git.rs` | 68% | 49/72 | 23 lines |
| `crates/code-viz-tauri/src/models.rs` | 72% | 18/25 | 7 lines |
| `crates/code-viz-dead-code/src/symbol_graph/resolver.rs` | 73% | 19/26 | 7 lines |
| `crates/code-viz-commands/src/dead_code.rs` | 75% | 3/4 | 1 line |
| `crates/code-viz-core/src/mocks/mock_filesystem.rs` | 75% | 21/28 | 7 lines |
| `crates/code-viz-dead-code/src/models.rs` | 77% | 21/27 | 6 lines |
| `crates/code-viz-dead-code/src/symbol_graph/queries.rs` | 78% | 22/28 | 6 lines |

## TypeScript Coverage Status

**Status:** ❌ Not Generated

TypeScript coverage was not generated due to test failures:
- **Test Results:** 50 failed, 555 passed (605 total)
- **Failing Test Suites:** 8 failed, 15 passed (23 total)
- **Primary Failures:** `AnalysisView.drilldown.test.tsx` (10 tests failed)

**Recommendation:** Fix failing tests before generating TypeScript coverage. The tests appear to be related to API endpoint issues ("Failed to parse URL from /api/analyze").

## Generated Artifacts

### Rust Coverage Reports
- **HTML Report:** `tarpaulin-report.html` (1.1MB)
- **XML Report:** `cobertura.xml` (106KB)
- **JSON Report:** `tarpaulin-report.json` (932KB)

### Coverage Commands Used

**Rust:**
```bash
cargo tarpaulin --out Html --out Xml --out Json \
  --exclude-files 'tests/*' \
  --exclude-files '*_test.rs' \
  --all-features \
  --workspace \
  --timeout 300
```

**TypeScript (attempted):**
```bash
npm run test:coverage -- \
  --coverage.reporter=html \
  --coverage.reporter=json \
  --coverage.all
```

## Next Steps (Task 32)

Based on this analysis, the following files should be prioritized for test coverage in Task 32:

### Priority 1: CLI Commands (442 lines)
Essential for user-facing functionality testing.

### Priority 2: Application Entry Points (83 lines)
Critical for integration testing.

### Priority 3: Core Git Implementation (142 lines)
`real_git.rs` has only 25% coverage with 142 uncovered lines.

### Priority 4: TypeScript Tests
Fix failing tests to enable coverage generation.

## Coverage Gaps Summary

- **Total Gap to 80%:** ~424 additional lines need coverage in Rust
- **Crates Below Threshold:** 5 out of 7 crates
- **Critical Priority Files:** 18 files with 0% coverage
- **TypeScript:** Full test suite needs stabilization

## Notes

- Coverage excludes test files (`tests/*`, `*_test.rs`) as per configuration
- Some 0% coverage files are entry points (main.rs) which are harder to unit test
- Mock implementations have lower coverage (68-75%), which is acceptable
- Core business logic in `code-viz-dead-code` and `code-viz-commands` has excellent coverage (87%)
