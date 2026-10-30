mod common;

use actix_web::{
    App,
    http::{Method, StatusCode},
    test as http,
};
use common::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::{
    io::{Read, Write},
    time::Duration,
};
use tutorlolv2::riot::{FrameChunk, RiotRealtime};
use tutorlolv2_protocol::*;
use tutorlolv2_server::server::routes::api;
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

macro_rules! app {
    ($pool:expr) => {
        http::init_service(App::new().app_data(state($pool.clone())).service(api(None))).await
    };
}

async fn request(
    app: &impl TestApp,
    method: Method,
    uri: &str,
    device: Option<&DeviceCredentials>,
    body: Vec<u8>,
) -> actix_web::dev::ServiceResponse {
    let mut req = http::TestRequest::default()
        .method(method)
        .uri(uri)
        .set_payload(body);
    if let Some(device) = device {
        req = req.insert_header(("Authorization", auth_header(device)));
    }
    http::call_service(app, req.to_request()).await
}

async fn recording(
    app: &impl TestApp,
    custom: Option<bool>,
    practice: bool,
) -> (DeviceCredentials, [u8; 16]) {
    let response = request(
        app,
        Method::POST,
        "/api/devices",
        None,
        encode(RegisterDevice {
            app_version: "0.3.0".into(),
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let device = decode(&http::read_body(response).await);
    let mut probe = inspect(&snapshot_json(60.0, 0.0276, "Ahri", 1))
        .unwrap()
        .probe;
    if practice {
        probe.game_mode = GameMode::PracticeTool;
    }
    let mut body = session(probe);
    body.lcu = custom.map(|is_custom| LcuGame {
        game_id: 123,
        queue_id: Some(420),
        is_custom: Some(is_custom),
    });
    let response = request(
        app,
        Method::POST,
        "/api/recordings/sessions",
        Some(&device),
        encode(body),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let started: SessionStarted = decode(&http::read_body(response).await);
    assert_eq!(started.share.mode, ShareMode::Private);
    assert!(started.share.code.is_none());
    (device, started.recording_id)
}

fn owner(id: [u8; 16], suffix: &str) -> String {
    format!("/api/recordings/{}/{suffix}", Uuid::from_bytes(id))
}
async fn share(
    app: &impl TestApp,
    device: &DeviceCredentials,
    id: [u8; 16],
    mode: ShareMode,
) -> ShareState {
    let response = request(
        app,
        Method::PUT,
        &owner(id, "share"),
        Some(device),
        encode(SetShareMode { mode }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK, "{}", owner(id, "share"));
    decode(&http::read_body(response).await)
}
async fn viewer_status(app: &impl TestApp, code: &str) -> StatusCode {
    request(
        app,
        Method::GET,
        &format!("/api/share/{code}"),
        None,
        vec![],
    )
    .await
    .status()
}
fn live_body(t: f32) -> Vec<u8> {
    brotli(&encode(RawSnapshot {
        captured_at_ms: 1_800_000_000_000 + t as i64 * 1000,
        game_time: t,
        json: snapshot_json(t, 0.0276, "Ahri", 1),
    }))
}
async fn push(app: &impl TestApp, device: &DeviceCredentials, id: [u8; 16], t: f32) -> StatusCode {
    request(
        app,
        Method::PUT,
        &owner(id, "live"),
        Some(device),
        live_body(t),
    )
    .await
    .status()
}
async fn end(app: &impl TestApp, device: &DeviceCredentials, id: [u8; 16]) {
    let response = request(
        app,
        Method::POST,
        &owner(id, "end"),
        Some(device),
        encode(EndRecording {
            reason: EndReason::GameEndEvent,
            result: None,
            final_game_time: Some(62.0),
        }),
    )
    .await;
    assert!(response.status().is_success());
}

#[sqlx::test(migrations = "./migrations")]
async fn lifecycle_retains_code_between_live_public_and_never_revives_revoked(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    let live = share(&app, &device, id, ShareMode::Live).await;
    let code = live.code.clone().unwrap();
    assert!(live.url.unwrap().ends_with(&format!("/watch/{code}")));
    let response = request(
        &app,
        Method::GET,
        &owner(id, "share"),
        Some(&device),
        vec![],
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let fetched: ShareState = decode(&http::read_body(response).await);
    assert_eq!(fetched.code.as_ref(), Some(&code));
    assert_eq!(
        share(&app, &device, id, ShareMode::Public)
            .await
            .code
            .as_ref(),
        Some(&code)
    );
    assert_eq!(
        share(&app, &device, id, ShareMode::Live)
            .await
            .code
            .as_ref(),
        Some(&code)
    );
    let normalized = code.replace('-', "").to_lowercase();
    assert_eq!(viewer_status(&app, &normalized).await, StatusCode::OK);
    let private = share(&app, &device, id, ShareMode::Private).await;
    assert!(private.code.is_none() && private.url.is_none());
    assert_eq!(viewer_status(&app, &code).await, StatusCode::GONE);
    for suffix in ["live?after=0", "frames/0"] {
        assert_eq!(
            request(
                &app,
                Method::GET,
                &format!("/api/share/{code}/{suffix}"),
                None,
                vec![]
            )
            .await
            .status(),
            StatusCode::GONE
        );
    }
    let next = share(&app, &device, id, ShareMode::Live)
        .await
        .code
        .unwrap();
    assert_ne!(next, code);
    end(&app, &device, id).await;
    assert_eq!(viewer_status(&app, &next).await, StatusCode::GONE);
    assert_eq!(push(&app, &device, id, 63.0).await, StatusCode::CONFLICT);
}

#[sqlx::test(migrations = "./migrations")]
async fn public_policy_rejects_custom_practice_and_unknown(pool: PgPool) {
    let app = app!(pool);
    for (custom, practice) in [(Some(true), false), (Some(false), true), (None, false)] {
        let (device, id) = recording(&app, custom, practice).await;
        let response = request(
            &app,
            Method::PUT,
            &owner(id, "share"),
            Some(&device),
            encode(SetShareMode {
                mode: ShareMode::Public,
            }),
        )
        .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(&http::read_body(response).await[..], b"public_not_allowed");
        assert!(
            share(&app, &device, id, ShareMode::Live)
                .await
                .code
                .is_some()
        );
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn owner_auth_invalid_codes_and_live_body_limits(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    let (stranger, _) = recording(&app, Some(false), false).await;
    assert_eq!(push(&app, &device, id, 61.0).await, StatusCode::CONFLICT);
    for suffix in ["share", "live"] {
        let body = if suffix == "share" {
            encode(SetShareMode {
                mode: ShareMode::Live,
            })
        } else {
            live_body(61.0)
        };
        assert_eq!(
            request(&app, Method::PUT, &owner(id, suffix), None, body.clone())
                .await
                .status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            request(&app, Method::PUT, &owner(id, suffix), Some(&stranger), body)
                .await
                .status(),
            StatusCode::NOT_FOUND
        );
    }
    for code in ["invalid", "!!!!!!!!", "ZZZZ-ZZZZ"] {
        assert_eq!(viewer_status(&app, code).await, StatusCode::NOT_FOUND);
    }
    share(&app, &device, id, ShareMode::Live).await;
    assert_eq!(
        request(
            &app,
            Method::PUT,
            &owner(id, "live"),
            Some(&device),
            vec![0; 65537]
        )
        .await
        .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        request(
            &app,
            Method::PUT,
            &owner(id, "live"),
            Some(&device),
            brotli(&vec![0; 524289])
        )
        .await
        .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn live_poll_receives_new_frame_and_revocation_or_end_wakes_waiters(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    let code = share(&app, &device, id, ShareMode::Live)
        .await
        .code
        .unwrap();
    assert_eq!(push(&app, &device, id, 61.0).await, StatusCode::NO_CONTENT);
    let response = request(
        &app,
        Method::GET,
        &format!("/api/share/{code}/live?after=0"),
        None,
        vec![],
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let seq: u64 = response
        .headers()
        .get("X-Frame-Seq")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(seq > 0);
    assert!(response.headers().contains_key("X-Captured-At"));
    assert!(response.headers().contains_key("X-Game-Time"));
    let bytes = http::read_body(response).await;
    let (frame, used): (RiotRealtime<'_>, _) =
        bincode::borrow_decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(frame.active_player.riot_id, ME);
    let poll_uri = format!("/api/share/{code}/live?after={seq}");
    let (response, ()) = tokio::time::timeout(Duration::from_secs(10), async {
        tokio::join!(request(&app, Method::GET, &poll_uri, None, vec![]), async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            assert_eq!(push(&app, &device, id, 62.0).await, StatusCode::NO_CONTENT);
        })
    })
    .await
    .expect("new frame must wake long poll");
    assert_eq!(response.status(), StatusCode::OK);
    let seq2: u64 = response
        .headers()
        .get("X-Frame-Seq")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(seq2 > seq);
    for ended in [false, true] {
        let code = if ended {
            share(&app, &device, id, ShareMode::Live)
                .await
                .code
                .unwrap()
        } else {
            code.clone()
        };
        let uri = format!("/api/share/{code}/live?after={seq2}");
        let (response, ()) = tokio::time::timeout(Duration::from_secs(10), async {
            tokio::join!(request(&app, Method::GET, &uri, None, vec![]), async {
                tokio::time::sleep(Duration::from_millis(50)).await;
                if ended {
                    end(&app, &device, id).await;
                } else {
                    share(&app, &device, id, ShareMode::Private).await;
                }
            })
        })
        .await
        .expect("revocation/end must wake long poll");
        assert_eq!(response.status(), StatusCode::GONE);
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn public_metadata_and_brotli_frames_survive_end(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    let code = share(&app, &device, id, ShareMode::Public)
        .await
        .code
        .unwrap();
    let chunk = RawChunk {
        format: RAW_CHUNK_FORMAT,
        seq: 0,
        snapshots: vec![RawSnapshot {
            captured_at_ms: 1_800_000_061_000,
            game_time: 61.0,
            json: snapshot_json(61.0, 0.0276, "Ahri", 1),
        }],
    };
    let bytes = encode(chunk);
    let sha: String = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let response = http::call_service(
        &app,
        http::TestRequest::post()
            .uri(&owner(id, "chunks/0"))
            .insert_header(("Authorization", auth_header(&device)))
            .insert_header(("X-Content-Sha256", sha))
            .set_payload(brotli(&bytes))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
    end(&app, &device, id).await;
    let response = request(
        &app,
        Method::GET,
        &format!("/api/share/{code}"),
        None,
        vec![],
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let info: ShareInfo = decode(&http::read_body(response).await);
    assert_eq!(info.mode, ShareMode::Public);
    assert!(matches!(info.status, ShareStatus::Ended { .. }));
    assert_eq!(info.summary.id, id);
    assert_eq!(info.chunks.len(), 1);
    let response = request(
        &app,
        Method::GET,
        &format!("/api/share/{code}/frames/0"),
        None,
        vec![],
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("Content-Encoding").unwrap(), "br");
    let body = http::read_body(response).await;
    let mut bytes = vec![];
    brotli::Decompressor::new(&body[..], 4096)
        .read_to_end(&mut bytes)
        .unwrap();
    let (frames, used): (FrameChunk<'_>, _) =
        bincode::borrow_decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(frames.frames.len(), 1);
    assert_eq!(frames.frames[0].1.active_player.riot_id, ME);
    assert_eq!(
        request(
            &app,
            Method::GET,
            &format!("/api/share/{code}/live?after=0"),
            None,
            vec![]
        )
        .await
        .status(),
        StatusCode::GONE
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn viewer_limit_releases_permit_when_poll_completes(pool: PgPool) {
    let mut settings = config();
    settings.max_viewers_per_code = 1;
    let app = http::init_service(
        App::new()
            .app_data(state_with(pool, settings))
            .service(api(None)),
    )
    .await;
    let (device, id) = recording(&app, Some(false), false).await;
    let code = share(&app, &device, id, ShareMode::Live)
        .await
        .code
        .unwrap();
    let uri = format!("/api/share/{code}/live?after=0");
    let (response, ()) = tokio::time::timeout(Duration::from_secs(10), async {
        tokio::join!(request(&app, Method::GET, &uri, None, vec![]), async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            assert_eq!(
                request(&app, Method::GET, &uri, None, vec![])
                    .await
                    .status(),
                StatusCode::TOO_MANY_REQUESTS
            );
            assert_eq!(push(&app, &device, id, 61.0).await, StatusCode::NO_CONTENT);
        })
    })
    .await
    .expect("first admitted viewer receives published frame");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        request(&app, Method::GET, &uri, None, vec![])
            .await
            .status(),
        StatusCode::OK,
        "completed long poll releases its viewer permit"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn live_rejects_wrong_match_and_trailing_snapshot_bytes(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    share(&app, &device, id, ShareMode::Live).await;
    let snapshot = RawSnapshot {
        captured_at_ms: 1_800_000_061_000,
        game_time: 61.0,
        json: snapshot_json(61.0, 0.0312, "Ahri", 1),
    };
    assert_eq!(
        request(
            &app,
            Method::PUT,
            &owner(id, "live"),
            Some(&device),
            brotli(&encode(snapshot))
        )
        .await
        .status(),
        StatusCode::CONFLICT
    );
    let snapshot = RawSnapshot {
        captured_at_ms: 1_800_000_061_000,
        game_time: 61.0,
        json: snapshot_json(61.0, 0.0276, "Ahri", 1),
    };
    let mut bytes = encode(snapshot);
    bytes.push(0);
    assert_eq!(
        request(
            &app,
            Method::PUT,
            &owner(id, "live"),
            Some(&device),
            brotli(&bytes)
        )
        .await
        .status(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn janitor_wakes_poll_and_resume_does_not_revive_live_code(pool: PgPool) {
    let app = app!(pool);
    let (device, id) = recording(&app, Some(false), false).await;
    let code = share(&app, &device, id, ShareMode::Live)
        .await
        .code
        .unwrap();
    sqlx::query!(
        "UPDATE recordings SET last_ingest_at = now() - interval '20 minutes' WHERE id = $1",
        Uuid::from_bytes(id)
    )
    .execute(&pool)
    .await
    .unwrap();
    let uri = format!("/api/share/{code}/live?after=0");
    let (response, ()) = tokio::time::timeout(Duration::from_secs(10), async {
        tokio::join!(request(&app, Method::GET, &uri, None, vec![]), async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            assert_eq!(
                tutorlolv2_server::server::janitor::expire_stale(&pool, 900)
                    .await
                    .unwrap(),
                1
            );
        })
    })
    .await
    .expect("expired capability must interrupt waiting viewer");
    assert_eq!(response.status(), StatusCode::GONE);
    let mut body = session(
        inspect(&snapshot_json(61.0, 0.0276, "Ahri", 1))
            .unwrap()
            .probe,
    );
    body.lcu = Some(LcuGame {
        game_id: 123,
        queue_id: Some(420),
        is_custom: Some(false),
    });
    let response = request(
        &app,
        Method::POST,
        "/api/recordings/sessions",
        Some(&device),
        encode(body),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let resumed: SessionStarted = decode(&http::read_body(response).await);
    assert!(resumed.resumed);
    assert_eq!(resumed.recording_id, id);
    assert_eq!(resumed.share.mode, ShareMode::Private);
    assert!(resumed.share.code.is_none());
    assert_eq!(viewer_status(&app, &code).await, StatusCode::GONE);
    assert_ne!(
        share(&app, &device, id, ShareMode::Live)
            .await
            .code
            .unwrap(),
        code
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn unknown_code_lookups_have_separate_anti_enumeration_budget(pool: PgPool) {
    use tutorlolv2_server::server::routes::RateLimits;
    let app = http::init_service(
        App::new()
            .app_data(state(pool))
            .service(api(Some(RateLimits::new(false)))),
    )
    .await;
    for attempt in 0..21 {
        let response = http::call_service(
            &app,
            http::TestRequest::get()
                .uri("/api/share/ZZZZ-ZZZZ")
                .peer_addr(std::net::SocketAddr::from((
                    [127, 0, 0, 1],
                    12345 + attempt,
                )))
                .to_request(),
        )
        .await;
        assert_eq!(
            response.status(),
            if attempt < 20 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::TOO_MANY_REQUESTS
            },
            "unknown lookup {}",
            attempt + 1
        );
    }
}
