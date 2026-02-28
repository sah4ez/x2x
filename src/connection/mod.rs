//! Connection management between X displays

use crate::x11::{X11Connection, X11Error, Window};
use crate::core::DpyInfo;
use anyhow::{Context, Result};
use log::{info, warn, debug, error};
use x11_dl::xlib::{
    self,
    Display,
    CurrentTime,
    PointerRoot,
    GrabModeAsync,
    PointerMotionMask,
    ButtonPressMask,
    ButtonReleaseMask,
    CWX,
    CWY,
    CWWidth,
    CWHeight,
    CWStackMode,
    Above,
};

use std::os::raw::{c_int, c_uint, c_ulong};
use std::ptr;
use std::sync::Arc;

/// Connection manager for X displays
///
/// Manages connection/disconnection between two X displays including:
/// - Trigger window management
/// - Pointer grabbing
/// - Keyboard grabbing
/// - Focus management
/// - Input event routing
pub struct ConnectionManager {
    /// Connection to "from" display
    from_conn: Arc<X11Connection>,
    /// Connection to "to" display
    to_conn: Arc<X11Connection>,
    /// Trigger window on from display
    trigger_window: Window,
    /// Big status window on from display
    big_window: Option<Window>,
    /// Current connection mode
    mode: ConnectionMode,
    /// Whether auto-up is enabled
    auto_up: bool,
    /// Event mask for trigger window
    event_mask: c_uint,
    /// Window attributes for creating windows
    window_attr: WindowAttributes,
}

/// Connection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMode {
    /// Disconnected - cursor stays on from display
    Disconnected,
    /// Connected - cursor can cross to to display
    Connected,
}

/// Window attributes for creating trigger and status windows
#[derive(Debug, Clone)]
pub struct WindowAttributes {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window background color (pixel value)
    pub background_pixel: u64,
    /// Window border width
    pub border_width: u32,
    /// Window border color (pixel value)
    pub border_pixel: u64,
    /// Override redirect flag
    pub override_redirect: bool,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            background_pixel: 0,
            border_width: 0,
            border_pixel: 0,
            override_redirect: true,
        }
    }
}

/// Focus state for saving/restoring
#[derive(Debug, Clone)]
pub struct FocusState {
    /// Focused window
    pub window: Window,
    /// Revert to window
    pub revert_to: c_int,
}

impl Default for FocusState {
    fn default() -> Self {
        Self {
            window: 0,
            revert_to: 0,
        }
    }
}

impl ConnectionManager {
    /// Create a new connection manager
    ///
    /// # Arguments
    ///
    /// * `from_conn` - Connection to source display
    /// * `to_conn` - Connection to target display
    /// * `trigger_window` - Trigger window ID
    /// * `event_mask` - Event mask for trigger window
    /// * `auto_up` - Enable auto keyboard state management
    pub fn new(
        from_conn: Arc<X11Connection>,
        to_conn: Arc<X11Connection>,
        trigger_window: Window,
        event_mask: c_uint,
        auto_up: bool,
    ) -> Self {
        Self {
            from_conn,
            to_conn,
            trigger_window,
            big_window: None,
            mode: ConnectionMode::Disconnected,
            auto_up,
            event_mask,
            window_attr: WindowAttributes::default(),
        }
    }

    /// Connect to the other display
    ///
    /// Implements DoConnect logic from original x2x.c:
    /// 1. Restore keyboard state if auto_up is enabled
    /// 2. Save current focus on from display
    /// 3. Set focus to PointerRoot
    /// 4. Map big status window (if any)
    /// 5. Grab pointer and keyboard
    /// 6. Enable motion events on trigger window
    /// 7. Set mode to Connected
    ///
    /// This is called when the cursor crosses the screen edge.
    pub fn connect(&mut self, from_focus: &mut FocusState, to_focus: &mut FocusState) -> Result<()> {
        info!("Connecting displays...");

        let from_display = self.from_conn.display_ptr();
        let to_display = self.to_conn.display_ptr();
        let from_xlib = self.from_conn.xlib();
        let to_xlib = self.to_conn.xlib();

        // Restore keyboard state if auto_up is enabled
        if self.auto_up {
            debug!("Restoring keyboard state...");
            // TODO: KeyboardState(from_display) - not yet implemented
        }

        // Set mode to connected
        self.mode = ConnectionMode::Connected;

        // Save current focus on from display
        unsafe {
            let mut focus_window: u64 = 0;
            let mut revert_to: c_int = 0;
            (from_xlib.XGetInputFocus)(from_display, &mut focus_window, &mut revert_to);

            from_focus.window = focus_window;
            from_focus.revert_to = revert_to;

            debug!("Saved from focus: window=0x{:x}, revert={}", focus_window, revert_to);
        }

        // Set focus to PointerRoot on from display
        unsafe {
            (from_xlib.XSetInputFocus)(
                from_display,
                PointerRoot as u64,
                0,
                CurrentTime as u64,
            );
            (from_xlib.XSync)(from_display, 0);
        }

        // Flush to display
        unsafe {
            (to_xlib.XFlush)(to_display);
        }

        // Restore focus on to display if saved
        if to_focus.window != 0 {
            unsafe {
                (to_xlib.XSetInputFocus)(
                    to_display,
                    to_focus.window,
                    to_focus.revert_to,
                    CurrentTime as u64,
                );
                (to_xlib.XSync)(to_display, 0);
                debug!("Restored to focus: window=0x{:x}, revert={}",
                    to_focus.window, to_focus.revert_to);
            }
        }

        // Map big status window (if any)
        if let Some(big_window) = self.big_window {
            unsafe {
                (from_xlib.XMapRaised)(from_display, big_window as u64);
                debug!("Mapped big status window 0x{:x}", big_window);
            }
        }

        // Grab pointer
        unsafe {
            let result = (from_xlib.XGrabPointer)(
                from_display,
                self.trigger_window as u64,
                1, // owner_events
                (PointerMotionMask | ButtonPressMask | ButtonReleaseMask) as u32,
                GrabModeAsync,
                GrabModeAsync,
                0, // confine_to
                0, // cursor
                CurrentTime as u64,
            );

            if result != 0 {
                warn!("Failed to grab pointer: {}", result);
            } else {
                debug!("Grabbed pointer on trigger window 0x{:x}", self.trigger_window);
            }
        }

        // Grab keyboard
        unsafe {
            let result = (from_xlib.XGrabKeyboard)(
                from_display,
                self.trigger_window as u64,
                1, // owner_events
                GrabModeAsync,
                GrabModeAsync,
                CurrentTime as u64,
            );

            if result != 0 {
                warn!("Failed to grab keyboard: {}", result);
            } else {
                debug!("Grabbed keyboard on trigger window 0x{:x}", self.trigger_window);
            }
        }

        // Enable motion events on trigger window
        unsafe {
            let event_mask = (self.event_mask as i64) | (PointerMotionMask as i64);
            (from_xlib.XSelectInput)(
                from_display,
                self.trigger_window as u64,
                event_mask,
            );
        }

        // Sync to ensure all operations are complete
        unsafe {
            (from_xlib.XSync)(from_display, 0);
        }

        info!("Connected to other display");
        Ok(())
    }

    /// Disconnect from the other display
    ///
    /// Implements DoDisconnect logic from original x2x.c:
    /// 1. Save current focus on to display
    /// 2. Set focus to PointerRoot
    /// 3. Restore saved focus on from display
    /// 4. Unmap big status window
    /// 5. Ungrab keyboard and pointer
    /// 6. Set clipboard owner if selection is active
    /// 7. Clear fake events if auto_up is enabled
    /// 8. Set mode to Disconnected
    ///
    /// This is called when the cursor crosses back to the from display.
    pub fn disconnect(
        &mut self,
        from_focus: &mut FocusState,
        to_focus: &mut FocusState,
        clipboard_owner: Option<Window>,
    ) -> Result<()> {
        info!("Disconnecting displays...");

        let from_display = self.from_conn.display_ptr();
        let to_display = self.to_conn.display_ptr();
        let from_xlib = self.from_conn.xlib();
        let to_xlib = self.to_conn.xlib();

        // Set mode to disconnected
        self.mode = ConnectionMode::Disconnected;

        // Save current focus on to display
        unsafe {
            let mut focus_window: u64 = 0;
            let mut revert_to: c_int = 0;
            (to_xlib.XGetInputFocus)(to_display, &mut focus_window, &mut revert_to);

            to_focus.window = focus_window;
            to_focus.revert_to = revert_to;

            debug!("Saved to focus: window=0x{:x}, revert={}", focus_window, revert_to);
        }

        // Set focus to PointerRoot on to display
        unsafe {
            (to_xlib.XSetInputFocus)(
                to_display,
                PointerRoot as u64,
                0,
                CurrentTime as u64,
            );
            (to_xlib.XSync)(to_display, 0);
        }

        // Flush from display
        unsafe {
            (from_xlib.XFlush)(from_display);
        }

        // Restore saved focus on from display
        if from_focus.window != 0 {
            unsafe {
                (from_xlib.XSetInputFocus)(
                    from_display,
                    from_focus.window,
                    from_focus.revert_to,
                    CurrentTime as u64,
                );
                (from_xlib.XSync)(from_display, 0);
                debug!("Restored from focus: window=0x{:x}, revert={}",
                    from_focus.window, from_focus.revert_to);
            }
        }

        // Unmap big status window
        if let Some(big_window) = self.big_window {
            unsafe {
                (from_xlib.XUnmapWindow)(from_display, big_window as u64);
                debug!("Unmapped big status window 0x{:x}", big_window);
            }
        }

        // Ungrab keyboard
        unsafe {
            (from_xlib.XUngrabKeyboard)(from_display, CurrentTime as u64);
            debug!("Ungrabbed keyboard");
        }

        // Ungrab pointer
        unsafe {
            (from_xlib.XUngrabPointer)(from_display, CurrentTime as u64);
            debug!("Ungrabbed pointer");
        }

        // Restore event mask on trigger window
        unsafe {
            (from_xlib.XSelectInput)(
                from_display,
                self.trigger_window as u64,
                self.event_mask as i64,
            );
        }

        // Set clipboard owner if provided
        if let Some(owner_window) = clipboard_owner {
            // TODO: XSetSelectionOwner - requires clipboard module integration
            debug!("Setting clipboard owner to 0x{:x}", owner_window);
        }

        // Clear fake events if auto_up is enabled
        if self.auto_up {
            debug!("Clearing fake events...");
            // TODO: FakeThingsUp - not yet implemented
        }

        // Sync to ensure all operations are complete
        unsafe {
            (from_xlib.XSync)(from_display, 0);
        }

        info!("Disconnected from other display");
        Ok(())
    }

    /// Get current connection mode
    pub fn mode(&self) -> ConnectionMode {
        self.mode
    }

    /// Check if currently connected
    pub fn is_connected(&self) -> bool {
        self.mode == ConnectionMode::Connected
    }

    /// Set the big status window
    pub fn set_big_window(&mut self, window: Option<Window>) {
        self.big_window = window;
        if let Some(w) = window {
            debug!("Set big status window: 0x{:x}", w);
        }
    }

    /// Get the big status window
    pub fn big_window(&self) -> Option<Window> {
        self.big_window
    }

    /// Set window attributes for creating windows
    pub fn set_window_attributes(&mut self, attr: WindowAttributes) {
        self.window_attr = attr;
    }

    /// Get window attributes
    pub fn window_attributes(&self) -> &WindowAttributes {
        &self.window_attr
    }

    /// Get trigger window
    pub fn trigger_window(&self) -> Window {
        self.trigger_window
    }

    /// Get "from" connection
    pub fn from_conn(&self) -> &Arc<X11Connection> {
        &self.from_conn
    }

    /// Get "to" connection
    pub fn to_conn(&self) -> &Arc<X11Connection> {
        &self.to_conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_mode() {
        assert_eq!(ConnectionMode::Disconnected, ConnectionMode::Disconnected);
        assert_eq!(ConnectionMode::Connected, ConnectionMode::Connected);
        assert_ne!(ConnectionMode::Disconnected, ConnectionMode::Connected);
    }

    #[test]
    fn test_window_attributes_default() {
        let attr = WindowAttributes::default();

        assert_eq!(attr.width, 1);
        assert_eq!(attr.height, 1);
        assert_eq!(attr.background_pixel, 0);
        assert_eq!(attr.border_width, 0);
        assert_eq!(attr.border_pixel, 0);
        assert!(attr.override_redirect);
    }

    #[test]
    fn test_focus_state_default() {
        let state = FocusState::default();

        assert_eq!(state.window, 0);
        assert_eq!(state.revert_to, 0);
    }
}
