//! Replay chunks and frame selection: the web app's `replay/cache.rs`.
//!
//! The web keeps each decoded chunk next to its buffer with `self_cell`. Here a chunk keeps its
//! bytes and the sorted game times of its frames; a frame is decoded again when it is shown
//! (decoding a chunk takes well under a millisecond). Chunks are `Send`, so frames can be
//! calculated on a background thread.
use crate::backend::BINCODE_CONFIG;
use std::{collections::BTreeMap, sync::Arc};
use tutorlolv2::riot::{FrameChunk, RiotRealtime};
use tutorlolv2_protocol::{ChunkInfo, FRAMES_FORMAT};

pub struct CachedChunk {
    bytes: Vec<u8>,
    /// Frame indices sorted by game time, then capture time.
    order: Vec<usize>,
    /// Game time of each frame, in `order`.
    pub times: Vec<f32>,
}

impl CachedChunk {
    pub fn decode(bytes: Vec<u8>, format: u16) -> Result<Arc<Self>, String> {
        let (order, times) = {
            let chunk = Self::parse(&bytes, format)?;
            let mut order = (0..chunk.frames.len()).collect::<Vec<_>>();

            // The recorder allows up to two seconds of game-time jitter. Normalize the order
            // while keeping every captured sample.
            order.sort_by(|&a, &b| {
                let (captured_a, frame_a) = &chunk.frames[a];
                let (captured_b, frame_b) = &chunk.frames[b];

                frame_a
                    .game_data
                    .game_time
                    .total_cmp(&frame_b.game_data.game_time)
                    .then(captured_a.cmp(captured_b))
            });

            let times = order
                .iter()
                .map(|&i| chunk.frames[i].1.game_data.game_time)
                .collect();

            (order, times)
        };

        Ok(Arc::new(Self {
            bytes,
            order,
            times,
        }))
    }

    fn parse(bytes: &[u8], format: u16) -> Result<FrameChunk<'_>, String> {
        let (chunk, consumed): (FrameChunk<'_>, _) =
            bincode::borrow_decode_from_slice(bytes, BINCODE_CONFIG).map_err(|e| e.to_string())?;

        if format != FRAMES_FORMAT || consumed != bytes.len() || chunk.format != format {
            return Err("Unsupported or damaged replay chunk. Try refreshing.".into());
        }

        Ok(chunk)
    }

    pub fn validate(&self, metadata: &ChunkInfo) -> Result<(), String> {
        if metadata.frames_format != FRAMES_FORMAT
            || self.times.len() != usize::from(metadata.snapshot_count)
            || self.times.iter().any(|time| !time.is_finite())
        {
            return Err("Invalid replay chunk metadata.".into());
        }

        Ok(())
    }

    /// Every frame with its capture time, in game time order.
    pub fn frames(&self) -> Vec<(i64, RiotRealtime<'_>)> {
        let Ok(chunk) = Self::parse(&self.bytes, FRAMES_FORMAT) else {
            return Vec::new();
        };

        let mut frames = chunk.frames.into_iter().map(Some).collect::<Vec<_>>();

        self.order
            .iter()
            .filter_map(|&i| frames.get_mut(i).and_then(Option::take))
            .collect()
    }

    /// The frame at `index` of the sorted frames.
    pub fn frame(&self, index: usize) -> Option<RiotRealtime<'_>> {
        let original = *self.order.get(index)?;
        let chunk = Self::parse(&self.bytes, FRAMES_FORMAT).ok()?;

        chunk
            .frames
            .into_iter()
            .nth(original)
            .map(|(_, frame)| frame)
    }
}

pub type ChunkCache = BTreeMap<u32, Arc<CachedChunk>>;

/// Keeps only the chunks around `time`, so memory stays bounded while seeking.
pub fn retain_neighbors(cache: &mut ChunkCache, chunks: &[ChunkInfo], time: f32) {
    let needed = super::timeline::neighbors(chunks, time);
    cache.retain(|seq, _| needed.contains(seq));
}

/// Return sample-and-hold only when the next captured sample is adjacent.
pub fn sample_index(times: &[f32], time: f32) -> Option<usize> {
    let index = times.iter().rposition(|value| *value <= time)?;

    if times[index] == time {
        return Some(index);
    }

    let next = *times.get(index + 1)?;
    (next - times[index] <= 2. && time < next).then_some(index)
}

pub fn selected_index(times: &[f32], time: f32, next_chunk: Option<f32>) -> Option<usize> {
    sample_index(times, time).or_else(|| {
        let index = times.len().checked_sub(1)?;
        let last = times[index];

        (time >= last && next_chunk.is_some_and(|next| next - last <= 2.0 && time < next))
            .then_some(index)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> RiotRealtime<'static> {
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/example.bincode"
        ));
        bincode::borrow_decode_from_slice(bytes, BINCODE_CONFIG)
            .unwrap()
            .0
    }

    #[test]
    fn sample_hold_requires_a_known_adjacent_sample() {
        assert_eq!(sample_index(&[1., 2., 10.], 1.5), Some(0));
        assert_eq!(sample_index(&[1., 2., 10.], 3.), None);
        assert_eq!(sample_index(&[1., 2.], 2.5), None);
        assert_eq!(sample_index(&[1., 2.], 0.), None);
    }

    #[test]
    fn accepted_capture_jitter_is_normalized_not_rejected() {
        let frame = fixture();
        let frames = [100., 102., 101., 100., 99.]
            .into_iter()
            .enumerate()
            .map(|(index, time)| {
                let mut frame = frame.clone();
                frame.game_data.game_time = time;
                (index as i64, frame)
            })
            .collect();
        let chunk = FrameChunk {
            format: FRAMES_FORMAT,
            frames,
        };
        let cached = CachedChunk::decode(
            bincode::encode_to_vec(chunk, BINCODE_CONFIG).unwrap(),
            FRAMES_FORMAT,
        )
        .unwrap();
        let metadata = ChunkInfo {
            seq: 0,
            first_game_time: 100.,
            last_game_time: 99.,
            snapshot_count: 5,
            frames_format: FRAMES_FORMAT,
        };

        cached.validate(&metadata).unwrap();
        assert_eq!(cached.times, vec![99., 100., 100., 101., 102.]);
        assert_eq!(selected_index(&cached.times, 100.75, None), Some(2));

        // Decoded frames follow the sorted order, and the later capture wins ties.
        let frames = cached.frames();
        assert_eq!(
            frames.iter().map(|f| f.0).collect::<Vec<_>>(),
            [4, 0, 3, 2, 1]
        );
        assert_eq!(cached.frame(4).unwrap().game_data.game_time, 102.);

        let mut store = super::super::extent::ExtentStore::default();
        store.observe_chunk(&metadata, &cached);
        let (ranges, end) = store.timeline(&[metadata], 99.);
        assert_eq!(end, 102.);
        assert_eq!(
            super::super::timeline::current_index(&ranges, 102.),
            Some(0)
        );
    }
}
