use crate::server::recordings::RecordingService;
use crate::server::response::{AppResult, Trace};
use crate::server::{codec::OCTET_STREAM, google::Owner, state::AppState};
use actix_web::http::StatusCode;
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};
use uuid::Uuid;

pub async fn frames(
    state: Data<AppState>,
    auth: Owner,
    path: Path<(Uuid, u32)>,
) -> AppResult<HttpResponse> {
    let (id, seq) = path.into_inner();
    let seq = i32::try_from(seq).trace_ctx(("Chunk not found", StatusCode::NOT_FOUND))?;
    let row = sqlx::query!(
        r#"
        SELECT c.frames,
               c.frames_format
        FROM recording_chunks c
        JOIN recordings r ON r.id = c.recording_id
        WHERE ((r.device_id = $1
                AND $1 IS NOT NULL)
               OR (r.account_id = $4
                   AND $4 IS NOT NULL))
            AND r.deleted_at IS NULL
            AND EXISTS
                (SELECT 1
                 FROM devices d
                 WHERE d.id = r.device_id
                     AND d.deleted_at IS NULL)
            AND r.id = $2
            AND c.seq = $3
        "#,
        auth.device_id,
        id,
        seq,
        auth.account_id,
    )
    .fetch_optional(&state.db)
    .await
    .trace_ctx("History storage unavailable")?
    .trace_ctx(("Chunk not found", StatusCode::NOT_FOUND))?;

    let frames = (RecordingService { db: &state.db })
        .current_frames(id, seq, row.frames, row.frames_format)
        .await?;

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Cache-Control", "no-store"))
        .insert_header(("Content-Encoding", "br"))
        .body(frames))
}

api_endpoint! {
    pub OwnerFrames {
        handler: frames, method: GET, path: "/api/me/recordings/{id}/frames/{seq}",
        path_params: { id: Uuid, seq: u32 }, query: none, body: none,
        response: brotli_bincode(tutorlolv2::riot::FrameChunk<'wire>), success: [200], auth: owner,
        request_headers: [], response_headers: ["Content-Encoding", "Cache-Control"],
    }
}
