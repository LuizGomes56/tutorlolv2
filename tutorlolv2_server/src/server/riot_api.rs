//! Riot API enrichment (spec §9.3): ACCOUNT-V1 resolves the host's PUUID,
//! SPECTATOR-V5 confirms the match and gives the real game id.

use super::db_enums::RiotGameSource;
use serde::Deserialize;
use sqlx::PgPool;
use std::{collections::BTreeSet, sync::Mutex, time::Duration};
use tokio::sync::mpsc;
use tutorlolv2_protocol::{GameType, Platform, Region, Regional};
use uuid::Uuid;

const USER_AGENT: &str = concat!("tutorlolv2-server/", env!("CARGO_PKG_VERSION"));
const QUEUE_CAPACITY: usize = 1024;
/// Attempts per request while Riot answers 429.
const MAX_ATTEMPTS: u32 = 5;
const DEFAULT_RETRY_AFTER: Duration = Duration::from_secs(1);

/// Enrichment of one new recording.
#[derive(Clone, Debug)]
pub struct EnrichJob {
    pub recording_id: Uuid,
    /// `gameName#tagLine` of the recording's player.
    pub riot_id: String,
    pub platform: Platform,
}

/// Hands jobs to the enrichment worker. Disabled without `RIOT_API_KEY`.
#[derive(Clone)]
pub struct EnrichQueue(Option<mpsc::Sender<EnrichJob>>);

impl EnrichQueue {
    pub fn disabled() -> Self {
        Self(None)
    }

    /// Never blocks: a full queue drops the job (the recording keeps its LCU values).
    pub fn push(&self, job: EnrichJob) {
        if let Some(sender) = &self.0
            && let Err(e) = sender.try_send(job)
        {
            eprintln!("Riot enrichment queue: {e}");
        }
    }
}

/// Logs a region the platform table doesn't know, once per value.
pub fn log_unmapped_region(region: &Region) {
    static SEEN: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

    if SEEN.lock().unwrap().insert(region.as_str().to_owned()) {
        eprintln!("No Riot platform for LCU region {region}; enrichment skipped");
    }
}

pub struct RiotClient {
    http: reqwest::Client,
    key: String,
    /// Replaces `https://{host}.api.riotgames.com` (tests only).
    base_override: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnrichOutcome {
    /// Riot confirmed the match: the recording is verified.
    Spectator,
    /// Not spectatable (Practice Tool, some customs): LCU values stay.
    NotSpectatable,
    /// Riot's current game isn't this recording's game (e.g. a session sent
    /// late after an outage, while the player is already in the next game).
    Mismatch,
    GaveUp,
}

/// Riot's `gameStartTime` and the recording's estimated start may differ by
/// loading time and clock error, never by a whole game.
const START_TOLERANCE_MS: i64 = 10 * 60 * 1000;

enum Reply {
    Ok(actix_web::web::Bytes),
    NotFound,
    Failed,
}

/// ACCOUNT-V1 `AccountDto`, only the field used.
#[derive(Deserialize)]
struct AccountDto {
    puuid: String,
}

/// SPECTATOR-V5 `CurrentGameInfo`, only the fields used. Participants and the
/// observer encryption key are never deserialized (spec §9.3).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActiveGameDto {
    game_id: u64,
    platform_id: Platform,
    game_type: GameType,
    game_queue_config_id: Option<u16>,
    /// Unix ms; 0 or missing before the game starts.
    #[serde(default)]
    game_start_time: Option<i64>,
}

/// Percent-encodes one URL path segment (everything but RFC 3986 unreserved characters).
fn encode_segment(segment: &str) -> String {
    segment
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

impl RiotClient {
    pub fn new(key: String, base_override: Option<String>) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(10))
            .build()
            .expect("static client configuration is valid");

        Self {
            http,
            key,
            base_override,
        }
    }

    fn base(&self, host: &str) -> String {
        match &self.base_override {
            Some(base) => base.trim_end_matches('/').to_owned(),
            None => format!("https://{host}.api.riotgames.com"),
        }
    }

    /// `None` when `riot_id` has no `#tagLine`.
    pub fn account_url(&self, regional: Regional, riot_id: &str) -> Option<String> {
        let (game_name, tag_line) = riot_id.rsplit_once('#')?;

        Some(format!(
            "{}/riot/account/v1/accounts/by-riot-id/{}/{}",
            self.base(regional.as_str()),
            encode_segment(game_name),
            encode_segment(tag_line)
        ))
    }

    /// `None` for a platform without a known host.
    pub fn spectator_url(&self, platform: &Platform, puuid: &str) -> Option<String> {
        Some(format!(
            "{}/lol/spectator/v5/active-games/by-summoner/{}",
            self.base(platform.host()?),
            encode_segment(puuid)
        ))
    }

    /// GET with the API key, waiting out `429`s (Riot's service limits answer
    /// them too) up to [`MAX_ATTEMPTS`] times.
    async fn get(&self, url: &str) -> Reply {
        for _ in 0..MAX_ATTEMPTS {
            let response = match self
                .http
                .get(url)
                .header("X-Riot-Token", &self.key)
                .send()
                .await
            {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Riot API request failed: {e}");
                    return Reply::Failed;
                }
            };

            match response.status().as_u16() {
                200 => return response.bytes().await.map_or(Reply::Failed, Reply::Ok),
                404 => return Reply::NotFound,
                429 => {
                    let wait = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse().ok())
                        .map_or(DEFAULT_RETRY_AFTER, Duration::from_secs);

                    tokio::time::sleep(wait).await;
                }
                status => {
                    eprintln!("Riot API answered {status} for {url}");
                    return Reply::Failed;
                }
            }
        }
        Reply::Failed
    }
}

/// PUUID of `riot_id`, from the 24 h cache or ACCOUNT-V1.
async fn puuid(
    db: &PgPool,
    riot: &RiotClient,
    regional: Regional,
    riot_id: &str,
) -> Option<String> {
    let cached = sqlx::query_scalar!(
        "
        SELECT puuid
        FROM riot_id_cache
        WHERE riot_id = $1
            AND resolved_at > now() - interval '24 hours'
        ",
        riot_id
    )
    .fetch_optional(db)
    .await
    .ok()?;

    if cached.is_some() {
        return cached;
    }

    let Reply::Ok(body) = riot.get(&riot.account_url(regional, riot_id)?).await else {
        return None;
    };

    let account = serde_json::from_slice::<AccountDto>(&body).ok()?;

    sqlx::query!(
        "
        INSERT INTO riot_id_cache (riot_id, puuid)
        VALUES ($1, $2)
        ON CONFLICT (riot_id)
        DO UPDATE
        SET puuid = excluded.puuid, resolved_at = now()
        ",
        riot_id,
        account.puuid
    )
    .execute(db)
    .await
    .ok()?;

    Some(account.puuid)
}

pub async fn enrich_one(db: &PgPool, riot: &RiotClient, job: &EnrichJob) -> EnrichOutcome {
    let Some(regional) = job.platform.regional() else {
        return EnrichOutcome::GaveUp;
    };

    let Some(puuid) = puuid(db, riot, regional, &job.riot_id).await else {
        return EnrichOutcome::GaveUp;
    };

    let Some(url) = riot.spectator_url(&job.platform, &puuid) else {
        return EnrichOutcome::GaveUp;
    };

    match riot.get(&url).await {
        Reply::Ok(body) => {
            let Ok(game) = serde_json::from_slice::<ActiveGameDto>(&body) else {
                return EnrichOutcome::GaveUp;
            };

            match is_same_game(db, job.recording_id, &game).await {
                Some(true) => {}
                Some(false) => {
                    return store_puuid(db, job.recording_id, &puuid, EnrichOutcome::Mismatch)
                        .await;
                }
                None => return EnrichOutcome::GaveUp,
            }

            let updated = sqlx::query!(
                "
                UPDATE recordings
                SET riot_game_id = $2,
                    platform_id = $3,
                    riot_game_source = $4,
                    queue_id = coalesce($5, queue_id),
                    is_custom = coalesce(is_custom, $6),
                    active_puuid = $7
                WHERE id = $1
                ",
                job.recording_id,
                game.game_id as i64,
                game.platform_id.as_str(),
                RiotGameSource::Spectator,
                game.game_queue_config_id.map(i32::from),
                !game.game_type.is_matched(),
                puuid,
            )
            .execute(db)
            .await;

            match updated {
                Ok(_) => EnrichOutcome::Spectator,
                Err(_) => EnrichOutcome::GaveUp,
            }
        }
        Reply::NotFound => {
            store_puuid(db, job.recording_id, &puuid, EnrichOutcome::NotSpectatable).await
        }
        Reply::Failed => EnrichOutcome::GaveUp,
    }
}

/// Whether Riot's current game is the recording's game: the League client's
/// game id must match when the recording has one; otherwise the start times
/// must be close. `None` on a database error.
async fn is_same_game(db: &PgPool, recording_id: Uuid, game: &ActiveGameDto) -> Option<bool> {
    let row = sqlx::query!(
        r#"
        SELECT riot_game_id,
            riot_game_source,
            (extract(epoch FROM started_at_estimate) * 1000)::bigint AS "started_ms!"
        FROM recordings
        WHERE id = $1
        "#,
        recording_id
    )
    .fetch_one(db)
    .await
    .ok()?;

    if row.riot_game_source == Some(RiotGameSource::Lcu)
        && let Some(lcu_game_id) = row.riot_game_id
    {
        return Some(lcu_game_id == game.game_id as i64);
    }

    Some(match game.game_start_time.filter(|&start| start > 0) {
        Some(start) => (start - row.started_ms).abs() <= START_TOLERANCE_MS,
        None => true,
    })
}

/// Keeps the host's PUUID without trusting Riot's game.
async fn store_puuid(
    db: &PgPool,
    recording_id: Uuid,
    puuid: &str,
    outcome: EnrichOutcome,
) -> EnrichOutcome {
    match sqlx::query!(
        "
        UPDATE recordings
        SET active_puuid = $2
        WHERE id = $1
        ",
        recording_id,
        puuid
    )
    .execute(db)
    .await
    {
        Ok(_) => outcome,
        Err(_) => EnrichOutcome::GaveUp,
    }
}

/// Starts the single enrichment worker.
pub fn spawn(db: PgPool, riot: RiotClient) -> EnrichQueue {
    let (sender, mut receiver) = mpsc::channel::<EnrichJob>(QUEUE_CAPACITY);

    tokio::spawn(async move {
        while let Some(job) = receiver.recv().await {
            enrich_one(&db, &riot, &job).await;
        }
    });

    EnrichQueue(Some(sender))
}
