//! HTTP calls to the TutorLoL server (spec §10.2) and chunk encoding.

use crate::BINCODE_CONFIG;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Read, Write},
    time::Duration,
};
use tauri_plugin_http::reqwest::{Client, Method};
use tutorlolv2_api::{
    self as api, Auth, BodyEncoding, ContractError, Endpoint, NoBody, NoHeaders, NoPath, NoQuery,
    ResponseBuffer, ResponseHeaders, ResponseMode,
};
use tutorlolv2_protocol::{
    BROTLI_LGWIN, BROTLI_QUALITY, DeviceCredentials, EndRecording, Id16, MAX_CHUNK_DECOMPRESSED,
    RawChunk, RawSnapshot, RecordingResult, RegisterDevice, SessionStarted, SetShareMode,
    ShareMode, ShareState, StartSession,
};

const OCTET_STREAM: &str = "application/octet-stream";
const MAX_BACKOFF_MS: u64 = 300_000;

fn sha_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Upload body (`brotli(bincode(chunk))`) and its `X-Content-Sha256`.
pub fn encode_chunk(chunk: &RawChunk) -> (Vec<u8>, String) {
    let raw =
        bincode::encode_to_vec(chunk, BINCODE_CONFIG).expect("encoding into a Vec cannot fail");

    let mut body = Vec::with_capacity(raw.len() / 8);

    {
        let mut writer =
            brotli::CompressorWriter::new(&mut body, 4096, BROTLI_QUALITY, BROTLI_LGWIN);

        writer
            .write_all(&raw)
            .expect("writing into a Vec cannot fail");
    }

    (body, sha_hex(&raw))
}

fn decompress(body: &[u8]) -> io::Result<Vec<u8>> {
    let mut raw = Vec::new();

    brotli::Decompressor::new(body, 4096)
        .take(MAX_CHUNK_DECOMPRESSED as u64 + 1)
        .read_to_end(&mut raw)?;

    Ok(raw)
}

/// `X-Content-Sha256` of a spooled upload body.
pub fn body_sha(body: &[u8]) -> io::Result<String> {
    Ok(sha_hex(&decompress(body)?))
}

/// Rewrites the seq inside a spooled chunk (only for resumed recordings, spec §7.2).
pub fn renumber(body: &[u8], new_seq: u32) -> io::Result<(Vec<u8>, String)> {
    let raw = decompress(body)?;
    let (mut chunk, _): (RawChunk, _) =
        bincode::decode_from_slice(&raw, BINCODE_CONFIG).map_err(io::Error::other)?;

    chunk.seq = new_seq;

    Ok(encode_chunk(&chunk))
}

/// Hyphenated lowercase UUID text, as the server's routes expect.
fn uuid_text(id: &Id16) -> String {
    let hex: String = id.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

/// `Device <uuid-hyphenated>.<base64url-nopad(secret)>` (spec §13.1).
pub fn device_auth_header(device_id: &Id16, secret: &[u8; 32]) -> String {
    format!(
        "Device {}.{}",
        uuid_text(device_id),
        URL_SAFE_NO_PAD.encode(secret)
    )
}

/// Exponential backoff with jitter: 1 s, 2 s, 4 s … capped at 5 min, scaled
/// by `0.5 + jitter / 2` (`jitter` ∈ [0, 1)).
pub fn backoff_ms(attempt: u32, jitter: f64) -> u64 {
    let base = 1000u64
        .saturating_mul(1u64.checked_shl(attempt).unwrap_or(u64::MAX))
        .min(MAX_BACKOFF_MS);

    (base as f64 * (0.5 + jitter / 2.0)) as u64
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkOutcome {
    Stored,
    /// Rate limited or server trouble: keep the chunk and try later.
    Retry {
        after_ms: Option<u64>,
    },
    /// Stop the pass when the server asks to slow down.
    RateLimited {
        after_ms: Option<u64>,
    },
    /// The device credentials are no longer valid (e.g. a database reset):
    /// keep the chunk and register again.
    Unauthorized,
    /// The server will never accept this chunk.
    Rejected(u16),
}

fn retry_delay(value: Option<&str>) -> Option<u64> {
    value
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(|s| s.saturating_mul(1000).min(MAX_BACKOFF_MS))
}

pub fn chunk_outcome(status: u16, retry_after: Option<&str>) -> ChunkOutcome {
    match status {
        200 | 201 => ChunkOutcome::Stored,
        401 => ChunkOutcome::Unauthorized,
        429 => ChunkOutcome::RateLimited {
            after_ms: retry_delay(retry_after),
        },
        408 | 500..=599 => ChunkOutcome::Retry {
            after_ms: retry_delay(retry_after),
        },
        other => ChunkOutcome::Rejected(other),
    }
}

/// Keeps remote diagnostics distinct from network/protocol failures until the UI/IPC boundary.
#[derive(Debug)]
struct ApiFailure {
    status: Option<u16>,
    message: String,
    remote: Option<tutorlolv2_protocol::ApiError>,
}
impl ApiFailure {
    fn transport(error: impl std::fmt::Display) -> Self {
        Self {
            status: None,
            message: error.to_string(),
            remote: None,
        }
    }
    fn invalid(status: u16, message: &str) -> Self {
        Self {
            status: Some(status),
            message: message.into(),
            remote: None,
        }
    }
    fn contract(status: Option<u16>, error: ContractError) -> Self {
        match error {
            ContractError::Remote { status, error } => Self {
                status: Some(status),
                message: error.message.clone(),
                remote: Some(error),
            },
            error => Self {
                status,
                message: error.to_string(),
                remote: None,
            },
        }
    }
    fn into_message(self) -> String {
        if let Some(remote) = self.remote.as_ref() {
            log::debug!("API failure: {remote:?}");
        }
        match self.status {Some(status)=>format!("{status}: {}",self.message),None=>self.message}
    }
    fn into_history(self) -> tutorlolv2_protocol::HistoryError {
        use tutorlolv2_protocol::HistoryErrorKind as K;
        let kind = match self.status {
            None => K::Offline,
            Some(401) => K::Unauthorized,
            Some(403) => K::Restricted,
            Some(404) => K::NotFound,
            Some(409) => K::Conflict,
            Some(422) => K::UnsupportedFormat,
            Some(200..=299) => K::InvalidData,
            _ => K::Other,
        };
        let retryable = self.status.is_none() || self.status.is_some_and(|s| s >= 500 || s == 429);
        crate::history::error(kind, &self.into_message(), retryable)
    }
    fn into_account(self) -> tutorlolv2_protocol::AccountError {
        use tutorlolv2_protocol::AccountErrorKind as K;
        let kind = if self.status == Some(401) {
            K::Unauthorized
        } else if self.status.is_none() || self.status.is_some_and(|s| s >= 500 || s == 429) {
            K::Unavailable
        } else {
            K::InvalidData
        };
        if let Some(remote) = self.remote.as_ref() {
            log::debug!("API failure: {remote:?}");
        }
        crate::account::error(
            kind,
            &self.message,
            self.status.is_none() || self.status.is_some_and(|s| s >= 500 || s == 429),
        )
    }
}

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    base: String,
    auth: Option<String>,
    account_auth: Option<String>,
    account_invalid: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub owner_generation: u64,
    pub device_id: Option<Id16>,
    owner_limit: std::sync::Arc<tokio::sync::Semaphore>,
}

impl ApiClient {
    /// Server origin, set at build time with `TUTORLOLV2_API_URL`.
    pub const BASE: &'static str = match option_env!("TUTORLOLV2_API_URL") {
        Some(url) => url,
        None => "http://127.0.0.1:8082",
    };

    pub fn new(credentials: Option<&DeviceCredentials>) -> Self {
        Self::with_base_url(Self::BASE.to_owned(), credentials)
    }

    pub fn with_base_url(base: String, credentials: Option<&DeviceCredentials>) -> Self {
        let http = Client::builder()
            .no_brotli()
            .no_gzip()
            .no_deflate()
            .no_zstd()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("static client configuration is valid");

        let auth = credentials.map(|c| device_auth_header(&c.device_id, &c.device_secret));

        Self {
            http,
            base,
            auth,
            account_auth: None,
            account_invalid: Default::default(),
            owner_generation: 0,
            device_id: credentials.map(|c| c.device_id),
            owner_limit: std::sync::Arc::new(tokio::sync::Semaphore::new(4)),
        }
    }

    pub fn set_account(&mut self, token: Option<&str>, generation: u64) {
        self.account_invalid
            .store(true, std::sync::atomic::Ordering::Release);
        self.account_invalid = Default::default();
        self.account_auth = token.map(|token| format!("Session {token}"));
        self.owner_generation = generation;
    }

    pub fn account_invalid(&self) -> bool {
        self.account_invalid
            .load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn check_owner(&self) -> Result<Option<Id16>, tutorlolv2_protocol::HistoryError> {
        if self.account_invalid() {
            return Err(crate::history::error(
                tutorlolv2_protocol::HistoryErrorKind::Unauthorized,
                "History account changed or expired",
                true,
            ));
        }
        Ok(self.owner_id())
    }

    /// Effective cache owner. Credentials never leave native code.
    pub fn owner_id(&self) -> Option<Id16> {
        use sha2::Digest;
        if self.account_invalid() {
            return None;
        }
        self.account_auth
            .as_ref()
            .map(|token| {
                let hash = sha2::Sha256::digest(token.as_bytes());
                let mut id = [0; 16];
                id.copy_from_slice(&hash[..16]);
                id
            })
            .or(self.device_id)
    }

    pub fn login_url(&self, port: u16, state: &str) -> String {
        api::GoogleStart::url(
            &self.base,
            &NoPath,
            &api::GoogleStartQuery {
                target: "desktop".into(),
                return_to: None,
                port: Some(port),
                app_state: Some(state.into()),
            },
        )
        .expect("desktop sign-in query is serializable")
    }

    /// Executes a declared endpoint. Success and failure bodies remain available for typed decoding.
    async fn request<E: Endpoint>(
        &self,
        path: &E::Path,
        query: &E::Query,
        headers: &E::Headers,
        body: &E::Body<'_>,
    ) -> Result<ResponseBuffer<E>, ApiFailure> {
        let _permit = if E::AUTH == Auth::Owner {
            Some(
                tokio::time::timeout(Duration::from_secs(10), self.owner_limit.acquire())
                    .await
                    .map_err(ApiFailure::transport)?
                    .map_err(ApiFailure::transport)?,
            )
        } else {
            None
        };
        let parts = E::request(&self.base, path, query, headers, body)
            .map_err(|e| ApiFailure::contract(None, e))?;
        self.execute::<E>(parts).await
    }

    async fn execute<E: Endpoint>(
        &self,
        mut parts: api::EndpointParts,
    ) -> Result<ResponseBuffer<E>, ApiFailure> {
        if parts.encoding == BodyEncoding::BrotliBincode {
            if E::PATH==api::PushLive::PATH && parts.body.len()>512*1024 {return Err(ApiFailure::invalid(413,"Live snapshot exceeds size limit"));}
            let mut compressed = Vec::new();
            {
                let mut writer = brotli::CompressorWriter::new(
                    &mut compressed,
                    4096,
                    BROTLI_QUALITY,
                    BROTLI_LGWIN,
                );
                writer
                    .write_all(&parts.body)
                    .map_err(ApiFailure::transport)?;
            }
            if E::PATH==api::PushLive::PATH && compressed.len()>64*1024 {return Err(ApiFailure::invalid(413,"Compressed live snapshot exceeds size limit"));}
            parts.body = compressed;
        }
        let auth = match parts.auth {
            Auth::Device => self.auth.as_ref(),
            Auth::Account => self.account_auth.as_ref(),
            Auth::Owner => self.account_auth.as_ref().or(self.auth.as_ref()),
            _ => None,
        };
        if matches!(parts.auth, Auth::Device | Auth::Account | Auth::Owner) && auth.is_none() {
            return Err(ApiFailure {
                status: Some(401),
                message: "Authentication required".into(),
                remote: None,
            });
        }
        let method =
            Method::from_bytes(parts.method.as_str().as_bytes()).expect("declared HTTP method");
        let mut request = self
            .http
            .request(method, parts.url)
            .header("Content-Type", OCTET_STREAM)
            .body(parts.body);
        if let Some(auth) = auth {
            request = request.header("Authorization", auth);
        }
        for (name, value) in parts.headers {
            request = request.header(name, value);
        }
        let mut response = request.send().await.map_err(ApiFailure::transport)?;
        let status = response.status().as_u16();
        if status == 401
            && matches!(parts.auth, Auth::Account | Auth::Owner)
            && self.account_auth.is_some()
        {
            self.account_invalid
                .store(true, std::sync::atomic::Ordering::Release);
        }
        let headers = ResponseHeaders(
            response
                .headers()
                .iter()
                .filter_map(|(name, value)| {
                    value
                        .to_str()
                        .ok()
                        .map(|v| (name.as_str().into(), v.into()))
                })
                .collect(),
        );
        let bound =
            if (200..300).contains(&status) && E::RESPONSE_MODE == ResponseMode::BrotliBincode {
                tutorlolv2_protocol::MAX_CHUNK_BYTES
            } else {
                tutorlolv2_protocol::response::MAX_RESPONSE_BYTES
            };
        if response
            .content_length()
            .is_some_and(|size| size > bound as u64)
        {
            return Err(ApiFailure::invalid(status, "API response exceeds limit"));
        }
        let mut bytes = Vec::new();
        while let Some(part) = response.chunk().await.map_err(ApiFailure::transport)? {
            if bytes.len().saturating_add(part.len()) > bound {
                return Err(ApiFailure::invalid(status, "API response exceeds limit"));
            }
            bytes.extend_from_slice(&part);
        }
        if (200..300).contains(&status) && E::RESPONSE_MODE == ResponseMode::BrotliBincode {
            if headers.get("Content-Encoding") != Some("br") {
                return Err(ApiFailure::invalid(status, "Unsupported replay encoding"));
            }
            bytes = decompress(&bytes).map_err(ApiFailure::transport)?;
        }
        ResponseBuffer::new(status, headers, bytes)
            .map_err(|e| ApiFailure::contract(Some(status), e))
    }

    pub async fn account(
        &self,
    ) -> Result<tutorlolv2_protocol::AccountInfo, tutorlolv2_protocol::AccountError> {
        let response = self
            .request::<api::GetAccount>(&NoPath, &NoQuery, &NoHeaders, &NoBody)
            .await
            .map_err(ApiFailure::into_account)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_account())
    }
    pub async fn confirm_desktop(&self) -> Result<(), tutorlolv2_protocol::AccountError> {
        let response = self
            .request::<api::ConfirmDesktop>(&NoPath, &NoQuery, &NoHeaders, &NoBody)
            .await
            .map_err(ApiFailure::into_account)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_account())
    }
    pub async fn logout(&self) -> Result<(), tutorlolv2_protocol::AccountError> {
        let response = self
            .request::<api::Logout>(&NoPath, &NoQuery, &NoHeaders, &NoBody)
            .await
            .map_err(ApiFailure::into_account)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_account())
    }
    pub async fn redeem(
        &self,
        ticket: tutorlolv2_protocol::RedeemTicket,
    ) -> Result<tutorlolv2_protocol::DesktopSession, tutorlolv2_protocol::AccountError> {
        let response = self
            .request::<api::RedeemDesktop>(&NoPath, &NoQuery, &NoHeaders, &ticket)
            .await
            .map_err(ApiFailure::into_account)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_account())
    }

    pub fn authenticated(&self) -> bool {
        self.auth.is_some()
    }

    pub async fn history_page(
        &self,
        before: Option<Id16>,
    ) -> Result<tutorlolv2_protocol::HistoryPage, tutorlolv2_protocol::HistoryError> {
        let response = self
            .request::<api::ListRecordings>(
                &NoPath,
                &api::Pagination {
                    before: before.map(api::Uuid::from_bytes),
                    limit: Some(100),
                },
                &NoHeaders,
                &NoBody,
            )
            .await
            .map_err(ApiFailure::into_history)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_history())
    }
    pub async fn history_details(
        &self,
        id: Id16,
    ) -> Result<tutorlolv2_protocol::RecordingDetails, tutorlolv2_protocol::HistoryError> {
        let response = self
            .request::<api::GetRecording>(
                &api::GetRecordingPath {
                    id: api::Uuid::from_bytes(id),
                },
                &NoQuery,
                &NoHeaders,
                &NoBody,
            )
            .await
            .map_err(ApiFailure::into_history)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_history())
    }
    pub async fn history_frames(
        &self,
        id: Id16,
        seq: u32,
    ) -> Result<Vec<u8>, tutorlolv2_protocol::HistoryError> {
        if seq > i32::MAX as u32 {
            return Err(crate::history::error(
                tutorlolv2_protocol::HistoryErrorKind::InvalidData,
                "Invalid chunk sequence",
                false,
            ));
        }
        let response = self
            .request::<api::OwnerFrames>(
                &api::OwnerFramesPath {
                    id: api::Uuid::from_bytes(id),
                    seq,
                },
                &NoQuery,
                &NoHeaders,
                &NoBody,
            )
            .await
            .map_err(ApiFailure::into_history)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_history())?;
        Ok(response.into_bytes())
    }
    pub async fn history_share(
        &self,
        id: Id16,
        mode: tutorlolv2_protocol::HistoryShareMode,
    ) -> Result<ShareState, tutorlolv2_protocol::HistoryError> {
        let response = self
            .request::<api::SetHistoryShare>(
                &api::SetHistorySharePath {
                    id: api::Uuid::from_bytes(id),
                },
                &NoQuery,
                &NoHeaders,
                &SetShareMode { mode: mode.into() },
            )
            .await
            .map_err(ApiFailure::into_history)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_history())
    }
    pub async fn register(&self, app_version: &str) -> Result<DeviceCredentials, String> {
        let response = self
            .request::<api::RegisterDeviceEndpoint>(
                &NoPath,
                &NoQuery,
                &NoHeaders,
                &RegisterDevice {
                    app_version: app_version.into(),
                },
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn start_session(&self, request: &StartSession) -> Result<SessionStarted, String> {
        let response = self
            .request::<api::StartSessionEndpoint>(&NoPath, &NoQuery, &NoHeaders, request)
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    /// Sealed spool data already has a validated bincode/Brotli body and content hash.
    /// Keep that byte representation intact when retrying; only the declared endpoint owns its path/method.
    pub async fn upload_chunk(
        &self,
        id: &Id16,
        seq: u32,
        body: Vec<u8>,
        sha_hex: &str,
    ) -> ChunkOutcome {
        let path = api::UploadChunkPath {
            id: api::Uuid::from_bytes(*id),
            seq,
        };
        let Ok(headers) = api::UploadChunk::headers(&api::ContentHash {
            sha256: sha_hex.into(),
        }) else {
            return ChunkOutcome::Rejected(422);
        };
        let Ok(url) = api::UploadChunk::url(&self.base, &path, &NoQuery) else {
            return ChunkOutcome::Retry { after_ms: None };
        };
        let parts = api::EndpointParts {
            method: api::UploadChunk::METHOD,
            url,
            headers,
            body,
            encoding: BodyEncoding::None,
            auth: api::UploadChunk::AUTH,
        };
        match self.execute::<api::UploadChunk>(parts).await {
            Ok(response) => {
                if (200..300).contains(&response.status()) && response.decode().is_err() {
                    return ChunkOutcome::Retry { after_ms: None };
                }
                chunk_outcome(response.status(), response.headers().get("Retry-After"))
            }
            Err(error) if error.status == Some(401) => ChunkOutcome::Unauthorized,
            Err(_) => ChunkOutcome::Retry { after_ms: None },
        }
    }
    pub async fn share(&self, id: &Id16) -> Result<ShareState, String> {
        let response = self
            .request::<api::GetRecordingShare>(
                &api::GetRecordingSharePath {
                    id: api::Uuid::from_bytes(*id),
                },
                &NoQuery,
                &NoHeaders,
                &NoBody,
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn set_share_mode(&self, id: &Id16, mode: ShareMode) -> Result<ShareState, String> {
        let response = self
            .request::<api::SetRecordingShare>(
                &api::SetRecordingSharePath {
                    id: api::Uuid::from_bytes(*id),
                },
                &NoQuery,
                &NoHeaders,
                &SetShareMode { mode },
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn upload_live(&self, id: &Id16, snapshot: &RawSnapshot) -> Result<(), String> {
        let response = self
            .request::<api::PushLive>(
                &api::PushLivePath {
                    id: api::Uuid::from_bytes(*id),
                },
                &NoQuery,
                &NoHeaders,
                snapshot,
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn heartbeat(&self, recording_id: &Id16) -> Result<(), String> {
        let response = self
            .request::<api::Heartbeat>(
                &api::HeartbeatPath {
                    id: api::Uuid::from_bytes(*recording_id),
                },
                &NoQuery,
                &NoHeaders,
                &NoBody,
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn end(&self, recording_id: &Id16, request: &EndRecording) -> Result<(), String> {
        let response = self
            .request::<api::EndRecordingEndpoint>(
                &api::EndRecordingEndpointPath {
                    id: api::Uuid::from_bytes(*recording_id),
                },
                &NoQuery,
                &NoHeaders,
                request,
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
    pub async fn result(
        &self,
        recording_id: &Id16,
        request: &RecordingResult,
    ) -> Result<(), String> {
        let response = self
            .request::<api::SetResult>(
                &api::SetResultPath {
                    id: api::Uuid::from_bytes(*recording_id),
                },
                &NoQuery,
                &NoHeaders,
                request,
            )
            .await
            .map_err(ApiFailure::into_message)?;
        response
            .decode()
            .map(drop)
            .map_err(|e| ApiFailure::contract(Some(response.status()), e).into_message())
    }
}

#[cfg(test)]
mod account_transport_tests {
    use super::*;
    #[tokio::test]
    async fn session_auth_is_only_used_for_owner_calls() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let mut requests = vec![];
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut buf = [0; 8192];
                let n = stream.read(&mut buf).unwrap();
                requests.push(String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase());
                stream.write_all(b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
            }
            requests
        });
        let mut api = ApiClient::with_base_url(
            base,
            Some(&DeviceCredentials {
                device_id: [1; 16],
                device_secret: [2; 32],
            }),
        );
        api.set_account(Some("native-only-token"), 1);
        assert!(api.history_page(None).await.is_err());
        assert!(api.account_invalid());
        assert!(matches!(
            api.upload_chunk(&[3; 16], 0, vec![], &"0".repeat(64)).await,
            ChunkOutcome::Unauthorized
        ));
        let requests = server.join().unwrap();
        assert!(requests[0].contains("authorization: session native-only-token"));
        assert!(requests[1].contains("authorization: device "));
        assert!(!requests[1].contains("native-only-token"));
    }
}

#[cfg(test)]
mod api_response_tests {
    use super::*;
    use tutorlolv2_protocol::{AccountInfo, ApiError, ApiResponse, ErrorCode};

    // Catches regressions that decode the envelope as a bare DTO or discard a remote error body.
    #[tokio::test]
    async fn account_decodes_bincode_success_and_remote_failure() {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let success = bincode::encode_to_vec(
                ApiResponse::new(
                    200,
                    Ok(AccountInfo {
                        email: "player@example.test".into(),
                        created_at_ms: 42,
                    }),
                )
                .unwrap(),
                BINCODE_CONFIG,
            )
            .unwrap();
            let failure = bincode::encode_to_vec(
                ApiResponse::<()>::new(
                    503,
                    Err(ApiError {
                        code: ErrorCode::Upstream,
                        message: "Account service unavailable".into(),
                        details: None,
                        traces: vec![],
                    }),
                )
                .unwrap(),
                BINCODE_CONFIG,
            )
            .unwrap();
            for (status, bytes) in [("200 OK", success), ("503 Service Unavailable", failure)] {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut request = [0; 4096];
                let n = stream.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..n]).to_ascii_lowercase();
                assert!(request.starts_with("get /api/account "));
                assert!(request.contains("authorization: session test-token"));
                write!(stream, "HTTP/1.1 {status}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len()).unwrap();
                stream.write_all(&bytes).unwrap();
            }
        });
        let mut api = ApiClient::with_base_url(base, None);
        api.set_account(Some("test-token"), 1);
        let account = api.account().await.unwrap();
        assert_eq!(account.email, "player@example.test");
        assert_eq!(account.created_at_ms, 42);
        let error = api.account().await.unwrap_err();
        assert_eq!(error.message, "Account service unavailable");
        assert!(error.retryable);
        server.join().unwrap();
    }
}

#[cfg(test)]
mod metadata_response_tests {
    use super::*;
    use tutorlolv2_protocol::{ApiResponse, HistoryPage};
    #[tokio::test]
    async fn registration_and_history_use_typed_envelopes_and_separate_auth() {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            let credentials = bincode::encode_to_vec(
                ApiResponse::new(
                    200,
                    Ok(DeviceCredentials {
                        device_id: [1; 16],
                        device_secret: [2; 32],
                    }),
                )
                .unwrap(),
                BINCODE_CONFIG,
            )
            .unwrap();
            let page = bincode::encode_to_vec(
                ApiResponse::new(
                    200,
                    Ok(HistoryPage {
                        items: vec![],
                        next_before: None,
                    }),
                )
                .unwrap(),
                BINCODE_CONFIG,
            )
            .unwrap();
            for (path, body) in [
                ("post /api/devices ", credentials),
                ("get /api/me/recordings?limit=100 ", page),
            ] {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut input = [0; 8192];
                let n = stream.read(&mut input).unwrap();
                let request = String::from_utf8_lossy(&input[..n]).to_ascii_lowercase();
                assert!(request.starts_with(path), "{request}");
                if path.starts_with("get") {
                    assert!(request.contains("authorization: session owner-token"));
                } else {
                    assert!(!request.contains("authorization:"));
                }
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();
                stream.write_all(&body).unwrap();
            }
        });
        let mut api = ApiClient::with_base_url(base, None);
        let credentials = api.register("test").await.unwrap();
        assert_eq!(credentials.device_id, [1; 16]);
        api.set_account(Some("owner-token"), 1);
        let page = api.history_page(None).await.unwrap();
        assert!(page.items.is_empty());
        server.join().unwrap();
    }
}
