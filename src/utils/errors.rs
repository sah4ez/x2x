//! Error types for x2x-rust

use thiserror::Error;

/// Main error type for x2x-rust
#[derive(Debug, Error)]
pub enum X2xError {
    /// X11-related errors
    #[error("X11 error: {0}")]
    X11(#[from] crate::x11::X11Error),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Connection errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Clipboard errors
    #[error("Clipboard error: {0}")]
    Clipboard(String),

    /// Input handling errors
    #[error("Input error: {0}")]
    Input(String),

    /// Invalid coordinate value
    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(i32),

    /// Display not available
    #[error("Display not available: {0}")]
    DisplayUnavailable(String),

    /// Feature not implemented yet
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, X2xError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let x2x_err: X2xError = io_err.into();
        assert!(matches!(x2x_err, X2xError::Io(_)));
    }
}
