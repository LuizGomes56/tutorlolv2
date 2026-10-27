use {
    crate::{
        HTTP_CLIENT, JsonRead, LOL_VERSION, MayFail,
        riot::RiotCdn,
        scraper::{self, Catalog, Recommendations},
    },
    rayon::iter::{ParallelBridge, ParallelIterator},
    reqwest::{StatusCode, header::RETRY_AFTER},
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    std::{
        collections::{BTreeMap, BTreeSet, HashMap},
        hash::Hash,
        path::PathBuf,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::{sync::Mutex, task::JoinSet, time::Instant},
    tutorlolv2_types::Position,
};

fn cache() -> PathBuf {
    PathBuf::from("cache/stats")
}

/// Game patch of [`LOL_VERSION`], such as `16.21` for `16.21.1`, which is the
/// prefix of the game version of the matches played on it
fn patch() -> &'static str {
    LOL_VERSION
        .rsplit_once('.')
        .map_or(LOL_VERSION, |(patch, _)| patch)
}

/// Ladders and match lists only describe the patch they were sampled on, so
/// they are cached per patch. Matches themselves are cached by id in a shared
/// directory, so interrupted downloads resume without fetching them again
fn patch_cache(platform: &str) -> PathBuf {
    cache().join(platform).join(patch())
}

/// Where [`parse`] saves the recommendations computed from match data
pub(crate) fn output() -> PathBuf {
    cache().join("data").with_extension("json")
}

/// Every platform, paired with its regional routing value used by `match-v5`
pub const PLATFORMS: [(&str, &str); 17] = [
    ("br1", "americas"),
    ("la1", "americas"),
    ("la2", "americas"),
    ("na1", "americas"),
    ("eun1", "europe"),
    ("euw1", "europe"),
    ("me1", "europe"),
    ("ru", "europe"),
    ("tr1", "europe"),
    ("jp1", "asia"),
    ("kr", "asia"),
    ("oc1", "sea"),
    ("ph2", "sea"),
    ("sg2", "sea"),
    ("th2", "sea"),
    ("tw2", "sea"),
    ("vn2", "sea"),
];

pub const DEFAULT_PLATFORMS: [&str; 3] = ["kr", "euw1", "na1"];
pub const DEFAULT_MATCHES: usize = 10_000;

/// Which ladders are sampled by [`download`], and how many matches
pub struct StatsConfig {
    /// Platforms whose high elo ladders are sampled, as listed in [`PLATFORMS`]
    pub platforms: Vec<String>,
    /// Most recent matches downloaded from each platform
    pub matches: usize,
}

impl Default for StatsConfig {
    fn default() -> Self {
        Self {
            platforms: DEFAULT_PLATFORMS.map(String::from).to_vec(),
            matches: DEFAULT_MATCHES,
        }
    }
}

impl StatsConfig {
    /// Resolves every configured platform to its entry in [`PLATFORMS`]
    fn platforms(&self) -> MayFail<Vec<(&'static str, &'static str)>> {
        self.platforms
            .iter()
            .map(|name| {
                PLATFORMS
                    .into_iter()
                    .find(|(platform, _)| platform.eq_ignore_ascii_case(name))
                    .ok_or_else(|| format!("[stats] Unknown platform {name:?}").into())
            })
            .collect()
    }
}
const LEAGUES: [&str; 3] = ["challengerleagues", "grandmasterleagues", "masterleagues"];

/// Ranked solo/duo queue
const QUEUE: u16 = 420;
const MATCHES_PER_PLAYER: u8 = 20;
const SAMPLE_DAYS: u64 = 14;
/// Games shorter than this are remakes or early surrenders
const MIN_GAME_DURATION: u32 = 600;

/// The application rate limit is 500 requests every 10 seconds per host,
/// so requests are spaced to stay slightly below it
const REQUEST_INTERVAL: Duration = Duration::from_millis(22);
const CONCURRENCY: usize = 32;
const MAX_RETRIES: u32 = 5;

/// Champion positions with fewer games than this are not recommended
const MIN_GAMES: usize = 20;
const MAX_ITEMS: usize = 9;
/// Minimum fraction of the games in which an item must be completed
const MIN_ITEM_RATE: f64 = 0.1;

fn api_key() -> MayFail<String> {
    if let Ok(key) = std::env::var("RIOT_API_KEY") {
        return Ok(key);
    }

    crate::read_to_string(".env")?
        .lines()
        .find_map(|line| {
            let key = line.trim().strip_prefix("RIOT_API_KEY=")?;
            Some(key.trim().trim_matches('"').to_string())
        })
        .ok_or_else(|| "[stats] RIOT_API_KEY is not set in the environment or in .env".into())
}

/// Spaces out the requests made to a single host
struct Limiter {
    /// Time at which the next request may be sent
    next: Mutex<Instant>,
    /// Requests are held until this time after the host rate limits us
    paused_until: Mutex<Instant>,
}

impl Limiter {
    fn new() -> Self {
        Self {
            next: Mutex::new(Instant::now()),
            paused_until: Mutex::new(Instant::now()),
        }
    }

    async fn wait(&self) {
        loop {
            let at = {
                let mut next = self.next.lock().await;
                let at = (*next).max(Instant::now());

                *next = at + REQUEST_INTERVAL;
                at
            };

            tokio::time::sleep_until(at).await;

            // Requests may have reserved their time before another request was
            // rate limited, so they wait for the pause and reserve a new time
            let paused_until = *self.paused_until.lock().await;

            if paused_until <= Instant::now() {
                return;
            }

            tokio::time::sleep_until(paused_until).await;
        }
    }

    /// Holds every request to this host until `duration` has elapsed
    async fn pause(&self, duration: Duration) {
        let until = Instant::now() + duration;

        let mut paused_until = self.paused_until.lock().await;
        *paused_until = (*paused_until).max(until);
        drop(paused_until);

        let mut next = self.next.lock().await;
        *next = (*next).max(until);
    }
}

struct RiotApi {
    key: String,
    limiters: HashMap<&'static str, Limiter>,
}

impl RiotApi {
    fn new(platforms: &[(&'static str, &'static str)]) -> MayFail<Self> {
        Ok(Self {
            key: api_key()?,
            limiters: platforms
                .iter()
                .copied()
                .flat_map(|(platform, region)| [platform, region])
                .map(|host| (host, Limiter::new()))
                .collect(),
        })
    }

    /// Requests `path` from the given host, retrying when rate limited or
    /// when the server fails
    async fn get<T: DeserializeOwned>(&self, host: &str, path: &str) -> MayFail<T> {
        let limiter = self
            .limiters
            .get(host)
            .ok_or_else(|| format!("[stats] No rate limiter for host {host:?}"))?;

        let url = format!("https://{host}.api.riotgames.com{path}");

        for attempt in 0..MAX_RETRIES {
            limiter.wait().await;

            let response = HTTP_CLIENT
                .get(&url)
                .header("X-Riot-Token", &self.key)
                .send()
                .await?;

            let status = response.status();

            if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                let seconds = response
                    .headers()
                    .get(RETRY_AFTER)
                    .and_then(|v| v.to_str().ok()?.parse().ok())
                    .unwrap_or(1 << attempt);

                println!("[warn] {status} from {host}, retrying in {seconds}s");
                limiter.pause(Duration::from_secs(seconds)).await;

                continue;
            }

            return Ok(response.error_for_status()?.json().await?);
        }

        Err(format!("[stats] Too many retries: {url}").into())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Match {
    info: MatchInfo,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MatchInfo {
    game_version: String,
    game_duration: u32,
    participants: Vec<Participant>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Participant {
    champion_name: String,
    team_position: String,
    item0: u32,
    item1: u32,
    item2: u32,
    item3: u32,
    item4: u32,
    item5: u32,
    perks: Perks,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Perks {
    stat_perks: StatPerks,
    styles: Vec<PerkStyle>,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
struct StatPerks {
    offense: u32,
    flex: u32,
    defense: u32,
}

#[derive(Deserialize, Serialize)]
struct PerkStyle {
    style: u32,
    selections: Vec<PerkSelection>,
}

#[derive(Deserialize, Serialize)]
struct PerkSelection {
    perk: u32,
}

impl Participant {
    fn items(&self) -> [u32; 6] {
        [
            self.item0, self.item1, self.item2, self.item3, self.item4, self.item5,
        ]
    }

    fn perks(&self, style: usize) -> Vec<u32> {
        self.perks
            .styles
            .get(style)
            .map(|s| s.selections.iter().map(|s| s.perk).collect())
            .unwrap_or_default()
    }

    /// Primary style, secondary style and keystone, which define a rune page
    fn page(&self) -> Option<(u32, u32, u32)> {
        let [primary, secondary] = self.perks.styles.get(0..2)? else {
            return None;
        };

        Some((primary.style, secondary.style, *self.perks(0).first()?))
    }
}

/// Runs `f` for every item, keeping at most [`CONCURRENCY`] tasks in flight
async fn for_each_concurrent<T, F>(items: impl IntoIterator<Item = T>, f: impl Fn(T) -> F)
where
    F: Future<Output = MayFail> + Send + 'static,
{
    async fn join_next(tasks: &mut JoinSet<MayFail>) {
        match tasks.join_next().await {
            Some(Ok(Err(e))) => println!("[error] {e:?}"),
            Some(Err(e)) => println!("[error] Joining future: {e:?}"),
            _ => {}
        }
    }

    let mut tasks = JoinSet::new();

    for item in items {
        if tasks.len() >= CONCURRENCY {
            join_next(&mut tasks).await;
        }

        tasks.spawn(f(item));
    }

    while !tasks.is_empty() {
        join_next(&mut tasks).await;
    }
}

/// Collects the players of every league in [`LEAGUES`] for a platform
async fn players(api: &RiotApi, platform: &str) -> MayFail<Vec<String>> {
    let path = patch_cache(platform).join("players").with_extension("json");

    if let Ok(true) = std::fs::exists(&path) {
        println!("[exists] {path:?}");
        return Vec::from_file(path);
    }

    #[derive(Deserialize)]
    struct League {
        entries: Vec<LeagueEntry>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct LeagueEntry {
        puuid: String,
        league_points: u32,
    }

    let mut players = Vec::new();

    for league in LEAGUES {
        let path = format!("/lol/league/v4/{league}/by-queue/RANKED_SOLO_5x5");
        let mut data = api.get::<League>(platform, &path).await?;

        println!(
            "[stats] {platform} {league}: {} players",
            data.entries.len()
        );

        data.entries
            .sort_by_key(|e| std::cmp::Reverse(e.league_points));

        players.extend(data.entries.into_iter().map(|e| e.puuid));
    }

    crate::write(path, serde_json::to_string(&players)?)?;
    Ok(players)
}

/// Collects the ids of the recent ranked matches of the players, from the top
/// of the ladder down, until at least `target` distinct ids are found
async fn match_ids(
    api: &Arc<RiotApi>,
    platform: &'static str,
    region: &'static str,
    players: Vec<String>,
    target: usize,
) -> MayFail<BTreeSet<String>> {
    let path = patch_cache(platform)
        .join("match_ids")
        .with_extension("json");

    if let Ok(true) = std::fs::exists(&path) {
        let ids = BTreeSet::from_file(&path)?;

        if ids.len() >= target {
            println!("[exists] {path:?}");
            return Ok(ids);
        }
    }

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .saturating_sub(Duration::from_secs(SAMPLE_DAYS * 24 * 60 * 60))
        .as_secs();

    let done = Arc::new(AtomicUsize::new(0));
    let ids = Arc::new(std::sync::Mutex::new(BTreeSet::new()));

    let found = |ids: &std::sync::Mutex<BTreeSet<String>>| ids.lock().map_or(0, |ids| ids.len());

    let players = players.into_iter().take_while(|_| found(&ids) < target);

    for_each_concurrent(players, |puuid| {
        let (api, done, ids) = (api.clone(), done.clone(), ids.clone());

        async move {
            let path = format!(
                "/lol/match/v5/matches/by-puuid/{puuid}/ids\
                ?queue={QUEUE}&type=ranked&startTime={start_time}&count={MATCHES_PER_PLAYER}"
            );

            let result = api.get::<Vec<String>>(region, &path).await;
            let done = done.fetch_add(1, Ordering::Relaxed) + 1;

            if done % 500 == 0 {
                println!("[stats] {platform}: match lists {done}");
            }

            ids.lock().map_err(|e| format!("{e:?}"))?.extend(result?);

            Ok(())
        }
    })
    .await;

    let ids = std::mem::take(&mut *ids.lock().map_err(|e| format!("{e:?}"))?);

    crate::write(path, serde_json::to_string(&ids)?)?;

    Ok(ids)
}

fn match_path(match_id: &str) -> PathBuf {
    cache()
        .join("matches")
        .join(match_id)
        .with_extension("json")
}

async fn download_platform(
    api: Arc<RiotApi>,
    platform: &'static str,
    region: &'static str,
    target: usize,
) -> MayFail {
    let players = players(&api, platform).await?;

    // Match ids increase over time, so the greatest ones are the most recent
    let match_ids = match_ids(&api, platform, region, players, target)
        .await?
        .into_iter()
        .rev()
        .take(target)
        .filter(|id| !matches!(std::fs::exists(match_path(id)), Ok(true)))
        .collect::<Vec<_>>();

    let total = match_ids.len();
    let done = Arc::new(AtomicUsize::new(0));

    println!("[stats] {platform}: {total} matches to download");

    for_each_concurrent(match_ids, |match_id| {
        let (api, done) = (api.clone(), done.clone());

        async move {
            let path = format!("/lol/match/v5/matches/{match_id}");
            let result = api.get::<Match>(region, &path).await;

            let done = done.fetch_add(1, Ordering::Relaxed) + 1;

            if done % 500 == 0 || done == total {
                println!("[stats] {platform}: matches {done}/{total}");
            }

            // Only the fields declared in `Match` are kept
            crate::write(match_path(&match_id), serde_json::to_string(&result?)?)
        }
    })
    .await;

    Ok(())
}

/// Downloads the most recent ranked matches of the high elo players of every
/// configured platform, concurrently between platforms
pub async fn download(config: &StatsConfig) -> MayFail {
    println!("[fn] stats::download");

    let platforms = config.platforms()?;
    let api = Arc::new(RiotApi::new(&platforms)?);
    let mut tasks = JoinSet::new();

    for (platform, region) in platforms {
        let api = api.clone();
        let target = config.matches;

        tasks.spawn(async move {
            download_platform(api, platform, region, target)
                .await
                .map_err(|e| format!("[stats] {platform}: {e:?}"))
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => println!("[error] {e}"),
            Err(e) => println!("[error] Joining future: {e:?}"),
        }
    }

    Ok(())
}

/// Most common value, with ties broken by the smallest value so that the
/// result is the same on every run
fn mode<T: Copy + Ord + Hash>(values: impl Iterator<Item = T>) -> Option<T> {
    let mut counts = HashMap::new();

    for value in values {
        *counts.entry(value).or_insert(0usize) += 1;
    }

    counts
        .into_iter()
        .max_by(|(a, x), (b, y)| x.cmp(y).then(b.cmp(a)))
        .map(|(value, _)| value)
}

/// Items that can't be upgraded any further, and boots, which can only be
/// upgraded late in the game
fn completed_items() -> MayFail<BTreeSet<u32>> {
    #[derive(Deserialize)]
    struct Item {
        depth: Option<u8>,
        #[serde(default)]
        into: Vec<String>,
        #[serde(default)]
        tags: Vec<String>,
    }

    Ok(RiotCdn::<u32, Item>::from_file("cache/riot/items.json")?
        .data
        .into_iter()
        .filter(|(_, item)| {
            item.depth.is_some_and(|depth| depth >= 2)
                && (item.into.is_empty() || item.tags.iter().any(|tag| tag == "Boots"))
        })
        .map(|(id, _)| id)
        .collect())
}

/// Recommends the items completed in at least [`MIN_ITEM_RATE`] of the games
fn recommend_items(
    games: &[&Participant],
    completed: &BTreeSet<u32>,
    items: &Catalog,
) -> BTreeSet<String> {
    let mut counts = HashMap::<u32, usize>::new();

    for game in games {
        let built = game
            .items()
            .into_iter()
            .filter(|id| completed.contains(id))
            .collect::<BTreeSet<_>>();

        for id in built {
            *counts.entry(id).or_default() += 1;
        }
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();

    counts.sort_by(|(a, x), (b, y)| y.cmp(x).then(a.cmp(b)));
    counts
        .into_iter()
        .take_while(|(_, count)| *count as f64 / games.len() as f64 >= MIN_ITEM_RATE)
        .filter_map(|(id, _)| items.get(id).map(String::from))
        .take(MAX_ITEMS)
        .collect()
}

/// Recommends the most common rune page, then the most common choice of
/// every slot among the games that used that page
fn recommend_runes(games: &[&Participant], runes: &Catalog) -> BTreeSet<String> {
    let Some(page) = mode(games.iter().filter_map(|game| game.page())) else {
        return BTreeSet::new();
    };

    let games = games
        .iter()
        .filter(|game| game.page() == Some(page))
        .collect::<Vec<_>>();

    let mut perks = vec![page.2];

    for slot in 1..=3 {
        perks.extend(mode(
            games
                .iter()
                .filter_map(|game| game.perks(0).get(slot).copied()),
        ));
    }

    let secondary = mode(games.iter().filter_map(|game| {
        let [a, b] = game.perks(1)[..] else {
            return None;
        };

        Some((a.min(b), a.max(b)))
    }));

    perks.extend(secondary.into_iter().flat_map(|(a, b)| [a, b]));

    for shard in [
        |s: StatPerks| s.offense,
        |s: StatPerks| s.flex,
        |s: StatPerks| s.defense,
    ] {
        perks.extend(mode(games.iter().map(|game| shard(game.perks.stat_perks))));
    }

    perks
        .into_iter()
        .filter_map(|id| runes.get(id).map(String::from))
        .collect()
}

/// Aggregates the downloaded matches of the current patch into the most
/// common items and runes of every champion position
pub fn parse() -> MayFail {
    println!("[fn] stats::parse");

    let items = Catalog::items()?;
    let runes = Catalog::runes()?;
    let completed = completed_items()?;

    let champions = scraper::champion_ids()?
        .into_iter()
        .map(|champion_id| (champion_id.to_lowercase(), champion_id))
        .collect::<HashMap<_, _>>();

    let patch = format!("{}.", patch());

    let matches = crate::read_dir(cache().join("matches"))?
        .par_bridge()
        .filter_map(|entry| Match::from_file(entry.path()).ok())
        .filter(|m| m.info.game_version.starts_with(&patch))
        .filter(|m| m.info.game_duration >= MIN_GAME_DURATION)
        .collect::<Vec<_>>();

    println!("[stats] {} matches from patch {patch}x", matches.len());

    let mut groups = BTreeMap::<(&str, Position), Vec<&Participant>>::new();

    for participant in matches.iter().flat_map(|m| &m.info.participants) {
        let champion = champions.get(&participant.champion_name.to_lowercase());

        if let Some(champion_id) = champion
            && let Ok(position) = participant.team_position.parse::<Position>()
        {
            groups
                .entry((champion_id, position))
                .or_default()
                .push(participant);
        }
    }

    let mut recommendations = Recommendations::new();

    for ((champion_id, position), games) in groups {
        if games.len() < MIN_GAMES {
            continue;
        }

        recommendations
            .entry(champion_id.to_string())
            .or_default()
            .insert(
                position.role().to_string(),
                [
                    recommend_items(&games, &completed, &items),
                    recommend_runes(&games, &runes),
                ],
            );
    }

    let json = serde_json::to_string_pretty(&recommendations)?;
    crate::write(output(), &json)
}

pub async fn run(config: &StatsConfig) -> MayFail {
    download(config).await?;
    parse()?;
    scraper::concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn queued_requests_wait_for_a_later_pause() {
        let limiter = Arc::new(Limiter::new());
        let start = Instant::now();

        // The first request goes out immediately, the second reserves a time
        // shortly after it and is already sleeping when the host pauses
        limiter.wait().await;

        let queued = {
            let limiter = limiter.clone();
            tokio::spawn(async move {
                limiter.wait().await;
                Instant::now()
            })
        };

        tokio::time::sleep(Duration::from_millis(5)).await;
        limiter.pause(Duration::from_millis(200)).await;

        let sent_at = queued.await.unwrap();
        assert!(sent_at - start >= Duration::from_millis(200));
    }
}
