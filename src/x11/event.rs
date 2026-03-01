//! X11 event handling

use crate::x11::{Atom, Time, Window};
#[allow(unused_imports)]
use std::mem;
#[allow(unused_imports)]
use x11_dl::xlib::{
    ButtonPress, ButtonRelease, ClientMessage, EnterNotify, KeyPress, KeyRelease, LeaveNotify,
    MotionNotify, PropertyNotify, SelectionClear, SelectionNotify, SelectionRequest,
};
use x11_dl::xlib::{XAnyEvent, XEvent as XlibEvent};

/// X11 event types
#[derive(Debug, Clone)]
#[allow(non_upper_case_globals)] // X11 constants are lowercase, suppress warnings
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
    #[allow(non_upper_case_globals)]
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
    #[allow(non_upper_case_globals)]
    SelectionNotify(XSelectionEvent),
    #[allow(non_upper_case_globals)]
    SelectionClear(XSelectionClearEvent),
    ColormapNotify(XColormapEvent),
    #[allow(non_upper_case_globals)]
    ClientMessage(XClientMessageEvent),
    MappingNotify(XMappingEvent),
    GenericEvent(XGenericEvent),
}

/// Motion notify event
#[derive(Debug, Clone)]
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
    pub state: u8,
    pub is_hint: bool,
    pub same_screen: bool,
}

/// Button event
#[derive(Debug, Clone)]
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
    pub state: u8,
    pub button: u32,
    pub same_screen: bool,
}

/// Key event
#[derive(Debug, Clone)]
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
    pub state: u8,
    pub keycode: u8,
    pub same_screen: bool,
}

/// Crossing event (enter/leave)
#[derive(Debug, Clone)]
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
    pub mode: u8,
    pub detail: u32,
    pub same_screen: bool,
    pub focus: bool,
    pub state: u8,
}

/// Focus change event
#[derive(Debug, Clone)]
pub struct XFocusChangeEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub mode: u8,
}

/// Keymap notify event
#[derive(Debug, Clone)]
pub struct XKeymapEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
}

/// Expose event
#[derive(Debug, Clone)]
pub struct XExposeEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
}

/// Graphics expose event
#[derive(Debug, Clone)]
pub struct XGraphicsExposeEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub drawable: u64,
}

/// Visibility notify event
#[derive(Debug, Clone)]
pub struct XVisibilityEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub state: u8,
}

/// Create window event
#[derive(Debug, Clone)]
pub struct XCreateWindowEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub parent: u64,
    pub window: u64,
}

/// Destroy window event
#[derive(Debug, Clone)]
pub struct XDestroyWindowEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
}

/// Unmap event
#[derive(Debug, Clone)]
pub struct XUnmapEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub from_configure: bool,
}

/// Map event
#[derive(Debug, Clone)]
pub struct XMapEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub override_redirect: bool,
}

/// Map request event
#[derive(Debug, Clone)]
pub struct XMapRequestEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub parent: u64,
    pub window: u64,
}

/// Reparent event
#[derive(Debug, Clone)]
pub struct XReparentEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub parent: u64,
    pub override_redirect: bool,
}

/// Configure event
#[derive(Debug, Clone)]
pub struct XConfigureEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: u64,
    pub override_redirect: bool,
}

/// Configure request event
#[derive(Debug, Clone)]
pub struct XConfigureRequestEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub parent: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub above: u64,
    pub detail: u32,
    pub value_mask: u64,
}

/// Gravity notify event
#[derive(Debug, Clone)]
pub struct XGravityEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub x: i32,
    pub y: i32,
}

/// Resize request event
#[derive(Debug, Clone)]
pub struct XResizeRequestEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub value_mask: u64,
}

/// Circulate event
#[derive(Debug, Clone)]
pub struct XCirculateEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub event: u64,
    pub window: u64,
    pub place: u32,
    pub result: u32,
}

/// Circulate request event
#[derive(Debug, Clone)]
pub struct XCirculateRequestEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub parent: u64,
    pub window: u64,
}

/// Property notify event
#[derive(Debug, Clone)]
pub struct XPropertyEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub atom: u64,
    pub time: u32,
    pub state: u8,
}

/// Selection request event
#[derive(Debug, Clone)]
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
    pub time: u32,
}

/// Selection notify event
#[derive(Debug, Clone)]
pub struct XSelectionEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub requestor: u64,
    pub selection: u64,
    pub target: u64,
    pub property: u64,
    pub time: u32,
}

/// Selection clear event
#[derive(Debug, Clone)]
pub struct XSelectionClearEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub selection: u64,
    pub time: u32,
}

/// Colormap notify event
#[derive(Debug, Clone)]
pub struct XColormapEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub colormap: u64,
    pub new: bool,
    pub state: u8,
}

/// Client message event
#[derive(Debug, Clone)]
pub struct XClientMessageEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: bool,
    pub display: *mut (),
    pub window: u64,
    pub message_type: u64,
    pub format: i32,
    pub time: u64,
}

/// Generic event
#[derive(Debug, Clone)]
pub struct XGenericEvent {
    pub type_: i32,
}

impl XEvent {
    /// Convert from Xlib XEvent to our XEvent enum
    ///
    /// This is unsafe because we're transmuting unions from x11-dl
    #[allow(non_upper_case_globals)] // Suppress pattern warnings for X11 constants
    #[allow(unused_unsafe)] // TODO: Make safe eventually
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
            _ => XEvent::GenericEvent(XGenericEvent { type_ }),
        }
    }

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

    pub fn window(&self) -> Option<u64> {
        match self {
            XEvent::MotionNotify(e) => Some(e.window),
            XEvent::ButtonPress(e) => Some(e.window),
            XEvent::ButtonRelease(e) => Some(e.window),
            XEvent::KeyPress(e) => Some(e.window),
            XEvent::KeyRelease(e) => Some(e.window),
            XEvent::EnterNotify(e) => Some(e.window),
            XEvent::LeaveNotify(e) => Some(e.window),
            XEvent::FocusIn(e) => None,
            XEvent::FocusOut(e) => None,
            XEvent::KeymapNotify(e) => Some(e.window),
            XEvent::Expose(e) => Some(e.window),
            XEvent::GraphicsExpose(e) => Some(e.drawable),
            XEvent::NoExpose(e) => None,
            XEvent::VisibilityNotify(e) => Some(e.window),
            XEvent::CreateNotify(e) => Some(e.window),
            XEvent::DestroyNotify(e) => Some(e.window),
            XEvent::UnmapNotify(e) => Some(e.window),
            XEvent::MapNotify(e) => Some(e.window),
            XEvent::MapRequest(e) => Some(e.window),
            XEvent::ReparentNotify(e) => Some(e.window),
            XEvent::ConfigureNotify(e) => Some(e.window),
            XEvent::ConfigureRequest(e) => Some(e.window),
            XEvent::GravityNotify(e) => Some(e.window),
            XEvent::ResizeRequest(e) => Some(e.window),
            XEvent::CirculateNotify(e) => Some(e.window),
            XEvent::CirculateRequest(e) => Some(e.window),
            XEvent::PropertyNotify(e) => Some(e.window),
            XEvent::SelectionRequest(e) => Some(e.owner),
            XEvent::SelectionNotify(e) => Some(e.requestor),
            XEvent::SelectionClear(e) => Some(e.window),
            XEvent::ColormapNotify(e) => Some(e.window),
            XEvent::ClientMessage(e) => Some(e.window),
            XEvent::MappingNotify(e) => None,
            XEvent::GenericEvent(e) => None,
        }
    }

    pub fn time(&self) -> u64 {
        match self {
            XEvent::MotionNotify(e) => e.time,
            XEvent::ButtonPress(e) => e.time,
            XEvent::ButtonRelease(e) => e.time,
            XEvent::KeyPress(e) => e.time,
            XEvent::KeyRelease(e) => e.time,
            XEvent::EnterNotify(e) => e.time,
            XEvent::LeaveNotify(e) => e.time,
            XEvent::FocusIn(e) => 0,
            XEvent::FocusOut(e) => 0,
            XEvent::KeymapNotify(e) => 0,
            XEvent::Expose(e) => e.time,
            XEvent::GraphicsExpose(e) => e.time,
            XEvent::NoExpose(e) => e.time,
            XEvent::VisibilityNotify(e) => e.time,
            XEvent::CreateNotify(e) => e.time,
            XEvent::DestroyNotify(e) => e.time,
            XEvent::UnmapNotify(e) => e.time,
            XEvent::MapNotify(e) => e.time,
            XEvent::MapRequest(e) => e.time,
            XEvent::ReparentNotify(e) => e.time,
            XEvent::ConfigureNotify(e) => e.time,
            XEvent::ConfigureRequest(e) => e.time,
            XEvent::GravityNotify(e) => e.time,
            XEvent::ResizeRequest(e) => e.time,
            XEvent::CirculateNotify(e) => e.time,
            XEvent::CirculateRequest(e) => e.time,
            XEvent::PropertyNotify(e) => e.time,
            XEvent::SelectionRequest(e) => e.time,
            XEvent::SelectionNotify(e) => e.time,
            XEvent::SelectionClear(e) => e.time,
            XEvent::ColormapNotify(e) => e.time,
            XEvent::ClientMessage(e) => e.time,
            XEvent::MappingNotify(e) => e.time,
            XEvent::GenericEvent(e) => e.type_ as u64,
        }
    }
}

/// Event handler trait
pub trait EventHandler {
    /// Handle an X11 event
    ///
    /// Returns Ok(true) if the event was handled and should stop processing,
    /// Ok(false) if the event was not handled.
    fn handle(&self, event: &XEvent, ctx: &mut crate::core::DpyInfo) -> anyhow::Result<bool>;
}
