pub mod constants;
pub mod error;
pub mod types;

// Stub modules for future implementation
pub mod http;
pub mod rate_limit;
pub mod client;
pub mod auth;
pub mod validation;
pub mod api;
pub mod pagination;

// Re-export primary types for convenience
pub use error::{Error, Result};
pub use types::*;
