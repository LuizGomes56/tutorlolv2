//! Browser HTTP I/O for the shared Rust endpoint contracts.
use std::{fmt, io::Write};
use tutorlolv2_api::{
    Auth, BodyEncoding, ContractError, Endpoint, ResponseBuffer, ResponseHeaders, ResponseMode,
};
use tutorlolv2_protocol::{HistoryError, HistoryErrorKind, response::MAX_RESPONSE_BYTES};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AbortController, ReadableStreamDefaultReader, RequestCredentials, RequestInit, Response,
    js_sys::{Reflect, Uint8Array},
};

#[derive(Debug)]
pub enum FetchError {
    Transport(String),
    Contract(ContractError),
    Response { status: u16, error: ContractError },
}
impl FetchError {
    pub fn response(status: u16, error: ContractError) -> Self {
        Self::Response { status, error }
    }
    fn status(&self) -> Option<u16> {
        match self {
            Self::Response { status, .. }
            | Self::Contract(ContractError::Remote { status, .. })
            | Self::Contract(ContractError::UnexpectedStatus(status)) => Some(*status),
            _ => None,
        }
    }
    pub fn message(self) -> String {
        match self.status() {
            Some(404) => return "Code not found. Check the code and try again.".into(),
            Some(410) => return "This match ended or is no longer shared.".into(),
            _ => {}
        }
        match self {
            Self::Contract(ContractError::Remote { error, .. })
            | Self::Response {
                error: ContractError::Remote { error, .. },
                ..
            } => error.message,
            error => error.to_string(),
        }
    }
    pub fn history(self) -> HistoryError {
        let status = self.status();
        let (kind, retryable) = match status {
            Some(401) => (HistoryErrorKind::Unauthorized, false),
            Some(403) => (HistoryErrorKind::Restricted, false),
            Some(404) => (HistoryErrorKind::NotFound, false),
            Some(409) => (HistoryErrorKind::Conflict, false),
            Some(422) => (HistoryErrorKind::UnsupportedFormat, false),
            Some(status) if status >= 500 || status == 429 => (HistoryErrorKind::Other, true),
            _ if matches!(self, Self::Transport(_)) => (HistoryErrorKind::Offline, true),
            _ => (HistoryErrorKind::InvalidData, false),
        };
        let message = match status {
            Some(401) => "Your session expired. Sign in again.".into(),
            Some(403) => "This action is restricted for this recording.".into(),
            Some(404) => "Recording not found.".into(),
            _ => self.message(),
        };
        HistoryError {
            kind,
            message,
            retryable,
        }
    }
    fn js(value: JsValue) -> Self {
        Self::Transport(format!("API connection failed: {value:?}"))
    }
}
impl From<ContractError> for FetchError {
    fn from(error: ContractError) -> Self {
        Self::Contract(error)
    }
}
impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(value) => f.write_str(value),
            Self::Contract(value) => value.fmt(f),
            Self::Response { status, error } => write!(f, "API response (HTTP {status}): {error}"),
        }
    }
}
impl std::error::Error for FetchError {}

pub struct WebClient {
    base: String,
}
impl WebClient {
    pub fn new(base: String) -> Self {
        Self { base }
    }
    pub fn public() -> Self {
        Self::new(
            option_env!("TUTORLOLV2_API_URL")
                .or(option_env!("TUTORLOL_API_URL"))
                .unwrap_or("http://127.0.0.1:8082")
                .trim_end_matches('/')
                .trim_end_matches("/games")
                .into(),
        )
    }
    pub async fn send<E: Endpoint>(
        &self,
        path: &E::Path,
        query: &E::Query,
        headers: &E::Headers,
        body: &E::Body<'_>,
        abort: Option<&AbortController>,
    ) -> Result<ResponseBuffer<E>, FetchError> {
        if E::RESPONSE_MODE == ResponseMode::Browser {
            return Err(ContractError::BrowserOnly.into());
        }
        let mut parts = E::request(&self.base, path, query, headers, body)?;
        if parts.body.len() > MAX_RESPONSE_BYTES {
            return Err(tutorlolv2_protocol::ProtocolError::Oversized.into());
        }
        if parts.encoding == BodyEncoding::BrotliBincode {
            let mut compressed = Vec::new();
            {
                let mut writer = brotli::CompressorWriter::new(
                    &mut compressed,
                    4096,
                    tutorlolv2_protocol::BROTLI_QUALITY,
                    tutorlolv2_protocol::BROTLI_LGWIN,
                );
                writer
                    .write_all(&parts.body)
                    .map_err(|e| FetchError::Transport(e.to_string()))?;
            }
            parts.body = compressed;
        }
        let init = RequestInit::new();
        init.set_method(parts.method.as_str());
        init.set_credentials(if matches!(parts.auth, Auth::Account | Auth::Owner) {
            RequestCredentials::Include
        } else {
            RequestCredentials::Omit
        });
        if let Some(abort) = abort {
            init.set_signal(Some(&abort.signal()));
        }
        let headers = web_sys::Headers::new().map_err(FetchError::js)?;
        headers
            .set("Content-Type", "application/octet-stream")
            .map_err(FetchError::js)?;
        headers
            .set("Accept", "application/octet-stream")
            .map_err(FetchError::js)?;
        for (name, value) in parts.headers {
            headers.set(&name, &value).map_err(FetchError::js)?;
        }
        init.set_headers(&headers);
        if parts.encoding != BodyEncoding::None {
            init.set_body(&Uint8Array::from(parts.body.as_slice()));
        }
        let response: Response = JsFuture::from(
            web_sys::window()
                .ok_or_else(|| FetchError::Transport("Browser unavailable".into()))?
                .fetch_with_str_and_init(&parts.url, &init),
        )
        .await
        .map_err(FetchError::js)?
        .dyn_into()
        .map_err(FetchError::js)?;
        let status = response.status();
        let mut headers = ResponseHeaders::default();
        for name in E::RESPONSE_HEADERS
            .iter()
            .copied()
            .chain(["Content-Encoding", "Retry-After"])
        {
            if let Some(value) = response.headers().get(name).map_err(FetchError::js)? {
                headers.0.push((name.into(), value));
            }
        }
        // Fetch transparently decompresses Content-Encoding. Bound the resulting bytes.
        let bytes = Self::read(response).await?;
        ResponseBuffer::new(status, headers, bytes).map_err(|e| FetchError::response(status, e))
    }
    async fn read(response: Response) -> Result<Vec<u8>, FetchError> {
        let Some(stream) = response.body() else {
            return Ok(vec![]);
        };
        let reader: ReadableStreamDefaultReader = stream
            .get_reader()
            .dyn_into()
            .map_err(|value: web_sys::js_sys::Object| FetchError::js(value.into()))?;
        let mut bytes = Vec::new();
        loop {
            let part = JsFuture::from(reader.read())
                .await
                .map_err(FetchError::js)?;
            if Reflect::get(&part, &"done".into())
                .map_err(FetchError::js)?
                .as_bool()
                == Some(true)
            {
                break;
            }
            let value = Reflect::get(&part, &"value".into()).map_err(FetchError::js)?;
            let chunk = Uint8Array::new(&value);
            if bytes.len().saturating_add(chunk.length() as usize) > MAX_RESPONSE_BYTES {
                let _ = reader.cancel();
                return Err(
                    ContractError::Protocol(tutorlolv2_protocol::ProtocolError::Oversized).into(),
                );
            }
            bytes.extend_from_slice(&chunk.to_vec());
        }
        Ok(bytes)
    }
}
impl From<tutorlolv2_protocol::ProtocolError> for FetchError {
    fn from(error: tutorlolv2_protocol::ProtocolError) -> Self {
        Self::Contract(ContractError::Protocol(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proxy_errors_keep_retry_policy_and_owner_404_uses_recording_message() {
        let error = FetchError::response(
            503,
            ContractError::Protocol(tutorlolv2_protocol::ProtocolError::Malformed),
        )
        .history();
        assert!(error.retryable);
        assert_eq!(error.kind, HistoryErrorKind::Other);
        let error = FetchError::response(
            404,
            ContractError::Remote {
                status: 404,
                error: tutorlolv2_protocol::ApiError {
                    code: tutorlolv2_protocol::ErrorCode::NotFound,
                    message: "missing".into(),
                    details: None,
                    traces: vec![],
                },
            },
        )
        .history();
        assert_eq!(error.kind, HistoryErrorKind::NotFound);
        assert_eq!(error.message, "Recording not found.");
        assert!(!error.retryable);
    }
}
