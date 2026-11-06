//! The desktop backend: recording, upload, history, sharing and the account.
//!
//! Copied from `tutorlolv2_desktop_app/src-tauri` (the modules had no Tauri imports apart from
//! a few seams) and adapted:
//!
//! - Tauri's runtime is replaced by our own multi-threaded tokio runtime (`Backend::start`).
//! - `app.emit(...)` events become [`BackendEvent`]s sent over a channel. The UI drains it in a
//!   gpui task (see `src/session.rs`).
//! - The 18 IPC commands become the methods below. The webview label checks are gone: only this
//!   process can call them.
//! - Data lives in `%APPDATA%\tutorlolv2`, apart from the desktop app's
//!   `%LOCALAPPDATA%\com.tutorlolv2`: the two apps never share state.
//!
//! The UI calls these methods with `Backend::spawn`, which runs the future on the runtime and
//! returns a handle that gpui's executor can await.
pub mod account;
pub mod delivery;
pub mod history;
pub mod lcu;
pub mod live_client;
pub mod recorder;
pub mod recorder_task;
pub mod sharing;
pub mod spool;
pub mod store;
pub mod uploader;

use delivery::Delivery;
use recorder_task::{LatestSnapshot, RecorderHandle};
use std::{future::Future, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tutorlolv2_protocol::{
    AccountError, AccountState, HistoryDetails, HistoryError, HistoryErrorKind, HistoryList,
    HistoryShareMode, RecordingState, ShareMode, ShareState,
};

/// Shared encoding for persisted records and server requests.
pub const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

/// How long closing the app may take to save the recording in progress.
const EXIT_FLUSH: Duration = Duration::from_secs(2);

/// What the webview used to receive as events.
#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BackendEvent {
    AccountChanged(AccountState),
    HistoryChanged,
    RecordingStateChanged(RecordingState),
    ShareStateChanged(ShareState),
}

/// Sends [`BackendEvent`]s to the UI. Dropped events (no UI listening) are fine.
#[derive(Clone)]
pub struct Events(mpsc::UnboundedSender<BackendEvent>);

impl Events {
    pub fn emit(&self, event: BackendEvent) {
        let _ = self.0.send(event);
    }
}

pub struct Backend {
    runtime: tokio::runtime::Runtime,
    recorder: RecorderHandle,
    latest: LatestSnapshot,
    events: Events,
}

/// `%APPDATA%\tutorlolv2`, or the temporary folder when `APPDATA` is not set.
pub fn data_dir() -> PathBuf {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("tutorlolv2")
}

impl Backend {
    /// Starts the runtime and the recorder, upload, sharing and history loops.
    pub fn start() -> std::io::Result<(Arc<Self>, mpsc::UnboundedReceiver<BackendEvent>)> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("tutorlolv2-backend")
            .enable_all()
            .build()?;

        let (sender, receiver) = mpsc::unbounded_channel();
        let events = Events(sender);
        let latest = LatestSnapshot::default();

        let recorder = {
            let _context = runtime.enter();
            recorder_task::spawn(data_dir(), events.clone(), latest.clone())
        };

        let backend = Arc::new(Self {
            runtime,
            recorder,
            latest,
            events,
        });

        Ok((backend, receiver))
    }

    /// Runs `future` on the backend runtime. The handle can be awaited from any executor.
    pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    fn shared(&self) -> Arc<Delivery> {
        self.recorder.shared.clone()
    }

    /// Seals the partial chunk and tries one upload pass, waiting at most two seconds.
    pub fn exit(&self) {
        self.recorder.exit(EXIT_FLUSH);
    }

    /// The latest live game snapshot (port 2999), polled once per second by the recorder.
    pub fn live_game(&self) -> Option<Arc<[u8]>> {
        self.latest.get()
    }

    pub fn recording_state(&self) -> RecordingState {
        self.recorder.shared.recording_state()
    }

    pub fn account(&self) -> impl Future<Output = Result<AccountState, AccountError>> + use<> {
        let shared = self.shared();
        async move { shared.account_state().await }
    }

    pub fn sign_in(&self) -> impl Future<Output = Result<AccountState, AccountError>> + use<> {
        let shared = self.shared();
        let events = self.events.clone();
        async move { shared.sign_in(&events).await }
    }

    pub fn sign_out(&self) -> impl Future<Output = Result<AccountState, AccountError>> + use<> {
        let shared = self.shared();
        let events = self.events.clone();

        async move {
            let result = shared.sign_out().await?;
            events.emit(BackendEvent::AccountChanged(result.clone()));
            events.emit(BackendEvent::HistoryChanged);
            Ok(result)
        }
    }

    pub fn set_share_mode(
        &self,
        mode: ShareMode,
    ) -> impl Future<Output = Result<ShareState, String>> + use<> {
        let shared = self.shared();
        let events = self.events.clone();

        async move {
            let result = shared.set_share_mode(mode).await;
            let current = shared.recording_state();

            events.emit(BackendEvent::ShareStateChanged(current.share.clone()));
            events.emit(BackendEvent::RecordingStateChanged(current));
            result
        }
    }

    /// The cached page, then a refresh from the server in the background.
    pub fn list_history(
        &self,
        offset: u32,
        limit: u32,
    ) -> impl Future<Output = Result<HistoryList, HistoryError>> + use<> {
        let shared = self.shared();
        let events = self.events.clone();

        async move {
            if !(1..=100).contains(&limit) {
                return Err(history::error(
                    HistoryErrorKind::InvalidData,
                    "History page size must be between 1 and 100",
                    false,
                ));
            }

            let cached = shared.cached_history(offset, limit).await?;
            queue_history(shared, events, false);
            Ok(cached)
        }
    }

    pub fn history_details(
        &self,
        recording_id: String,
    ) -> impl Future<Output = Result<HistoryDetails, HistoryError>> + use<> {
        let shared = self.shared();

        async move {
            let id = history::parse_id(&recording_id)?;
            let api = shared.api.read().await.clone();
            let details = api.history_details(id).await?;

            shared.history.details(api.check_owner()?, details)
        }
    }

    pub fn history_frames(
        &self,
        recording_id: String,
        seq: u32,
    ) -> impl Future<Output = Result<Vec<u8>, HistoryError>> + use<> {
        let shared = self.shared();

        async move {
            let id = history::parse_id(&recording_id)?;
            let api = shared.api.read().await.clone();
            let bytes = api.history_frames(id, seq).await?;

            shared.history.check_device(api.check_owner()?)?;
            Ok(bytes)
        }
    }

    pub fn set_history_share_mode(
        &self,
        recording_id: String,
        mode: HistoryShareMode,
    ) -> impl Future<Output = Result<ShareState, HistoryError>> + use<> {
        let shared = self.shared();
        let events = self.events.clone();

        async move {
            let id = history::parse_id(&recording_id)?;
            let share = shared.set_history_share_mode(id, mode).await?;

            events.emit(BackendEvent::HistoryChanged);
            Ok(share)
        }
    }

    pub fn hide_history_entry(&self, recording_id: &str) -> Result<(), HistoryError> {
        self.recorder
            .shared
            .history
            .hide(history::parse_id(recording_id)?)?;

        self.events.emit(BackendEvent::HistoryChanged);
        Ok(())
    }

    /// Rebuilds the history index from the server.
    pub fn repair_history(&self) {
        let _context = self.runtime.enter();
        queue_history(self.shared(), self.events.clone(), true);
    }
}

impl Backend {
    /// A shared match by its code (the Watch page).
    pub fn watch_info(
        &self,
        code: String,
    ) -> impl Future<Output = Result<tutorlolv2_protocol::ShareInfo, String>> + use<> {
        async move { uploader::ApiClient::new(None).share_info(&code).await }
    }

    pub fn watch_frames(
        &self,
        code: String,
        seq: u32,
    ) -> impl Future<Output = Result<Vec<u8>, String>> + use<> {
        async move {
            uploader::ApiClient::new(None)
                .share_frames(&code, seq)
                .await
        }
    }

    pub fn watch_poll(
        &self,
        code: String,
        after: u64,
    ) -> impl Future<
        Output = Result<tutorlolv2_api::ResponseBuffer<tutorlolv2_api::PollLive>, String>,
    > + use<> {
        async move { uploader::ApiClient::new(None).poll_live(&code, after).await }
    }
}

/// Refreshes the history from the server unless a refresh is already running.
fn queue_history(shared: Arc<Delivery>, events: Events, repair: bool) {
    if !shared.history.begin(repair) {
        return;
    }

    tokio::spawn(async move {
        let api = shared.api.read().await.clone();
        shared.history.refresh(api.clone(), repair).await;

        if api.account_invalid() {
            let _ = shared.account_state().await;
        }

        events.emit(BackendEvent::HistoryChanged);
    });
}
