//! x11 module - X11 bindings and operations
//!
//! This module provides safe wrappers around Xlib functions and
//! handles low-level X11 operations.

pub mod connection;
pub mod event;
pub mod extension;
pub mod selection;

pub use connection::X11Connection;
pub use event::{XEvent, EventHandler};
pub use extension::{XTestExtension, DpmsExtension};
pub use selection::{X11Clipboard, SelectionState};

use anyhow::Result;

/// X11 error types
#[derive(Debug, thiserror::Error)]
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
