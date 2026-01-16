//! Lines of Code (LOC) Calculation Module
//!
//! Calculates lines of code excluding comments and blank lines.
//! Uses Tree-sitter comment ranges for accurate detection.

use tree_sitter::Range;

/// Calculate lines of code excluding comments and blank lines
///
/// # Arguments
/// * `source` - Source code text
/// * `comment_ranges` - Tree-sitter comment ranges from parser
///
/// # Returns
/// Number of lines containing executable code
pub fn calculate_loc(source: &str, comment_ranges: &[Range]) -> usize {
    let mut loc = 0;

    for (i, line) in source.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // Check if line contains any code
        // We scan the line for any character that is NOT whitespace and NOT inside a comment.
        if contains_code(i, line, comment_ranges) {
            loc += 1;
        }
    }

    loc
}

/// Check if a line contains code (non-comment, non-whitespace content)
fn contains_code(row: usize, line: &str, comment_ranges: &[Range]) -> bool {
    let mut chars = line.char_indices().peekable();

    // Find first non-whitespace char
    while let Some((_col, c)) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Iterate through content
    while let Some((col, _c)) = chars.next() {
        match check_comment_at_position(row, col, comment_ranges) {
            None => return true, // Not in comment - found code!
            Some(None) => return false, // Comment extends to end of line
            Some(Some(end_col)) => skip_to_comment_end(&mut chars, end_col),
        }
    }

    false
}

/// Check if position is inside a comment range
///
/// # Returns
/// - `None` if not in comment
/// - `Some(None)` if in comment extending to end of line
/// - `Some(Some(end_col))` if in comment ending at end_col on same line
fn check_comment_at_position(
    row: usize,
    col: usize,
    comment_ranges: &[Range],
) -> Option<Option<usize>> {
    for range in comment_ranges {
        if is_in_range(row, col, range) {
            if range.end_point.row == row {
                return Some(Some(range.end_point.column));
            }
            return Some(None); // Comment extends to later line
        }
    }
    None // Not in comment
}

/// Skip characters until end of comment on current line
fn skip_to_comment_end(
    chars: &mut std::iter::Peekable<std::str::CharIndices>,
    end_col: usize,
) {
    while let Some((c_col, _)) = chars.peek() {
        if *c_col < end_col {
            chars.next();
        } else {
            break;
        }
    }
}

/// Check if position is within a Tree-sitter range
fn is_in_range(row: usize, col: usize, range: &Range) -> bool {
    let start = range.start_point;
    let end = range.end_point;

    // Check start
    if row < start.row { return false; }
    if row == start.row && col < start.column { return false; }

    // Check end
    if row > end.row { return false; }
    if row == end.row && col >= end.column { return false; }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::get_parser;

    // NOTE: Test code uses unwrap() for test fixtures and assertions.
    // This is acceptable because:
    // 1. Test data is controlled and known to be valid
    // 2. Test failures (panics) are the desired outcome when setup fails
    // 3. Panics in tests provide clear failure points for debugging

    #[test]
    fn test_loc_calculation() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            function main() {
                // This is a comment
                console.log("Hello"); // Inline comment
                /* Block comment
                   spanning lines */
                let x = 1;
            }
        "#;
        // LOC:
        // 1. empty
        // 2. function... (LOC)
        // 3. // ... (Skip)
        // 4. console... (LOC)
        // 5. /* ... (Skip)
        // 6. ... */ (Skip)
        // 7. let x = 1; (LOC)
        // 8. } (LOC)
        // 9. empty
        // Total: 4

        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 4);
    }

    #[test]
    fn test_comments_excluded() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            // Line 1
            // Line 2

            /* Block
               Line 4 */
        "#;
        // All comments or blank. LOC = 0.
        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 0);
    }

    #[test]
    fn test_mixed_line_comments() {
        let parser = get_parser("typescript").unwrap();
        let source = "let x = 1; // Comment";
        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 1);
    }

    #[test]
    fn test_multiline_comments_excluded() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            /*
             * Multi-line
             * Comment
             */
        "#;
        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 0);
    }

    #[test]
    fn test_comment_then_code() {
        // Rare case: /* c */ code
        let parser = get_parser("typescript").unwrap();
        let source = "/* c */ let x = 1;";
        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 1);
    }

    #[test]
    fn test_code_inside_comment_block() {
        // Checking boundaries
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            let a = 1;
            /* start
            mid
            end */ let b = 2;
        "#;
        // 1. let a (LOC)
        // 2. /* start (Skip)
        // 3. mid (Skip)
        // 4. end */ let b (LOC)
        // Total: 2
        let tree = parser.parse(source).unwrap();
        let comment_ranges = parser.find_comment_ranges(&tree);
        let loc = calculate_loc(source, &comment_ranges);
        assert_eq!(loc, 2);
    }
}
