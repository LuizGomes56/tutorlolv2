use crate::server::{codec::Response, state::AppState};
use actix_web::{HttpResponse, error::*, web::Data};

use crate::server::google::*;

pub async fn confirm_desktop(s: Data<AppState>, a: AccountAuth) -> Response {
    let device = a
        .device_id
        .filter(|_| !a.cookie)
        .ok_or_else(|| ErrorUnauthorized("Desktop session required"))?;

    let mut tx = s.db.begin().await.map_err(storage)?;
    let current = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM devices
        WHERE id = $1
            AND deleted_at IS NULL
            FOR
            UPDATE
        "#,
        device,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(storage)?;

    if current.is_none() {
        return Err(ErrorUnauthorized("Device unavailable"));
    }

    // Recheck after acquiring the lock: the extractor may have authenticated
    // before a concurrent login/logout replaced the device binding.
    let updated = sqlx::query!(
        r#"
        UPDATE account_sessions s
        SET desktop_confirmed_at = COALESCE(s.desktop_confirmed_at, clock_timestamp())
        WHERE s.id = $1
            AND s.account_id = $2
            AND s.device_id = $3
            AND s.kind = 'Desktop'
            AND s.revoked_at IS NULL
            AND s.expires_at > clock_timestamp()
            AND EXISTS
                (SELECT 1
                 FROM devices d
                 WHERE d.id = $3
                     AND d.account_session_id = s.id
                     AND d.account_id = s.account_id
                     AND d.deleted_at IS NULL)
            AND EXISTS
                (SELECT 1
                 FROM accounts a
                 WHERE a.id = s.account_id
                     AND a.deleted_at IS NULL)
        "#,
        a.session_id,
        a.account_id,
        device,
    )
    .execute(&mut *tx)
    .await
    .map_err(storage)?;

    if updated.rows_affected() != 1 {
        return Err(ErrorUnauthorized("Desktop binding changed"));
    }

    sqlx::query!(
        r#"
        UPDATE recordings
        SET account_id = $1
        WHERE device_id = $2
            AND account_id IS NULL
        "#,
        a.account_id,
        device,
    )
    .execute(&mut *tx)
    .await
    .map_err(storage)?;

    tx.commit().await.map_err(storage)?;

    Ok(HttpResponse::NoContent().finish())
}

api_endpoint! {
    pub ConfirmDesktop {
        handler: confirm_desktop, method: POST, path: "/api/auth/desktop/confirm",
        path_params: {  }, query: none, body: none,
        response: empty, success: [204], auth: account,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
