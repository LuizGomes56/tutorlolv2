//! Mounting of the `/api` routes with their body and rate limits (spec §10.5).

use super::sharing;
use super::{
    auth::parse_device_header,
    games::{calculator_handler, realtime_handler},
    recordings::{end, heartbeat, register_device, set_result, start_session, upload_chunk},
    state::AppState,
};
use actix_governor::{
    Governor, GovernorConfig, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError,
};
use actix_web::{
    Scope,
    dev::{HttpServiceFactory, ServiceRequest},
    http::header,
    middleware::Condition,
    web::{self, Data, PayloadConfig},
};
use std::time::Duration;
use tutorlolv2_protocol::MAX_CHUNK_BYTES;

/// Body limit of every `/api` route except chunk uploads.
const DEFAULT_BODY_LIMIT: usize = 64 * 1024;

/// Rate-limit key: the device for authenticated routes once its credentials
/// are verified, otherwise the client IP. A made-up device id therefore never
/// gets a fresh budget.
#[derive(Clone, Copy, Debug)]
pub struct ClientKey {
    trusted_proxy: bool,
    per_device: bool,
}

impl ClientKey {
    pub fn address(raw: &str) -> Option<std::net::IpAddr> {
        raw.parse::<std::net::SocketAddr>()
            .map(|addr| addr.ip())
            .ok()
            .or_else(|| raw.parse().ok())
    }
}

impl KeyExtractor for ClientKey {
    type Key = String;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        if self.per_device
            && let Some((device, secret)) = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .and_then(parse_device_header)
            && req
                .app_data::<Data<AppState>>()
                .is_some_and(|state| state.auth_cache.is_verified(device, &secret))
        {
            return Ok(format!("device:{device}"));
        }

        let info = req.connection_info();

        let ip = if self.trusted_proxy {
            info.realip_remote_addr()
        } else {
            info.peer_addr()
        };

        ip.and_then(Self::address)
            .map(|ip| format!("ip:{ip}"))
            .ok_or_else(|| SimpleKeyExtractionError::new("Could not identify the client"))
    }
}

type Limit = GovernorConfig<ClientKey, actix_governor::governor::middleware::NoOpMiddleware>;

/// `burst` requests at once, then one every `period`.
fn limit(trusted_proxy: bool, per_device: bool, period: Duration, burst: u32) -> Limit {
    GovernorConfigBuilder::default()
        .key_extractor(ClientKey {
            trusted_proxy,
            per_device,
        })
        .period(period)
        .burst_size(burst)
        .finish()
        .expect("rate limit periods and bursts are non-zero")
}

/// The rate limiters of spec §10.5. Build them **once** and clone them into
/// every worker's `App`: clones share their counters, while building them per
/// worker would multiply every budget by the number of workers.
#[derive(Clone)]
pub struct RateLimits {
    /// 5 per hour per IP.
    devices: Limit,
    /// 10/min with bursts of 30 (spool flushes).
    chunks: Limit,
    /// 30/min.
    sessions: Limit,
    /// Heartbeat, end and result together: 30/min.
    control: Limit,
    live: Limit,
    viewers: Limit,
    /// Owner replay reads: 120/min, enough for bounded chunk prefetch/chart extraction.
    history: Limit,
}

impl RateLimits {
    /// `trusted_proxy`: read the client IP from `X-Forwarded-For` (only behind a reverse proxy).
    pub fn new(trusted_proxy: bool) -> Self {
        Self {
            devices: limit(trusted_proxy, false, Duration::from_secs(3600 / 5), 5),
            chunks: limit(trusted_proxy, true, Duration::from_secs(6), 30),
            sessions: limit(trusted_proxy, true, Duration::from_secs(2), 30),
            control: limit(trusted_proxy, true, Duration::from_secs(2), 30),
            live: limit(trusted_proxy, true, Duration::from_millis(500), 5),
            history: limit(trusted_proxy, true, Duration::from_millis(500), 120),
            viewers: limit(trusted_proxy, false, Duration::from_millis(500), 120),
        }
    }
}

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
                .route(web::get().to(sharing::owner_share))
                .route(web::put().to(sharing::set_share)),
            enabled,
            &limits.sessions
        ))
        .service(guarded!(
            web::resource("/{id}/live")
                .app_data(PayloadConfig::new(sharing::MAX_LIVE_BYTES))
                .route(web::put().to(sharing::push_live)),
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
pub fn api(limits: Option<RateLimits>) -> Scope {
    let enabled = limits.is_some();
    let viewer_limits = limits.clone().unwrap_or_else(|| RateLimits::new(false));

    web::scope("/api")
        .app_data(sharing::ViewerPolicy(enabled))
        .app_data(PayloadConfig::new(DEFAULT_BODY_LIMIT))
        .service(
            web::scope("/games")
                .service(realtime_handler)
                .service(calculator_handler),
        )
        .service(devices(limits.clone()))
        .service(recordings(limits))
        .service(guarded!(
            super::me::scope().wrap(
                actix_web::middleware::DefaultHeaders::new().add(("Cache-Control", "no-store"))
            ),
            enabled,
            &viewer_limits.history
        ))
        .service(guarded!(
            web::scope("/share")
                .route("/{code}", web::get().to(sharing::info))
                .route("/{code}/live", web::get().to(sharing::live))
                .route("/{code}/frames/{seq}", web::get().to(sharing::frames)),
            enabled,
            &viewer_limits.viewers
        ))
}
