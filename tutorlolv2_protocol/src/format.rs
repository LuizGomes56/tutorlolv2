//! Wire formats of the recording uploads (spec §6.3).

use alloc::vec::Vec;
use bincode::{Decode, Encode};

pub const RAW_CHUNK_FORMAT: u16 = 1;
pub const FRAMES_FORMAT: u16 = 1;
pub const MAX_SNAPSHOTS_PER_CHUNK: usize = 120;
/// A chunk is sealed once its first snapshot is this old (wall clock).
pub const CHUNK_SEAL_MS: i64 = 60_000;
/// Largest accepted chunk upload body (compressed).
pub const MAX_CHUNK_BYTES: usize = 2 << 20;
/// Largest accepted chunk after decompression (zip-bomb guard).
pub const MAX_CHUNK_DECOMPRESSED: usize = 16 << 20;
pub const BROTLI_QUALITY: u32 = 5;
/// 16 MiB window, so every snapshot can reference the previous one.
pub const BROTLI_LGWIN: u32 = 24;
/// `gameTime` may jitter backwards this much inside one chunk (seconds).
const GAME_TIME_TOLERANCE: f32 = 2.0;

/// One verbatim response of `GET /liveclientdata/allgamedata`.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct RawSnapshot {
    /// Unix ms on the host's clock. Informational only: the server keeps its own times.
    pub captured_at_ms: i64,
    /// Copy of `gameData.gameTime`, so indexes need no JSON parse.
    pub game_time: f32,
    pub json: Vec<u8>,
}

/// A sealed batch of consecutive snapshots of one recording. The recording id
/// travels in the upload URL, not here.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct RawChunk {
    pub format: u16,
    /// Must equal the `{seq}` of the upload URL.
    pub seq: u32,
    pub snapshots: Vec<RawSnapshot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkError {
    Format(u16),
    SeqMismatch {
        expected: u32,
        got: u32,
    },
    Empty,
    TooManySnapshots(usize),
    /// Index of the first snapshot whose `captured_at_ms` isn't greater than the previous one.
    CapturedAtNotIncreasing(usize),
    /// Index of the first snapshot whose `game_time` dropped more than the tolerance.
    GameTimeWentBack(usize),
}

impl RawChunk {
    pub fn validate(&self, expected_seq: u32) -> Result<(), ChunkError> {
        if self.format != RAW_CHUNK_FORMAT {
            return Err(ChunkError::Format(self.format));
        }

        if self.seq != expected_seq {
            return Err(ChunkError::SeqMismatch {
                expected: expected_seq,
                got: self.seq,
            });
        }

        match self.snapshots.len() {
            0 => return Err(ChunkError::Empty),
            n if n > MAX_SNAPSHOTS_PER_CHUNK => return Err(ChunkError::TooManySnapshots(n)),
            _ => {}
        }

        for (i, pair) in self.snapshots.windows(2).enumerate() {
            if pair[1].captured_at_ms <= pair[0].captured_at_ms {
                return Err(ChunkError::CapturedAtNotIncreasing(i + 1));
            }

            if pair[1].game_time < pair[0].game_time - GAME_TIME_TOLERANCE {
                return Err(ChunkError::GameTimeWentBack(i + 1));
            }
        }

        Ok(())
    }
}
