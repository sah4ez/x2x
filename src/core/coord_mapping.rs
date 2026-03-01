//! Coordinate mapping between displays

use crate::x11::ScreenInfo;
use anyhow::Result;
use std::cmp;

/// Coordinate mapping tables
///
/// This structure manages the transformation of coordinates from one display
/// to another, based on screen layout and connection points.
pub struct CoordinateMapping {
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub n_screens: usize,
    pub from_width: u32,
    pub from_height: u32,
}

impl CoordinateMapping {
    /// Create new coordinate mapping
    ///
    /// Builds transformation tables based on the layout mode (horizontal/vertical)
    /// and direction (left/right/up/down).
    pub fn new(
        from_screen: &ScreenInfo,
        to_screens: &[ScreenInfo],
        mode: LayoutMode,
    ) -> Result<Self> {
        if to_screens.is_empty() {
            return Ok(Self {
                x_tables: Vec::new(),
                y_tables: Vec::new(),
                n_screens: 0,
                from_width: from_screen.width,
                from_height: from_screen.height,
            });
        }

        let n_screens = to_screens.len();
        let from_width = from_screen.width as usize;
        let from_height = from_screen.height as usize;

        // Initialize tables with default values
        // COORD_INCR (-1) means "move to next screen"
        // COORD_DECR (-2) means "move to previous screen"
        let mut x_tables: Vec<Vec<i16>> = vec![vec![0; from_width]; n_screens];
        let mut y_tables: Vec<Vec<i16>> = vec![vec![0; from_height]; n_screens];

        // Build tables based on layout mode
        match mode {
            LayoutMode::Horizontal { direction } => {
                Self::build_horizontal_tables(from_screen, to_screens, direction, &mut x_tables)?;
            }
            LayoutMode::Vertical { direction } => {
                Self::build_vertical_tables(from_screen, to_screens, direction, &mut y_tables)?;
            }
        }

        Ok(Self {
            x_tables,
            y_tables,
            n_screens,
            from_width: from_screen.width as u32,
            from_height: from_screen.height as u32,
        })
    }

    /// Build horizontal layout tables (left/right connection)
    fn build_horizontal_tables(
        from_screen: &ScreenInfo,
        to_screens: &[ScreenInfo],
        direction: Direction,
        x_tables: &mut [Vec<i16>],
    ) -> Result<()> {
        let from_width = from_screen.width as usize;

        for (to_idx, to_screen) in to_screens.iter().enumerate() {
            let to_width = to_screen.width as usize;

            // For each X coordinate on from display, compute the corresponding X coordinate
            // on to display (or special value if switching screens)
            for x in 0..from_width {
                if to_idx < x_tables.len() {
                    // Check if we're at the edge of the from screen
                    if direction == Direction::Right {
                        if x >= from_width - 10 && x < from_width {
                            // Near right edge - switch to to screen
                            x_tables[to_idx][x] = crate::core::COORD_INCR;
                        } else if x < 10 {
                            // Near left edge - came from to screen
                            x_tables[to_idx][x] = crate::core::COORD_DECR;
                        } else {
                            // Normal coordinate - map proportionally
                            let ratio = to_width as f64 / from_width as f64;
                            x_tables[to_idx][x] = (x as f64 * ratio) as i16;
                        }
                    } else if direction == Direction::Left {
                        if x < 10 && x >= 0 {
                            // Near left edge - switch to to screen
                            x_tables[to_idx][x] = crate::core::COORD_INCR;
                        } else if x >= from_width - 10 && x < from_width {
                            // Near right edge - came from to screen
                            x_tables[to_idx][x] = crate::core::COORD_DECR;
                        } else {
                            // Normal coordinate - map proportionally
                            let ratio = to_width as f64 / from_width as f64;
                            x_tables[to_idx][x] = (x as f64 * ratio) as i16;
                        }
                    } else {
                        // No connection - keep on from screen
                        x_tables[to_idx][x] = x as i16;
                    }
                } else {
                    x_tables[to_idx][x] = x as i16;
                }
            }
        }

        Ok(())
    }

    /// Build vertical layout tables (up/down connection)
    fn build_vertical_tables(
        from_screen: &ScreenInfo,
        to_screens: &[ScreenInfo],
        direction: Direction,
        y_tables: &mut [Vec<i16>],
    ) -> Result<()> {
        let from_height = from_screen.height as usize;

        for (to_idx, to_screen) in to_screens.iter().enumerate() {
            let to_height = to_screen.height as usize;

            // For each Y coordinate on from display, compute the corresponding Y coordinate
            // on to display (or special value if switching screens)
            for y in 0..from_height {
                if to_idx < y_tables.len() {
                    // Check if we're at the edge of the from screen
                    if direction == Direction::Down {
                        if y >= from_height - 10 && y < from_height {
                            // Near bottom edge - switch to to screen
                            y_tables[to_idx][y] = crate::core::COORD_INCR;
                        } else if y < 10 {
                            // Near top edge - came from to screen
                            y_tables[to_idx][y] = crate::core::COORD_DECR;
                        } else {
                            // Normal coordinate - map proportionally
                            let ratio = to_height as f64 / from_height as f64;
                            y_tables[to_idx][y] = (y as f64 * ratio) as i16;
                        }
                    } else if direction == Direction::Up {
                        if y < 10 && y >= 0 {
                            // Near top edge - switch to to screen
                            y_tables[to_idx][y] = crate::core::COORD_INCR;
                        } else if y >= from_height - 10 && y < from_height {
                            // Near bottom edge - came from to screen
                            y_tables[to_idx][y] = crate::core::COORD_DECR;
                        } else {
                            // Normal coordinate - map proportionally
                            let ratio = to_height as f64 / from_height as f64;
                            y_tables[to_idx][y] = (y as f64 * ratio) as i16;
                        }
                    } else {
                        // No connection - keep on from screen
                        y_tables[to_idx][y] = y as i16;
                    }
                } else {
                    y_tables[to_idx][y] = y as i16;
                }
            }
        }

        Ok(())
    }

    /// Map X coordinate from source to target display
    ///
    /// Returns a coordinate value that may be:
    /// - Normal coordinate (>= 0) for use on target display
    /// - COORD_INCR (-1) indicating switch to next screen
    /// - COORD_DECR (-2) indicating switch to previous screen
    pub fn map_x(&self, from_x: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens || from_x < 0 || from_x >= self.from_width as i32 {
            return crate::core::COORD_INCR as i32;
        }

        let coord = self
            .x_tables
            .get(to_screen)
            .and_then(|table| table.get(from_x as usize))
            .copied()
            .unwrap_or(crate::core::COORD_INCR);

        coord as i32
    }

    /// Map Y coordinate from source to target display
    ///
    /// Returns a coordinate value that may be:
    /// - Normal coordinate (>= 0) for use on target display
    /// - COORD_INCR (-1) indicating switch to next screen
    /// - COORD_DECR (-2) indicating switch to previous screen
    pub fn map_y(&self, from_y: i32, to_screen: usize) -> i32 {
        if to_screen >= self.n_screens || from_y < 0 || from_y >= self.from_height as i32 {
            return crate::core::COORD_INCR as i32;
        }

        let coord = self
            .y_tables
            .get(to_screen)
            .and_then(|table| table.get(from_y as usize))
            .copied()
            .unwrap_or(crate::core::COORD_INCR);

        coord as i32
    }

    /// Check if coordinate is special (COORD_INCR or COORD_DECR)
    pub fn is_special(&self, coord: i32) -> bool {
        coord == crate::core::COORD_INCR as i32 || coord == crate::core::COORD_DECR as i32
    }

    /// Get screen width for a target display
    pub fn to_screen_width(&self, to_screen: usize) -> Option<u32> {
        self.x_tables.get(to_screen).map(|table| table.len() as u32)
    }

    /// Get screen height for a target display
    pub fn to_screen_height(&self, to_screen: usize) -> Option<u32> {
        self.y_tables.get(to_screen).map(|table| table.len() as u32)
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
    use crate::core::COORD_DECR;
    use crate::core::COORD_INCR;

    #[test]
    fn test_direction_roundtrip() {
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);
        assert_eq!(Direction::Up, Direction::Up);
        assert_eq!(Direction::Down, Direction::Down);
    }

    #[test]
    fn test_layout_mode_variants() {
        let horizontal = LayoutMode::Horizontal {
            direction: Direction::Right,
        };
        assert_eq!(
            horizontal,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            }
        );

        let vertical = LayoutMode::Vertical {
            direction: Direction::Up,
        };
        assert_eq!(
            vertical,
            LayoutMode::Vertical {
                direction: Direction::Up,
            }
        );
    }

    #[test]
    fn test_special_coordinates() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            },
        )
        .unwrap();

        assert!(mapping.is_special(COORD_INCR as i32));
        assert!(mapping.is_special(COORD_DECR as i32));
        assert!(!mapping.is_special(100));
    }

    #[test]
    fn test_new_with_empty_screens() {
        let from_screen = ScreenInfo {
            screen_num: 0,
            root: 0,
            width: 1920,
            height: 1080,
        };
        let to_screens: Vec<ScreenInfo> = vec![];

        let mapping = CoordinateMapping::new(
            &from_screen,
            &to_screens,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            },
        )
        .unwrap();

        assert_eq!(mapping.n_screens, 0);
        assert_eq!(mapping.x_tables.len(), 0);
        assert_eq!(mapping.y_tables.len(), 0);
    }

    #[test]
    fn test_map_x_out_of_bounds() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            LayoutMode::Horizontal {
                direction: Direction::Right,
            },
        )
        .unwrap();

        // Test out-of-bounds coordinates
        let result = mapping.map_x(-1, 0);
        assert_eq!(result, COORD_INCR as i32);

        let result = mapping.map_x(1920, 0);
        assert_eq!(result, COORD_INCR as i32);
    }

    #[test]
    fn test_map_y_out_of_bounds() {
        let to_screens = vec![];

        let mapping = CoordinateMapping::new(
            &ScreenInfo {
                screen_num: 0,
                root: 0,
                width: 1920,
                height: 1080,
            },
            &to_screens,
            LayoutMode::Vertical {
                direction: Direction::Up,
            },
        )
        .unwrap();

        // Test out-of-bounds coordinates
        let result = mapping.map_y(-1, 0);
        assert_eq!(result, COORD_INCR as i32);

        let result = mapping.map_y(1080, 0);
        assert_eq!(result, COORD_INCR as i32);
    }
}
