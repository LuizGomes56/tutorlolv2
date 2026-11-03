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
            tutorlolv2_types::BINCODE_CONFIG.with_limit::<MAX_RESPONSE_BYTES>(),
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
            tutorlolv2_types::BINCODE_CONFIG.with_limit::<MAX_RESPONSE_BYTES>(),
        )
        .map_err(|_| ProtocolError::Malformed)?;
        if consumed != bytes.len() {
            return Err(ProtocolError::TrailingBytes);
        }
        value.validate(http_status)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{string::String, vec};
    #[test]
    fn validates_status_and_full_bounded_decode() {
        let response = ApiResponse::new(200, Ok(7u32)).unwrap();
        let bytes = bincode::encode_to_vec(response, tutorlolv2_types::BINCODE_CONFIG).unwrap();
        assert_eq!(
            ApiResponse::<u32>::decode(&bytes, 200).unwrap().result,
            Ok(7)
        );
        assert!(ApiResponse::<u32>::decode(&bytes, 201).is_err());
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(ApiResponse::<u32>::decode(&trailing, 200).is_err());
        assert!(ApiResponse::<u32>::decode(&vec![0; MAX_RESPONSE_BYTES + 1], 200).is_err());
        assert!(ApiResponse::<u32>::new(204, Ok(7)).is_err());
        assert!(ApiResponse::<u32>::new(500, Ok(7)).is_err());
        assert!(ApiResponse::<u32>::decode(&[255], 200).is_err());
    }
    #[test]
    fn borrowing_and_allocation_limits_are_enforced() {
        let bytes = bincode::encode_to_vec(
            ApiResponse::new(200, Ok("borrowed")).unwrap(),
            tutorlolv2_types::BINCODE_CONFIG,
        )
        .unwrap();
        let response = ApiResponse::<&str>::borrow_decode(&bytes, 200).unwrap();
        let text = response.result.unwrap();
        assert_eq!(text, "borrowed");
        assert!(
            (bytes.as_ptr() as usize..bytes.as_ptr() as usize + bytes.len())
                .contains(&(text.as_ptr() as usize))
        );
        let hostile =
            bincode::encode_to_vec((200u16, 0u32, u64::MAX), tutorlolv2_types::BINCODE_CONFIG)
                .unwrap();
        assert!(ApiResponse::<String>::decode(&hostile, 200).is_err());
    }
    #[test]
    fn failure_layout_is_independent_of_success_type() {
        let error = ApiError {
            code: ErrorCode::NotFound,
            message: String::from("missing"),
            details: None,
            traces: vec![],
        };
        let bytes = bincode::encode_to_vec(
            ApiResponse::<()>::new(404, Err(error.clone())).unwrap(),
            tutorlolv2_types::BINCODE_CONFIG,
        )
        .unwrap();
        assert_eq!(
            ApiResponse::<String>::decode(&bytes, 404).unwrap().result,
            Err(error.clone())
        );
        assert_eq!(
            ApiResponse::<alloc::vec::Vec<u64>>::decode(&bytes, 404)
                .unwrap()
                .result,
            Err(error)
        );
    }
}
