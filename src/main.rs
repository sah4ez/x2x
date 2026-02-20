//! x2x-rust: Control X displays with shared keyboard/mouse
//!
//! This is a Rust rewrite of the original x2x utility (C implementation).
//!
//! ## Overview
//!
//! x2x allows the keyboard and mouse on one X display to control another X display.
//! It also shares X clipboards between the displays.
//!
//! ## Architecture
//!
//! The application is divided into several modules:
//!
//! - **x11**: X11 bindings and low-level X operations
//! - **core**: Core data structures (DpyInfo, ShadowDisplay, coordinate mapping)
//! - **input**: Input event handling (mouse, keyboard, fake event queue)
//! - **clipboard**: Clipboard sharing via X Selection mechanism
//! - **utils**: Error handling, configuration, and utilities
//! - **win32**: Windows/Cygwin support (optional, behind `win32` feature)

mod x11;
mod core;
mod input;
mod clipboard;
mod utils;

#[cfg(feature = "win32")]
mod win32;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, warn};

use utils::config::Config;
use x11::X11Connection;
use core::DpyInfo;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    // Parse command line arguments
    let config = Config::parse();

    debug!("Configuration: {:?}", config);

    info!("Starting x2x-rust...");

    // Open X11 connections
    let from_display_name = config.from_display_name().unwrap_or(":0");
    let to_display_name = config.to_display_name();

    info!("Opening connections: {} -> {}", from_display_name, to_display_name);

    let from_conn = X11Connection::open(Some(from_display_name))
        .with_context(|| format!("Failed to open display: {}", from_display_name))?;

    let to_conn = X11Connection::open(Some(to_display_name))
        .with_context(|| format!("Failed to open display: {}", to_display_name))?;

    info!("X11 connections opened successfully");

    // Check XTest extension on target display
    if !to_conn.has_xtest() {
        warn!("XTest extension not available on target display");
        warn!("Input simulation may not work");
    }

    // Initialize display info
    let from_screen_info = from_conn.current_screen_info()?;
    let to_screen_info = to_conn.current_screen_info()?;

    info!("From display: {}x{}", from_screen_info.width, from_screen_info.height);
    info!("To display: {}x{}", to_screen_info.width, to_screen_info.height);

    // Create DpyInfo
    let from_conn_arc = std::sync::Arc::new(from_conn);
    let to_conn_arc = std::sync::Arc::new(to_conn);

    let mut dpy_info = DpyInfo::new(from_conn_arc, to_conn_arc)?;

    info!("Display info initialized");

    // TODO: Run event loop
    // For now, just demonstrate that everything compiles
    info!("Event loop not yet implemented - see RUST_REFACTOR_PLAN.md");

    Ok(())
}
