use std::{cell::RefCell, collections::BTreeMap, rc::Weak};
use tutorlolv2_protocol::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, Function, Promise, Reflect};

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
            .invoke(serde_json::json!({"offset":offset,"limit":50}))
            .await
    }

    async fn details(id: &str) -> Result<HistoryDetails, HistoryError> {
        Command::GetHistoryDetails
            .invoke(serde_json::json!({"recordingId":id}))
            .await
    }

    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
        owner_frames(id, seq).await
    }

    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError> {
        Command::SetHistoryShareMode
            .invoke(serde_json::json!({"recordingId":id,"mode":mode}))
            .await
    }

    async fn hide(id: &str) -> Result<(), HistoryError> {
        Command::HideHistoryEntry
            .invoke(serde_json::json!({"recordingId":id}))
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

struct FrameFuture(std::rc::Rc<std::cell::RefCell<FrameRequest>>);

impl Future for FrameFuture {
    type Output = Result<Vec<u8>, HistoryError>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.0.borrow_mut();

        if let Some(result) = &state.result {
            return std::task::Poll::Ready(result.clone());
        }

        if !state.waiters.iter().any(|w| w.will_wake(cx.waker())) {
            state.waiters.push(cx.waker().clone());
        }

        std::task::Poll::Pending
    }
}

async fn owner_frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
    let key = (OWNER_EPOCH.with(|v| v.get()), id.to_owned(), seq);
    let (request, start) = FRAME_REQUESTS.with(|requests| {
        let mut requests = requests.borrow_mut();

        if let Some(request) = requests.get(&key).and_then(std::rc::Weak::upgrade) {
            (request, false)
        } else {
            let request = std::rc::Rc::new(std::cell::RefCell::new(FrameRequest {
                result: None,
                waiters: vec![],
            }));

            requests.insert(key.clone(), std::rc::Rc::downgrade(&request));
            (request, true)
        }
    });

    if start {
        let request = request.clone();

        yew::platform::spawn_local(async move {
            let result = async {
                let value = Command::GetHistoryFrames
                    .invoke_raw(serde_json::json!({"recordingId":key.1,"seq":seq}))
                    .await?;

                if value.is_instance_of::<js_sys::ArrayBuffer>()
                    || value.is_instance_of::<js_sys::Uint8Array>()
                {
                    let bytes = js_sys::Uint8Array::new(&value);

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
                std::mem::take(&mut state.waiters)
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
    static OWNER_EPOCH: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static WEB_CURSORS: RefCell<BTreeMap<u32, Option<Id16>>> = RefCell::new(BTreeMap::from([(0,None)]));
    static WEB_HIDDEN: RefCell<std::collections::BTreeSet<String>> = const { RefCell::new(std::collections::BTreeSet::new()) };
}

pub fn clear_web_history() {
    OWNER_EPOCH.with(|v| v.set(v.get().wrapping_add(1)));
    FRAME_REQUESTS.with(|v| v.borrow_mut().clear());
    WEB_CURSORS.with(|v| *v.borrow_mut() = BTreeMap::from([(0, None)]));
    WEB_HIDDEN.with(|v| v.borrow_mut().clear());
}

pub fn owner_api_base() -> String {
    option_env!("TUTORLOLV2_API_URL")
        .or(option_env!("TUTORLOL_API_URL"))
        .unwrap_or("/api")
        .trim_end_matches('/')
        .trim_end_matches("/games")
        .into()
}

pub async fn http_request(
    path: &str,
    method: &str,
    body: Option<Vec<u8>>,
) -> Result<web_sys::Response, HistoryError> {
    let epoch = OWNER_EPOCH.with(|v| v.get());
    let init = web_sys::RequestInit::new();
    init.set_method(method);
    init.set_credentials(web_sys::RequestCredentials::Include);
    let headers = web_sys::Headers::new().map_err(|_| error("Request unavailable"))?;
    headers
        .set("Content-Type", "application/octet-stream")
        .map_err(|_| error("Request unavailable"))?;
    init.set_headers(&headers);
    if let Some(body) = body {
        init.set_body(&js_sys::Uint8Array::from(body.as_slice()));
    }
    let response = JsFuture::from(
        web_sys::window()
            .ok_or_else(|| error("Window unavailable"))?
            .fetch_with_str_and_init(&format!("{}/{path}", owner_api_base()), &init),
    )
    .await
    .map_err(|_| HistoryError {
        kind: HistoryErrorKind::Offline,
        message: "Could not reach your account. Check your connection and retry.".into(),
        retryable: true,
    })?
    .dyn_into()
    .map_err(|_| error("Invalid server response"))?;
    if OWNER_EPOCH.with(|v| v.get()) != epoch {
        return Err(error("Account changed. Refresh history."));
    }
    Ok(response)
}

pub async fn http_decode<T: bincode::Decode<()>>(
    response: web_sys::Response,
) -> Result<T, HistoryError> {
    if !response.ok() {
        if response.status() == 401 {
            crate::account::invalidate();
        }
        return Err(HistoryError {
            kind: if response.status() == 401 {
                HistoryErrorKind::Unauthorized
            } else if response.status() == 404 {
                HistoryErrorKind::NotFound
            } else {
                HistoryErrorKind::Other
            },
            message: match response.status() {
                401 => "Your session expired. Sign in again.".into(),
                403 => "This action is restricted for this recording.".into(),
                404 => "Recording not found.".into(),
                status => format!("Account service unavailable (HTTP {status}). Retry."),
            },
            retryable: response.status() != 401,
        });
    }
    let bytes = crate::watch::api::bytes(&response)
        .await
        .map_err(|e| error(&e))?;
    let (value, used) = bincode::decode_from_slice(&bytes, crate::utils::BINCODE_CONFIG)
        .map_err(|_| error("Invalid account response. Retry."))?;
    if used != bytes.len() {
        return Err(error("Invalid account response. Retry."));
    }
    Ok(value)
}

fn web_entry(summary: HistorySummary, share: ShareState) -> HistoryEntry {
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
        let path = format!(
            "me/recordings?limit=50{}",
            before
                .map(|id| format!("&before={}", uuid(id)))
                .unwrap_or_default()
        );
        let page: HistoryPage = http_decode(http_request(&path, "GET", None).await?).await?;
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
        let recording: RecordingDetails =
            http_decode(http_request(&format!("me/recordings/{id}"), "GET", None).await?).await?;
        if recording.frames_format != FRAMES_FORMAT {
            return Err(error("This replay needs a newer viewer."));
        }
        let entry = web_entry(recording.summary.clone(), recording.share.clone());
        Ok(HistoryDetails { recording, entry })
    }
    async fn frames(id: &str, seq: u32) -> Result<Vec<u8>, HistoryError> {
        let response =
            http_request(&format!("me/recordings/{id}/frames/{seq}"), "GET", None).await?;
        if !response.ok() {
            let _: RecordingDetails = http_decode(response).await?;
            return Err(error("Invalid frames response"));
        }
        // Fetch decodes Content-Encoding: br. The bytes here are already FrameChunk.
        crate::watch::api::bytes(&response)
            .await
            .map_err(|e| error(&e))
    }
    async fn share(id: &str, mode: HistoryShareMode) -> Result<ShareState, HistoryError> {
        let body = bincode::encode_to_vec(
            SetShareMode { mode: mode.into() },
            crate::utils::BINCODE_CONFIG,
        )
        .map_err(|_| error("Invalid share request"))?;
        http_decode(http_request(&format!("me/recordings/{id}/share"), "PUT", Some(body)).await?)
            .await
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
