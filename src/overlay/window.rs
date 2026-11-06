//! Win32 styles of the overlay window: the desktop app's `overlay.rs` window calls.
//!
//! gpui opens the overlay as a borderless pop-up (`WS_EX_TOOLWINDOW`, no taskbar button) with a
//! transparent background. This module adds what Tauri's `set_ignore_cursor_events`,
//! `set_focusable(false)` and `always_on_top` did:
//!
//! - `WS_EX_TOPMOST`: above the game.
//! - `WS_EX_NOACTIVATE`: clicks never take focus from the game. Activating a window can minimize
//!   an exclusive fullscreen game.
//! - `WS_EX_LAYERED | WS_EX_TRANSPARENT` while closed: the mouse goes through to the game.
#![cfg(windows)]

use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::{
    Foundation::{COLORREF, HWND},
    UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GetForegroundWindow, GetWindowLongPtrW, HWND_TOPMOST, IsWindow, LWA_ALPHA,
        SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetForegroundWindow, SetLayeredWindowAttributes,
        SetWindowLongPtrW, SetWindowPos, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOPMOST,
        WS_EX_TRANSPARENT,
    },
};

/// The window's `HWND`, stored as an integer so it can be kept in `Send` state.
pub fn hwnd(window: &Window) -> Option<isize> {
    match HasWindowHandle::window_handle(window).ok()?.as_raw() {
        RawWindowHandle::Win32(handle) => Some(handle.hwnd.get()),
        _ => None,
    }
}

fn handle(hwnd: isize) -> HWND {
    HWND(hwnd as *mut _)
}

/// Topmost, never activated, and click-through until opened.
pub fn prepare(hwnd: isize) {
    let hwnd = handle(hwnd);

    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let extra = (WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_LAYERED | WS_EX_TRANSPARENT).0;

        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | extra as isize);
        // A layered window draws nothing until its attributes are set; keep it fully opaque
        // (its background is already transparent).
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
    }
}

/// Opens (takes the mouse) or closes (click-through) the overlay. Closing gives the
/// foreground back to the window that had it when the overlay opened.
pub fn set_interactive(hwnd: isize, interactive: bool, previous: &mut Option<isize>) {
    let hwnd = handle(hwnd);

    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let transparent = WS_EX_TRANSPARENT.0 as isize;

        match interactive {
            true => {
                *previous = Some(GetForegroundWindow().0 as isize);
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style & !transparent);
            }
            false => {
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style | transparent);

                if let Some(previous) = previous.take().map(handle)
                    && GetForegroundWindow() == hwnd
                    && IsWindow(Some(previous)).as_bool()
                {
                    let _ = SetForegroundWindow(previous);
                }
            }
        }
    }
}
