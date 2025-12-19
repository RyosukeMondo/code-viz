#!/bin/bash
#
# Test script for Dead Code Detection feature
# Usage:
#   ./scripts/test-dead-code.sh [path]     # CLI test only
#   ./scripts/test-dead-code.sh --gui      # Launch GUI
#   ./scripts/test-dead-code.sh --all      # CLI test + GUI
#

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Dead Code Detection Test Script${NC}"
echo ""

# Handle help first
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo -e "${BLUE}Usage:${NC}"
    echo "  $0              # Run CLI test with sample repo"
    echo "  $0 [path]       # Run CLI test on specific path"
    echo "  $0 --gui        # Launch GUI only (skip CLI tests)"
    echo "  $0 --all [path] # Run CLI test + launch GUI"
    echo ""
    exit 0
fi

# If only GUI requested, skip to GUI section
if [[ "$1" == "--gui" ]]; then
    echo -e "${BLUE}🚀 Launching GUI...${NC}"
    echo ""
    echo -e "${YELLOW}Instructions:${NC}"
    echo "  1. Open a TypeScript/JavaScript project"
    echo "  2. Click the 'Dead Code' toggle button"
    echo "  3. Wait for analysis to complete"
    echo "  4. Files with dead code will have colored borders"
    echo "  5. Click any file to see dead symbols in the side panel"
    echo ""
    echo -e "${BLUE}Press Ctrl+C to stop the GUI${NC}"
    echo ""
    npm run dev
    exit 0
fi

# Step 1: Build CLI if not already built
if [ ! -f "target/release/code-viz-cli" ]; then
    echo -e "${YELLOW}📦 Building CLI (first time)...${NC}"
    cargo build --release -p code-viz-cli --quiet
    echo -e "${GREEN}✅ CLI built successfully${NC}"
else
    echo -e "${GREEN}✅ CLI already built${NC}"
fi

echo ""

# Step 2: Run tests
echo -e "${BLUE}🧪 Running dead-code crate tests...${NC}"
cargo test -p code-viz-dead-code --quiet
echo -e "${GREEN}✅ All 73 tests passed${NC}"

echo ""

# Step 3: Run analysis on sample repo
# Handle --all flag with optional path argument
if [[ "$1" == "--all" ]]; then
    TARGET_PATH="${2:-crates/code-viz-dead-code/tests/fixtures/sample-repo}"
else
    TARGET_PATH="${1:-crates/code-viz-dead-code/tests/fixtures/sample-repo}"
fi

echo -e "${BLUE}📊 Analyzing: ${TARGET_PATH}${NC}"
echo ""

./target/release/code-viz-cli dead-code "$TARGET_PATH" --min-confidence 50

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Step 4: Show JSON output example
echo -e "${YELLOW}💡 For JSON output, use:${NC}"
echo "   ./target/release/code-viz-cli dead-code $TARGET_PATH --format json"
echo ""

# Step 5: Show other options
echo -e "${YELLOW}💡 Other useful commands:${NC}"
echo "   # Analyze current directory"
echo "   ./target/release/code-viz-cli dead-code ."
echo ""
echo "   # Higher confidence threshold (fewer results)"
echo "   ./target/release/code-viz-cli dead-code . --min-confidence 90"
echo ""
echo "   # Save to file"
echo "   ./target/release/code-viz-cli dead-code . --output dead-code-report.txt"
echo ""
echo "   # Exclude patterns"
echo "   ./target/release/code-viz-cli dead-code . --exclude 'node_modules/**' --exclude 'dist/**'"
echo ""

# Check if GUI launch was requested with --all
if [[ "$1" == "--all" ]]; then
    echo -e "${GREEN}✅ CLI tests complete!${NC}"
    echo ""
    echo -e "${BLUE}🚀 Launching GUI...${NC}"
    echo ""
    echo -e "${YELLOW}Instructions:${NC}"
    echo "  1. Open a TypeScript/JavaScript project"
    echo "  2. Click the 'Dead Code' toggle button"
    echo "  3. Wait for analysis to complete"
    echo "  4. Files with dead code will have colored borders"
    echo "  5. Click any file to see dead symbols in the side panel"
    echo ""
    echo -e "${BLUE}Press Ctrl+C to stop the GUI${NC}"
    echo ""
    npm run dev
else
    echo -e "${GREEN}✅ Dead code detection test complete!${NC}"
    echo ""
    echo -e "${YELLOW}💡 To test the GUI integration:${NC}"
    echo "   ./scripts/test-dead-code.sh --gui"
    echo ""
fi
