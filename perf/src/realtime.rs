//! Synthetic Live Client Data fixtures. JSON parsing always happens before timing.
use super::{Measurement, Result, Settings, SplitMix64, StatsMode, generate, summarize};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
use tutorlolv2::{ChampionId, Position, riot::*};

#[derive(Serialize, Deserialize)]
pub struct RealtimeWorkload<'a> {
    pub schema_version: u32,
    pub generator: String,
    pub seed: u64,
    pub enemies_per_game: usize,
    /// Allies excluding the active player.
    pub allies_per_game: usize,
    #[serde(borrow)]
    pub games: Vec<RiotRealtime<'a>>,
}

impl RealtimeWorkload<'_> {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 || self.games.is_empty() {
            return Err("expected realtime schema 1 and at least one game".into());
        }

        let expected_players = self
            .enemies_per_game
            .checked_add(self.allies_per_game)
            .and_then(|n| n.checked_add(1))
            .ok_or("player count overflow")?;

        for game in &self.games {
            let player = game
                .all_players
                .iter()
                .find(|p| p.riot_id == game.active_player.riot_id)
                .ok_or("active player must be present in allPlayers")?;

            if !matches!(player.team, "ORDER" | "CHAOS")
                || game
                    .all_players
                    .iter()
                    .any(|p| !matches!(p.team, "ORDER" | "CHAOS"))
            {
                return Err("expected ORDER/CHAOS teams".into());
            }

            if game.all_players.len() != expected_players
                || game
                    .all_players
                    .iter()
                    .filter(|p| p.team != player.team)
                    .count()
                    != self.enemies_per_game
            {
                return Err("realtime player counts do not match corpus metadata".into());
            }
        }

        Ok(())
    }
}

const fn position(champion: ChampionId) -> &'static str {
    match champion.main_position() {
        Position::Top => "TOP",
        Position::Jungle => "JUNGLE",
        Position::Middle => "MIDDLE",
        Position::Bottom => "BOTTOM",
        Position::Support => "SUPPORT",
    }
}

/// Return serialized input so the benchmark can borrow strings directly from the
/// JSON buffer through the production RiotRealtime deserializer.
pub fn generate_realtime(
    seed: u64,
    games: usize,
    enemies: usize,
    allies: usize,
) -> Result<Vec<u8>> {
    let players = enemies.checked_add(allies).ok_or("player count overflow")?;
    let source = generate(seed, games, players, StatsMode::Explicit)?;

    let mut rng = SplitMix64(seed ^ 0x6c69766567616d65);
    let mut output = Vec::with_capacity(games);

    for (game_index, game) in source.games.iter().enumerate() {
        let active = &game.active_player;
        let own_id = format!("bench-{game_index}-self#TEST");

        let own_team = if game_index % 2 == 0 {
            "ORDER"
        } else {
            "CHAOS"
        };
        let other_team = if own_team == "ORDER" {
            "CHAOS"
        } else {
            "ORDER"
        };

        let mut stats = active.data.stats.ok_or("explicit player stats required")?;

        // Reverse base100(): live percent penetration is the remaining fraction.
        stats.armor_penetration_percent = 1.0 - stats.armor_penetration_percent / 100.0;
        stats.magic_penetration_percent = 1.0 - stats.magic_penetration_percent / 100.0;

        let make_player = |champion: ChampionId,
                           mega: bool,
                           level,
                           items: &[tutorlolv2::ItemId],
                           id: &str,
                           team,
                           rng: &mut SplitMix64| {
            json!({
                "championName": if mega {
                    "Mega Gnar"
                } else {
                    champion.name()
                },
                // Enum variants use the game's internal champion keys.
                "rawSkinName": format!("game_character_skin_displayname_{champion:?}_0"),
                "rawChampionName": if mega {
                    "game_character_displayname_GnarBig".to_owned()
                } else {
                    format!("game_character_displayname_{champion:?}")
                },
                "riotId": id,
                "team": team,
                "position": position(champion),
                "level": level,
                "items": items.iter().map(|item| RiotItems {
                    item_id: item.riot_id()
                }).collect::<Vec<_>>(),
                "scores": RiotScoreboard {
                    kills: rng.index(16) as u8,
                    deaths: rng.index(16) as u8,
                    assists: rng.index(31) as u16,
                    creep_score: rng.index(401) as u16
                }
            })
        };

        let mut all_players = vec![make_player(
            active.data.champion_id,
            active.data.is_mega_gnar,
            active.data.level,
            &active.data.items,
            &own_id,
            own_team,
            &mut rng,
        )];

        for (i, player) in game.enemy_players.iter().enumerate() {
            let id = format!("bench-{game_index}-player-{i}#TEST");
            let team = if i < allies { own_team } else { other_team };

            all_players.push(make_player(
                player.champion_id,
                player.is_mega_gnar,
                player.level,
                &player.items,
                &id,
                team,
                &mut rng,
            ));
        }

        let mut events = Vec::new();

        for (dragon, count, killer) in [
            (
                "Earth",
                game.dragons.ally_earth_dragons,
                Some(own_id.clone()),
            ),
            ("Fire", game.dragons.ally_fire_dragons, Some(own_id.clone())),
            ("Chemtech", rng.index(3) as u16, Some(own_id.clone())),
            (
                "Earth",
                game.dragons.enemy_earth_dragons,
                if enemies > 0 {
                    Some(format!("bench-{game_index}-player-{allies}#TEST"))
                } else {
                    None
                },
            ),
        ] {
            if let Some(killer) = killer {
                for _ in 0..count {
                    events.push(json!({ "DragonType": dragon, "KillerName": killer }));
                }
            }
        }

        // Most live events have no damage-relevant dragon type.
        for _ in 0..96 {
            events.push(json!({ "DragonType": null, "KillerName": null }));
        }

        let ranks = active.abilities;
        let runes = if game_index % 4 == 3 {
            None
        } else {
            Some(
                active
                    .runes
                    .iter()
                    .map(|rune| RiotGeneralRunes { id: rune.riot_id() })
                    .collect::<Vec<_>>(),
            )
        };

        output.push(json!({
            "activePlayer": {
                "riotId": own_id,
                "level": active.data.level,
                "championStats": stats,
                "abilities": RiotAbilities {
                    q: RiotAbility { ability_level: ranks.q },
                    w: RiotAbility { ability_level: ranks.w },
                    e: RiotAbility { ability_level: ranks.e },
                    r: RiotAbility { ability_level: ranks.r }
                },
                "fullRunes": {
                    "generalRunes": runes
                }
            },
            "allPlayers": all_players,
            "events": {
                "Events": events
            },
            "gameData": RiotRealtimeGameData {
                game_time: 600.0 + rng.index(1801) as f32,
                map_number: 11
            }
        }));
    }

    Ok(serde_json::to_vec_pretty(&json!({
        "schema_version": 1,
        "generator": "live-splitmix64-v1",
        "seed": seed,
        "enemies_per_game": enemies,
        "allies_per_game": allies,
        "games": output
    }))?)
}

pub fn run_realtime(games: &[RiotRealtime<'_>], settings: Settings) -> Result<Measurement> {
    if games.is_empty() || settings.iterations == 0 || settings.samples == 0 {
        return Err("games, iterations, and samples must be nonzero".into());
    }

    let measured_calls = settings
        .iterations
        .checked_mul(settings.samples)
        .ok_or("call count overflow")?;

    // Validate errors before timing; successful calls below measure the Result too.
    for game in games {
        drop(tutorlolv2::realtime::realtime(game).map_err(|error| error.to_string())?);
    }

    let mut cursor = 0;

    for _ in 0..settings.warmup {
        drop(black_box(tutorlolv2::realtime::realtime(black_box(
            &games[cursor],
        ))));

        cursor += 1;

        if cursor == games.len() {
            cursor = 0;
        }
    }

    let mut overhead = Duration::ZERO;

    for _ in 0..1000 {
        let start = Instant::now();
        overhead += start.elapsed();
    }

    let mut samples = Vec::with_capacity(settings.samples);

    for _ in 0..settings.samples {
        let mut elapsed = Duration::ZERO;

        for _ in 0..settings.iterations {
            let input = black_box(&games[cursor]);
            let start = Instant::now();
            let output = black_box(tutorlolv2::realtime::realtime(input));

            elapsed += start.elapsed();
            drop(output);
            cursor += 1;

            if cursor == games.len() {
                cursor = 0;
            }
        }

        samples.push(elapsed.as_nanos() as f64 / settings.iterations as f64);
    }

    let summary = summarize(&samples)?;

    Ok(Measurement {
        settings,
        measured_calls,
        samples_ns_per_call: samples,
        timer_overhead_mean_ns: overhead.as_nanos() as f64 / 1000.0,
        summary,
    })
}
