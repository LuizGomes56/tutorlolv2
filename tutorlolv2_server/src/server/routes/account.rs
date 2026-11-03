use crate::server::{
    codec::{Response, respond},
    state::AppState,
};
use actix_web::web::Data;
use tutorlolv2_protocol::AccountInfo;

use crate::server::google::*;

pub async fn account(s: Data<AppState>, a: AccountAuth) -> Response {
    let row = sqlx::query!(
        r#"
        SELECT email, created_at
        FROM accounts
        WHERE id = $1
            AND deleted_at IS NULL
        "#,
        a.account_id,
    )
    .fetch_one(&s.db)
    .await
    .map_err(storage)?;

    respond(AccountInfo {
        email: row.email,
        created_at_ms: row.created_at.timestamp_millis(),
    })
}
