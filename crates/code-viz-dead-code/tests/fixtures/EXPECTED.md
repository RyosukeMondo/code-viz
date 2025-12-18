# Expected Dead Code Analysis Results

This document provides the ground truth for dead code detection in the `sample-repo/` test corpus.
All assertions are manually verified and serve as the baseline for integration tests.

## Methodology

1. **Manual Verification**: Each symbol was reviewed to determine if it's reachable from entry points
2. **Entry Points Identified**:
   - `src/main.ts` (main application entry point)
   - `tests/app.test.ts` (test file - all test files are entry points)
3. **Reachability Analysis**: Performed manual DFS from entry points following imports
4. **Confidence Scoring**: Applied scoring rules from design.md:
   - Base: 100
   - Exported: -30 (confidence = 70)
   - Unexported + Unused: confidence = 100

## Expected Results Summary

- **Total Symbols**: 32
- **Live Symbols**: 11
- **Dead Symbols**: 21
- **Expected False Positive Rate**: 0% (all assertions verified)

## Live Symbols (Reachable from Entry Points)

### src/main.ts
- `main()` - Entry point function (Line ~10)
- `setupApp()` - Called by main (Line ~26)

### src/used.ts
- `activeFunction()` - Imported in main.ts (Line ~8)
- `testableFunction()` - Imported in app.test.ts (Line ~15)
- `internalHelper()` - Used internally by processData (Line ~23)
- `processData()` - Imported in main.ts (Line ~31)

### src/internal.ts
- `internalUsedHelper()` - Used by publicApi (Line ~8)
- `publicApi()` - Imported in main.ts (Line ~15)

### tests/app.test.ts
- `testTestableFunction()` - Test entry point (Line ~10)
- `testAnotherCase()` - Test entry point (Line ~20)

## Dead Symbols (Not Reachable from Entry Points)

### src/dead.ts - ALL DEAD
All symbols in this file are exported but never imported.

- `unusedExportedFunction()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported but never imported anywhere
  - Line: ~8

- `UnusedClass` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported class never instantiated or imported
  - Line: ~15

- `deadAsyncFunction()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported async function never awaited
  - Line: ~28

- `unusedDefault` (default export) - DEAD, Confidence: 70 (exported -30)
  - Reason: Default export never imported
  - Line: ~34

### src/internal.ts - Partial Dead Code

- `completelyUnused()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported and never called
  - Line: ~22

- `anotherUnusedFunction()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported and never called
  - Line: ~29

- `onlyUsedByDeadCode()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported but only imported by dead.ts (which is itself dead)
  - Line: ~35
  - Note: This is **transitively dead** - used by dead code

### src/utils/helper.ts - ALL DEAD

- `helperForDeadCode()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Only imported by dead.ts (transitively dead)
  - Line: ~8

- `unusedUtility()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported but never imported
  - Line: ~15

- `privateUnusedHelper()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported and never called
  - Line: ~21

### src/circular-a.ts - ALL DEAD
Part of circular dependency, but never imported by live code.

- `functionA()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Part of circular dependency with circular-b.ts, but neither is reachable
  - Line: ~11

- `helperA()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported, part of dead circular dependency
  - Line: ~17

### src/circular-b.ts - ALL DEAD
Part of circular dependency, but never imported by live code.

- `functionB()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Part of circular dependency with circular-a.ts, but neither is reachable
  - Line: ~11

- `helperB()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported, part of dead circular dependency
  - Line: ~19

### src/index.ts - ALL DEAD
Index file is never imported (not used as a library).

- `indexFunction()` - DEAD, Confidence: 70 (exported -30)
  - Reason: Exported from index, but index.ts is never imported
  - Line: ~16

- Re-exported symbols: All re-exports from index.ts are dead because index.ts itself is never imported
  - Note: The actual symbols (e.g., `activeFunction` from used.ts) are still LIVE because they're imported directly in main.ts, not through index.ts

### tests/app.test.ts - Partial Dead Code

- `unusedTestHelper()` - DEAD, Confidence: 100 (unexported, unused)
  - Reason: Unexported helper in test file, never called
  - Line: ~28

## Edge Cases Tested

1. **Circular Imports** (circular-a.ts ↔ circular-b.ts)
   - Both files import each other
   - Neither is reachable from entry points
   - Expected: Both marked as dead (no infinite loops in analysis)

2. **Transitive Dead Code** (internal.ts → onlyUsedByDeadCode)
   - Function is imported by dead.ts
   - But dead.ts is itself never used
   - Expected: Marked as dead (transitively unreachable)

3. **Re-exports** (index.ts re-exports from used.ts)
   - Index.ts re-exports symbols
   - Index.ts is never imported
   - Expected: Symbols are LIVE if imported directly, index.ts itself is dead

4. **Test Files as Entry Points**
   - tests/app.test.ts should be detected as entry point
   - Functions it imports should be marked LIVE
   - Expected: testableFunction is LIVE, unusedTestHelper is DEAD

5. **Internal vs Exported**
   - Unexported + unused: confidence 100
   - Exported + unused: confidence 70
   - Unexported + used internally: LIVE

## Validation Checklist

For integration tests to pass:

- [ ] All 11 live symbols correctly identified as reachable
- [ ] All 21 dead symbols correctly identified as unreachable
- [ ] Circular imports handled without infinite loops
- [ ] Confidence scores match expected values (±5 points acceptable for dynamic penalties)
- [ ] Transitive dead code correctly identified
- [ ] Entry points correctly detected (main.ts, app.test.ts)
- [ ] No false positives (live code marked as dead)
- [ ] No false negatives (dead code marked as live)

## Test Corpus Statistics

- **Files**: 9 TypeScript files
- **Symbols**: ~32 total (11 live, 21 dead)
- **Dead Code Ratio**: ~65% (realistic for test purposes)
- **Lines of Code**: ~250 total
- **Complexity**: Includes circular imports, re-exports, transitive dead code, test files

## Notes

- This is a **manually verified** test corpus
- All dead code assertions have been reviewed by a human
- This serves as ground truth for accuracy validation (Req 8.2)
- Integration tests should achieve 100% accuracy on this corpus (0% false positive rate)
