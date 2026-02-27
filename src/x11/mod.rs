//! x11 module - X11 bindings and operations
//!
//! This module provides safe wrappers around Xlib functions and
//! handles low-level X11 operations.

pub mod connection;
pub mod event;
pub mod extension;
pub mod selection;
pub mod error_handler;

pub use connection::X11Connection;
pub use event::{XEvent, EventHandler};
pub use extension::{XTestExtension, DpmsExtension};
pub use selection::{X11Clipboard, SelectionState};
pub use error_handler::{setup_error_handler, get_last_error, store_error, X11ErrorInfo};

use anyhow::Result;

/// X11 error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum X11Error {
    #[error("Failed to open display: {0}")]
    OpenDisplayFailed(String),

    #[error("X11 error: {0}")]
    Generic(String),

    #[error("XTest extension not available")]
    XTestNotAvailable,

    #[error("Invalid window: 0x{0:x}")]
    InvalidWindow(u64),

    #[error("Invalid screen: {0}")]
    InvalidScreen(i32),
}

/// X11 Window type (alias for clarity)
pub type Window = u64;

/// X11 Atom type
pub type Atom = u32;

/// X11 Time type
pub type Time = u32;

/// X11 Screen information
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    pub screen_num: i32,
    pub root: Window,
    pub width: u32,
    pub height: u32,
}

/// Initialize X11 library
///
/// This must be called before any X11 operations.
pub fn init_xlib() -> Result<()> {
    unsafe {
        // Call XInitThreads if needed for thread safety
        // x11-dl handles this automatically usually
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_aliases() {
        // Test that type aliases are defined
        let window: Window = 0;
        let atom: Atom = 0;
        let time: Time = 0;
        assert_eq!(window, 0);
        assert_eq!(atom, 0);
        assert_eq!(time, 0);
    }

    #[test]
    fn test_screen_info() {
        let info = ScreenInfo {
            screen_num: 0,
            root: 0,
            width: 1920,
            height: 1080,
        };
        assert_eq!(info.screen_num, 0);
        assert_eq!(info.root, 0);
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
    }

    #[test]
    fn test_x11_error_display() {
        let err = X11Error::OpenDisplayFailed(":0".to_string());
        assert!(err.to_string().contains(":0"));

        let err = X11Error::Generic("test".to_string());
        assert!(err.to_string().contains("test"));
    }

    #[test]
    fn test_init_xlib() {
        // Test that init_xlib doesn't panic
        assert!(init_xlib().is_ok());
    }
}
