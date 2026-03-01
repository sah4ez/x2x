# x2x-rust

Control X displays with shared keyboard/mouse

This is a Rust rewrite of the [original x2x](https://github.com/dottedmag/x2x) utility.

## Status

**🚧 Work in Progress - See [RUST_REFACTOR_PLAN.md](RUST_REFACTOR_PLAN.md)**

This is an early-stage rewrite. Many features are not yet implemented.

## What is x2x?

x2x allows the keyboard and mouse on one X display to control another X display. It also shares X clipboards between the displays.

## Building

### Prerequisites

- Rust 1.70+ with Cargo
- X11 development libraries:
  - `libx11-dev` (Debian/Ubuntu)
  - `libxext-dev` (Debian/Ubuntu)
  - `libxtst-dev` (Debian/Ubuntu)

### Install dependencies

```bash
# Debian/Ubuntu
sudo apt-get install libx11-dev libxext-dev libxtst-dev

# Fedora
sudo dnf install libX11-devel libXext-devel libXtst-devel

# Arch
sudo pacman -S libx11 libxext libxtst
```

### Build

```bash
cargo build --release
```

The binary will be at `target/release/x2x-rust`.

## Usage

```bash
# Basic usage
x2x-rust -to :1

# Specify source display
x2x-rust -from :0 -to :1

# Connect on east edge
x2x-rust -to :1 -edge east

# Vertical layout
x2x-rust -to :1 -edge south -vertical

# Wait for display
x2x-rust -to :1 -wait
```

### Options

```
-f, --from <DISPLAY>      Display to control from
-t, --to <DISPLAY>        Display to control to (required)
-e, --edge <EDGE>         Connect on screen edge: east, west, north, south
-w, --wait                Wait for display if unavailable
--btn-block               Block mouse buttons during auto-disconnect
--vertical                Vertical screen layout
-F, --font <FONT>         Font name for status window
-d, --debug               Enable debug logging
-c, --config <FILE>       Config file path (not yet implemented)
```

## Development

### Project Structure

```
src/
├── main.rs           # Entry point
├── x11/              # X11 bindings and operations
│   ├── connection.rs # X11 connection management
│   ├── event.rs      # Event types and handlers
│   ├── extension.rs  # XTest, DPMS extensions
│   └── selection.rs  # Clipboard handling
├── core/             # Core data structures
│   ├── dpy_info.rs   # Display information
│   ├── coord_mapping.rs # Coordinate transformation
│   └── state.rs      # Input state (fake events, sticky keys)
├── input/            # Input event handling
│   ├── mouse.rs      # Mouse input
│   ├── keyboard.rs   # Keyboard input
│   └── fake.rs       # Fake event queue
├── clipboard/        # Clipboard sharing
│   └── x11_selection.rs # X Selection mechanism
├── utils/            # Utilities
│   ├── config.rs     # CLI configuration
│   └── errors.rs     # Error types
└── win32/            # Windows support (feature: win32)
    ├── keymap.rs     # Windows → X key mapping
    └── window.rs     # Windows window management
```

### Running Tests

```bash
# Unit tests
cargo test

# With output
cargo test -- --nocapture

# Run specific test
cargo test test_fake_queue

# Coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

### Linting

```bash
# Check code
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Roadmap

See [RUST_REFACTOR_PLAN.md](RUST_REFACTOR_PLAN.md) for the complete implementation plan.

### Milestones

- [x] Project setup and structure
- [ ] X11 bindings and basic framework
- [ ] Core data structures
- [ ] Coordinate mapping
- [ ] Mouse input handling
- [ ] Keyboard input handling
- [ ] Clipboard sharing
- [ ] Connection management
- [ ] CLI argument parsing
- [ ] Testing and debugging
- [ ] Windows support (optional)

## Contributing

Contributions are welcome! Please read [RUST_REFACTOR_PLAN.md](RUST_REFACTOR_PLAN.md) first to understand the implementation direction.

## License

MIT/BSD license (same as original x2x).

Windows support includes code under GPLv2+.

## Acknowledgments

- Original x2x by David Chaiken (DEC)
- Current C maintainer: Mikhail Gusarov
- This Rust rewrite is based on the work of the x2x community
