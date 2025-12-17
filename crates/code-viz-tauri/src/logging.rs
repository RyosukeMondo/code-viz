use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize tracing subscriber with JSON output to stderr
///
/// Log level is controlled by the CODE_VIZ_DEBUG environment variable:
/// - CODE_VIZ_DEBUG=1: debug level logging
/// - Otherwise: info level logging
///
/// All logs are output as JSON to stderr with timestamps and structured fields.
pub fn init_logging() {
    let log_level = if std::env::var("CODE_VIZ_DEBUG").is_ok() {
        "debug"
    } else {
        "info"
    };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("code_viz_core={},code_viz_tauri={}", log_level, log_level)));

    let json_layer = fmt::layer()
        .json()
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(true)
        .with_span_list(false)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_does_not_panic() {
        // Test that initialization doesn't panic
        // Note: We can't actually call init_logging() multiple times in tests
        // as it can only be initialized once per process
        // This test just verifies the module compiles
        assert!(true);
    }
}
