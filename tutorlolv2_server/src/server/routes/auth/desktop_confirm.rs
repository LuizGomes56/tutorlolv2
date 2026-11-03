use crate::server::response::{AppError, AppResult, Trace};
use crate::server::state::AppState;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, web::Data};

use crate::server::google::*;

pub async fn confirm_desktop(s: Data<AppState>, a: AccountAuth) -> AppResult<HttpResponse> {
    let device = a
        .device_id
        .filter(|_| !a.cookie)
        .trace_ctx(("Desktop session required", StatusCode::UNAUTHORIZED))?;

    let mut tx =
        s.db.begin()
            .await
            .trace_ctx("Account storage unavailable")?;
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
    .trace_ctx("Account storage unavailable")?;

    if current.is_none() {
        return Err(AppError::unauthorized("Device unavailable"));
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
    .trace_ctx("Account storage unavailable")?;

    if updated.rows_affected() != 1 {
        return Err(AppError::unauthorized("Desktop binding changed"));
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
    .trace_ctx("Account storage unavailable")?;

    tx.commit().await.trace_ctx("Account storage unavailable")?;

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
