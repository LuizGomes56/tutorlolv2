use crate::server::endpoint::EndpointExt;
use crate::server::response::Trace;
use crate::server::{codec::Bincode, google::Owner, sharing, state::AppState};
use actix_web::web::{Data, Path};
use tutorlolv2_protocol::*;
use uuid::Uuid;

pub async fn set_share(
    state: Data<AppState>,
    auth: Owner,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> crate::server::response::ApiResult<ShareState> {
    let share = (sharing::SharingService { state: &state })
        .change_share(auth, id.into_inner(), request, true)
        .await
        .trace()?;
    Ok(tutorlolv2_api::SetHistoryShare::reply(share).no_store())
}

api_endpoint! {
    pub SetHistoryShare {
        handler: set_share, method: PUT, path: "/api/me/recordings/{id}/share",
        path_params: { id: Uuid }, query: none, body: bincode(tutorlolv2_protocol::SetShareMode),
        response: envelope(tutorlolv2_protocol::ShareState), success: [200], auth: owner,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
