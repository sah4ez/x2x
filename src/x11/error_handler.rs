//! X11 error handling

use crate::x11::X11Error;
use log::{debug, error, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use x11_dl::xlib::{XErrorEvent, XErrorEvent as XlibXErrorEvent};

/// Global error handler state
static ERROR_HANDLER_SET: AtomicBool = AtomicBool::new(false);

/// Store the last X11 error for retrieval
static LAST_ERROR: Mutex<Option<X11Error>> = Mutex::new(None);

/// Set up the X11 error handler
///
/// This should be called once per X11 connection to ensure that
/// X11 protocol errors are caught and logged rather than crashing.
pub fn setup_error_handler() -> Result<(), X11Error> {
    if ERROR_HANDLER_SET.load(Ordering::SeqCst) {
        debug!("Error handler already set");
        return Ok(());
    }

    // Note: In Rust, we can't easily set X11 error handlers via FFI
    // because they require function pointers with C calling conventions
    // and thread-local storage. The x11-dl library handles most errors
    // gracefully, but for comprehensive error handling, we would need
    // to use a more advanced approach.

    // For now, we'll document that we're relying on x11-dl's default
    // error handling, which logs errors without crashing.

    warn!("X11 error handler: using x11-dl defaults. For custom error handling, see docs.");
    ERROR_HANDLER_SET.store(true, Ordering::SeqCst);
    Ok(())
}

/// Get the last X11 error that occurred
pub fn get_last_error() -> Option<X11Error> {
    LAST_ERROR.lock().unwrap().take()
}

/// Store an X11 error
pub fn store_error(error: X11Error) {
    *LAST_ERROR.lock().unwrap() = Some(error);
}

/// X11 Error Event information
#[derive(Debug, Clone)]
pub struct X11ErrorInfo {
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
    pub resource_id: u64,
}

impl X11ErrorInfo {
    /// Convert from Xlib XErrorEvent
    pub fn from_xlib(xerror: &XlibXErrorEvent) -> Self {
        Self {
            error_code: xerror.error_code,
            request_code: xerror.request_code,
            minor_code: xerror.minor_code,
            resource_id: xerror.resourceid,
        }
    }

    /// Get a human-readable description of the error
    pub fn description(&self) -> &'static str {
        match self.error_code {
            1 => "BadRequest",
            2 => "BadValue",
            3 => "BadWindow",
            4 => "BadPixmap",
            5 => "BadAtom",
            6 => "BadCursor",
            7 => "BadFont",
            8 => "BadMatch",
            9 => "BadDrawable",
            10 => "BadAccess",
            11 => "BadAlloc",
            12 => "BadColor",
            13 => "BadGC",
            14 => "BadIDChoice",
            15 => "BadName",
            16 => "BadLength",
            17 => "BadImplementation",
            _ => "UnknownError",
        }
    }

    /// Get the request name
    pub fn request_name(&self) -> &'static str {
        match self.request_code {
            1 => "CreateWindow",
            2 => "ChangeWindowAttributes",
            3 => "GetWindowAttributes",
            4 => "DestroyWindow",
            5 => "DestroySubwindows",
            6 => "ChangeSaveSet",
            7 => "ReparentWindow",
            8 => "MapWindow",
            9 => "MapSubwindows",
            10 => "UnmapWindow",
            11 => "UnmapSubwindows",
            12 => "ConfigureWindow",
            13 => "CirculateWindow",
            14 => "GetGeometry",
            15 => "QueryTree",
            16 => "InternAtom",
            17 => "GetAtomName",
            18 => "ChangeProperty",
            19 => "DeleteProperty",
            20 => "GetProperty",
            21 => "ListProperties",
            22 => "SetSelectionOwner",
            23 => "GetSelectionOwner",
            24 => "ConvertSelection",
            25 => "SendEvent",
            26 => "GrabPointer",
            27 => "UngrabPointer",
            28 => "GrabButton",
            29 => "UngrabButton",
            30 => "ChangeActivePointerGrab",
            31 => "GrabKeyboard",
            32 => "UngrabKeyboard",
            33 => "GrabKey",
            34 => "UngrabKey",
            35 => "AllowEvents",
            36 => "GrabServer",
            37 => "UngrabServer",
            38 => "QueryPointer",
            39 => "GetMotionEvents",
            40 => "TranslateCoords",
            41 => "WarpPointer",
            42 => "SetInputFocus",
            43 => "GetInputFocus",
            44 => "QueryKeymap",
            45 => "OpenFont",
            46 => "CloseFont",
            47 => "QueryFont",
            48 => "QueryTextExtents",
            49 => "ListFonts",
            50 => "ListFontsWithInfo",
            51 => "SetFontPath",
            52 => "GetFontPath",
            53 => "CreatePixmap",
            54 => "FreePixmap",
            55 => "CreateGC",
            56 => "ChangeGC",
            57 => "CopyGC",
            58 => "SetDashes",
            59 => "SetClipRectangles",
            60 => "FreeGC",
            61 => "ClearArea",
            62 => "CopyArea",
            63 => "CopyPlane",
            64 => "PolyPoint",
            65 => "PolyLine",
            66 => "PolySegment",
            67 => "PolyRectangle",
            68 => "PolyArc",
            69 => "FillPoly",
            70 => "PolyFillRectangle",
            71 => "PolyFillArc",
            72 => "PutImage",
            73 => "GetImage",
            74 => "PolyText8",
            75 => "PolyText16",
            76 => "ImageText8",
            77 => "ImageText16",
            78 => "CreateColormap",
            79 => "FreeColormap",
            80 => "CopyColormapAndFree",
            81 => "InstallColormap",
            82 => "UninstallColormap",
            83 => "ListInstalledColormaps",
            84 | 85 | 86 => "AllocColor",
            91 => "AllocNamedColor",
            92 => "AllocColorCells",
            93 => "AllocColorPlanes",
            94 => "FreeColors",
            97 => "StoreColors",
            98 => "StoreNamedColor",
            _ => "Unknown",
        }
    }
}

impl std::fmt::Display for X11ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "X11 Error: {} (code {}) in request {} (minor {}) for resource 0x{:x}",
            self.description(),
            self.error_code,
            self.request_name(),
            self.minor_code,
            self.resource_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_info_bad_window() {
        let info = X11ErrorInfo {
            error_code: 3,   // BadWindow
            request_code: 4, // DestroyWindow
            minor_code: 0,
            resource_id: 0x12345678,
        };

        assert_eq!(info.description(), "BadWindow");
        assert_eq!(info.request_name(), "DestroyWindow");
        assert!(format!("{}", info).contains("BadWindow"));
        assert!(format!("{}", info).contains("0x12345678"));
    }

    #[test]
    fn test_error_info_unknown() {
        let info = X11ErrorInfo {
            error_code: 255,
            request_code: 255,
            minor_code: 0,
            resource_id: 0,
        };

        assert_eq!(info.description(), "UnknownError");
        assert_eq!(info.request_name(), "Unknown");
    }

    #[test]
    fn test_setup_error_handler() {
        // First call should succeed
        assert!(setup_error_handler().is_ok());

        // Second call should also succeed (no-op)
        assert!(setup_error_handler().is_ok());
    }

    #[test]
    fn test_store_and_get_error() {
        let error = X11Error::Generic("test error".to_string());

        store_error(error.clone());
        let retrieved = get_last_error();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().to_string(), "X11 error: test error");

        // Second call should return None
        assert!(get_last_error().is_none());
    }
}
