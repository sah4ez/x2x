//! Display information structures

use crate::x11::X11Connection;
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::Arc;

/// Main display information structure
#[derive(Debug)]
pub struct DpyInfo {
    // From display
    pub from_conn: Arc<X11Connection>,
    pub from_root: u64,
    pub from_trigger: Option<u64>,
    pub from_big: Option<u64>,

    // To display
    pub to_conn: Arc<X11Connection>,
    pub to_root: u64,

    // Connection state
    pub mode: ConnectionMode,
    pub to_screen: usize,
    pub last_from_coord: i32,
    pub unreasonable_delta: i32,

    // Coordinate mapping tables
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub from_conn_coord: i32,
    pub from_disc_coord: i32,
    pub from_incr_coord: i32,
    pub from_decr_coord: i32,

    // Selection state
    pub selection_state: SelectionState,

    // Input state tracking
    pub fake_queue: VecDeque<super::FakeEvent>,
    pub button_mapping: [u8; super::N_BUTTONS],

    // Shadow displays
    pub shadows: Vec<ShadowDisplay>,
}

impl DpyInfo {
    /// Create a new DpyInfo structure
    pub fn new(
        from_conn: Arc<X11Connection>,
        to_conn: Arc<X11Connection>,
    ) -> Result<Self> {
        let from_root = from_conn.root_window();
        let to_root = to_conn.root_window();

        Ok(Self {
            from_conn,
            to_conn,
            from_root,
            to_root,
            from_trigger: None,
            from_big: None,
            mode: ConnectionMode::Disconnected,
            to_screen: 0,
            last_from_coord: 0,
            unreasonable_delta: 10,
            x_tables: Vec::new(),
            y_tables: Vec::new(),
            from_conn_coord: 0,
            from_disc_coord: 0,
            from_incr_coord: 0,
            from_decr_coord: 0,
            selection_state: SelectionState::default(),
            fake_queue: VecDeque::new(),
            button_mapping: [0; super::N_BUTTONS],
            shadows: Vec::new(),
        })
    }

    /// Connect to the other display
    pub fn connect(&mut self) -> Result<()> {
        // TODO: Implement DoConnect logic
        todo!("Implement DpyInfo::connect")
    }

    /// Disconnect from the other display
    pub fn disconnect(&mut self) -> Result<()> {
        // TODO: Implement DoDisconnect logic
        todo!("Implement DpyInfo::disconnect")
    }
}

/// Shadow display for multi-monitor setups
#[derive(Debug, Clone)]
pub struct ShadowDisplay {
    pub name: String,
    pub conn: Arc<X11Connection>,
    pub led_mask: u64,
    pub flush_required: bool,
    pub dpms_status: DpmsStatus,
}

/// Connection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMode {
    Disconnected,
    Connected,
}

/// DPMS status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpmsStatus {
    Unknown,
    NotSupported,
    Supported { level: u16 },
}

/// Selection state
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub state: SelectionInternalState,
    pub owner: Option<u64>,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionInternalState {
    Off,
    On,
    Wait,
}

impl Default for SelectionInternalState {
    fn default() -> Self {
        Self::Off
    }
}
