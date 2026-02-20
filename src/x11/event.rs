//! X11 event handling

use crate::x11::{Window, Atom, Time};
use x11_dl::xlib::{XEvent as XlibEvent, XAnyEvent};
use x11_dl::xlib::{MotionNotify, ButtonPress, ButtonRelease, KeyPress, KeyRelease,
                      EnterNotify, LeaveNotify, SelectionRequest, SelectionNotify,
                      SelectionClear, PropertyNotify, ClientMessage};

/// X11 event types
#[derive(Debug, Clone)]
pub enum XEvent {
    MotionNotify(XMotionEvent),
    ButtonPress(XButtonEvent),
    ButtonRelease(XButtonEvent),
    KeyPress(XKeyEvent),
    KeyRelease(XKeyEvent),
    EnterNotify(XCrossingEvent),
    LeaveNotify(XCrossingEvent),
    FocusIn(XFocusChangeEvent),
    FocusOut(XFocusChangeEvent),
    KeymapNotify(XKeymapEvent),
    Expose(XExposeEvent),
    GraphicsExpose(XGraphicsExposeEvent),
    NoExpose(XNoExposeEvent),
    VisibilityNotify(XVisibilityEvent),
    CreateNotify(XCreateWindowEvent),
    DestroyNotify(XDestroyWindowEvent),
    UnmapNotify(XUnmapEvent),
    MapNotify(XMapEvent),
    MapRequest(XMapRequestEvent),
    ReparentNotify(XReparentEvent),
    ConfigureNotify(XConfigureEvent),
    ConfigureRequest(XConfigureRequestEvent),
    GravityNotify(XGravityEvent),
    ResizeRequest(XResizeRequestEvent),
    CirculateNotify(XCirculateEvent),
    CirculateRequest(XCirculateRequestEvent),
    PropertyNotify(XPropertyEvent),
    SelectionRequest(XSelectionRequestEvent),
    SelectionNotify(XSelectionEvent),
    SelectionClear(XSelectionClearEvent),
    ColormapNotify(XColormapEvent),
    ClientMessage(XClientMessageEvent),
    MappingNotify(XMappingEvent),
    GenericEvent(XGenericEvent),
}

/// Motion notify event
#[derive(Debug, Clone, Copy)]
pub struct XMotionEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u32,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub is_hint: u8,
    pub same_screen: bool,
}

/// Button event (press/release)
#[derive(Debug, Clone, Copy)]
pub struct XButtonEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u32,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub button: u32,
    pub same_screen: bool,
}

/// Key event (press/release)
#[derive(Debug, Clone, Copy)]
pub struct XKeyEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u32,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u8,
    pub same_screen: bool,
}

/// Crossing event (enter/leave)
#[derive(Debug, Clone, Copy)]
pub struct XCrossingEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u32,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub mode: i32,
    pub detail: i32,
    pub same_screen: bool,
    pub focus: bool,
    pub state: u32,
}

// ... other event types would be defined here

#[derive(Debug, Clone, Copy)]
pub struct XExposeEvent;
#[derive(Debug, Clone, Copy)]
pub struct XGraphicsExposeEvent;
#[derive(Debug, Clone, Copy)]
pub struct XNoExposeEvent;
#[derive(Debug, Clone, Copy)]
pub struct XVisibilityEvent;
#[derive(Debug, Clone, Copy)]
pub struct XCreateWindowEvent;
#[derive(Debug, Clone, Copy)]
pub struct XDestroyWindowEvent;
#[derive(Debug, Clone, Copy)]
pub struct XUnmapEvent;
#[derive(Debug, Clone, Copy)]
pub struct XMapEvent;
#[derive(Debug, Clone, Copy)]
pub struct XMapRequestEvent;
#[derive(Debug, Clone, Copy)]
pub struct XReparentEvent;
#[derive(Debug, Clone, Copy)]
pub struct XConfigureEvent;
#[derive(Debug, Clone, Copy)]
pub struct XConfigureRequestEvent;
#[derive(Debug, Clone, Copy)]
pub struct XGravityEvent;
#[derive(Debug, Clone, Copy)]
pub struct XResizeRequestEvent;
#[derive(Debug, Clone, Copy)]
pub struct XCirculateEvent;
#[derive(Debug, Clone, Copy)]
pub struct XCirculateRequestEvent;
#[derive(Debug, Clone, Copy)]
pub struct XPropertyEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub atom: u32,
    pub time: u32,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionRequestEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub owner: u64,
    pub requestor: u64,
    pub selection: u32,
    pub target: u32,
    pub property: u32,
    pub time: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub requestor: u64,
    pub selection: u32,
    pub target: u32,
    pub property: u32,
    pub time: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionClearEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub selection: u32,
    pub time: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct XColormapEvent;
#[derive(Debug, Clone, Copy)]
pub struct XClientMessageEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub message_type: u32,
    pub format: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XMappingEvent;
#[derive(Debug, Clone, Copy)]
pub struct XKeymapEvent;
#[derive(Debug, Clone, Copy)]
pub struct XFocusChangeEvent;

#[derive(Debug, Clone, Copy)]
pub struct XGenericEvent {
    pub type_: i32,
}

/// Event handler trait
pub trait EventHandler {
    /// Handle an X11 event
    ///
    /// Returns Ok(true) if event processing should stop,
    /// Ok(false) to continue processing.
    fn handle(
        &self,
        event: &XEvent,
        ctx: &mut crate::core::DpyInfo,
    ) -> anyhow::Result<bool>;
}
