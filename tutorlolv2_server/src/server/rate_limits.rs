//! Shared API rate-limit configuration.
use super::response::{AppError, Trace};
use super::{auth::parse_device_header, state::AppState};
use actix_governor::governor::{
    NotUntil,
    clock::{Clock, DefaultClock, QuantaInstant},
};
use actix_governor::{GovernorConfig, GovernorConfigBuilder, KeyExtractor};
use actix_web::{
    HttpResponse, HttpResponseBuilder, ResponseError, dev::ServiceRequest, http::header, web::Data,
};
use std::time::Duration;

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
    type KeyExtractionError = AppError;

    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut builder: HttpResponseBuilder,
    ) -> HttpResponse {
        let seconds = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs()
            .max(1);
        let mut response = AppError::rate_limited("Too many requests")
            .retry_after(seconds)
            .error_response();
        for (name, value) in builder.finish().headers() {
            if name != header::RETRY_AFTER {
                response.headers_mut().insert(name.clone(), value.clone());
            }
        }
        response
    }
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
            .trace_ctx("Could not identify the client")
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
    pub(crate) devices: Limit,
    /// 10/min with bursts of 30 (spool flushes).
    pub(crate) chunks: Limit,
    /// 30/min.
    pub(crate) sessions: Limit,
    /// Heartbeat, end and result together: 30/min.
    pub(crate) control: Limit,
    pub(crate) live: Limit,
    pub(crate) viewers: Limit,
    /// Owner replay reads: 120/min, enough for bounded chunk prefetch/chart extraction.
    pub(crate) history: Limit,
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
