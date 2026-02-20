//! Mouse input handling

use crate::x11::event::{XEvent, XMotionEvent, XButtonEvent};
use crate::x11::X11Connection;
use crate::core::DpyInfo;
use crate::input::InputHandler;
use anyhow::Result;

/// Mouse event handler
pub struct MouseHandler {
    conn: std::sync::Arc<X11Connection>,
}

impl MouseHandler {
    /// Create a new mouse handler
    pub fn new(conn: std::sync::Arc<X11Connection>) -> Self {
        Self { conn }
    }

    /// Handle a motion notify event
    fn handle_motion(&self, event: &XMotionEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // TODO: Implement ProcessMotionNotify logic:
        // 1. Get to_screen from ctx.to_screen
        // 2. Map coordinates using ctx.x_tables/y_tables
        // 3. Sanity check (unreasonableDelta)
        // 4. Handle COORD_INCR/COORD_DECR
        // 5. Fake motion on to display
        todo!("Implement MouseHandler::handle_motion")
    }

    /// Handle a button press event
    fn handle_button_press(&self, event: &XButtonEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // TODO: Implement ProcessButtonPress logic
        todo!("Implement MouseHandler::handle_button_press")
    }

    /// Handle a button release event
    fn handle_button_release(&self, event: &XButtonEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // TODO: Implement ProcessButtonRelease logic
        todo!("Implement MouseHandler::handle_button_release")
    }
}

impl InputHandler for MouseHandler {
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool> {
        match event {
            XEvent::MotionNotify(motion) => self.handle_motion(motion, ctx),
            XEvent::ButtonPress(btn) => self.handle_button_press(btn, ctx),
            XEvent::ButtonRelease(btn) => self.handle_button_release(btn, ctx),
            _ => Ok(false),
        }
    }
}
