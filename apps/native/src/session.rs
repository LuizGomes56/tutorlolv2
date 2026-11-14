//! The UI side of the backend: one [`Session`] entity, shared by every page.
//!
//! It starts the [`Backend`], drains its events into plain fields and notifies observers, which
//! replaces the webview's `listen("account-changed" | "history-changed" | ...)` subscriptions
//! and the account context provider. Pages read it with [`Session::global`] and call the
//! backend with [`Session::backend`].
use crate::backend::{Backend, BackendEvent};
use gpui::{App, AppContext, Context, Entity, Global, SharedString};
use protocol::{AccountState, RecordingState};
use std::sync::Arc;

pub struct Session {
    backend: Option<Arc<Backend>>,
    /// Why the backend could not start (its runtime or data folder).
    pub start_error: Option<SharedString>,
    /// `None` until the first account check returns.
    pub account: Option<AccountState>,
    /// A sign-in or sign-out request is running.
    pub account_busy: bool,
    pub recording: Option<RecordingState>,
    /// Changes whenever the backend reports that the history changed.
    pub history_revision: u64,
}

struct GlobalSession(Entity<Session>);

impl Global for GlobalSession {}

impl Session {
    /// Starts the backend and installs the session. Call once, at startup.
    pub fn init(cx: &mut App) {
        let (backend, receiver, start_error) = match Backend::start() {
            Ok((backend, receiver)) => (Some(backend), Some(receiver), None),
            Err(error) => (
                None,
                None,
                Some(format!("The recorder could not start: {error}").into()),
            ),
        };

        let recording = backend.as_ref().map(|backend| backend.recording_state());
        let session = cx.new(|_| Self {
            backend: backend.clone(),
            start_error,
            account: None,
            account_busy: false,
            recording,
            history_revision: 0,
        });

        if let Some(mut receiver) = receiver {
            let weak = session.downgrade();

            cx.spawn(async move |cx| {
                while let Some(event) = receiver.recv().await {
                    let alive = weak
                        .update(cx, |session, cx| session.apply(event, cx))
                        .is_ok();

                    if !alive {
                        break;
                    }
                }
            })
            .detach();
        }

        if let Some(backend) = backend {
            // Closing the app seals the partial chunk and tries one upload (up to 2 s).
            cx.on_app_quit(move |_| {
                let backend = backend.clone();
                async move { backend.exit() }
            })
            .detach();
        }

        cx.set_global(GlobalSession(session.clone()));
        session.update(cx, |session, cx| session.refresh_account(cx));
    }

    pub fn global(cx: &App) -> Entity<Session> {
        cx.global::<GlobalSession>().0.clone()
    }

    /// The backend, or `None` if it failed to start.
    pub fn backend(cx: &App) -> Option<Arc<Backend>> {
        Self::global(cx).read(cx).backend.clone()
    }

    fn apply(&mut self, event: BackendEvent, cx: &mut Context<Self>) {
        match event {
            BackendEvent::AccountChanged(state) => self.account = Some(state),
            BackendEvent::HistoryChanged => {
                self.history_revision = self.history_revision.wrapping_add(1)
            }
            BackendEvent::RecordingStateChanged(state) => self.recording = Some(state),
            BackendEvent::ShareStateChanged(share) => {
                if let Some(recording) = &mut self.recording {
                    recording.share = share;
                }
            }
        }

        cx.notify();
    }

    /// Asks the backend for the account, which also validates the session with the server.
    pub fn refresh_account(&mut self, cx: &mut Context<Self>) {
        let Some(backend) = self.backend.clone() else {
            return;
        };

        let request = backend.spawn(backend.account());

        cx.spawn(async move |this, cx| {
            let result = request.await;

            this.update(cx, |this, cx| {
                this.set_account_result(result.map_err(|e| e.to_string()), cx)
            })
            .ok();
        })
        .detach();
    }

    pub fn sign_in(&mut self, cx: &mut Context<Self>) {
        self.account_request(cx, |backend| backend.spawn(backend.sign_in()));
    }

    pub fn sign_out(&mut self, cx: &mut Context<Self>) {
        self.account_request(cx, |backend| backend.spawn(backend.sign_out()));
    }

    fn account_request(
        &mut self,
        cx: &mut Context<Self>,
        start: impl FnOnce(
            &Backend,
        )
            -> tokio::task::JoinHandle<Result<AccountState, protocol::AccountError>>,
    ) {
        let Some(backend) = self.backend.clone() else {
            return;
        };

        if self.account_busy {
            return;
        }

        self.account_busy = true;
        cx.notify();

        let request = start(&backend);

        cx.spawn(async move |this, cx| {
            let result = request.await;

            this.update(cx, |this, cx| {
                this.account_busy = false;
                this.set_account_result(result.map_err(|e| e.to_string()), cx);
            })
            .ok();
        })
        .detach();
    }

    fn set_account_result(
        &mut self,
        result: Result<Result<AccountState, protocol::AccountError>, String>,
        cx: &mut Context<Self>,
    ) {
        match result {
            Ok(Ok(state)) => self.account = Some(state),
            Ok(Err(error)) => {
                let mut state = self.account.clone().unwrap_or(AccountState {
                    email: None,
                    signing_in: false,
                    last_error: None,
                });

                state.last_error = Some(error.message);
                self.account = Some(state);
            }
            Err(join_error) => {
                log::error!("Account request failed: {join_error}");
            }
        }

        cx.notify();
    }
}
