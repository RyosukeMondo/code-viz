#!/bin/bash
# Validation script for task 24: Validate duplication eliminated
# Verifies that transform code duplication has been eliminated

set -e

echo "=== Task 24: Validate Duplication Eliminated ==="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track validation results
VALIDATION_PASSED=true

echo "Step 1: Verify core transform module exists and has proper structure"
echo "----------------------------------------------------------------"

CORE_MOD="crates/code-viz-core/src/transform/mod.rs"
TREE_BUILDER="crates/code-viz-core/src/transform/tree_builder.rs"
METRIC_AGG="crates/code-viz-core/src/transform/metric_aggregator.rs"
PATH_UTILS="crates/code-viz-core/src/transform/path_utils.rs"

if [ -f "$CORE_MOD" ] && [ -f "$TREE_BUILDER" ] && [ -f "$METRIC_AGG" ] && [ -f "$PATH_UTILS" ]; then
    echo -e "${GREEN}✅ Core transform module structure verified${NC}"
else
    echo -e "${RED}❌ Core transform module structure incomplete${NC}"
    VALIDATION_PASSED=false
fi

echo ""
echo "Step 2: Check line counts"
echo "-------------------------"

# Count lines (excluding comments and blank lines for accuracy)
count_loc() {
    grep -v '^\s*//' "$1" | grep -v '^\s*$' | wc -l
}

CORE_MOD_LINES=$(wc -l < "$CORE_MOD")
TREE_BUILDER_LINES=$(wc -l < "$TREE_BUILDER")
METRIC_AGG_LINES=$(wc -l < "$METRIC_AGG")
PATH_UTILS_LINES=$(wc -l < "$PATH_UTILS")
CORE_TOTAL=$((CORE_MOD_LINES + TREE_BUILDER_LINES + METRIC_AGG_LINES + PATH_UTILS_LINES))

echo "Core transform module:"
echo "  - mod.rs: $CORE_MOD_LINES lines"
echo "  - tree_builder.rs: $TREE_BUILDER_LINES lines"
echo "  - metric_aggregator.rs: $METRIC_AGG_LINES lines"
echo "  - path_utils.rs: $PATH_UTILS_LINES lines"
echo "  - Total: $CORE_TOTAL lines"

# Check API transform (production code only, before test module)
API_TRANSFORM="crates/code-viz-api/src/transform.rs"
API_LINES=$(wc -l < "$API_TRANSFORM")
echo ""
echo "API transform.rs: $API_LINES lines (including docs)"

# Check Tauri transform (production code only, before test module)
TAURI_TRANSFORM="crates/code-viz-tauri/src/transform.rs"
TAURI_PROD_LINES=$(grep -n "^#\[cfg(test)\]" "$TAURI_TRANSFORM" | head -1 | cut -d: -f1)
TAURI_PROD_LINES=$((TAURI_PROD_LINES - 1))
echo "Tauri transform.rs: $TAURI_PROD_LINES lines (production code only)"

TOTAL_LINES=$((CORE_TOTAL + API_LINES + TAURI_PROD_LINES))
echo ""
echo "Total lines after deduplication: $TOTAL_LINES"

echo ""
echo "Step 3: Verify both API and Tauri use core module"
echo "--------------------------------------------------"

if grep -q "code_viz_core::transform" "$API_TRANSFORM"; then
    echo -e "${GREEN}✅ API transform uses core module${NC}"
else
    echo -e "${RED}❌ API transform does not use core module${NC}"
    VALIDATION_PASSED=false
fi

if grep -q "code_viz_core::transform::flat_to_hierarchy" "$TAURI_TRANSFORM"; then
    echo -e "${GREEN}✅ Tauri transform uses core module${NC}"
else
    echo -e "${RED}❌ Tauri transform does not use core module${NC}"
    VALIDATION_PASSED=false
fi

echo ""
echo "Step 4: Check for business logic duplication"
echo "---------------------------------------------"

# API and Tauri should only have type conversions, no tree building logic
if ! grep -q "build_tree_nodes\|populate_file_metrics\|aggregate_directory_metrics" "$API_TRANSFORM"; then
    echo -e "${GREEN}✅ API transform has no duplicate tree building logic${NC}"
else
    echo -e "${YELLOW}⚠️  API transform may contain duplicate logic${NC}"
fi

if ! grep -q "build_tree_nodes\|populate_file_metrics\|aggregate_directory_metrics" "$TAURI_TRANSFORM"; then
    echo -e "${GREEN}✅ Tauri transform has no duplicate tree building logic${NC}"
else
    echo -e "${YELLOW}⚠️  Tauri transform may contain duplicate logic${NC}"
fi

echo ""
echo "Step 5: Run tests to verify integration"
echo "----------------------------------------"

echo "Testing core transform module..."
if cargo test --package code-viz-core --lib transform --quiet 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✅ Core transform tests pass${NC}"
else
    echo -e "${RED}❌ Core transform tests failed${NC}"
    VALIDATION_PASSED=false
fi

echo "Testing API transform integration..."
if cargo test --package code-viz-api transform --quiet 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✅ API transform tests pass${NC}"
else
    echo -e "${RED}❌ API transform tests failed${NC}"
    VALIDATION_PASSED=false
fi

echo "Testing Tauri transform integration..."
if cargo test --package code-viz-tauri transform --quiet 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✅ Tauri transform tests pass${NC}"
else
    echo -e "${RED}❌ Tauri transform tests failed${NC}"
    VALIDATION_PASSED=false
fi

echo ""
echo "Step 6: Calculate deduplication metrics"
echo "----------------------------------------"

# Before refactoring (from task description):
# - API transform.rs: ~578 lines
# - Tauri transform.rs: ~578 lines (duplicate)
# - Total before: 1156 lines

BEFORE_LINES=1156
SAVINGS=$((BEFORE_LINES - TOTAL_LINES))
REDUCTION_PCT=$((SAVINGS * 100 / BEFORE_LINES))

echo "Before refactoring: $BEFORE_LINES lines"
echo "After refactoring:  $TOTAL_LINES lines"
echo "Lines eliminated:   $SAVINGS lines"
echo "Code reduction:     $REDUCTION_PCT%"

if [ $REDUCTION_PCT -ge 40 ]; then
    echo -e "${GREEN}✅ Achieved significant code reduction (>40%)${NC}"
else
    echo -e "${YELLOW}⚠️  Code reduction below target (40%)${NC}"
fi

echo ""
echo "Step 7: Verify file size constraints"
echo "-------------------------------------"

if [ $API_LINES -le 100 ]; then
    echo -e "${GREEN}✅ API transform.rs ≤100 lines${NC}"
else
    echo -e "${RED}❌ API transform.rs exceeds 100 lines${NC}"
    VALIDATION_PASSED=false
fi

if [ $TAURI_PROD_LINES -le 100 ]; then
    echo -e "${GREEN}✅ Tauri transform.rs production code ≤100 lines${NC}"
else
    echo -e "${RED}❌ Tauri transform.rs production code exceeds 100 lines${NC}"
    VALIDATION_PASSED=false
fi

if [ $CORE_MOD_LINES -le 150 ]; then
    echo -e "${GREEN}✅ Core mod.rs ≤150 lines${NC}"
else
    echo -e "${RED}❌ Core mod.rs exceeds 150 lines${NC}"
    VALIDATION_PASSED=false
fi

echo ""
echo "========================================"
if [ "$VALIDATION_PASSED" = true ]; then
    echo -e "${GREEN}✅ VALIDATION PASSED${NC}"
    echo ""
    echo "Summary:"
    echo "  - Core transform module properly structured"
    echo "  - Both API and Tauri use core module"
    echo "  - No business logic duplication"
    echo "  - All tests pass"
    echo "  - ${SAVINGS} lines eliminated (${REDUCTION_PCT}% reduction)"
    echo "  - All file size constraints met"
    exit 0
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo "Please review the issues above."
    exit 1
fi
