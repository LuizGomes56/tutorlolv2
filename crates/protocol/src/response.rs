use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};
use core::fmt;
use serde::{Deserialize, Serialize};

/// Maximum encoded envelope and allocation budget accepted by shared decoders.
pub const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub result: Result<T, ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
    pub traces: Vec<ErrorTrace>,
}

/// Variant order is part of the bincode wire contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub enum ErrorCode {
    InvalidRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Gone,
    Conflict,
    PayloadTooLarge,
    UnsupportedFormat,
    RateLimited,
    Upstream,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct ErrorTrace {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    Oversized,
    Malformed,
    TrailingBytes,
    StatusMismatch { http: u16, envelope: u16 },
    InvalidStatus,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API protocol error: {self:?}")
    }
}

impl core::error::Error for ProtocolError {}

/// Transport remains platform-owned; remote failures retain their status and traces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError<T> {
    Transport(T),
    Protocol(ProtocolError),
    Remote { status: u16, error: ApiError },
}

impl<T: fmt::Display> fmt::Display for ClientError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(e) => write!(f, "Transport error: {e}"),
            Self::Protocol(e) => e.fmt(f),
            Self::Remote { status, error } => write!(f, "HTTP {status}: {}", error.message),
        }
    }
}

impl<T: core::error::Error + 'static> core::error::Error for ClientError<T> {}

impl<T> ApiResponse<T> {
    pub fn new(status: u16, result: Result<T, ApiError>) -> Result<Self, ProtocolError> {
        let value = Self { status, result };
        value.validate(status)?;
        Ok(value)
    }

    pub fn validate(&self, http_status: u16) -> Result<(), ProtocolError> {
        if self.status != http_status {
            return Err(ProtocolError::StatusMismatch {
                http: http_status,
                envelope: self.status,
            });
        }

        // These statuses cannot carry an envelope body.
        let valid = match self.result {
            Ok(_) => (200..300).contains(&self.status) && self.status != 204 && self.status != 205,
            Err(_) => (400..600).contains(&self.status),
        };

        if valid {
            Ok(())
        } else {
            Err(ProtocolError::InvalidStatus)
        }
    }

    pub fn into_result<E>(self) -> Result<T, ClientError<E>> {
        self.validate(self.status).map_err(ClientError::Protocol)?;
        self.result.map_err(|error| ClientError::Remote {
            status: self.status,
            error,
        })
    }
}

impl<T: Decode<()>> ApiResponse<T> {
    pub fn decode(bytes: &[u8], http_status: u16) -> Result<Self, ProtocolError> {
        if bytes.len() > MAX_RESPONSE_BYTES {
            return Err(ProtocolError::Oversized);
        }

        let (value, consumed): (Self, _) = bincode::decode_from_slice(
            bytes,
            types::BINCODE_CONFIG.with_limit::<MAX_RESPONSE_BYTES>(),
        )
        .map_err(|_| ProtocolError::Malformed)?;

        if consumed != bytes.len() {
            return Err(ProtocolError::TrailingBytes);
        }

        value.validate(http_status)?;
        Ok(value)
    }
}

impl<'de, T: bincode::BorrowDecode<'de, ()>> ApiResponse<T> {
    pub fn borrow_decode(bytes: &'de [u8], http_status: u16) -> Result<Self, ProtocolError> {
        if bytes.len() > MAX_RESPONSE_BYTES {
            return Err(ProtocolError::Oversized);
        }

        let (value, consumed): (Self, _) = bincode::borrow_decode_from_slice(
            bytes,
            types::BINCODE_CONFIG.with_limit::<MAX_RESPONSE_BYTES>(),
        )
        .map_err(|_| ProtocolError::Malformed)?;

        if consumed != bytes.len() {
            return Err(ProtocolError::TrailingBytes);
        }

        value.validate(http_status)?;
        Ok(value)
    }
}
