#!/bin/bash
set -e

# Script to check for unwrap() and expect() calls in production Rust code
# Exits with error if any are found in non-test code

echo "🔍 Checking for unwrap() and expect() in production code..."

# Use ripgrep with multiline context to exclude test functions
# Strategy: Find all unwrap/expect, then filter out:
# 1. Test files (by path/name)
# 2. Lines with justification comments
# 3. Doc comments
# 4. Invalid query patterns (compile-time constants)

TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# First pass: find all unwrap/expect in Rust files (excluding obvious test files)
rg --type rust '\.(unwrap|expect)\(' crates/ \
  --glob '!**/*test*.rs' \
  --glob '!**/tests/**' \
  --glob '!**/build.rs' \
  --line-number \
  > "$TEMP_FILE" || true

# Second pass: filter out acceptable cases
VIOLATIONS=$(cat "$TEMP_FILE" \
  | grep -v '//.*Test-only' \
  | grep -v '//.*test-only' \
  | grep -v '//.*Safe:' \
  | grep -v '//.*safe:' \
  | grep -v '//.*Justif' \
  | grep -v '//.*justif' \
  | grep -v '//.*BUG:' \
  | grep -v '//.*controlled' \
  | grep -v '//.*fixture' \
  | grep -v '//.*programming error' \
  | grep -v '//.*not a runtime error' \
  | grep -v 'expect("Invalid.*query")' \
  | grep -v 'expect("BUG:' \
  | grep -v '^\s*///' \
  | grep -v '^\s*\*' \
  || true)

# Third pass: check each violation to see if it's in a test function
FINAL_VIOLATIONS=""
while IFS= read -r line; do
  # Extract file and line number
  FILE=$(echo "$line" | cut -d: -f1)
  LINE_NUM=$(echo "$line" | cut -d: -f2)

  # Check if this line is inside a #[test] function or #[cfg(test)] module
  # Look backwards from the line to find #[test] or #[cfg(test)]
  CONTEXT=$(sed -n "1,${LINE_NUM}p" "$FILE" | tail -20)

  # Skip if we find #[test] or #[cfg(test)] in recent context
  if echo "$CONTEXT" | grep -q '#\[test\]'; then
    continue
  fi

  if echo "$CONTEXT" | grep -q '#\[cfg(test)\]'; then
    continue
  fi

  # This is a genuine violation
  if [ -z "$FINAL_VIOLATIONS" ]; then
    FINAL_VIOLATIONS="$line"
  else
    FINAL_VIOLATIONS="$FINAL_VIOLATIONS"$'\n'"$line"
  fi
done <<< "$VIOLATIONS"

if [ -n "$FINAL_VIOLATIONS" ]; then
  echo ""
  echo "❌ Found unwrap() or expect() calls in production code:"
  echo ""
  echo "$FINAL_VIOLATIONS"
  echo ""
  echo "📚 See MIGRATION.md for guidance on proper error handling patterns."
  echo ""
  echo "Production code should use:"
  echo "  - Result types with ? operator"
  echo "  - .ok_or() or .ok_or_else() for Option types"
  echo "  - Graceful error handling with proper error types"
  echo ""
  echo "Test code can use unwrap() but should document why:"
  echo "  // Test-only unwrap: controlled fixture data"
  echo "  value.unwrap()"
  echo ""
  echo "expect() is allowed for programming errors with BUG: prefix:"
  echo "  .expect(\"BUG: Invalid query - programming error, not runtime error\")"
  echo ""

  exit 1
fi

echo "✅ No unwrap() or expect() found in production code"
exit 0
