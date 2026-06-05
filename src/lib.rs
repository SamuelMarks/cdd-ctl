#![deny(missing_docs)]

//! cdd-ctl
//! 
//! The main CLI orchestrator that delegates to `cdd-engine` and `cdd-gateway`.

/// Configuration module
pub mod config;
/// Error module
pub mod error;

pub use config::AppConfig;
pub use error::CddError;
