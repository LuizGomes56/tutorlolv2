//! Mounting of the `/api` routes with their body and rate limits (spec §10.5).

pub use crate::server::rate_limits::{ClientKey, RateLimits};
use crate::server::sharing;
use actix_governor::Governor;
use actix_web::{
    Scope,
    dev::HttpServiceFactory,
    middleware::Condition,
    web::{self, PayloadConfig},
};
use devices::register_device;
use games::{calculator::calculator_handler, realtime::realtime_handler};
use recordings::{
    chunk::upload_chunk, end::end, heartbeat::heartbeat, result::set_result,
    sessions::start_session,
};
use tutorlolv2_protocol::MAX_CHUNK_BYTES;

/// Body limit of every `/api` route except chunk uploads.
const DEFAULT_BODY_LIMIT: usize = 64 * 1024;

/// Applies `$config` to a fully built resource; a no-op when `$enabled` is false.
macro_rules! guarded {
    ($resource:expr, $enabled:expr, $config:expr) => {
        $resource.wrap(Condition::new($enabled, Governor::new($config)))
    };
}

/// `POST /devices`. `None` mounts the routes without limits (tests).
pub fn devices(limits: Option<RateLimits>) -> impl HttpServiceFactory + 'static {
    let enabled = limits.is_some();
    let limits = limits.unwrap_or_else(|| RateLimits::new(false));

    guarded!(
        web::resource("/devices").route(web::post().to(register_device)),
        enabled,
        &limits.devices
    )
}

/// `/recordings/*`. `None` mounts the routes without limits (tests).
pub fn recordings(limits: Option<RateLimits>) -> Scope {
    let enabled = limits.is_some();
    let limits = limits.unwrap_or_else(|| RateLimits::new(false));

    web::scope("/recordings")
        .service(guarded!(
            web::resource("/{id}/share")
                .route(web::get().to(recordings::share::owner_share))
                .route(web::put().to(recordings::share::set_share)),
            enabled,
            &limits.sessions
        ))
        .service(guarded!(
            web::resource("/{id}/live")
                .app_data(PayloadConfig::new(sharing::MAX_LIVE_BYTES))
                .route(web::put().to(recordings::live::push_live)),
            enabled,
            &limits.live
        ))
        .service(guarded!(
            web::resource("/sessions").route(web::post().to(start_session)),
            enabled,
            &limits.sessions
        ))
        .service(guarded!(
            web::resource("/{id}/chunks/{seq}")
                .app_data(PayloadConfig::new(MAX_CHUNK_BYTES))
                .route(web::post().to(upload_chunk)),
            enabled,
            &limits.chunks
        ))
        .service(guarded!(
            web::resource("/{id}/heartbeat").route(web::post().to(heartbeat)),
            enabled,
            &limits.control
        ))
        .service(guarded!(
            web::resource("/{id}/end").route(web::post().to(end)),
            enabled,
            &limits.control
        ))
        .service(guarded!(
            web::resource("/{id}/result").route(web::patch().to(set_result)),
            enabled,
            &limits.control
        ))
}

/// Everything under `/api`.
pub fn api(limits: Option<RateLimits>) -> impl HttpServiceFactory {
    let enabled = limits.is_some();
    let viewer_limits = limits.clone().unwrap_or_else(|| RateLimits::new(false));

    web::scope("/api")
        .wrap(actix_web::middleware::from_fn(crate::server::cors::policy))
        .app_data(sharing::ViewerPolicy(enabled))
        .app_data(PayloadConfig::new(DEFAULT_BODY_LIMIT))
        .service(
            web::scope("/games")
                .service(realtime_handler)
                .service(calculator_handler),
        )
        .service(guarded!(
            web::scope("/auth")
                .wrap(
                    actix_web::middleware::DefaultHeaders::new().add(("Cache-Control", "no-store"))
                )
                .route("/google/start", web::get().to(auth::google_start::start))
                .route(
                    "/google/callback",
                    web::get().to(auth::google_callback::callback)
                )
                .route(
                    "/desktop/redeem",
                    web::post().to(auth::desktop_redeem::redeem)
                )
                .route(
                    "/desktop/confirm",
                    web::post().to(auth::desktop_confirm::confirm_desktop)
                )
                .route("/logout", web::post().to(auth::logout::logout)),
            enabled,
            &viewer_limits.sessions
        ))
        .route("/account", web::get().to(account::account))
        .service(devices(limits.clone()))
        .service(recordings(limits))
        .service(guarded!(
            crate::server::me::scope().wrap(
                actix_web::middleware::DefaultHeaders::new().add(("Cache-Control", "no-store"))
            ),
            enabled,
            &viewer_limits.history
        ))
        .service(guarded!(
            web::scope("/share")
                .route("/{code}", web::get().to(share::info::info))
                .route("/{code}/live", web::get().to(share::live::live))
                .route("/{code}/frames/{seq}", web::get().to(share::frames::frames)),
            enabled,
            &viewer_limits.viewers
        ))
}

pub mod games {
    pub mod calculator;
    pub mod realtime;
}

pub mod recordings {
    pub mod chunk;
    pub mod end;
    pub mod heartbeat;
    pub mod live;
    pub mod result;
    pub mod sessions;
    pub mod share;
}

pub mod auth {
    pub mod desktop_confirm;
    pub mod desktop_redeem;
    pub mod google_callback;
    pub mod google_start;
    pub mod logout;
}

pub mod me {
    pub mod frames;
    pub mod recording;
    pub mod recordings;
    pub mod share;
}

pub mod share {
    pub mod frames;
    pub mod info;
    pub mod live;
}

pub mod account;
pub mod devices;

pub fn me_scope() -> Scope {
    web::scope("/me/recordings")
        .app_data(web::PathConfig::default().error_handler(|_, _| {
            actix_web::error::ErrorBadRequest("Invalid recording ID or sequence")
        }))
        .route("", web::get().to(me::recordings::list))
        .route("/{id}", web::get().to(me::recording::details))
        .route("/{id}/frames/{seq}", web::get().to(me::frames::frames))
        .route("/{id}/share", web::put().to(me::share::set_share))
}
