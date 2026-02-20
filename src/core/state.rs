//! Input state management (fake events, sticky keys)

use std::collections::{HashSet, VecDeque};

/// Fake event for simulating input
#[derive(Debug, Clone)]
pub enum FakeEvent {
    Key {
        keysym: u32,
        keycode: u8,
        is_press: bool,
    },
    Button {
        button: u32,
        is_press: bool,
    },
}

/// Queue for fake input events
pub struct FakeQueue {
    events: VecDeque<FakeEvent>,
    active_keys: HashSet<u32>,
    active_buttons: HashSet<u32>,
}

impl FakeQueue {
    /// Create a new fake queue
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            active_keys: HashSet::new(),
            active_buttons: HashSet::new(),
        }
    }

    /// Push a fake event to the queue
    pub fn push(&mut self, event: FakeEvent) {
        self.events.push_back(event);
    }

    /// Pop the next fake event from the queue
    pub fn pop(&mut self) -> Option<FakeEvent> {
        self.events.pop_front()
    }

    /// Check if there are pending fake events
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get the number of pending events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Process all fake events
    pub fn process_all<F>(&mut self, mut handler: F) -> anyhow::Result<()>
    where
        F: FnMut(&FakeEvent) -> anyhow::Result<()>,
    {
        while let Some(event) = self.pop() {
            handler(&event)?;

            // Update active state
            match &event {
                FakeEvent::Key { keysym, is_press, .. } => {
                    if *is_press {
                        self.active_keys.insert(*keysym);
                    } else {
                        self.active_keys.remove(keysym);
                    }
                }
                FakeEvent::Button { button, is_press } => {
                    if *is_press {
                        self.active_buttons.insert(*button);
                    } else {
                        self.active_buttons.remove(button);
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if a key is currently active (pressed)
    pub fn is_key_active(&self, keysym: u32) -> bool {
        self.active_keys.contains(&keysym)
    }

    /// Check if a button is currently active (pressed)
    pub fn is_button_active(&self, button: u32) -> bool {
        self.active_buttons.contains(&button)
    }

    /// Clear all active keys and buttons
    pub fn clear_active(&mut self) {
        self.active_keys.clear();
        self.active_buttons.clear();
    }
}

impl Default for FakeQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Sticky keys manager
///
/// Sticky keys allow modifier keys to "stick" after being pressed once,
/// making it easier to use keyboard shortcuts.
pub struct StickyKeys {
    active_keys: HashSet<u32>,
}

impl StickyKeys {
    /// Create a new sticky keys manager
    pub fn new() -> Self {
        Self {
            active_keys: HashSet::new(),
        }
    }

    /// Toggle a sticky key
    pub fn toggle(&mut self, keysym: u32) -> bool {
        if self.active_keys.contains(&keysym) {
            self.active_keys.remove(&keysym);
            false
        } else {
            self.active_keys.insert(keysym);
            true
        }
    }

    /// Check if a key is sticky
    pub fn is_sticky(&self, keysym: u32) -> bool {
        self.active_keys.contains(&keysym)
    }

    /// Clear all sticky keys
    pub fn clear(&mut self) {
        self.active_keys.clear();
    }

    /// Get the number of active sticky keys
    pub fn len(&self) -> usize {
        self.active_keys.len()
    }
}

impl Default for StickyKeys {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_queue() {
        let mut queue = FakeQueue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        queue.push(FakeEvent::Key {
            keysym: 0x61, // 'a'
            keycode: 10,
            is_press: true,
        });

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);

        let event = queue.pop();
        assert!(event.is_some());
        assert!(queue.is_empty());
    }

    #[test]
    fn test_sticky_keys() {
        let mut sticky = StickyKeys::new();

        assert!(!sticky.is_sticky(0xffe1)); // XK_Shift_L

        let is_active = sticky.toggle(0xffe1);
        assert!(is_active);
        assert!(sticky.is_sticky(0xffe1));

        let is_active = sticky.toggle(0xffe1);
        assert!(!is_active);
        assert!(!sticky.is_sticky(0xffe1));
    }
}
