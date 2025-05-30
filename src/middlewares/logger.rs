use actix_web::{
    Error,
    body::{BoxBody, EitherBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

pub async fn logger_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<EitherBody<BoxBody, BoxBody>>, Error> {
    println!("Request: {}", req.path());

    let res = next.call(req).await?;
    Ok(res.map_into_right_body())
}
