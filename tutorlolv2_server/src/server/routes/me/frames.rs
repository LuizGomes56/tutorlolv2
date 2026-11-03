use crate::server::{
    codec::{OCTET_STREAM, Response},
    google::Owner,
    state::AppState,
};
use actix_web::{
    HttpResponse,
    error::{ErrorNotFound, ErrorUnprocessableEntity},
    web::{Data, Path},
};
use tutorlolv2_protocol::*;
use uuid::Uuid;

use crate::server::me::*;

pub async fn frames(state: Data<AppState>, auth: Owner, path: Path<(Uuid, u32)>) -> Response {
    let (id, seq) = path.into_inner();
    let seq = i32::try_from(seq).map_err(|_| ErrorNotFound("Chunk not found"))?;
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
    .map_err(internal)?
    .ok_or_else(|| ErrorNotFound("Chunk not found"))?;

    if row.frames_format != FRAMES_FORMAT as i16 {
        return Err(ErrorUnprocessableEntity(
            "Frames unavailable for this chunk",
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type(OCTET_STREAM)
        .insert_header(("Cache-Control", "no-store"))
        .insert_header(("Content-Encoding", "br"))
        .body(row.frames))
}

api_endpoint! {
    pub OwnerFrames {
        handler: frames, method: GET, path: "/api/me/recordings/{id}/frames/{seq}",
        path_params: { id: Uuid, seq: u32 }, query: none, body: none,
        response: brotli_bincode(tutorlolv2::riot::FrameChunk<'wire>), success: [200], auth: owner,
        request_headers: [], response_headers: ["Content-Encoding", "Cache-Control"],
    }
}
