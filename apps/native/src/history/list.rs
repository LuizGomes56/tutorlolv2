//! The match history list: the web app's `history/list.rs` (`HistoryListView`, `HistoryRow`).
//!
//! Pages of 50 come from the backend's local cache, which refreshes from the server in the
//! background and reports `HistoryChanged`; the list reloads then. Native design: one card per
//! match with the champion, result, K/D/A and upload state, instead of the web's plain text.
use crate::{
    components::{
        hover_docs::HoverDocsExt,
        image::{Entry, image},
        ui::{self, Tone, duration, local_time},
    },
    route::{Route, navigate},
    session::Session,
    theme,
};
use gpui::{
    Context, FontWeight, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Task, Window, div, prelude::*, rgb,
};
use math::ChampionId;
use protocol::{GameResult, HistoryEntry, HistoryList as Page};
use std::{collections::BTreeSet, str::FromStr};

const PAGE_SIZE: u32 = 50;

/// `result`.
pub fn result(value: Option<GameResult>) -> &'static str {
    match value {
        Some(GameResult::Win) => "Win",
        Some(GameResult::Lose) => "Loss",
        Some(GameResult::Unknown) => "Unknown result",
        Some(GameResult::Remake) => "Remake",
        None => "Result pending",
    }
}

fn result_color(value: Option<GameResult>) -> u32 {
    match value {
        Some(GameResult::Win) => 0x34d399,
        Some(GameResult::Lose) => 0xfb7185,
        _ => theme::STD_400,
    }
}

/// `upload`: where the recording is in its upload.
pub fn upload(entry: &HistoryEntry) -> String {
    if entry.recording_id.is_none() {
        return "Waiting to upload".into();
    }

    if entry.summary.is_none() {
        return "Waiting for metadata".into();
    }

    let mut parts = vec![];

    if entry.lost_chunks > 0 {
        parts.push(format!("Capture loss: {} chunks", entry.lost_chunks));
    }

    if entry.pending_chunks > 0 || entry.pending_control {
        parts.push(format!(
            "Uploading: {} chunks{}",
            entry.pending_chunks,
            if entry.pending_control {
                " · control pending"
            } else {
                ""
            }
        ));
    }

    if parts.is_empty() {
        parts.push(
            if entry
                .summary
                .as_ref()
                .is_some_and(|s| s.ended_at_ms.is_none())
            {
                "Ongoing"
            } else {
                "Complete"
            }
            .into(),
        );
    }

    parts.join(" · ")
}

/// `uuid`: the canonical text form of a recording id.
pub fn uuid(id: [u8; 16]) -> String {
    ::api::Uuid::from_bytes(id).hyphenated().to_string()
}

/// `identity`: server id, or the local id while the recording waits for one.
pub fn identity(entry: &HistoryEntry) -> String {
    entry
        .recording_id
        .map(uuid)
        .unwrap_or_else(|| format!("local:{}", entry.local_id.map(uuid).unwrap_or_default()))
}

pub struct HistoryList {
    page: Option<Page>,
    offset: u32,
    error: Option<SharedString>,
    notice: Option<SharedString>,
    repairing: bool,
    /// The row asking to confirm hiding.
    confirming: Option<String>,
    /// Rows whose hide request is running.
    hiding: BTreeSet<String>,
    row_errors: std::collections::BTreeMap<String, SharedString>,
    revision: u64,
    _load: Option<Task<()>>,
    _session: Subscription,
}

impl HistoryList {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let session = Session::global(cx);
        let revision = session.read(cx).history_revision;

        let subscription = cx.observe(&session, |this: &mut Self, session, cx| {
            let revision = session.read(cx).history_revision;

            if revision != this.revision {
                this.revision = revision;
                this.load(cx);
            }

            cx.notify();
        });

        let mut list = Self {
            page: None,
            offset: 0,
            error: None,
            notice: None,
            repairing: false,
            confirming: None,
            hiding: BTreeSet::new(),
            row_errors: Default::default(),
            revision,
            _load: None,
            _session: subscription,
        };

        list.load(cx);
        list
    }

    fn load(&mut self, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            self.error = Session::global(cx).read(cx).start_error.clone();
            return;
        };

        let request = backend.spawn(backend.list_history(self.offset, PAGE_SIZE));

        self._load = Some(cx.spawn(async move |this, cx| {
            let result = request.await;

            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(page)) => {
                        this.page = Some(page);
                        this.error = None;
                    }
                    Ok(Err(error)) => this.error = Some(format!("List: {}", error.message).into()),
                    Err(error) => this.error = Some(format!("List: {error}").into()),
                }

                cx.notify();
            });
        }));
    }

    fn turn(&mut self, offset: u32, cx: &mut Context<Self>) {
        self.offset = offset;
        self.load(cx);
        cx.notify();
    }

    fn repair(&mut self, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            return;
        };

        backend.repair_history();
        self.notice = Some(
            "Repair queued. Server metadata and pending recordings are being reconciled. Hidden \
             flags cannot be recovered from a lost index."
                .into(),
        );
        self.load(cx);
        cx.notify();
    }

    fn hide(&mut self, id: String, cx: &mut Context<Self>) {
        let Some(backend) = Session::backend(cx) else {
            return;
        };

        if !self.hiding.insert(id.clone()) {
            return;
        }

        // Hiding only writes the local index, so it is quick enough to do here.
        let result = backend.hide_history_entry(&id);

        self.hiding.remove(&id);
        self.confirming = None;

        match result {
            Ok(()) => {
                self.row_errors.remove(&id);
                self.load(cx);
            }
            Err(error) => {
                self.row_errors
                    .insert(id, format!("Hide: {}", error.message).into());
            }
        }

        cx.notify();
    }

    fn row(&self, entry: &HistoryEntry, cx: &mut Context<Self>) -> gpui::AnyElement {
        let key = identity(entry);
        let id = entry.recording_id.map(uuid);

        let (title, details) = match &entry.summary {
            Some(summary) => {
                let recording = &summary.recording;
                let champion = ChampionId::from_str(&recording.champion).ok();

                let icon = match champion {
                    Some(champion) => div()
                        .id(SharedString::from(format!("history-champion-{key}")))
                        .hover_docs(champion)
                        .child(image(champion.image_source()).size_12().rounded_md())
                        .into_any_element(),
                    None => div()
                        .size_12()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_md()
                        .bg(rgb(theme::STD_800))
                        .child("?")
                        .into_any_element(),
                };

                let name = champion.map_or(recording.champion.clone(), |c| c.name().to_owned());
                let length = recording
                    .game_length_s
                    .map(|v| v as f32)
                    .unwrap_or(recording.last_game_time);

                let title = div().flex().items_center().gap_3().child(icon).child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::STD_100))
                                        .child(name),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(result_color(recording.result)))
                                        .child(result(recording.result)),
                                ),
                        )
                        .child(ui::muted(format!(
                            "{:?} · Queue {} · {}",
                            recording.game_mode,
                            recording
                                .queue_id
                                .map(|queue| queue.to_string())
                                .unwrap_or_else(|| "—".into()),
                            local_time(recording.started_at_ms)
                        ))),
                );

                let details = ui::muted(format!(
                    "{}/{}/{} · {} CS · {}{} · {} s covered",
                    summary.kills,
                    summary.deaths,
                    summary.assists,
                    summary.creep_score,
                    duration(length),
                    if recording.game_length_s.is_none() {
                        " (incomplete)"
                    } else {
                        ""
                    },
                    summary.coverage_s
                ));

                (title.into_any_element(), details)
            }
            None => (
                ui::heading("Waiting for upload / metadata").into_any_element(),
                ui::muted("Result — · K/D/A — · CS — · Duration —"),
            ),
        };

        let state = ui::muted(format!("{:?} · {}", entry.share.mode, upload(entry)));

        let actions = id.clone().map(|id| {
            let busy = self.hiding.contains(&id);
            let open = {
                let id = id.clone();
                move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut gpui::App| {
                    navigate(Route::HistoryDetails { id: id.clone() }, cx)
                }
            };

            let actions = ui::actions().child(ui::button(
                SharedString::from(format!("open-{id}")),
                "Open replay",
                Tone::Primary,
                false,
                false,
                open,
            ));

            match self.confirming.as_deref() == Some(id.as_str()) {
                true => actions
                    .child(ui::muted(
                        "Remove only from this device's history? Sharing and uploads continue.",
                    ))
                    .child(ui::button(
                        SharedString::from(format!("hide-{id}")),
                        "Hide on this device",
                        Tone::Danger,
                        busy,
                        false,
                        cx.listener({
                            let id = id.clone();
                            move |this, _, _, cx| this.hide(id.clone(), cx)
                        }),
                    ))
                    .child(ui::button(
                        SharedString::from(format!("cancel-{id}")),
                        "Cancel",
                        Tone::Secondary,
                        busy,
                        false,
                        cx.listener(|this, _, _, cx| {
                            this.confirming = None;
                            cx.notify();
                        }),
                    )),
                false => actions.child(ui::button(
                    SharedString::from(format!("ask-hide-{id}")),
                    "Hide…",
                    Tone::Secondary,
                    false,
                    false,
                    cx.listener({
                        let id = id.clone();
                        move |this, _, _, cx| {
                            this.confirming = Some(id.clone());
                            cx.notify();
                        }
                    }),
                )),
            }
        });

        ui::card()
            .child(title)
            .child(details)
            .child(state)
            .children(
                entry
                    .last_error
                    .clone()
                    .map(|error| ui::status(error, false)),
            )
            .children(actions)
            .children(
                id.and_then(|id| self.row_errors.get(&id).cloned())
                    .map(|error| ui::status(error, true)),
            )
            .into_any_element()
    }
}

impl Render for HistoryList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = ui::actions()
            .child(ui::button(
                "history-refresh",
                "Refresh",
                Tone::Secondary,
                false,
                false,
                cx.listener(|this, _, _, cx| this.load(cx)),
            ))
            .child(ui::button(
                "history-repair",
                "Repair index",
                Tone::Secondary,
                self.repairing,
                false,
                cx.listener(|this, _, _, cx| this.repair(cx)),
            ));

        let mut page = ui::page("Match history")
            .child(ui::muted(
                "Matches recorded on this device and, when signed in, your account.",
            ))
            .child(header)
            .children(self.notice.clone().map(|notice| ui::status(notice, false)))
            .children(self.error.clone().map(|error| {
                ui::actions()
                    .child(ui::status(error, true))
                    .child(ui::button(
                        "history-retry",
                        "Retry",
                        Tone::Secondary,
                        false,
                        false,
                        cx.listener(|this, _, _, cx| this.load(cx)),
                    ))
            }));

        let Some(list) = self.page.clone() else {
            return page.child(ui::status("Loading cached history…", false));
        };

        if list.refreshing {
            page = page.child(ui::status("Refreshing cached history…", false));
        }

        if let Some(error) = &list.refresh_error {
            page = page.child(ui::status(
                format!("Showing cached metadata. Refresh: {error}"),
                false,
            ));
        }

        if list.entries.is_empty() {
            page = page.child(ui::muted("No recorded matches on this page."));
        }

        let rows = list
            .entries
            .iter()
            .map(|entry| self.row(entry, cx))
            .collect::<Vec<_>>();

        let offset = self.offset;

        page.child(div().flex().flex_col().gap_3().children(rows))
            .child(
                ui::actions()
                    .child(ui::button(
                        "history-previous",
                        "Previous 50",
                        Tone::Secondary,
                        offset == 0,
                        false,
                        cx.listener(move |this, _, _, cx| {
                            this.turn(offset.saturating_sub(PAGE_SIZE), cx)
                        }),
                    ))
                    .child(ui::muted(format!("Page {}", offset / PAGE_SIZE + 1)))
                    .child(ui::button(
                        "history-next",
                        "Next 50",
                        Tone::Secondary,
                        !list.has_more,
                        false,
                        cx.listener(move |this, _, _, cx| this.turn(offset + PAGE_SIZE, cx)),
                    )),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{ShareMode, ShareState};

    fn provisional() -> HistoryEntry {
        HistoryEntry {
            local_id: Some([1; 16]),
            recording_id: None,
            summary: None,
            share: ShareState {
                mode: ShareMode::Private,
                code: None,
                url: None,
            },
            pending_chunks: 3,
            pending_control: true,
            lost_chunks: 1,
            last_error: None,
            hidden: false,
        }
    }

    #[test]
    fn provisional_identity_never_becomes_a_server_identity() {
        let mut entry = provisional();
        assert!(identity(&entry).starts_with("local:"));
        assert_eq!(upload(&entry), "Waiting to upload");
        entry.recording_id = Some([2; 16]);
        assert_eq!(identity(&entry), uuid([2; 16]));
        assert_eq!(upload(&entry), "Waiting for metadata");
    }

    #[test]
    fn unknown_results_and_champions_stay_honest() {
        assert_eq!(result(None), "Result pending");
        assert_eq!(result(Some(GameResult::Unknown)), "Unknown result");
        assert_eq!(
            ChampionId::from_str("aatrox").ok(),
            Some(ChampionId::Aatrox)
        );
        assert_eq!(ChampionId::from_str("future-champion").ok(), None);
        assert_eq!(uuid([0; 16]), "00000000-0000-0000-0000-000000000000");
    }
}
