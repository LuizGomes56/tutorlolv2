use crate::{ENV_CONFIG, middlewares::MwOutput, server::schemas::APIResponse};
use actix_web::{
    HttpResponse,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccessRequest {
    pub password: String,
}

/// Some routes may be accessible only by the owner of the server
/// Image uploads, generators, and updaters must be controlled
/// Exposing this routes can overload server with too many thread spawns
/// Also can cause excessive resource usage from third-party services
pub async fn password_middleware(
    body: Json<AccessRequest>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> MwOutput {
    if body.password != ENV_CONFIG.system_password {
        let response: HttpResponse = HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Password given is invalid",
            data: (),
        });

        return Ok(req.into_response(response.map_into_left_body()));
    }

    let res: ServiceResponse = next.call(req).await?;
    Ok(res.map_into_right_body())
}
