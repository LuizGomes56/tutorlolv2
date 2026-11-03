use crate::server::response::{AppError, AppResult, Trace};
use crate::server::{auth::DeviceAuth, state::AppState};
use actix_web::http::StatusCode;
use actix_web::{
    HttpResponse,
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
) -> AppResult<HttpResponse> {
    let id = id.into_inner();

    if body.len() > MAX_LIVE_BYTES {
        return Err(AppError::payload_too_large("Live snapshot too large"));
    }

    // Ownership first, before decompressing any untrusted data.
    let mut tx = state.db.begin().await.trace()?;

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
    .trace()?
    .trace_ctx(("Unknown recording", StatusCode::NOT_FOUND))?;

    if recording.ended_at.is_some() {
        return Err(AppError::conflict("The recording has ended"));
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
    .trace()?
    .is_some();

    if !shared {
        return Err(AppError::conflict("The recording is private"));
    }

    let bytes = body.to_vec();
    let derived = web::block(move || SelfContainedFrame::decode(&bytes))
        .await
        .trace()?
        .trace()?;

    let (stored, _) = bincode::decode_from_slice::<tutorlolv2_protocol::Probe, _>(
        &recording.probe,
        BINCODE_CONFIG,
    )
    .trace()?;

    if stored.consistent(&derived.probe).is_err() {
        return Err(AppError::conflict("The snapshot belongs to another match"));
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
    .trace()?;

    // Publish under the same row lock used by end/share mutations. Their
    // post-commit close cannot race ahead and leave a stale frame behind.
    state.live.publish(
        id,
        derived.captured_at,
        derived.game_time,
        derived.bytes,
        state.config.max_viewers_per_code,
    );

    tx.commit().await.trace()?;

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
