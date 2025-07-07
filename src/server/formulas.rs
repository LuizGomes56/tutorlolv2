use super::schemas::APIResponse;
use crate::{INTERNAL_CHAMPIONS_STR, INTERNAL_NAMES, WRITER_CONTENT, send_response};
use actix_web::{HttpResponse, Responder, routes, web::Path};
use rustc_hash::FxHashMap;

#[routes]
#[get("/champions/{name}")]
#[get("/champions")]
pub async fn formulas_champions(name: Option<Path<String>>) -> impl Responder {
    if let Some(name) = name {
        send_response!(format!(
            "{}\n{}",
            WRITER_CONTENT.get(&name).unwrap_or(&"Not found"),
            INTERNAL_CHAMPIONS_STR.get(&name).unwrap_or(&"Not found")
        ))
    } else {
        let mut map = FxHashMap::default();
        for champion_id in INTERNAL_NAMES.values() {
            map.insert(
                champion_id,
                format!(
                    "{}\n{}",
                    WRITER_CONTENT.get(champion_id).unwrap_or(&"Not found"),
                    INTERNAL_CHAMPIONS_STR
                        .get(champion_id)
                        .unwrap_or(&"Not found")
                ),
            );
        }
        send_response!(map)
    }
}
