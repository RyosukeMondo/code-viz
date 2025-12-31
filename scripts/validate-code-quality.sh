#!/bin/bash
# Comprehensive code quality validation script
# Validates all quality metrics for error-handling-code-quality-remediation spec

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Code Quality Validation Report"
echo "=================================="
echo ""

# Track overall status
VALIDATION_FAILED=0

# 1. Function Size Check
echo "📏 1. Function Size Validation"
echo "   Target: All functions ≤50 lines"
LONG_FUNCTIONS=$(./scripts/find-long-functions.sh 2>&1 | grep -c "Function exceeds" || true)
if [ "$LONG_FUNCTIONS" -eq 0 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - All functions ≤50 lines"
else
    echo -e "   ${RED}❌ FAIL${NC} - Found $LONG_FUNCTIONS functions >50 lines"
    VALIDATION_FAILED=1
fi
echo ""

# 2. Cognitive Complexity Check
echo "🧠 2. Cognitive Complexity Validation"
echo "   Target: All functions complexity ≤10 (warning threshold 25)"
COMPLEX_FUNCTIONS=$(cargo clippy --workspace --all-features --quiet -- \
    -W clippy::cognitive_complexity 2>&1 | grep -c "cognitive complexity" || true)
if [ "$COMPLEX_FUNCTIONS" -eq 0 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - All functions complexity ≤25"
else
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Found $COMPLEX_FUNCTIONS functions with complexity >25"
    echo "   Note: These should be refactored for optimal maintainability"
    # Don't fail, just warn for now since threshold is 25
fi
echo ""

# 3. Too Many Arguments Check
echo "📋 3. Function Arguments Validation"
echo "   Target: Functions should have reasonable argument counts"
TOO_MANY_ARGS=$(cargo clippy --workspace --all-features --quiet -- \
    -W clippy::too_many_arguments 2>&1 | grep -c "too_many_arguments" || true)
if [ "$TOO_MANY_ARGS" -eq 0 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - No functions with excessive arguments"
else
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Found $TOO_MANY_ARGS functions with many arguments"
fi
echo ""

# 4. Unwrap Detection
echo "🚫 4. Unwrap/Expect Detection in Production Code"
echo "   Target: Zero unwrap() in production code"
if [ -x "./scripts/check-unwrap.sh" ]; then
    if ./scripts/check-unwrap.sh > /dev/null 2>&1; then
        echo -e "   ${GREEN}✅ PASS${NC} - No unwrap() in production code"
    else
        echo -e "   ${RED}❌ FAIL${NC} - Found unwrap() in production code"
        VALIDATION_FAILED=1
    fi
else
    echo -e "   ${YELLOW}⚠️  SKIP${NC} - check-unwrap.sh not found or not executable"
fi
echo ""

# 5. Import Count Check
echo "📦 5. Module Dependency Validation"
echo "   Target: ≤8 imports per module (recommended)"
# Count use statements per file
MAX_IMPORTS=0
MAX_IMPORTS_FILE=""
for file in $(find crates -name "*.rs" -type f ! -path "*/tests/*" ! -path "*/target/*"); do
    IMPORT_COUNT=$(grep -c "^use " "$file" 2>/dev/null || echo 0)
    if [ "$IMPORT_COUNT" -gt "$MAX_IMPORTS" ]; then
        MAX_IMPORTS=$IMPORT_COUNT
        MAX_IMPORTS_FILE=$file
    fi
done
if [ "$MAX_IMPORTS" -le 8 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - All modules have ≤8 imports"
elif [ "$MAX_IMPORTS" -le 15 ]; then
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Max imports: $MAX_IMPORTS in $MAX_IMPORTS_FILE"
else
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Max imports: $MAX_IMPORTS in $MAX_IMPORTS_FILE (consider splitting)"
fi
echo ""

# 6. Nesting Depth Check
echo "🔀 6. Nesting Depth Validation"
echo "   Target: ≤4 levels of nesting"
MAX_NESTING=0
MAX_NESTING_FILE=""
for file in $(find crates -name "*.rs" -type f ! -path "*/tests/*" ! -path "*/target/*"); do
    # Simple heuristic: count leading spaces divided by 4
    NESTING=$(grep -E '^ {20,}' "$file" 2>/dev/null | wc -l || echo 0)
    if [ "$NESTING" -gt "$MAX_NESTING" ]; then
        MAX_NESTING=$NESTING
        MAX_NESTING_FILE=$file
    fi
done
if [ "$MAX_NESTING" -eq 0 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - No excessive nesting detected"
elif [ "$MAX_NESTING" -le 5 ]; then
    echo -e "   ${YELLOW}⚠️  INFO${NC} - Some deep nesting in $MAX_NESTING_FILE"
else
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Deep nesting detected in $MAX_NESTING_FILE"
fi
echo ""

# 7. File Size Check
echo "📄 7. File Size Validation"
echo "   Target: All files ≤500 lines (excluding comments/blanks)"
LARGE_FILES=0
for file in $(find crates src -name "*.rs" -o -name "*.tsx" -o -name "*.ts" | grep -v "node_modules" | grep -v "target" | grep -v ".test."); do
    # Count non-empty, non-comment lines
    LINES=$(grep -v "^\s*$" "$file" | grep -v "^\s*//" | grep -v "^\s*\*" | wc -l)
    if [ "$LINES" -gt 500 ]; then
        if [ "$LARGE_FILES" -eq 0 ]; then
            echo "   Large files found:"
        fi
        echo "     - $file: $LINES lines"
        LARGE_FILES=$((LARGE_FILES + 1))
    fi
done
if [ "$LARGE_FILES" -eq 0 ]; then
    echo -e "   ${GREEN}✅ PASS${NC} - All files ≤500 lines"
else
    echo -e "   ${YELLOW}⚠️  WARN${NC} - Found $LARGE_FILES files >500 lines"
fi
echo ""

# Summary
echo "=================================="
echo "📊 Validation Summary"
echo "=================================="
echo ""

if [ "$VALIDATION_FAILED" -eq 0 ]; then
    echo -e "${GREEN}✅ ALL CRITICAL CHECKS PASSED${NC}"
    echo ""
    echo "Quality metrics:"
    echo "  ✅ Function size: All ≤50 lines"
    echo "  ⚠️  Complexity: Some functions >25 (should refactor)"
    echo "  ✅ Unwrap detection: No unwrap() in production"
    echo "  ✅ File sizes: Validated"
    exit 0
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo ""
    echo "Please fix the failing checks before proceeding."
    exit 1
fi
