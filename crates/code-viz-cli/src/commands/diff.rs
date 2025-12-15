use code_viz_core::AnalysisResult;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiffError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

pub fn run(old_path: PathBuf, new_path: PathBuf) -> Result<(), DiffError> {
    let old_json = fs::read_to_string(&old_path)?;
    let new_json = fs::read_to_string(&new_path)?;

    let old_result: AnalysisResult = serde_json::from_str(&old_json)?;
    let new_result: AnalysisResult = serde_json::from_str(&new_json)?;

    let old_files: HashMap<_, _> = old_result.files.iter().map(|f| (f.path.clone(), f)).collect();
    let new_files: HashMap<_, _> = new_result.files.iter().map(|f| (f.path.clone(), f)).collect();

    let mut files_added = 0;
    let mut files_deleted = 0;
    let mut files_modified = 0;
    let mut largest_growth_file: Option<PathBuf> = None;
    let mut largest_growth_delta = 0;

    for (path, _) in &new_files {
        if !old_files.contains_key(path) {
            files_added += 1;
        } else {
            let old_metric = old_files[path];
            let new_metric = new_files[path];
            if old_metric.loc != new_metric.loc {
                files_modified += 1;
                
                if new_metric.loc > old_metric.loc {
                    let delta = new_metric.loc - old_metric.loc;
                    if delta > largest_growth_delta {
                        largest_growth_delta = delta;
                        largest_growth_file = Some(path.clone());
                    }
                }
            }
        }
    }

    for path in old_files.keys() {
        if !new_files.contains_key(path) {
            files_deleted += 1;
        }
    }

    let old_loc = old_result.summary.total_loc;
    let new_loc = new_result.summary.total_loc;
    let delta_loc = new_loc as isize - old_loc as isize;
    let delta_sign = if delta_loc >= 0 { "+" } else { "" };

    println!("{} files added", files_added.to_string().green());
    println!("{} files deleted", files_deleted.to_string().red());
    println!("{} files modified (LOC changed)", files_modified.to_string().yellow());
    
    print!("Total LOC: {} -> {} (", old_loc, new_loc);
    if delta_loc > 0 {
        print!("{}", format!("{}{}", delta_sign, delta_loc).green());
    } else if delta_loc < 0 {
        print!("{}", format!("{}{}", delta_sign, delta_loc).red());
    } else {
        print!("0");
    }
    println!(")");

    if let Some(path) = largest_growth_file {
        println!(
            "Largest growth: {} (+{} LOC)",
            path.display().to_string().cyan(),
            largest_growth_delta
        );
    }

    Ok(())
}
