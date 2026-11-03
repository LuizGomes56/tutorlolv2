use crate::server::{
    codec::{OCTET_STREAM, Response},
    state::AppState,
};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity},
    web::{Data, Path},
};
use tutorlolv2_protocol::{CodeMode, FRAMES_FORMAT};

use crate::server::sharing::*;

pub async fn frames(
    state: Data<AppState>,
    path: Path<(String, u32)>,
    req: HttpRequest,
) -> Response {
    let (code, seq) = path.into_inner();
    let (code, cap) = state.capability(&code, &req).await?;

    let chunk = sqlx::query!(
        r#"
        SELECT frames,
               frames_format
        FROM recording_chunks
        WHERE recording_id = $1
            AND seq = $2
        "#,
        cap.recording,
        i32::try_from(seq).map_err(|_| ErrorNotFound("Chunk not found"))?,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Chunk not found"))?;

    if chunk.frames_format != FRAMES_FORMAT as i16 {
        return Err(ErrorUnprocessableEntity(
            "Frames unavailable for this chunk",
        ));
    }

    let latest = Capability::load(&state.db, &code).await?;

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
        .body(chunk.frames))
}

api_endpoint! {
    pub ShareFrames {
        handler: frames, method: GET, path: "/api/share/{code}/frames/{seq}",
        path_params: { code: String, seq: u32 }, query: none, body: none,
        response: brotli_bincode(tutorlolv2::riot::FrameChunk<'wire>), success: [200], auth: capability,
        request_headers: [], response_headers: ["Content-Encoding", "Cache-Control"],
    }
}
