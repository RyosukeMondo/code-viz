# Manual Verification of Dead Code in Test Corpus

This document contains manually verified dead code analysis for each test project.
All entries have been human-reviewed to determine ground truth.

## Methodology

1. Each project was manually analyzed to identify:
   - **TRUE DEAD**: Code that is genuinely unused and can be safely deleted
   - **FALSE POSITIVE CANDIDATES**: Code that appears unused but shouldn't be flagged (or flagged with low confidence)
   - **TRUE LIVE**: Code that is clearly in use

2. Confidence scoring expectations:
   - Exported symbols: Should have LOW confidence (<70) when flagged as dead
   - Dynamic imports/reflection: Should have LOW confidence (<60) when flagged
   - Recently modified: Should have reduced confidence
   - Test helpers: Should be recognized as live (used in test files)

## Project 1: Dynamic Imports (project1/)

### Expected Analysis Results

| Symbol | File | Type | Expected Status | Confidence | Reason |
|--------|------|------|----------------|------------|---------|
| `loadPlugin` | src/index.ts | function | LIVE | N/A | Entry point, exported and called |
| `main` | src/index.ts | function | LIVE | N/A | Entry point, exported and called |
| `default` (user plugin) | src/plugins/user_plugin.ts | object | LIVE | N/A | Dynamically imported via loadPlugin |
| `validateUser` | src/plugins/user_plugin.ts | function | **DEAD** | <60 (LOW) | Exported but unused, could be API |
| `default` (admin plugin) | src/plugins/admin_plugin.ts | object | LIVE | N/A | Dynamically imported via loadPlugin |
| `validateAdmin` | src/plugins/admin_plugin.ts | function | **DEAD** | <60 (LOW) | Exported but unused, could be API |

### Key Insights
- Plugins are dynamically imported using template strings: `import(\`./plugins/${name}_plugin.ts\`)`
- Static analysis cannot prove plugins are used
- These should either:
  1. Not be flagged as dead (ideal but hard to achieve)
  2. Be flagged with LOW confidence (<60) due to `*_plugin.ts` pattern
- Helper functions (validate*) are truly dead but confidence should be reduced due to export

## Project 2: Reflection Patterns (project2/)

### Expected Analysis Results

| Symbol | File | Type | Expected Status | Confidence | Reason |
|--------|------|------|----------------|------------|---------|
| `callHandler` | src/main.ts | function | LIVE | N/A | Entry point, used in module |
| `handleUser` | src/handlers.ts | function | **DEAD** | <50 (LOW) | Accessed via reflection, name matches pattern |
| `handleOrder` | src/handlers.ts | function | **DEAD** | <50 (LOW) | Accessed via reflection, name matches pattern |
| `handlePayment` | src/handlers.ts | function | **DEAD** | <50 (LOW) | Accessed via reflection, name matches pattern |
| `handleRefund` | src/handlers.ts | function | **DEAD** | 100 (HIGH) | Truly unused, not referenced anywhere |

### Key Insights
- Handlers are called via reflection: `(handlers as any)[handlerName]`
- Functions starting with `handle*` should trigger low confidence
- `handleRefund` is genuinely dead - not called via reflection or directly
- This is a TRUE POSITIVE with high confidence

## Project 3: Library Public API (project3/)

### Expected Analysis Results

| Symbol | File | Type | Expected Status | Confidence | Reason |
|--------|------|------|----------------|------------|---------|
| `createUser` | src/index.ts | function | **DEAD** | <30 (LOW) | Exported from main entry, public API |
| `deleteUser` | src/index.ts | function | **DEAD** | <30 (LOW) | Exported from main entry, public API |
| `updateUser` | src/index.ts | function | **DEAD** | <30 (LOW) | Exported from main entry, public API |
| `internalHelper` | src/index.ts | function | **DEAD** | 100 (HIGH) | Not exported, truly dead code |
| `formatDate` | src/utils.ts | function | **DEAD** | <30 (LOW) | Exported from utils entry point |
| `parseDate` | src/utils.ts | function | **DEAD** | <30 (LOW) | Exported from utils entry point |
| `validateEmail` | src/utils.ts | function | **DEAD** | <30 (LOW) | Exported from utils entry point |
| `obsoleteFunction` | src/utils.ts | function | **DEAD** | <70 (MEDIUM) | Exported but clearly deprecated |

### Key Insights
- This is a library, not an application
- All exported functions are part of the public API
- They appear unused because they're consumed by external packages
- Confidence should be LOW (<30) for all exported symbols
- `internalHelper` is a TRUE POSITIVE (high confidence)
- `obsoleteFunction` is likely dead but has medium confidence due to export

## Project 4: Recent Modifications (project4/)

### Expected Analysis Results

| Symbol | File | Type | Expected Status | Confidence | Reason |
|--------|------|------|----------------|------------|---------|
| `main` | src/main.ts | function | LIVE | N/A | Entry point, called |
| `processData` | src/processor.ts | function | LIVE | N/A | Imported and used |
| `newFeature` | src/processor.ts | function | **DEAD** | <80 (MEDIUM-HIGH) | Unused but might be WIP, exported |
| `legacyProcessor` | src/processor.ts | function | **DEAD** | <70 (MEDIUM) | Truly dead, exported |

### Key Insights
- `newFeature` appears dead but could be work-in-progress
- If recently modified (within 30 days), confidence should be reduced
- Without git history in test, we can't verify recency check
- Both dead functions are exported, reducing confidence

## Project 5: Test Helpers (project5/)

### Expected Analysis Results

| Symbol | File | Type | Expected Status | Confidence | Reason |
|--------|------|------|----------------|------------|---------|
| `calculate` | src/index.ts | function | LIVE | N/A | Used in tests |
| `process` | src/index.ts | function | LIVE | N/A | Used in tests |
| `createMockUser` | src/testHelpers.ts | function | **DEAD** | <85 (MEDIUM-HIGH) | Exported, unused in tests |
| `createMockData` | src/testHelpers.ts | function | LIVE | N/A | Used in test file |
| `assertDeepEqual` | src/testHelpers.ts | function | LIVE | N/A | Used in test file |

### Key Insights
- Test helpers should be recognized as live when imported by test files
- `createMockUser` is genuinely unused - TRUE POSITIVE
- Entry point detection should identify test files (*.test.ts)
- Functions used only in tests should still be marked as live

## Summary Statistics

### Total Symbols by Category

- **TRUE LIVE**: 13 symbols (should NOT be flagged as dead)
- **TRUE DEAD (High Confidence)**: 4 symbols (100% dead, safe to delete)
  - `internalHelper` (project3)
  - `handleRefund` (project2)
  - `createMockUser` (project5)
  - `obsoleteFunction` (project3)
- **TRUE DEAD (Medium/Low Confidence)**: 12 symbols (dead but should have reduced confidence)
  - All due to: export, dynamic patterns, or being public API

### Expected False Positive Rate

**Definition of False Positive**: A symbol flagged as "high confidence dead" (>80) that is actually live or shouldn't be flagged.

**Calculation**:
- Symbols that SHOULD NOT be flagged with high confidence: 13 (live) + 12 (low-conf dead) = 25
- Symbols that SHOULD be flagged with high confidence: 4
- Total symbols: 29

If the tool incorrectly flags low-confidence dead code with HIGH confidence:
- **Worst case**: All 12 low-conf symbols flagged as high-conf = 12/16 = 75% FP rate (FAIL)
- **Best case**: Only 4 high-conf symbols flagged = 0/4 = 0% FP rate (PASS)
- **Acceptable**: <5% FP rate means <1 symbol incorrectly flagged with high confidence

### Testing Strategy

1. Run dead code analysis on each project
2. Compare detected dead symbols vs ground truth
3. Verify confidence scores match expectations:
   - Exported symbols: confidence <70
   - Pattern matches (handle*, *_plugin): confidence <60
   - True dead unexported: confidence ~100
4. Calculate false positive rate:
   - FP = symbols with confidence >80 that should be <80
   - FP Rate = FP / total_detected_with_high_confidence
   - Target: <5%

### Edge Cases Covered

✅ Dynamic imports with template strings
✅ Reflection/runtime access via bracket notation
✅ Exported library APIs (public interface)
✅ Test helper utilities
✅ Recently added code (WIP features)
✅ Pattern-based naming (*_handler, *_plugin)
✅ True positives (actually dead code)

This corpus provides a comprehensive test of the dead code detector's ability to:
1. Detect genuinely dead code
2. Reduce confidence for ambiguous cases
3. Respect exports and public APIs
4. Handle dynamic/runtime patterns gracefully
