use crate::server::response::{AppError, AppResult, Trace};
use crate::server::{codec::OCTET_STREAM, state::AppState};
use actix_web::http::StatusCode;
use actix_web::{
    HttpRequest, HttpResponse,
    web::{Data, Path, Query},
};
use std::time::Duration;

use crate::server::sharing::*;

pub async fn live(
    state: Data<AppState>,
    code: Path<String>,
    query: Query<After>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    let (code, cap) = state.capability(&code, &req).await.trace()?;

    if cap.ended.is_some() {
        return Err(AppError::gone("The match has ended"));
    }

    let (mut receiver, _permit) = state
        .live
        .subscribe(cap.recording, state.config.max_viewers_per_code)
        .trace_ctx(("Viewer limit reached", StatusCode::TOO_MANY_REQUESTS))?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);

    loop {
        let cap = Capability::load(&state.db, &code).await.trace()?;

        if cap.ended.is_some() {
            return Err(AppError::gone("The match has ended"));
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
                let cap = Capability::load(&state.db, &code).await.trace()?;

                if cap.ended.is_some() {
                    return Err(AppError::gone("The match has ended"));
                }

                return Ok(HttpResponse::NoContent().finish());
            },
            _ = tokio::time::sleep_until(deadline) => return Ok(HttpResponse::NoContent().finish()),
            _ = tokio::time::sleep(Duration::from_secs(1)) => {},
        }
    }
}

api_contract! { pub struct After { #[serde(default)] pub after: u64 } }

api_endpoint! {
    pub PollLive {
        handler: live, method: GET, path: "/api/share/{code}/live",
        path_params: { code: String }, query: form(After), body: none,
        response: live(math::riot::RiotRealtime<'wire>), success: [200,204], auth: capability,
        request_headers: [], response_headers: ["X-Frame-Seq", "X-Captured-At", "X-Game-Time", "Cache-Control"],
    }
}
