#!/bin/bash
#
# Automated monitoring and testing script
# Runs tests and monitors for errors automatically
#

set -e

echo "🔍 Starting automated test monitoring..."

# Run unit tests
echo "📝 Running unit tests..."
npm test -- --run --reporter=verbose 2>&1 | tee /tmp/unit-test.log

# Check for failures
if grep -q "FAIL" /tmp/unit-test.log; then
    echo "❌ Unit tests failed!"
    grep -A 10 "FAIL" /tmp/unit-test.log
    exit 1
fi

echo "✅ All unit tests passed!"

# Type check
echo "🔧 Running type check..."
npm run type-check 2>&1 | tee /tmp/type-check.log

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ Type check failed!"
    exit 1
fi

echo "✅ Type check passed!"

# Lint
echo "📋 Running linter..."
npm run lint 2>&1 | tee /tmp/lint.log || true

echo "✅ Monitoring complete!"
echo ""
echo "Summary:"
echo "- Unit tests: PASSED"
echo "- Type check: PASSED"
echo "- Linter: CHECK"
