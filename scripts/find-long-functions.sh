#!/bin/bash
# Find functions exceeding 50 lines in Rust code

for file in $(find crates -name "*.rs" -type f ! -path "*/tests/*" ! -path "*/target/*"); do
    # Simple approach: count lines between function declarations and their closing braces
    awk '
    /^[[:space:]]*(pub )?fn [a-zA-Z_]/ {
        if (in_fn && brace_count == 0) {
            lines = NR - fn_start
            if (lines > 50) {
                printf "%s:%d: %s - %d lines\n", FILENAME, fn_start, fn_name, lines
            }
        }
        in_fn = 1
        fn_start = NR
        fn_name = $0
        brace_count = 0

        # Count braces on the function declaration line
        opening = gsub(/{/, "{", $0)
        closing = gsub(/}/, "}", $0)
        brace_count += opening - closing

        # Check if function ends on same line
        if (brace_count == 0 && opening > 0) {
            in_fn = 0
        }
        next
    }
    in_fn {
        opening = gsub(/{/, "{", $0)
        closing = gsub(/}/, "}", $0)
        brace_count += opening - closing

        if (brace_count == 0) {
            lines = NR - fn_start + 1
            if (lines > 50) {
                printf "%s:%d: %d lines\n", FILENAME, fn_start, lines
            }
            in_fn = 0
        }
    }
    ' "$file"
done | sort -t: -k3 -rn | head -50
