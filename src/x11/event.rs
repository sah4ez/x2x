//! X11 event handling

use crate::x11::{Window, Atom, Time};
use x11_dl::xlib::{XEvent as XlibEvent, XAnyEvent};

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
    SelectionClear(XSelectionClearEvent),
    SelectionRequest(XSelectionRequestEvent),
    SelectionNotify(XSelectionEvent),
    ColormapNotify(XColormapEvent),
    ClientMessage(XClientMessageEvent),
    MappingNotify(XMappingEvent),
    GenericEvent(XGenericEvent),
}

impl XEvent {
    /// Convert Xlib event to our XEvent enum
    pub fn from_xlib_event(xevent: &XlibEvent) -> Self {
        unsafe {
            let any = xevent.any;
            match any.type_ {
                xlib::MotionNotify => {
                    let ev = xevent.motion;
                    Self::MotionNotify(XMotionEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        state: ev.state,
                        is_hint: ev.is_hint,
                        same_screen: ev.same_screen != 0,
                    })
                }
                xlib::ButtonPress => {
                    let ev = xevent.button;
                    Self::ButtonPress(XButtonEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        state: ev.state,
                        button: ev.button,
                        same_screen: ev.same_screen != 0,
                    })
                }
                xlib::ButtonRelease => {
                    let ev = xevent.button;
                    Self::ButtonRelease(XButtonEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        state: ev.state,
                        button: ev.button,
                        same_screen: ev.same_screen != 0,
                    })
                }
                xlib::KeyPress => {
                    let ev = xevent.key;
                    Self::KeyPress(XKeyEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        state: ev.state,
                        keycode: ev.keycode,
                        same_screen: ev.same_screen != 0,
                    })
                }
                xlib::KeyRelease => {
                    let ev = xevent.key;
                    Self::KeyRelease(XKeyEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        state: ev.state,
                        keycode: ev.keycode,
                        same_screen: ev.same_screen != 0,
                    })
                }
                xlib::EnterNotify => {
                    let ev = xevent.crossing;
                    Self::EnterNotify(XCrossingEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        mode: ev.mode,
                        detail: ev.detail,
                        same_screen: ev.same_screen != 0,
                        focus: ev.focus != 0,
                        state: ev.state,
                    })
                }
                xlib::LeaveNotify => {
                    let ev = xevent.crossing;
                    Self::LeaveNotify(XCrossingEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        root: ev.root as u64,
                        subwindow: ev.subwindow as u64,
                        time: ev.time as u32,
                        x: ev.x,
                        y: ev.y,
                        x_root: ev.x_root,
                        y_root: ev.y_root,
                        mode: ev.mode,
                        detail: ev.detail,
                        same_screen: ev.same_screen != 0,
                        focus: ev.focus != 0,
                        state: ev.state,
                    })
                }
                xlib::SelectionRequest => {
                    let ev = xevent.selection_request;
                    Self::SelectionRequest(XSelectionRequestEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        owner: ev.owner as u64,
                        requestor: ev.requestor as u64,
                        selection: ev.selection as u32,
                        target: ev.target as u32,
                        property: ev.property as u32,
                        time: ev.time as u32,
                    })
                }
                xlib::SelectionNotify => {
                    let ev = xevent.selection;
                    Self::SelectionNotify(XSelectionEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        requestor: ev.requestor as u64,
                        selection: ev.selection as u32,
                        target: ev.target as u32,
                        property: ev.property as u32,
                        time: ev.time as u32,
                    })
                }
                xlib::SelectionClear => {
                    let ev = xevent.selection_clear;
                    Self::SelectionClear(XSelectionClearEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        selection: ev.selection as u32,
                        time: ev.time as u32,
                    })
                }
                xlib::PropertyNotify => {
                    let ev = xevent.property;
                    Self::PropertyNotify(XPropertyEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        atom: ev.atom as u32,
                        time: ev.time as u32,
                        state: ev.state,
                    })
                }
                xlib::ClientMessage => {
                    let ev = xevent.client_message;
                    Self::ClientMessage(XClientMessageEvent {
                        type_: ev.type_,
                        serial: ev.serial,
                        send_event: ev.send_event != 0,
                        display: ev.display,
                        window: ev.window as u64,
                        message_type: ev.message_type as u32,
                        format: ev.format,
                    })
                }
                _ => {
                    // For unhandled event types, create a generic placeholder
                    Self::GenericEvent(XGenericEvent {
                        type_: any.type_,
                    })
                }
            }
        }
    }
}

/// Motion notify event
#[derive(Debug, Clone, Copy)]
pub struct XMotionEvent {
    pub type_: u8,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
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
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
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
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
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
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
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

/// Placeholder for other event types
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
