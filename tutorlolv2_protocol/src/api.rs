//! Request and response bodies of the recording endpoints (spec §10.2).
//! Every body is bincode with `tutorlolv2_types::BINCODE_CONFIG`.

use crate::{EndReason, GameMode, GameResult, Probe, Region, ShareMode};
use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type Id16 = [u8; 16];

/// `POST /api/devices`
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct RegisterDevice {
    pub app_version: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct DeviceCredentials {
    pub device_id: Id16,
    pub device_secret: [u8; 32],
}

/// What the League client says about the current game (whitelisted fields only).
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct LcuGame {
    pub game_id: u64,
    pub queue_id: Option<u16>,
    pub is_custom: Option<bool>,
}

/// `POST /api/recordings/sessions`
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct StartSession {
    pub probe: Probe,
    /// LCU `region-locale.region`; the server maps it with [`Region::platform`].
    pub region: Option<Region>,
    pub lcu: Option<LcuGame>,
    pub app_version: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct SessionStarted {
    pub recording_id: Id16,
    pub resumed: bool,
    pub next_chunk_seq: u32,
    pub share: ShareState,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct ShareState {
    pub mode: ShareMode,
    pub code: Option<String>,
    pub url: Option<String>,
}

/// `POST /api/recordings/{id}/end`
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct EndRecording {
    pub reason: EndReason,
    pub result: Option<GameResult>,
    pub final_game_time: Option<f32>,
}

/// `PATCH /api/recordings/{id}/result`
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct RecordingResult {
    pub result: GameResult,
    pub game_length_s: Option<u32>,
    pub riot_game: Option<LcuGame>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct SetShareMode {
    pub mode: ShareMode,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub seq: u32,
    pub first_game_time: f32,
    pub last_game_time: f32,
    pub snapshot_count: u16,
    pub frames_format: u16,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct RecordingSummary {
    pub id: Id16,
    pub champion: String,
    pub game_mode: GameMode,
    pub queue_id: Option<u16>,
    pub active_riot_id: String,
    pub started_at_ms: i64,
    pub first_game_time: f32,
    pub last_game_time: f32,
    pub game_length_s: Option<u32>,
    pub result: Option<GameResult>,
    pub is_custom: Option<bool>,
    pub is_practice: bool,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub enum ShareStatus {
    Live { last_frame_age_ms: u64 },
    HostDisconnected,
    Ended { result: Option<GameResult> },
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct ShareInfo {
    pub mode: ShareMode,
    pub status: ShareStatus,
    pub summary: RecordingSummary,
    pub chunks: Vec<ChunkInfo>,
    pub frames_format: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingStatus {
    Idle,
    Starting,
    Recording,
    Suspended,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct UploadState {
    pub pending_chunks: u32,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct RecordingState {
    pub status: RecordingStatus,
    pub recording_id: Option<Id16>,
    pub game_time: Option<f32>,
    pub share: ShareState,
    pub upload: UploadState,
    pub is_custom: Option<bool>,
    pub is_practice: bool,
}
