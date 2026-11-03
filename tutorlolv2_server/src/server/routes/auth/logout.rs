use crate::server::{codec::Response, state::AppState};
use actix_web::{HttpResponse, error::*, web::Data};

use crate::server::google::*;

pub async fn logout(s: Data<AppState>, a: AccountAuth) -> Response {
    let mut tx = s.db.begin().await.map_err(storage)?;

    if let Some(d) = a.device_id {
        sqlx::query!(
            r#"
            UPDATE devices
            SET account_id = NULL,
                account_session_id = NULL
            WHERE id = $1
                AND account_session_id = $2
            "#,
            d,
            a.session_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    }

    sqlx::query!(
        r#"
        UPDATE account_sessions
        SET revoked_at = now()
        WHERE id = $1
        "#,
        a.session_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(storage)?;

    tx.commit().await.map_err(storage)?;
    let mut r = HttpResponse::NoContent().finish();

    if a.cookie {
        r.add_cookie(&cookie(&s, "tl_session", String::new(), 0))
            .map_err(ErrorInternalServerError)?;
    }

    Ok(r)
}
