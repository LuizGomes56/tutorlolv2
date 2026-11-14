//! Sharing of the match being recorded: the web app's `desktop/sharing.rs` (`SharingBar`).
//!
//! The recording state comes from the [`Session`], which follows the backend's
//! `RecordingStateChanged` events. Modes: Private (only this device), Live (anyone with the code
//! can watch and rewind until the match ends) and Public (also after it ends).
use crate::{
    components::ui::{self, Tone},
    session::Session,
    theme,
};
use gpui::{
    ClipboardItem, Context, FontWeight, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div, prelude::*, rgb,
};
use protocol::{RecordingStatus, ShareMode, display_code};

pub struct SharingBar {
    busy: bool,
    error: Option<SharedString>,
    notice: Option<SharedString>,
    _session: Subscription,
}

impl SharingBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let session = cx.observe(&Session::global(cx), |_, _, cx| cx.notify());

        Self {
            busy: false,
            error: None,
            notice: None,
            _session: session,
        }
    }

    fn set_mode(&mut self, mode: ShareMode, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            return;
        };

        if self.busy {
            return;
        }

        self.busy = true;
        self.notice = None;
        cx.notify();

        let request = backend.spawn(backend.set_share_mode(mode));

        cx.spawn(async move |this, cx| {
            let result = request.await;

            this.update(cx, |this, cx| {
                this.busy = false;
                this.error = match result {
                    Ok(Ok(_)) => None,
                    Ok(Err(error)) => Some(error.into()),
                    Err(error) => Some(error.to_string().into()),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for SharingBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session = Session::global(cx).read(cx);
        let state = session.recording.clone();
        let start_error = session.start_error.clone();

        let available = state.as_ref().is_some_and(|state| {
            state.recording_id.is_some()
                && matches!(
                    state.status,
                    RecordingStatus::Recording | RecordingStatus::Suspended
                )
        });

        let public_reason = state.as_ref().and_then(|state| {
            if state.is_practice {
                Some("Public is unavailable for Practice Tool games.")
            } else if state.is_custom == Some(true) {
                Some("Public is unavailable for custom games.")
            } else if state.is_custom.is_none() {
                Some("Public is unavailable until the game type is known.")
            } else {
                None
            }
        });

        let mode = state
            .as_ref()
            .map_or(ShareMode::Private, |state| state.share.mode);

        let status = match state.as_ref().map(|state| state.status) {
            Some(RecordingStatus::Recording) => ("Recording", 0x34d399),
            Some(RecordingStatus::Suspended) => ("Paused", 0xfbbf24),
            Some(RecordingStatus::Starting) => ("Starting", theme::SKY_300),
            Some(RecordingStatus::Idle) | None => ("Not recording", theme::STD_400),
        };

        let buttons = [
            (ShareMode::Private, "Private"),
            (ShareMode::Live, "Live"),
            (ShareMode::Public, "Public"),
        ]
        .map(|(target, label)| {
            let disabled =
                self.busy || !available || (target == ShareMode::Public && public_reason.is_some());

            ui::button(
                SharedString::from(format!("share-{label}")),
                label,
                Tone::Secondary,
                disabled,
                mode == target,
                cx.listener(move |this, _, _, cx| this.set_mode(target, cx)),
            )
        });

        let description = match mode {
            ShareMode::Private => "Private: only this device can view the recording.",
            ShareMode::Live => {
                "Live: anyone with the code can watch and rewind until the match ends."
            }
            ShareMode::Public => {
                "Public: anyone with the code can watch and replay after the match ends."
            }
        };

        let share = state.as_ref().map(|state| state.share.clone());
        let code = share.as_ref().and_then(|share| share.code.clone());
        let url = share.as_ref().and_then(|share| share.url.clone());

        let copy = [("Copy code", code.clone()), ("Copy link", url)]
            .into_iter()
            .filter_map(|(label, text)| {
                text.map(|text| {
                    ui::button(
                        label,
                        label,
                        Tone::Secondary,
                        false,
                        false,
                        cx.listener(move |this, _, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
                            this.notice = Some(format!("{label}: copied").into());
                            cx.notify();
                        }),
                    )
                })
            });

        let upload_error = state
            .as_ref()
            .and_then(|state| state.upload.last_error.clone());

        ui::card()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_4()
                    .child(ui::heading("Share this match"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_sm()
                            .child(div().size_2().rounded_full().bg(rgb(status.1)))
                            .child(div().text_color(rgb(status.1)).child(status.0)),
                    ),
            )
            .child(ui::muted(description))
            .child(
                ui::actions()
                    .children(buttons)
                    .children(code.map(|code| {
                        div()
                            .px_3()
                            .font_family(theme::CODE_FONT)
                            .font_weight(FontWeight::BOLD)
                            .text_lg()
                            .text_color(rgb(theme::SKY_300))
                            .child(display_code(&code))
                    }))
                    .children(copy),
            )
            .when(!available, |card| {
                card.child(ui::muted("Start a match to share it."))
            })
            .children(public_reason.filter(|_| available).map(ui::muted))
            .children(
                upload_error.map(|error| {
                    ui::status(format!("Recording upload will retry: {error}"), false)
                }),
            )
            .when(self.busy, |card| {
                card.child(ui::status("Updating sharing...", false))
            })
            .children(self.notice.clone().map(|notice| ui::status(notice, false)))
            .children(self.error.clone().map(|error| ui::status(error, true)))
            .children(start_error.map(|error| ui::status(error, true)))
    }
}
