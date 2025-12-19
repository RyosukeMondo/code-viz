#!/bin/bash

# CLI test utilities for code-viz-cli

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
CLI_BIN="$PROJECT_ROOT/target/debug/code-viz-cli"

# Build the CLI if not already built or if out of date
build_cli() {
    echo "Building CLI..."
    (cd "$PROJECT_ROOT" && cargo build --bin code-viz-cli)
    return $?
}

# Validate that a file contains valid JSON
validate_json() {
    local file=$1
    if [ ! -f "$file" ]; then
        echo "Error: File $file not found"
        return 1
    fi
    jq . "$file" > /dev/null 2>&1
    return $?
}

# Compare two JSON files, ignoring timestamps and other non-deterministic fields
compare_json_files() {
    local actual=$1
    local expected=$2
    
    if [ ! -f "$actual" ]; then
        echo "Error: Actual file $actual not found"
        return 1
    fi
    if [ ! -f "$expected" ]; then
        echo "Error: Expected file $expected not found"
        return 1
    fi

    # Normalize actual output by stripping timestamps before comparison
    local normalized_actual=$(mktemp)
    jq 'del(.timestamp, .files[].last_modified)' "$actual" > "$normalized_actual"
    
    diff -u "$expected" "$normalized_actual"
    local result=$?
    
    rm "$normalized_actual"
    return $result
}

# Run a CLI analysis test for a given fixture and compare with expected output
run_cli_test() {
    local fixture_path=$1
    local expected_json=$2
    local actual_json=$(mktemp)
    
    echo "Running analysis on $fixture_path..."
    "$CLI_BIN" analyze "$fixture_path" --format json > "$actual_json"
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo "Error: CLI failed with exit code $exit_code"
        rm "$actual_json"
        return $exit_code
    fi
    
    echo "Comparing output with $expected_json..."
    compare_json_files "$actual_json" "$expected_json"
    local result=$?
    
    if [ $result -eq 0 ]; then
        echo "Test PASSED"
    else
        echo "Test FAILED"
    fi
    
    rm "$actual_json"
    return $result
}
