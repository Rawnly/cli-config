#[deny(missing_docs)]

/// Core Module
pub mod config;

/// Errors
pub mod error;

pub mod fs;

// wrap default result type and inject local crate error
pub type Result<T> = std::result::Result<T, error::Error>;
