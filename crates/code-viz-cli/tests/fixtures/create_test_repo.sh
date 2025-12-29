#!/bin/bash
# Create a deterministic test Git repository for E2E testing
set -e

REPO_DIR="$(dirname "$0")/test-repo"

# Clean up if exists
rm -rf "$REPO_DIR"
mkdir -p "$REPO_DIR"
cd "$REPO_DIR"

# Initialize Git with fixed identity
git init
git config user.name "Test User"
git config user.email "test@example.com"

# Create src directory
mkdir -p src

# Commit 1: Initial project structure (2024-01-01 00:00:00)
cat > src/main.rs << 'EOF'
fn main() {
    println!("Hello, world!");
}
EOF

cat > src/lib.rs << 'EOF'
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
EOF

cat > README.md << 'EOF'
# Test Project
A test repository for code-viz.
EOF

git add .
GIT_AUTHOR_DATE="2024-01-01T00:00:00" GIT_COMMITTER_DATE="2024-01-01T00:00:00" \
  git commit -m "Initial commit"

# Commit 2: Add duplicate code (2024-01-02 00:00:00)
cat > src/duplicate.rs << 'EOF'
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
EOF

git add src/duplicate.rs
GIT_AUTHOR_DATE="2024-01-02T00:00:00" GIT_COMMITTER_DATE="2024-01-02T00:00:00" \
  git commit -m "Add duplicate functions

🤖 Generated with Claude Code

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# Commit 3: Add complex function (2024-01-03 00:00:00)
cat > src/complex.rs << 'EOF'
pub fn complex_function(x: i32, y: i32) -> i32 {
    if x > 0 {
        if y > 0 {
            for i in 0..x {
                if i % 2 == 0 {
                    println!("{}", i);
                }
            }
        } else {
            while y < 0 {
                y += 1;
            }
        }
    }
    x + y
}
EOF

git add src/complex.rs
GIT_AUTHOR_DATE="2024-01-03T00:00:00" GIT_COMMITTER_DATE="2024-01-03T00:00:00" \
  git commit -m "Add complex function with high cognitive complexity"

# Commit 4: Modify main.rs - add comments (bloat) (2024-01-04 00:00:00)
cat > src/main.rs << 'EOF'
// This is the main entry point
// It prints a greeting message
// to the console output
fn main() {
    // Print hello world
    println!("Hello, world!");
}
EOF

git add src/main.rs
GIT_AUTHOR_DATE="2024-01-04T00:00:00" GIT_COMMITTER_DATE="2024-01-04T00:00:00" \
  git commit -m "Add documentation comments"

# Commit 5: Create coupling (2024-01-05 00:00:00)
cat > src/utils.rs << 'EOF'
use crate::lib::{add, multiply};

pub fn calculate(a: i32, b: i32) -> i32 {
    add(a, b) + multiply(a, b)
}
EOF

git add src/utils.rs
GIT_AUTHOR_DATE="2024-01-05T00:00:00" GIT_COMMITTER_DATE="2024-01-05T00:00:00" \
  git commit -m "Add utils with coupling to lib"

# Create feature branch
git checkout -b feature/new-feature

# Commit 6: Add feature (2024-01-06 00:00:00)
cat > src/feature.rs << 'EOF'
pub fn feature_function() -> String {
    "New feature".to_string()
}
EOF

git add src/feature.rs
GIT_AUTHOR_DATE="2024-01-06T00:00:00" GIT_COMMITTER_DATE="2024-01-06T00:00:00" \
  git commit -m "Add new feature"

# Return to main
git checkout main

echo "✅ Test repository created successfully at $REPO_DIR"
echo "Commits: $(git rev-list --count HEAD)"
echo "Files: $(find src -type f | wc -l)"
