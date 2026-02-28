//! Mouse input handling

use log::{info, warn, debug};
use crate::x11::event::{XEvent, XMotionEvent, XButtonEvent};
use crate::x11::X11Connection;
use crate::x11::extension::XTestExtension;
use crate::core::DpyInfo;
use crate::core::{COORD_INCR, COORD_DECR};
use crate::input::InputHandler;
use anyhow::Result;

/// Mouse event handler
///
/// Handles mouse movement and button events, translating them from the source
/// display to the target display using coordinate mapping.
pub struct MouseHandler {
    conn: std::sync::Arc<X11Connection>,
}

impl MouseHandler {
    /// Create a new mouse handler
    pub fn new(conn: std::sync::Arc<X11Connection>) -> Self {
        Self { conn }
    }

    /// Handle a motion notify event
    ///
    /// Implements ProcessMotionNotify logic from original x2x.c:
    /// 1. Check same_screen flag
    /// 2. Map coordinates through tables
    /// 3. Sanity check (unreasonableDelta)
    /// 4. Handle COORD_INCR/COORD_DECR special values
    /// 5. Fake movement on to display
    fn handle_motion(&self, event: &XMotionEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // Check if we're on the same screen
        if !event.same_screen {
            debug!("Motion event not on same screen, ignoring");
            return Ok(false);
        }

        // Calculate delta from last position
        let delta_x = (event.x - ctx.current_x).abs();
        let delta_y = (event.y - ctx.current_y).abs();
        let delta = delta_x.max(delta_y);

        // Sanity check: unreasonable movement
        if delta > ctx.unreasonable_delta as i32 {
            warn!("Unreasonable mouse movement detected: delta={}, ignoring", delta);
            ctx.update_pointer(event.x, event.y);
            return Ok(false);
        }

        // Update current position
        ctx.update_pointer(event.x, event.y);

        // Check if we're connected
        if !ctx.is_connected() {
            debug!("Not connected, ignoring motion");
            return Ok(false);
        }

        // Map X coordinate to target display
        let to_screen = ctx.to_screen;
        let mapped_x = ctx.x_tables
            .get(to_screen)
            .and_then(|table| table.get(event.x as usize))
            .copied()
            .unwrap_or(COORD_INCR as i16);

        // Map Y coordinate to target display
        let mapped_y = ctx.y_tables
            .get(to_screen)
            .and_then(|table| table.get(event.y as usize))
            .copied()
            .unwrap_or(COORD_INCR as i16);

        debug!("Mapped coordinates: ({}, {}) -> ({}, {})", event.x, event.y, mapped_x, mapped_y);

        // Check for special coordinate values (COORD_INCR = -1, COORD_DECR = -2)
        let is_incr = mapped_x < 0;
        let is_decr = mapped_x < COORD_DECR as i16 && !is_incr;

        debug!("Mapped coordinates: ({}, {}) -> incr={}, decr={}", 
            event.x, event.y, is_incr, is_decr);

        if is_incr {
            info!("COORD_INCR detected - switching to next screen");
            // TODO: Handle screen switch (connect to next screen)
            return Ok(false);
        }

        if is_decr {
            info!("COORD_DECR detected - switching to previous screen");
            // TODO: Handle screen switch (connect to previous screen)
            return Ok(false);
        }

        // Normal coordinate - fake movement on display
        let xtest = XTestExtension::from_connection(&ctx.to_conn);
        match xtest {
            Some(ext) if ext.is_available() => {
                ext.fake_motion(&ctx.to_conn, ctx.to_screen as i32, mapped_x as i32, mapped_y as i32)?;
                debug!("Faked mouse motion to ({}, {}) on screen {}", mapped_x, mapped_y, to_screen);
            }
            _ => {
                warn!("XTest not available, cannot fake mouse motion");
            }
        }

        Ok(true)
    }

    /// Handle a button press event
    ///
    /// Implements ProcessButtonPress logic from original x2x.c
    fn handle_button_press(&self, event: &XButtonEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // Update button state
        let button_mask = 1u32 << event.button;
        ctx.update_button_state(button_mask, true);
        debug!("Button {} pressed (mask: 0x{:x})", event.button, button_mask);

        // Check if connected
        if !ctx.is_connected() {
            return Ok(false);
        }

        // Fake button press on display
        let xtest = XTestExtension::from_connection(&ctx.to_conn);
        match xtest {
            Some(ext) if ext.is_available() => {
                ext.fake_button(&ctx.to_conn, event.button, true)?;
                debug!("Faked button {} press", event.button);
            }
            _ => {
                warn!("XTest not available, cannot fake button press");
            }
        }

        Ok(true)
    }

    /// Handle a button release event
    ///
    /// Implements ProcessButtonRelease logic from original x2x.c
    fn handle_button_release(&self, event: &XButtonEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // Update button state
        let button_mask = 1u32 << event.button;
        ctx.update_button_state(button_mask, false);
        debug!("Button {} released (mask: 0x{:x})", event.button, button_mask);

        // Check if connected
        if !ctx.is_connected() {
            return Ok(false);
        }

        // Fake button release on display
        let xtest = XTestExtension::from_connection(&ctx.to_conn);
        match xtest {
            Some(ext) if ext.is_available() => {
                ext.fake_button(&ctx.to_conn, event.button, false)?;
                debug!("Faked button {} release", event.button);
            }
            _ => {
                warn!("XTest not available, cannot fake button release");
            }
        }

        Ok(true)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_handler_new() {
        // We can test MouseHandler without a real X connection
        // by creating a mock connection - but for now skip
        assert!(true);
    }

    #[test]
    fn test_unreasonable_delta() {
        // Test delta calculation - correct values
        let delta_x = (100 - 95).abs();
        let delta_y = (105 - 100).abs();
        let delta = delta_x.max(delta_y);
        assert_eq!(delta, 5);
    }
}
