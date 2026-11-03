use crate::server::{auth::DeviceAuth, codec::Response, state::AppState};
use actix_web::{
    HttpResponse,
    error::{ErrorConflict, ErrorInternalServerError},
    web::{Data, Path},
};
use tutorlolv2_protocol::EndReason;
use uuid::Uuid;

use crate::server::recordings::*;

pub async fn heartbeat(state: Data<AppState>, auth: DeviceAuth, path: Path<Uuid>) -> Response {
    let id = path.into_inner();

    // One statement, so an end committing meanwhile is re-checked by Postgres
    // instead of being overwritten
    let alive = sqlx::query!(
        r#"
        UPDATE recordings
        SET last_ingest_at = now(),
            ended_at = NULL,
            end_reason = NULL
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
            AND (ended_at IS NULL
                 OR (end_reason = $3
                     AND NOT EXISTS
                         (SELECT 1
                          FROM recordings other
                          WHERE other.device_id = $2
                              AND other.id <> $1
                              AND other.ended_at IS NULL
                              AND other.deleted_at IS NULL)))
        "#,
        id,
        auth.device_id,
        EndReason::Timeout,
    )
    .execute(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .rows_affected()
        == 1;

    if alive {
        return Ok(HttpResponse::NoContent().finish());
    }

    // Explain why: unknown recording, a real end, or another open recording
    let recording = (RecordingService { db: &state.db })
        .owned(id, auth.device_id)
        .await?;

    if recording.end_reason == Some(EndReason::Timeout) {
        Err(ErrorConflict("Another recording of this device is open"))
    } else {
        Err(ErrorConflict("The recording has ended"))
    }
}

api_endpoint! {
    pub Heartbeat {
        handler: heartbeat, method: POST, path: "/api/recordings/{id}/heartbeat",
        path_params: { id: Uuid }, query: none, body: none,
        response: empty, success: [204], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
