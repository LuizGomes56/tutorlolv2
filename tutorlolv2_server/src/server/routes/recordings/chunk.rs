use crate::server::{
    auth::DeviceAuth,
    codec::Response,
    ingest::{IngestError, derive_chunk, hex},
    state::AppState,
};
use actix_web::{
    HttpRequest, HttpResponse,
    error::{
        ErrorConflict, ErrorInternalServerError, ErrorNotFound, ErrorPayloadTooLarge,
        ErrorUnprocessableEntity,
    },
    web::{self, Bytes, Data, Path},
};
use tutorlolv2_protocol::RAW_CHUNK_FORMAT;
use uuid::Uuid;

use crate::server::recordings::*;

pub async fn upload_chunk(
    state: Data<AppState>,
    auth: DeviceAuth,
    path: Path<(Uuid, u32)>,
    req: HttpRequest,
    body: Bytes,
) -> Response {
    let (id, seq) = path.into_inner();
    let sha_header = req
        .headers()
        .get(CONTENT_SHA256)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .ok_or_else(|| ErrorUnprocessableEntity("Missing X-Content-Sha256"))?;

    let recording = sqlx::query!(
        r#"
        SELECT probe,
               active_riot_id
        FROM recordings
        WHERE id = $1
            AND device_id = $2
            AND deleted_at IS NULL
        "#,
        id,
        auth.device_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Unknown recording"))?;

    if let Some(response) = (RecordingService { db: &state.db })
        .existing_chunk(id, seq, &sha_header)
        .await?
    {
        return Ok(response);
    }

    let compressed = body.clone();
    let derived = web::block(move || derive_chunk(&compressed, seq, &sha_header))
        .await
        .map_err(ErrorInternalServerError)?
        .map_err(|e| match e {
            IngestError::TooLarge => ErrorPayloadTooLarge("Chunk too large once decompressed"),
            IngestError::Invalid(reason) => ErrorUnprocessableEntity(reason),
            IngestError::Mismatch(reason) => ErrorConflict(reason),
        })?;

    let stored = RecordingService::decode_probe(&recording.probe)
        .ok_or_else(|| ErrorInternalServerError("Stored probe is unreadable"))?;

    if derived.first.probe.active_riot_id != recording.active_riot_id
        || stored.consistent(&derived.first.probe).is_err()
    {
        return Err(ErrorConflict("The chunk belongs to another match"));
    }

    let mut tx = state.db.begin().await.map_err(ErrorInternalServerError)?;

    let inserted = sqlx::query!(
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
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11
        )
        ON CONFLICT(recording_id, seq) DO NOTHING
        "#,
        id,
        seq as i32,
        derived.first_game_time,
        derived.last_game_time,
        derived.snapshot_count as i16,
        RAW_CHUNK_FORMAT as i16,
        &derived.raw_sha256,
        derived.raw_size as i32,
        body.as_ref(),
        derived.frames_format as i16,
        derived.frames_br,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .rows_affected()
        == 1;

    if !inserted {
        // Lost a race with an identical or conflicting upload of the same seq
        drop(tx);

        let sha = hex(&derived.raw_sha256);

        return (RecordingService { db: &state.db })
            .existing_chunk(id, seq, &sha)
            .await?
            .ok_or_else(|| ErrorInternalServerError("Chunk vanished"));
    }

    let last = &derived.last;
    // Live-client JSON has no LCU identity. Keep the session's known Riot id
    // when advancing the snapshot fields used by subsequent resume checks.
    let mut latest_probe = last.probe.clone();
    latest_probe.riot_game = stored.riot_game;
    let last_probe = bincode::encode_to_vec(&latest_probe, tutorlolv2_types::BINCODE_CONFIG)
        .map_err(ErrorInternalServerError)?;

    let coverage = (derived.last_game_time - derived.first_game_time + 1.0).round() as i32;

    // SET expressions read the row as it was, so every CASE compares against the old last_game_time
    sqlx::query!(
        r#"
        UPDATE recordings
        SET last_ingest_at = now(),
            coverage_s = coverage_s + $2,
            first_game_time = least(first_game_time, $3),
            last_game_time = greatest(last_game_time, $4),
            probe = CASE
                        WHEN $4 > last_game_time THEN $5
                        ELSE probe
                    END,
                    kills = CASE
                                WHEN $4 > last_game_time THEN $6
                                ELSE kills
                            END,
                            deaths = CASE
                                         WHEN $4 > last_game_time THEN $7
                                         ELSE deaths
                                     END,
                                     assists = CASE
                                                   WHEN $4 > last_game_time THEN $8
                                                   ELSE assists
                                               END,
                                               creep_score = CASE
                                                                 WHEN $4 > last_game_time THEN $9
                                                                 ELSE creep_score
                                                             END,
                                                             LEVEL = CASE
                                                                         WHEN $4 > last_game_time THEN $10
                                                                         ELSE LEVEL
                                                                     END,
                                                                     champion = CASE
                                                                                    WHEN $4 > last_game_time THEN $11
                                                                                    ELSE champion
                                                                                END
        WHERE id = $1
        "#,
        id,
        coverage,
        derived.first_game_time,
        derived.last_game_time,
        last_probe,
        last.summary.kills as i16,
        last.summary.deaths as i16,
        last.summary.assists as i16,
        last.summary.creep_score as i16,
        last.summary.level as i16,
        last.summary.champion,
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())
}
