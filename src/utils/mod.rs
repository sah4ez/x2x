//! Utilities and configuration
//!
//! This module provides configuration parsing, error handling,
//! and other utilities.

pub mod config;
pub mod errors;

pub use config::Config;
pub use errors::{X2xError, Result};
