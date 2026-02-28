//! X11 Clipboard implementation for inter-display sharing

use crate::x11::{X11Connection, Window, Atom, Time, X11Error};
use anyhow::{Context, Result};
use log::{info, warn, debug, error};
use std::collections::HashMap;
use std::os::raw::{c_char, c_int, c_uchar, c_void};
use std::sync::{Arc, Mutex};
use std::ffi::CString;
use std::ptr;

use x11_dl::xlib::{
    self,
    Display,
    XEvent,
    XSelectionRequestEvent,
    XSelectionEvent,
    XSelectionClearEvent,
    XSelectionEvent as XSelectionEventLib,
    XSelectionRequestEvent as XSelectionRequestEventLib,
    XSelectionClearEvent as XSelectionClearEventLib,
    Atom as XlibAtom,
    Window as XlibWindow,
    Time as XlibTime,
    CurrentTime,
    PropModeReplace,
    PropModeAppend,
    True,
    False,
};

/// Clipboard selection types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Selection {
    /// Primary selection (X11 PRIMARY)
    Primary,
    /// Secondary selection (X11 SECONDARY)
    Secondary,
    /// Clipboard selection (X11 CLIPBOARD)
    Clipboard,
}

impl Selection {
    /// Get the atom name for this selection type
    pub fn atom_name(&self) -> &'static str {
        match self {
            Selection::Primary => "PRIMARY",
            Selection::Secondary => "SECONDARY",
            Selection::Clipboard => "CLIPBOARD",
        }
    }

    /// Get the atom name in lowercase
    pub fn atom_name_lower(&self) -> &'static str {
        match self {
            Selection::Primary => "primary",
            Selection::Secondary => "secondary",
            Selection::Clipboard => "clipboard",
        }
    }
}

/// Clipboard target types (formats)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipboardTarget {
    /// UTF-8 string
    Utf8String,
    /// Latin-1 string
    String,
    /// Text (generic)
    Text,
    /// Multiple targets (not yet supported)
    Multiple,
    /// Incremental transfer (not yet supported)
    Incremental,
}

impl ClipboardTarget {
    /// Get the atom name for this target type
    pub fn atom_name(&self) -> &'static str {
        match self {
            ClipboardTarget::Utf8String => "UTF8_STRING",
            ClipboardTarget::String => "STRING",
            ClipboardTarget::Text => "TEXT",
            ClipboardTarget::Multiple => "MULTIPLE",
            ClipboardTarget::Incremental => "INCR",
        }
    }
}

/// Selection state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    /// Selection is not active
    Off,
    /// Selection is active (we own it)
    On,
    /// Waiting for selection request response
    Wait,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self::Off
    }
}

/// Clipboard data
#[derive(Debug, Clone)]
pub struct ClipboardData {
    /// Raw data bytes
    pub data: Vec<u8>,
    /// Data format (target type)
    pub format: ClipboardTarget,
    /// Timestamp when data was acquired
    pub timestamp: Time,
    /// Data revision counter
    pub revision: u32,
}

impl ClipboardData {
    /// Create new clipboard data
    pub fn new(data: Vec<u8>, format: ClipboardTarget, timestamp: Time) -> Self {
        Self {
            data,
            format,
            timestamp,
            revision: 0,
        }
    }

    /// Get data as UTF-8 string
    pub fn as_utf8(&self) -> Option<String> {
        if self.format == ClipboardTarget::Utf8String {
            String::from_utf8(self.data.clone()).ok()
        } else {
            // Try to convert from Latin-1
            String::from_utf8_lossy(&self.data).to_string().into()
        }
    }

    /// Increment revision counter
    pub fn bump_revision(&mut self) {
        self.revision += 1;
    }
}

/// Clipboard selection data
#[derive(Debug, Clone)]
pub struct SelectionData {
    /// The clipboard data
    pub data: Option<ClipboardData>,
    /// Current selection state
    pub state: SelectionState,
    /// Owner window (if we own it)
    pub owner_window: Option<Window>,
    /// Ping timestamp for synchronization
    pub ping_time: Option<Time>,
    /// Property window for data transfer
    pub prop_window: Window,
}

impl SelectionData {
    /// Create new selection data
    pub fn new(prop_window: Window) -> Self {
        Self {
            data: None,
            state: SelectionState::Off,
            owner_window: None,
            ping_time: None,
            prop_window,
        }
    }

    /// Check if we own the selection
    pub fn is_owned(&self) -> bool {
        self.state == SelectionState::On
    }

    /// Get the current data
    pub fn get_data(&self) -> Option<&ClipboardData> {
        self.data.as_ref()
    }

    /// Set the selection data
    pub fn set_data(&mut self, data: ClipboardData) {
        self.data = Some(data);
        if let Some(d) = &mut self.data {
            d.bump_revision();
        }
    }

    /// Clear the selection data
    pub fn clear_data(&mut self) {
        self.data = None;
    }

    /// Set selection state
    pub fn set_state(&mut self, state: SelectionState) {
        self.state = state;
    }

    /// Set owner window
    pub fn set_owner(&mut self, window: Window) {
        self.owner_window = Some(window);
        self.state = SelectionState::On;
    }

    /// Clear ownership
    pub fn clear_owner(&mut self) {
        self.owner_window = None;
        self.state = SelectionState::Off;
    }
}

/// Atom cache for X11 atoms
#[derive(Debug, Clone)]
pub struct AtomCache {
    pub primary: Atom,
    pub secondary: Atom,
    pub clipboard: Atom,
    pub targets: Atom,
    pub multiple: Atom,
    pub text: Atom,
    pub utf8_string: Atom,
    pub string: Atom,
    pub incremental: Atom,
    pub delete_property: Atom,
    pub timestamp: Atom,
}

impl AtomCache {
    /// Create new atom cache with placeholder values
    /// Actual values will be filled by XInternAtom calls
    pub fn new() -> Self {
        Self {
            primary: 0,
            secondary: 0,
            clipboard: 0,
            targets: 0,
            multiple: 0,
            text: 0,
            utf8_string: 0,
            string: 0,
            incremental: 0,
            delete_property: 0,
            timestamp: 0,
        }
    }

    /// Get atom for selection type
    pub fn get_selection_atom(&self, selection: Selection) -> Option<Atom> {
        match selection {
            Selection::Primary => Some(self.primary),
            Selection::Secondary => Some(self.secondary),
            Selection::Clipboard => Some(self.clipboard),
        }
    }

    /// Get atom for target type
    pub fn get_target_atom(&self, target: ClipboardTarget) -> Option<Atom> {
        match target {
            ClipboardTarget::Utf8String => Some(self.utf8_string),
            ClipboardTarget::String => Some(self.string),
            ClipboardTarget::Text => Some(self.text),
            ClipboardTarget::Multiple => Some(self.multiple),
            ClipboardTarget::Incremental => Some(self.incremental),
        }
    }
}

impl Default for AtomCache {
    fn default() -> Self {
        Self::new()
    }
}

/// X11 Clipboard manager
///
/// Manages clipboard sharing between X displays using the X Selection mechanism.
/// Supports PRIMARY, SECONDARY, and CLIPBOARD selections with UTF-8 encoding.
pub struct X11Clipboard {
    /// Connection to X display
    conn: Arc<X11Connection>,
    /// Selection data map
    selections: Mutex<HashMap<Selection, SelectionData>>,
    /// Atom cache
    atoms: Mutex<AtomCache>,
    /// Property window for data transfer
    prop_window: Window,
    /// Ping atom for synchronization
    ping_atom: Atom,
    /// Ping in progress flag
    ping_in_progress: Mutex<bool>,
    /// Last timestamp for uniqueness
    last_timestamp: Mutex<Time>,
}

impl X11Clipboard {
    /// Create a new clipboard manager
    ///
    /// # Arguments
    ///
    /// * `conn` - X11 connection
    /// * `prop_window` - Window to use for property transfers
    /// * `ping_atom` - Atom to use for ping-pong synchronization
    pub fn new(
        conn: Arc<X11Connection>,
        prop_window: Window,
        ping_atom: Atom,
    ) -> Result<Self> {
        // Initialize atoms
        let atoms = Self::init_atoms(&conn)?;

        // Initialize selection data
        let mut selections = HashMap::new();
        selections.insert(Selection::Primary, SelectionData::new(prop_window));
        selections.insert(Selection::Secondary, SelectionData::new(prop_window));
        selections.insert(Selection::Clipboard, SelectionData::new(prop_window));

        info!("X11Clipboard created for window 0x{:x}", prop_window);

        Ok(Self {
            conn,
            selections: Mutex::new(selections),
            atoms: Mutex::new(atoms),
            prop_window,
            ping_atom,
            ping_in_progress: Mutex::new(false),
            last_timestamp: Mutex::new(0),
        })
    }

    /// Initialize X11 atoms
    fn init_atoms(conn: &Arc<X11Connection>) -> Result<AtomCache> {
        let display = conn.display_ptr();
        let mut cache = AtomCache::new();

        unsafe {
            // Get atoms using XInternAtom
            let primary = Self::intern_atom(display, "PRIMARY", false)?;
            let secondary = Self::intern_atom(display, "SECONDARY", false)?;
            let clipboard = Self::intern_atom(display, "CLIPBOARD", false)?;
            let targets = Self::intern_atom(display, "TARGETS", false)?;
            let multiple = Self::intern_atom(display, "MULTIPLE", false)?;
            let text = Self::intern_atom(display, "TEXT", false)?;
            let utf8_string = Self::intern_atom(display, "UTF8_STRING", false)?;
            let string_atom = Self::intern_atom(display, "STRING", false)?;
            let incremental = Self::intern_atom(display, "INCR", false)?;
            let delete_property = Self::intern_atom(display, "DELETE", false)?;
            let timestamp = Self::intern_atom(display, "TIMESTAMP", false)?;

            cache.primary = primary;
            cache.secondary = secondary;
            cache.clipboard = clipboard;
            cache.targets = targets;
            cache.multiple = multiple;
            cache.text = text;
            cache.utf8_string = utf8_string;
            cache.string = string_atom;
            cache.incremental = incremental;
            cache.delete_property = delete_property;
            cache.timestamp = timestamp;
        }

        info!("Initialized clipboard atoms");
        Ok(cache)
    }

    /// Intern an atom with X server
    unsafe fn intern_atom(
        display: *mut Display,
        name: &str,
        only_if_exists: bool,
    ) -> Result<Atom> {
        let xlib = xlib::Xlib::open().context("Failed to load xlib")?;
        let c_name = CString::new(name).context("Failed to create CString")?;

        let atom = (xlib.XInternAtom)(
            display,
            c_name.as_ptr(),
            if only_if_exists { 1 } else { 0 },
        );

        if atom == 0 {
            Err(anyhow::anyhow!("Failed to intern atom: {}", name))
        } else {
            debug!("Interned atom: {} = 0x{:x}", name, atom);
            Ok(atom as Atom)
        }
    }

    /// Handle a SelectionRequest event
    ///
    /// Implements ProcessSelectionRequest logic from original x2x.c:
    /// 1. Check if we own the selection
    /// 2. Check if target is supported
    /// 3. Send ping to other display for data
    /// 4. Store request for later response
    pub fn handle_selection_request(
        &self,
        event: &XSelectionRequestEvent,
        is_from_display: bool,
    ) -> Result<bool> {
        debug!("Handling SelectionRequest: selection=0x{:x}, target=0x{:x}, requestor=0x{:x}",
            event.selection, event.target, event.requestor);

        let atoms = self.atoms.lock().unwrap();
        let mut selections = self.selections.lock().unwrap();

        // Determine which selection this is
        let selection = if event.selection as u32 == atoms.primary {
            Selection::Primary
        } else if event.selection as u32 == atoms.clipboard {
            Selection::Clipboard
        } else if event.selection as u32 == atoms.secondary {
            Selection::Secondary
        } else {
            debug!("Unknown selection 0x{:x}, ignoring", event.selection);
            return Ok(false);
        };

        // Check if we own the selection and state is ON
        let sel_data = selections.get(&selection)
            .ok_or_else(|| anyhow::anyhow!("Selection not found"))?;

        if sel_data.state != SelectionState::On {
            debug!("We don't own selection, state={:?}", sel_data.state);
            return Ok(false);
        }

        // Check if target is supported
        let target_supported = event.target as u32 == atoms.text
            || event.target as u32 == atoms.utf8_string
            || event.target as u32 == atoms.string;

        if !target_supported {
            warn!("Unsupported target 0x{:x}, refusing request", event.target);
            // TODO: Send SelectionNotify with None property
            return Ok(false);
        }

        // Send ping to get data from other display
        drop(atoms);
        drop(selections);
        self.send_ping()?;

        info!("Processed SelectionRequest for {:?}", selection);
        Ok(true)
    }

    /// Handle a SelectionNotify event
    ///
    /// Implements ProcessSelectionNotify logic from original x2x.c:
    /// 1. Check if this is a response to our request
    /// 2. Get property data from requestor
    /// 3. Update selection data
    /// 4. Send SelectionNotify to original requestor
    pub fn handle_selection_notify(
        &self,
        event: &XSelectionEvent,
        is_from_display: bool,
    ) -> Result<bool> {
        debug!("Handling SelectionNotify: selection=0x{:x}, property=0x{:x}, requestor=0x{:x}",
            event.selection, event.property, event.requestor);

        // Check if property is None (refusal)
        if event.property == 0u32 as XlibAtom {
            debug!("SelectionNotify with None property (refusal)");
            return Ok(false);
        }

        let atoms = self.atoms.lock().unwrap();

        // Determine which selection this is
        let selection = if event.selection as u32 == atoms.primary {
            Selection::Primary
        } else if event.selection as u32 == atoms.clipboard {
            Selection::Clipboard
        } else if event.selection as u32 == atoms.secondary {
            Selection::Secondary
        } else {
            debug!("Unknown selection 0x{:x}", event.selection);
            return Ok(false);
        };

        drop(atoms);

        // Get property data
        let data = self.get_property_data(event.requestor, event.property as u32)?;

        if let Some(data_ref) = &data {
            // Update selection data
            let mut selections = self.selections.lock().unwrap();
            let sel_data = selections.get_mut(&selection)
                .ok_or_else(|| anyhow::anyhow!("Selection not found"))?;

            // Determine format from atom
            let format = ClipboardTarget::Utf8String; // Default to UTF-8

            let clipboard_data = ClipboardData::new(data_ref.clone(), format, event.time as Time);
            sel_data.set_data(clipboard_data);
            sel_data.set_state(SelectionState::On);

            info!("Updated selection data for {:?}: {} bytes", selection, data_ref.len());
        } else {
            warn!("Failed to get property data");
        }

        Ok(true)
    }

    /// Handle a SelectionClear event
    ///
    /// Implements ProcessSelectionClear logic from original x2x.c:
    /// 1. Clear selection ownership
    /// 2. Clear selection data
    /// 3. Set state to WAIT or OFF
    pub fn handle_selection_clear(
        &self,
        event: &XSelectionClearEvent,
        is_from_display: bool,
    ) -> Result<bool> {
        info!("Handling SelectionClear: selection=0x{:x}, window=0x{:x}",
            event.selection, event.window);

        let atoms = self.atoms.lock().unwrap();

        // Determine which selection this is
        let selection = if event.selection as u32 == atoms.primary {
            Selection::Primary
        } else if event.selection as u32 == atoms.clipboard {
            Selection::Clipboard
        } else if event.selection as u32 == atoms.secondary {
            Selection::Secondary
        } else {
            debug!("Unknown selection 0x{:x}", event.selection);
            return Ok(false);
        };

        drop(atoms);

        // Clear ownership and data
        let mut selections = self.selections.lock().unwrap();
        let sel_data = selections.get_mut(&selection)
            .ok_or_else(|| anyhow::anyhow!("Selection not found"))?;

        sel_data.clear_owner();
        sel_data.clear_data();
        sel_data.set_state(SelectionState::Off);

        info!("Cleared selection {:?}", selection);
        Ok(true)
    }

    /// Send ping for clipboard synchronization
    fn send_ping(&self) -> Result<()> {
        let mut ping_in_progress = self.ping_in_progress.lock().unwrap();

        if *ping_in_progress {
            debug!("Ping already in progress, skipping");
            return Ok(());
        }

        let display = self.conn.display_ptr();
        let xlib = xlib::Xlib::open().context("Failed to load xlib")?;

        unsafe {
            // Change property as ping
            let result = (xlib.XChangeProperty)(
                display,
                self.prop_window as XlibWindow,
                self.ping_atom as XlibAtom,
                xlib::XA_PRIMARY,
                8, // 8-bit format
                PropModeAppend,
                ptr::null(),
                0, // No data
            );

            if result == 0 {
                return Err(anyhow::anyhow!("XChangeProperty failed"));
            }
        }

        *ping_in_progress = true;
        debug!("Sent ping to window 0x{:x}", self.prop_window);
        Ok(())
    }

    /// Get property data from window
    fn get_property_data(&self, window: Window, property: Atom) -> Result<Option<Vec<u8>>> {
        let display = self.conn.display_ptr();
        let xlib = xlib::Xlib::open().context("Failed to load xlib")?;

        let mut actual_type = 0u64;
        let mut actual_format = 0i32;
        let mut nitems = 0u64;
        let mut bytes_after = 0u64;
        let mut prop_data: *mut u8 = ptr::null_mut();

        unsafe {
            let result = (xlib.XGetWindowProperty)(
                display,
                window as XlibWindow,
                property as XlibAtom,
                0, // offset
                1024 * 1024, // 1MB max size
                True, // delete
                xlib::AnyPropertyType as XlibAtom,
                &mut actual_type,
                &mut actual_format,
                &mut nitems,
                &mut bytes_after,
                &mut prop_data,
            );

            if result != 0 {
                debug!("XGetWindowProperty returned error: {}", result);
                return Ok(None);
            }

            if actual_type == 0u64 || nitems == 0 {
                debug!("Property not found or empty");
                if !prop_data.is_null() {
                    (xlib.XFree)(prop_data as *mut c_void);
                }
                return Ok(None);
            }

            // Copy data to Vec
            let byte_count = (nitems * (actual_format as u64) / 8) as usize;
            let mut data = vec![0u8; byte_count];
            ptr::copy_nonoverlapping(prop_data, data.as_mut_ptr(), byte_count);

            (xlib.XFree)(prop_data as *mut c_void);

            info!("Got property data: {} bytes", byte_count);
            Ok(Some(data))
        }
    }

    /// Request selection from X server
    ///
    /// # Arguments
    ///
    /// * `selection` - Selection type to request
    /// * `target` - Target format
    /// * `property` - Property to store result
    /// * `time` - Timestamp for request
    pub fn request_selection(
        &self,
        selection: Selection,
        target: ClipboardTarget,
        property: Atom,
        time: Time,
    ) -> Result<()> {
        let display = self.conn.display_ptr();
        let xlib = xlib::Xlib::open().context("Failed to load xlib")?;

        let atoms = self.atoms.lock().unwrap();

        let selection_atom = atoms.get_selection_atom(selection)
            .ok_or_else(|| anyhow::anyhow!("Invalid selection"))?;
        let target_atom = atoms.get_target_atom(target)
            .ok_or_else(|| anyhow::anyhow!("Invalid target"))?;

        drop(atoms);

        unsafe {
            let result = (xlib.XConvertSelection)(
                display,
                selection_atom as XlibAtom,
                target_atom as XlibAtom,
                property as XlibAtom,
                self.prop_window as XlibWindow,
                time as XlibTime,
            );

            if result == 0 {
                return Err(anyhow::anyhow!("XConvertSelection failed"));
            }
        }

        info!("Requested selection {:?} (target={:?})", selection, target);
        Ok(())
    }

    /// Set selection ownership
    ///
    /// # Arguments
    ///
    /// * `selection` - Selection type
    /// * `owner` - Owner window
    /// * `time` - Timestamp
    pub fn set_selection_owner(
        &self,
        selection: Selection,
        owner: Window,
        time: Time,
    ) -> Result<()> {
        let display = self.conn.display_ptr();
        let xlib = xlib::Xlib::open().context("Failed to load xlib")?;

        let atoms = self.atoms.lock().unwrap();

        let selection_atom = atoms.get_selection_atom(selection)
            .ok_or_else(|| anyhow::anyhow!("Invalid selection"))?;

        drop(atoms);

        unsafe {
            (xlib.XSetSelectionOwner)(
                display,
                selection_atom as XlibAtom,
                owner as XlibWindow,
                time as XlibTime,
            );
        }

        // Update local state
        let mut selections = self.selections.lock().unwrap();
        if let Some(sel_data) = selections.get_mut(&selection) {
            sel_data.set_owner(owner);
        }

        info!("Set selection owner for {:?}: 0x{:x}", selection, owner);
        Ok(())
    }

    /// Get clipboard data
    pub fn get_data(&self, selection: Selection) -> Option<Vec<u8>> {
        let selections = self.selections.lock().unwrap();
        selections.get(&selection)
            .and_then(|s| s.get_data())
            .map(|d| d.data.clone())
    }

    /// Set clipboard data
    pub fn set_data(&self, selection: Selection, data: Vec<u8>, format: ClipboardTarget) -> Result<()> {
        let data_len = data.len();

        let mut selections = self.selections.lock().unwrap();
        let sel_data = selections.get_mut(&selection)
            .ok_or_else(|| anyhow::anyhow!("Selection not found"))?;

        let clipboard_data = ClipboardData::new(data, format, 0);
        sel_data.set_data(clipboard_data);

        info!("Set clipboard data for {:?}: {} bytes", selection, data_len);
        Ok(())
    }

    /// Get selection state
    pub fn get_state(&self, selection: Selection) -> SelectionState {
        let selections = self.selections.lock().unwrap();
        selections.get(&selection)
            .map(|s| s.state)
            .unwrap_or(SelectionState::Off)
    }

    /// Check if we own the selection
    pub fn is_owned(&self, selection: Selection) -> bool {
        self.get_state(selection) == SelectionState::On
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_types() {
        assert_eq!(Selection::Primary.atom_name(), "PRIMARY");
        assert_eq!(Selection::Secondary.atom_name(), "SECONDARY");
        assert_eq!(Selection::Clipboard.atom_name(), "CLIPBOARD");
    }

    #[test]
    fn test_clipboard_target_types() {
        assert_eq!(ClipboardTarget::Utf8String.atom_name(), "UTF8_STRING");
        assert_eq!(ClipboardTarget::String.atom_name(), "STRING");
        assert_eq!(ClipboardTarget::Text.atom_name(), "TEXT");
    }

    #[test]
    fn test_selection_data() {
        let prop_window = 0x12345678;
        let mut sel_data = SelectionData::new(prop_window);

        assert!(!sel_data.is_owned());
        assert!(sel_data.get_data().is_none());

        let data = ClipboardData::new(b"test".to_vec(), ClipboardTarget::Utf8String, 0);
        sel_data.set_data(data);
        assert!(sel_data.get_data().is_some());

        sel_data.set_owner(prop_window);
        assert!(sel_data.is_owned());

        sel_data.clear_owner();
        assert!(!sel_data.is_owned());
    }

    #[test]
    fn test_clipboard_data() {
        let data_vec = b"Hello, world!".to_vec();
        let mut data = ClipboardData::new(data_vec.clone(), ClipboardTarget::Utf8String, 0);

        assert_eq!(data.data, b"Hello, world!");
        assert_eq!(data.revision, 0);

        data.bump_revision();
        assert_eq!(data.revision, 1);

        let utf8_str = data.as_utf8();
        assert_eq!(utf8_str, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_selection_state() {
        let mut state = SelectionState::default();
        assert_eq!(state, SelectionState::Off);

        state = SelectionState::On;
        assert_eq!(state, SelectionState::On);

        state = SelectionState::Wait;
        assert_eq!(state, SelectionState::Wait);
    }

    #[test]
    fn test_atom_cache() {
        let cache = AtomCache::new();

        assert_eq!(cache.primary, 0);
        assert_eq!(cache.utf8_string, 0);

        assert!(cache.get_selection_atom(Selection::Primary).is_some());
        assert!(cache.get_target_atom(ClipboardTarget::Utf8String).is_some());
    }
}
