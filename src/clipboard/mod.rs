//! Clipboard manager for inter-display sharing

use crate::x11::X11Connection;
use anyhow::Result;

pub mod x11_selection;

#[cfg(test)]
mod tests;

/// Clipboard manager for inter-display sharing
pub struct ClipboardManager {
    from_clipboard: x11_selection::X11Clipboard,
    to_clipboard: x11_selection::X11Clipboard,
}

impl ClipboardManager {
    pub fn new(from_conn: std::sync::Arc<X11Connection>, to_conn: std::sync::Arc<X11Connection>) -> Result<Self> {
        Ok(Self {
            from_clipboard: x11_selection::X11Clipboard::new(from_conn)?,
            to_clipboard: x11_selection::X11Clipboard::new(to_conn)?,
        })
    }

    /// Handle a clipboard event from "from" display
    pub fn handle_from_event(&mut self, _event: &crate::x11::event::XEvent, _ctx: &mut crate::core::DpyInfo) -> Result<()> {
        // TODO: Implement clipboard event forwarding
        Ok(())
    }

    /// Handle a clipboard event from "to" display
    pub fn handle_to_event(&mut self, _event: &crate::x11::event::XEvent, _ctx: &mut crate::core::DpyInfo) -> Result<()> {
        // TODO: Implement clipboard event forwarding
        Ok(())
    }
}
