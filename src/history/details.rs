//! One recorded match: the web app's `history/details.rs` (`HistoryDetailsView`,
//! `ShareControls`), with its replay and charts.
//!
//! Details are fetched again every five seconds while the match is ongoing, and whenever the
//! backend reports a history change. Choosing a sample in the charts seeks the replay.
use crate::{
    components::ui::{self, Tone},
    history::{
        charts::{Charts, Seek},
        list::upload,
    },
    replay::{ReplaySource, ReplayView},
    route::{Route, navigate},
    session::Session,
    theme,
};
use gpui::{
    AppContext, ClipboardItem, Context, Entity, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, Subscription, Task, Window, div, prelude::*, rgb,
};
use std::time::Duration;
use tutorlolv2_protocol::{HistoryDetails as Details, HistoryShareMode, ShareMode, display_code};

const ONGOING_POLL: Duration = Duration::from_secs(5);

pub struct HistoryDetails {
    id: String,
    details: Option<Details>,
    error: Option<SharedString>,
    share_busy: bool,
    share_error: Option<SharedString>,
    notice: Option<SharedString>,
    replay: Option<Entity<ReplayView>>,
    charts: Option<Entity<Charts>>,
    revision: u64,
    _load: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl HistoryDetails {
    pub fn new(id: String, cx: &mut Context<Self>) -> Self {
        let session = Session::global(cx);
        let revision = session.read(cx).history_revision;

        let subscription = cx.observe(&session, |this: &mut Self, session, cx| {
            let revision = session.read(cx).history_revision;

            if revision != this.revision {
                this.revision = revision;
                this.load(cx);
            }
        });

        let mut details = Self {
            id,
            details: None,
            error: None,
            share_busy: false,
            share_error: None,
            notice: None,
            replay: None,
            charts: None,
            revision,
            _load: None,
            _subscriptions: vec![subscription],
        };

        details.load(cx);
        details
    }

    /// Fetches the details, then again every five seconds until the match has ended.
    fn load(&mut self, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            self.error = Session::global(cx).read(cx).start_error.clone();
            return;
        };

        let id = self.id.clone();

        self._load = Some(cx.spawn(async move |this, cx| {
            loop {
                let result = backend.spawn(backend.history_details(id.clone())).await;

                let ongoing = this
                    .update(cx, |this, cx| {
                        let ongoing = match result {
                            Ok(Ok(details)) => {
                                let ongoing = details.recording.summary.ended_at_ms.is_none();
                                this.set_details(details, cx);
                                this.error = None;
                                ongoing
                            }
                            Ok(Err(error)) => {
                                this.error = Some(format!("Details: {}", error.message).into());
                                false
                            }
                            Err(error) => {
                                this.error = Some(format!("Details: {error}").into());
                                false
                            }
                        };

                        cx.notify();
                        ongoing
                    })
                    .unwrap_or(false);

                if !ongoing {
                    break;
                }

                cx.background_executor().timer(ONGOING_POLL).await;
            }
        }));
    }

    fn set_details(&mut self, details: Details, cx: &mut Context<Self>) {
        let recording = &details.recording;
        let chunks = recording.chunks.clone();
        let end = recording
            .summary
            .recording
            .game_length_s
            .map(|v| v as f32)
            .unwrap_or(recording.summary.recording.last_game_time);

        match &self.replay {
            Some(replay) => {
                replay.update(cx, |replay, cx| replay.set_chunks(chunks.clone(), end, cx))
            }
            None => {
                let source = ReplaySource::Owner(self.id.clone());
                let replay = cx.new(|cx| ReplayView::new(source.clone(), chunks.clone(), end, cx));
                let charts = cx.new(|cx| Charts::new(source, chunks.clone(), cx));

                let replay_handle = replay.clone();
                self._subscriptions
                    .push(cx.subscribe(&charts, move |_, _, Seek(time), cx| {
                        replay_handle.update(cx, |replay, cx| replay.seek(*time, cx));
                    }));

                self.replay = Some(replay);
                self.charts = Some(charts);
            }
        }

        if let Some(charts) = &self.charts
            && self.details.as_ref().map(|old| &old.recording.chunks) != Some(&chunks)
            && self.details.is_some()
        {
            charts.update(cx, |charts, cx| charts.load(chunks, cx));
        }

        self.details = Some(details);
    }

    fn set_share(&mut self, mode: HistoryShareMode, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            return;
        };

        if self.share_busy {
            return;
        }

        self.share_busy = true;
        cx.notify();

        let request = backend.spawn(backend.set_history_share_mode(self.id.clone(), mode));

        cx.spawn(async move |this, cx| {
            let result = request.await;

            let _ = this.update(cx, |this, cx| {
                this.share_busy = false;

                match result {
                    Ok(Ok(share)) => {
                        if let Some(details) = &mut this.details {
                            details.recording.share = share.clone();
                            details.entry.share = share;
                        }

                        this.share_error = None;
                    }
                    Ok(Err(error)) => {
                        this.share_error = Some(format!("Sharing: {}", error.message).into())
                    }
                    Err(error) => this.share_error = Some(format!("Sharing: {error}").into()),
                }

                cx.notify();
            });
        })
        .detach();
    }

    /// `ShareControls` and the sharing notes.
    fn sharing(&self, details: &Details, cx: &mut Context<Self>) -> gpui::Div {
        let summary = &details.recording.summary;
        let share = &details.recording.share;
        let ended = summary.ended_at_ms.is_some();
        let restricted =
            summary.recording.is_practice || summary.recording.is_custom != Some(false);

        let modes = [
            (HistoryShareMode::Private, "Private"),
            (HistoryShareMode::Public, "Public"),
        ]
        .map(|(mode, label)| {
            let disabled =
                self.share_busy || !ended || (mode == HistoryShareMode::Public && restricted);

            ui::button(
                SharedString::from(format!("history-share-{label}")),
                label,
                Tone::Secondary,
                disabled,
                ShareMode::from(mode) == share.mode,
                cx.listener(move |this, _, _, cx| this.set_share(mode, cx)),
            )
        });

        let copy = [
            ("Copy code", share.code.clone()),
            ("Copy link", share.url.clone()),
        ]
        .into_iter()
        .filter_map(|(label, text)| {
            text.map(|text| {
                ui::button(
                    SharedString::from(format!("history-{label}")),
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

        let open = share.url.clone().map(|url| {
            ui::button(
                "history-open-viewer",
                "Open public viewer",
                Tone::Secondary,
                false,
                false,
                move |_, _, cx| cx.open_url(&url),
            )
        });

        let status = match (self.share_busy, &self.notice) {
            (true, _) => "Updating sharing...".into(),
            (false, Some(notice)) => notice.clone(),
            (false, None) => "Owner access remains available with Private sharing.".into(),
        };

        ui::card()
            .child(ui::heading("Sharing"))
            .child(ui::muted(format!("Confirmed: {:?}", share.mode)))
            .child(
                ui::actions()
                    .children(modes)
                    .children(share.code.as_ref().map(|code| {
                        div()
                            .px_3()
                            .font_family(theme::CODE_FONT)
                            .font_weight(FontWeight::BOLD)
                            .text_lg()
                            .text_color(rgb(theme::SKY_300))
                            .child(display_code(code))
                    }))
                    .children(copy)
                    .children(open),
            )
            .child(ui::status(status, false))
            .children(
                self.share_error
                    .clone()
                    .map(|error| ui::status(error, true)),
            )
            .when(!ended, |card| {
                card.child(ui::muted(
                    "Sharing controls become available after the server confirms the match ended.",
                ))
            })
            .when(restricted, |card| {
                card.child(ui::muted(
                    "Public is unavailable for practice/custom games or while the game type is \
                     unknown.",
                ))
            })
    }
}

impl Render for HistoryDetails {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let back = ui::actions()
            .child(ui::button(
                "history-back",
                "← Back to history",
                Tone::Secondary,
                false,
                false,
                |_, _, cx| navigate(Route::History, cx),
            ))
            .child(ui::button(
                "history-refresh-details",
                "Refresh details",
                Tone::Secondary,
                false,
                false,
                cx.listener(|this, _, _, cx| this.load(cx)),
            ));

        let page = ui::page("Recorded match")
            .max_w_full()
            .child(back)
            .children(self.error.clone().map(|error| ui::status(error, true)));

        let Some(details) = self.details.clone() else {
            return page.when(self.error.is_none(), |page| {
                page.child(ui::status("Loading owner recording…", false))
            });
        };

        let recording = &details.recording.summary.recording;

        page.child(
            ui::card()
                .child(ui::heading(format!(
                    "{} · {}",
                    recording.champion, recording.active_riot_id
                )))
                .child(ui::muted(upload(&details.entry)))
                .children(
                    details
                        .entry
                        .last_error
                        .clone()
                        .map(|error| ui::status(error, false)),
                ),
        )
        .child(self.sharing(&details, cx))
        .children(self.replay.clone())
        .children(self.charts.clone())
    }
}
