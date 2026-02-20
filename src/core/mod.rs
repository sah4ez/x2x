//! Core data structures and logic
//!
//! This module contains the main data structures for managing X displays
//! and their state.

pub mod dpy_info;
pub mod coord_mapping;
pub mod state;
pub mod event_loop;

pub use dpy_info::{DpyInfo, ShadowDisplay, ConnectionMode};
pub use coord_mapping::{CoordinateMapping, LayoutMode, Direction};
pub use state::{FakeEvent, FakeQueue, StickyKeys};
pub use event_loop::EventLoop;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(N_BUTTONS, 20);
        assert_eq!(MAX_BUTTON_MAP_EVENTS, 20);
        assert_eq!(COORD_INCR, -1);
        assert_eq!(COORD_DECR, -2);
    }

    #[test]
    fn test_is_special_coord() {
        assert!(is_special_coord(COORD_INCR));
        assert!(is_special_coord(COORD_DECR));
        assert!(is_special_coord(-5));
        assert!(!is_special_coord(0));
        assert!(!is_special_coord(100));
    }

    #[test]
    fn test_special_coord() {
        assert_eq!(special_coord(COORD_INCR), COORD_INCR);
        assert_eq!(special_coord(COORD_DECR), COORD_DECR);
        assert_eq!(special_coord(-5), -5);
        assert_eq!(special_coord(0), 0);
        assert_eq!(special_coord(100), 0);
    }
}
