//! X11 connection management using x11-dl

use log::{info, warn, debug};

use crate::x11::{X11Error, Window, Atom, Time, ScreenInfo};
use anyhow::{Context, Result};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use std::sync::Arc;
use std::ptr;

// Import x11-dl library
use x11_dl::xlib::{Display, XEvent as XlibEvent, Window as XlibWindow};

/// X11 connection wrapper
pub struct X11Connection {
    display: *mut Display,
    screen: i32,
    xlib: x11_dl::xlib::Xlib,
}

unsafe impl Send for X11Connection {}
unsafe impl Sync for X11Connection {}

impl X11Connection {
    /// Open an X11 connection
    pub fn open(display_name: Option<&str>) -> Result<Self> {
        // Load Xlib
        let xlib = x11_dl::xlib::Xlib::open().context("Failed to load Xlib")?;

        // Open display
        let display_name_cstring = display_name.map(|s| CString::new(s).unwrap());
        let display_name_ptr = display_name_cstring
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(ptr::null());

        let display = unsafe { (xlib.XOpenDisplay)(display_name_ptr) };

        if display.is_null() {
            return Err(X11Error::OpenDisplayFailed(
                display_name.unwrap_or(":0").to_string(),
            )
            .into());
        }

        let screen = unsafe { (xlib.XDefaultScreen)(display) };

        info!(
            "Opened X display: screen={}",
            screen,
        );

        Ok(Self {
            display,
            screen,
            xlib,
        })
    }

    /// Get the raw display pointer
    pub fn display_ptr(&self) -> *mut Display {
        self.display
    }

    /// Get the xlib library
    pub fn xlib(&self) -> &x11_dl::xlib::Xlib {
        &self.xlib
    }

    /// Get the default screen number
    pub fn screen(&self) -> i32 {
        self.screen
    }

    /// Get the root window for the default screen
    pub fn root_window(&self) -> Window {
        unsafe { (self.xlib.XDefaultRootWindow)(self.display) as u64 }
    }

    /// Get the root window for a specific screen
    pub fn root_window_of_screen(&self, screen: i32) -> Window {
        unsafe {
            (self.xlib.XRootWindow)(self.display, screen) as u64
        }
    }

    /// Get the screen width
    pub fn screen_width(&self, screen: i32) -> i32 {
        unsafe { (self.xlib.XDisplayWidth)(self.display, screen) }
    }

    /// Get the screen height
    pub fn screen_height(&self, screen: i32) -> i32 {
        unsafe { (self.xlib.XDisplayHeight)(self.display, screen) }
    }

    /// Flush the output buffer
    pub fn flush(&self) -> Result<()> {
        let result = unsafe { (self.xlib.XFlush)(self.display) };
        if result != 0 {
            Ok(())
        } else {
            Err(X11Error::Generic("XFlush failed".to_string()).into())
        }
    }

    /// Synchronize with server
    pub fn sync(&self, discard: bool) -> Result<()> {
        unsafe { (self.xlib.XSync)(self.display, if discard { 1 } else { 0 }) };
        Ok(())
    }

    /// Check if there are pending events
    pub fn pending(&self) -> i32 {
        unsafe { (self.xlib.XPending)(self.display) }
    }

    /// Get the file descriptor for select()
    pub fn connection_number(&self) -> i32 {
        unsafe { (self.xlib.XConnectionNumber)(self.display) }
    }

    /// Get the next event from the queue (blocking)
    pub fn next_event(&self) -> crate::x11::event::XEvent {
        let mut xevent: XlibEvent = unsafe { std::mem::zeroed() };
        unsafe { (self.xlib.XNextEvent)(self.display, &mut xevent) };
        // TODO: Convert XlibEvent to XEvent properly
        // For now, return GenericEvent as placeholder
        crate::x11::event::XEvent::GenericEvent(crate::x11::event::XGenericEvent { type_: 0 })
    }

    /// Peek at the next event without removing it
    pub fn peek_event(&self) -> Option<crate::x11::event::XEvent> {
        if self.pending() == 0 {
            return None;
        }
        let mut xevent: XlibEvent = unsafe { std::mem::zeroed() };
        unsafe {
            (self.xlib.XPeekEvent)(self.display, &mut xevent);
        }
        // TODO: Convert XlibEvent to XEvent properly
        Some(crate::x11::event::XEvent::GenericEvent(crate::x11::event::XGenericEvent { type_: 0 }))
    }

    /// Get screen information
    pub fn screen_info(&self, screen_num: i32) -> Result<ScreenInfo> {
        if screen_num < 0 || screen_num >= self.screen_count()? {
            return Err(X11Error::InvalidScreen(screen_num).into());
        }

        Ok(ScreenInfo {
            screen_num,
            root: self.root_window_of_screen(screen_num),
            width: self.screen_width(screen_num) as u32,
            height: self.screen_height(screen_num) as u32,
        })
    }

    /// Get the number of screens
    pub fn screen_count(&self) -> Result<i32> {
        let count = unsafe { (self.xlib.XScreenCount)(self.display) };
        Ok(count)
    }

    /// Get the current screen information
    pub fn current_screen_info(&self) -> Result<ScreenInfo> {
        self.screen_info(self.screen)
    }

    /// Query pointer position
    pub fn query_pointer(&self, window: Window) -> Result<PointerInfo> {
        let mut root_return: XlibWindow = 0;
        let mut child_return: XlibWindow = 0;
        let mut root_x_return: c_int = 0;
        let mut root_y_return: c_int = 0;
        let mut win_x_return: c_int = 0;
        let mut win_y_return: c_int = 0;
        let mut mask_return: c_uint = 0;

        let result = unsafe {
            (self.xlib.XQueryPointer)(
                self.display,
                window as XlibWindow,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            )
        };

        if result == 0 {
            return Err(X11Error::Generic("XQueryPointer failed".to_string()).into());
        }

        Ok(PointerInfo {
            root: root_return as u64,
            child: child_return as u64,
            root_x: root_x_return as i32,
            root_y: root_y_return as i32,
            win_x: win_x_return as i32,
            win_y: win_y_return as i32,
            mask: mask_return,
        })
    }

    /// Warp pointer to position
    pub fn warp_pointer(
        &self,
        src_window: Option<Window>,
        dst_window: Window,
        src_x: i32,
        src_y: i32,
        src_width: u32,
        src_height: u32,
        dst_x: i32,
        dst_y: i32,
    ) -> Result<()> {
        let src_xlib_window = src_window.unwrap_or(0) as XlibWindow;
        unsafe {
            (self.xlib.XWarpPointer)(
                self.display,
                src_xlib_window,
                dst_window as XlibWindow,
                src_x as c_int,
                src_y as c_int,
                src_width as c_uint,
                src_height as c_uint,
                dst_x as c_int,
                dst_y as c_int,
            );
        }
        Ok(())
    }
}

impl Drop for X11Connection {
    fn drop(&mut self) {
        if !self.display.is_null() {
            unsafe {
                (self.xlib.XCloseDisplay)(self.display);
            }
        }
    }
}

/// Pointer information from XQueryPointer
#[derive(Debug, Clone)]
pub struct PointerInfo {
    pub root: u64,
    pub child: u64,
    pub root_x: i32,
    pub root_y: i32,
    pub win_x: i32,
    pub win_y: i32,
    pub mask: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires X server
    fn test_open_display() {
        let conn = X11Connection::open(None);
        assert!(conn.is_ok());
    }
}
