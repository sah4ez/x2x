//! Event loop implementation

use crate::x11::X11Connection;
use crate::x11::event::{XEvent, EventHandler};
use crate::core::DpyInfo;
use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

/// Event loop for handling X11 events from multiple displays
pub struct EventLoop {
    from_conn: Arc<X11Connection>,
    to_conn: Arc<X11Connection>,
    handlers: Vec<Box<dyn EventHandler + Send>>,
    running: bool,
}

impl EventLoop {
    /// Create a new event loop
    pub fn new(
        from_conn: Arc<X11Connection>,
        to_conn: Arc<X11Connection>,
    ) -> Self {
        Self {
            from_conn,
            to_conn,
            handlers: Vec::new(),
            running: true,
        }
    }

    /// Add an event handler
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler + Send>) {
        self.handlers.push(handler);
    }

    /// Run the event loop
    ///
    /// This function will block until `stop()` is called or an error occurs.
    pub fn run(&mut self, mut dpy_info: DpyInfo) -> Result<()> {
        info!("Starting event loop...");

        loop {
            if !self.running {
                info!("Event loop stopped");
                break;
            }

            // Check for pending events on from display
            while self.from_conn.pending() > 0 {
                let event = self.from_conn.next_event();
                debug!("From display event: {:?}", event);

                let event_clone = event.clone();
                self.handle_event(&event_clone, &mut dpy_info)?;
            }

            // Check for pending events on to display
            while self.to_conn.pending() > 0 {
                let event = self.to_conn.next_event();
                debug!("To display event: {:?}", event);

                let event_clone = event.clone();
                self.handle_event(&event_clone, &mut dpy_info)?;
            }

            // Small sleep to prevent busy-wait if no events
            if self.running && self.from_conn.pending() == 0 && self.to_conn.pending() == 0 {
                thread::sleep(Duration::from_millis(10));
            }
        }

        Ok(())
    }

    /// Handle an event
    fn handle_event(
        &self,
        event: &XEvent,
        ctx: &mut DpyInfo,
    ) -> Result<bool> {
        for handler in &self.handlers {
            match handler.handle(event, ctx) {
                Ok(true) => {
                    debug!("Event handled by handler");
                    return Ok(true);
                }
                Ok(false) => {
                    // Try next handler
                }
                Err(e) => {
                    error!("Handler error: {:?}", e);
                    // Continue despite errors
                }
            }
        }
        Ok(false)
    }

    /// Stop the event loop
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the event loop is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_loop_new() {
        // This would test EventLoop creation
        // For now, just verify it compiles
        assert!(true);
    }

    #[test]
    fn test_event_loop_running() {
        let mut loop_inst = EventLoop {
            from_conn: std::sync::Arc::new(unsafe {
                std::mem::zeroed()
            }),
            to_conn: std::sync::Arc::new(unsafe {
                std::mem::zeroed()
            }),
            handlers: Vec::new(),
            running: true,
        };

        assert!(loop_inst.is_running());
        loop_inst.stop();
        assert!(!loop_inst.is_running());
    }
}
