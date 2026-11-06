//! The Watch page: the web app's `watch/mod.rs` (`Watch`, `Viewer`).
//!
//! Anyone with a share code can watch a match: live while it is played (Live sharing), and as
//! a replay of the shared chunks (Live until the match ends, Public after). The match info is
//! fetched again every five seconds, so a revoked code closes the viewer. Live frames are
//! polled one after another; replays use the shared [`ReplayView`].
//!
//! Native design: a code field and a viewer card with a Live / Replay switch, instead of the
//! web's single timeline that follows the live edge.
use crate::{
    components::{
        text_input::{TextInput, TextInputEvent},
        ui::{self, Tone},
    },
    livegame::{LivegameView, game::Game},
    replay::{AccessError, ReplaySource, ReplayView},
    route::{Route, navigate},
    session::Session,
    theme,
};
use gpui::{
    AppContext, Context, Entity, Focusable, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, Subscription, Task, Window, div, prelude::*, px, rgb,
};
use std::{rc::Rc, time::Duration};
use tutorlolv2_protocol::{ShareInfo, ShareStatus, display_code, normalize_code};

const INFO_EVERY: Duration = Duration::from_secs(5);
/// Wait before asking again when the server had no new live frame.
const LIVE_IDLE: Duration = Duration::from_millis(500);

pub struct Watch {
    input: Entity<TextInput>,
    invalid: bool,
    viewer: Option<Entity<Viewer>>,
    /// The code in the route that could not be read.
    bad_route: bool,
    _input: Subscription,
}

/// The code to watch: a normalized share code, or the debug demo.
fn parse_code(text: &str) -> Option<String> {
    #[cfg(debug_assertions)]
    if text.trim().eq_ignore_ascii_case(demo::CODE) {
        return Some(demo::CODE.into());
    }

    normalize_code(text)
}

impl Watch {
    pub fn new(code: Option<String>, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let input = TextInput::new("ABCD-EFGH", cx);
            if let Some(code) = &code {
                let mut input = input;
                input.set_text(code.clone(), cx);
                input
            } else {
                input
            }
        });

        let subscription = cx.subscribe(&input, |this, _, event, cx| {
            if let TextInputEvent::Changed(_) = event
                && this.invalid
            {
                this.invalid = false;
                cx.notify();
            }
        });

        let parsed = code.as_deref().and_then(parse_code);
        let viewer = parsed.map(|code| cx.new(|cx| Viewer::new(code, cx)));

        Self {
            input,
            invalid: false,
            bad_route: code.is_some() && viewer.is_none(),
            viewer,
            _input: subscription,
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let text = self.input.read(cx).text().to_string();

        match parse_code(&text) {
            Some(code) => navigate(Route::WatchCode { code }, cx),
            None => {
                self.invalid = true;
                cx.notify();
            }
        }
    }
}

impl Render for Watch {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entry = ui::card()
            .child(ui::heading("Share code"))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .w(px(220.))
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .border_1()
                            .border_color(rgb(if self.invalid {
                                0xfb7185
                            } else {
                                theme::STD_700
                            }))
                            .bg(rgb(theme::STD_950))
                            .font_family(theme::CODE_FONT)
                            .text_lg()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(|this, _, window, cx| {
                                    window.focus(&this.input.read(cx).focus_handle(cx))
                                }),
                            )
                            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, cx| {
                                if event.keystroke.key == "enter" {
                                    this.submit(cx);
                                }
                            }))
                            .child(self.input.clone()),
                    )
                    .child(ui::button(
                        "watch-submit",
                        "Watch",
                        Tone::Primary,
                        false,
                        false,
                        cx.listener(|this, _, _, cx| this.submit(cx)),
                    )),
            )
            .child(match self.invalid {
                true => ui::status(
                    "Enter the eight-character code from the host. Spaces and dashes are \
                     optional.",
                    true,
                ),
                false => ui::muted("No account needed. Ask the host for their share code."),
            });

        ui::page("Watch a match")
            .max_w_full()
            .child(entry)
            .when(self.bad_route, |page| {
                page.child(ui::status(
                    "Invalid share code. Enter the eight-character code above.",
                    true,
                ))
            })
            .children(self.viewer.clone())
    }
}

pub struct Viewer {
    code: String,
    info: Option<ShareInfo>,
    message: Option<SharedString>,
    live_message: Option<SharedString>,
    /// The code was revoked or does not exist.
    fatal: bool,
    /// Showing the live game rather than the replay.
    follow_live: bool,
    live: Entity<LivegameView>,
    has_live: bool,
    replay: Option<Entity<ReplayView>>,
    _info: Option<Task<()>>,
    _live: Option<Task<()>>,
    _replay_errors: Option<Subscription>,
}

impl Viewer {
    fn new(code: String, cx: &mut Context<Self>) -> Self {
        let live = cx.new(LivegameView::new);

        live.update(cx, |live, cx| {
            live.set_game(
                None,
                Some("Waiting for the host's next frame...".into()),
                cx,
            )
        });

        let mut viewer = Self {
            code,
            info: None,
            message: Some("Loading match...".into()),
            live_message: None,
            fatal: false,
            follow_live: true,
            live,
            has_live: false,
            replay: None,
            _info: None,
            _live: None,
            _replay_errors: None,
        };

        viewer.start(cx);
        viewer
    }

    fn ended(&self) -> bool {
        self.info
            .as_ref()
            .is_some_and(|info| matches!(info.status, ShareStatus::Ended { .. }))
    }

    /// (Re)starts the info refresh. The live poll starts once the match is known to be live.
    fn start(&mut self, cx: &mut Context<Self>) {
        self.fatal = false;
        let code = self.code.clone();

        self._info = Some(cx.spawn(async move |this, cx| {
            loop {
                let result = info(&code, cx).await;

                let keep = this
                    .update(cx, |this, cx| {
                        match result {
                            Ok(info) => {
                                this.message = None;
                                this.set_info(info, cx);
                            }
                            Err(error) => {
                                let revoked = error.starts_with("This match")
                                    || error.starts_with("Code not found")
                                    || error.starts_with("404")
                                    || error.starts_with("410");

                                this.message = Some(error.into());

                                if revoked {
                                    this.close(cx);
                                }
                            }
                        }

                        cx.notify();
                        !this.fatal && this.message.is_none()
                    })
                    .unwrap_or(false);

                if !keep {
                    break;
                }

                cx.background_executor().timer(INFO_EVERY).await;
            }
        }));
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        self.fatal = true;
        self.info = None;
        self.replay = None;
        self._live = None;
        self.live
            .update(cx, |live, cx| live.set_game(None, None, cx));
    }

    fn set_info(&mut self, info: ShareInfo, cx: &mut Context<Self>) {
        let end = info
            .summary
            .game_length_s
            .map_or(info.summary.last_game_time, |v| v as f32);

        match &self.replay {
            Some(replay) => {
                let chunks = info.chunks.clone();
                replay.update(cx, |replay, cx| replay.set_chunks(chunks, end, cx));
            }
            None => {
                let source = ReplaySource::Capability(self.code.clone());
                let chunks = info.chunks.clone();
                let replay = cx.new(|cx| ReplayView::new(source, chunks, end, cx));

                self._replay_errors =
                    Some(cx.subscribe(&replay, |this, _, AccessError(error), cx| {
                        this.message = Some(error.clone().into());
                        this.close(cx);
                        cx.notify();
                    }));
                self.replay = Some(replay);
            }
        }

        let live = !matches!(info.status, ShareStatus::Ended { .. });
        self.info = Some(info);

        if live && self._live.is_none() {
            self.poll_live(cx);
        } else if !live {
            self._live = None;
            self.follow_live = false;
        }
    }

    /// Asks for the frame after the last one, calculates it and shows it, until stopped.
    fn poll_live(&mut self, cx: &mut Context<Self>) {
        let code = self.code.clone();
        self.live_message = None;

        self._live = Some(cx.spawn(async move |this, cx| {
            let mut seq = 0u64;

            loop {
                let result = live_frame(&code, seq, cx).await;

                let keep = this
                    .update(cx, |this, cx| match result {
                        Ok(Some((next, game))) => {
                            seq = next;
                            this.has_live = true;
                            this.live_message = None;
                            this.live.update(cx, |live, cx| {
                                live.set_game(Some(Rc::new(game)), None, cx)
                            });
                            cx.notify();
                            Some(false)
                        }
                        Ok(None) => Some(true),
                        Err(error) => {
                            this.live_message = Some(error.into());
                            cx.notify();
                            None
                        }
                    })
                    .ok()
                    .flatten();

                match keep {
                    Some(true) => cx.background_executor().timer(LIVE_IDLE).await,
                    Some(false) => {}
                    None => break,
                }
            }
        }));
    }
}

/// The match info, sorted by chunk time.
async fn info(code: &str, cx: &mut gpui::AsyncApp) -> Result<ShareInfo, String> {
    #[cfg(debug_assertions)]
    if code == demo::CODE {
        return Ok(demo::info());
    }

    let backend = cx
        .update(|cx| Session::backend(cx))
        .ok()
        .flatten()
        .ok_or("The recorder is not running")?;

    let mut info = backend
        .spawn(backend.watch_info(code.to_owned()))
        .await
        .map_err(|e| e.to_string())??;

    if info.frames_format != tutorlolv2_protocol::FRAMES_FORMAT {
        return Err("This replay needs a newer viewer. Update the app and retry.".into());
    }

    info.chunks
        .sort_by(|a, b| a.first_game_time.total_cmp(&b.first_game_time));

    Ok(info)
}

/// The next live frame after `after`, calculated: `None` when there is none yet.
async fn live_frame(
    code: &str,
    after: u64,
    cx: &mut gpui::AsyncApp,
) -> Result<Option<(u64, Game)>, String> {
    #[cfg(debug_assertions)]
    if code == demo::CODE {
        return Err("The demo has no live stream; use Replay.".into());
    }

    let backend = cx
        .update(|cx| Session::backend(cx))
        .ok()
        .flatten()
        .ok_or("The recorder is not running")?;

    let response = backend
        .spawn(backend.watch_poll(code.to_owned(), after))
        .await
        .map_err(|e| e.to_string())??;

    cx.background_spawn(async move {
        let Some(frame) = response.decode().map_err(|e| e.to_string())? else {
            return Ok(None);
        };

        if frame.seq <= after {
            return Err("The live stream returned an old frame. Retry to reconnect.".into());
        }

        let game = Game::from_frame(&frame.frame).map_err(|e| e.to_string())?;
        Ok(Some((frame.seq, game)))
    })
    .await
}

impl Render for Viewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ended = self.ended();
        let live_available = !ended && self.info.is_some();
        let viewing_live = self.follow_live && live_available;

        let status = self.info.as_ref().map(|info| match &info.status {
            ShareStatus::Live { .. } if viewing_live => ("● LIVE", 0xfb7185),
            ShareStatus::Live { .. } => ("Viewing replay · live continues", theme::SKY_300),
            ShareStatus::HostDisconnected => ("Waiting for host", 0xfbbf24),
            ShareStatus::Ended { .. } => ("Match ended · replay", theme::STD_400),
        });

        let header = ui::card()
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .font_family(theme::CODE_FONT)
                            .font_weight(FontWeight::BOLD)
                            .text_xl()
                            .text_color(rgb(theme::SKY_300))
                            .child(display_code(&self.code)),
                    )
                    .children(self.info.as_ref().map(|info| {
                        div().text_color(rgb(theme::STD_200)).child(format!(
                            "{} · {}",
                            info.summary.champion, info.summary.active_riot_id
                        ))
                    }))
                    .children(status.map(|(text, color)| {
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(color))
                            .child(text)
                    })),
            )
            .when(live_available, |card| {
                card.child(
                    ui::actions()
                        .child(ui::button(
                            "watch-live",
                            "Live",
                            Tone::Secondary,
                            false,
                            viewing_live,
                            cx.listener(|this, _, _, cx| {
                                this.follow_live = true;
                                cx.notify();
                            }),
                        ))
                        .child(ui::button(
                            "watch-replay",
                            "Replay",
                            Tone::Secondary,
                            false,
                            !viewing_live,
                            cx.listener(|this, _, _, cx| {
                                this.follow_live = false;
                                cx.notify();
                            }),
                        )),
                )
            })
            .children(self.message.clone().map(|message| {
                ui::actions()
                    .child(ui::status(message, self.fatal))
                    .child(ui::button(
                        "watch-retry",
                        "Retry",
                        Tone::Secondary,
                        false,
                        false,
                        cx.listener(|this, _, _, cx| {
                            this.message = Some("Loading match...".into());
                            this.start(cx);
                            cx.notify();
                        }),
                    ))
            }))
            .children(
                self.live_message
                    .clone()
                    .filter(|_| viewing_live)
                    .map(|message| {
                        ui::actions()
                            .child(ui::status(message, false))
                            .child(ui::button(
                                "watch-live-retry",
                                "Retry",
                                Tone::Secondary,
                                false,
                                false,
                                cx.listener(|this, _, _, cx| {
                                    this.poll_live(cx);
                                    cx.notify();
                                }),
                            ))
                    }),
            );

        let body = match (self.fatal, viewing_live) {
            (true, _) => None,
            (false, true) => Some(self.live.clone().into_any_element()),
            (false, false) => self.replay.clone().map(IntoElement::into_any_element),
        };

        div().flex().flex_col().gap_4().child(header).children(body)
    }
}

/// `DEMODEMO` in debug builds: a made-up share of three chunks of the test fixture, with a
/// gap between the second and the third, to try the replay viewer without a server.
#[cfg(debug_assertions)]
pub mod demo {
    use crate::backend::BINCODE_CONFIG;
    use tutorlolv2::riot::{FrameChunk, RiotRealtime};
    use tutorlolv2_protocol::{
        ChunkInfo, FRAMES_FORMAT, GameMode, RecordingSummary, ShareInfo, ShareMode, ShareStatus,
    };

    pub const CODE: &str = "DEMODEMO";

    /// `(first game time, frames)` of each chunk, one frame per second.
    const CHUNKS: [(f32, u16); 3] = [(600., 60), (660., 60), (780., 60)];

    fn fixture() -> RiotRealtime<'static> {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/example.bincode"
        ));
        bincode::borrow_decode_from_slice(bytes, BINCODE_CONFIG)
            .expect("fixture decodes")
            .0
    }

    pub fn info() -> ShareInfo {
        let frame = fixture();
        let chunks = CHUNKS
            .iter()
            .enumerate()
            .map(|(seq, &(first, count))| ChunkInfo {
                seq: seq as u32,
                first_game_time: first,
                last_game_time: first + f32::from(count - 1),
                snapshot_count: count,
                frames_format: FRAMES_FORMAT,
            })
            .collect();

        ShareInfo {
            mode: ShareMode::Public,
            status: ShareStatus::Ended { result: None },
            summary: RecordingSummary {
                id: [0; 16],
                champion: frame.active_player.riot_id.to_owned(),
                game_mode: GameMode::Classic,
                queue_id: None,
                active_riot_id: frame.active_player.riot_id.to_owned(),
                started_at_ms: 0,
                first_game_time: 600.,
                last_game_time: 839.,
                game_length_s: Some(840),
                result: None,
                is_custom: Some(false),
                is_practice: false,
            },
            chunks,
            frames_format: FRAMES_FORMAT,
        }
    }

    /// The bytes of chunk `seq`: the fixture frame at each second, with growing game time.
    pub fn frames(seq: u32) -> Result<Vec<u8>, String> {
        let &(first, count) = CHUNKS.get(seq as usize).ok_or("Code not found")?;
        let frame = fixture();
        let frames = (0..count)
            .map(|i| {
                let mut frame = frame.clone();
                frame.game_data.game_time = first + f32::from(i);
                (i64::from(i), frame)
            })
            .collect();

        Ok(bincode::encode_to_vec(
            FrameChunk {
                format: FRAMES_FORMAT,
                frames,
            },
            BINCODE_CONFIG,
        )
        .expect("demo chunk encodes"))
    }
}
