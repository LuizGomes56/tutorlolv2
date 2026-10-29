//! Request and response bodies of the recording endpoints (spec §10.2).
//! Every body is bincode with `tutorlolv2_types::BINCODE_CONFIG`.

use crate::{EndReason, GameResult, Probe, Region, ShareMode};
use alloc::string::String;
use bincode::{Decode, Encode};

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

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
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
