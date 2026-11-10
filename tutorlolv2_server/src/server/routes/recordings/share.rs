use crate::server::endpoint::EndpointExt;
use crate::server::response::Trace;
use crate::server::{auth::DeviceAuth, codec::Bincode, state::AppState};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use tutorlolv2_protocol::SetShareMode;
use uuid::Uuid;

use crate::server::sharing::*;

pub async fn owner_share(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
) -> crate::server::response::ApiResult<tutorlolv2_protocol::ShareState> {
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
    .trace()?;

    exists.trace_ctx(("Unknown recording", StatusCode::NOT_FOUND))?;

    Ok(tutorlolv2_api::GetRecordingShare::reply(
        state.share_state(*id).await.trace()?,
    ))
}

pub async fn set_share(
    state: Data<AppState>,
    auth: DeviceAuth,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> crate::server::response::ApiResult<tutorlolv2_protocol::ShareState> {
    (SharingService { state: &state })
        .change_share(auth.into(), id.into_inner(), request, false)
        .await
        .trace()
        .map(tutorlolv2_api::SetRecordingShare::reply)
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
