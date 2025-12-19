use crate::config_loader;
use crate::output::{self, MetricsFormatter};
use code_viz_core::{analyze, AnalysisConfig, AnalysisResult};
use code_viz_core::traits::{AppContext, FileSystem};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Watch setup failed: {0}")]
    NotifyError(#[from] notify::Error),

    #[error("Analysis failed: {0}")]
    AnalysisError(#[from] code_viz_core::analyzer::AnalysisError),

    #[error("Config error: {0}")]
    ConfigError(#[from] crate::config_loader::ConfigError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Formatting failed: {0}")]
    FormattingFailed(#[from] crate::output::FormatterError),
}

pub fn run(path: PathBuf, format: String, verbose: bool, _ctx: impl AppContext, _fs: impl FileSystem) -> Result<(), WatchError> {
    // Setup logging
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init();

    // Load config
    let mut config = AnalysisConfig::default();
    let current_dir = std::env::current_dir()?;
    let file_config = config_loader::load_config(&current_dir)?;
    if let Some(analysis) = file_config.analysis {
        if let Some(file_excludes) = analysis.exclude {
            config.exclude_patterns = file_excludes;
        }
    }

    // Initial analysis
    if format != "json" {
        println!("Performing initial analysis...");
    }
    let mut current_result = analyze(&path, &config)?;
    print_output(&current_result, &format)?;

    // Setup channel
    let (tx, rx) = channel();

    // Setup watcher
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    if format != "json" {
        println!("Watching for changes in {}...", path.display());
    }

    // Setup Ctrl+C handler
    let running = Arc::new(Mutex::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let mut running = r.lock().unwrap();
        *running = false;
        // We can't break the loop easily from here unless we send a signal or checking flag
        // The main loop checks flag
    }).expect("Error setting Ctrl-C handler");

    // Loop
    loop {
        // Check running flag
        if !*running.lock().unwrap() {
            if format != "json" {
                println!("\nStopping watch mode...");
            }
            break;
        }

        // Wait for event (with timeout to check running flag periodically if needed, 
        // but recv() blocks. Ctrl+C handler can exit process or we use timeout loop)
        // Ideally we use a select or timeout.
        // Let's use recv_timeout loop to check running flag.
        let event_res = rx.recv_timeout(Duration::from_millis(500));

        match event_res {
            Ok(event_res) => {
                match event_res {
                    Ok(event) => {
                        // Collect events for debounce window (100ms)
                        let mut changed_paths = HashSet::new();
                        add_paths_from_event(&mut changed_paths, event);

                        let deadline = SystemTime::now() + Duration::from_millis(100);
                        while let Ok(dur) = deadline.duration_since(SystemTime::now()) {
                            if let Ok(res) = rx.recv_timeout(dur) {
                                if let Ok(e) = res {
                                    add_paths_from_event(&mut changed_paths, e);
                                }
                            } else {
                                break; // Timeout (debounce window end) or Disconnected
                            }
                        }

                        if !changed_paths.is_empty() {
                            handle_changes(&mut current_result, changed_paths, &format)?;
                        }
                    }
                    Err(e) => eprintln!("Watch error: {}", e),
                }
            }
            Err(RecvTimeoutError::Timeout) => continue, // Just loop to check running flag
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn add_paths_from_event(paths: &mut HashSet<PathBuf>, event: notify::Event) {
    for path in event.paths {
        // Filter by extension
        if let Some(ext) = path.extension() {
            match ext.to_string_lossy().as_ref() {
                "ts" | "tsx" | "js" | "jsx" | "rs" | "py" => {
                    paths.insert(path);
                }
                _ => {}
            }
        }
    }
}

fn handle_changes(
    result: &mut AnalysisResult,
    paths: HashSet<PathBuf>,
    format: &str,
) -> Result<(), WatchError> {
    let mut updated = false;

    for path in paths {
        // Check if file exists (modification/creation) or deleted
        if path.exists() {
            // Re-analyze file
            match code_viz_core::analyzer::process_file(&path) {
                Ok(metrics) => {
                    // Update result.files
                    if let Some(existing) = result.files.iter_mut().find(|f| f.path == metrics.path) {
                        *existing = metrics;
                    } else {
                        result.files.push(metrics);
                    }
                    if format != "json" {
                        // Print update
                        // Find the metrics we just added/updated to get correct reference or use `metrics` variable
                        // Using `metrics` variable directly
                        // We need to re-fetch it from array to be safe? No, `metrics` is owned.
                        // Wait, I moved metrics into array.
                        // I'll assume it worked.
                        let m = result.files.iter().find(|f| f.path == path).unwrap();
                        println!(
                            "[{}] {}: {} LOC ({} funcs)",
                            chrono::Local::now().format("%H:%M:%S"),
                            m.path.display(),
                            m.loc,
                            m.function_count
                        );
                    }
                    updated = true;
                }
                Err(e) => {
                    eprintln!("Failed to re-analyze {:?}: {}", path, e);
                }
            }
        } else {
            // File deleted
            if let Some(idx) = result.files.iter().position(|f| f.path == path) {
                result.files.remove(idx);
                if format != "json" {
                    println!("[{}] Deleted: {}", chrono::Local::now().format("%H:%M:%S"), path.display());
                }
                updated = true;
            }
        }
    }

    if updated {
        // Re-calculate summary
        result.summary = code_viz_core::analyzer::calculate_summary(&result.files);
        result.timestamp = SystemTime::now();

        if format == "json" {
            print_output(result, format)?;
        }
    }

    Ok(())
}

fn print_output(result: &AnalysisResult, format: &str) -> Result<(), WatchError> {
    if format == "json" {
        // Compact JSON on one line
        let json = serde_json::to_string(result).map_err(|_| crate::output::FormatterError::FormattingFailed)?;
        println!("{}", json);
    } else {
        // For text, we printed incremental updates.
        // Maybe print summary again?
        // "print updated metrics ... if --format json ... else print '[timestamp] path: X LOC'"
        // So for text mode, we don't print full summary every time, just the update line.
        // Initial analysis prints full summary.
        if !result.files.is_empty() { // Check if initial call
             // Actually `run` calls `print_output` for initial result.
             // If format is text, we want full summary.
             // But inside loop, we handle incremental prints manually.
             // So `print_output` is mainly for JSON or initial Text.
             
             // How to distinguish?
             // `run` calls `print_output` initially.
             // `handle_changes` calls `print_output` ONLY if JSON.
             // Text updates are handled in `handle_changes` loop.
             // So we are good.
             
             if format != "json" {
                 // Use TextFormatter
                 let formatter = output::text::TextFormatter;
                 let output = formatter.format(result)?;
                 println!("{}", output);
             }
        }
    }
    Ok(())
}
