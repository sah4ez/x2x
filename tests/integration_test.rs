//! Integration tests for x2x-rust

// These tests require a real X server running
// Run with: cargo test --test integration_test -- --ignored

use anyhow::Result;
use x2x_rust::{Config, DpyInfo, X11Connection};

/// Integration test for X11 connection
#[test]
#[ignore = "requires X server"]
fn test_x11_connection() -> Result<()> {
    let config = Config::parse();

    // Open X display
    let conn = X11Connection::open(None)?;

    println!("Successfully opened X display");
    println!("Screen: {}", conn.screen());
    println!("Root window: 0x{:x}", conn.root_window());

    Ok(())
}

/// Integration test for coordinate mapping
#[test]
#[ignore = "requires two X displays"]
fn test_coordinate_mapping_integration() -> Result<()> {
    // This test would require two X displays
    // For now, it's a placeholder for future implementation

    Ok(())
}

/// Integration test for clipboard sharing
#[test]
#[ignore = "requires two X displays"]
fn test_clipboard_integration() -> Result<()> {
    // This test would verify clipboard sharing between displays
    // For now, it's a placeholder for future implementation

    Ok(())
}

/// Integration test for connection management
#[test]
#[ignore = "requires two X displays"]
fn test_connection_management_integration() -> Result<()> {
    // This test would verify connect/disconnect logic
    // For now, it's a placeholder for future implementation

    Ok(())
}
