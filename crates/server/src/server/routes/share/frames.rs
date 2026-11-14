use crate::server::recordings::RecordingService;
use crate::server::response::{AppResult, Trace};
use crate::server::sharing::*;
use crate::server::{codec::OCTET_STREAM, state::AppState};
use actix_web::http::StatusCode;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Path},
};
use protocol::CodeMode;

pub async fn frames(
    state: Data<AppState>,
    path: Path<(String, u32)>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let (code, seq) = path.into_inner();
    let (code, cap) = state.capability(&code, &req).await.trace()?;
    let seq = i32::try_from(seq).trace_ctx(("Chunk not found", StatusCode::NOT_FOUND))?;

    let chunk = sqlx::query!(
        r#"
        SELECT frames,
               frames_format
        FROM recording_chunks
        WHERE recording_id = $1
            AND seq = $2
        "#,
        cap.recording,
        seq,
    )
    .fetch_optional(&state.db)
    .await
    .trace()?
    .trace_ctx(("Chunk not found", StatusCode::NOT_FOUND))?;

    let frames = (RecordingService { db: &state.db })
        .current_frames(cap.recording, seq, chunk.frames, chunk.frames_format)
        .await?;

    let latest = Capability::load(&state.db, &code).await.trace()?;

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Content-Encoding", "br"))
        .insert_header((
            "Cache-Control",
            if latest.mode == CodeMode::Live {
                "no-store"
            } else {
                "private, max-age=300"
            },
        ))
        .body(frames))
}

api_endpoint! {
    pub ShareFrames {
        handler: frames, method: GET, path: "/api/share/{code}/frames/{seq}",
        path_params: { code: String, seq: u32 }, query: none, body: none,
        response: brotli_bincode(math::riot::FrameChunk<'wire>), success: [200], auth: capability,
        request_headers: [], response_headers: ["Content-Encoding", "Cache-Control"],
    }
}
