mod common;

use actix_web::{App, test as http};
use common::*;
use sqlx::PgPool;
use tutorlolv2_protocol::{CodeMode, EndReason, GameMode, LcuGame, Platform, Region, ShareMode};
use tutorlolv2_server::server::{
    db_enums::{RiotGameSource, ShareDeleteReason},
    janitor::expire_stale,
    routes::{devices, recordings},
};
use uuid::Uuid;

const CODE: &str = "ABCDEFGH";

macro_rules! app {
    ($pool:expr) => {
        http::init_service(
            App::new()
                .app_data(state($pool.clone()))
                .service(devices(None))
                .service(recordings(None)),
        )
        .await
    };
}

async fn open_count(pool: &PgPool, device: Uuid) -> i64 {
    sqlx::query_scalar!(
        r#"SELECT count(*) AS "count!" FROM recordings WHERE device_id = $1 AND ended_at IS NULL"#,
        device
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrations = "./migrations")]
async fn first_session_creates_recording(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let mut request = session(probe(120.0, 0.0276, "ahri"));
    request.probe.game_mode = GameMode::PracticeTool;
    request.region = Some(Region::Na);
    request.lcu = Some(LcuGame {
        game_id: 5669549208,
        queue_id: Some(3140),
        is_custom: Some(true),
    });
    let started = start(&app, &device, request).await;
    assert!(!started.resumed);
    assert_eq!(started.next_chunk_seq, 0);
    assert_eq!(started.share.mode, ShareMode::Private);

    let row = sqlx::query!(
        r#"SELECT champion, game_mode AS "game_mode: GameMode", is_practice, active_riot_id,
                  platform_id AS "platform_id: Platform", riot_game_id, queue_id, is_custom, first_game_time,
                  riot_game_source AS "riot_game_source: RiotGameSource"
           FROM recordings WHERE id = $1"#,
        Uuid::from_bytes(started.recording_id),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.champion, "ahri");
    assert_eq!(row.game_mode, GameMode::PracticeTool);
    assert!(row.is_practice);
    assert_eq!(row.active_riot_id, ME);
    assert_eq!(row.platform_id, Some(Platform::Na1));
    assert_eq!(
        (row.riot_game_id, row.queue_id, row.is_custom),
        (Some(5669549208), Some(3140), Some(true))
    );
    assert_eq!(row.first_game_time, 120.0);
    assert_eq!(row.riot_game_source, Some(RiotGameSource::Lcu));
}

#[sqlx::test(migrations = "./migrations")]
async fn same_match_resumes_with_next_seq(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let first = start(&app, &device, session(probe(120.0, 0.0276, "ahri"))).await;
    let id = Uuid::from_bytes(first.recording_id);
    for seq in 0..=2 {
        sqlx::query!(
            "INSERT INTO recording_chunks (recording_id, seq, first_game_time, last_game_time, snapshot_count,
             raw_format, raw_sha256, raw_size, raw, frames_format, frames)
             VALUES ($1, $2, 0, 1, 1, 1, $3, 1, $4, 1, $4)",
            id,
            seq,
            &[0u8; 32][..],
            &[0u8][..],
        )
        .execute(&pool)
        .await
        .unwrap();
    }
    let again = start(&app, &device, session(probe(121.0, 0.0276, "ahri"))).await;
    assert_eq!(again.recording_id, first.recording_id);
    assert!(again.resumed);
    assert_eq!(again.next_chunk_seq, 3);
}

#[sqlx::test(migrations = "./migrations")]
async fn different_match_supersedes_open_one(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let old = start(&app, &device, session(probe(1500.0, 0.0276, "ahri"))).await;
    let new = start(&app, &device, session(probe(40.0, 0.0312, "zoe"))).await;
    assert_ne!(new.recording_id, old.recording_id);
    assert!(!new.resumed);

    let old_row = sqlx::query!(
        r#"SELECT ended_at IS NOT NULL AS "ended!", end_reason AS "end_reason: EndReason" FROM recordings WHERE id = $1"#,
        Uuid::from_bytes(old.recording_id),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (old_row.ended, old_row.end_reason),
        (true, Some(EndReason::Superseded))
    );
    assert_eq!(
        open_count(&pool, Uuid::from_bytes(device.device_id)).await,
        1
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn timed_out_recording_is_reopened_when_same(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let first = start(&app, &device, session(probe(120.0, 0.0276, "ahri"))).await;
    let id = Uuid::from_bytes(first.recording_id);
    sqlx::query!(
        "UPDATE recordings SET last_ingest_at = now() - interval '20 minutes',
         ended_at = now() - interval '20 minutes', end_reason = $2 WHERE id = $1",
        id,
        EndReason::Timeout as EndReason,
    )
    .execute(&pool)
    .await
    .unwrap();

    let again = start(&app, &device, session(probe(1320.0, 0.0276, "ahri"))).await;
    assert_eq!(again.recording_id, first.recording_id);
    assert!(again.resumed);
    let open = sqlx::query_scalar!(r#"SELECT ended_at IS NULL AND end_reason IS NULL AS "open!" FROM recordings WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(open);

    // the next janitor tick must not end it again before any chunk arrives
    assert_eq!(expire_stale(&pool, 900).await.unwrap(), 0);
}

#[sqlx::test(migrations = "./migrations")]
async fn old_timed_out_recording_is_not_reopened(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let first = start(&app, &device, session(probe(120.0, 0.0276, "ahri"))).await;
    sqlx::query!(
        "UPDATE recordings SET last_ingest_at = now() - interval '7 hours', ended_at = now() - interval '7 hours', end_reason = $2 WHERE id = $1",
        Uuid::from_bytes(first.recording_id),
        EndReason::Timeout as EndReason,
    )
    .execute(&pool)
        .await
        .unwrap();
    let again = start(&app, &device, session(probe(25_320.0, 0.0276, "ahri"))).await;
    assert_ne!(again.recording_id, first.recording_id);
}

#[sqlx::test(migrations = "./migrations")]
async fn live_code_is_soft_deleted_on_supersede(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let old = start(&app, &device, session(probe(1500.0, 0.0276, "ahri"))).await;
    sqlx::query!(
        "INSERT INTO share_codes (recording_id, code, mode) VALUES ($1, $2, $3)",
        Uuid::from_bytes(old.recording_id),
        CODE,
        CodeMode::Live as CodeMode,
    )
    .execute(&pool)
    .await
    .unwrap();
    start(&app, &device, session(probe(40.0, 0.0312, "zoe"))).await;
    let code = sqlx::query!(
        r#"SELECT deleted_at IS NOT NULL AS "deleted!", delete_reason AS "delete_reason: ShareDeleteReason"
           FROM share_codes WHERE code = $1"#,
        CODE,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (code.deleted, code.delete_reason),
        (true, Some(ShareDeleteReason::GameEnded))
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn other_devices_recordings_are_never_matched(pool: PgPool) {
    let app = app!(pool);
    let a = register(&app).await;
    let b = register(&app).await;
    let first = start(&app, &a, session(probe(120.0, 0.0276, "ahri"))).await;
    let other = start(&app, &b, session(probe(121.0, 0.0276, "ahri"))).await;
    assert_ne!(other.recording_id, first.recording_id);
    assert!(!other.resumed);
    assert_eq!(open_count(&pool, Uuid::from_bytes(a.device_id)).await, 1);
}

#[sqlx::test(migrations = "./migrations")]
async fn session_requires_device_auth(pool: PgPool) {
    let app = app!(pool);
    let request = http::TestRequest::post()
        .uri("/recordings/sessions")
        .set_payload(encode(session(probe(1.0, 0.0276, "ahri"))))
        .to_request();
    assert_eq!(
        http::call_service(&app, request).await.status(),
        actix_web::http::StatusCode::UNAUTHORIZED
    );
}
