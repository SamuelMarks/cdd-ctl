#![deny(missing_docs)]
#![allow(unused_imports)]

//! cdd-ctl
//!
//! The main CLI orchestrator that delegates to `cdd-engine` and `cdd-gateway`.

pub use cdd_engine::config::AppConfig;
pub use cdd_engine::error::CddEngineError;

pub use cdd_gateway::api;
pub use cdd_gateway::db;
pub use cdd_gateway::github;
pub use cdd_gateway::{CddRepository, GitHubClient, PgRepository, ReqwestGitHubClient};

pub use cdd_engine::daemon;
pub use cdd_engine::error;
pub use cdd_engine::{ProcessConfig, ProcessManager};

/// Configuration re-exports
pub mod config {
    pub use cdd_engine::config::*;
}
