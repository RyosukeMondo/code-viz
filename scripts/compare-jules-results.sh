#!/bin/bash
# Compare 3 Jules implementations and select the best one

OUTPUT_DIR="/tmp/jules-results"
COMPARISON_REPORT="$OUTPUT_DIR/comparison-report.md"

SESSION_1="2659273988513172518"
SESSION_2="12515482228610819834"
SESSION_3="12739335481447685542"

echo "Comparing Jules implementations..."
echo ""

# Function to analyze a patch
analyze_patch() {
    local session_id=$1
    local patch_file="$OUTPUT_DIR/session-${session_id}.patch"

    if [ ! -f "$patch_file" ]; then
        echo "ERROR: Patch file not found: $patch_file"
        return 1
    fi

    echo "## Session #${session_id}" >> "$COMPARISON_REPORT"
    echo "" >> "$COMPARISON_REPORT"

    # Extract statistics
    local files_changed=$(grep -c "^diff --git" "$patch_file" 2>/dev/null || echo 0)
    local lines_added=$(grep -c "^+" "$patch_file" 2>/dev/null || echo 0)
    local lines_removed=$(grep -c "^-" "$patch_file" 2>/dev/null || echo 0)
    local tests_added=$(grep -c "#\[test\]" "$patch_file" 2>/dev/null || echo 0)
    local trait_usage=$(grep -c "impl.*trait" "$patch_file" 2>/dev/null || echo 0)

    # Check for architectural compliance
    local has_direct_io=$(grep -c "std::fs::" "$patch_file" 2>/dev/null || echo 0)
    local has_println=$(grep -c "println!" "$patch_file" 2>/dev/null || echo 0)
    local uses_mocks=$(grep -c "MockFileSystem\|MockGit\|MockContext" "$patch_file" 2>/dev/null || echo 0)

    # Calculate score
    local score=0

    # Positive points
    ((score += tests_added * 10))         # Tests are critical
    ((score += trait_usage * 5))          # Trait usage is good
    ((score += uses_mocks * 5))           # Mock usage is good

    # Negative points
    ((score -= has_direct_io * 50))       # Direct I/O is BAD
    ((score -= has_println * 10))         # println! is bad in production

    # Prefer smaller implementations (less bloat)
    if [ $lines_added -lt 200 ]; then
        ((score += 10))
    fi

    echo "### Statistics" >> "$COMPARISON_REPORT"
    echo "- Files changed: $files_changed" >> "$COMPARISON_REPORT"
    echo "- Lines added: $lines_added" >> "$COMPARISON_REPORT"
    echo "- Lines removed: $lines_removed" >> "$COMPARISON_REPORT"
    echo "- Tests added: $tests_added" >> "$COMPARISON_REPORT"
    echo "" >> "$COMPARISON_REPORT"

    echo "### Architecture Compliance" >> "$COMPARISON_REPORT"
    if [ $has_direct_io -gt 0 ]; then
        echo "- ❌ **VIOLATION**: Direct I/O detected ($has_direct_io occurrences)" >> "$COMPARISON_REPORT"
    else
        echo "- ✅ No direct I/O (follows trait-based DI)" >> "$COMPARISON_REPORT"
    fi

    if [ $has_println -gt 0 ]; then
        echo "- ⚠️  println! usage detected ($has_println occurrences)" >> "$COMPARISON_REPORT"
    else
        echo "- ✅ No println! in production code" >> "$COMPARISON_REPORT"
    fi

    if [ $uses_mocks -gt 0 ]; then
        echo "- ✅ Uses mocks in tests ($uses_mocks occurrences)" >> "$COMPARISON_REPORT"
    else
        echo "- ❌ No mock usage detected" >> "$COMPARISON_REPORT"
    fi

    if [ $trait_usage -gt 0 ]; then
        echo "- ✅ Implements traits ($trait_usage implementations)" >> "$COMPARISON_REPORT"
    fi
    echo "" >> "$COMPARISON_REPORT"

    echo "### Quality Score: $score" >> "$COMPARISON_REPORT"
    echo "" >> "$COMPARISON_REPORT"
    echo "---" >> "$COMPARISON_REPORT"
    echo "" >> "$COMPARISON_REPORT"

    # Return score for comparison
    echo "$score"
}

# Initialize report
cat > "$COMPARISON_REPORT" << EOF
# Jules Implementation Comparison Report

**Task**: Add code churn rate metric calculation

**Generated**: $(date)

**Sessions compared**:
- Session #1: $SESSION_1
- Session #2: $SESSION_2
- Session #3: $SESSION_3

---

EOF

# Analyze all patches
echo "Analyzing Session #1..."
score_1=$(analyze_patch "$SESSION_1")

echo "Analyzing Session #2..."
score_2=$(analyze_patch "$SESSION_2")

echo "Analyzing Session #3..."
score_3=$(analyze_patch "$SESSION_3")

# Determine winner
echo "## Winner Selection" >> "$COMPARISON_REPORT"
echo "" >> "$COMPARISON_REPORT"

if [ $score_1 -ge $score_2 ] && [ $score_1 -ge $score_3 ]; then
    winner="$SESSION_1"
    winner_score=$score_1
    winner_num="1"
elif [ $score_2 -ge $score_1 ] && [ $score_2 -ge $score_3 ]; then
    winner="$SESSION_2"
    winner_score=$score_2
    winner_num="2"
else
    winner="$SESSION_3"
    winner_score=$score_3
    winner_num="3"
fi

echo "**Winner: Session #${winner_num}** (Score: $winner_score)" >> "$COMPARISON_REPORT"
echo "" >> "$COMPARISON_REPORT"
echo "Session scores:" >> "$COMPARISON_REPORT"
echo "- Session #1: $score_1" >> "$COMPARISON_REPORT"
echo "- Session #2: $score_2" >> "$COMPARISON_REPORT"
echo "- Session #3: $score_3" >> "$COMPARISON_REPORT"
echo "" >> "$COMPARISON_REPORT"

echo "## Next Steps" >> "$COMPARISON_REPORT"
echo "" >> "$COMPARISON_REPORT"
echo "To apply the winning implementation:" >> "$COMPARISON_REPORT"
echo "\`\`\`bash" >> "$COMPARISON_REPORT"
echo "jules remote pull --session $winner --apply" >> "$COMPARISON_REPORT"
echo "\`\`\`" >> "$COMPARISON_REPORT"
echo "" >> "$COMPARISON_REPORT"
echo "Or teleport to the session:" >> "$COMPARISON_REPORT"
echo "\`\`\`bash" >> "$COMPARISON_REPORT"
echo "jules teleport $winner" >> "$COMPARISON_REPORT"
echo "\`\`\`" >> "$COMPARISON_REPORT"

# Display report
echo ""
echo "========================================"
cat "$COMPARISON_REPORT"
echo "========================================"
echo ""
echo "Full report saved to: $COMPARISON_REPORT"
echo ""
echo "Winner: Session #${winner_num} (ID: $winner)"
echo ""
echo "To apply winning implementation:"
echo "  jules remote pull --session $winner --apply"
