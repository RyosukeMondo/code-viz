#!/usr/bin/env node
/**
 * Quick test script to check what analyze_repository returns
 * Run with: node test-analyze.js
 */

const path = require('path');
const { spawn } = require('child_process');

console.log('Testing analyze_repository command...\n');

// Build a minimal test
const testCode = `
use code_viz_core::{analyze, AnalysisConfig};
use code_viz_tauri::transform::flat_to_hierarchy;
use std::path::PathBuf;

fn main() {
    let repo_path = PathBuf::from("/home/rmondo/repos/code-viz");
    let config = AnalysisConfig::default();

    match analyze(&repo_path, &config) {
        Ok(result) => {
            println!("Files analyzed: {}", result.files.len());
            println!("Total LOC: {}", result.summary.total_loc);
            if let Some(first_file) = result.files.first() {
                println!("First file: {:?}", first_file.path);
                println!("  LOC: {}", first_file.loc);
                println!("  Language: {}", first_file.language);
                println!("  Size: {}", first_file.size_bytes);
            }

            let tree = flat_to_hierarchy(result.files);
            println!("\\nTree root:");
            println!("  Name: {}", tree.name);
            println!("  LOC: {}", tree.loc);
            println!("  Complexity: {}", tree.complexity);
            println!("  Children: {}", tree.children.len());
            if let Some(first_child) = tree.children.first() {
                println!("  First child: {} (LOC: {})", first_child.name, first_child.loc);
            }
        }
        Err(e) => {
            eprintln!("Analysis failed: {}", e);
            std::process::exit(1);
        }
    }
}
`;

// Write test file
const fs = require('fs');
const testDir = '/tmp/code-viz-test';
const testFile = `${testDir}/test_analyze.rs`;

if (!fs.existsSync(testDir)) {
    fs.mkdirSync(testDir, { recursive: true });
}

fs.writeFileSync(testFile, testCode);

// Create Cargo.toml
const cargoToml = `
[package]
name = "test-analyze"
version = "0.1.0"
edition = "2021"

[dependencies]
code-viz-core = { path = "/home/rmondo/repos/code-viz/crates/code-viz-core" }
code-viz-tauri = { path = "/home/rmondo/repos/code-viz/crates/code-viz-tauri" }
`;

fs.writeFileSync(`${testDir}/Cargo.toml`, cargoToml);
fs.mkdirSync(`${testDir}/src`, { recursive: true });
fs.renameSync(testFile, `${testDir}/src/main.rs`);

console.log('Building and running test...\n');

const cargo = spawn('cargo', ['run', '--quiet'], {
    cwd: testDir,
    stdio: 'inherit'
});

cargo.on('close', (code) => {
    // Cleanup
    fs.rmSync(testDir, { recursive: true, force: true });
    process.exit(code);
});
