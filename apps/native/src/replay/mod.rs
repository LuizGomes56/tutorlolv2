//! The replay viewer: the web app's `replay/mod.rs` (`OwnerReplay`) and `replay/controller.rs`.
//!
//! A recording is a list of chunks of frames (Riot snapshots, about one per second). The
//! viewer keeps the chunks around the current time ([`cache::retain_neighbors`]), picks the
//! frame for the time ([`cache::selected_index`]), calculates it on a background thread and
//! shows it with the live game layout. Seeking or playing replaces the load in progress.
pub mod cache;
pub mod extent;
pub mod timeline;

use crate::{
    components::{
        slider::slider,
        ui::{self, Tone, duration},
    },
    livegame::{LivegameView, game::Game},
    session::Session,
    theme,
};
use cache::{CachedChunk, ChunkCache};
use extent::ExtentStore;
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, Render, SharedString, Styled, Task, Window, div,
    prelude::*, rgb,
};
use protocol::ChunkInfo;
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc, time::Duration};

/// Playback advances in steps of this many game seconds per real second fraction.
const PLAY_STEP: Duration = Duration::from_millis(250);

/// Where chunks come from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplaySource {
    /// The owner's recording, by id (History).
    Owner(String),
    /// A shared match, by code (Watch).
    Capability(String),
}

pub type Load = Pin<Box<dyn Future<Output = Result<Vec<u8>, String>>>>;

impl ReplaySource {
    /// The chunk's bytes, fetched by the backend.
    pub fn load(&self, seq: u32, cx: &App) -> Load {
        #[cfg(debug_assertions)]
        if let Self::Capability(code) = self
            && code == crate::watch::demo::CODE
        {
            let frames = crate::watch::demo::frames(seq);
            return Box::pin(async move { frames });
        }

        let Some(backend) = Session::backend(cx) else {
            return Box::pin(async { Err("The recorder is not running".to_owned()) });
        };

        match self {
            Self::Owner(id) => {
                let request = backend.spawn(backend.history_frames(id.clone(), seq));

                Box::pin(async move {
                    match request.await {
                        Ok(result) => result.map_err(|error| error.message),
                        Err(error) => Err(error.to_string()),
                    }
                })
            }
            Self::Capability(code) => {
                let request = backend.spawn(backend.watch_frames(code.clone(), seq));

                Box::pin(async move { request.await.map_err(|e| e.to_string())? })
            }
        }
    }
}

/// An access error (revoked or unknown code), which ends a Watch session.
pub struct AccessError(pub String);

pub struct ReplayView {
    source: ReplaySource,
    chunks: Vec<ChunkInfo>,
    /// The recording's end before chunks are observed: its length, or its last game time.
    end_hint: f32,
    time: f32,
    playing: bool,
    speed: u32,
    cache: ChunkCache,
    extents: ExtentStore,
    /// The chunk and frame index currently shown, to skip recalculating the same frame.
    shown: Option<(u32, usize)>,
    message: Option<SharedString>,
    view: Entity<LivegameView>,
    focus: FocusHandle,
    _load: Option<Task<()>>,
    _play: Option<Task<()>>,
}

impl EventEmitter<AccessError> for ReplayView {}

impl ReplayView {
    pub fn new(
        source: ReplaySource,
        mut chunks: Vec<ChunkInfo>,
        end_hint: f32,
        cx: &mut Context<Self>,
    ) -> Self {
        chunks.sort_by(|a, b| a.first_game_time.total_cmp(&b.first_game_time));

        let time = chunks.first().map_or(0., |chunk| chunk.first_game_time);
        let mut replay = Self {
            source,
            chunks,
            end_hint,
            time,
            playing: false,
            speed: 1,
            cache: ChunkCache::new(),
            extents: ExtentStore::default(),
            shown: None,
            message: None,
            view: cx.new(LivegameView::new),
            focus: cx.focus_handle(),
            _load: None,
            _play: None,
        };

        replay.reload(cx);
        replay
    }

    /// New chunk metadata (an ongoing recording grows). Keeps the current time.
    pub fn set_chunks(
        &mut self,
        mut chunks: Vec<ChunkInfo>,
        end_hint: f32,
        cx: &mut Context<Self>,
    ) {
        chunks.sort_by(|a, b| a.first_game_time.total_cmp(&b.first_game_time));

        if chunks != self.chunks || end_hint != self.end_hint {
            self.chunks = chunks;
            self.end_hint = end_hint;
            self.shown = None;
            self.reload(cx);
        }
    }

    fn end(&self) -> f32 {
        self.extents.timeline(&self.chunks, self.end_hint).1
    }

    pub fn seek(&mut self, time: f32, cx: &mut Context<Self>) {
        self.time = time.clamp(0., self.end().max(0.));
        self.reload(cx);
        cx.notify();
    }

    fn toggle_play(&mut self, cx: &mut Context<Self>) {
        self.playing = !self.playing;

        self._play = self.playing.then(|| {
            cx.spawn(async move |this, cx| {
                loop {
                    cx.background_executor().timer(PLAY_STEP).await;

                    let keep = this
                        .update(cx, |this, cx| {
                            let end = this.end();
                            let next = (this.time + 0.25 * this.speed as f32).min(end);

                            this.seek(next, cx);

                            if next >= end {
                                this.playing = false;
                            }

                            this.playing
                        })
                        .unwrap_or(false);

                    if !keep {
                        break;
                    }
                }
            })
        });

        cx.notify();
    }

    fn show(
        &mut self,
        game: Option<Rc<Game>>,
        message: Option<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.message = message;
        self.view
            .update(cx, |view, cx| view.set_game(game, None, cx));
        cx.notify();
    }

    /// Loads the chunks around the current time and shows its frame.
    fn reload(&mut self, cx: &mut Context<Self>) {
        let time = self.time;
        let ranges = self.extents.timeline(&self.chunks, time).0;

        cache::retain_neighbors(&mut self.cache, &ranges, time);

        let Some(index) = timeline::current_index(&ranges, time) else {
            self.shown = None;
            self.show(
                None,
                Some("No recording at this time. Choose a covered part of the timeline.".into()),
                cx,
            );
            return;
        };

        let current = self.chunks[index].seq;
        let next = ranges.get(index + 1).map(|chunk| chunk.first_game_time);
        let mut order = vec![current];

        order.extend(
            timeline::neighbors(&ranges, time)
                .into_iter()
                .filter(|seq| *seq != current),
        );

        self._load = Some(cx.spawn(async move |this, cx| {
            for seq in order {
                let Ok((metadata, cached, load)) = this.update(cx, |this, cx| {
                    let metadata = this.chunks.iter().find(|c| c.seq == seq).cloned();
                    let cached = metadata.as_ref().and_then(|metadata| {
                        this.cache
                            .get(&seq)
                            .filter(|chunk| {
                                this.extents.matches(metadata) && chunk.validate(metadata).is_ok()
                            })
                            .cloned()
                    });

                    let load = cached.is_none().then(|| this.source.load(seq, cx));

                    if cached.is_none() && seq == current && this.shown.is_none() {
                        this.message = Some("Loading replay…".into());
                        cx.notify();
                    }

                    (metadata, cached, load)
                }) else {
                    return;
                };

                let Some(metadata) = metadata else { continue };

                let chunk = match (cached, load) {
                    (Some(chunk), _) => Ok(chunk),
                    (None, Some(load)) => match load.await {
                        Ok(bytes) => {
                            let format = metadata.frames_format;
                            let decoded = cx
                                .background_spawn(async move { CachedChunk::decode(bytes, format) })
                                .await;

                            decoded.and_then(|chunk| chunk.validate(&metadata).map(|()| chunk))
                        }
                        Err(error) => Err(error),
                    },
                    (None, None) => continue,
                };

                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(error) => {
                        let access = error.starts_with("This match")
                            || error.starts_with("Code not found")
                            || error.contains("404");

                        let _ = this.update(cx, |this, cx| {
                            if access && matches!(this.source, ReplaySource::Capability(_)) {
                                cx.emit(AccessError(error.clone()));
                            }

                            if seq == current {
                                this.shown = None;
                                this.show(None, Some(error.into()), cx);
                            }
                        });

                        continue;
                    }
                };

                let Ok(selected) = this.update(cx, |this, cx| {
                    if this.extents.observe_chunk(&metadata, &chunk) {
                        cx.notify();
                    }

                    this.cache.insert(seq, chunk.clone());

                    if seq != current {
                        return None;
                    }

                    match cache::selected_index(&chunk.times, time, next) {
                        Some(frame) if this.shown == Some((seq, frame)) => {
                            this.message = None;
                            None
                        }
                        Some(frame) => Some(frame),
                        None => {
                            this.shown = None;
                            this.show(
                                None,
                                Some(
                                    "No recording at this time. Choose a covered part of the timeline."
                                        .into(),
                                ),
                                cx,
                            );
                            None
                        }
                    }
                }) else {
                    return;
                };

                let Some(frame) = selected else { continue };
                let calculated = calculate(chunk, frame, cx).await;

                let _ = this.update(cx, |this, cx| match calculated {
                    Ok(game) => {
                        this.shown = Some((seq, frame));
                        this.show(Some(Rc::new(game)), None, cx);
                    }
                    Err(error) => {
                        this.shown = None;
                        this.show(None, Some(error.into()), cx);
                    }
                });
            }
        }));
    }
}

/// Calculates one frame of a chunk on a background thread.
async fn calculate(
    chunk: Arc<CachedChunk>,
    frame: usize,
    cx: &mut gpui::AsyncApp,
) -> Result<Game, String> {
    cx.background_spawn(async move {
        let frame = chunk.frame(frame).ok_or("Missing replay frame")?;
        Game::from_frame(&frame).map_err(|error| error.to_string())
    })
    .await
}

impl Focusable for ReplayView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for ReplayView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let end = self.end();
        let ranges = self.extents.timeline(&self.chunks, end).0;
        let gaps = timeline::gaps(&ranges, end);
        let empty = self.chunks.is_empty();

        let speeds = [1, 4, 16].map(|speed| {
            ui::button(
                SharedString::from(format!("replay-speed-{speed}")),
                format!("{speed}×"),
                Tone::Secondary,
                false,
                self.speed == speed,
                cx.listener(move |this, _, _, cx| {
                    this.speed = speed;
                    cx.notify();
                }),
            )
        });

        let timeline = ui::card()
            .id("replay")
            .track_focus(&self.focus)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, _| window.focus(&this.focus)),
            )
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                match event.keystroke.key.as_str() {
                    "left" => this.seek(this.time - 10., cx),
                    "right" => this.seek(this.time + 10., cx),
                    "space" => this.toggle_play(cx),
                    _ => {}
                }
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(ui::heading(format!(
                        "Replay {} / {}",
                        duration(self.time),
                        duration(end)
                    )))
                    .child(ui::muted(
                        "← / → seek 10 seconds, Space plays. Shaded parts were not captured.",
                    )),
            )
            .child(slider(
                "replay-time",
                self.time,
                end,
                empty,
                &gaps,
                cx.listener(|this, time: &f32, _, cx| this.seek(*time, cx)),
            ))
            .child(
                ui::actions()
                    .child(ui::button(
                        "replay-play",
                        if self.playing { "Pause" } else { "Play" },
                        Tone::Primary,
                        empty,
                        false,
                        cx.listener(|this, _, _, cx| this.toggle_play(cx)),
                    ))
                    .children(speeds),
            )
            .children(self.message.clone().map(|message| {
                ui::actions()
                    .child(ui::status(message, false))
                    .child(ui::button(
                        "replay-retry",
                        "Retry replay",
                        Tone::Secondary,
                        false,
                        false,
                        cx.listener(|this, _, _, cx| {
                            this.cache.clear();
                            this.shown = None;
                            this.reload(cx);
                        }),
                    ))
            }));

        div()
            .flex()
            .flex_col()
            .gap_4()
            .text_color(rgb(theme::TEXT))
            .child(timeline)
            .child(self.view.clone())
    }
}
