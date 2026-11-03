// Explicit declarations are parsed by the Rust contract generator. The compile-time
// marker/handler references prevent a declaration from naming a missing item.
macro_rules! api_endpoint {
    (pub $marker:ident { handler: $handler:ident, $($metadata:tt)* }) => {
        const _: () = {
            let _ = $handler;
            let _: core::marker::PhantomData<tutorlolv2_api::$marker> = core::marker::PhantomData;
        };
    };
}
macro_rules! api_contract {
    ($($definition:tt)*) => {};
}

// Mounting of the `/api` routes with their body and rate limits (spec §10.5).

pub use crate::server::rate_limits::{ClientKey, RateLimits};
use crate::server::{endpoint::EndpointExt, sharing};
use actix_governor::Governor;
use actix_web::{
    ResponseError, Scope,
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
use tutorlolv2_api as api;
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
        api::RegisterDeviceEndpoint::resource("/api")
            .route(api::RegisterDeviceEndpoint::route().to(register_device)),
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
            api::GetRecordingShare::resource("/api/recordings")
                .route(api::GetRecordingShare::route().to(recordings::share::owner_share))
                .route(api::SetRecordingShare::route().to(recordings::share::set_share)),
            enabled,
            &limits.sessions
        ))
        .service(guarded!(
            api::PushLive::resource("/api/recordings")
                .app_data(PayloadConfig::new(sharing::MAX_LIVE_BYTES))
                .route(api::PushLive::route().to(recordings::live::push_live)),
            enabled,
            &limits.live
        ))
        .service(guarded!(
            api::StartSessionEndpoint::resource("/api/recordings")
                .route(api::StartSessionEndpoint::route().to(start_session)),
            enabled,
            &limits.sessions
        ))
        .service(guarded!(
            api::UploadChunk::resource("/api/recordings")
                .app_data(PayloadConfig::new(MAX_CHUNK_BYTES))
                .route(api::UploadChunk::route().to(upload_chunk)),
            enabled,
            &limits.chunks
        ))
        .service(guarded!(
            api::Heartbeat::resource("/api/recordings")
                .route(api::Heartbeat::route().to(heartbeat)),
            enabled,
            &limits.control
        ))
        .service(guarded!(
            api::EndRecordingEndpoint::resource("/api/recordings")
                .route(api::EndRecordingEndpoint::route().to(end)),
            enabled,
            &limits.control
        ))
        .service(guarded!(
            api::SetResult::resource("/api/recordings")
                .route(api::SetResult::route().to(set_result)),
            enabled,
            &limits.control
        ))
}

/// Everything under `/api`.
pub fn api(limits: Option<RateLimits>) -> impl HttpServiceFactory {
    let enabled = limits.is_some();
    let viewer_limits = limits.clone().unwrap_or_else(|| RateLimits::new(false));

    web::scope("/api")
        .app_data(
            web::PathConfig::default().error_handler(|error, _| {
                crate::server::response::AppError::bad_request(error).into()
            }),
        )
        .app_data(
            web::QueryConfig::default().error_handler(|error, _| {
                crate::server::response::AppError::bad_request(error).into()
            }),
        )
        .default_service(web::route().to(|| async {
            crate::server::response::AppError::not_found("Unknown API route").error_response()
        }))
        .wrap(actix_web::middleware::from_fn(crate::server::cors::policy))
        .app_data(sharing::ViewerPolicy(enabled))
        .app_data(PayloadConfig::new(DEFAULT_BODY_LIMIT))
        .service(
            web::scope("/games")
                .service(
                    api::Realtime::resource("/api/games")
                        .route(api::Realtime::route().to(realtime_handler)),
                )
                .service(
                    api::Calculator::resource("/api/games")
                        .route(api::Calculator::route().to(calculator_handler)),
                ),
        )
        .service(guarded!(
            web::scope("/auth")
                .wrap(
                    actix_web::middleware::DefaultHeaders::new().add(("Cache-Control", "no-store"))
                )
                .service(
                    api::GoogleStart::resource("/api/auth")
                        .route(api::GoogleStart::route().to(auth::google_start::start))
                )
                .service(
                    api::GoogleCallback::resource("/api/auth")
                        .route(api::GoogleCallback::route().to(auth::google_callback::callback))
                )
                .service(
                    api::RedeemDesktop::resource("/api/auth")
                        .route(api::RedeemDesktop::route().to(auth::desktop_redeem::redeem))
                )
                .service(
                    api::ConfirmDesktop::resource("/api/auth").route(
                        api::ConfirmDesktop::route().to(auth::desktop_confirm::confirm_desktop)
                    )
                )
                .service(
                    api::Logout::resource("/api/auth")
                        .route(api::Logout::route().to(auth::logout::logout))
                ),
            enabled,
            &viewer_limits.sessions
        ))
        .service(
            api::GetAccount::resource("/api").route(api::GetAccount::route().to(account::account)),
        )
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
                .service(
                    api::ShareInfoEndpoint::resource("/api/share")
                        .route(api::ShareInfoEndpoint::route().to(share::info::info))
                )
                .service(
                    api::PollLive::resource("/api/share")
                        .route(api::PollLive::route().to(share::live::live))
                )
                .service(
                    api::ShareFrames::resource("/api/share")
                        .route(api::ShareFrames::route().to(share::frames::frames))
                ),
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
            crate::server::response::AppError::bad_request("Invalid recording ID or sequence")
                .into()
        }))
        .service(
            api::ListRecordings::resource("/api/me/recordings")
                .route(api::ListRecordings::route().to(me::recordings::list)),
        )
        .service(
            api::GetRecording::resource("/api/me/recordings")
                .route(api::GetRecording::route().to(me::recording::details)),
        )
        .service(
            api::OwnerFrames::resource("/api/me/recordings")
                .route(api::OwnerFrames::route().to(me::frames::frames)),
        )
        .service(
            api::SetHistoryShare::resource("/api/me/recordings")
                .route(api::SetHistoryShare::route().to(me::share::set_share)),
        )
}
