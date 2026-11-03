use crate::server::{
    auth::DeviceAuth,
    codec::{Bincode, Response},
    db_enums::RiotGameSource,
    state::AppState,
};
use actix_web::{
    HttpResponse,
    error::ErrorInternalServerError,
    web::{Data, Path},
};
use tutorlolv2_protocol::RecordingResult;
use uuid::Uuid;

use crate::server::recordings::*;

pub async fn set_result(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<Uuid>,
    Bincode(request): Bincode<RecordingResult>,
) -> Response {
    let id = path.into_inner();

    (RecordingService { db: &state.db })
        .owned(id, auth.device_id)
        .await?;

    let lcu = request.riot_game.as_ref();

    sqlx::query!(
        r#"
        UPDATE recordings
        SET RESULT = $2,
                     game_length_s = coalesce($3, game_length_s),
                     riot_game_id = CASE
                                        WHEN $4::bigint IS NOT NULL
                                             AND riot_game_source IS DISTINCT
                                             FROM $7 THEN $4
                                        ELSE riot_game_id
                                    END,
                                    queue_id = CASE
                                                   WHEN $4::bigint IS NOT NULL
                                                        AND riot_game_source IS DISTINCT
                                                        FROM $7 THEN coalesce($5, queue_id)
                                                   ELSE queue_id
                                               END,
                                               is_custom = CASE
                                                               WHEN $4::bigint IS NOT NULL
                                                                    AND riot_game_source IS DISTINCT
                                                                    FROM $7 THEN coalesce($6, is_custom)
                                                               ELSE is_custom
                                                           END,
                                                           riot_game_source = CASE
                                                                                  WHEN $4::bigint IS NOT NULL
                                                                                       AND riot_game_source IS DISTINCT
                                                                                       FROM $7 THEN $8
                                                                                  ELSE riot_game_source
                                                                              END
        WHERE id = $1
        "#,
        id,
        request.result,
        request.game_length_s.map(|s| s as i32),
        lcu.map(|l| l.game_id as i64),
        lcu.and_then(|l| l.queue_id).map(i32::from),
        lcu.and_then(|l| l.is_custom),
        RiotGameSource::Spectator,
        RiotGameSource::Lcu,
    )
    .execute(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

api_endpoint! {
    pub SetResult {
        handler: set_result, method: PATCH, path: "/api/recordings/{id}/result",
        path_params: { id: Uuid }, query: none, body: bincode(tutorlolv2_protocol::RecordingResult),
        response: empty, success: [204], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
