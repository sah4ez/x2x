# x2x-rust

Control X displays with shared keyboard and mouse.

## Overview

x2x-rust is a modern Rust rewrite of the classic x2x utility. It allows you to use the keyboard and mouse on one X display to control another X display, with clipboard sharing support.

## Features

- **X11 Integration**: Full support for X11 displays using x11-dl
- **Input Processing**: Mouse and keyboard event handling
- **Coordinate Mapping**: Seamless cursor movement across displays
- **Clipboard Sharing**: Bidirectional clipboard sharing (PRIMARY, SECONDARY, CLIPBOARD)
- **Connection Management**: Connect/disconnect with focus management
- **CLI**: Modern command-line interface with clap
- **UTF-8 Support**: Full Unicode support

## Installation

### Prerequisites

- X11 development libraries
- Rust toolchain (stable)

```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev libxext-dev libxtst-dev

# Fedora/RHEL
sudo dnf install libX11-devel libXext-devel libXtst-devel
```

### Build from source

```bash
git clone https://github.com/your-username/x2x-rust.git
cd x2x-rust
cargo build --release
```

## Usage

### Basic usage

```bash
# Control local display :0 to control display :1
./x2x-rust -t :1

# Control from display :0 to display :1
./x2x-rust -f :0 -t :1
```

### Command-line options

```
x2x-rust [OPTIONS]

OPTIONS:
    -f, --from <DISPLAY>        Display to control from (default: current display)
    -t, --to <DISPLAY>          Display to control to [required]
    -e, --edge <EDGE>          Connect on screen edge: east, west, north, south
    -w, --wait                  Wait for display if unavailable
        --btn-block              Block mouse buttons during auto-disconnect
        --vertical                Vertical screen layout (instead of horizontal)
    -F, --font <FONT>           Font name for status window
    -d, --debug                 Enable debug logging
    -c, --config <FILE>         Config file path
    -h, --help                   Print help information
    -V, --version                Print version information
```

### Examples

```bash
# Connect to remote display
x2x-rust -f :0 -t user@remote:0

# Use vertical layout with custom font
x2x-rust -t localhost:1 --vertical --font "fixed-13"

# Enable debug logging
x2x-rust -t :1 --debug

# Wait for display to become available
x2x-rust -t :1 --wait
```

## Screen Layouts

### Horizontal layout (default)

```
┌─────────────┐  ┌─────────────┐
│   FROM      │──▶│    TO       │
│   Display   │  │   Display   │
│   (:0)      │  │   (:1)      │
└─────────────┘  └─────────────┘
```

### Vertical layout

```
┌─────────────┐
│   FROM      │
│   Display   │
└─────────────┘
       │
       ▼
┌─────────────┐
│    TO       │
│   Display   │
└─────────────┘
```

## Clipboard Support

x2x-rust shares clipboard data between displays using the X Selection mechanism:

- **PRIMARY**: Text selection (highlighted text)
- **SECONDARY**: Secondary selection buffer
- **CLIPBOARD**: Standard clipboard (Ctrl+C / Ctrl+V)

Clipboard data is automatically synchronized in both directions.

## Keyboard Handling

- Full keyboard event processing
- Sticky keys support (modifiers "stick" after pressing once)
- Modifier key handling (Shift, Ctrl, Alt, Meta/Super)
- UTF-8 key mapping

## Mouse Handling

- Motion events with coordinate mapping
- Button press/release
- Multi-screen crossing detection
- Pointer grabbing for exclusive control

## Architecture

```
x2x-rust/
├── src/
│   ├── main.rs              # Entry point
│   ├── x11/                 # X11 bindings
│   │   ├── connection.rs     # X11 connection wrapper
│   │   ├── event.rs         # Event handling
│   │   ├── extension.rs     # XTest, DPMS
│   │   ├── clipboard.rs     # X Selection clipboard
│   │   └── error_handler.rs # Error handling
│   ├── core/                # Core data structures
│   │   ├── dpy_info.rs      # Display info
│   │   ├── coord_mapping.rs # Coordinate mapping
│   │   ├── state.rs        # State management
│   │   └── event_loop.rs   # Event loop
│   ├── input/               # Input handling
│   │   ├── mouse.rs        # Mouse handler
│   │   ├── keyboard.rs     # Keyboard handler
│   │   └── fake.rs         # Fake event queue
│   ├── clipboard/           # Clipboard manager
│   │   └── mod.rs          # High-level clipboard
│   ├── connection/          # Connection management
│   │   └── mod.rs          # Connection manager
│   └── utils/               # Utilities
│       ├── config.rs       # CLI configuration
│       └── errors.rs       # Error types
└── tests/                   # Integration tests
```

## Development

### Run tests

```bash
# Unit tests
cargo test

# Integration tests (requires X server)
cargo test --test integration_test

# All tests
cargo test --all
```

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Lint

```bash
# Check for issues
cargo clippy

# Format code
cargo fmt
```

## Comparison with original x2x

| Feature | Original (C) | x2x-rust |
|---------|---------------|------------|
| Memory safety | ❌ Manual | ✅ Guaranteed |
| Thread safety | ❌ Manual | ✅ Built-in |
| Error handling | ❌ Return codes | ✅ Result<T> |
| Type safety | ⚠️ C types | ✅ Strong types |
| Modern tooling | ❌ Makefiles | ✅ Cargo |
| Documentation | ⚠️ Man pages | ✅ Rustdoc |
| Testing | ⚠️ Manual | ✅ Built-in |

## License

Original x2x is licensed under the X Consortium License. This Rust rewrite maintains compatibility with the original license.

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Ensure all tests pass
6. Submit a pull request

## Issues

Report issues and feature requests on the project's GitHub page.

## See Also

- Original x2x: https://github.com/dottedmag/x2x
- Xlib documentation: https://www.x.org/releases/current/doc/libX11/
- x11-dl: https://github.com/Daggerbot/x11-rs
