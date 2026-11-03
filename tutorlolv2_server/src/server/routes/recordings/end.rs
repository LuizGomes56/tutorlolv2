use crate::server::response::{AppError, AppResult, Trace};
use crate::server::{auth::DeviceAuth, codec::Bincode, state::AppState};
use actix_web::{
    HttpResponse,
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
) -> AppResult<HttpResponse> {
    let id = path.into_inner();

    if request.reason == EndReason::Timeout {
        return Err(AppError::unprocessable(
            "Only the server ends recordings by timeout",
        ));
    }

    (RecordingService { db: &state.db })
        .owned(id, auth.device_id)
        .await?;

    let mut tx = state.db.begin().await.trace()?;

    RecordingService::end_recording_tx(&mut tx, id, request.reason, Utc::now())
        .await
        .trace()?;

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
    .trace()?;

    tx.commit().await.trace()?;
    state.live.close(id);

    Ok(HttpResponse::NoContent().finish())
}

api_endpoint! {
    pub EndRecordingEndpoint {
        handler: end, method: POST, path: "/api/recordings/{id}/end",
        path_params: { id: Uuid }, query: none, body: bincode(tutorlolv2_protocol::EndRecording),
        response: empty, success: [204], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
