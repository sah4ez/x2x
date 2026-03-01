//! Utilities and configuration
//!
//! This module provides configuration parsing, error handling,
//! and other utilities.

pub mod config;
pub mod errors;

pub use config::Config;
pub use errors::{Result, X2xError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_is_visible() {
        // Test that Config is properly exported
        let config = Config::default();
        assert_eq!(config.to_display, "localhost:1");
    }

    #[test]
    fn test_error_is_visible() {
        // Test that X2xError is properly exported
        use std::io;

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let x2x_err: X2xError = io_err.into();
        assert!(matches!(x2x_err, X2xError::Io(_)));
    }
}
