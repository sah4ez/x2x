//! Fake input event queue

use crate::x11::X11Connection;
use anyhow::Result;

/// Re-export FakeEvent from core module
pub use crate::core::FakeEvent;

/// Fake input manager
///
/// Manages the queue of fake input events and processes them
/// on the target display.
pub struct FakeManager {
    conn: std::sync::Arc<X11Connection>,
    queue: crate::core::FakeQueue,
}

impl FakeManager {
    /// Create a new fake manager
    pub fn new(conn: std::sync::Arc<X11Connection>) -> Self {
        Self {
            conn,
            queue: crate::core::FakeQueue::new(),
        }
    }

    /// Queue a fake key event
    pub fn queue_key(&mut self, keysym: u32, keycode: u8, is_press: bool) {
        self.queue.push(FakeEvent::Key {
            keysym,
            keycode,
            is_press,
        });
    }

    /// Queue a fake button event
    pub fn queue_button(&mut self, button: u32, is_press: bool) {
        self.queue.push(FakeEvent::Button { button, is_press });
    }

    /// Process all queued fake events
    pub fn process_queue(&mut self) -> Result<()> {
        self.queue.process_all(|event| self.process_single_event(event))
    }

    /// Process a single fake event
    fn process_single_event(&self, event: &FakeEvent) -> Result<()> {
        use crate::x11::extension::XTestExtension;

        match event {
            FakeEvent::Key { keycode, is_press, .. } => {
                XTestExtension::fake_key(&self.conn, *keycode, *is_press)?;
            }
            FakeEvent::Button { button, is_press } => {
                XTestExtension::fake_button(&self.conn, *button, *is_press)?;
            }
        }

        Ok(())
    }

    /// Check if there are pending events
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Get the number of pending events
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    /// Clear all pending events
    pub fn clear(&mut self) {
        self.queue.clear_active();
        while self.queue.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_manager() {
        // This is a stub test - real tests would require a mock X11 connection
        // For now, just verify the API compiles
        assert!(true);
    }
}
