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
    password: String,
}

impl AccessRequest {
    pub fn password(&self) -> &str {
        &self.password
    }
}

pub async fn password_middleware(
    state: Data<AppState>,
    body: Json<AccessRequest>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, BoxBody>>, Error> {
    if body.password() != state.password {
        let response = HttpResponse::Unauthorized().json(APIResponse {
            success: false,
            message: "Password given is invalid".to_string(),
            data: (),
        });

        return Ok(req.into_response(response.map_into_left_body()));
    }

    let res = next.call(req).await?;
    Ok(res.map_into_right_body())
}
