use code_viz_dead_code::DeadCodeResult;
use code_viz_dead_code::models::SymbolKind;
use colored::*;
use serde_json;
use std::fmt::Write;

#[derive(Debug)]
pub enum DeadCodeFormatterError {
    JsonSerializationFailed,
    TextFormattingFailed,
}

impl std::fmt::Display for DeadCodeFormatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonSerializationFailed => write!(f, "Failed to serialize to JSON"),
            Self::TextFormattingFailed => write!(f, "Failed to format text output"),
        }
    }
}

impl std::error::Error for DeadCodeFormatterError {}

/// Format dead code result as pretty-printed JSON
pub fn format_json(result: &DeadCodeResult) -> Result<String, DeadCodeFormatterError> {
    serde_json::to_string_pretty(result)
        .map_err(|_| DeadCodeFormatterError::JsonSerializationFailed)
}

/// Format dead code result as human-readable text with colors
pub fn format_text(result: &DeadCodeResult) -> Result<String, DeadCodeFormatterError> {
    let mut output = String::new();
    let summary = &result.summary;

    // Header
    writeln!(output, "\n{}", "Dead Code Analysis Summary".bold())
        .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;
    writeln!(output, "{}", "=".repeat(50))
        .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    // Summary statistics
    writeln!(
        output,
        "Total files analyzed:     {}",
        summary.total_files
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "Files with dead code:     {} ({:.1}%)",
        summary.files_with_dead_code,
        if summary.total_files > 0 {
            (summary.files_with_dead_code as f64 / summary.total_files as f64) * 100.0
        } else {
            0.0
        }
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "Total dead code:          {} LOC ({:.1}%)",
        summary.total_dead_loc,
        summary.dead_code_ratio * 100.0
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "Dead functions:           {}",
        summary.dead_functions
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "Dead classes:             {}",
        summary.dead_classes
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    // High-confidence deletions
    let high_confidence_count = result
        .files
        .iter()
        .flat_map(|f| &f.dead_code)
        .filter(|s| s.confidence >= 90)
        .count();

    writeln!(
        output,
        "{}",
        format!("High-confidence deletions: {}", high_confidence_count).green()
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    // Top files with dead code
    if !result.files.is_empty() {
        writeln!(output, "\n{}", "Top files by dead code:".bold())
            .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

        // Sort files by total dead LOC
        let mut files_sorted: Vec<_> = result.files.iter().collect();
        files_sorted.sort_by(|a, b| {
            let a_loc: usize = a.dead_code.iter().map(|s| s.loc).sum();
            let b_loc: usize = b.dead_code.iter().map(|s| s.loc).sum();
            b_loc.cmp(&a_loc)
        });

        for (i, file) in files_sorted.iter().take(10).enumerate() {
            let total_dead_loc: usize = file.dead_code.iter().map(|s| s.loc).sum();
            let symbol_count = file.dead_code.len();
            writeln!(
                output,
                "  {}. {} - {} LOC ({} symbols)",
                i + 1,
                file.path.display(),
                total_dead_loc,
                symbol_count
            )
            .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;
        }
    }

    // Detailed breakdown by confidence tier
    writeln!(output, "\n{}", "Confidence Breakdown:".bold())
        .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    let all_symbols: Vec<_> = result
        .files
        .iter()
        .flat_map(|f| &f.dead_code)
        .collect();

    let high_conf = all_symbols.iter().filter(|s| s.confidence >= 90).count();
    let medium_conf = all_symbols
        .iter()
        .filter(|s| s.confidence >= 70 && s.confidence < 90)
        .count();
    let low_conf = all_symbols.iter().filter(|s| s.confidence < 70).count();

    writeln!(
        output,
        "  {} High (≥90):   {} symbols",
        "●".green(),
        high_conf
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "  {} Medium (70-89): {} symbols",
        "●".yellow(),
        medium_conf
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    writeln!(
        output,
        "  {} Low (<70):    {} symbols",
        "●".red(),
        low_conf
    )
    .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

    // Detailed file listing
    if !result.files.is_empty() {
        writeln!(output, "\n{}", "Dead Code by File:".bold())
            .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

        for file in &result.files {
            writeln!(output, "\n  {}", file.path.display().to_string().cyan())
                .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

            // Sort symbols by confidence (highest first)
            let mut symbols_sorted = file.dead_code.clone();
            symbols_sorted.sort_by(|a, b| b.confidence.cmp(&a.confidence));

            for symbol in symbols_sorted {
                let confidence_colored = colorize_confidence(symbol.confidence);
                let kind_str = format_symbol_kind(symbol.kind);

                writeln!(
                    output,
                    "    {} {} (lines {}-{}, {} LOC, confidence: {})",
                    kind_str,
                    symbol.symbol.bold(),
                    symbol.line_start,
                    symbol.line_end,
                    symbol.loc,
                    confidence_colored
                )
                .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;

                // Show reason if available
                if !symbol.reason.is_empty() {
                    writeln!(output, "      Reason: {}", symbol.reason.dimmed())
                        .map_err(|_| DeadCodeFormatterError::TextFormattingFailed)?;
                }
            }
        }
    }

    Ok(output)
}

/// Colorize confidence score based on thresholds
fn colorize_confidence(confidence: u8) -> String {
    let conf_str = format!("{}%", confidence);
    if confidence >= 90 {
        conf_str.green().to_string()
    } else if confidence >= 70 {
        conf_str.yellow().to_string()
    } else {
        conf_str.red().to_string()
    }
}

/// Format symbol kind as human-readable string
fn format_symbol_kind(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::Function => "fn",
        SymbolKind::ArrowFunction => "=>",
        SymbolKind::Class => "class",
        SymbolKind::Method => "method",
        SymbolKind::Variable => "var",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_dead_code::{DeadCodeSummary, DeadSymbol, FileDeadCode};
    use std::path::PathBuf;

    fn create_sample_result() -> DeadCodeResult {
        DeadCodeResult {
            summary: DeadCodeSummary {
                total_files: 10,
                files_with_dead_code: 2,
                dead_functions: 5,
                dead_classes: 1,
                total_dead_loc: 150,
                dead_code_ratio: 0.15,
            },
            files: vec![
                FileDeadCode {
                    path: PathBuf::from("src/utils.ts"),
                    dead_code: vec![
                        DeadSymbol {
                            symbol: "unusedFunction".to_string(),
                            kind: SymbolKind::Function,
                            line_start: 10,
                            line_end: 20,
                            loc: 10,
                            confidence: 95,
                            reason: "Not imported or called anywhere".to_string(),
                            last_modified: None,
                        },
                        DeadSymbol {
                            symbol: "oldHelper".to_string(),
                            kind: SymbolKind::ArrowFunction,
                            line_start: 25,
                            line_end: 30,
                            loc: 5,
                            confidence: 85,
                            reason: "Exported but never used".to_string(),
                            last_modified: None,
                        },
                    ],
                },
                FileDeadCode {
                    path: PathBuf::from("src/legacy.ts"),
                    dead_code: vec![DeadSymbol {
                        symbol: "LegacyClass".to_string(),
                        kind: SymbolKind::Class,
                        line_start: 1,
                        line_end: 100,
                        loc: 100,
                        confidence: 65,
                        reason: "Exported and recently modified".to_string(),
                        last_modified: None,
                    }],
                },
            ],
        }
    }

    #[test]
    fn test_format_json() {
        let result = create_sample_result();
        let json = format_json(&result).unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["summary"]["total_files"], 10);
        assert_eq!(parsed["summary"]["dead_functions"], 5);
        assert_eq!(parsed["files"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_format_text() {
        let result = create_sample_result();
        let text = format_text(&result).unwrap();

        // Verify key elements are present
        assert!(text.contains("Dead Code Analysis Summary"));
        assert!(text.contains("Total files analyzed:     10"));
        assert!(text.contains("Files with dead code:     2"));
        assert!(text.contains("Total dead code:          150 LOC"));
        assert!(text.contains("Dead functions:           5"));
        assert!(text.contains("Dead classes:             1"));
        assert!(text.contains("unusedFunction"));
        assert!(text.contains("LegacyClass"));
        assert!(text.contains("src/utils.ts"));
        assert!(text.contains("src/legacy.ts"));
    }

    #[test]
    fn test_format_text_empty_result() {
        let result = DeadCodeResult {
            summary: DeadCodeSummary {
                total_files: 5,
                files_with_dead_code: 0,
                dead_functions: 0,
                dead_classes: 0,
                total_dead_loc: 0,
                dead_code_ratio: 0.0,
            },
            files: vec![],
        };

        let text = format_text(&result).unwrap();
        assert!(text.contains("Total files analyzed:     5"));
        assert!(text.contains("Files with dead code:     0"));
        assert!(text.contains("Total dead code:          0 LOC"));
    }

    #[test]
    fn test_confidence_colorization() {
        // Just verify these don't panic
        let high = colorize_confidence(95);
        let medium = colorize_confidence(80);
        let low = colorize_confidence(50);

        assert!(high.contains("95"));
        assert!(medium.contains("80"));
        assert!(low.contains("50"));
    }

    #[test]
    fn test_symbol_kind_formatting() {
        assert_eq!(format_symbol_kind(SymbolKind::Function), "fn");
        assert_eq!(format_symbol_kind(SymbolKind::ArrowFunction), "=>");
        assert_eq!(format_symbol_kind(SymbolKind::Class), "class");
        assert_eq!(format_symbol_kind(SymbolKind::Method), "method");
        assert_eq!(format_symbol_kind(SymbolKind::Variable), "var");
    }
}
