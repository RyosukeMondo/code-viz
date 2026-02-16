pub mod analyze;
pub mod churn;
pub mod compare;
pub mod dead_code;
pub mod export;
pub mod shared;
pub mod timeline;

pub use analyze::{analyze_ai_commits, analyze_repository};
pub use churn::calculate_code_churn;
pub use dead_code::calculate_dead_code;
pub use export::export_report;
pub use timeline::generate_timeline;
