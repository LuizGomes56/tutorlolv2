use crate::server::{
    auth::DeviceAuth,
    codec::{Bincode, Response, respond},
    state::AppState,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    web::{Data, Path},
};
use tutorlolv2_protocol::SetShareMode;
use uuid::Uuid;

use crate::server::sharing::*;

pub async fn owner_share(state: Data<AppState>, auth: DeviceAuth, id: Path<Uuid>) -> Response {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        "#,
        *id,
        auth.device_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    exists.ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    respond(state.share_state(*id).await?)
}

pub async fn set_share(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> Response {
    (SharingService { state: &state })
        .change_share(auth.into(), id.into_inner(), request, false)
        .await
}

api_endpoint! {
    pub GetRecordingShare {
        handler: owner_share, method: GET, path: "/api/recordings/{id}/share",
        path_params: { id: Uuid }, query: none, body: none,
        response: envelope(tutorlolv2_protocol::ShareState), success: [200], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}

api_endpoint! {
    pub SetRecordingShare {
        handler: set_share, method: PUT, path: "/api/recordings/{id}/share",
        path_params: { id: Uuid }, query: none, body: bincode(tutorlolv2_protocol::SetShareMode),
        response: envelope(tutorlolv2_protocol::ShareState), success: [200], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
