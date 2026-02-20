//! Coordinate mapping between displays

use crate::x11::ScreenInfo;
use anyhow::Result;

/// Coordinate mapping tables
pub struct CoordinateMapping {
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub n_screens: usize,
    pub from_width: u32,
    pub from_height: u32,
}

impl CoordinateMapping {
    /// Create new coordinate mapping
    pub fn new(
        from_screen: &ScreenInfo,
        to_screens: &[ScreenInfo],
        mode: LayoutMode,
    ) -> Result<Self> {
        // TODO: Implement table building logic from C code
        // This involves precalculating coordinate transformations

        let n_screens = to_screens.len();
        let x_tables = Vec::new();
        let y_tables = Vec::new();

        Ok(Self {
            x_tables,
            y_tables,
            n_screens,
            from_width: from_screen.width,
            from_height: from_screen.height,
        })
    }

    /// Map X coordinate
    pub fn map_x(&self, from_x: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens {
            return super::COORD_INCR as i32;
        }

        if from_x < 0 || from_x >= self.from_width as i32 {
            return super::COORD_INCR as i32;
        }

        self.x_tables
            .get(to_screen)
            .and_then(|table| table.get(from_x as usize))
            .copied()
            .unwrap_or(super::COORD_INCR as i32) as i32
    }

    /// Map Y coordinate
    pub fn map_y(&self, from_y: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens {
            return super::COORD_INCR as i32;
        }

        if from_y < 0 || from_y >= self.from_height as i32 {
            return super::COORD_INCR as i32;
        }

        self.y_tables
            .get(to_screen)
            .and_then(|table| table.get(from_y as usize))
            .copied()
            .unwrap_or(super::COORD_INCR as i32) as i32
    }

    /// Check if coordinate is special
    pub fn is_special(&self, coord: i32) -> bool {
        coord == super::COORD_INCR as i32 || coord == super::COORD_DECR as i32
    }
}

/// Layout mode for screen arrangement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Horizontal { direction: Direction },
    Vertical { direction: Direction },
}

/// Direction for screen connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_roundtrip() {
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);
        assert_eq!(Direction::Up, Direction::Up);
        assert_eq!(Direction::Down, Direction::Down);
    }

    #[test]
    fn test_special_coordinates() {
        let from_screen = ScreenInfo {
            screen_num: 0,
            root: 0,
            width: 1920,
            height: 1080,
        };
        let to_screens = vec![ScreenInfo {
            screen_num: 0,
            root: 1,
            width: 1920,
            height: 1080,
        }];

        let mapping = CoordinateMapping::new(
            &from_screen,
            &to_screens,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            },
        ).unwrap();

        assert!(mapping.is_special(super::COORD_INCR as i32));
        assert!(mapping.is_special(super::COORD_DECR as i32));
        assert!(!mapping.is_special(100));
    }
}
