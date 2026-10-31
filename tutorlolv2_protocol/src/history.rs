//! Additive owner-history and native IPC contracts. Existing M2 wire layouts stay unchanged.
use crate::{ChunkInfo, EndReason, Id16, RecordingSummary, ShareMode, ShareState};
use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct HistorySummary {
    pub recording: RecordingSummary,
    pub ended_at_ms: Option<i64>,
    pub end_reason: Option<EndReason>,
    pub coverage_s: u32,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u16,
    pub creep_score: u16,
    pub level: u8,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct HistoryListItem {
    pub summary: HistorySummary,
    pub share: ShareState,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct HistoryPage {
    pub items: Vec<HistoryListItem>,
    pub next_before: Option<Id16>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct RecordingDetails {
    pub summary: HistorySummary,
    pub chunks: Vec<ChunkInfo>,
    pub share: ShareState,
    pub frames_format: u16,
}

/// IPC-only subset. HTTP uses the existing SetShareMode encoding, not this enum's ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryShareMode {
    Private,
    Public,
}

impl From<HistoryShareMode> for ShareMode {
    fn from(value: HistoryShareMode) -> Self {
        match value {
            HistoryShareMode::Private => Self::Private,
            HistoryShareMode::Public => Self::Public,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub local_id: Option<Id16>,
    pub recording_id: Option<Id16>,
    pub summary: Option<HistorySummary>,
    pub share: ShareState,
    pub pending_chunks: u32,
    pub pending_control: bool,
    pub lost_chunks: u32,
    pub last_error: Option<String>,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryList {
    pub entries: Vec<HistoryEntry>,
    pub has_more: bool,
    pub refreshing: bool,
    pub refresh_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryDetails {
    pub recording: RecordingDetails,
    pub entry: HistoryEntry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryErrorKind {
    Offline,
    Unauthorized,
    NotFound,
    Conflict,
    Restricted,
    UnsupportedFormat,
    InvalidData,
    Storage,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryError {
    pub kind: HistoryErrorKind,
    pub message: String,
    pub retryable: bool,
}
