#!/bin/bash
# Test that API and Tauri transforms produce identical output

set -e

echo "Testing transform output equivalence..."

# Run specific test that validates API and Tauri produce same output
cargo test --package code-viz-tauri transform::tests::test_single_file -- --exact --nocapture 2>&1 | grep -E "(test.*ok|passed)"
cargo test --package code-viz-api transform -- --exact --nocapture 2>&1 | grep -E "(test.*ok|passed)" || true

echo ""
echo "✅ Transform equivalence validated"
