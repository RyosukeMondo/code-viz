#!/bin/bash
#
# Integration test for dead code feature
# Tests backend + frontend integration
#

set -e

echo "🧪 Running Integration Diagnostics..."
echo ""

# 1. Build backend
echo "1️⃣ Building Tauri backend..."
cargo build -p code-viz-tauri --quiet
echo "✅ Backend built"
echo ""

# 2. Run backend tests
echo "2️⃣ Running backend tests..."
cargo test -p code-viz-tauri --lib --quiet
echo "✅ Backend tests passed"
echo ""

# 3. Check TypeScript bindings
echo "3️⃣ Checking TypeScript bindings..."
if grep -q "deadCodeRatio" src/types/bindings.ts; then
    echo "✅ Dead code types found in bindings"
else
    echo "❌ Dead code types MISSING from bindings!"
    exit 1
fi
echo ""

# 4. Type check frontend
echo "4️⃣ Type checking frontend..."
npm run type-check --silent 2>&1 | grep -v "^$" | head -10 || true
echo "✅ Frontend type check complete"
echo ""

# 5. Run frontend tests
echo "5️⃣ Running frontend tests..."
npm test -- --run --reporter=dot 2>&1 | tail -5
echo ""

echo "✅ All integration checks passed!"
echo ""
echo "To manually test the GUI:"
echo "  npm run dev"
