//! Input event handling
//!
//! This module handles mouse and keyboard input events,
//! including the fake event queue for simulating input.

pub mod fake;
pub mod keyboard;
pub mod mouse;

pub use fake::FakeEvent;
pub use keyboard::KeyboardHandler;
pub use mouse::MouseHandler;

use crate::core::DpyInfo;
use crate::x11::event::XEvent;
use anyhow::Result;

/// Input handler trait
///
/// All input handlers implement this trait.
pub trait InputHandler {
    /// Handle an input event
    ///
    /// Returns Ok(true) if the event was handled and should stop processing,
    /// Ok(false) if the event was not handled.
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool>;
}

#[cfg(test)]
mod tests;
