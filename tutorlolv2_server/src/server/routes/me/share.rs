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
