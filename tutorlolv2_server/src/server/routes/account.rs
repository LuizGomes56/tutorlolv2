use crate::server::endpoint::EndpointExt;
use crate::server::response::Trace;
use crate::server::state::AppState;
use actix_web::web::Data;
use tutorlolv2_protocol::AccountInfo;

use crate::server::google::*;

pub async fn account(
    s: Data<AppState>,
    a: AccountAuth,
) -> crate::server::response::ApiResult<AccountInfo> {
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
    .trace_ctx("Account storage unavailable")?;

    Ok(tutorlolv2_api::GetAccount::reply(AccountInfo {
        email: row.email,
        created_at_ms: row.created_at.timestamp_millis(),
    }))
}

api_endpoint! {
    pub GetAccount {
        handler: account, method: GET, path: "/api/account",
        path_params: {  }, query: none, body: none,
        response: envelope(tutorlolv2_protocol::AccountInfo), success: [200], auth: account,
        request_headers: [], response_headers: ["Cache-Control"],
    }
}
