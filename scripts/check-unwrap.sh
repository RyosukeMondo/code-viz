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

# First pass: find all unwrap/expect in Rust files (excluding obvious test files and mocks)
rg --type rust '\.(unwrap|expect)\(' crates/ \
  --glob '!**/*test*.rs' \
  --glob '!**/tests/**' \
  --glob '!**/build.rs' \
  --glob '!**/benches/**' \
  --glob '!**/mocks/**' \
  --glob '!**/mock_*.rs' \
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
  | grep -v '///' \
  | grep -v '^[^:]*:[^:]*:\s*/\*' \
  || true)

# Third pass: check each violation to see if it's in a test function
FINAL_VIOLATIONS=""
while IFS= read -r line; do
  # Extract file and line number
  FILE=$(echo "$line" | cut -d: -f1)
  LINE_NUM=$(echo "$line" | cut -d: -f2)

  # Check if this is a multi-line expect("BUG:...") - look at next line in file
  LINE_CONTENT=$(echo "$line" | cut -d: -f3-)
  if echo "$LINE_CONTENT" | grep -q "\.expect(\$"; then
    NEXT_LINE_NUM=$((LINE_NUM + 1))
    NEXT_LINE=$(sed -n "${NEXT_LINE_NUM}p" "$FILE")
    if echo "$NEXT_LINE" | grep -q "BUG:"; then
      continue
    fi
  fi

  # Check if this line is inside a #[test] function or #[cfg(test)] module
  # Strategy: Check if the file has a #[cfg(test)] module before this line
  # and no module closing brace between the #[cfg(test)] and our line

  # Get all content up to our line
  BEFORE_LINE=$(sed -n "1,${LINE_NUM}p" "$FILE")

  # Check for #[test] attribute nearby (within 30 lines before)
  RECENT_CONTEXT=$(echo "$BEFORE_LINE" | tail -30)
  if echo "$RECENT_CONTEXT" | grep -q '#\[test\]'; then
    continue
  fi

  # Check if there's a "Test-only unwrap" comment in the preceding lines (within 5 lines)
  PRECEDING_LINES=$(echo "$BEFORE_LINE" | tail -5)
  if echo "$PRECEDING_LINES" | grep -q "Test-only unwrap"; then
    continue
  fi

  # Check if we're in a #[cfg(test)] module
  # Find the last #[cfg(test)] before our line
  LAST_CFG_TEST_LINE=$(echo "$BEFORE_LINE" | grep -n '#\[cfg(test)\]' | tail -1 | cut -d: -f1)

  if [ -n "$LAST_CFG_TEST_LINE" ]; then
    # We found a #[cfg(test)] before this line
    # Now check if there's a module closure between #[cfg(test)] and our line
    # Count braces to see if we're still in the test module
    AFTER_CFG=$(sed -n "${LAST_CFG_TEST_LINE},${LINE_NUM}p" "$FILE")

    # Simple heuristic: if we see "mod tests" or similar after #[cfg(test)], we're in a test module
    if echo "$AFTER_CFG" | head -5 | grep -q 'mod \(tests\|test\)'; then
      continue
    fi
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
