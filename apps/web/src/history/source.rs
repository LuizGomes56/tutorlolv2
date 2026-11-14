use ::api::{
    Endpoint, GetRecording, GetRecordingPath, GoogleStart, GoogleStartQuery, ListRecordings,
    NoBody, NoHeaders, NoPath, NoQuery, OwnerFrames, OwnerFramesPath, Pagination, ResponseBuffer,
    SetHistoryShare, SetHistorySharePath, Uuid,
};
use protocol::*;
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    pin::Pin,
    rc::{Rc, Weak},
    task::Poll,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, ArrayBuffer, Function, Promise, Reflect, Uint8Array};

use crate::utils::api::FetchError;

#[derive(Clone, Copy, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    ListHistory,
    GetHistoryDetails,
    GetHistoryFrames,
    SetHistoryShareMode,
    HideHistoryEntry,
    RepairHistory,
    GetAccount,
    SignIn,
    SignOut,
}

#[derive(Clone, Copy, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Event {
    HistoryChanged,
}

fn error(message: &str) -> HistoryError {
    HistoryError {
        kind: HistoryErrorKind::Other,
        message: message.into(),
        retryable: true,
    }
}

pub fn member(value: &JsValue, key: &str) -> Result<JsValue, HistoryError> {
    Reflect::get(value, &key.into()).map_err(|_| error("Desktop connection unavailable"))
}

pub fn tauri() -> Result<JsValue, HistoryError> {
    member(
        &web_sys::window()
            .ok_or_else(|| error("Window unavailable"))?
            .into(),
        "__TAURI__",
    )
}

fn decode<T: serde::de::DeserializeOwned>(value: &JsValue) -> Result<T, HistoryError> {
    let text = js_sys::JSON::stringify(value)
        .map_err(|_| error("Invalid desktop response"))?
        .as_string()
        .ok_or_else(|| error("Invalid desktop response"))?;

    serde_json::from_str(&text).map_err(|_| error("Invalid desktop response"))
}

impl Command {
    pub async fn invoke<T: serde::de::DeserializeOwned>(
        self,
        args: serde_json::Value,
    ) -> Result<T, HistoryError> {
        decode(&self.invoke_raw(args).await?)
    }

    async fn invoke_raw(self, args: serde_json::Value) -> Result<JsValue, HistoryError> {
        let core = member(&tauri()?, "core")?;
        let invoke: Function = member(&core, "invoke")?
            .dyn_into()
            .map_err(|_| error("Desktop connection unavailable"))?;

        let args = js_sys::JSON::parse(&args.to_string()).map_err(|_| error("Invalid request"))?;
        let result = invoke
            .call2(&core, &JsValue::from_str(self.into()), &args)
            .map_err(|_| error("Desktop request failed"))?;

        let value = JsFuture::from(Promise::resolve(&result))
            .await
            .map_err(|value| {
                decode::<HistoryError>(&value).unwrap_or_else(|_| {
                    decode::<AccountError>(&value)
                        .map(|account| HistoryError {
                            kind: HistoryErrorKind::Other,
                            message: account.message,
                            retryable: account.retryable,
                        })
                        .unwrap_or_else(|_| error("Desktop request failed. Retry."))
                })
            })?;

        Ok(value)
    }
}

pub trait HistorySource {
    async fn list(offset: u32) -> Result<HistoryList, HistoryError>;
    async fn details(id: &str) -> Result<HistoryDetails, HistoryError>;
    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError>;
    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError>;
    async fn hide(id: &str) -> Result<(), HistoryError>;
    async fn repair() -> Result<(), HistoryError>;
}

pub struct IpcHistory;

impl HistorySource for IpcHistory {
    async fn list(offset: u32) -> Result<HistoryList, HistoryError> {
        Command::ListHistory
            .invoke(serde_json::json!({
                "offset": offset,
                "limit": 50
            }))
            .await
    }

    async fn details(id: &str) -> Result<HistoryDetails, HistoryError> {
        Command::GetHistoryDetails
            .invoke(serde_json::json!({
                "recordingId": id
            }))
            .await
    }

    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
        owner_frames(id, seq).await
    }

    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError> {
        Command::SetHistoryShareMode
            .invoke(serde_json::json!({
                "recordingId": id,
                "mode": mode
            }))
            .await
    }

    async fn hide(id: &str) -> Result<(), HistoryError> {
        Command::HideHistoryEntry
            .invoke(serde_json::json!({
                "recordingId": id
            }))
            .await
    }

    async fn repair() -> Result<(), HistoryError> {
        Command::RepairHistory.invoke(serde_json::json!({})).await
    }
}

/// Preserve UUID identity rather than treating a local identity as a server ID.
pub fn uuid(id: Id16) -> String {
    let s = id.iter().map(|v| format!("{v:02x}")).collect::<String>();

    format!(
        "{}-{}-{}-{}-{}",
        &s[..8],
        &s[8..12],
        &s[12..16],
        &s[16..20],
        &s[20..]
    )
}

pub fn identity(entry: &HistoryEntry) -> String {
    entry
        .recording_id
        .map(uuid)
        .unwrap_or_else(|| format!("local:{}", entry.local_id.map(uuid).unwrap_or_default()))
}

// Native requests cannot be aborted, so concurrent consumers share the in-flight
// transport only. Completed buffers are never kept in this registry.
struct FrameRequest {
    result: Option<Result<Vec<u8>, HistoryError>>,
    waiters: Vec<std::task::Waker>,
}

thread_local! {
    static FRAME_REQUESTS: RefCell<BTreeMap<(u64, String, u32), Weak<RefCell<FrameRequest>>>> = const {
        RefCell::new(BTreeMap::new())
    };
}

struct FrameFuture(Rc<RefCell<FrameRequest>>);

impl Future for FrameFuture {
    type Output = Result<Vec<u8>, HistoryError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.borrow_mut();

        if let Some(result) = &state.result {
            return Poll::Ready(result.clone());
        }

        if !state.waiters.iter().any(|w| w.will_wake(cx.waker())) {
            state.waiters.push(cx.waker().clone());
        }

        Poll::Pending
    }
}

async fn owner_frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
    let key = (OWNER_EPOCH.with(|v| v.get()), id.to_owned(), seq);

    let (request, start) = FRAME_REQUESTS.with(|requests| {
        let mut requests = requests.borrow_mut();

        if let Some(request) = requests.get(&key).and_then(Weak::upgrade) {
            (request, false)
        } else {
            let request = Rc::new(RefCell::new(FrameRequest {
                result: None,
                waiters: vec![],
            }));

            requests.insert(key.clone(), Rc::downgrade(&request));
            (request, true)
        }
    });

    if start {
        let request = request.clone();

        yew::platform::spawn_local(async move {
            let result = async {
                let value = Command::GetHistoryFrames
                    .invoke_raw(serde_json::json!({
                        "recordingId": key.1,
                        "seq": seq
                    }))
                    .await?;

                if value.is_instance_of::<ArrayBuffer>() || value.is_instance_of::<Uint8Array>() {
                    let bytes = Uint8Array::new(&value);

                    if bytes.length() > 64 * 1024 * 1024 {
                        return Err(error("Replay chunk exceeds the viewer limit"));
                    }

                    Ok(bytes.to_vec())
                } else {
                    decode::<Vec<u8>>(&value)
                }
            }
            .await;

            let waiters = {
                let mut state = request.borrow_mut();

                state.result = Some(result);
                core::mem::take(&mut state.waiters)
            };

            FRAME_REQUESTS.with(|requests| {
                requests.borrow_mut().remove(&key);
            });

            for waiter in waiters {
                waiter.wake();
            }
        });
    }

    FrameFuture(request).await
}

thread_local! {
    static OWNER_EPOCH: Cell<u64> = const { Cell::new(0) };
    static WEB_CURSORS: RefCell<BTreeMap<u32, Option<Id16>>> = RefCell::new(BTreeMap::from([(0, None)]));
    static WEB_HIDDEN: RefCell<BTreeSet<String>> = const {
        RefCell::new(BTreeSet::new())
    };
}

pub fn clear_web_history() {
    OWNER_EPOCH.with(|v| v.set(v.get().wrapping_add(1)));
    FRAME_REQUESTS.with(|v| v.borrow_mut().clear());
    WEB_CURSORS.with(|v| *v.borrow_mut() = BTreeMap::from([(0, None)]));
    WEB_HIDDEN.with(|v| v.borrow_mut().clear());
}

impl HttpHistory {
    fn base() -> String {
        option_env!("TUTORLOLV2_API_URL")
            .or(option_env!("TUTORLOL_API_URL"))
            .unwrap_or("/api")
            .trim_end_matches('/')
            .trim_end_matches("/games")
            .into()
    }

    pub fn sign_in_url() -> String {
        GoogleStart::url(
            &Self::base(),
            &NoPath,
            &GoogleStartQuery {
                target: "web".into(),
                return_to: Some("/history".into()),
                port: None,
                app_state: None,
            },
        )
        .expect("sign-in query is serializable")
    }

    pub async fn request<E: Endpoint>(
        path: &E::Path,
        query: &E::Query,
        headers: &E::Headers,
        body: &E::Body<'_>,
    ) -> Result<ResponseBuffer<E>, HistoryError> {
        let epoch = OWNER_EPOCH.with(|v| v.get());
        let response = crate::utils::api::WebClient::new(Self::base())
            .send::<E>(path, query, headers, body, None)
            .await
            .map_err(FetchError::history)?;

        if OWNER_EPOCH.with(|v| v.get()) != epoch {
            return Err(error("Account changed. Refresh history."));
        }

        Ok(response)
    }

    pub fn decode<E: Endpoint>(
        response: &ResponseBuffer<E>,
    ) -> Result<E::Output<'_>, HistoryError> {
        if response.status() == 401 {
            crate::account::invalidate();
        }

        response
            .decode()
            .map_err(|e| FetchError::response(response.status(), e).history())
    }
}

const fn web_entry(summary: HistorySummary, share: ShareState) -> HistoryEntry {
    HistoryEntry {
        local_id: None,
        recording_id: Some(summary.recording.id),
        summary: Some(summary),
        share,
        pending_chunks: 0,
        pending_control: false,
        lost_chunks: 0,
        last_error: None,
        hidden: false,
    }
}

pub struct HttpHistory;

impl HistorySource for HttpHistory {
    async fn list(offset: u32) -> Result<HistoryList, HistoryError> {
        let epoch = OWNER_EPOCH.with(|v| v.get());
        let before = WEB_CURSORS
            .with(|v| v.borrow().get(&offset).copied())
            .ok_or_else(|| error("Refresh history to restart pagination."))?;

        let response = Self::request::<ListRecordings>(
            &NoPath,
            &Pagination {
                before: before.map(Uuid::from_bytes),
                limit: Some(50),
            },
            &NoHeaders,
            &NoBody,
        )
        .await?;

        let page = Self::decode(&response)?;

        if OWNER_EPOCH.with(|v| v.get()) != epoch {
            return Err(error("Account changed. Refresh history."));
        }

        WEB_CURSORS.with(|v| {
            v.borrow_mut().insert(offset + 50, page.next_before);
        });

        let entries = page
            .items
            .into_iter()
            .map(|v| web_entry(v.summary, v.share))
            .filter(|v| WEB_HIDDEN.with(|h| !h.borrow().contains(&identity(v))))
            .collect();

        Ok(HistoryList {
            entries,
            has_more: page.next_before.is_some(),
            refreshing: false,
            refresh_error: None,
        })
    }

    async fn details(id: &str) -> Result<HistoryDetails, HistoryError> {
        let id = Uuid::try_parse(id).map_err(|_| error("Invalid recording ID"))?;
        let response =
            Self::request::<GetRecording>(&GetRecordingPath { id }, &NoQuery, &NoHeaders, &NoBody)
                .await?;

        let recording = Self::decode(&response)?;

        if recording.frames_format != FRAMES_FORMAT {
            return Err(error("This replay needs a newer viewer."));
        }

        let entry = web_entry(recording.summary.clone(), recording.share.clone());
        Ok(HistoryDetails { recording, entry })
    }

    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
        let id = Uuid::try_parse(id).map_err(|_| error("Invalid recording ID"))?;
        let response = Self::request::<OwnerFrames>(
            &OwnerFramesPath { id, seq },
            &NoQuery,
            &NoHeaders,
            &NoBody,
        )
        .await?;

        Self::decode(&response)?;
        Ok(response.into_bytes())
    }

    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError> {
        let id = Uuid::try_parse(id).map_err(|_| error("Invalid recording ID"))?;
        let response = Self::request::<SetHistoryShare>(
            &SetHistorySharePath { id },
            &NoQuery,
            &NoHeaders,
            &SetShareMode { mode: mode.into() },
        )
        .await?;

        Self::decode(&response)
    }

    async fn hide(id: &str) -> Result<(), HistoryError> {
        WEB_HIDDEN.with(|v| {
            v.borrow_mut().insert(id.into());
        });

        Ok(())
    }

    async fn repair() -> Result<(), HistoryError> {
        Err(error("Index repair is available in the desktop app."))
    }
}

/// Build-selected owner transport. Public capability Watch remains independent.
pub struct OwnerHistory;

impl HistorySource for OwnerHistory {
    async fn list(offset: u32) -> Result<HistoryList, HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::list(offset).await
        } else {
            HttpHistory::list(offset).await
        }
    }

    async fn details(id: &str) -> Result<HistoryDetails, HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::details(id).await
        } else {
            HttpHistory::details(id).await
        }
    }

    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::frames(id, seq).await
        } else {
            HttpHistory::frames(id, seq).await
        }
    }

    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::share(id, mode).await
        } else {
            HttpHistory::share(id, mode).await
        }
    }

    async fn hide(id: &str) -> Result<(), HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::hide(id).await
        } else {
            HttpHistory::hide(id).await
        }
    }

    async fn repair() -> Result<(), HistoryError> {
        if cfg!(feature = "tauri") {
            IpcHistory::repair().await
        } else {
            HttpHistory::repair().await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_commands_are_typed() {
        let name: &str = Command::GetHistoryFrames.into();
        assert_eq!(name, "get_history_frames");
        let event: &str = Event::HistoryChanged.into();
        assert_eq!(event, "history-changed");
    }
    #[test]
    fn owner_change_resets_pagination_and_hidden_rows() {
        let before = OWNER_EPOCH.with(|v| v.get());
        WEB_CURSORS.with(|v| {
            v.borrow_mut().insert(50, Some([7; 16]));
        });
        WEB_HIDDEN.with(|v| {
            v.borrow_mut().insert("old-owner".into());
        });
        clear_web_history();
        assert_eq!(OWNER_EPOCH.with(|v| v.get()), before.wrapping_add(1));
        assert_eq!(
            WEB_CURSORS.with(|v| v.borrow().clone()),
            BTreeMap::from([(0, None)])
        );
        assert!(WEB_HIDDEN.with(|v| v.borrow().is_empty()));
    }
    #[test]
    fn safe_account_commands_are_typed() {
        let commands: [&str; 3] = [
            Command::GetAccount.into(),
            Command::SignIn.into(),
            Command::SignOut.into(),
        ];
        assert_eq!(commands, ["get_account", "sign_in", "sign_out"]);
    }
    #[test]
    fn uuid_is_canonical() {
        assert_eq!(uuid([0; 16]), "00000000-0000-0000-0000-000000000000");
    }
}
