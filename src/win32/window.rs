//! Windows window management for -fromwin support

#[cfg(feature = "win32")]
use windows::Win32::UI::WindowsAndMessaging::*;

#[cfg(feature = "win32")]
/// Windows window for x2x
///
/// This handles the edge window and status window used when
/// running x2x from a Windows machine.
pub struct Win32Window {
    hwnd_edge: Option<HWND>,
    hwnd_big: Option<HWND>,
    monitor_rect: Option<windows::Win32::Foundation::RECT>,
}

#[cfg(feature = "win32")]
impl Win32Window {
    /// Create a new Win32Window
    pub fn new() -> crate::win32::Result<Self> {
        Ok(Self {
            hwnd_edge: None,
            hwnd_big: None,
            monitor_rect: None,
        })
    }

    /// Create the edge window
    pub fn create_edge_window(&mut self) -> crate::win32::Result<()> {
        // TODO: Implement edge window creation
        // This is a thin window at the edge of the screen
        // that triggers connection when the mouse crosses it
        todo!("Implement Win32Window::create_edge_window")
    }

    /// Create the big status window
    pub fn create_big_window(&mut self) -> crate::win32::Result<()> {
        // TODO: Implement big window creation
        // This window displays connection status
        todo!("Implement Win32Window::create_big_window")
    }

    /// Process Windows messages
    pub fn process_messages(&mut self) -> crate::win32::Result<bool> {
        // TODO: Implement message loop
        // Use GetMessage/DispatchMessage
        todo!("Implement Win32Window::process_messages")
    }

    /// Move edge window to specified position
    pub fn move_edge_window(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> crate::win32::Result<()> {
        // TODO: Implement SetWindowPos
        todo!("Implement Win32Window::move_edge_window")
    }

    /// Show or hide the big window
    pub fn show_big_window(&self, show: bool) -> crate::win32::Result<()> {
        // TODO: Implement ShowWindow
        todo!("Implement Win32Window::show_big_window")
    }
}

#[cfg(feature = "win32")]
impl Default for Win32Window {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
