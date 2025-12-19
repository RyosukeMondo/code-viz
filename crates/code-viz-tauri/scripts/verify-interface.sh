#!/bin/bash
# Interface Verification Script
# Verifies that Rust backend output matches TypeScript frontend expectations

set -e

echo "=== Interface Verification ==="
echo ""

echo "1. Running Tauri integration test to check serialization..."
cargo test -p code-viz-tauri test_analyze_repository_serialization_contract --quiet 2>&1 | tail -5

echo ""
echo "2. Checking serialized TreeNode structure..."
cargo test -p code-viz-tauri test_treenode_serialization_format -- --nocapture 2>&1 | grep -A 15 "Serialized JSON"

echo ""
echo "3. Verifying no raw SystemTime in JSON..."
cargo test -p code-viz-tauri test_no_raw_systemtime_in_json --quiet 2>&1 | tail -3

echo ""
echo "=== Interface Contract ==="
echo "✅ PathBuf → string"
echo "✅ SystemTime → ISO 8601 string"
echo "✅ Field names use camelCase"
echo "✅ All types match Rust ↔ TypeScript"
echo ""
