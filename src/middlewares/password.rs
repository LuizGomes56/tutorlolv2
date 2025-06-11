use std::time::Instant;

use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, EitherBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::{Data, Json},
};
use serde::Deserialize;

use crate::{AppState, server::schemas::APIResponse};

#[derive(Deserialize)]
pub struct AccessRequest {
    pub password: String,
}

// Some routes may be accessible only by the owner of the server
// Image uploads, generators, and updaters must be controlled
// Exposing this routes can overload server with too many thread spawns
// Also can cause excessive resource usage from third-party services
pub async fn password_middleware(
    state: Data<AppState>,
    body: Json<AccessRequest>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, BoxBody>>, Error> {
    println!("Instant Time: {:#?}", Instant::now());

    if body.password != state.envcfg.system_password {
        let response: HttpResponse = HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Password given is invalid".to_string(),
            data: (),
        });

        return Ok(req.into_response(response.map_into_left_body()));
    }

    let res: ServiceResponse = next.call(req).await?;
    Ok(res.map_into_right_body())
}
