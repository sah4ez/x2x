//! X11 extensions (XTest, DPMS, etc.)

use crate::x11::{X11Connection, X11Error};
use anyhow::{Context, Result};
use std::os::raw::{c_int, c_uint};
use std::ptr;

use x11_dl::xlib;
use x11_dl::xtst;
use x11_dl::xext;

/// XTest extension for fake input events
pub struct XTestExtension {
    xtst: xtst::Xtst,
    major_opcode: u8,
    first_event: u8,
    first_error: u8,
}

impl XTestExtension {
    /// Create XTest extension handler from a connection
    pub fn from_connection(conn: &X11Connection) -> Option<Self> {
        let xtst = conn.xtst()?.clone();

        let mut event_base: c_int = 0;
        let mut error_base: c_int = 0;
        let mut major: c_int = 0;
        let mut minor: c_int = 0;

        let result = unsafe {
            (xtst.XTestQueryExtension)(
                conn.display_ptr(),
                &mut event_base,
                &mut error_base,
                &mut major,
                &mut minor,
            )
        };

        if result == 0 {
            warn!("XTest extension not available");
            return None;
        }

        info!(
            "XTest extension initialized: op={}, ev={}, err={}, v={}.{}",
            event_base, event_base, error_base, major, minor
        );

        Some(Self {
            xtst,
            major_opcode: event_base as u8,
            first_event: event_base as u8,
            first_error: error_base as u8,
        })
    }

    /// Check if XTest extension is available
    pub fn check_available(conn: &X11Connection) -> bool {
        Self::from_connection(conn).is_some()
    }

    /// Fake a mouse motion event
    pub fn fake_motion(
        &self,
        conn: &X11Connection,
        screen: i32,
        x: i32,
        y: i32,
    ) -> Result<()> {
        unsafe {
            (self.xtst.XTestFakeMotionEvent)(
                conn.display_ptr(),
                screen as c_int,
                x as c_int,
                y as c_int,
                0, // CurrentTime
            );
            conn.flush()?;
        }
        Ok(())
    }

    /// Fake a button press/release event
    pub fn fake_button(
        &self,
        conn: &X11Connection,
        button: u32,
        is_press: bool,
    ) -> Result<()> {
        unsafe {
            (self.xtst.XTestFakeButtonEvent)(
                conn.display_ptr(),
                button as c_uint,
                if is_press { xlib::True } else { xlib::False },
                0, // CurrentTime
            );
            conn.flush()?;
        }
        Ok(())
    }

    /// Fake a key press/release event
    pub fn fake_key(
        &self,
        conn: &X11Connection,
        keycode: u8,
        is_press: bool,
    ) -> Result<()> {
        unsafe {
            (self.xtst.XTestFakeKeyEvent)(
                conn.display_ptr(),
                keycode as u32,
                if is_press { xlib::True } else { xlib::False },
                0, // CurrentTime
            );
            conn.flush()?;
        }
        Ok(())
    }

    /// Grab control of the input devices
    pub fn grab_control(
        &self,
        conn: &X11Connection,
        screen: i32,
    ) -> Result<()> {
        unsafe {
            (self.xtst.XTestGrabControl)(
                conn.display_ptr(),
                screen as c_int,
                xlib::True,
            );
        }
        Ok(())
    }

    /// Release control of the input devices
    pub fn release_control(
        &self,
        conn: &X11Connection,
        screen: i32,
    ) -> Result<()> {
        unsafe {
            (self.xtst.XTestGrabControl)(
                conn.display_ptr(),
                screen as c_int,
                xlib::False,
            );
        }
        Ok(())
    }
}

/// DPMS (Display Power Management Signaling) extension
pub struct DpmsExtension {
    xext: xext::Xext,
    major_opcode: u8,
    first_event: u8,
    first_error: u8,
}

impl DpmsExtension {
    /// Create DPMS extension handler from a connection
    pub fn from_connection(conn: &X11Connection) -> Option<Self> {
        let xext = conn.xext()?.clone();

        let mut event_base: c_int = 0;
        let mut error_base: c_int = 0;
        let mut major: c_int = 0;
        let mut minor: c_int = 0;

        let result = unsafe {
            (xext.DPMSQueryExtension)(
                conn.display_ptr(),
                &mut event_base,
                &mut error_base,
            )
        };

        if result == 0 {
            info!("DPMS extension not available");
            return None;
        }

        // Get version info
        let _ = unsafe {
            (xext.DPMSGetVersion)(
                conn.display_ptr(),
                &mut major,
                &mut minor,
            )
        };

        info!(
            "DPMS extension initialized: op={}, ev={}, err={}, v={}.{}",
            event_base, event_base, error_base, major, minor
        );

        Some(Self {
            xext,
            major_opcode: event_base as u8,
            first_event: event_base as u8,
            first_error: error_base as u8,
        })
    }

    /// Check if DPMS is available
    pub fn check_available(conn: &X11Connection) -> bool {
        Self::from_connection(conn).is_some()
    }

    /// Check if DPMS is enabled
    pub fn is_enabled(&self, conn: &X11Connection) -> Result<bool> {
        let mut state: u16 = 0;
        unsafe {
            (self.xext.DPMSInfo)(
                conn.display_ptr(),
                ptr::null_mut(),
                &mut state,
            );
        }
        Ok(state != 0)
    }

    /// Enable or disable DPMS
    pub fn enable(
        &self,
        conn: &X11Connection,
        enable: bool,
    ) -> Result<()> {
        unsafe {
            if enable {
                (self.xext.DPMSEnable)(conn.display_ptr());
            } else {
                (self.xext.DPMSDisable)(conn.display_ptr());
            }
        }
        Ok(())
    }

    /// Force DPMS to a specific level
    pub fn force_level(
        &self,
        conn: &X11Connection,
        level: u16,
    ) -> Result<()> {
        unsafe {
            (self.xext.DPMSForceLevel)(conn.display_ptr(), level);
        }
        Ok(())
    }

    /// Get current DPMS level
    pub fn get_level(&self, conn: &X11Connection) -> Result<u16> {
        let mut level: u16 = 0;
        let mut state: u16 = 0;
        unsafe {
            (self.xext.DPMSInfo)(
                conn.display_ptr(),
                &mut level,
                &mut state,
            );
        }
        Ok(level)
    }
}

/// DPMS power levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpmsLevel {
    Normal = 0,
    Standby = 1,
    Suspend = 2,
    Off = 3,
}

impl DpmsLevel {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::Standby),
            2 => Some(Self::Suspend),
            3 => Some(Self::Off),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires X server with XTest
    fn test_xtest_extension() {
        // This would test XTestExtension functionality
        // with a real X server
    }
}
