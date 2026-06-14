use crate::dev_response;
use actix_web::{HttpResponse, Responder, get};
use tutorlolv2_dev::HTTP_CLIENT;

#[get("/riot")]
pub async fn update_riot() -> impl Responder {
    dev_response!(HTTP_CLIENT.update_riot_cache().await)
}

#[get("/version")]
pub async fn update_version() -> impl Responder {
    dev_response!(unsafe { HTTP_CLIENT.update_env_version().await })
}
