use crate::server::{
    codec::{Bincode, Response},
    google::Owner,
    sharing,
    state::AppState,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Path},
};
use tutorlolv2_protocol::*;
use uuid::Uuid;

use crate::server::me::*;

pub async fn set_share(
    state: Data<AppState>,
    auth: Owner,
    id: Path<Uuid>,
    Bincode(request): Bincode<SetShareMode>,
) -> Response {
    (sharing::SharingService { state: &state })
        .change_share(auth, id.into_inner(), request, true)
        .await
        .map(no_store)
        .map_err(|error| {
            if error.as_response_error().status_code().is_server_error() {
                ErrorInternalServerError("History storage unavailable")
            } else {
                error
            }
        })
}

api_endpoint! {
    pub SetHistoryShare {
        handler: set_share, method: PUT, path: "/api/me/recordings/{id}/share",
        path_params: { id: Uuid }, query: none, body: bincode(tutorlolv2_protocol::SetShareMode),
        response: envelope(tutorlolv2_protocol::ShareState), success: [200], auth: owner,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
