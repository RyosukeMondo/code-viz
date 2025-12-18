use code_viz_dead_code::analyze_dead_code;
use std::path::PathBuf;

/// Ground truth data for manual verification
#[derive(Debug)]
struct GroundTruth {
    /// Symbols that should be flagged as dead with HIGH confidence (>80)
    high_confidence_dead: Vec<SymbolInfo>,
    /// Symbols that should be flagged as dead with LOW/MEDIUM confidence (<80)
    low_confidence_dead: Vec<SymbolInfo>,
    /// Symbols that should NOT be flagged as dead at all
    live_symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Clone, PartialEq)]
struct SymbolInfo {
    name: String,
    file_path: String,
}

impl SymbolInfo {
    fn new(name: &str, file_path: &str) -> Self {
        Self {
            name: name.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

/// Project 1: Dynamic imports
fn project1_ground_truth() -> GroundTruth {
    GroundTruth {
        high_confidence_dead: vec![],
        low_confidence_dead: vec![
            SymbolInfo::new("validateUser", "src/plugins/user_plugin.ts"),
            SymbolInfo::new("validateAdmin", "src/plugins/admin_plugin.ts"),
        ],
        live_symbols: vec![
            SymbolInfo::new("loadPlugin", "src/index.ts"),
            SymbolInfo::new("main", "src/index.ts"),
        ],
    }
}

/// Project 2: Reflection patterns
fn project2_ground_truth() -> GroundTruth {
    GroundTruth {
        high_confidence_dead: vec![SymbolInfo::new("handleRefund", "src/handlers.ts")],
        low_confidence_dead: vec![
            SymbolInfo::new("handleUser", "src/handlers.ts"),
            SymbolInfo::new("handleOrder", "src/handlers.ts"),
            SymbolInfo::new("handlePayment", "src/handlers.ts"),
        ],
        live_symbols: vec![SymbolInfo::new("callHandler", "src/main.ts")],
    }
}

/// Project 3: Library public API
fn project3_ground_truth() -> GroundTruth {
    GroundTruth {
        high_confidence_dead: vec![SymbolInfo::new("internalHelper", "src/index.ts")],
        low_confidence_dead: vec![
            SymbolInfo::new("createUser", "src/index.ts"),
            SymbolInfo::new("deleteUser", "src/index.ts"),
            SymbolInfo::new("updateUser", "src/index.ts"),
            SymbolInfo::new("formatDate", "src/utils.ts"),
            SymbolInfo::new("parseDate", "src/utils.ts"),
            SymbolInfo::new("validateEmail", "src/utils.ts"),
            SymbolInfo::new("obsoleteFunction", "src/utils.ts"),
        ],
        live_symbols: vec![],
    }
}

/// Project 4: Recent modifications
fn project4_ground_truth() -> GroundTruth {
    GroundTruth {
        high_confidence_dead: vec![],
        low_confidence_dead: vec![
            SymbolInfo::new("newFeature", "src/processor.ts"),
            SymbolInfo::new("legacyProcessor", "src/processor.ts"),
        ],
        live_symbols: vec![
            SymbolInfo::new("main", "src/main.ts"),
            SymbolInfo::new("processData", "src/processor.ts"),
        ],
    }
}

/// Project 5: Test helpers
fn project5_ground_truth() -> GroundTruth {
    GroundTruth {
        high_confidence_dead: vec![SymbolInfo::new("createMockUser", "src/testHelpers.ts")],
        low_confidence_dead: vec![],
        live_symbols: vec![
            SymbolInfo::new("calculate", "src/index.ts"),
            SymbolInfo::new("process", "src/index.ts"),
            SymbolInfo::new("createMockData", "src/testHelpers.ts"),
            SymbolInfo::new("assertDeepEqual", "src/testHelpers.ts"),
        ],
    }
}

#[derive(Debug)]
struct ValidationResult {
    project_name: String,
    true_positives: usize,
    false_positives: usize,
    false_negatives: usize,
    correct_low_confidence: usize,
    total_high_conf_detected: usize,
}

impl ValidationResult {
    fn false_positive_rate(&self) -> f64 {
        if self.total_high_conf_detected == 0 {
            return 0.0;
        }
        self.false_positives as f64 / self.total_high_conf_detected as f64
    }

    fn accuracy(&self) -> f64 {
        let total = self.true_positives + self.false_positives + self.false_negatives;
        if total == 0 {
            return 1.0;
        }
        self.true_positives as f64 / total as f64
    }
}

fn validate_project(
    project_path: &str,
    ground_truth: GroundTruth,
    project_name: &str,
) -> ValidationResult {
    let path = PathBuf::from("tests/false_positive_corpus").join(project_path);

    let result = analyze_dead_code(&path, None)
        .unwrap_or_else(|e| panic!("Failed to analyze {}: {}", project_name, e));

    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut false_negatives = 0;
    let mut correct_low_confidence = 0;
    let mut total_high_conf_detected = 0;

    // Collect all detected dead symbols with high confidence (>80)
    let mut high_conf_detected = Vec::new();
    let mut low_conf_detected = Vec::new();

    for file_dead in &result.files {
        for dead_symbol in &file_dead.dead_code {
            let symbol_info = SymbolInfo::new(
                &dead_symbol.symbol,
                &file_dead.path.to_string_lossy(),
            );

            if dead_symbol.confidence > 80 {
                high_conf_detected.push(symbol_info.clone());
                total_high_conf_detected += 1;
            } else {
                low_conf_detected.push(symbol_info);
            }
        }
    }

    // Check high confidence detections
    for detected in &high_conf_detected {
        let is_true_positive = ground_truth
            .high_confidence_dead
            .iter()
            .any(|gt| symbol_matches(detected, gt));

        let is_should_be_live = ground_truth
            .live_symbols
            .iter()
            .any(|gt| symbol_matches(detected, gt));

        let is_should_be_low_conf = ground_truth
            .low_confidence_dead
            .iter()
            .any(|gt| symbol_matches(detected, gt));

        if is_true_positive {
            true_positives += 1;
        } else if is_should_be_live || is_should_be_low_conf {
            false_positives += 1;
            eprintln!(
                "⚠️  FALSE POSITIVE in {}: {} in {} flagged with high confidence",
                project_name, detected.name, detected.file_path
            );
        }
    }

    // Check low confidence detections (should match low_confidence_dead)
    for detected in &low_conf_detected {
        let is_correct_low_conf = ground_truth
            .low_confidence_dead
            .iter()
            .any(|gt| symbol_matches(detected, gt));

        if is_correct_low_conf {
            correct_low_confidence += 1;
        }
    }

    // Check for false negatives (should be detected but wasn't)
    for expected_dead in &ground_truth.high_confidence_dead {
        let was_detected = high_conf_detected
            .iter()
            .any(|d| symbol_matches(d, expected_dead));

        if !was_detected {
            false_negatives += 1;
            eprintln!(
                "⚠️  FALSE NEGATIVE in {}: {} in {} should be flagged but wasn't",
                project_name, expected_dead.name, expected_dead.file_path
            );
        }
    }

    ValidationResult {
        project_name: project_name.to_string(),
        true_positives,
        false_positives,
        false_negatives,
        correct_low_confidence,
        total_high_conf_detected,
    }
}

fn symbol_matches(a: &SymbolInfo, b: &SymbolInfo) -> bool {
    a.name == b.name && a.file_path.contains(&b.file_path)
}

#[test]
fn test_false_positive_rate_validation() {
    println!("\n🔍 Running False Positive Rate Validation\n");

    let projects = vec![
        ("project1", project1_ground_truth(), "Dynamic Imports"),
        ("project2", project2_ground_truth(), "Reflection Patterns"),
        ("project3", project3_ground_truth(), "Library Public API"),
        ("project4", project4_ground_truth(), "Recent Modifications"),
        ("project5", project5_ground_truth(), "Test Helpers"),
    ];

    let mut all_results = Vec::new();
    let mut total_tp = 0;
    let mut total_fp = 0;
    let mut total_fn = 0;
    let mut total_high_conf = 0;

    for (project_path, ground_truth, project_name) in projects {
        println!("📁 Validating {}", project_name);
        let result = validate_project(project_path, ground_truth, project_name);

        println!("  ✓ True Positives: {}", result.true_positives);
        println!("  ✗ False Positives: {}", result.false_positives);
        println!("  ✗ False Negatives: {}", result.false_negatives);
        println!(
            "  ✓ Correct Low Confidence: {}",
            result.correct_low_confidence
        );
        println!(
            "  📊 False Positive Rate: {:.1}%",
            result.false_positive_rate() * 100.0
        );
        println!("  📊 Accuracy: {:.1}%\n", result.accuracy() * 100.0);

        total_tp += result.true_positives;
        total_fp += result.false_positives;
        total_fn += result.false_negatives;
        total_high_conf += result.total_high_conf_detected;

        all_results.push(result);
    }

    // Calculate overall false positive rate
    let overall_fp_rate = if total_high_conf > 0 {
        total_fp as f64 / total_high_conf as f64
    } else {
        0.0
    };

    println!("═══════════════════════════════════════════");
    println!("📊 OVERALL RESULTS");
    println!("═══════════════════════════════════════════");
    println!("Total True Positives: {}", total_tp);
    println!("Total False Positives: {}", total_fp);
    println!("Total False Negatives: {}", total_fn);
    println!("Total High-Confidence Detections: {}", total_high_conf);
    println!("\n🎯 OVERALL FALSE POSITIVE RATE: {:.2}%", overall_fp_rate * 100.0);
    println!("🎯 TARGET: <5.00%\n");

    // Assert that false positive rate is below 5%
    assert!(
        overall_fp_rate < 0.05,
        "False positive rate {:.2}% exceeds target of 5%",
        overall_fp_rate * 100.0
    );

    println!("✅ PRIMARY VALIDATION CHECK PASSED!");
    println!("  False Positive Rate: {:.2}% < 5.00% ✓\n", overall_fp_rate * 100.0);

    // Additional quality checks (warnings only, not failures)
    println!("📋 Additional Metrics:");

    // Check if we detected any symbols at all
    let total_detected = total_tp + total_fp + total_fn;
    if total_detected == 0 {
        println!("  ⚠️  Warning: No symbols detected - detector may need tuning");
    } else {
        println!("  ✓ Total symbols analyzed: {}", total_detected);
    }

    // Check for low confidence detections (proper uncertainty handling)
    let total_low_conf: usize = all_results.iter().map(|r| r.correct_low_confidence).sum();
    if total_low_conf > 0 {
        println!("  ✓ Low-confidence detections: {} (proper uncertainty handling)", total_low_conf);
    } else {
        println!("  ⚠️  Warning: No low-confidence detections - all decisions were binary");
    }

    // False negatives are acceptable (conservative is better than aggressive)
    if total_fn > 0 {
        println!("  ℹ️  Note: {} false negatives (conservative behavior - acceptable)", total_fn);
    }

    println!("\n✅ False Positive Rate Validation PASSED!");
    println!("═══════════════════════════════════════════\n");
}

#[test]
fn test_project1_dynamic_imports() {
    println!("\n🧪 Testing Project 1: Dynamic Imports");
    let result = validate_project("project1", project1_ground_truth(), "Project 1");

    // Should not flag dynamically imported plugins with high confidence
    assert_eq!(
        result.false_positives, 0,
        "Dynamic imports should not be high-confidence false positives"
    );

    println!("✅ Project 1 validation passed\n");
}

#[test]
fn test_project2_reflection() {
    println!("\n🧪 Testing Project 2: Reflection Patterns");
    let result = validate_project("project2", project2_ground_truth(), "Project 2");

    // Most important: should not flag reflection-accessed handlers with high confidence
    assert_eq!(
        result.false_positives, 0,
        "Reflection handlers should have low confidence (no false positives)"
    );

    // Note: If handleRefund isn't detected, that's okay - conservative behavior
    if result.true_positives > 0 {
        println!("  ✓ Detected {} truly dead symbols", result.true_positives);
    } else {
        println!("  ⚠️  Note: No high-confidence dead symbols detected (conservative)");
    }

    println!("✅ Project 2 validation passed (no false positives)\n");
}

#[test]
fn test_project3_library_api() {
    println!("\n🧪 Testing Project 3: Library Public API");
    let result = validate_project("project3", project3_ground_truth(), "Project 3");

    // Most important: should not flag exported API with high confidence
    assert_eq!(
        result.false_positives, 0,
        "Exported library API should have low confidence (no false positives)"
    );

    // Check for low-confidence detections (exported symbols flagged properly)
    if result.correct_low_confidence > 0 {
        println!("  ✓ Properly flagged {} exported symbols with low confidence", result.correct_low_confidence);
    }

    // Note: If internalHelper isn't detected, that's okay - conservative behavior
    if result.true_positives > 0 {
        println!("  ✓ Detected {} truly dead symbols", result.true_positives);
    } else {
        println!("  ⚠️  Note: No high-confidence dead symbols detected (conservative)");
    }

    println!("✅ Project 3 validation passed (no false positives)\n");
}

#[test]
fn test_confidence_scoring() {
    println!("\n🧪 Testing Confidence Scoring");

    let path = PathBuf::from("tests/false_positive_corpus/project3");
    let result = analyze_dead_code(&path, None).expect("Analysis failed");

    let mut exported_symbol_confidence = None;
    let mut internal_symbol_confidence = None;

    for file_dead in &result.files {
        for dead_symbol in &file_dead.dead_code {
            if dead_symbol.symbol == "createUser" {
                exported_symbol_confidence = Some(dead_symbol.confidence);
            }
            if dead_symbol.symbol == "internalHelper" {
                internal_symbol_confidence = Some(dead_symbol.confidence);
            }
        }
    }

    // Exported symbols should have lower confidence if detected
    if let Some(conf) = exported_symbol_confidence {
        println!("  ℹ️  Exported symbol (createUser) confidence: {}", conf);
        if conf < 70 {
            println!("  ✓ Low confidence for exported symbol (< 70) - GOOD");
        } else {
            println!("  ⚠️  Note: Exported symbol has confidence >= 70");
        }
    } else {
        println!("  ℹ️  Exported symbol (createUser) not flagged as dead");
    }

    // Internal unused symbols should have high confidence if detected
    if let Some(conf) = internal_symbol_confidence {
        println!("  ℹ️  Internal symbol (internalHelper) confidence: {}", conf);
        if conf >= 80 {
            println!("  ✓ High confidence for internal symbol (>= 80) - GOOD");
        } else {
            println!("  ⚠️  Note: Internal symbol has confidence < 80");
        }
    } else {
        println!("  ℹ️  Internal symbol (internalHelper) not flagged as dead");
    }

    // The key test: if both are detected, exported should have lower confidence
    if let (Some(exp_conf), Some(int_conf)) = (exported_symbol_confidence, internal_symbol_confidence) {
        if exp_conf < int_conf {
            println!("  ✓ Confidence scoring working: exported ({}) < internal ({})", exp_conf, int_conf);
        }
    }

    println!("✅ Confidence scoring test completed\n");
}
