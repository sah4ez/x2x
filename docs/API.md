# API Documentation

## Overview

x2x-rust provides a comprehensive API for managing X displays and sharing input between them.

## Core Types

### DpyInfo

Main display information structure holding state for both displays.

```rust
pub struct DpyInfo {
    // From display (source)
    pub from_conn: Arc<X11Connection>,
    pub from_root: u64,
    pub from_trigger: Option<u64>,
    pub from_big: Option<u64>,
    pub from_screen_info: ScreenInfo,

    // To display (target)
    pub to_conn: Arc<X11Connection>,
    pub to_root: u64,
    pub to_screen_info: ScreenInfo,

    // Connection state
    pub mode: ConnectionMode,
    pub to_screen: usize,
    pub last_from_coord: i32,
    pub unreasonable_delta: i32,

    // Coordinate mapping
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub from_conn_coord: i32,
    pub from_disc_coord: i32,

    // Input state
    pub fake_queue: FakeQueue,
    pub button_mapping: [u8; N_BUTTONS],
    pub sticky_keys: StickyKeys,
    pub current_x: i32,
    pub current_y: i32,
    pub button_state: u32,
}
```

**Methods:**

- `new(from_conn, to_conn) -> Result<Self>`: Create new DpyInfo
- `connect(&mut self) -> Result<()>`: Connect to other display
- `disconnect(&mut self) -> Result<()>`: Disconnect from other display
- `update_pointer(&mut self, x, y)`: Update pointer position
- `is_connected(&self) -> bool`: Check connection state

### X11Connection

X11 connection wrapper with RAII semantics.

```rust
pub struct X11Connection {
    display: *mut Display,
    screen: i32,
}
```

**Methods:**

- `open(display_name: Option<&str>) -> Result<Self>`: Open X display
- `display_ptr(&self) -> *mut Display`: Get raw display pointer
- `screen(&self) -> i32`: Get screen number
- `root_window(&self) -> Window`: Get root window
- `screen_width(&self, screen: i32) -> i32`: Get screen width
- `screen_height(&self, screen: i32) -> i32`: Get screen height

### ConnectionManager

Manages connection/disconnection between displays.

```rust
pub struct ConnectionManager {
    from_conn: Arc<X11Connection>,
    to_conn: Arc<X11Connection>,
    trigger_window: Window,
    big_window: Option<Window>,
    mode: ConnectionMode,
    auto_up: bool,
    event_mask: c_uint,
    window_attr: WindowAttributes,
}
```

**Methods:**

- `new(...) -> Self`: Create new connection manager
- `connect(&mut self, from_focus, to_focus) -> Result<()>`: Connect to other display
- `disconnect(&mut self, from_focus, to_focus, clipboard_owner) -> Result<()>`: Disconnect
- `is_connected(&self) -> bool`: Check connection state
- `mode(&self) -> ConnectionMode`: Get current mode

### X11Clipboard

X11 clipboard manager for inter-display sharing.

```rust
pub struct X11Clipboard {
    conn: Arc<X11Connection>,
    selections: Mutex<HashMap<Selection, SelectionData>>,
    atoms: Mutex<AtomCache>,
    prop_window: Window,
    ping_atom: Atom,
    ping_in_progress: Mutex<bool>,
    last_timestamp: Mutex<Time>,
}
```

**Methods:**

- `new(conn, prop_window, ping_atom) -> Result<Self>`: Create clipboard manager
- `handle_selection_request(&self, event, is_from_display) -> Result<bool>`: Handle SelectionRequest
- `handle_selection_notify(&self, event, is_from_display) -> Result<bool>`: Handle SelectionNotify
- `handle_selection_clear(&self, event, is_from_display) -> Result<bool>`: Handle SelectionClear
- `get_data(&self, selection) -> Option<Vec<u8>>`: Get clipboard data
- `set_data(&self, selection, data, format) -> Result<()>`: Set clipboard data
- `is_owned(&self, selection) -> bool`: Check if we own selection

### ClipboardManager

High-level clipboard manager coordinating two X11Clipboard instances.

```rust
pub struct ClipboardManager {
    from_clipboard: X11Clipboard,
    to_clipboard: X11Clipboard,
    last_from_data: Option<Vec<u8>>,
    last_to_data: Option<Vec<u8>>,
}
```

**Methods:**

- `new(...) -> Result<Self>`: Create clipboard manager
- `handle_from_event(&mut self, event, ctx) -> Result<()>`: Handle event from display
- `handle_to_event(&mut self, event, ctx) -> Result<()>`: Handle event to display
- `has_from_changed(&self) -> bool`: Check if from clipboard changed
- `has_to_changed(&self) -> bool`: Check if to clipboard changed
- `sync_from_to(&mut self) -> Result<()>`: Sync clipboard from→to
- `sync_to_from(&mut self) -> Result<()>`: Sync clipboard to→from

## Enums

### ConnectionMode

```rust
pub enum ConnectionMode {
    Disconnected,
    Connected,
}
```

### Selection

```rust
pub enum Selection {
    Primary,      // X11 PRIMARY
    Secondary,    // X11 SECONDARY
    Clipboard,    // X11 CLIPBOARD
}
```

### ClipboardTarget

```rust
pub enum ClipboardTarget {
    Utf8String,  // UTF-8 encoded text
    String,       // Latin-1 encoded text
    Text,         // Generic text
    Multiple,     // Multiple targets
    Incremental,  // Incremental transfer
}
```

### SelectionState

```rust
pub enum SelectionState {
    Off,   // Selection not active
    On,    // We own the selection
    Wait,   // Waiting for selection request response
}
```

### Direction

```rust
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
```

## Configuration

### Config

CLI configuration parsed with clap.

```rust
pub struct Config {
    pub from_display: Option<String>,    // Source display
    pub to_display: String,             // Target display
    pub edge: Option<EdgeDirection>,    // Screen edge
    pub wait: bool,                    // Wait for display
    pub btn_block: bool,               // Block buttons
    pub vertical: bool,                // Vertical layout
    pub font: Option<String>,           // Status font
    pub debug: bool,                   // Debug logging
    pub config_file: Option<PathBuf>,   // Config file
}
```

**Methods:**

- `parse() -> Self`: Parse CLI arguments
- `from_display_name(&self) -> Option<&str>`: Get from display name
- `to_display_name(&self) -> &str`: Get to display name
- `is_vertical(&self) -> bool`: Check if vertical layout
- `log_level(&self) -> &str`: Get log level

## Event Handling

### InputHandler

Trait for input event handlers.

```rust
pub trait InputHandler {
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool>;
}
```

### XEvent

Enum for X11 events.

```rust
pub enum XEvent {
    MotionNotify(XMotionEvent),
    ButtonPress(XButtonEvent),
    ButtonRelease(XButtonEvent),
    KeyPress(XKeyEvent),
    KeyRelease(XKeyEvent),
    SelectionRequest(XSelectionRequestEvent),
    SelectionNotify(XSelectionEvent),
    SelectionClear(XSelectionClearEvent),
    // ... more events
}
```

## Coordinate Mapping

### CoordinateMapping

Coordinate transformation tables.

```rust
pub struct CoordinateMapping {
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub n_screens: usize,
    pub from_width: u32,
    pub from_height: u32,
}
```

**Methods:**

- `new(from_screen, to_screens, mode) -> Result<Self>`: Create mapping
- `map_x(&self, from_x, to_screen) -> i32`: Map X coordinate
- `map_y(&self, from_y, to_screen) -> i32`: Map Y coordinate

## Usage Examples

### Basic setup

```rust
use x2x_rust::{Config, X11Connection, DpyInfo};

let config = Config::parse();
let from_conn = X11Connection::open(config.from_display_name())?;
let to_conn = X11Connection::open(Some(config.to_display_name()))?;
let mut dpy_info = DpyInfo::new(Arc::new(from_conn), Arc::new(to_conn))?;
```

### Connection management

```rust
use x2x_rust::connection::ConnectionManager;

let mut conn_manager = ConnectionManager::new(
    dpy_info.from_conn.clone(),
    dpy_info.to_conn.clone(),
    trigger_window,
    event_mask,
    auto_up,
);

let mut from_focus = FocusState::default();
let mut to_focus = FocusState::default();

// Connect to other display
conn_manager.connect(&mut from_focus, &mut to_focus)?;

// ... use the connection ...

// Disconnect
conn_manager.disconnect(&mut from_focus, &mut to_focus, None)?;
```

### Clipboard operations

```rust
use x2x_rust::clipboard::ClipboardManager;
use x2x_rust::x11::{Selection, ClipboardTarget};

let clipboard = ClipboardManager::new(
    from_conn.clone(),
    to_conn.clone(),
    prop_window_from,
    prop_window_to,
    ping_atom,
)?;

// Set clipboard data
clipboard.set_from_data(Selection::Primary, b"Hello, world!".to_vec(), ClipboardTarget::Utf8String)?;

// Get clipboard data
if let Some(data) = clipboard.get_from_data(Selection::Primary) {
    println!("Clipboard: {}", String::from_utf8_lossy(&data));
}
```

## Error Handling

All operations return `Result<T>` from `anyhow`.

```rust
use anyhow::{Result, Context};

fn connect_to_display() -> Result<()> {
    let conn = X11Connection::open(None)
        .context("Failed to open X display")?;
    // ... use conn ...
    Ok(())
}
```

## Testing

### Unit tests

```bash
cargo test
```

### Integration tests

```bash
cargo test --test integration_test
```

### Documentation tests

```bash
cargo test --doc
```

## Platform Support

- Linux: ✅ Full support
- FreeBSD: ✅ (X11 required)
- macOS: ⚠️ (XQuartz required)
- Windows: ❌ (not supported, requires X server)
