//! Core data structures and logic
//!
//! This module contains the main data structures for managing X displays
//! and their state.

pub mod dpy_info;
pub mod coord_mapping;
pub mod state;

pub use dpy_info::{DpyInfo, ShadowDisplay, ConnectionMode};
pub use coord_mapping::{CoordinateMapping, LayoutMode, Direction};
pub use state::{FakeEvent, FakeQueue, StickyKeys};

// Constants
pub const N_BUTTONS: usize = 20;
pub const MAX_BUTTON_MAP_EVENTS: usize = 20;

// Special coordinate values
pub const COORD_INCR: i16 = -1;
pub const COORD_DECR: i16 = -2;

/// Check if a coordinate is special (COORD_INCR or COORD_DECR)
pub const fn is_special_coord(coord: i16) -> bool {
    coord < 0
}

/// Convert coordinate to special value if applicable
pub const fn special_coord(coord: i16) -> i16 {
    if is_special_coord(coord) { coord } else { 0 }
}
