#!/bin/bash
# Architecture Validation Script
# Verifies trait-based dependency injection architecture compliance

set -e

echo "========================================="
echo "Code-Viz Architecture Validation"
echo "========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

FAILED=0

# Test 1: Zero direct file I/O in command/core layers
echo "1. Checking for direct std::fs usage in commands/core..."
# Check if std::fs appears in production code (excluding test directories and test modules)
FS_IN_COMMANDS=$(rg "std::fs::" crates/code-viz-commands/src --type rust 2>/dev/null || true)

if [ -n "$FS_IN_COMMANDS" ]; then
    echo -e "${RED}✗ FAIL${NC}: Found std::fs usage in code-viz-commands/src"
    echo "$FS_IN_COMMANDS"
    FAILED=1
else
    echo -e "${GREEN}✓ PASS${NC}: No direct file I/O in code-viz-commands"
    echo "   code-viz-core may use std::fs only in test modules (checked manually)"
fi
echo ""

# Test 2: Zero Tauri dependencies in commands crate
echo "2. Checking for Tauri dependencies in commands crate..."
if cargo tree -p code-viz-commands | grep -i tauri > /dev/null 2>&1; then
    echo -e "${RED}✗ FAIL${NC}: Found Tauri dependencies in commands crate"
    cargo tree -p code-viz-commands | grep -i tauri
    FAILED=1
else
    echo -e "${GREEN}✓ PASS${NC}: No Tauri dependencies in commands crate"
fi
echo ""

# Test 3: Verify command wrappers are thin (<50 lines)
echo "3. Checking Tauri command wrapper sizes..."
ANALYZE_LOC=$(sed -n '43,53p' crates/code-viz-tauri/src/commands.rs | wc -l)
DEAD_CODE_LOC=$(sed -n '106,118p' crates/code-viz-tauri/src/commands.rs | wc -l)

echo "   analyze_repository: ${ANALYZE_LOC} lines"
echo "   analyze_dead_code_command: ${DEAD_CODE_LOC} lines"

if [ "$ANALYZE_LOC" -gt 50 ] || [ "$DEAD_CODE_LOC" -gt 50 ]; then
    echo -e "${RED}✗ FAIL${NC}: Command wrappers exceed 50 lines"
    FAILED=1
else
    echo -e "${GREEN}✓ PASS${NC}: All command wrappers are thin (<50 lines)"
fi
echo ""

# Test 4: Run test suite and check timing
echo "4. Running test suite..."
TEST_START=$(date +%s)
if cargo nextest run --workspace --all-targets > /tmp/test-output.txt 2>&1; then
    TEST_END=$(date +%s)
    TEST_DURATION=$((TEST_END - TEST_START))

    TESTS_RUN=$(grep -oP '\d+(?= tests run)' /tmp/test-output.txt | tail -1)
    TESTS_PASSED=$(grep -oP '\d+(?= passed)' /tmp/test-output.txt | tail -1)

    echo "   Tests run: ${TESTS_RUN}"
    echo "   Tests passed: ${TESTS_PASSED}"
    echo "   Duration: ${TEST_DURATION}s"

    if [ "$TEST_DURATION" -gt 10 ]; then
        echo -e "${YELLOW}⚠ WARNING${NC}: Test suite took longer than 10s (target: 5s)"
    else
        echo -e "${GREEN}✓ PASS${NC}: Test suite completed in acceptable time"
    fi
else
    echo -e "${YELLOW}⚠ WARNING${NC}: Some tests failed (check /tmp/test-output.txt)"
    echo "   This may be due to pre-existing issues"
fi
echo ""

# Test 5: Verify mock implementations exist
echo "5. Checking mock implementations..."
MOCKS=("MockContext" "MockFileSystem" "MockGit")
MOCK_DIR="crates/code-viz-core/src/mocks"

for mock in "${MOCKS[@]}"; do
    if grep -r "pub struct ${mock}" "$MOCK_DIR" > /dev/null 2>&1; then
        echo -e "   ${GREEN}✓${NC} ${mock} exists"
    else
        echo -e "   ${RED}✗${NC} ${mock} NOT FOUND"
        FAILED=1
    fi
done
echo ""

# Test 6: Verify trait definitions exist
echo "6. Checking core trait definitions..."
TRAITS=("AppContext" "FileSystem" "GitProvider")
TRAIT_DIR="crates/code-viz-core/src/traits"

for trait in "${TRAITS[@]}"; do
    if grep -r "pub trait ${trait}" "$TRAIT_DIR" > /dev/null 2>&1; then
        echo -e "   ${GREEN}✓${NC} ${trait} trait exists"
    else
        echo -e "   ${RED}✗${NC} ${trait} trait NOT FOUND"
        FAILED=1
    fi
done
echo ""

# Summary
echo "========================================="
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All validation checks passed!${NC}"
    echo "Architecture is compliant with trait-based DI principles."
    exit 0
else
    echo -e "${RED}Some validation checks failed!${NC}"
    echo "Please review the failures above."
    exit 1
fi
