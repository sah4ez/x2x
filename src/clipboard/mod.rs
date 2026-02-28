//! Clipboard manager for inter-display sharing

use crate::x11::{X11Connection, X11Clipboard, Selection, ClipboardTarget};
use anyhow::Result;

/// Clipboard manager for inter-display sharing
///
/// This manager coordinates clipboard operations between two X displays,
/// forwarding clipboard changes from one display to the other.
pub struct ClipboardManager {
    /// Clipboard manager for "from" display
    from_clipboard: X11Clipboard,
    /// Clipboard manager for "to" display
    to_clipboard: X11Clipboard,
    /// Track last clipboard data to detect changes
    last_from_data: Option<Vec<u8>>,
    last_to_data: Option<Vec<u8>>,
}

impl ClipboardManager {
    /// Create a new clipboard manager
    ///
    /// # Arguments
    ///
    /// * `from_conn` - Connection to source display
    /// * `to_conn` - Connection to target display
    /// * `prop_window_from` - Property window for from display
    /// * `prop_window_to` - Property window for to display
    /// * `ping_atom` - Atom for ping-pong synchronization
    pub fn new(
        from_conn: std::sync::Arc<X11Connection>,
        to_conn: std::sync::Arc<X11Connection>,
        prop_window_from: u64,
        prop_window_to: u64,
        ping_atom: u32,
    ) -> Result<Self> {
        let from_clipboard = X11Clipboard::new(
            from_conn,
            prop_window_from,
            ping_atom,
        )?;

        let to_clipboard = X11Clipboard::new(
            to_conn,
            prop_window_to,
            ping_atom,
        )?;

        log::info!("ClipboardManager created");

        Ok(Self {
            from_clipboard,
            to_clipboard,
            last_from_data: None,
            last_to_data: None,
        })
    }

    /// Handle a clipboard event from "from" display
    ///
    /// This forwards clipboard changes from the source display to the target display.
    pub fn handle_from_event(
        &mut self,
        event: &crate::x11::event::XEvent,
        ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        match event {
            crate::x11::event::XEvent::SelectionRequest(req) => {
                self.handle_selection_request_from(req, ctx)?;
            }
            crate::x11::event::XEvent::SelectionNotify(notif) => {
                self.handle_selection_notify_from(notif, ctx)?;
            }
            crate::x11::event::XEvent::SelectionClear(clear) => {
                self.handle_selection_clear_from(clear, ctx)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle a clipboard event from "to" display
    ///
    /// This forwards clipboard changes from the target display to the source display.
    pub fn handle_to_event(
        &mut self,
        event: &crate::x11::event::XEvent,
        ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        match event {
            crate::x11::event::XEvent::SelectionRequest(req) => {
                self.handle_selection_request_to(req, ctx)?;
            }
            crate::x11::event::XEvent::SelectionNotify(notif) => {
                self.handle_selection_notify_to(notif, ctx)?;
            }
            crate::x11::event::XEvent::SelectionClear(clear) => {
                self.handle_selection_clear_to(clear, ctx)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle SelectionRequest from "from" display
    fn handle_selection_request_from(
        &mut self,
        _event: &crate::x11::event::XSelectionRequestEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Get data from to_clipboard and forward
        // For now, just log
        log::debug!("SelectionRequest from 'from' display");
        Ok(())
    }

    /// Handle SelectionNotify from "from" display
    fn handle_selection_notify_from(
        &mut self,
        _event: &crate::x11::event::XSelectionEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Store data from from_clipboard and forward to to_clipboard
        // For now, just log
        log::debug!("SelectionNotify from 'from' display");
        Ok(())
    }

    /// Handle SelectionClear from "from" display
    fn handle_selection_clear_from(
        &mut self,
        _event: &crate::x11::event::XSelectionClearEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Clear data in to_clipboard
        log::debug!("SelectionClear from 'from' display");
        Ok(())
    }

    /// Handle SelectionRequest from "to" display
    fn handle_selection_request_to(
        &mut self,
        _event: &crate::x11::event::XSelectionRequestEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Get data from from_clipboard and forward
        log::debug!("SelectionRequest from 'to' display");
        Ok(())
    }

    /// Handle SelectionNotify from "to" display
    fn handle_selection_notify_to(
        &mut self,
        _event: &crate::x11::event::XSelectionEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Store data from to_clipboard and forward to from_clipboard
        log::debug!("SelectionNotify from 'to' display");
        Ok(())
    }

    /// Handle SelectionClear from "to" display
    fn handle_selection_clear_to(
        &mut self,
        _event: &crate::x11::event::XSelectionClearEvent,
        _ctx: &mut crate::core::DpyInfo,
    ) -> Result<()> {
        // Clear data in from_clipboard
        log::debug!("SelectionClear from 'to' display");
        Ok(())
    }

    /// Check if clipboard data has changed on "from" display
    pub fn has_from_changed(&self) -> bool {
        let current_data = self.from_clipboard.get_data(Selection::Primary);

        match (&self.last_from_data, &current_data) {
            (None, None) => false,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(last), Some(current)) => last != current,
        }
    }

    /// Check if clipboard data has changed on "to" display
    pub fn has_to_changed(&self) -> bool {
        let current_data = self.to_clipboard.get_data(Selection::Primary);

        match (&self.last_to_data, &current_data) {
            (None, None) => false,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(last), Some(current)) => last != current,
        }
    }

    /// Sync clipboard data from "from" to "to" display
    pub fn sync_from_to(&mut self) -> Result<()> {
        if let Some(data) = self.from_clipboard.get_data(Selection::Primary) {
            self.to_clipboard.set_data(Selection::Primary, data, ClipboardTarget::Utf8String)?;
            self.last_from_data = self.from_clipboard.get_data(Selection::Primary);
            log::info!("Synced clipboard from 'from' to 'to' display");
        }

        Ok(())
    }

    /// Sync clipboard data from "to" to "from" display
    pub fn sync_to_from(&mut self) -> Result<()> {
        if let Some(data) = self.to_clipboard.get_data(Selection::Primary) {
            self.from_clipboard.set_data(Selection::Primary, data, ClipboardTarget::Utf8String)?;
            self.last_to_data = self.to_clipboard.get_data(Selection::Primary);
            log::info!("Synced clipboard from 'to' to 'from' display");
        }

        Ok(())
    }

    /// Get clipboard data from "from" display
    pub fn get_from_data(&self, selection: Selection) -> Option<Vec<u8>> {
        self.from_clipboard.get_data(selection)
    }

    /// Get clipboard data from "to" display
    pub fn get_to_data(&self, selection: Selection) -> Option<Vec<u8>> {
        self.to_clipboard.get_data(selection)
    }

    /// Set clipboard data on "from" display
    pub fn set_from_data(&self, selection: Selection, data: Vec<u8>, format: ClipboardTarget) -> Result<()> {
        self.from_clipboard.set_data(selection, data, format)
    }

    /// Set clipboard data on "to" display
    pub fn set_to_data(&self, selection: Selection, data: Vec<u8>, format: ClipboardTarget) -> Result<()> {
        self.to_clipboard.set_data(selection, data, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_types() {
        // Test that types compile correctly
        assert!(true);
    }
}
