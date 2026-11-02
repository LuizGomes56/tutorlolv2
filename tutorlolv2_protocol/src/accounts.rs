//! Additive account contracts. Session credentials stay native and never enter IPC payloads.
use alloc::string::String;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Safe account metadata returned by `GET /api/account`.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    pub created_at_ms: i64,
}

/// One-use desktop ticket. This is exchanged by native Rust with DeviceAuth.
#[derive(Clone, PartialEq, Eq, Encode, Decode)]
pub struct RedeemTicket {
    pub ticket: String,
}

/// Native-only session credential; deliberately lacks Serde and Debug implementations.
///
/// A session must not become serializable through a webview IPC response:
/// ```compile_fail
/// let session = tutorlolv2_protocol::DesktopSession {
///     token: "native-only".into(),
///     email: "player@example.test".into(),
/// };
/// let _ = serde_json::to_value(session);
/// ```
#[derive(Clone, PartialEq, Eq, Encode, Decode)]
pub struct DesktopSession {
    pub token: String,
    pub email: String,
}

/// Safe main-window account state; credentials are never returned through IPC.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountState {
    pub email: Option<String>,
    pub signing_in: bool,
    pub last_error: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountErrorKind {
    Unavailable,
    Cancelled,
    Unauthorized,
    Storage,
    InvalidData,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountError {
    pub kind: AccountErrorKind,
    pub message: String,
    pub retryable: bool,
}
