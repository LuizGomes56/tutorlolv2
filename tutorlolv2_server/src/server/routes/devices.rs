use crate::server::{
    auth::secret_hash,
    codec::{Bincode, Response, respond},
    state::AppState,
};
use actix_web::{error::ErrorInternalServerError, web::Data};
use tutorlolv2_protocol::{DeviceCredentials, RegisterDevice};

pub async fn register_device(
    state: Data<AppState>,
    Bincode(request): Bincode<RegisterDevice>,
) -> Response {
    let mut device_secret = [0u8; 32];

    getrandom::fill(&mut device_secret).map_err(ErrorInternalServerError)?;

    let device_id = sqlx::query_scalar!(
        r#"
        INSERT INTO devices (
            secret_hash,
            app_version
        ) VALUES (
            $1,
            $2
        )
        RETURNING id
        "#,
        &secret_hash(&device_secret),
        request.app_version,
    )
    .fetch_one(&state.db)
    .await
    .map_err(ErrorInternalServerError)?;

    respond(DeviceCredentials {
        device_id: device_id.into_bytes(),
        device_secret,
    })
}

api_endpoint! {
    pub RegisterDeviceEndpoint {
        handler: register_device, method: POST, path: "/api/devices",
        path_params: {  }, query: none, body: bincode(tutorlolv2_protocol::RegisterDevice),
        response: envelope(tutorlolv2_protocol::DeviceCredentials), success: [200], auth: anonymous,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
