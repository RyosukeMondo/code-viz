#!/bin/bash

# CLI Integration Tests for code-viz-cli

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Define paths relative to project root
FIXTURES_DIR="crates/code-viz-cli/tests/fixtures"
EXPECTED_DIR="crates/code-viz-cli/tests/expected"
HELPERS_DIR="crates/code-viz-cli/tests/helpers"

# Source utilities (source from project root context)
source "$PROJECT_ROOT/$HELPERS_DIR/test_utils.sh"

# Test results
PASSED=0
FAILED=0

# Helper to run a test and track results
run_test() {
    local test_name=$1
    echo "------------------------------------------------"
    echo "Running test: $test_name"
    
    if $test_name; then
        echo "Test $test_name: PASSED"
        PASSED=$((PASSED + 1))
    else
        echo "Test $test_name: FAILED"
        FAILED=$((FAILED + 1))
    fi
}

# --- Test Cases ---

test_simple_analysis() {
    run_cli_test "$FIXTURES_DIR/simple-repo" "$EXPECTED_DIR/simple-repo.json"
}

test_empty_repository() {
    run_cli_test "$FIXTURES_DIR/empty-repo" "$EXPECTED_DIR/empty-repo.json"
}

test_invalid_path() {
    echo "Running analysis on non-existent path..."
    "$CLI_BIN" analyze "/non/existent/path" > /dev/null 2>&1
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo "Successfully failed with exit code $exit_code"
        return 0
    else
        echo "Error: CLI should have failed on non-existent path"
        return 1
    fi
}

test_help_command() {
    echo "Running help command..."
    "$CLI_BIN" --help > /dev/null 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        echo "Successfully displayed help"
        return 0
    else
        echo "Error: CLI failed to display help"
        return 1
    fi
}

# --- Main Runner ---

# Build CLI first
build_cli || exit 1

# Change to project root to ensure relative paths work as expected in analysis output
cd "$PROJECT_ROOT"

# Run tests
run_test test_simple_analysis
run_test test_empty_repository
run_test test_invalid_path
run_test test_help_command

# Summary
echo "================================================"
echo "Test Summary:"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Total:  $((PASSED + FAILED))"
echo "================================================"

if [ $FAILED -gt 0 ]; then
    exit 1
else
    exit 0
fi