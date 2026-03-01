//! X11 event handling

use crate::x11::{Atom, Time, Window};
use std::mem;
use x11_dl::xlib::{
    ButtonPress, ButtonRelease, ClientMessage, EnterNotify, KeyPress, KeyRelease, LeaveNotify,
    MotionNotify, PropertyNotify, SelectionClear, SelectionNotify, SelectionRequest,
};
use x11_dl::xlib::{XAnyEvent, XEvent as XlibEvent};

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
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub is_hint: i8,
    pub same_screen: bool,
}

/// Button event (press/release)
#[derive(Debug, Clone, Copy)]
pub struct XButtonEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
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
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    pub same_screen: bool,
}

/// Crossing event (enter/leave)
#[derive(Debug, Clone, Copy)]
pub struct XCrossingEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub root: u64,
    pub subwindow: u64,
    pub time: u64,
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

// Other event types - simplified versions
#[derive(Debug, Clone, Copy)]
pub struct XExposeEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XGraphicsExposeEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XNoExposeEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XVisibilityEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XCreateWindowEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XDestroyWindowEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XUnmapEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XMapEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XMapRequestEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XReparentEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XConfigureEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XConfigureRequestEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XGravityEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XResizeRequestEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XCirculateEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XCirculateRequestEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XColormapEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XKeymapEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XFocusChangeEvent {
    pub type_: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XMappingEvent {
    pub type_: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct XPropertyEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub atom: u64,
    pub time: u64,
    pub state: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionRequestEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub owner: u64,
    pub requestor: u64,
    pub selection: u64,
    pub target: u64,
    pub property: u64,
    pub time: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub requestor: u64,
    pub selection: u64,
    pub target: u64,
    pub property: u64,
    pub time: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct XSelectionClearEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub selection: u64,
    pub time: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct XClientMessageEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub message_type: u64,
    pub format: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct XGenericEvent {
    pub type_: i32,
}

impl XEvent {
    /// Convert from Xlib XEvent to our XEvent enum
    ///
    /// This is unsafe because we're transmuting unions from x11-dl
    pub unsafe fn from_xlib_event(xlib_event: &XlibEvent) -> Self {
        let type_ = xlib_event.get_type();

        match type_ {
            MotionNotify => {
                let motion =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XMotionEvent);
                XEvent::MotionNotify(XMotionEvent {
                    type_: motion.type_,
                    serial: motion.serial as u64,
                    send_event: motion.send_event != 0,
                    display: motion.display.cast(),
                    window: motion.window as u64,
                    root: motion.root as u64,
                    subwindow: motion.subwindow as u64,
                    time: motion.time,
                    x: motion.x as i32,
                    y: motion.y as i32,
                    x_root: motion.x_root as i32,
                    y_root: motion.y_root as i32,
                    state: motion.state,
                    is_hint: motion.is_hint,
                    same_screen: motion.same_screen != 0,
                })
            }
            ButtonPress => {
                let button =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XButtonEvent);
                XEvent::ButtonPress(XButtonEvent {
                    type_: button.type_,
                    serial: button.serial as u64,
                    send_event: button.send_event != 0,
                    display: button.display.cast(),
                    window: button.window as u64,
                    root: button.root as u64,
                    subwindow: button.subwindow as u64,
                    time: button.time,
                    x: button.x as i32,
                    y: button.y as i32,
                    x_root: button.x_root as i32,
                    y_root: button.y_root as i32,
                    state: button.state,
                    button: button.button as u32,
                    same_screen: button.same_screen != 0,
                })
            }
            ButtonRelease => {
                let button =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XButtonEvent);
                XEvent::ButtonRelease(XButtonEvent {
                    type_: button.type_,
                    serial: button.serial as u64,
                    send_event: button.send_event != 0,
                    display: button.display.cast(),
                    window: button.window as u64,
                    root: button.root as u64,
                    subwindow: button.subwindow as u64,
                    time: button.time,
                    x: button.x as i32,
                    y: button.y as i32,
                    x_root: button.x_root as i32,
                    y_root: button.y_root as i32,
                    state: button.state,
                    button: button.button as u32,
                    same_screen: button.same_screen != 0,
                })
            }
            KeyPress => {
                let key = &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XKeyEvent);
                XEvent::KeyPress(XKeyEvent {
                    type_: key.type_,
                    serial: key.serial as u64,
                    send_event: key.send_event != 0,
                    display: key.display.cast(),
                    window: key.window as u64,
                    root: key.root as u64,
                    subwindow: key.subwindow as u64,
                    time: key.time,
                    x: key.x as i32,
                    y: key.y as i32,
                    x_root: key.x_root as i32,
                    y_root: key.y_root as i32,
                    state: key.state,
                    keycode: key.keycode,
                    same_screen: key.same_screen != 0,
                })
            }
            KeyRelease => {
                let key = &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XKeyEvent);
                XEvent::KeyRelease(XKeyEvent {
                    type_: key.type_,
                    serial: key.serial as u64,
                    send_event: key.send_event != 0,
                    display: key.display.cast(),
                    window: key.window as u64,
                    root: key.root as u64,
                    subwindow: key.subwindow as u64,
                    time: key.time,
                    x: key.x as i32,
                    y: key.y as i32,
                    x_root: key.x_root as i32,
                    y_root: key.y_root as i32,
                    state: key.state,
                    keycode: key.keycode,
                    same_screen: key.same_screen != 0,
                })
            }
            EnterNotify => {
                let crossing =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XCrossingEvent);
                XEvent::EnterNotify(XCrossingEvent {
                    type_: crossing.type_,
                    serial: crossing.serial as u64,
                    send_event: crossing.send_event != 0,
                    display: crossing.display.cast(),
                    window: crossing.window as u64,
                    root: crossing.root as u64,
                    subwindow: crossing.subwindow as u64,
                    time: crossing.time,
                    x: crossing.x as i32,
                    y: crossing.y as i32,
                    x_root: crossing.x_root as i32,
                    y_root: crossing.y_root as i32,
                    mode: crossing.mode,
                    detail: crossing.detail,
                    same_screen: crossing.same_screen != 0,
                    focus: crossing.focus != 0,
                    state: crossing.state,
                })
            }
            LeaveNotify => {
                let crossing =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XCrossingEvent);
                XEvent::LeaveNotify(XCrossingEvent {
                    type_: crossing.type_,
                    serial: crossing.serial as u64,
                    send_event: crossing.send_event != 0,
                    display: crossing.display.cast(),
                    window: crossing.window as u64,
                    root: crossing.root as u64,
                    subwindow: crossing.subwindow as u64,
                    time: crossing.time,
                    x: crossing.x as i32,
                    y: crossing.y as i32,
                    x_root: crossing.x_root as i32,
                    y_root: crossing.y_root as i32,
                    mode: crossing.mode,
                    detail: crossing.detail,
                    same_screen: crossing.same_screen != 0,
                    focus: crossing.focus != 0,
                    state: crossing.state,
                })
            }
            SelectionRequest => {
                let sel = &*(xlib_event as *const XlibEvent
                    as *const x11_dl::xlib::XSelectionRequestEvent);
                XEvent::SelectionRequest(XSelectionRequestEvent {
                    type_: sel.type_,
                    serial: sel.serial as u64,
                    send_event: sel.send_event != 0,
                    display: sel.display.cast(),
                    owner: sel.owner as u64,
                    requestor: sel.requestor as u64,
                    selection: sel.selection,
                    target: sel.target,
                    property: sel.property,
                    time: sel.time,
                })
            }
            SelectionNotify => {
                let sel =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XSelectionEvent);
                XEvent::SelectionNotify(XSelectionEvent {
                    type_: sel.type_,
                    serial: sel.serial as u64,
                    send_event: sel.send_event != 0,
                    display: sel.display.cast(),
                    requestor: sel.requestor as u64,
                    selection: sel.selection,
                    target: sel.target,
                    property: sel.property,
                    time: sel.time,
                })
            }
            SelectionClear => {
                let sel =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XSelectionClearEvent);
                XEvent::SelectionClear(XSelectionClearEvent {
                    type_: sel.type_,
                    serial: sel.serial as u64,
                    send_event: sel.send_event != 0,
                    display: sel.display.cast(),
                    window: sel.window as u64,
                    selection: sel.selection,
                    time: sel.time,
                })
            }
            PropertyNotify => {
                let prop =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XPropertyEvent);
                XEvent::PropertyNotify(XPropertyEvent {
                    type_: prop.type_,
                    serial: prop.serial as u64,
                    send_event: prop.send_event != 0,
                    display: prop.display.cast(),
                    window: prop.window as u64,
                    atom: prop.atom,
                    time: prop.time,
                    state: prop.state,
                })
            }
            ClientMessage => {
                let msg =
                    &*(xlib_event as *const XlibEvent as *const x11_dl::xlib::XClientMessageEvent);
                XEvent::ClientMessage(XClientMessageEvent {
                    type_: msg.type_,
                    serial: msg.serial as u64,
                    send_event: msg.send_event != 0,
                    display: msg.display.cast(),
                    window: msg.window as u64,
                    message_type: msg.message_type,
                    format: msg.format,
                })
            }
            _ => {
                // Generic fallback for unhandled event types
                log::debug!("Unhandled X11 event type: {}", type_);
                XEvent::GenericEvent(XGenericEvent {
                    type_: type_ as i32,
                })
            }
        }
    }

    /// Get the event type as a numeric value
    pub fn event_type(&self) -> i32 {
        match self {
            XEvent::MotionNotify(e) => e.type_,
            XEvent::ButtonPress(e) => e.type_,
            XEvent::ButtonRelease(e) => e.type_,
            XEvent::KeyPress(e) => e.type_,
            XEvent::KeyRelease(e) => e.type_,
            XEvent::EnterNotify(e) => e.type_,
            XEvent::LeaveNotify(e) => e.type_,
            XEvent::FocusIn(e) => e.type_,
            XEvent::FocusOut(e) => e.type_,
            XEvent::KeymapNotify(e) => e.type_,
            XEvent::Expose(e) => e.type_,
            XEvent::GraphicsExpose(e) => e.type_,
            XEvent::NoExpose(e) => e.type_,
            XEvent::VisibilityNotify(e) => e.type_,
            XEvent::CreateNotify(e) => e.type_,
            XEvent::DestroyNotify(e) => e.type_,
            XEvent::UnmapNotify(e) => e.type_,
            XEvent::MapNotify(e) => e.type_,
            XEvent::MapRequest(e) => e.type_,
            XEvent::ReparentNotify(e) => e.type_,
            XEvent::ConfigureNotify(e) => e.type_,
            XEvent::ConfigureRequest(e) => e.type_,
            XEvent::GravityNotify(e) => e.type_,
            XEvent::ResizeRequest(e) => e.type_,
            XEvent::CirculateNotify(e) => e.type_,
            XEvent::CirculateRequest(e) => e.type_,
            XEvent::PropertyNotify(e) => e.type_,
            XEvent::SelectionRequest(e) => e.type_,
            XEvent::SelectionNotify(e) => e.type_,
            XEvent::SelectionClear(e) => e.type_,
            XEvent::ColormapNotify(e) => e.type_,
            XEvent::ClientMessage(e) => e.type_,
            XEvent::MappingNotify(e) => e.type_,
            XEvent::GenericEvent(e) => e.type_,
        }
    }

    /// Get the window associated with this event
    pub fn window(&self) -> Option<u64> {
        match self {
            XEvent::MotionNotify(e) => Some(e.window),
            XEvent::ButtonPress(e) => Some(e.window),
            XEvent::ButtonRelease(e) => Some(e.window),
            XEvent::KeyPress(e) => Some(e.window),
            XEvent::KeyRelease(e) => Some(e.window),
            XEvent::EnterNotify(e) => Some(e.window),
            XEvent::LeaveNotify(e) => Some(e.window),
            XEvent::PropertyNotify(e) => Some(e.window),
            XEvent::SelectionClear(e) => Some(e.window),
            _ => None,
        }
    }

    /// Get the timestamp of this event
    pub fn time(&self) -> u64 {
        match self {
            XEvent::MotionNotify(e) => e.time,
            XEvent::ButtonPress(e) => e.time,
            XEvent::ButtonRelease(e) => e.time,
            XEvent::KeyPress(e) => e.time,
            XEvent::KeyRelease(e) => e.time,
            XEvent::EnterNotify(e) => e.time,
            XEvent::LeaveNotify(e) => e.time,
            XEvent::PropertyNotify(e) => e.time,
            XEvent::SelectionRequest(e) => e.time,
            XEvent::SelectionNotify(e) => e.time,
            XEvent::SelectionClear(e) => e.time,
            _ => 0,
        }
    }
}

/// Event handler trait
pub trait EventHandler {
    /// Handle an X11 event
    ///
    /// Returns Ok(true) if event processing should stop,
    /// Ok(false) to continue processing.
    fn handle(&self, event: &XEvent, ctx: &mut crate::core::DpyInfo) -> anyhow::Result<bool>;
}
