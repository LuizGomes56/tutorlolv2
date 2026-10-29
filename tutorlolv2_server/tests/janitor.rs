mod common;

use actix_web::{App, test as http};
use common::*;
use sqlx::PgPool;
use tutorlolv2_protocol::{CodeMode, EndReason};
use tutorlolv2_server::server::{
    db_enums::ShareDeleteReason,
    janitor::expire_stale,
    routes::{devices, recordings},
};
use uuid::Uuid;

async fn recording_idle_for(app: &impl TestApp, pool: &PgPool, minutes: f64) -> Uuid {
    let device = register(app).await;
    let id = Uuid::from_bytes(start(app, &device, session(probe(120.0, 0.0276, "ahri"))).await.recording_id);
    sqlx::query!("UPDATE recordings SET last_ingest_at = now() - $2 * interval '1 minute' WHERE id = $1", id, minutes)
        .execute(pool)
        .await
        .unwrap();
    id
}

#[sqlx::test(migrations = "./migrations")]
async fn expires_only_stale_open_recordings(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool.clone())).service(devices(None)).service(recordings(None))).await;
    let stale = recording_idle_for(&app, &pool, 16.0).await;
    let fresh = recording_idle_for(&app, &pool, 1.0).await;
    let ended = recording_idle_for(&app, &pool, 30.0).await;
    sqlx::query!("UPDATE recordings SET ended_at = now(), end_reason = $2 WHERE id = $1", ended, EndReason::GameEndEvent as EndReason)
        .execute(&pool)
        .await
        .unwrap();
    let deleted = recording_idle_for(&app, &pool, 30.0).await;
    sqlx::query!("UPDATE recordings SET deleted_at = now() WHERE id = $1", deleted).execute(&pool).await.unwrap();

    assert_eq!(expire_stale(&pool, 900).await.unwrap(), 1);

    let row = sqlx::query!(
        r#"SELECT ended_at = last_ingest_at AS "at_last_ingest!", end_reason AS "end_reason: EndReason" FROM recordings WHERE id = $1"#,
        stale
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(row.at_last_ingest);
    assert_eq!(row.end_reason, Some(EndReason::Timeout));

    for (id, open) in [(fresh, true), (ended, false), (deleted, true)] {
        let is_open = sqlx::query_scalar!(r#"SELECT ended_at IS NULL AS "open!" FROM recordings WHERE id = $1"#, id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(is_open, open, "{id}");
    }
    assert_eq!(expire_stale(&pool, 900).await.unwrap(), 0, "idempotent");
}

#[sqlx::test(migrations = "./migrations")]
async fn expiring_soft_deletes_live_code_but_not_public(pool: PgPool) {
    let app = http::init_service(App::new().app_data(state(pool.clone())).service(devices(None)).service(recordings(None))).await;
    let live = recording_idle_for(&app, &pool, 20.0).await;
    let public = recording_idle_for(&app, &pool, 20.0).await;
    for (id, code, mode) in [(live, "AAAA0000", CodeMode::Live), (public, "BBBB0000", CodeMode::Public)] {
        sqlx::query!("INSERT INTO share_codes (recording_id, code, mode) VALUES ($1, $2, $3)", id, code, mode as CodeMode)
            .execute(&pool)
            .await
            .unwrap();
    }

    assert_eq!(expire_stale(&pool, 900).await.unwrap(), 2);

    for (code, expected) in [("AAAA0000", Some(ShareDeleteReason::GameEnded)), ("BBBB0000", None)] {
        let reason = sqlx::query_scalar!(r#"SELECT delete_reason AS "r: ShareDeleteReason" FROM share_codes WHERE code = $1"#, code)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(reason, expected, "{code}");
    }
}
