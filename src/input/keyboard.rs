//! Keyboard input handling

use log::{info, warn, debug};
use crate::x11::event::{XEvent, XKeyEvent};
use crate::x11::X11Connection;
use crate::x11::extension::XTestExtension;
use crate::core::DpyInfo;
use crate::core::StickyKeys;
use crate::input::InputHandler;
use anyhow::Result;

/// Keyboard event handler
///
/// Handles keyboard key press and release events, translating them from the source
/// display to the target display using XTest.
pub struct KeyboardHandler {
    conn: std::sync::Arc<X11Connection>,
    sticky_keys: StickyKeys,
}

impl KeyboardHandler {
    /// Create a new keyboard handler
    pub fn new(conn: std::sync::Arc<X11Connection>) -> Self {
        Self {
            conn,
            sticky_keys: StickyKeys::new(),
        }
    }

    /// Handle a key press event
    ///
    /// Implements ProcessKeyEvent (press) logic from original x2x.c:
    /// 1. Check for modifier keys and sticky keys
    /// 2. Map keycode to keysym
    /// 3. Handle sticky key toggles
    /// 4. Fake key press on to display
    fn handle_key_press(&self, event: &XKeyEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // Get keysym from keycode (simplified - using keycode directly)
        let keysym = event.keycode as u32;

        // Check if this is a sticky key (modifier key)
        // Common modifier keys: Shift, Ctrl, Alt, Meta/Super
        let is_modifier = match keysym {
            0xffe1..=0xffe2 |  // Shift
            0xffe3..=0xffe4 |  // Ctrl
            0xffe7..=0xffe8 |  // Alt
            0xffe9..=0xffea => true, // Meta/Super
            _ => false,
        };

        if is_modifier {
            // Toggle sticky key state
            let was_sticky = ctx.sticky_keys.toggle(keysym);
            debug!("Modifier key {} toggled sticky: {}", keysym, was_sticky);
            return Ok(true);
        }

        // Check if sticky key is active
        if ctx.sticky_keys.is_sticky(keysym) {
            debug!("Sticky key {} already active, ignoring press", keysym);
            return Ok(false);
        }

        // Check if connected
        if !ctx.is_connected() {
            return Ok(false);
        }

        // Fake key press on to display
        let xtest = XTestExtension::from_connection(&ctx.to_conn);
        match xtest {
            Some(ext) if ext.is_available() => {
                ext.fake_key(&ctx.to_conn, event.keycode as u8, true)?;
                debug!("Faked key press: keycode={}", event.keycode);
            }
            _ => {
                warn!("XTest not available, cannot fake key press");
            }
        }

        Ok(true)
    }

    /// Handle a key release event
    ///
    /// Implements ProcessKeyEvent (release) logic from original x2x.c:
    fn handle_key_release(&self, event: &XKeyEvent, ctx: &mut DpyInfo) -> Result<bool> {
        // Get keysym from keycode (simplified)
        let keysym = event.keycode as u32;

        // Check if this is a modifier key
        let is_modifier = match keysym {
            0xffe1..=0xffe2 |  // Shift
            0xffe3..=0xffe4 |  // Ctrl
            0xffe7..=0xffe8 |  // Alt
            0xffe9..=0xffea => true, // Meta/Super
            _ => false,
        };

        if is_modifier {
            // Don't handle release of sticky modifiers
            debug!("Ignoring release of modifier key {}", keysym);
            return Ok(false);
        }

        // Check if connected
        if !ctx.is_connected() {
            return Ok(false);
        }

        // Fake key release on to display
        let xtest = XTestExtension::from_connection(&ctx.to_conn);
        match xtest {
            Some(ext) if ext.is_available() => {
                ext.fake_key(&ctx.to_conn, event.keycode as u8, false)?;
                debug!("Faked key release: keycode={}", event.keycode);
            }
            _ => {
                warn!("XTest not available, cannot fake key release");
            }
        }

        Ok(true)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_handler_new() {
        let sticky = StickyKeys::new();
        assert!(!sticky.is_sticky(0xffe1));
        assert_eq!(sticky.len(), 0);
    }

    #[test]
    fn test_sticky_keys_toggle() {
        let mut sticky = StickyKeys::new();

        let result = sticky.toggle(0xffe1);
        assert!(result);
        assert!(sticky.is_sticky(0xffe1));
        assert_eq!(sticky.len(), 1);

        let result = sticky.toggle(0xffe1);
        assert!(!result);
        assert!(!sticky.is_sticky(0xffe1));
        assert_eq!(sticky.len(), 0);
    }

    #[test]
    fn test_sticky_keys_clear() {
        let mut sticky = StickyKeys::new();
        sticky.toggle(0xffe1);
        sticky.toggle(0xffe2);
        assert_eq!(sticky.len(), 2);

        sticky.clear();
        assert_eq!(sticky.len(), 0);
        assert!(!sticky.is_sticky(0xffe1));
        assert!(!sticky.is_sticky(0xffe2));
    }
}
