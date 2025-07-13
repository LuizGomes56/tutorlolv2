use actix_web::{
    body::{BoxBody, EitherBody},
    dev::ServiceResponse,
};

pub mod jsonmw;
pub mod logger;
pub mod password;

pub(self) type MwOutput = Result<ServiceResponse<EitherBody<BoxBody, BoxBody>>, actix_web::Error>;
