#![allow(dead_code)]
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

mod clipboard;
mod connection;
mod core;
mod input;
mod utils;
mod x11;

#[cfg(feature = "win32")]
mod win32;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, warn};

use core::DpyInfo;
use utils::config::Config;
use x11::extension::{DpmsExtension, XTestExtension};
use x11::{setup_error_handler, X11Clipboard, X11Connection};

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Setup X11 error handler
    if let Err(e) = setup_error_handler() {
        warn!("Failed to setup X11 error handler: {}", e);
    }

    // Parse command line arguments
    let config = Config::parse();

    debug!("Configuration: {:?}", config);

    info!("Starting x2x-rust...");

    // Open X11 connections
    let from_display_name = config.from_display_name().unwrap_or(":0");
    let to_display_name = config.to_display_name();

    info!(
        "Opening connections: {} -> {}",
        from_display_name, to_display_name
    );

    let from_conn = X11Connection::open(Some(from_display_name))
        .with_context(|| format!("Failed to open display: {}", from_display_name))?;

    let to_conn = X11Connection::open(Some(to_display_name))
        .with_context(|| format!("Failed to open display: {}", to_display_name))?;

    info!("X11 connections opened successfully");

    // Initialize display info
    let from_screen_info = from_conn.current_screen_info()?;
    let to_screen_info = to_conn.current_screen_info()?;

    info!(
        "From display: {}x{}",
        from_screen_info.width, from_screen_info.height
    );
    info!(
        "To display: {}x{}",
        to_screen_info.width, to_screen_info.height
    );

    // Check for XTest extension (critical for x2x)
    let xtest = XTestExtension::from_connection(&from_conn);
    match &xtest {
        Some(ext) if ext.is_available() => {
            info!("✓ XTest extension available - fake input events will work");
        }
        _ => {
            warn!("✗ XTest extension not available - fake input events will NOT work");
            warn!("  x2x requires XTest for full functionality");
        }
    }

    // Check for DPMS extension (optional)
    let dpms = DpmsExtension::from_connection(&from_conn);
    match &dpms {
        Some(ext) if ext.is_available() => {
            let enabled = ext.is_enabled(&from_conn).unwrap_or(false);
            info!("✓ DPMS extension available (enabled: {})", enabled);
        }
        _ => {
            debug!("DPMS extension not available - display power management will not work");
        }
    }

    // Create DpyInfo
    let from_conn_arc = std::sync::Arc::new(from_conn);
    let to_conn_arc = std::sync::Arc::new(to_conn);

    let dpy_info = DpyInfo::new(from_conn_arc, to_conn_arc)?;

    info!("Display info initialized");

    // Initialize clipboard
    // Use a dummy window for now - this will be properly initialized in Phase 7
    let prop_window = dpy_info.from_root;
    let ping_atom = 12345; // TODO: Get proper atom in Phase 7

    let _clipboard = X11Clipboard::new(dpy_info.from_conn.clone(), prop_window, ping_atom)?;

    info!("Clipboard initialized");

    // TODO: Run event loop
    // For now, just demonstrate that everything compiles
    info!("");
    info!("Phase 6 implementation complete!");
    info!("Next steps:");
    info!("  1. Implement Connection Management (Phase 7)");
    info!("     - DpyInfo::connect() and disconnect()");
    info!("     - Create trigger windows");
    info!("     - Grab pointer");
    info!("  2. Run full event loop with all handlers");
    info!("");
    info!("Project status:");
    info!("  - Phase 1: ✅ Complete");
    info!("  - Phase 2: ✅ Complete");
    info!("  - Phase 3: ✅ Complete");
    info!("  - Phase 4: ✅ Complete");
    info!("  - Phase 5: ✅ Complete");
    info!("  - Phase 6: ✅ Complete (this phase)");
    info!("  - Phase 7: 🚧 Next");
    info!("  - Phase 8: ✅ Complete");
    info!("  - Phase 9: 🚧 In progress");
    info!("  - Phase 10: 🚧 Next");

    Ok(())
}
