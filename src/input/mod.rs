//! Input event handling
//!
//! This module handles mouse and keyboard input events,
//! including the fake event queue for simulating input.

pub mod mouse;
pub mod keyboard;
pub mod fake;

pub use mouse::MouseHandler;
pub use keyboard::KeyboardHandler;
pub use fake::{FakeEvent, FakeQueue};

use crate::x11::event::XEvent;
use crate::core::DpyInfo;
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
