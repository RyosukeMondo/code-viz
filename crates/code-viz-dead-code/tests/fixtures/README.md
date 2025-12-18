# Test Fixtures for Dead Code Detection

This directory contains test fixtures for validating dead code analysis accuracy.

## Structure

```
fixtures/
├── sample-repo/          # Sample TypeScript repository with known dead code
│   ├── src/
│   │   ├── main.ts       # Entry point (LIVE)
│   │   ├── used.ts       # Functions used by main/tests (LIVE)
│   │   ├── dead.ts       # Completely unused exports (DEAD)
│   │   ├── internal.ts   # Mix of live and dead code
│   │   ├── circular-a.ts # Circular dependency test (DEAD)
│   │   ├── circular-b.ts # Circular dependency test (DEAD)
│   │   ├── index.ts      # Re-export test (DEAD)
│   │   └── utils/
│   │       └── helper.ts # Dead utilities (DEAD)
│   ├── tests/
│   │   └── app.test.ts   # Test file entry point (LIVE)
│   └── package.json      # Package configuration
├── EXPECTED.md           # Ground truth - manually verified results
└── README.md            # This file
```

## Purpose

The `sample-repo` provides a realistic TypeScript codebase with:
- **Known dead code** (manually verified)
- **Edge cases**: circular imports, re-exports, transitive dead code
- **Multiple entry points**: main.ts and test files
- **Mixed scenarios**: exported vs unexported, used vs unused

## Usage

Integration tests use this corpus to:
1. Run dead code analysis on `sample-repo/`
2. Compare results against `EXPECTED.md`
3. Verify accuracy metrics (false positive rate < 5%)
4. Test edge case handling (circular imports, etc.)

## Ground Truth

See [EXPECTED.md](./EXPECTED.md) for the complete list of expected dead code symbols with confidence scores and rationale.

**Summary**:
- Total symbols: ~32
- Live symbols: 11
- Dead symbols: 21
- Expected accuracy: 100% (manually verified)

## Adding New Test Cases

To add new test scenarios:

1. Create new TypeScript files in `sample-repo/src/`
2. Manually verify which symbols are dead/live
3. Document in `EXPECTED.md` with confidence scores
4. Update this README with the new scenario
5. Run integration tests to validate

## Validation Methodology

Each symbol in the corpus has been:
1. Manually traced from entry points (main.ts, test files)
2. Verified as reachable or unreachable
3. Assigned expected confidence score based on design.md rules
4. Documented with rationale in EXPECTED.md

This ensures the test corpus serves as reliable ground truth for accuracy validation.
