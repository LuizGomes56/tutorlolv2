use crate::server::{auth::DeviceAuth, codec::Response, state::AppState};
use actix_web::{
    HttpResponse,
    error::{
        ErrorConflict, ErrorInternalServerError, ErrorNotFound, ErrorPayloadTooLarge,
        ErrorUnprocessableEntity,
    },
    web::{self, Bytes, Data, Path},
};
use tutorlolv2_types::BINCODE_CONFIG;
use uuid::Uuid;

use crate::server::sharing::*;

pub async fn push_live(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
    body: Bytes,
) -> Response {
    let id = id.into_inner();

    if body.len() > MAX_LIVE_BYTES {
        return Err(ErrorPayloadTooLarge("Live snapshot too large"));
    }

    // Ownership first, before decompressing any untrusted data.
    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    let recording = sqlx::query!(
        r#"
        SELECT ended_at,
               probe
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
            FOR
            UPDATE
        "#,
        id,
        auth.device_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    if recording.ended_at.is_some() {
        return Err(ErrorConflict("The recording has ended"));
    }

    let shared = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM share_codes
        WHERE recording_id = $1
            AND deleted_at IS NULL
        "#,
        id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .is_some();

    if !shared {
        return Err(ErrorConflict("The recording is private"));
    }

    let bytes = body.to_vec();
    let derived = web::block(move || SelfContainedFrame::decode(&bytes))
        .await
        .map_err(ErrorInternalServerError)?
        .map_err(|e| {
            if e.too_large {
                ErrorPayloadTooLarge(e.message)
            } else {
                ErrorUnprocessableEntity(e.message)
            }
        })?;

    let (stored, _) = bincode::decode_from_slice::<tutorlolv2_protocol::Probe, _>(
        &recording.probe,
        BINCODE_CONFIG,
    )
    .map_err(ErrorInternalServerError)?;

    if stored.consistent(&derived.probe).is_err() {
        return Err(ErrorConflict("The snapshot belongs to another match"));
    }

    sqlx::query!(
        r#"
        UPDATE recordings
        SET last_ingest_at = now()
        WHERE id = $1
        "#,
        id,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    // Publish under the same row lock used by end/share mutations. Their
    // post-commit close cannot race ahead and leave a stale frame behind.
    state.live.publish(
        id,
        derived.captured_at,
        derived.game_time,
        derived.bytes,
        state.config.max_viewers_per_code,
    );

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

api_endpoint! {
    pub PushLive {
        handler: push_live, method: PUT, path: "/api/recordings/{id}/live",
        path_params: { id: Uuid }, query: none, body: brotli_bincode(tutorlolv2_protocol::RawSnapshot),
        response: empty, success: [204], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
