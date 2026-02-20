//! Keyboard input handling

use log::{info, warn, debug};

use crate::x11::event::{XEvent, XKeyEvent};
use crate::x11::X11Connection;
use crate::core::DpyInfo;
use crate::input::InputHandler;
use anyhow::Result;

/// Keyboard event handler
pub struct KeyboardHandler {
    conn: std::sync::Arc<X11Connection>,
    sticky_keys: crate::core::StickyKeys,
}

impl KeyboardHandler {
    /// Create a new keyboard handler
    pub fn new(conn: std::sync::Arc<X11Connection>) -> Self {
        Self {
            conn,
            sticky_keys: crate::core::StickyKeys::new(),
        }
    }

    /// Handle a key press event
    fn handle_key_press(&self, _event: &XKeyEvent, _ctx: &mut DpyInfo) -> Result<bool> {
        // TODO: Implement ProcessKeyEvent logic for press
        todo!("Implement KeyboardHandler::handle_key_press")
    }

    /// Handle a key release event
    fn handle_key_release(&self, _event: &XKeyEvent, _ctx: &mut DpyInfo) -> Result<bool> {
        // TODO: Implement ProcessKeyEvent logic for release
        todo!("Implement KeyboardHandler::handle_key_release")
    }
}

impl InputHandler for KeyboardHandler {
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool> {
        match event {
            XEvent::KeyPress(key) => self.handle_key_press(key, ctx),
            XEvent::KeyRelease(key) => self.handle_key_release(key, ctx),
            _ => Ok(false),
        }
    }
}
