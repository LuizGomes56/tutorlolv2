use actix_web::{body::BoxBody, dev::ServiceRequest, middleware::Next};

use crate::middlewares::MwOutput;

/// Logs all the incoming requests to the server
pub async fn logger_middleware(req: ServiceRequest, next: Next<BoxBody>) -> MwOutput {
    println!("Request: {}", req.path());

    let res = next.call(req).await?;
    Ok(res.map_into_right_body())
}
