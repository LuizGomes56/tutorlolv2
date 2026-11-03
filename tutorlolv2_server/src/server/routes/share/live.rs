use crate::server::{
    codec::{OCTET_STREAM, Response},
    state::AppState,
};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{ErrorGone, ErrorTooManyRequests},
    web::{Data, Path, Query},
};
use std::time::Duration;

use crate::server::sharing::*;

pub async fn live(
    state: Data<AppState>,
    code: Path<String>,
    query: Query<After>,
    req: HttpRequest,
) -> Response {
    let (code, cap) = state.capability(&code, &req).await?;

    if cap.ended.is_some() {
        return Err(ErrorGone("The match has ended"));
    }

    let (mut receiver, _permit) = state
        .live
        .subscribe(cap.recording, state.config.max_viewers_per_code)
        .ok_or_else(|| ErrorTooManyRequests("Viewer limit reached"))?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);

    loop {
        let cap = Capability::load(&state.db, &code).await?;

        if cap.ended.is_some() {
            return Err(ErrorGone("The match has ended"));
        }

        let latest = receiver.borrow_and_update().clone();

        if let Some(frame) = latest.filter(|frame| frame.seq > query.after) {
            return Ok(HttpResponse::Ok()
                .content_type(OCTET_STREAM)
                .insert_header(("Cache-Control", "no-store"))
                .insert_header(("X-Frame-Seq", frame.seq.to_string()))
                .insert_header(("X-Captured-At", frame.captured_at.to_string()))
                .insert_header(("X-Game-Time", frame.game_time.to_string()))
                .body(frame.bytes.clone()));
        }

        tokio::select! {
            changed = receiver.changed() => if changed.is_err() {
                let cap = Capability::load(&state.db, &code).await?;

                if cap.ended.is_some() {
                    return Err(ErrorGone("The match has ended"));
                }

                return Ok(HttpResponse::NoContent().finish());
            },
            _ = tokio::time::sleep_until(deadline) => return Ok(HttpResponse::NoContent().finish()),
            _ = tokio::time::sleep(Duration::from_secs(1)) => {},
        }
    }
}
