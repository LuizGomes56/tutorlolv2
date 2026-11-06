//! The Livegame page: the web app's `livegame/page.rs` (`Livegame`).
//!
//! The recorder polls port 2999 once per second and keeps the latest Riot JSON. This page reads
//! it once per second, calculates it on a background thread and shows the result in a
//! [`LivegameView`]. A newer snapshot is only calculated after the previous one finished.
pub mod game;
mod sharing;
pub mod view;

use crate::session::Session;
use game::Game;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Task,
    Window, div,
};
use sharing::SharingBar;
use std::{rc::Rc, sync::Arc, time::Duration};
pub use view::LivegameView;

const POLL: Duration = Duration::from_secs(1);

pub struct Livegame {
    sharing: Entity<SharingBar>,
    view: Entity<LivegameView>,
    _poll: Task<()>,
}

impl Livegame {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let view = cx.new(LivegameView::new);
        let sharing = cx.new(SharingBar::new);

        let poll = poll(cx, |this: &mut Self, game, error, cx| {
            this.view
                .update(cx, |view, cx| view.set_game(game, error, cx));
        });

        Self {
            sharing,
            view,
            _poll: poll,
        }
    }
}

/// Reads the recorder's latest snapshot once per second, calculates it on a background
/// thread and gives the result to `apply`. A snapshot is calculated once; `apply` gets
/// `None` and a message while no game runs. Used by the Livegame page and the overlay.
pub fn poll<T: 'static>(
    cx: &mut Context<T>,
    apply: impl Fn(&mut T, Option<Rc<Game>>, Option<SharedString>, &mut Context<T>) + 'static,
) -> Task<()> {
    cx.spawn(async move |this, cx| {
        let mut previous: Option<Arc<[u8]>> = None;
        let mut idle = false;

        loop {
            let Ok(snapshot) = this.update(cx, |_, cx| {
                Session::backend(cx).and_then(|backend| backend.live_game())
            }) else {
                return;
            };

            // Debug builds can show the test fixture instead of a running game.
            #[cfg(debug_assertions)]
            if snapshot.is_none() && previous.is_none() && demo_requested() {
                let result = cx.background_spawn(async { demo_game() }).await;
                let _ = this.update(cx, |this, cx| match result {
                    Ok(game) => apply(this, Some(Rc::new(game)), None, cx),
                    Err(error) => apply(this, None, Some(error.to_string().into()), cx),
                });
                return;
            }

            match snapshot {
                // The same snapshot as last time: nothing to recalculate.
                Some(bytes)
                    if previous
                        .as_ref()
                        .is_some_and(|old| Arc::ptr_eq(old, &bytes)) => {}
                Some(bytes) => {
                    previous = Some(bytes.clone());
                    idle = false;

                    let result = cx
                        .background_spawn(async move { Game::from_json(&bytes) })
                        .await;

                    let alive = this
                        .update(cx, |this, cx| match result {
                            Ok(game) => apply(this, Some(Rc::new(game)), None, cx),
                            Err(error) => apply(this, None, Some(error.to_string().into()), cx),
                        })
                        .is_ok();

                    if !alive {
                        return;
                    }
                }
                None if !idle => {
                    previous = None;
                    idle = true;

                    let alive = this
                        .update(cx, |this, cx| {
                            apply(this, None, Some("Waiting for a game...".into()), cx)
                        })
                        .is_ok();

                    if !alive {
                        return;
                    }
                }
                None => {}
            }

            cx.background_executor().timer(POLL).await;
        }
    })
}

/// `TUTORLOLV2_DEMO_GAME=1`: show `tests/fixtures/example.bincode` when no game is running.
#[cfg(debug_assertions)]
fn demo_requested() -> bool {
    std::env::var_os("TUTORLOLV2_DEMO_GAME").is_some_and(|value| value == "1")
}

#[cfg(debug_assertions)]
fn demo_game() -> Result<Game, game::Error> {
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/example.bincode"
    ));
    let (frame, _): (tutorlolv2::riot::RiotRealtime<'_>, _) =
        bincode::borrow_decode_from_slice(bytes, crate::backend::BINCODE_CONFIG)?;

    Game::from_frame(&frame)
}

impl Render for Livegame {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .pb_96()
            .child(div().p_4().pb_0().child(self.sharing.clone()))
            .child(self.view.clone())
    }
}
