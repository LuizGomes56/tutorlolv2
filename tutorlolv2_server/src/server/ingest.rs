//! Validation of uploaded chunks and derivation of viewer frames (spec §10.3).
//! CPU-bound: call from `web::block`.

use bincode::{BorrowDecode, Encode};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use tutorlolv2::riot::RiotRealtime;
use tutorlolv2_protocol::{
    BROTLI_LGWIN, BROTLI_QUALITY, FRAMES_FORMAT, MAX_CHUNK_DECOMPRESSED, RawChunk, Snapshot,
    inspect,
};

/// Server-derived frames of one chunk, stored brotli-compressed and served
/// with `Content-Encoding: br`.
#[derive(Encode, BorrowDecode)]
pub struct FrameChunk<'a> {
    pub format: u16,
    /// `(captured_at_ms, frame)`
    pub frames: Vec<(i64, RiotRealtime<'a>)>,
}

#[derive(Debug)]
pub enum IngestError {
    /// 413
    TooLarge,
    /// 422
    Invalid(String),
    /// 409
    Mismatch(String),
}

/// What a valid chunk contributes to its recording.
#[derive(Debug)]
pub struct Derived {
    pub raw_sha256: [u8; 32],
    /// Uncompressed bincode bytes.
    pub raw_size: usize,
    pub snapshot_count: u16,
    pub first_game_time: f32,
    pub last_game_time: f32,
    pub first: Snapshot,
    pub last: Snapshot,
    /// [`FRAMES_FORMAT`], or [`NO_FRAMES`] when they couldn't be derived.
    pub frames_format: u16,
    /// Empty when `frames_format` is [`NO_FRAMES`].
    pub frames_br: Vec<u8>,
}

/// `frames_format` of a chunk whose raw data is stored but whose frames
/// couldn't be derived; they can be regenerated later from the raw chunk (D18).
pub const NO_FRAMES: u16 = 0;

/// Lowercase hex, the format of `X-Content-Sha256`.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn brotli_compress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() / 8);

    {
        let mut writer =
            brotli::CompressorWriter::new(&mut out, 4096, BROTLI_QUALITY, BROTLI_LGWIN);

        writer
            .write_all(data)
            .expect("writing into a Vec cannot fail");
    }

    out
}

fn invalid(e: impl std::fmt::Display) -> IngestError {
    IngestError::Invalid(e.to_string())
}

pub fn derive_chunk(
    body: &[u8],
    expected_seq: u32,
    header_sha_hex: &str,
) -> Result<Derived, IngestError> {
    let mut raw = Vec::new();

    brotli::Decompressor::new(body, 4096)
        .take(MAX_CHUNK_DECOMPRESSED as u64 + 1)
        .read_to_end(&mut raw)
        .map_err(invalid)?;

    if raw.len() > MAX_CHUNK_DECOMPRESSED {
        return Err(IngestError::TooLarge);
    }

    let raw_sha256: [u8; 32] = Sha256::digest(&raw).into();

    if !header_sha_hex.eq_ignore_ascii_case(&hex(&raw_sha256)) {
        return Err(invalid("X-Content-Sha256 doesn't match the chunk"));
    }

    let config = bincode::config::standard().with_limit::<MAX_CHUNK_DECOMPRESSED>();
    let (chunk, used): (RawChunk, _) = bincode::decode_from_slice(&raw, config).map_err(invalid)?;

    if used != raw.len() {
        return Err(invalid("Unexpected trailing chunk bytes"));
    }

    chunk
        .validate(expected_seq)
        .map_err(|e| invalid(format!("{e:?}")))?;

    let snapshots = chunk
        .snapshots
        .iter()
        .map(|s| inspect(&s.json).map_err(|e| invalid(format!("{e:?}"))))
        .collect::<Result<Vec<Snapshot>, _>>()?;

    let first = snapshots
        .first()
        .expect("RawChunk::validate rejects empty chunks")
        .clone();

    let last = snapshots
        .last()
        .expect("RawChunk::validate rejects empty chunks")
        .clone();

    if snapshots
        .iter()
        .any(|s| s.probe.active_riot_id != first.probe.active_riot_id)
    {
        return Err(IngestError::Mismatch(
            "The active player changed inside the chunk".into(),
        ));
    }

    let (frames_format, frames_br) = match derive_frames(&chunk) {
        Some(frames) => (FRAMES_FORMAT, brotli_compress(&frames)),
        None => (NO_FRAMES, Vec::new()),
    };

    Ok(Derived {
        raw_sha256,
        raw_size: raw.len(),
        snapshot_count: snapshots.len() as u16,
        first_game_time: first.game_time,
        last_game_time: last.game_time,
        first,
        last,
        frames_format,
        frames_br,
    })
}

/// Bincode `FrameChunk`, best effort: raw data is never rejected because of
/// frames (D18). `RiotRealtime` borrows its strings, so JSON with `\u`
/// escapes is first re-encoded as plain UTF-8.
fn derive_frames(chunk: &RawChunk) -> Option<Vec<u8>> {
    fn encode(frames: Vec<(i64, RiotRealtime<'_>)>) -> Option<Vec<u8>> {
        bincode::encode_to_vec(
            FrameChunk {
                format: FRAMES_FORMAT,
                frames,
            },
            bincode::config::standard(),
        )
        .ok()
    }

    let direct = chunk
        .snapshots
        .iter()
        .map(|s| serde_json::from_slice(&s.json).map(|frame| (s.captured_at_ms, frame)))
        .collect::<Result<Vec<_>, _>>();

    if let Ok(frames) = direct {
        return encode(frames);
    }

    let plain = chunk
        .snapshots
        .iter()
        .map(|s| {
            serde_json::from_slice::<serde_json::Value>(&s.json)
                .ok()
                .and_then(|v| serde_json::to_vec(&v).ok())
        })
        .collect::<Option<Vec<Vec<u8>>>>()?;

    let frames = chunk
        .snapshots
        .iter()
        .zip(&plain)
        .map(|(s, json)| serde_json::from_slice(json).map(|frame| (s.captured_at_ms, frame)))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    encode(frames)
}
