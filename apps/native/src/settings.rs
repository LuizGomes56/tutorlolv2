//! The Settings page: the web app's `desktop/mod.rs` (`Settings`) and `desktop/shortcut.rs`.
//!
//! Shows the overlay shortcut and records a new one: while recording, the global hook lets
//! keys through and the next key press with its modifiers becomes the shortcut. Escape
//! cancels. The overlay can also be opened from here to arrange its widgets.
use crate::{
    components::ui::{self, Tone},
    overlay::{self, Shortcut},
    theme,
};
use gpui::{
    Context, FocusHandle, FontWeight, IntoElement, KeyDownEvent, Keystroke, ParentElement, Render,
    SharedString, Styled, Subscription, Window, div, prelude::*, rgb,
};

/// The browser `KeyboardEvent.code` of a gpui key, as the shortcut settings store it.
fn key_code(key: &str) -> Option<String> {
    let code = match key {
        _ if key.len() == 1 && key.as_bytes()[0].is_ascii_lowercase() => {
            format!("Key{}", key.to_ascii_uppercase())
        }
        _ if key.len() == 1 && key.as_bytes()[0].is_ascii_digit() => format!("Digit{key}"),
        _ if key.len() > 1
            && key.starts_with('f')
            && key[1..].parse::<u32>().is_ok_and(|n| (1..=24).contains(&n)) =>
        {
            key.to_ascii_uppercase()
        }
        "space" => "Space".into(),
        "enter" => "Enter".into(),
        "tab" => "Tab".into(),
        "backspace" => "Backspace".into(),
        "insert" => "Insert".into(),
        "delete" => "Delete".into(),
        "home" => "Home".into(),
        "end" => "End".into(),
        "pageup" => "PageUp".into(),
        "pagedown" => "PageDown".into(),
        "left" => "ArrowLeft".into(),
        "up" => "ArrowUp".into(),
        "right" => "ArrowRight".into(),
        "down" => "ArrowDown".into(),
        ";" => "Semicolon".into(),
        "=" => "Equal".into(),
        "," => "Comma".into(),
        "-" => "Minus".into(),
        "." => "Period".into(),
        "/" => "Slash".into(),
        "`" => "Backquote".into(),
        "[" => "BracketLeft".into(),
        "\\" => "Backslash".into(),
        "]" => "BracketRight".into(),
        "'" => "Quote".into(),
        _ => return None,
    };

    Some(code)
}

fn from_keystroke(keystroke: &Keystroke) -> Result<Shortcut, String> {
    let code = key_code(&keystroke.key)
        .ok_or("Choose a letter, number, function key, navigation key or punctuation key")?;
    let modifiers = &keystroke.modifiers;

    let shortcut = Shortcut {
        code,
        ctrl: modifiers.control,
        alt: modifiers.alt,
        shift: modifiers.shift,
        meta: modifiers.platform,
    };

    shortcut.validate()?;
    Ok(shortcut)
}

/// `Ctrl+Shift+K`.
fn describe(shortcut: &Shortcut) -> String {
    let key = shortcut
        .code
        .strip_prefix("Key")
        .or_else(|| shortcut.code.strip_prefix("Digit"))
        .unwrap_or(&shortcut.code);

    [
        (shortcut.ctrl, "Ctrl"),
        (shortcut.alt, "Alt"),
        (shortcut.shift, "Shift"),
        (shortcut.meta, "Win"),
    ]
    .into_iter()
    .filter_map(|(on, name)| on.then_some(name))
    .chain(std::iter::once(key))
    .collect::<Vec<_>>()
    .join("+")
}

pub struct Settings {
    focus: FocusHandle,
    recording: bool,
    notice: Option<SharedString>,
    error: Option<SharedString>,
    _overlay: Option<Subscription>,
}

impl Settings {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let subscription =
            overlay::state(cx).map(|state| cx.observe(&state, |_, _, cx| cx.notify()));

        Self {
            focus: cx.focus_handle(),
            recording: false,
            notice: None,
            error: None,
            _overlay: subscription,
        }
    }

    fn record(&mut self, recording: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.recording = recording;
        overlay::set_recording(recording);

        if recording {
            window.focus(&self.focus);
            self.notice = Some("Press the new shortcut. Escape cancels.".into());
            self.error = None;
        }

        cx.notify();
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !self.recording {
            return;
        }

        if event.keystroke.key == "escape" {
            self.notice = None;
            self.record(false, window, cx);
            return;
        }

        match from_keystroke(&event.keystroke).and_then(|shortcut| {
            let text = describe(&shortcut);
            overlay::save_shortcut(shortcut, cx).map(|()| text)
        }) {
            Ok(text) => {
                self.notice = Some(format!("Saved: {text}").into());
                self.record(false, window, cx);
            }
            Err(error) => {
                self.error = Some(error.into());
                cx.notify();
            }
        }

        cx.stop_propagation();
    }
}

impl Render for Settings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = overlay::state(cx).map(|state| state.read(cx));
        let current = state.map_or_else(Shortcut::default, |state| state.shortcut.clone());
        let warning = state.and_then(|state| state.warning.clone());
        let overlay_error = state.and_then(|state| state.error.clone());

        let shortcut = ui::card()
            .child(ui::heading("Overlay shortcut"))
            .child(ui::muted(
                "Opens and closes the overlay from anywhere, including during a game. Ctrl+O \
                 selects the next enemy and Escape closes the overlay.",
            ))
            .child(
                ui::actions()
                    .child(
                        div()
                            .px_3()
                            .py_1p5()
                            .rounded_md()
                            .border_1()
                            .border_color(rgb(if self.recording {
                                theme::SKY_300
                            } else {
                                theme::STD_700
                            }))
                            .font_family(theme::CODE_FONT)
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::STD_100))
                            .child(match self.recording {
                                true => "Press keys…".to_owned(),
                                false => describe(&current),
                            }),
                    )
                    .child(ui::button(
                        "settings-record",
                        if self.recording {
                            "Cancel"
                        } else {
                            "Change shortcut"
                        },
                        Tone::Secondary,
                        false,
                        false,
                        cx.listener(|this, _, window, cx| {
                            let recording = !this.recording;
                            this.record(recording, window, cx)
                        }),
                    ))
                    .child(ui::button(
                        "settings-reset",
                        "Use Ctrl+K",
                        Tone::Secondary,
                        self.recording,
                        false,
                        cx.listener(|this, _, _, cx| {
                            match overlay::save_shortcut(Shortcut::default(), cx) {
                                Ok(()) => this.notice = Some("Saved: Ctrl+K".into()),
                                Err(error) => this.error = Some(error.into()),
                            }
                            cx.notify();
                        }),
                    )),
            )
            .children(self.notice.clone().map(|notice| ui::status(notice, false)))
            .children(self.error.clone().map(|error| ui::status(error, true)))
            .children(warning.map(|warning| ui::status(warning, true)));

        let widgets = ui::card()
            .child(ui::heading("Widgets and layout"))
            .child(ui::muted(
                "Open the overlay to move and resize widgets and choose which stay visible when \
                 you close it. Your choices and positions are saved on this device.",
            ))
            .child(ui::actions().child(ui::button(
                "settings-toggle-overlay",
                "Toggle overlay",
                Tone::Primary,
                overlay_error.is_some(),
                false,
                |_, _, cx| overlay::toggle(cx),
            )))
            .children(overlay_error.map(|error| ui::status(error, true)));

        ui::page("Overlay settings")
            .id("settings")
            .track_focus(&self.focus)
            .on_key_down(cx.listener(Self::key_down))
            .child(ui::muted(
                "Use one shortcut to open the overlay, arrange your widgets, and return to your \
                 game.",
            ))
            .child(shortcut)
            .child(widgets)
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        if self.recording {
            overlay::set_recording(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpui_keys_map_to_browser_codes() {
        assert_eq!(key_code("k").as_deref(), Some("KeyK"));
        assert_eq!(key_code("7").as_deref(), Some("Digit7"));
        assert_eq!(key_code("f12").as_deref(), Some("F12"));
        assert_eq!(key_code("left").as_deref(), Some("ArrowLeft"));
        assert_eq!(key_code("escape"), None);
        assert_eq!(describe(&Shortcut::default()), "Ctrl+K");
    }
}
