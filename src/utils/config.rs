//! Configuration parsing and management
//!
//! This module handles command-line argument parsing using clap.

use clap::Parser;
use std::path::PathBuf;

/// x2x-rust: Control X displays with shared keyboard/mouse
#[derive(Parser, Debug)]
#[command(name = "x2x")]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Display to control from
    #[arg(short = 'f', long, value_name = "DISPLAY")]
    pub from_display: Option<String>,

    /// Display to control to (required)
    #[arg(short = 't', long, value_name = "DISPLAY")]
    pub to_display: String,

    /// Connect on screen edge: east, west, north, south
    #[arg(short = 'e', long, value_enum)]
    pub edge: Option<EdgeDirection>,

    /// Wait for display if unavailable
    #[arg(short = 'w', long)]
    pub wait: bool,

    /// Block mouse buttons during auto-disconnect
    #[arg(long)]
    pub btn_block: bool,

    /// Vertical screen layout (instead of horizontal)
    #[arg(long)]
    pub vertical: bool,

    /// Font name for status window
    #[arg(short = 'F', long)]
    pub font: Option<String>,

    /// Enable debug logging
    #[arg(short = 'd', long)]
    pub debug: bool,

    /// Config file path (optional)
    #[arg(short = 'c', long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,
}

impl Config {
    /// Get the X display name for the "from" display
    pub fn from_display_name(&self) -> Option<&str> {
        self.from_display.as_deref()
    }

    /// Get the X display name for the "to" display
    pub fn to_display_name(&self) -> &str {
        &self.to_display
    }

    /// Determine if layout is vertical
    pub fn is_vertical(&self) -> bool {
        self.vertical
    }

    /// Get log level from config
    pub fn log_level(&self) -> &str {
        if self.debug {
            "debug"
        } else {
            "info"
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            from_display: None,
            to_display: String::from("localhost:1"),
            edge: None,
            wait: false,
            btn_block: false,
            vertical: false,
            font: None,
            debug: false,
            config_file: None,
        }
    }
}

/// Screen edge direction for connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum EdgeDirection {
    East,
    West,
    North,
    South,
}

impl EdgeDirection {
    /// Convert to internal direction type
    pub fn to_direction(&self) -> crate::core::Direction {
        match self {
            EdgeDirection::East => crate::core::Direction::Right,
            EdgeDirection::West => crate::core::Direction::Left,
            EdgeDirection::North => crate::core::Direction::Up,
            EdgeDirection::South => crate::core::Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.to_display, "localhost:1");
        assert!(!config.wait);
        assert!(!config.vertical);
    }

    #[test]
    fn test_edge_direction_conversion() {
        assert_eq!(EdgeDirection::East.to_direction(), crate::core::Direction::Right);
        assert_eq!(EdgeDirection::West.to_direction(), crate::core::Direction::Left);
        assert_eq!(EdgeDirection::North.to_direction(), crate::core::Direction::Up);
        assert_eq!(EdgeDirection::South.to_direction(), crate::core::Direction::Down);
    }
}
