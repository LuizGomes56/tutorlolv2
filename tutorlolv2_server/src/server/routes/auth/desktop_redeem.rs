use crate::server::endpoint::EndpointExt;
use crate::server::response::{AppError, Trace};
use crate::server::{auth::DeviceAuth, codec::Bincode, state::AppState};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use std::time::Duration;
use tutorlolv2_protocol::{DesktopSession, RedeemTicket};

use crate::server::google::*;

pub async fn redeem(
    s: Data<AppState>,
    auth: DeviceAuth,
    Bincode(q): Bincode<RedeemTicket>,
) -> crate::server::response::ApiResult<DesktopSession> {
    s.google_credentials().trace()?;

    let (account, at) = s
        .google
        .tickets
        .lock()
        .unwrap()
        .remove(&q.ticket)
        .trace_ctx(("Ticket expired or used", StatusCode::GONE))?;

    if at.elapsed() >= Duration::from_secs(120) {
        return Err(AppError::gone("Ticket expired or used"));
    }

    let mut tx =
        s.db.begin()
            .await
            .trace_ctx("Account storage unavailable")?;

    // NO KEY UPDATE protects deletion/identity changes without blocking the
    // account FK KEY SHARE taken by a concurrent recording creation. The device
    // lock below is shared with start_session/logout ownership transitions.
    let email = sqlx::query_scalar!(
        r#"
        SELECT email
        FROM accounts
        WHERE id = $1
            AND deleted_at IS NULL
            FOR NO KEY
            UPDATE
        "#,
        account,
    )
    .fetch_optional(&mut *tx)
    .await
    .trace_ctx("Account storage unavailable")?
    .trace_ctx(("Account unavailable", StatusCode::UNAUTHORIZED))?;

    let device = sqlx::query_scalar!(
        r#"
        SELECT id
        FROM devices
        WHERE id = $1
            AND deleted_at IS NULL
            FOR
            UPDATE
        "#,
        auth.device_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .trace_ctx("Account storage unavailable")?;

    if device.is_none() {
        return Err(AppError::unauthorized("Device unavailable"));
    }

    let (t, id) = (GoogleAuthService { state: &s })
        .create_session(&mut tx, account, Some(auth.device_id))
        .await
        .trace()?;
    sqlx::query!(
        "UPDATE devices
        SET account_id = $1,
            account_session_id = $2
        WHERE id = $3",
        account,
        id,
        auth.device_id,
    )
    .execute(&mut *tx)
    .await
    .trace_ctx("Account storage unavailable")?;

    sqlx::query!(
        "UPDATE recordings
        SET account_id = $1
        WHERE device_id = $2
            AND account_id IS NULL",
        account,
        auth.device_id,
    )
    .execute(&mut *tx)
    .await
    .trace_ctx("Account storage unavailable")?;

    tx.commit().await.trace_ctx("Account storage unavailable")?;

    Ok(tutorlolv2_api::RedeemDesktop::reply(DesktopSession {
        token: t,
        email,
    }))
}

api_endpoint! {
    pub RedeemDesktop {
        handler: redeem, method: POST, path: "/api/auth/desktop/redeem",
        path_params: {  }, query: none, body: bincode(tutorlolv2_protocol::RedeemTicket),
        response: envelope(tutorlolv2_protocol::DesktopSession), success: [200], auth: device,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
