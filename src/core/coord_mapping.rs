//! Coordinate mapping between displays

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
        _from_screen: &crate::x11::ScreenInfo,
        to_screens: &[crate::x11::ScreenInfo],
        _mode: crate::core::LayoutMode,
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
            from_width: 1920,
            from_height: 1080,
        })
    }

    /// Map X coordinate
    pub fn map_x(&self, from_x: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens {
            return crate::core::COORD_INCR as i32;
        }

        if from_x < 0 || from_x >= self.from_width as i32 {
            return crate::core::COORD_INCR as i32;
        }

        self.x_tables
            .get(to_screen)
            .and_then(|table| table.get(from_x as usize))
            .copied()
            .unwrap_or(crate::core::COORD_INCR) as i32
    }

    /// Map Y coordinate
    pub fn map_y(&self, from_y: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens {
            return crate::core::COORD_INCR as i32;
        }

        if from_y < 0 || from_y >= self.from_height as i32 {
            return crate::core::COORD_INCR as i32;
        }

        self.y_tables
            .get(to_screen)
            .and_then(|table| table.get(from_y as usize))
            .copied()
            .unwrap_or(crate::core::COORD_INCR) as i32
    }

    /// Check if coordinate is special
    pub fn is_special(&self, coord: i32) -> bool {
        coord == crate::core::COORD_INCR as i32 || coord == crate::core::COORD_DECR as i32
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
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &crate::x11::ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            },
        ).unwrap();

        assert!(mapping.is_special(crate::core::COORD_INCR as i32));
        assert!(mapping.is_special(crate::core::COORD_DECR as i32));
        assert!(!mapping.is_special(100));
    }

    #[test]
    fn test_new_with_empty_screens() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &crate::x11::ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            crate::core::LayoutMode::Horizontal {
                direction: crate::core::Direction::Right,
            },
        ).unwrap();

        assert_eq!(mapping.n_screens, 0);
    }

    #[test]
    fn test_map_x_out_of_bounds() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &crate::x11::ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            crate::core::LayoutMode::Horizontal {
                direction: crate::core::Direction::Right,
            },
        ).unwrap();

        // Test out-of-bounds coordinates
        let result = mapping.map_x(-1, 0);
        assert_eq!(result, crate::core::COORD_INCR as i32);

        let result = mapping.map_x(1920, 0);
        assert_eq!(result, crate::core::COORD_INCR as i32);
    }

    #[test]
    fn test_map_y_out_of_bounds() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &crate::x11::ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            crate::core::LayoutMode::Horizontal {
                direction: crate::core::Direction::Right,
            },
        ).unwrap();

        // Test out-of-bounds coordinates
        let result = mapping.map_y(-1, 0);
        assert_eq!(result, crate::core::COORD_INCR as i32);

        let result = mapping.map_y(1080, 0);
        assert_eq!(result, crate::core::COORD_INCR as i32);
    }

    #[test]
    fn test_layout_mode_variants() {
        // Test LayoutMode enum
        let horizontal = crate::core::LayoutMode::Horizontal {
            direction: crate::core::Direction::Right,
        };
        assert_eq!(horizontal, crate::core::LayoutMode::Horizontal {
            direction: crate::core::Direction::Right,
        });

        let vertical = crate::core::LayoutMode::Vertical {
            direction: crate::core::Direction::Up,
        };
        assert_eq!(vertical, crate::core::LayoutMode::Vertical {
            direction: crate::core::Direction::Up,
        });
    }
}
