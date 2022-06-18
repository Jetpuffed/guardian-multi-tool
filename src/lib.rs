pub mod bnet_entities;
pub mod bnet_endpoints;

/// Result type shared across all modules
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
