use thiserror::Error;

/// Errors that can occur during contract validation
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid value for field {0}: {1}")]
    InvalidValue(String, String),
    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),
}

mod helpers;

/// Tests for Specta schema validation
/// Ensures that the Rust types correctly generate the expected TypeScript schemas
mod specta_schema_tests {
    // TODO: Implement Specta schema validation tests
}

/// Tests for serialization round-trip validation
/// Ensures that data can be correctly serialized and deserialized via IPC
mod serialization_tests {
    // TODO: Implement serialization round-trip validation tests
}

/// Tests for ECharts compatibility validation
/// Ensures that the JSON output matches ECharts treemap requirements
mod echarts_compatibility_tests {
    // TODO: Implement ECharts compatibility validation tests
}
