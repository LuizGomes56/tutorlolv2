use super::schemas::APIResponse;
use crate::{INTERNAL_CHAMPIONS_STR, WRITER_CONTENT, send_response};
use actix_web::{HttpResponse, Responder, get, web::Path};

#[get("/champions/{name}")]
pub async fn formulas_champions(name: Path<String>) -> impl Responder {
    let data = format!(
        "{}\n{}",
        WRITER_CONTENT.get(&name).unwrap_or(&"Not found"),
        INTERNAL_CHAMPIONS_STR.get(&name).unwrap_or(&"Not found")
    );
    send_response!(data)
}
