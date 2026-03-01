//! X11 connection management using x11-dl

use log::{debug, info, warn};

use crate::x11::{Atom, ScreenInfo, Window, X11Error};

pub type Time = u64;


use anyhow::{Context, Result};
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_uint, c_char, c_ulong, c_long};
use std::ptr;
use std::sync::Arc;

// Import x11-dl library
use x11_dl::xlib::{
    Display, Window as XlibWindow, XEvent as XlibEvent,
    VisualID, Cursor, GC, Pixmap, Colormap, XGCValues, XSetWindowAttributes, Visual,
    CWOverrideRedirect, CWBackPixel, CWBorderPixel, CWEventMask,
    KeyPressMask, KeyReleaseMask, ButtonPressMask, ButtonReleaseMask,
    EnterWindowMask, LeaveWindowMask, PointerMotionMask,
    ExposureMask, StructureNotifyMask, PropertyChangeMask,
    PropModeReplace, PropModeAppend, PropModePrepend,
    InputOutput, InputOnly, CopyFromParent,
    CurrentTime,
};

/// Window class type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowClass {
    InputOutput,
    InputOnly,
}

/// Set window attributes
#[derive(Debug, Clone)]
pub struct SetWindowAttributes {
    pub background_pixmap: Option<Pixmap>,
    pub background_pixel: Option<c_ulong>,
    pub border_pixmap: Option<Pixmap>,
    pub border_pixel: Option<c_ulong>,
    pub bit_gravity: Option<i32>,
    pub win_gravity: Option<i32>,
    pub backing_store: Option<i32>,
    pub backing_planes: Option<c_ulong>,
    pub backing_pixel: Option<c_ulong>,
    pub save_under: Option<bool>,
    pub event_mask: Option<c_ulong>,
    pub do_not_propagate_mask: Option<c_ulong>,
    pub override_redirect: Option<bool>,
    pub colormap: Option<Colormap>,
    pub cursor: Option<Cursor>,
}

impl Default for SetWindowAttributes {
    fn default() -> Self {
        Self {
            background_pixmap: None,
            background_pixel: None,
            border_pixmap: None,
            border_pixel: None,
            bit_gravity: None,
            win_gravity: None,
            backing_store: None,
            backing_planes: None,
            backing_pixel: None,
            save_under: None,
            event_mask: None,
            do_not_propagate_mask: None,
            override_redirect: None,
            colormap: None,
            cursor: None,
        }
    }
}

/// Property mode for change_property
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropMode {
    Replace,
    Prepend,
    Append,
}

/// Graphics context values
#[derive(Debug, Clone)]
pub struct GCValues {
    pub function: Option<i32>,
    pub plane_mask: Option<c_ulong>,
    pub foreground: Option<c_ulong>,
    pub background: Option<c_ulong>,
    pub line_width: Option<i32>,
    pub line_style: Option<i32>,
    pub cap_style: Option<i32>,
    pub join_style: Option<i32>,
    pub fill_style: Option<i32>,
    pub fill_rule: Option<i32>,
    pub tile: Option<Pixmap>,
    pub stipple: Option<Pixmap>,
    pub tile_stipple_x_origin: Option<i32>,
    pub tile_stipple_y_origin: Option<i32>,
    pub font: Option<c_ulong>,
    pub subwindow_mode: Option<i32>,
    pub graphics_exposures: Option<bool>,
    pub clip_x_origin: Option<i32>,
    pub clip_y_origin: Option<i32>,
    pub clip_mask: Option<Pixmap>,
    pub dash_offset: Option<i32>,
    pub dashes: Option<i32>,
    pub arc_mode: Option<i32>,
}

impl Default for GCValues {
    fn default() -> Self {
        Self {
            function: None,
            plane_mask: None,
            foreground: None,
            background: None,
            line_width: None,
            line_style: None,
            cap_style: None,
            join_style: None,
            fill_style: None,
            fill_rule: None,
            tile: None,
            stipple: None,
            tile_stipple_x_origin: None,
            tile_stipple_y_origin: None,
            font: None,
            subwindow_mode: None,
            graphics_exposures: None,
            clip_x_origin: None,
            clip_y_origin: None,
            clip_mask: None,
            dash_offset: None,
            dashes: None,
            arc_mode: None,
        }
    }
}

/// Text extents information
#[derive(Debug, Clone)]
pub struct TextExtents {
    pub font_ascent: i32,
    pub font_descent: i32,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
    pub overall_ascent: i32,
    pub overall_descent: i32,
}

/// Grab mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrabMode {
    Sync,
    Async,
}

/// Grab status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrabStatus {
    Success,
    AlreadyGrabbed,
    InvalidTime,
    NotViewable,
    Frozen,
}

/// Revert to mode for input focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevertTo {
    Parent,
    PointerRoot,
    None,
}

/// Focus state
#[derive(Debug, Clone)]
pub struct FocusState {
    pub focus: Window,
    pub revert_to: RevertTo,
}

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
            return Err(
                X11Error::OpenDisplayFailed(display_name.unwrap_or(":0").to_string()).into(),
            );
        }

        let screen = unsafe { (xlib.XDefaultScreen)(display) };

        info!("Opened X display: screen={}", screen,);

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
        unsafe { (self.xlib.XRootWindow)(self.display, screen) as u64 }
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
        unsafe { crate::x11::event::XEvent::from_xlib_event(&xevent) }
    }

    /// Peek at the next event without removing it
    pub fn peek_event(&self) -> crate::x11::event::XEvent {
        let mut xevent: XlibEvent = unsafe { std::mem::zeroed() };
        unsafe {
            (self.xlib.XPeekEvent)(self.display, &mut xevent);
        }
        unsafe { crate::x11::event::XEvent::from_xlib_event(&xevent) }
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

    /// Create a new window
    pub fn create_window(
        &self,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        depth: i32,
        class: WindowClass,
        visual: VisualID,
        attr_mask: c_ulong,
        attributes: &SetWindowAttributes,
    ) -> Result<Window> {
        let class_value = match class {
            WindowClass::InputOutput => InputOutput as u32,
            WindowClass::InputOnly => InputOnly as u32,
        };

        let visual_ptr = if depth == CopyFromParent {
            ptr::null_mut()
        } else {
            &visual as *const VisualID as *mut Visual
        };

        // Build XSetWindowAttributes
        let mut xattr: x11_dl::xlib::XSetWindowAttributes = unsafe { std::mem::zeroed() };

        if let Some(bg_pixel) = attributes.background_pixel {
            xattr.background_pixel = bg_pixel;
        }
        if let Some(border_pixel) = attributes.border_pixel {
            xattr.border_pixel = border_pixel;
        }
        if let Some(event_mask) = attributes.event_mask {
            xattr.event_mask = event_mask as i64;
        }
        if let Some(override_redirect) = attributes.override_redirect {
            xattr.override_redirect = if override_redirect { 1 } else { 0 };
        }

        unsafe {
            let window = (self.xlib.XCreateWindow)(
                self.display,
                parent as XlibWindow,
                x,
                y,
                width,
                height,
                border_width,
                depth,
                class_value,
                visual_ptr,
                attr_mask,
                &mut xattr,
            );

            if window == 0 {
                return Err(X11Error::Generic("XCreateWindow failed".to_string()).into());
            }

            Ok(window as u64)
        }
    }

    /// Map a window (make it visible)
    pub fn map_window(&self, window: Window) -> Result<()> {
        unsafe {
            (self.xlib.XMapWindow)(self.display, window as XlibWindow);
        }
        Ok(())
    }

    /// Unmap a window (make it invisible)
    pub fn unmap_window(&self, window: Window) -> Result<()> {
        unsafe {
            (self.xlib.XUnmapWindow)(self.display, window as XlibWindow);
        }
        Ok(())
    }

    /// Destroy a window
    pub fn destroy_window(&self, window: Window) -> Result<()> {
        unsafe {
            (self.xlib.XDestroyWindow)(self.display, window as XlibWindow);
        }
        Ok(())
    }

    /// Intern an atom (get or create)
    pub fn intern_atom(&self, name: &str, only_if_exists: bool) -> Result<Atom> {
        let name_cstring = CString::new(name).context("Invalid atom name")?;

        unsafe {
            let atom = (self.xlib.XInternAtom)(
                self.display,
                name_cstring.as_ptr(),
                if only_if_exists { 1 } else { 0 },
            );

            if atom == 0 {
                return Err(X11Error::Generic("XInternAtom failed".to_string()).into());
            }

            Ok(atom as u32)
        }
    }

    /// Get atom name
    pub fn get_atom_name(&self, atom: Atom) -> Result<String> {
        unsafe {
            let name_ptr = (self.xlib.XGetAtomName)(self.display, atom as u64);

            if name_ptr.is_null() {
                return Err(X11Error::Generic("XGetAtomName failed".to_string()).into());
            }

            let name = CStr::from_ptr(name_ptr).to_string_lossy().to_string();
            (self.xlib.XFree)(name_ptr as *mut _);

            Ok(name)
        }
    }

    /// Change window property
    pub fn change_property(
        &self,
        window: Window,
        property: Atom,
        type_: Atom,
        format: i32,
        mode: PropMode,
        data: &[u8],
    ) -> Result<()> {
        let prop_mode = match mode {
            PropMode::Replace => PropModeReplace,
            PropMode::Prepend => PropModePrepend,
            PropMode::Append => PropModeAppend,
        };

        let nelements = data.len() / (format as usize / 8);

        unsafe {
            (self.xlib.XChangeProperty)(
                self.display,
                window as XlibWindow,
                property as u64,
                type_ as u64,
                format,
                prop_mode as i32,
                data.as_ptr() as *const u8,
                nelements as i32,
            );
        }

        Ok(())
    }

    /// Get window property
    pub fn get_property(
        &self,
        window: Window,
        property: Atom,
        type_: Atom,
        offset: c_long,
        length: c_long,
        delete: bool,
    ) -> Result<(Atom, u8, Vec<u8>)> {
        let mut actual_type_return: u64 = 0;
        let mut actual_format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut bytes_after_return: c_ulong = 0;
        let mut prop_return: *mut u8 = ptr::null_mut();

        unsafe {
            let result = (self.xlib.XGetWindowProperty)(
                self.display,
                window as XlibWindow,
                property as u64,
                offset,
                length,
                if delete { 1 } else { 0 },
                type_ as u64,
                &mut actual_type_return,
                &mut actual_format_return,
                &mut nitems_return,
                &mut bytes_after_return,
                &mut prop_return,
            );

            if result != 0 || prop_return.is_null() {
                return Err(X11Error::Generic("XGetWindowProperty failed".to_string()).into());
            }

            let format_size = (actual_format_return / 8) as usize;
            let data_len = (nitems_return * format_size as c_ulong) as usize;
            let data = Vec::from_raw_parts(prop_return, data_len, data_len);

            Ok((actual_type_return as u32, actual_format_return as u8, data))
        }
    }

    /// Delete a window property
    pub fn delete_property(&self, window: Window, property: Atom) -> Result<()> {
        unsafe {
            (self.xlib.XDeleteProperty)(self.display, window as XlibWindow, property as u64);
        }
        Ok(())
    }

    /// Send an event to a window
    pub fn send_event(
        &self,
        window: Window,
        propagate: bool,
        event_mask: i64,
        event: &XlibEvent,
    ) -> Result<()> {
        let mut xevent: XlibEvent = unsafe { std::mem::zeroed() };
        xevent = *event;

        unsafe {
            (self.xlib.XSendEvent)(
                self.display,
                window as XlibWindow,
                if propagate { 1 } else { 0 },
                event_mask,
                &mut xevent,
            );
        }

        Ok(())
    }

    /// Create a graphics context
    pub fn create_gc(
        &self,
        drawable: Window,
        values: &GCValues,
    ) -> Result<GC> {
        let mut xvalues: x11_dl::xlib::XGCValues = unsafe { std::mem::zeroed() };
        let mut mask: c_ulong = 0;

        // Build mask and values from GCValues
        if let Some(fg) = values.foreground {
            mask |= x11_dl::xlib::GCForeground as c_ulong;
            xvalues.foreground = fg;
        }
        if let Some(bg) = values.background {
            mask |= x11_dl::xlib::GCBackground as c_ulong;
            xvalues.background = bg;
        }
        if let Some(font) = values.font {
            mask |= x11_dl::xlib::GCFont as c_ulong;
            xvalues.font = font;
        }
        if let Some(func) = values.function {
            mask |= x11_dl::xlib::GCFunction as c_ulong;
            xvalues.function = func as i32;
        }

        unsafe {
            let gc = (self.xlib.XCreateGC)(
                self.display,
                drawable as XlibWindow,
                mask,
                &mut xvalues,
            );

            if gc.is_null() {
                return Err(X11Error::Generic("XCreateGC failed".to_string()).into());
            }

            Ok(gc)
        }
    }

    /// Free a graphics context
    pub fn free_gc(&self, gc: GC) -> Result<()> {
        unsafe {
            (self.xlib.XFreeGC)(self.display, gc);
        }
        Ok(())
    }

    /// Load a font by name
    pub fn load_font(&self, font_name: &str) -> Result<c_ulong> {
        let font_name_cstring = CString::new(font_name).context("Invalid font name")?;

        unsafe {
            let font = (self.xlib.XLoadFont)(self.display, font_name_cstring.as_ptr());

            if font == 0 {
                return Err(X11Error::Generic("XLoadFont failed".to_string()).into());
            }

            Ok(font)
        }
    }

    /// Free a font
    pub fn free_font(&self, font: c_ulong) -> Result<()> {
        unsafe {
            (self.xlib.XFreeFont)(self.display, font as *mut _);
        }
        Ok(())
    }

    /// Get text extents
    pub fn text_extents(&self, font: c_ulong, text: &str) -> Result<TextExtents> {
        let text_cstring = CString::new(text).context("Invalid text")?;

        let mut direction_return: i32 = 0;
        let mut font_ascent_return: i32 = 0;
        let mut font_descent_return: i32 = 0;
        let mut overall_return: x11_dl::xlib::XCharStruct = unsafe { std::mem::zeroed() };

        unsafe {
            (self.xlib.XTextExtents)(
                font as *mut _,
                text_cstring.as_ptr() as *const c_char,
                text.len() as i32,
                &mut direction_return,
                &mut font_ascent_return,
                &mut font_descent_return,
                &mut overall_return,
            );
        }

        Ok(TextExtents {
            font_ascent: font_ascent_return,
            font_descent: font_descent_return,
            overall_width: overall_return.width as i32,
            overall_left: overall_return.lbearing as i32,
            overall_right: overall_return.rbearing as i32,
            overall_ascent: overall_return.ascent as i32,
            overall_descent: overall_return.descent as i32,
        })
    }

    /// Draw text
    pub fn draw_text(
        &self,
        drawable: Window,
        gc: GC,
        x: i32,
        y: i32,
        text: &str,
    ) -> Result<()> {
        let text_cstring = CString::new(text).context("Invalid text")?;

        unsafe {
            (self.xlib.XDrawString)(
                self.display,
                drawable as XlibWindow,
                gc,
                x,
                y,
                text_cstring.as_ptr() as *const c_char,
                text.len() as i32,
            );
        }

        Ok(())
    }

    /// Clear a window area
    pub fn clear_area(
        &self,
        window: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        exposures: bool,
    ) -> Result<()> {
        unsafe {
            (self.xlib.XClearArea)(
                self.display,
                window as XlibWindow,
                x,
                y,
                width,
                height,
                if exposures { 1 } else { 0 },
            );
        }

        Ok(())
    }

    /// Grab the pointer
    pub fn grab_pointer(
        &self,
        grab_window: Window,
        owner_events: bool,
        event_mask: u64,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        confine_to: Window,
        cursor: Cursor,
        time: Time,
    ) -> Result<GrabStatus> {
        unsafe {
            let result = (self.xlib.XGrabPointer)(
                self.display,
                grab_window as XlibWindow,
                if owner_events { 1 } else { 0 },
                event_mask as u32,
                pointer_mode as i32,
                keyboard_mode as i32,
                confine_to as XlibWindow,
                cursor,
                time,
            );

            match result {
                0 => Ok(GrabStatus::Success),
                1 => Ok(GrabStatus::AlreadyGrabbed),
                2 => Ok(GrabStatus::InvalidTime),
                3 => Ok(GrabStatus::NotViewable),
                4 => Ok(GrabStatus::Frozen),
                _ => Err(X11Error::Generic(format!("XGrabPointer returned {}", result)).into()),
            }
        }
    }

    /// Ungrab the pointer
    pub fn ungrab_pointer(&self, time: Time) -> Result<()> {
        unsafe {
            (self.xlib.XUngrabPointer)(self.display, time);
        }
        Ok(())
    }

    /// Grab the keyboard
    pub fn grab_keyboard(
        &self,
        grab_window: Window,
        owner_events: bool,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        time: Time,
    ) -> Result<GrabStatus> {
        unsafe {
            let result = (self.xlib.XGrabKeyboard)(
                self.display,
                grab_window as XlibWindow,
                if owner_events { 1 } else { 0 },
                pointer_mode as i32,
                keyboard_mode as i32,
                time,
            );

            match result {
                0 => Ok(GrabStatus::Success),
                1 => Ok(GrabStatus::AlreadyGrabbed),
                2 => Ok(GrabStatus::InvalidTime),
                3 => Ok(GrabStatus::NotViewable),
                4 => Ok(GrabStatus::Frozen),
                _ => Err(X11Error::Generic(format!("XGrabKeyboard returned {}", result)).into()),
            }
        }
    }

    /// Ungrab the keyboard
    pub fn ungrab_keyboard(&self, time: Time) -> Result<()> {
        unsafe {
            (self.xlib.XUngrabKeyboard)(self.display, time);
        }
        Ok(())
    }

    /// Set input focus
    pub fn set_input_focus(
        &self,
        focus: Window,
        revert_to: RevertTo,
        time: Time,
    ) -> Result<()> {
        unsafe {
            (self.xlib.XSetInputFocus)(
                self.display,
                focus as XlibWindow,
                revert_to as i32,
                time,
            );
        }
        Ok(())
    }

    /// Get input focus
    pub fn get_input_focus(&self) -> Result<FocusState> {
        let mut focus_return: XlibWindow = 0;
        let mut revert_to_return: i32 = 0;

        unsafe {
            (self.xlib.XGetInputFocus)(
                self.display,
                &mut focus_return,
                &mut revert_to_return,
            );
        }

        let revert_to = match revert_to_return {
            0 => RevertTo::Parent,
            1 => RevertTo::PointerRoot,
            2 => RevertTo::None,
            _ => return Err(X11Error::Generic("Invalid revert_to value".to_string()).into()),
        };

        Ok(FocusState {
            focus: focus_return as u64,
            revert_to,
        })
    }

    /// Set selection owner
    pub fn set_selection_owner(
        &self,
        owner: Window,
        selection: Atom,
        time: Time,
    ) -> Result<bool> {
        unsafe {
            let result = (self.xlib.XSetSelectionOwner)(
                self.display,
                owner as XlibWindow,
                selection as u64,
                time,
            );

            Ok(result != 0)
        }
    }

    /// Get selection owner
    pub fn get_selection_owner(&self, selection: Atom) -> Result<Window> {
        unsafe {
            let owner = (self.xlib.XGetSelectionOwner)(self.display, selection as u64);

            if owner == 0 {
                return Ok(0); // No owner
            }

            Ok(owner as u64)
        }
    }

    /// Convert selection
    pub fn convert_selection(
        &self,
        requestor: Window,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Time,
    ) -> Result<()> {
        unsafe {
            (self.xlib.XConvertSelection)(
                self.display,
                selection as u64,
                target as u64,
                property as u64,
                requestor as XlibWindow,
                time,
            );
        }

        Ok(())
    }

    /// Select input events for a window
    pub fn select_input(&self, window: Window, event_mask: i64) -> Result<()> {
        unsafe {
            (self.xlib.XSelectInput)(self.display, window as XlibWindow, event_mask);
        }

        Ok(())
    }

    /// Raise window to top of stack
    pub fn raise_window(&self, window: Window) -> Result<()> {
        unsafe {
            (self.xlib.XRaiseWindow)(self.display, window as XlibWindow);
        }

        Ok(())
    }

    /// Lower window to bottom of stack
    pub fn lower_window(&self, window: Window) -> Result<()> {
        unsafe {
            (self.xlib.XLowerWindow)(self.display, window as XlibWindow);
        }

        Ok(())
    }

    /// Move and resize window
    pub fn move_resize_window(
        &self,
        window: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        unsafe {
            (self.xlib.XMoveResizeWindow)(
                self.display,
                window as XlibWindow,
                x,
                y,
                width,
                height,
            );
        }

        Ok(())
    }

    /// Reparent window
    pub fn reparent_window(
        &self,
        window: Window,
        parent: Window,
        x: i32,
        y: i32,
    ) -> Result<()> {
        unsafe {
            (self.xlib.XReparentWindow)(
                self.display,
                window as XlibWindow,
                parent as XlibWindow,
                x,
                y,
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

impl std::fmt::Debug for X11Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("X11Connection")
            .field("display", &self.display)
            .field("screen", &self.screen)
            .finish()
    }
}
