//! The global overlay shortcuts: the desktop app's `wkb.rs` (a `WH_KEYBOARD_LL` hook).
//!
//! The hook runs on its own message thread and decides with [`KeyboardState`] whether a key
//! press is the toggle shortcut, Ctrl+O (next enemy) or Escape (close). Matching presses are
//! consumed so the game does not see them. Actions go to the UI over a channel; gpui's `App`
//! is not `Send`, unlike Tauri's `AppHandle`.
#![cfg(windows)]

use super::shortcut::{Action, KeyboardState, physical_key};
use std::{
    cell::RefCell,
    sync::{
        OnceLock,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, MAPVK_VK_TO_VSC_EX, MapVirtualKeyW},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, HC_ACTION, KBDLLHOOKSTRUCT,
            LLKHF_EXTENDED, MSG, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx,
            WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

static ACTIONS: OnceLock<UnboundedSender<Action>> = OnceLock::new();
/// Ctrl+K until the settings are loaded.
static BINDING: AtomicU32 = AtomicU32::new(0x24b);
/// While the settings page records a new shortcut, the hook lets every key through.
static RECORDING_UNTIL: AtomicU64 = AtomicU64::new(0);
/// Whether the overlay is open (interactive); Escape only closes an open overlay.
static FOCUSED: AtomicBool = AtomicBool::new(false);
static CLOCK: OnceLock<Instant> = OnceLock::new();

thread_local! { static KEYBOARD: RefCell<KeyboardState> = RefCell::new(KeyboardState::new()); }

pub fn set_binding(binding: u32) {
    BINDING.store(binding, Ordering::Release);
}

pub fn set_focused(focused: bool) {
    FOCUSED.store(focused, Ordering::Release);
}

/// Suspends the shortcuts for 30 seconds while a new one is recorded, or resumes them.
pub fn set_recording(recording: bool) {
    let until = match recording {
        true => CLOCK.get_or_init(Instant::now).elapsed().as_millis() as u64 + 30_000,
        false => 0,
    };

    RECORDING_UNTIL.store(until, Ordering::Release);
}

/// Installs the hook on a dedicated thread. Fails if it is already installed.
pub fn install(actions: UnboundedSender<Action>) -> Result<(), String> {
    CLOCK.get_or_init(Instant::now);

    ACTIONS
        .set(actions)
        .map_err(|_| "Keyboard hook is already installed")?;

    let (ready_tx, ready_rx) = std::sync::mpsc::sync_channel(1);

    thread::Builder::new()
        .name("overlay-keyboard".into())
        .spawn(move || unsafe {
            // Seed held keys before installing the hook. Never query asynchronous key state
            // in the callback.
            KEYBOARD.with(|state| {
                let mut state = state.borrow_mut();

                for vk in 0..256 {
                    if !matches!(vk, 0x10..=0x12)
                        && GetAsyncKeyState(vk as i32) as u16 & 0x8000 != 0
                    {
                        let scan = MapVirtualKeyW(vk, MAPVK_VK_TO_VSC_EX);
                        let key = physical_key(vk, scan & 0xff, scan & 0xff00 != 0);

                        state.process(key, true, 0, true, false);
                    }
                }
            });

            let hook = GetModuleHandleW(None).and_then(|module| {
                SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), Some(module.into()), 0)
            });

            let hook = match hook {
                Ok(hook) => {
                    let _ = ready_tx.send(Ok(()));
                    hook
                }
                Err(error) => {
                    let _ = ready_tx.send(Err(error.to_string()));
                    return;
                }
            };

            let mut msg = MSG::default();

            loop {
                let status = GetMessageW(&mut msg, None, 0, 0).0;

                if status <= 0 {
                    if status < 0 {
                        log::error!("Keyboard hook message loop failed");
                    }

                    break;
                }

                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = UnhookWindowsHookEx(hook);
        })
        .map_err(|error| error.to_string())?;

    ready_rx
        .recv_timeout(Duration::from_secs(5))
        .map_err(|error| error.to_string())?
        .map_err(|error| format!("Cannot install overlay keyboard hook: {error}"))
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if code == HC_ACTION as i32 {
            let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let msg = wparam.0 as u32;
            let down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            let up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

            // Remappers and accessibility keyboards can send injected events. This hook
            // never injects input itself, so accepting them cannot create a feedback loop.
            if (down || up) && kb.vkCode < 256 {
                let key = physical_key(kb.vkCode, kb.scanCode, kb.flags.contains(LLKHF_EXTENDED));

                let consume = KEYBOARD.with(|state| {
                    let mut state = state.borrow_mut();
                    let recording = CLOCK
                        .get()
                        .map(|start| {
                            (start.elapsed().as_millis() as u64)
                                < RECORDING_UNTIL.load(Ordering::Acquire)
                        })
                        .unwrap_or(false);

                    let decision = state.process(
                        key,
                        down,
                        BINDING.load(Ordering::Acquire),
                        recording,
                        FOCUSED.load(Ordering::Acquire),
                    );

                    if let Some(action) = decision.action
                        && !ACTIONS.get().is_some_and(|tx| tx.send(action).is_ok())
                    {
                        state.allow_passthrough(key);

                        return false;
                    }

                    decision.consume
                });

                if consume {
                    return LRESULT(1);
                }
            }
        }

        CallNextHookEx(None, code, wparam, lparam)
    }
}
