//! Windows/Cygwin support
//!
//! This module provides Windows-specific functionality for
//! running x2x from a Windows machine (via Cygwin or native Windows).

#[cfg(feature = "win32")]
pub mod keymap;
#[cfg(feature = "win32")]
pub mod window;

#[cfg(feature = "win32")]
pub use keymap::Win32KeyMap;
#[cfg(feature = "win32")]
pub use window::Win32Window;

#[cfg(feature = "win32")]
use anyhow::Result;

#[cfg(feature = "win32")]
/// Windows-specific error type
#[derive(Debug, thiserror::Error)]
pub enum Win32Error {
    #[error("Windows API error: {0}")]
    ApiError(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),

    #[error("Key mapping error: {0}")]
    KeyMapError(String),
}

#[cfg(feature = "win32")]
pub type Result<T> = std::result::Result<T, Win32Error>;
