use crate::server::response::{AppResult, Trace};
use crate::server::state::AppState;
use actix_web::{HttpResponse, web::Data};

use crate::server::google::*;

pub async fn logout(s: Data<AppState>, a: AccountAuth) -> AppResult<HttpResponse> {
    let mut tx =
        s.db.begin()
            .await
            .trace_ctx("Account storage unavailable")?;

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
        .trace_ctx("Account storage unavailable")?;
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
    .trace_ctx("Account storage unavailable")?;

    tx.commit().await.trace_ctx("Account storage unavailable")?;
    let mut r = HttpResponse::NoContent().finish();

    if a.cookie {
        r.add_cookie(&cookie(&s, "tl_session", String::new(), 0))
            .trace()?;
    }

    Ok(r)
}

api_endpoint! {
    pub Logout {
        handler: logout, method: POST, path: "/api/auth/logout",
        path_params: {  }, query: none, body: none,
        response: empty, success: [204], auth: account,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
