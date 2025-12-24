#!/bin/bash
# Monitor Jules sessions and pull results when complete

SESSION_1="2659273988513172518"
SESSION_2="12515482228610819834"
SESSION_3="12739335481447685542"

OUTPUT_DIR="/tmp/jules-results"
mkdir -p "$OUTPUT_DIR"

echo "Monitoring 3 Jules sessions..."
echo "Session #1: $SESSION_1"
echo "Session #2: $SESSION_2"
echo "Session #3: $SESSION_3"
echo ""

check_session_status() {
    local session_id=$1
    jules remote list --session | grep "$session_id" | awk '{print $NF}'
}

pull_session() {
    local session_id=$1
    local output_file="$OUTPUT_DIR/session-${session_id}.patch"
    echo "Pulling session $session_id..."
    jules remote pull --session "$session_id" > "$output_file" 2>&1
    echo "Saved to: $output_file"
}

# Monitor until all sessions are complete
while true; do
    status_1=$(check_session_status "$SESSION_1")
    status_2=$(check_session_status "$SESSION_2")
    status_3=$(check_session_status "$SESSION_3")

    echo "[$(date '+%H:%M:%S')] Status - Session #1: $status_1 | Session #2: $status_2 | Session #3: $status_3"

    # Check if all completed or failed
    completed=0
    for status in "$status_1" "$status_2" "$status_3"; do
        if [[ "$status" == "Completed" ]] || [[ "$status" == "Failed" ]] || [[ "$status" == "Awaiting User Feedback" ]]; then
            ((completed++))
        fi
    done

    if [ $completed -eq 3 ]; then
        echo ""
        echo "All sessions finished!"
        echo ""
        break
    fi

    sleep 30  # Check every 30 seconds
done

# Pull results from all sessions
echo "Pulling results..."
echo ""

pull_session "$SESSION_1"
pull_session "$SESSION_2"
pull_session "$SESSION_3"

echo ""
echo "All results pulled to: $OUTPUT_DIR"
echo ""
echo "Next step: Run comparison script to select best implementation"
echo "  ./scripts/compare-jules-results.sh"
