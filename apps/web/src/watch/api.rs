pub use crate::replay::cache::CachedChunk;

use crate::utils::api::{FetchError, WebClient};
use ::api::{
    After, NoBody, NoHeaders, NoQuery, PollLive, PollLivePath, ResponseBuffer, ShareFrames,
    ShareFramesPath, ShareInfoEndpoint, ShareInfoEndpointPath,
};
use protocol::{FRAMES_FORMAT, ShareInfo};
use web_sys::AbortController;

#[cfg(test)]
use {crate::utils::BINCODE_CONFIG, math::riot::RiotRealtime};

pub struct WatchApi;
impl WatchApi {
    pub async fn info(code: &str, abort: &AbortController) -> Result<ShareInfo, String> {
        let response = WebClient::public()
            .send::<ShareInfoEndpoint>(
                &ShareInfoEndpointPath { code: code.into() },
                &NoQuery,
                &NoHeaders,
                &NoBody,
                Some(abort),
            )
            .await
            .map_err(FetchError::message)?;

        let mut value = response
            .decode()
            .map_err(|e| FetchError::response(response.status(), e).message())?;

        if value.frames_format != FRAMES_FORMAT {
            return Err("This replay needs a newer viewer. Refresh the app and retry.".into());
        }

        value
            .chunks
            .sort_by(|a, b| a.first_game_time.total_cmp(&b.first_game_time));

        Ok(value)
    }

    pub async fn poll(
        code: &str,
        after: u64,
        abort: &AbortController,
    ) -> Result<ResponseBuffer<PollLive>, String> {
        WebClient::public()
            .send::<PollLive>(
                &PollLivePath { code: code.into() },
                &After { after },
                &NoHeaders,
                &NoBody,
                Some(abort),
            )
            .await
            .map_err(FetchError::message)
    }

    pub async fn frames(code: &str, seq: u32, abort: &AbortController) -> Result<Vec<u8>, String> {
        let response = WebClient::public()
            .send::<ShareFrames>(
                &ShareFramesPath {
                    code: code.into(),
                    seq,
                },
                &NoQuery,
                &NoHeaders,
                &NoBody,
                Some(abort),
            )
            .await
            .map_err(FetchError::message)?;

        response
            .decode()
            .map_err(|e| FetchError::response(response.status(), e).message())?;

        Ok(response.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use math::riot::FrameChunk;

    #[test]
    fn borrowed_chunk_keeps_its_buffer_and_rejects_wrong_format_or_trailing_bytes() {
        let fixture = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/example.bincode"));
        let (frame, _): (RiotRealtime<'_>, _) =
            bincode::borrow_decode_from_slice(fixture, BINCODE_CONFIG).unwrap();
        let chunk = FrameChunk {
            format: 1,
            frames: vec![(123, frame)],
        };
        let bytes = bincode::encode_to_vec(&chunk, BINCODE_CONFIG).unwrap();
        let cached = CachedChunk::decode(bytes.clone(), 1).unwrap();
        assert_eq!(cached.borrow_dependent().frames[0].0, 123);
        assert_eq!(cached.borrow_dependent().frames[0].1, chunk.frames[0].1);
        assert!(CachedChunk::decode(bytes.clone(), 2).is_err());
        let mut trailing = bytes;
        trailing.push(0);
        assert!(CachedChunk::decode(trailing, 1).is_err());
    }
    #[test]
    fn code_entry_uses_shared_normalization() {
        assert_eq!(
            protocol::normalize_code("abcd efgh"),
            Some("ABCDEFGH".into())
        );
        assert_eq!(
            protocol::normalize_code("abcd-efgh"),
            Some("ABCDEFGH".into())
        );
        assert_eq!(protocol::normalize_code("bad"), None);
    }
}
