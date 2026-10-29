mod common;

use actix_web::{App, http::StatusCode, test as http};
use common::*;
use sqlx::PgPool;
use tutorlolv2_protocol::{
    CodeMode, DeviceCredentials, EndReason, EndRecording, GameResult, LcuGame, RecordingResult,
};
use tutorlolv2_server::server::{
    db_enums::{RiotGameSource, ShareDeleteReason},
    routes::{devices, recordings},
};
use uuid::Uuid;

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

async fn recording(app: &impl TestApp) -> (DeviceCredentials, Uuid) {
    let device = register(app).await;
    let started = start(app, &device, session(probe(120.0, 0.0276, "ahri"))).await;
    (device, Uuid::from_bytes(started.recording_id))
}

fn end_body(
    reason: EndReason,
    result: Option<GameResult>,
    final_game_time: Option<f32>,
) -> Vec<u8> {
    encode(EndRecording {
        reason,
        result,
        final_game_time,
    })
}

#[sqlx::test(migrations = "./migrations")]
async fn heartbeat_updates_last_ingest(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    sqlx::query!(
        "UPDATE recordings SET last_ingest_at = now() - interval '5 minutes' WHERE id = $1",
        id
    )
    .execute(&pool)
    .await
    .unwrap();
    let (status, _) = post_as(
        &app,
        &device,
        &format!("/recordings/{id}/heartbeat"),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let fresh = sqlx::query_scalar!(r#"SELECT last_ingest_at > now() - interval '5 seconds' AS "fresh!" FROM recordings WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(fresh);
}

#[sqlx::test(migrations = "./migrations")]
async fn heartbeat_reopens_timed_out_recording(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    sqlx::query!(
        "UPDATE recordings SET ended_at = now(), end_reason = $2 WHERE id = $1",
        id,
        EndReason::Timeout as EndReason
    )
    .execute(&pool)
    .await
    .unwrap();
    let (status, _) = post_as(
        &app,
        &device,
        &format!("/recordings/{id}/heartbeat"),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let open = sqlx::query_scalar!(r#"SELECT ended_at IS NULL AND end_reason IS NULL AS "open!" FROM recordings WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(open);
}

#[sqlx::test(migrations = "./migrations")]
async fn heartbeat_does_not_reopen_when_another_recording_is_open(pool: PgPool) {
    let app = app!(pool);
    let (device, old) = recording(&app).await;
    sqlx::query!(
        "UPDATE recordings SET ended_at = now(), end_reason = $2 WHERE id = $1",
        old,
        EndReason::Timeout as EndReason
    )
    .execute(&pool)
    .await
    .unwrap();
    start(&app, &device, session(probe(40.0, 0.0312, "zoe"))).await;
    let (status, _) = post_as(
        &app,
        &device,
        &format!("/recordings/{old}/heartbeat"),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test(migrations = "./migrations")]
async fn heartbeat_on_game_ended_recording_is_409(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    let uri = format!("/recordings/{id}/end");
    assert_eq!(
        post_as(
            &app,
            &device,
            &uri,
            end_body(EndReason::GameEndEvent, Some(GameResult::Win), None)
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );
    let (status, _) = post_as(
        &app,
        &device,
        &format!("/recordings/{id}/heartbeat"),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test(migrations = "./migrations")]
async fn end_is_idempotent_and_expires_live_code(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    sqlx::query!(
        "INSERT INTO share_codes (recording_id, code, mode) VALUES ($1, $2, $3)",
        id,
        "ABCDEFGH",
        CodeMode::Live as CodeMode
    )
    .execute(&pool)
    .await
    .unwrap();
    let uri = format!("/recordings/{id}/end");
    assert_eq!(
        post_as(
            &app,
            &device,
            &uri,
            end_body(EndReason::GameEndEvent, None, None)
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        post_as(
            &app,
            &device,
            &uri,
            end_body(EndReason::LcuEndOfGame, Some(GameResult::Lose), None)
        )
        .await
        .0,
        StatusCode::NO_CONTENT
    );

    let row = sqlx::query!(
        r#"SELECT end_reason AS "end_reason: EndReason", result AS "result: GameResult" FROM recordings WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        row.end_reason,
        Some(EndReason::GameEndEvent),
        "first end wins"
    );
    assert_eq!(
        row.result,
        Some(GameResult::Lose),
        "a missing result is filled in later"
    );

    let reason = sqlx::query_scalar!(
        r#"SELECT delete_reason AS "r: ShareDeleteReason" FROM share_codes WHERE code = $1"#,
        "ABCDEFGH"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(reason, Some(ShareDeleteReason::GameEnded));
}

#[sqlx::test(migrations = "./migrations")]
async fn end_with_timeout_reason_is_422(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    let (status, _) = post_as(
        &app,
        &device,
        &format!("/recordings/{id}/end"),
        end_body(EndReason::Timeout, None, None),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrations = "./migrations")]
async fn end_stores_game_length_from_final_game_time(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    let body = end_body(
        EndReason::GameEndEvent,
        Some(GameResult::Lose),
        Some(1020.3645),
    );
    assert_eq!(
        post_as(&app, &device, &format!("/recordings/{id}/end"), body)
            .await
            .0,
        StatusCode::NO_CONTENT
    );
    let length = sqlx::query_scalar!("SELECT game_length_s FROM recordings WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(length, Some(1020));
}

#[sqlx::test(migrations = "./migrations")]
async fn control_of_other_device_is_404(pool: PgPool) {
    let app = app!(pool);
    let (_, id) = recording(&app).await;
    let stranger = register(&app).await;
    assert_eq!(
        post_as(
            &app,
            &stranger,
            &format!("/recordings/{id}/heartbeat"),
            vec![]
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let body = end_body(EndReason::GameEndEvent, None, None);
    assert_eq!(
        post_as(&app, &stranger, &format!("/recordings/{id}/end"), body)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let result = encode(RecordingResult {
        result: GameResult::Win,
        game_length_s: None,
        riot_game: None,
    });
    assert_eq!(
        patch_as(&app, &stranger, &format!("/recordings/{id}/result"), result).await,
        StatusCode::NOT_FOUND
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn result_does_not_override_spectator_ids(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    sqlx::query!(
        "UPDATE recordings SET riot_game_id = 5669550383, queue_id = 2400, is_custom = false, riot_game_source = $2 WHERE id = $1",
        id,
        RiotGameSource::Spectator as RiotGameSource,
    )
    .execute(&pool)
    .await
    .unwrap();
    let body = encode(RecordingResult {
        result: GameResult::Lose,
        game_length_s: Some(1020),
        riot_game: Some(LcuGame {
            game_id: 1,
            queue_id: Some(1),
            is_custom: Some(true),
        }),
    });
    assert_eq!(
        patch_as(&app, &device, &format!("/recordings/{id}/result"), body).await,
        StatusCode::NO_CONTENT
    );
    let row = sqlx::query!(
        r#"SELECT result AS "result: GameResult", game_length_s, riot_game_id, queue_id, is_custom FROM recordings WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.result, Some(GameResult::Lose));
    assert_eq!(row.game_length_s, Some(1020));
    assert_eq!(
        (row.riot_game_id, row.queue_id, row.is_custom),
        (Some(5669550383), Some(2400), Some(false))
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn result_fills_lcu_ids_when_not_verified(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;
    let body = encode(RecordingResult {
        result: GameResult::Win,
        game_length_s: None,
        riot_game: Some(LcuGame {
            game_id: 5669550383,
            queue_id: Some(2400),
            is_custom: Some(false),
        }),
    });
    assert_eq!(
        patch_as(&app, &device, &format!("/recordings/{id}/result"), body).await,
        StatusCode::NO_CONTENT
    );
    let row = sqlx::query!(
        r#"SELECT riot_game_id, riot_game_source AS "source: RiotGameSource" FROM recordings WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (row.riot_game_id, row.source),
        (Some(5669550383), Some(RiotGameSource::Lcu))
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn result_with_only_game_id_keeps_known_queue(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let mut request = session(probe(120.0, 0.0276, "ahri"));
    request.lcu = Some(LcuGame {
        game_id: 111,
        queue_id: Some(450),
        is_custom: Some(false),
    });
    let id = Uuid::from_bytes(start(&app, &device, request).await.recording_id);

    let body = encode(RecordingResult {
        result: GameResult::Win,
        game_length_s: Some(1000),
        riot_game: Some(LcuGame {
            game_id: 111,
            queue_id: None,
            is_custom: None,
        }),
    });
    assert_eq!(
        patch_as(&app, &device, &format!("/recordings/{id}/result"), body).await,
        StatusCode::NO_CONTENT
    );
    let row = sqlx::query!(
        "SELECT riot_game_id, queue_id, is_custom FROM recordings WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (row.riot_game_id, row.queue_id, row.is_custom),
        (Some(111), Some(450), Some(false))
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn heartbeat_racing_an_end_never_reopens(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app).await;

    // Hold the row so the heartbeat's UPDATE waits while an end commits underneath it
    let mut tx = pool.begin().await.unwrap();
    sqlx::query!("SELECT id FROM recordings WHERE id = $1 FOR UPDATE", id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let uri = format!("/recordings/{id}/heartbeat");
    let heartbeat = post_as(&app, &device, &uri, vec![]);
    let ender = async {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        sqlx::query!(
            "UPDATE recordings SET ended_at = now(), end_reason = $2 WHERE id = $1",
            id,
            EndReason::GameEndEvent as EndReason
        )
        .execute(&mut *tx)
        .await
        .unwrap();
        tx.commit().await.unwrap();
    };
    let ((status, _), ()) = tokio::join!(heartbeat, ender);

    assert_eq!(status, StatusCode::CONFLICT);
    let row = sqlx::query!(r#"SELECT ended_at IS NOT NULL AS "ended!", end_reason AS "reason: EndReason" FROM recordings WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        (row.ended, row.reason),
        (true, Some(EndReason::GameEndEvent))
    );
}
