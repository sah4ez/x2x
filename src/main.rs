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
use log::{debug, error, info};

use utils::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    // Parse command line arguments
    let config = Config::parse();

    debug!("Configuration: {:?}", config);

    info!("Starting x2x-rust...");

    // TODO: Implement main logic
    // 1. Open X11 connections
    // 2. Initialize DpyInfo
    // 3. Register event handlers
    // 4. Run event loop

    error!("Not yet implemented - see RUST_REFACTOR_PLAN.md for implementation plan");

    Ok(())
}
