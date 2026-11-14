mod common;
use actix_web::{App, http::StatusCode, test as http};
use common::*;
use protocol::*;
use server::server::routes::{api, devices, recordings};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "./migrations")]
async fn owner_history_authorization_and_finished_lifecycle(pool: PgPool) {
    let app = http::init_service(
        App::new()
            .app_data(state(pool.clone()))
            .service(devices(None))
            .service(recordings(None))
            .service(api(None)),
    )
    .await;
    let owner = register(&app).await;
    let other = register(&app).await;
    let started = start(&app, &owner, session(probe(60., 0., "ahri"))).await;
    let id = Uuid::from_bytes(started.recording_id);
    for suffix in ["".to_owned(), format!("/{id}"), format!("/{id}/frames/0")] {
        let uri = format!("/api/me/recordings{suffix}");
        assert_eq!(
            http::call_service(&app, http::TestRequest::get().uri(&uri).to_request())
                .await
                .status(),
            StatusCode::UNAUTHORIZED
        );
    }
    for suffix in [format!("/{id}"), format!("/{id}/frames/0")] {
        assert_eq!(
            http::call_service(
                &app,
                http::TestRequest::get()
                    .uri(&format!("/api/me/recordings{suffix}"))
                    .insert_header(("Authorization", auth_header(&other)))
                    .to_request()
            )
            .await
            .status(),
            StatusCode::NOT_FOUND
        );
    }
    for device in [None, Some(&other)] {
        let mut request = http::TestRequest::put()
            .uri(&format!("/api/me/recordings/{id}/share"))
            .set_payload(encode(SetShareMode {
                mode: ShareMode::Public,
            }));
        if let Some(device) = device {
            request = request.insert_header(("Authorization", auth_header(device)));
        }
        assert_eq!(
            http::call_service(&app, request.to_request())
                .await
                .status(),
            if device.is_some() {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::UNAUTHORIZED
            }
        );
    }
    let list = http::call_service(
        &app,
        http::TestRequest::get()
            .uri("/api/me/recordings")
            .insert_header(("Authorization", auth_header(&owner)))
            .to_request(),
    )
    .await;
    assert_eq!(list.status(), StatusCode::OK);
    assert_eq!(list.headers().get("Cache-Control").unwrap(), "no-store");
    let page: HistoryPage = decode(&http::read_body(list).await);
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].summary.recording.id, started.recording_id);
    for (mode, expected) in [
        (ShareMode::Live, StatusCode::BAD_REQUEST),
        (ShareMode::Public, StatusCode::CONFLICT),
    ] {
        assert_eq!(
            http::call_service(
                &app,
                http::TestRequest::put()
                    .uri(&format!("/api/me/recordings/{id}/share"))
                    .insert_header(("Authorization", auth_header(&owner)))
                    .set_payload(encode(SetShareMode { mode }))
                    .to_request()
            )
            .await
            .status(),
            expected
        );
    }
    sqlx::query!(
        r#"
        UPDATE recordings
        SET ended_at = now(),
            end_reason = 'Timeout',
            is_custom = FALSE
        WHERE id = $1
        "#,
        id,
    )
    .execute(&pool)
    .await
    .unwrap();
    for policy in [Some(true), None] {
        sqlx::query!(
            r#"
            UPDATE recordings
            SET is_custom = $2
            WHERE id = $1
            "#,
            id,
            policy,
        )
        .execute(&pool)
        .await
        .unwrap();
        let response = http::call_service(
            &app,
            http::TestRequest::put()
                .uri(&format!("/api/me/recordings/{id}/share"))
                .insert_header(("Authorization", auth_header(&owner)))
                .set_payload(encode(SetShareMode {
                    mode: ShareMode::Public,
                }))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    sqlx::query!(
        r#"
        UPDATE recordings
        SET is_custom = FALSE,
                        is_practice = TRUE
        WHERE id = $1
        "#,
        id,
    )
    .execute(&pool)
    .await
    .unwrap();
    let response = http::call_service(
        &app,
        http::TestRequest::put()
            .uri(&format!("/api/me/recordings/{id}/share"))
            .insert_header(("Authorization", auth_header(&owner)))
            .set_payload(encode(SetShareMode {
                mode: ShareMode::Public,
            }))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    sqlx::query!(
        r#"
        UPDATE recordings
        SET is_practice = FALSE
        WHERE id = $1
        "#,
        id,
    )
    .execute(&pool)
    .await
    .unwrap();
    let uri = format!("/api/me/recordings/{id}/share");
    let req = || {
        http::TestRequest::put()
            .uri(&uri)
            .insert_header(("Authorization", auth_header(&owner)))
            .set_payload(encode(SetShareMode {
                mode: ShareMode::Public,
            }))
            .to_request()
    };
    let (a, b) = tokio::join!(
        http::call_service(&app, req()),
        http::call_service(&app, req())
    );
    assert_eq!(a.status(), StatusCode::OK);
    assert_eq!(b.status(), StatusCode::OK);
    let a: ShareState = decode(&http::read_body(a).await);
    let b: ShareState = decode(&http::read_body(b).await);
    assert_eq!(a.code, b.code);
    let mut codes = vec![];
    for mode in [
        ShareMode::Public,
        ShareMode::Public,
        ShareMode::Private,
        ShareMode::Public,
    ] {
        let response = http::call_service(
            &app,
            http::TestRequest::put()
                .uri(&format!("/api/me/recordings/{id}/share"))
                .insert_header(("Authorization", auth_header(&owner)))
                .set_payload(encode(SetShareMode { mode }))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let share: ShareState = decode(&http::read_body(response).await);
        codes.push(share.code);
    }
    assert_eq!(codes[0], codes[1]);
    assert_eq!(codes[2], None);
    assert_ne!(codes[0], codes[3]);
}

#[sqlx::test(migrations = "./migrations")]
async fn bounded_cursor_pages_and_owner_frames(pool: PgPool) {
    let app = http::init_service(
        App::new()
            .app_data(state(pool.clone()))
            .service(devices(None))
            .service(recordings(None))
            .service(api(None)),
    )
    .await;
    let owner = register(&app).await;
    let other = register(&app).await;
    let started = start(&app, &owner, session(probe(60., 0., "ahri"))).await;
    let id = Uuid::from_bytes(started.recording_id);
    sqlx::query!(
        r#"
        UPDATE recordings
        SET ended_at = now(),
            end_reason = 'Timeout'
        WHERE id = $1
        "#,
        id,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query!(
        r#"
        INSERT INTO recordings (
            device_id,
            game_mode,
            map_number,
            fingerprint,
            probe,
            active_riot_id,
            champion,
            started_at_estimate,
            first_game_time,
            last_game_time,
            app_version,
            ended_at,
            end_reason
        )
        SELECT $1,
                       'CLASSIC',
                       11,
                       decode(repeat('00', 32), 'hex'),
                       decode('00', 'hex'),
                       'Host#1',
                       'ahri',
                       now(),
                       0,
                       60,
                       'test',
                       now(),
                       'Timeout'
                FROM generate_series(1, 104)
        "#,
        Uuid::from_bytes(owner.device_id),
    )
    .execute(&pool)
    .await
    .unwrap();
    for uri in ["?limit=0", "?limit=101", "?before=garbage", "/not-uuid"] {
        assert_eq!(
            http::call_service(
                &app,
                http::TestRequest::get()
                    .uri(&format!("/api/me/recordings{uri}"))
                    .insert_header(("Authorization", auth_header(&owner)))
                    .to_request()
            )
            .await
            .status(),
            StatusCode::BAD_REQUEST
        );
    }
    let mut ids = vec![];
    let mut cursor = None;
    loop {
        let uri = cursor.map_or_else(
            || "/api/me/recordings".into(),
            |id| format!("/api/me/recordings?before={}", Uuid::from_bytes(id)),
        );
        let res = http::call_service(
            &app,
            http::TestRequest::get()
                .uri(&uri)
                .insert_header(("Authorization", auth_header(&owner)))
                .to_request(),
        )
        .await;
        assert_eq!(res.status(), StatusCode::OK);
        let page: HistoryPage = decode(&http::read_body(res).await);
        if cursor.is_none() {
            assert_eq!(page.items.len(), 50);
        }
        ids.extend(page.items.iter().map(|i| i.summary.recording.id));
        cursor = page.next_before;
        if cursor.is_none() {
            break;
        }
    }
    assert_eq!(ids.len(), 105);
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), 105);
    for limit in [1, 100] {
        let res = http::call_service(
            &app,
            http::TestRequest::get()
                .uri(&format!("/api/me/recordings?limit={limit}"))
                .insert_header(("Authorization", auth_header(&owner)))
                .to_request(),
        )
        .await;
        let page: HistoryPage = decode(&http::read_body(res).await);
        assert_eq!(page.items.len(), limit);
    }
    // A persisted unsupported chunk stays visible, but is never mislabelled derived data.
    sqlx::query!(
        r#"
        INSERT INTO recording_chunks (
            recording_id,
            seq,
            first_game_time,
            last_game_time,
            snapshot_count,
            raw_format,
            raw_sha256,
            raw_size,
            RAW,
            frames_format,
            frames
        ) VALUES (
            $1,
            0,
            30,
            40,
            1,
            1,
            decode(repeat('00', 32), 'hex'),
            1,
            decode('7b7d', 'hex'),
            0,
            decode('00', 'hex')
        )
        "#,
        id,
    )
    .execute(&pool)
    .await
    .unwrap();
    for (device, seq, expected) in [
        (&owner, 0, StatusCode::UNPROCESSABLE_ENTITY),
        (&owner, 1, StatusCode::NOT_FOUND),
        (&other, 0, StatusCode::NOT_FOUND),
    ] {
        let res = http::call_service(
            &app,
            http::TestRequest::get()
                .uri(&format!("/api/me/recordings/{id}/frames/{seq}"))
                .insert_header(("Authorization", auth_header(device)))
                .to_request(),
        )
        .await;
        assert_eq!(res.status(), expected);
    }
    let res = http::call_service(
        &app,
        http::TestRequest::get()
            .uri(&format!("/api/me/recordings/{id}"))
            .insert_header(("Authorization", auth_header(&owner)))
            .to_request(),
    )
    .await;
    let details: RecordingDetails = decode(&http::read_body(res).await);
    assert_eq!(details.chunks.len(), 1);
    assert_eq!(details.chunks[0].frames_format, 0);
    let plain = encode(math::riot::FrameChunk {
        format: FRAMES_FORMAT,
        frames: vec![],
    });
    let mut bytes = Vec::new();
    {
        use std::io::Write;
        let mut writer = brotli::CompressorWriter::new(&mut bytes, 4096, 5, 24);
        writer.write_all(&plain).unwrap();
    }
    sqlx::query!(
        r#"
        UPDATE recording_chunks
        SET frames_format = $2,
            frames = $3
        WHERE recording_id = $1
        "#,
        id,
        FRAMES_FORMAT as i16,
        &bytes,
    )
    .execute(&pool)
    .await
    .unwrap();
    let res = http::call_service(
        &app,
        http::TestRequest::get()
            .uri(&format!("/api/me/recordings/{id}/frames/0"))
            .insert_header(("Authorization", auth_header(&owner)))
            .to_request(),
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Encoding").unwrap(), "br");
    assert_eq!(res.headers().get("Cache-Control").unwrap(), "no-store");
    let body = http::read_body(res).await;
    assert_eq!(body.as_ref(), bytes);
    let mut decoded = Vec::new();
    {
        use std::io::Read;
        brotli::Decompressor::new(body.as_ref(), 4096)
            .read_to_end(&mut decoded)
            .unwrap();
    }
    let (chunk, used) = bincode::borrow_decode_from_slice::<math::riot::FrameChunk, _>(
        &decoded,
        types::BINCODE_CONFIG,
    )
    .unwrap();
    assert_eq!(used, decoded.len());
    assert_eq!(chunk.format, FRAMES_FORMAT);
    // Even cached valid auth must not expose a newly soft-deleted device's records.
    sqlx::query!(
        r#"
        UPDATE devices
        SET deleted_at = now()
        WHERE id = $1
        "#,
        Uuid::from_bytes(owner.device_id),
    )
    .execute(&pool)
    .await
    .unwrap();
    let res = http::call_service(
        &app,
        http::TestRequest::get()
            .uri(&format!("/api/me/recordings/{id}"))
            .insert_header(("Authorization", auth_header(&owner)))
            .to_request(),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migrations")]
async fn invalid_persisted_summary_never_wraps_or_fabricates_zero(pool: PgPool) {
    let app = http::init_service(
        App::new()
            .app_data(state(pool.clone()))
            .service(devices(None))
            .service(recordings(None))
            .service(api(None)),
    )
    .await;
    let owner = register(&app).await;
    let started = start(&app, &owner, session(probe(60., 0., "ahri"))).await;
    sqlx::query!(
        r#"
        UPDATE recordings
        SET coverage_s = - 1,
                           kills = 300
        WHERE id = $1
        "#,
        Uuid::from_bytes(started.recording_id),
    )
    .execute(&pool)
    .await
    .unwrap();
    let res = http::call_service(
        &app,
        http::TestRequest::get()
            .uri("/api/me/recordings")
            .insert_header(("Authorization", auth_header(&owner)))
            .to_request(),
    )
    .await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[sqlx::test(migrations = "./migrations")]
async fn owner_lookup_plans_use_existing_indexes(pool: PgPool) {
    let app = http::init_service(
        App::new()
            .app_data(state(pool.clone()))
            .service(devices(None)),
    )
    .await;
    let device = register(&app).await;
    let device = Uuid::from_bytes(device.device_id);
    let other = Uuid::from_bytes(register(&app).await.device_id);
    sqlx::query!(
        r#"
        INSERT INTO recordings (
            device_id,
            game_mode,
            map_number,
            fingerprint,
            probe,
            active_riot_id,
            champion,
            started_at_estimate,
            first_game_time,
            last_game_time,
            app_version,
            ended_at,
            end_reason
        )
        SELECT CASE
                           WHEN n % 100 = 0 THEN $1::UUID
                           ELSE $2::UUID
                       END,
                       'CLASSIC',
                       11,
                       decode(repeat('00', 32), 'hex'),
                       decode('00', 'hex'),
                       'Host#1',
                       'ahri',
                       now(),
                       0,
                       60,
                       'test',
                       now(),
                       'Timeout'
                FROM generate_series(1, 10000) n
        "#,
        device,
        other,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query!(
        r#"
        ANALYZE recordings
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    let plan = sqlx::query_scalar!(
        r#"
        EXPLAIN
        SELECT r.id,
               s.code
        FROM recordings r
        LEFT JOIN share_codes s ON s.recording_id = r.id
        AND s.deleted_at IS NULL
        WHERE r.device_id = $1
            AND r.deleted_at IS NULL
            AND r.id < $2
        ORDER BY r.id DESC
        LIMIT 51
        "#,
        device,
        Uuid::max(),
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let plan = plan
        .into_iter()
        .map(Option::unwrap)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(plan.contains("recordings_by_device"), "{plan}");
    let plan = sqlx::query_scalar!(
        r#"
        EXPLAIN
        SELECT frames
        FROM recording_chunks
        WHERE recording_id = $1
            AND seq = $2
        "#,
        Uuid::max(),
        0i32,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let plan = plan
        .into_iter()
        .map(Option::unwrap)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(plan.contains("recording_chunks_pkey"), "{plan}");
}
