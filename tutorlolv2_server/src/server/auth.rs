//! Device authentication (spec §13.1): `Authorization: Device <id>.<base64url(secret)>`.

use super::state::AppState;
use actix_web::{
    FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized, http::header, web::Data,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::RwLock,
    time::{Duration, Instant},
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

const SCHEME: &str = "Device ";
/// Verified credentials skip the database for this long (avoids a query per live snapshot).
const CACHE_TTL: Duration = Duration::from_secs(10 * 60);

/// A request made by a registered, not soft-deleted device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceAuth {
    pub device_id: Uuid,
}

/// Parses `Device <uuid-hyphenated>.<base64url-nopad 32 bytes>`.
pub fn parse_device_header(value: &str) -> Option<(Uuid, [u8; 32])> {
    let (id, secret) = value.strip_prefix(SCHEME)?.split_once('.')?;
    let id = Uuid::try_parse(id).ok()?;
    let secret = URL_SAFE_NO_PAD.decode(secret).ok()?.try_into().ok()?;

    Some((id, secret))
}

pub fn secret_hash(secret: &[u8; 32]) -> [u8; 32] {
    Sha256::digest(secret).into()
}

/// Recently verified `(device, sha256(secret))` pairs.
#[derive(Default)]
pub struct AuthCache {
    verified: RwLock<HashMap<(Uuid, [u8; 32]), Instant>>,
}

impl AuthCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether these credentials were verified recently (no database access).
    pub fn is_verified(&self, id: Uuid, secret: &[u8; 32]) -> bool {
        self.verified
            .read()
            .unwrap()
            .get(&(id, secret_hash(secret)))
            .is_some_and(|at| at.elapsed() < CACHE_TTL)
    }

    pub async fn verify(&self, db: &PgPool, id: Uuid, secret: &[u8; 32]) -> sqlx::Result<bool> {
        let hash = secret_hash(secret);
        let key = (id, hash);

        if let Some(at) = self.verified.read().unwrap().get(&key)
            && at.elapsed() < CACHE_TTL
        {
            return Ok(true);
        }

        let stored = sqlx::query_scalar!(
            "
            SELECT secret_hash
            FROM devices
            WHERE id = $1
                AND deleted_at IS NULL
            ",
            id
        )
        .fetch_optional(db)
        .await?;

        let valid = stored.is_some_and(|stored| bool::from(stored.ct_eq(&hash)));

        if valid {
            sqlx::query!(
                "
                UPDATE devices
                SET last_seen_at = now()
                WHERE id = $1
                ",
                id
            )
            .execute(db)
            .await?;

            let mut verified = self.verified.write().unwrap();

            verified.retain(|_, at| at.elapsed() < CACHE_TTL);
            verified.insert(key, Instant::now());
        }

        Ok(valid)
    }
}

impl FromRequest for DeviceAuth {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let credentials = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(parse_device_header);

        let state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {
            let (Some((id, secret)), Some(state)) = (credentials, state) else {
                return Err(ErrorUnauthorized("Missing or malformed device credentials"));
            };

            match state.auth_cache.verify(&state.db, id, &secret).await {
                Ok(true) => Ok(DeviceAuth { device_id: id }),
                Ok(false) => Err(ErrorUnauthorized("Unknown device or wrong secret")),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
            }
        })
    }
}
