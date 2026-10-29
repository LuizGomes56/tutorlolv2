mod common;

use actix_web::{App, http::StatusCode, test as http};
use common::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::io::{Read, Write};
use tutorlolv2_protocol::*;
use tutorlolv2_server::server::{
    ingest::*,
    routes::{devices, recordings},
};
use uuid::Uuid;

/// A complete `allgamedata` body (valid for `inspect` and for `RiotRealtime`).
fn snapshot_json(t: f32, start_time: f64, champion_raw: &str, kills: u8) -> Vec<u8> {
    let stats = json!({
        "abilityPower": 0.0, "armor": 30.0, "physicalLethality": 0.0, "armorPenetrationPercent": 1.0,
        "attackDamage": 60.0, "attackSpeed": 0.7, "critChance": 0.0, "critDamage": 175.0,
        "currentHealth": 600.0, "magicPenetrationFlat": 0.0, "magicPenetrationPercent": 1.0,
        "magicResist": 30.0, "maxHealth": 600.0, "resourceMax": 400.0, "resourceValue": 400.0
    });
    let player = |riot_id: &str, team: &str, raw: &str, name: &str, kills: u8| {
        json!({
            "riotId": riot_id, "championName": name, "rawChampionName": format!("game_character_displayname_{raw}"),
            "rawSkinName": format!("game_character_displayname_{raw}"), "skinID": 0, "team": team, "isBot": false,
            "position": "MIDDLE", "level": 3, "items": [{"itemID": 1056}],
            "runes": {"keystone": {"id": 8005}},
            "scores": {"kills": kills, "deaths": 1, "assists": 2, "creepScore": 20}
        })
    };
    let body = json!({
        "activePlayer": {
            "riotId": ME, "level": 3, "championStats": stats, "fullRunes": {"generalRunes": [{"id": 8005}]},
            "abilities": {"Q": {"abilityLevel": 1}, "W": {"abilityLevel": 1}, "E": {"abilityLevel": 1}, "R": {"abilityLevel": 0}}
        },
        "allPlayers": [player(ME, "ORDER", champion_raw, champion_raw, kills), player("Foe#1", "CHAOS", "Zed", "Zed", 0)],
        "events": {"Events": [{"EventID": 0, "EventName": "GameStart", "EventTime": start_time}]},
        "gameData": {"gameMode": "CLASSIC", "gameTime": t, "mapName": "Map11", "mapNumber": 11, "mapTerrain": "Default"}
    });
    serde_json::to_vec(&body).unwrap()
}

fn brotli(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut out, 4096, 5, 24);
        writer.write_all(data).unwrap();
    }
    out
}

fn sha_hex(data: &[u8]) -> String {
    Sha256::digest(data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// `(brotli body, X-Content-Sha256)` of a chunk of `snapshots` = `(captured_at_ms, json)`.
fn encode_chunk(seq: u32, snapshots: Vec<(i64, Vec<u8>)>) -> (Vec<u8>, String) {
    let chunk =
        RawChunk {
            format: RAW_CHUNK_FORMAT,
            seq,
            snapshots:
                snapshots
                    .into_iter()
                    .map(|(captured_at_ms, json)| {
                        let game_time = serde_json::from_slice::<serde_json::Value>(&json).unwrap()
                            ["gameData"]["gameTime"]
                            .as_f64()
                            .unwrap() as f32;
                        RawSnapshot {
                            captured_at_ms,
                            game_time,
                            json,
                        }
                    })
                    .collect(),
        };
    let bincode = encode(&chunk);
    (brotli(&bincode), sha_hex(&bincode))
}

/// Snapshots every second from `from` to `to` (inclusive) of the match that started at `start_time`.
fn seconds(from: u32, to: u32, start_time: f64) -> Vec<(i64, Vec<u8>)> {
    (from..=to)
        .map(|t| {
            (
                1_800_000_000_000 + t as i64 * 1000,
                snapshot_json(t as f32, start_time, "Ahri", (t / 60) as u8),
            )
        })
        .collect()
}

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

async fn upload(
    app: &impl TestApp,
    device: &DeviceCredentials,
    id: [u8; 16],
    seq: u32,
    body: Vec<u8>,
    sha: &str,
) -> StatusCode {
    let request = http::TestRequest::post()
        .uri(&format!(
            "/recordings/{}/chunks/{seq}",
            Uuid::from_bytes(id)
        ))
        .insert_header(("Authorization", auth_header(device)))
        .insert_header(("X-Content-Sha256", sha))
        .set_payload(body)
        .to_request();
    http::call_service(app, request).await.status()
}

/// A device with a recording started from the first snapshot of the match at `start_time`.
async fn recording(app: &impl TestApp, start_time: f64) -> (DeviceCredentials, [u8; 16]) {
    let device = register(app).await;
    let first = inspect(&snapshot_json(60.0, start_time, "Ahri", 1)).unwrap();
    let started = start(app, &device, session(first.probe)).await;
    (device, started.recording_id)
}

#[test]
fn derive_rejects_zip_bomb() {
    let zeros = vec![0u8; 17 << 20];
    let body = brotli(&zeros);
    assert!(body.len() < MAX_CHUNK_BYTES);
    assert!(matches!(
        derive_chunk(&body, 0, &sha_hex(&zeros)),
        Err(IngestError::TooLarge)
    ));
}

#[test]
fn derive_rejects_bad_sha_and_trailing_bytes() {
    let (body, sha) = encode_chunk(0, seconds(60, 62, 0.0276));
    assert!(matches!(
        derive_chunk(&body, 0, "00"),
        Err(IngestError::Invalid(_))
    ));
    assert!(
        matches!(derive_chunk(&body, 1, &sha), Err(IngestError::Invalid(_))),
        "wrong seq"
    );

    let chunk = RawChunk {
        format: RAW_CHUNK_FORMAT,
        seq: 0,
        snapshots: vec![RawSnapshot {
            captured_at_ms: 1,
            game_time: 60.0,
            json: snapshot_json(60.0, 0.0276, "Ahri", 0),
        }],
    };
    let mut trailing = encode(&chunk);
    trailing.push(0);
    assert!(matches!(
        derive_chunk(&brotli(&trailing), 0, &sha_hex(&trailing)),
        Err(IngestError::Invalid(_))
    ));
}

#[test]
fn derive_builds_decodable_frames() {
    let (body, sha) = encode_chunk(4, seconds(60, 64, 0.0276));
    let derived = derive_chunk(&body, 4, &sha).unwrap();
    assert_eq!(derived.snapshot_count, 5);
    assert_eq!(
        (derived.first_game_time, derived.last_game_time),
        (60.0, 64.0)
    );
    assert_eq!(derived.last.summary.champion, "ahri");

    let mut frames = Vec::new();
    brotli::Decompressor::new(&derived.frames_br[..], 4096)
        .read_to_end(&mut frames)
        .unwrap();
    let (decoded, used): (FrameChunk, _) =
        bincode::borrow_decode_from_slice(&frames, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, frames.len());
    assert_eq!(decoded.format, FRAMES_FORMAT);
    assert_eq!(decoded.frames.len(), 5);
    assert_eq!(decoded.frames[0].1.active_player.riot_id, ME);
}

#[sqlx::test(migrations = "./migrations")]
async fn chunk_201_then_identical_200_then_different_409(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let (body, sha) = encode_chunk(0, seconds(60, 70, 0.0276));
    assert_eq!(
        upload(&app, &device, id, 0, body.clone(), &sha).await,
        StatusCode::CREATED
    );
    assert_eq!(
        upload(&app, &device, id, 0, body, &sha).await,
        StatusCode::OK
    );
    let (other, other_sha) = encode_chunk(0, seconds(60, 71, 0.0276));
    assert_eq!(
        upload(&app, &device, id, 0, other, &other_sha).await,
        StatusCode::CONFLICT
    );

    let row = sqlx::query!(
        r#"SELECT (SELECT count(*) FROM recording_chunks WHERE recording_id = r.id) AS "chunks!", kills, coverage_s
           FROM recordings r WHERE id = $1"#,
        Uuid::from_bytes(id),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((row.chunks, row.kills, row.coverage_s), (1, 1, 11));
}

#[sqlx::test(migrations = "./migrations")]
async fn chunk_from_other_match_is_409(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let (body, sha) = encode_chunk(0, seconds(60, 62, 0.0312));
    assert_eq!(
        upload(&app, &device, id, 0, body, &sha).await,
        StatusCode::CONFLICT
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn chunk_of_other_device_is_404(pool: PgPool) {
    let app = app!(pool);
    let (_, id) = recording(&app, 0.0276).await;
    let stranger = register(&app).await;
    let (body, sha) = encode_chunk(0, seconds(60, 62, 0.0276));
    assert_eq!(
        upload(&app, &stranger, id, 0, body, &sha).await,
        StatusCode::NOT_FOUND
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn invalid_snapshot_is_422(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let mut replay: serde_json::Value =
        serde_json::from_slice(&snapshot_json(61.0, 0.0276, "Ahri", 0)).unwrap();
    replay["activePlayer"] =
        json!({"error": "Spectator mode doesn't currently support this feature"});
    let (body, sha) = encode_chunk(
        0,
        vec![
            (1, snapshot_json(60.0, 0.0276, "Ahri", 0)),
            (2, serde_json::to_vec(&replay).unwrap()),
        ],
    );
    assert_eq!(
        upload(&app, &device, id, 0, body, &sha).await,
        StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn oversized_body_is_413(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let body = vec![0u8; MAX_CHUNK_BYTES + 1];
    assert_eq!(
        upload(&app, &device, id, 0, body, "00").await,
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn late_older_chunk_keeps_newest_summary(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let (newer, newer_sha) = encode_chunk(1, seconds(120, 180, 0.0276));
    let (older, older_sha) = encode_chunk(0, seconds(60, 119, 0.0276));
    assert_eq!(
        upload(&app, &device, id, 1, newer, &newer_sha).await,
        StatusCode::CREATED
    );
    assert_eq!(
        upload(&app, &device, id, 0, older, &older_sha).await,
        StatusCode::CREATED
    );

    let row = sqlx::query!(
        r#"SELECT last_game_time, first_game_time, kills,
                  (SELECT count(*) FROM recording_chunks WHERE recording_id = r.id) AS "chunks!"
           FROM recordings r WHERE id = $1"#,
        Uuid::from_bytes(id),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (
            row.last_game_time,
            row.first_game_time,
            row.kills,
            row.chunks
        ),
        (180.0, 60.0, 3, 2)
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn chunk_after_end_is_accepted_and_does_not_reopen(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    sqlx::query!(
        "UPDATE recordings SET ended_at = now(), end_reason = $2 WHERE id = $1",
        Uuid::from_bytes(id),
        EndReason::GameEndEvent as EndReason,
    )
    .execute(&pool)
    .await
    .unwrap();
    let (body, sha) = encode_chunk(0, seconds(60, 62, 0.0276));
    assert_eq!(
        upload(&app, &device, id, 0, body, &sha).await,
        StatusCode::CREATED
    );
    let ended = sqlx::query_scalar!(
        r#"SELECT ended_at IS NOT NULL AS "ended!" FROM recordings WHERE id = $1"#,
        Uuid::from_bytes(id)
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(ended);
}

#[sqlx::test(migrations = "./migrations")]
async fn chunk_with_skewed_host_clock_is_accepted(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let (past, past_sha) = encode_chunk(
        0,
        vec![
            (1, snapshot_json(60.0, 0.0276, "Ahri", 0)),
            (2, snapshot_json(61.0, 0.0276, "Ahri", 0)),
        ],
    );
    let year_2100 = 4_102_444_800_000;
    let (future, future_sha) =
        encode_chunk(1, vec![(year_2100, snapshot_json(62.0, 0.0276, "Ahri", 0))]);
    assert_eq!(
        upload(&app, &device, id, 0, past, &past_sha).await,
        StatusCode::CREATED
    );
    assert_eq!(
        upload(&app, &device, id, 1, future, &future_sha).await,
        StatusCode::CREATED
    );
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "needs local captures"]
async fn whole_aram_capture_ingests(pool: PgPool) {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../captures/aram-20261029-101315/live");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let snapshots: Vec<(i64, Vec<u8>)> = files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            (
                1_800_000_000_000 + i as i64 * 1000,
                std::fs::read(f).unwrap(),
            )
        })
        .collect();

    let app = app!(pool);
    let device = register(&app).await;
    let started = start(
        &app,
        &device,
        session(inspect(&snapshots[0].1).unwrap().probe),
    )
    .await;
    let mut total = 0;
    for (seq, chunk) in snapshots.chunks(60).enumerate() {
        let (body, sha) = encode_chunk(seq as u32, chunk.to_vec());
        total += body.len();
        assert_eq!(
            upload(&app, &device, started.recording_id, seq as u32, body, &sha).await,
            StatusCode::CREATED,
            "seq {seq}"
        );
    }
    assert!(total < 300_000, "{total} bytes");
    let last = sqlx::query_scalar!(
        "SELECT last_game_time FROM recordings WHERE id = $1",
        Uuid::from_bytes(started.recording_id)
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(last > 1020.0, "{last}");
}

/// The same snapshot with the active player's Riot ID written as JSON `\u` escapes.
fn escaped(json: Vec<u8>) -> Vec<u8> {
    String::from_utf8(json)
        .unwrap()
        .replace(ME, "\\u6df1\\u6d77\\u00e0#002")
        .into_bytes()
}

#[sqlx::test(migrations = "./migrations")]
async fn escaped_riot_ids_are_ingested_with_frames(pool: PgPool) {
    let app = app!(pool);
    let device = register(&app).await;
    let first = escaped(snapshot_json(60.0, 0.0276, "Ahri", 0));
    let started = start(&app, &device, session(inspect(&first).unwrap().probe)).await;
    let snapshots = (60..=62)
        .map(|t| {
            (
                1_800_000_000_000 + t as i64 * 1000,
                escaped(snapshot_json(t as f32, 0.0276, "Ahri", 0)),
            )
        })
        .collect();
    let (body, sha) = encode_chunk(0, snapshots);
    assert_eq!(
        upload(&app, &device, started.recording_id, 0, body, &sha).await,
        StatusCode::CREATED
    );

    let row = sqlx::query!(
        "SELECT frames_format, frames FROM recording_chunks WHERE recording_id = $1",
        Uuid::from_bytes(started.recording_id)
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.frames_format, FRAMES_FORMAT as i16);
    let mut frames = Vec::new();
    brotli::Decompressor::new(&row.frames[..], 4096)
        .read_to_end(&mut frames)
        .unwrap();
    let (decoded, _): (FrameChunk, _) =
        bincode::borrow_decode_from_slice(&frames, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(decoded.frames[0].1.active_player.riot_id, "深海à#002");
}

#[sqlx::test(migrations = "./migrations")]
async fn raw_data_is_kept_when_frames_cannot_be_derived(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, 0.0276).await;
    let without_stats = |t: f32| {
        let mut v: serde_json::Value =
            serde_json::from_slice(&snapshot_json(t, 0.0276, "Ahri", 0)).unwrap();
        v["activePlayer"]
            .as_object_mut()
            .unwrap()
            .remove("championStats");
        serde_json::to_vec(&v).unwrap()
    };
    let (body, sha) = encode_chunk(0, vec![(1, without_stats(60.0)), (2, without_stats(61.0))]);
    assert_eq!(
        upload(&app, &device, id, 0, body.clone(), &sha).await,
        StatusCode::CREATED
    );
    let row = sqlx::query!("SELECT frames_format, length(frames) AS frames_len, raw FROM recording_chunks WHERE recording_id = $1", Uuid::from_bytes(id))
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.frames_format, 0, "frames are flagged as not derived");
    assert_eq!(row.frames_len, Some(0));
    assert_eq!(row.raw, body, "the raw chunk is stored as uploaded");
}
