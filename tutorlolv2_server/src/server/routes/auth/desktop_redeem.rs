use crate::server::{
    auth::DeviceAuth,
    codec::{Bincode, Response, respond},
    state::AppState,
};
use actix_web::{error::*, web::Data};
use std::time::Duration;
use tutorlolv2_protocol::{DesktopSession, RedeemTicket};

use crate::server::google::*;

pub async fn redeem(
    s: Data<AppState>,
    auth: DeviceAuth,
    Bincode(q): Bincode<RedeemTicket>,
) -> Response {
    enabled(&s)?;

    let (account, at) = s
        .google
        .tickets
        .lock()
        .unwrap()
        .remove(&q.ticket)
        .ok_or_else(|| ErrorGone("Ticket expired or used"))?;

    if at.elapsed() >= Duration::from_secs(120) {
        return Err(ErrorGone("Ticket expired or used"));
    }

    let mut tx = s.db.begin().await.map_err(storage)?;

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
    .map_err(storage)?
    .ok_or_else(|| ErrorUnauthorized("Account unavailable"))?;

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
    .map_err(storage)?;

    if device.is_none() {
        return Err(ErrorUnauthorized("Device unavailable"));
    }

    let (t, id) = (GoogleAuthService { state: &s })
        .create_session(&mut tx, account, Some(auth.device_id))
        .await?;
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
    .map_err(storage)?;

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
    .map_err(storage)?;

    tx.commit().await.map_err(storage)?;

    respond(DesktopSession { token: t, email })
}
