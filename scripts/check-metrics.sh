#!/usr/bin/env bash

# Code Quality Metrics Checker
# This script checks code quality metrics on staged files
# Exit codes: 0 = pass, 1 = fail

set -e

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

echo "🔍 Running code quality checks on staged files..."
echo ""

# Get list of staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$STAGED_FILES" ]; then
    echo "${GREEN}✓${NC} No staged files to check"
    exit 0
fi

# Check 1: File sizes (fail if any .rs or .tsx file >500 lines excluding comments)
echo "📏 Checking file sizes (max 500 lines)..."

check_file_size() {
    local file=$1
    local max_lines=500

    # Count non-comment, non-blank lines
    local line_count
    if [[ $file == *.rs ]]; then
        # Rust: exclude // comments, /* */ comments, and blank lines
        line_count=$(grep -v -E '^\s*(//)' "$file" | grep -v -E '^\s*$' | wc -l)
    elif [[ $file == *.tsx ]] || [[ $file == *.ts ]]; then
        # TypeScript: exclude // comments and blank lines
        line_count=$(grep -v -E '^\s*(//)' "$file" | grep -v -E '^\s*$' | wc -l)
    else
        return 0
    fi

    if [ "$line_count" -gt "$max_lines" ]; then
        echo "${RED}✗${NC} $file: $line_count lines (max $max_lines)"
        echo "  → Split this file into smaller modules"
        ((ERRORS++))
    fi
}

for file in $STAGED_FILES; do
    if [[ -f "$file" ]] && [[ $file == *.rs ]] || [[ $file == *.tsx ]] || [[ $file == *.ts ]]; then
        check_file_size "$file"
    fi
done

# Check 2: Function sizes (warn if >50 lines, fail if >100)
echo "🔧 Checking function sizes (max 50 lines recommended, 100 hard limit)..."

check_function_size() {
    local file=$1

    if [[ $file == *.rs ]]; then
        # Simple heuristic: look for functions with excessive lines between fn and closing }
        # This is a basic check - may have false positives/negatives
        local in_function=0
        local function_start_line=0
        local function_name=""
        local brace_count=0
        local line_num=0

        while IFS= read -r line; do
            ((line_num++))

            # Skip comments and blank lines
            if [[ $line =~ ^[[:space:]]*// ]] || [[ -z "${line// }" ]]; then
                continue
            fi

            # Detect function start (simplified - may need improvement)
            if [[ $line =~ ^[[:space:]]*fn[[:space:]]+([a-zA-Z0-9_]+) ]] || \
               [[ $line =~ ^[[:space:]]*(pub[[:space:]]+)?fn[[:space:]]+([a-zA-Z0-9_]+) ]]; then
                if [ $in_function -eq 0 ]; then
                    in_function=1
                    function_start_line=$line_num
                    function_name="${BASH_REMATCH[1]}"
                    if [ -z "$function_name" ]; then
                        function_name="${BASH_REMATCH[2]}"
                    fi
                    brace_count=0
                fi
            fi

            # Count braces
            if [ $in_function -eq 1 ]; then
                local open_braces="${line//[^\{]/}"
                local close_braces="${line//[^\}]/}"
                ((brace_count += ${#open_braces} - ${#close_braces}))

                # Function ended
                if [ $brace_count -eq 0 ] && [[ $line =~ \} ]]; then
                    local function_lines=$((line_num - function_start_line + 1))

                    if [ "$function_lines" -gt 100 ]; then
                        echo "${RED}✗${NC} $file:$function_start_line: function '$function_name' is $function_lines lines (max 100)"
                        echo "  → Break this function into smaller helpers"
                        ((ERRORS++))
                    elif [ "$function_lines" -gt 50 ]; then
                        echo "${YELLOW}⚠${NC} $file:$function_start_line: function '$function_name' is $function_lines lines (recommended max 50)"
                        ((WARNINGS++))
                    fi

                    in_function=0
                fi
            fi
        done < "$file"
    fi
}

for file in $STAGED_FILES; do
    if [[ -f "$file" ]] && [[ $file == *.rs ]]; then
        check_function_size "$file"
    fi
done

# Check 3: unwrap() in staged files (fail if found in production code, allow in tests)
echo "🚫 Checking for unwrap() in production code..."

for file in $STAGED_FILES; do
    if [[ -f "$file" ]] && [[ $file == *.rs ]]; then
        # Skip test files
        if [[ $file == *test*.rs ]] || [[ $file == */tests/* ]]; then
            continue
        fi

        # Check for unwrap() or expect()
        if grep -n -E '\.unwrap\(\)|\.expect\(' "$file" > /dev/null; then
            # Check if it's in a #[cfg(test)] block
            local has_unwrap=0
            local in_test_module=0
            local line_num=0

            while IFS= read -r line; do
                ((line_num++))

                # Check for test module start
                if [[ $line =~ \#\[cfg\(test\)\] ]]; then
                    in_test_module=1
                fi

                # Check for module end (simplified)
                if [[ $line =~ ^[[:space:]]*\}[[:space:]]*$ ]] && [ $in_test_module -eq 1 ]; then
                    in_test_module=0
                fi

                # Check for unwrap/expect outside test modules
                if [ $in_test_module -eq 0 ] && [[ $line =~ \.unwrap\(\)|\.expect\( ]]; then
                    # Check for "Test-only unwrap:" comment
                    if ! [[ $line =~ Test-only\ unwrap: ]]; then
                        echo "${RED}✗${NC} $file:$line_num: found unwrap()/expect() in production code"
                        echo "  → Use proper error handling with Result and ?"
                        echo "  → See MIGRATION.md for patterns"
                        has_unwrap=1
                    fi
                fi
            done < "$file"

            if [ $has_unwrap -eq 1 ]; then
                ((ERRORS++))
            fi
        fi
    fi
done

# Check 4: TypeScript any usage (warn if found, provide guidance)
echo "🔎 Checking for TypeScript 'any' usage..."

for file in $STAGED_FILES; do
    if [[ -f "$file" ]] && [[ $file == *.ts ]] || [[ $file == *.tsx ]]; then
        # Skip test files for any check (more lenient)
        if [[ $file == *.test.ts* ]] || [[ $file == */tests/* ]] || [[ $file == */__tests__/* ]]; then
            continue
        fi

        # Look for ': any' or 'as any'
        if grep -n -E ':\s*any\b|as\s+any\b' "$file" > /dev/null; then
            local any_count=$(grep -c -E ':\s*any\b|as\s+any\b' "$file" || true)
            echo "${YELLOW}⚠${NC} $file: found $any_count usage(s) of 'any'"
            echo "  → Consider using specific types or unknown/Record<string, unknown>"
            ((WARNINGS++))
        fi
    fi
done

echo ""
echo "========================================="

# Summary
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo "${GREEN}✓ All checks passed!${NC}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo "${YELLOW}⚠ $WARNINGS warning(s) found${NC}"
    echo "Warnings do not block commits, but please address them"
    exit 0
else
    echo "${RED}✗ $ERRORS error(s) and $WARNINGS warning(s) found${NC}"
    echo ""
    echo "Fix the errors above before committing."
    echo "You can bypass with: git commit --no-verify (not recommended)"
    exit 1
fi
