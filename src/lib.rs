pub mod bnet;

/// Result type shared across all modules
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
