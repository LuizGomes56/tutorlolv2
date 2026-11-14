//! The in-game overlay: the desktop app's `overlay.rs`, `wkb.rs`, `shortcut*.rs` and the web
//! app's `overlay/`.
//!
//! A transparent, borderless, topmost window covers the primary display. Closed, it lets the
//! mouse through and shows only the widgets kept visible. The toggle shortcut (Ctrl+K by
//! default, saved in `overlay-shortcut.json`) opens it to arrange widgets; Escape or the
//! shortcut closes it and gives the foreground back to the game. Windows only.
mod shortcut;
#[cfg(test)]
mod shortcut_tests;
mod view;

#[cfg(windows)]
mod keyboard;
#[cfg(windows)]
mod window;

pub use shortcut::Shortcut;

use gpui::{App, AppContext, Entity, Global, WindowHandle};
use std::path::PathBuf;
use view::OverlayView;

pub struct OverlayState {
    view: Option<WindowHandle<OverlayView>>,
    hwnd: Option<isize>,
    open: bool,
    /// The foreground window before the overlay opened.
    previous: Option<isize>,
    pub shortcut: Shortcut,
    /// Why the saved shortcut was not used.
    pub warning: Option<String>,
    /// Why the overlay or its shortcuts are unavailable.
    pub error: Option<String>,
}

struct GlobalOverlay(Entity<OverlayState>);

impl Global for GlobalOverlay {}

fn shortcut_path() -> PathBuf {
    crate::backend::data_dir().join("overlay-shortcut.json")
}

pub fn state(cx: &App) -> Option<Entity<OverlayState>> {
    cx.try_global::<GlobalOverlay>()
        .map(|global| global.0.clone())
}

/// Loads the shortcut, installs the keyboard hook and opens the (closed) overlay window.
pub fn init(cx: &mut App) {
    let (shortcut, warning) = match Shortcut::load(&shortcut_path()) {
        Ok(shortcut) => (shortcut, None),
        Err(error) => (
            Shortcut::default(),
            Some(format!(
                "{error}. Ctrl+K is active; save a shortcut to replace these settings."
            )),
        ),
    };

    let entity = cx.new(|_| OverlayState {
        view: None,
        hwnd: None,
        open: false,
        previous: None,
        shortcut,
        warning,
        error: None,
    });

    cx.set_global(GlobalOverlay(entity.clone()));

    #[cfg(windows)]
    {
        let binding = entity.read(cx).shortcut.validate().unwrap_or(0x24b);
        keyboard::set_binding(binding);

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

        match keyboard::install(sender) {
            Ok(()) => {
                cx.spawn(async move |cx| {
                    while let Some(action) = receiver.recv().await {
                        let _ = cx.update(|cx| match action {
                            shortcut::Action::Toggle => toggle(cx),
                            shortcut::Action::Close => set_open(false, cx),
                            shortcut::Action::Change => {
                                if let Some(view) = state(cx).and_then(|s| s.read(cx).view) {
                                    let _ = view.update(cx, |view, _, cx| view.next_enemy(cx));
                                }
                            }
                        });
                    }
                })
                .detach();
            }
            Err(error) => entity.update(cx, |state, _| state.error = Some(error)),
        }

        open_window(&entity, cx);

        // Debug builds: `TUTORLOLV2_OVERLAY_OPEN=1` opens the overlay at startup.
        #[cfg(debug_assertions)]
        if std::env::var_os("TUTORLOLV2_OVERLAY_OPEN").is_some_and(|value| value == "1") {
            set_open(true, cx);
        }
    }

    #[cfg(not(windows))]
    entity.update(cx, |state, _| {
        state.error = Some("The overlay is only available on Windows.".into())
    });
}

#[cfg(windows)]
fn open_window(entity: &Entity<OverlayState>, cx: &mut App) {
    use gpui::{
        Bounds, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, point, px,
        size,
    };

    let bounds = cx.primary_display().map_or(
        Bounds::new(point(px(0.), px(0.)), size(px(1920.), px(1080.))),
        |display| display.bounds(),
    );

    let mut hwnd = None;
    let result = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: None,
            focus: false,
            show: true,
            kind: WindowKind::PopUp,
            is_movable: false,
            is_resizable: false,
            is_minimizable: false,
            window_background: WindowBackgroundAppearance::Transparent,
            app_id: Some("tutorlolv2-overlay".into()),
            ..Default::default()
        },
        |window, cx| {
            hwnd = window::hwnd(window);
            cx.new(OverlayView::new)
        },
    );

    entity.update(cx, |state, cx| {
        match result {
            Ok(view) => {
                if let Some(hwnd) = hwnd {
                    window::prepare(hwnd);
                }

                state.view = Some(view);
                state.hwnd = hwnd;
            }
            Err(error) => state.error = Some(format!("Could not open the overlay: {error}")),
        }

        cx.notify();
    });
}

pub fn is_open(cx: &App) -> bool {
    state(cx).is_some_and(|state| state.read(cx).open)
}

/// Opens (interactive) or closes (click-through) the overlay.
pub fn set_open(open: bool, cx: &mut App) {
    let Some(entity) = state(cx) else { return };

    let view = entity.update(cx, |state, cx| {
        if state.open == open {
            return None;
        }

        state.open = open;

        #[cfg(windows)]
        {
            keyboard::set_focused(open);

            if let Some(hwnd) = state.hwnd {
                window::set_interactive(hwnd, open, &mut state.previous);
            }
        }

        cx.notify();
        state.view
    });

    if let Some(view) = view {
        let _ = view.update(cx, |view, _, cx| view.set_open(open, cx));
    }
}

pub fn toggle(cx: &mut App) {
    let open = !is_open(cx);
    set_open(open, cx);
}

/// Validates, saves and applies a new toggle shortcut.
pub fn save_shortcut(shortcut: Shortcut, cx: &mut App) -> Result<(), String> {
    #[cfg_attr(not(windows), allow(unused_variables))]
    let binding = shortcut.validate()?;

    shortcut.save(&shortcut_path())?;

    #[cfg(windows)]
    keyboard::set_binding(binding);

    if let Some(entity) = state(cx) {
        entity.update(cx, |state, cx| {
            state.shortcut = shortcut;
            state.warning = None;
            cx.notify();
        });
    }

    Ok(())
}

/// Lets every key through while the settings page records a new shortcut.
pub fn set_recording(recording: bool) {
    #[cfg(windows)]
    keyboard::set_recording(recording);

    #[cfg(not(windows))]
    let _ = recording;
}
