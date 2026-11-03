//! Runtime-independent HTTP contracts. HTTP transports own compression, credentials and I/O.
use crate::{ApiError, ApiResponse, ProtocolError};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, marker::PhantomData};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
}

impl Method {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseMode {
    Envelope,
    Empty,
    BorrowedEnvelope,
    Bincode,
    BrotliBincode,
    Browser,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyEncoding {
    None,
    Bincode,
    BrotliBincode,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Auth {
    Anonymous,
    Device,
    Account,
    Owner,
    Capability,
    Browser,
}
#[derive(Clone, Debug, Default)]
pub struct NoPath;
#[derive(Clone, Debug, Default)]
pub struct NoQuery;
#[derive(Clone, Debug, Default)]
pub struct NoBody;
#[derive(Clone, Debug, Default)]
pub struct NoHeaders;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentHash {
    pub sha256: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Acknowledgement {
    pub status: u16,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResponseHeaders(pub Vec<(String, String)>);
impl ResponseHeaders {
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
    pub fn required(&self, name: &str) -> Result<&str, ContractError> {
        self.get(name)
            .ok_or_else(|| ContractError::Header(name.into()))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    Protocol(ProtocolError),
    Remote { status: u16, error: ApiError },
    UnexpectedStatus(u16),
    UnexpectedBody,
    Header(String),
    Encode,
    Query,
    BaseUrl,
    BrowserOnly,
}
impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API contract error: {self:?}")
    }
}
impl core::error::Error for ContractError {}
impl From<ProtocolError> for ContractError {
    fn from(value: ProtocolError) -> Self {
        Self::Protocol(value)
    }
}

pub trait Endpoint: Sized {
    type Path;
    type Query;
    type Headers;
    type Body<'a>;
    type Output<'a>;
    const METHOD: Method;
    const PATH: &'static str;
    const SUCCESS: &'static [u16];
    const RESPONSE_MODE: ResponseMode;
    const BODY_ENCODING: BodyEncoding;
    const AUTH: Auth;
    const REQUEST_HEADERS: &'static [&'static str];
    const RESPONSE_HEADERS: &'static [&'static str];
    fn path(path: &Self::Path) -> String;
    fn query(query: &Self::Query) -> Result<String, ContractError>;
    fn headers(headers: &Self::Headers) -> Result<Vec<(String, String)>, ContractError>;
    /// Returns logical bincode bytes. A transport must compress when BODY_ENCODING is BrotliBincode.
    fn encode_body(body: &Self::Body<'_>) -> Result<Vec<u8>, ContractError>;
    fn decode<'a>(
        status: u16,
        headers: &ResponseHeaders,
        bytes: &'a [u8],
    ) -> Result<Self::Output<'a>, ContractError>;
    /// `base` is an origin or mount prefix before `/api`, never an endpoint-specific URL.
    fn url(base: &str, path: &Self::Path, query: &Self::Query) -> Result<String, ContractError> {
        if base.contains(['?', '#']) {
            return Err(ContractError::BaseUrl);
        }
        let base = base.trim_end_matches('/');
        let base = base.strip_suffix("/api").unwrap_or(base);
        let query = Self::query(query)?;
        Ok(format!(
            "{base}{}{}{}",
            Self::path(path),
            if query.is_empty() { "" } else { "?" },
            query
        ))
    }
    fn request(
        base: &str,
        path: &Self::Path,
        query: &Self::Query,
        headers: &Self::Headers,
        body: &Self::Body<'_>,
    ) -> Result<EndpointParts, ContractError> {
        Ok(EndpointParts {
            method: Self::METHOD,
            url: Self::url(base, path, query)?,
            headers: Self::headers(headers)?,
            body: Self::encode_body(body)?,
            encoding: Self::BODY_ENCODING,
            auth: Self::AUTH,
        })
    }
}
#[derive(Debug)]
pub struct EndpointParts {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub encoding: BodyEncoding,
    pub auth: Auth,
}
/// Owns transport-decoded bytes. HTTP Brotli decoding belongs to the platform transport.
///
/// ```compile_fail
/// use tutorlolv2_protocol::endpoint::{Endpoint, ResponseBuffer};
/// fn escape<E: Endpoint>(buffer: ResponseBuffer<E>) -> E::Output<'static> { buffer.decode().unwrap() }
/// ```
pub struct ResponseBuffer<E: Endpoint> {
    status: u16,
    headers: ResponseHeaders,
    bytes: Vec<u8>,
    marker: PhantomData<E>,
}
impl<E: Endpoint> ResponseBuffer<E> {
    pub fn new(
        status: u16,
        headers: ResponseHeaders,
        bytes: Vec<u8>,
    ) -> Result<Self, ContractError> {
        if bytes.len() > crate::response::MAX_RESPONSE_BYTES {
            return Err(ProtocolError::Oversized.into());
        }
        Ok(Self {
            status,
            headers,
            bytes,
            marker: PhantomData,
        })
    }
    pub fn status(&self) -> u16 {
        self.status
    }
    pub fn headers(&self) -> &ResponseHeaders {
        &self.headers
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn decode(&self) -> Result<E::Output<'_>, ContractError> {
        E::decode(self.status, &self.headers, &self.bytes)
    }
}
/// Error decoding happens before endpoint-specific successful decoding, including raw/empty endpoints.
pub fn check_status<E: Endpoint>(status: u16, bytes: &[u8]) -> Result<(), ContractError> {
    if (400..600).contains(&status) {
        let response = ApiResponse::<()>::decode(bytes, status)?;
        return match response.result {
            Err(error) => Err(ContractError::Remote { status, error }),
            Ok(_) => Err(ProtocolError::InvalidStatus.into()),
        };
    }
    if !E::SUCCESS.contains(&status) {
        return Err(ContractError::UnexpectedStatus(status));
    }
    Ok(())
}
pub fn decode_bincode<'a, T: bincode::BorrowDecode<'a, ()>>(
    bytes: &'a [u8],
) -> Result<T, ContractError> {
    if bytes.len() > crate::response::MAX_RESPONSE_BYTES {
        return Err(ProtocolError::Oversized.into());
    }
    let (value, consumed) = bincode::borrow_decode_from_slice(
        bytes,
        tutorlolv2_types::BINCODE_CONFIG.with_limit::<{ crate::response::MAX_RESPONSE_BYTES }>(),
    )
    .map_err(|_| ContractError::Protocol(ProtocolError::Malformed))?;
    if consumed != bytes.len() {
        return Err(ProtocolError::TrailingBytes.into());
    }
    Ok(value)
}
pub fn encode_bincode<T: bincode::Encode>(value: &T) -> Result<Vec<u8>, ContractError> {
    bincode::encode_to_vec(value, tutorlolv2_types::BINCODE_CONFIG)
        .map_err(|_| ContractError::Encode)
}
pub fn encode_segment(value: impl fmt::Display) -> String {
    let value = value.to_string();
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char)
            }
            _ => {
                use core::fmt::Write;
                let _ = write!(out, "%{byte:02X}");
            }
        }
    }
    out
}
