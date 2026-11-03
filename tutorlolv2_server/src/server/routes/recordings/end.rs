use crate::server::{
    auth::DeviceAuth,
    codec::{Bincode, Response},
    state::AppState,
};
use actix_web::{
    HttpResponse,
    error::{ErrorInternalServerError, ErrorUnprocessableEntity},
    web::{Data, Path},
};
use chrono::Utc;
use tutorlolv2_protocol::{EndReason, EndRecording};
use uuid::Uuid;

use crate::server::recordings::*;

pub async fn end(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<Uuid>,
    Bincode(request): Bincode<EndRecording>,
) -> Response {
    let id = path.into_inner();

    if request.reason == EndReason::Timeout {
        return Err(ErrorUnprocessableEntity(
            "Only the server ends recordings by timeout",
        ));
    }

    (RecordingService { db: &state.db })
        .owned(id, auth.device_id)
        .await?;

    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    RecordingService::end_recording_tx(&mut tx, id, request.reason, Utc::now())
        .await
        .map_err(ErrorInternalServerError)?;

    sqlx::query!(
        r#"
        UPDATE recordings
        SET RESULT = coalesce(RESULT, $2),
                     game_length_s = coalesce(game_length_s, $3)
        WHERE id = $1
        "#,
        id,
        request.result,
        request.final_game_time.map(|t| t.round() as i32),
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;
    state.live.close(id);

    Ok(HttpResponse::NoContent().finish())
}
