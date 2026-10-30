//! Repeatable calculator workloads and batched wall-clock measurements.

pub mod realtime;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    hint::black_box,
    num::NonZeroU8,
    time::{Duration, Instant},
};
use strum::VariantArray;
use tutorlolv2::{
    ChampionId, GameMap, ItemId, RuneId,
    calculator::{InferStats, calculator},
    model::*,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum StatsMode {
    Inferred,
    Explicit,
    Mixed,
}

#[derive(Serialize, Deserialize)]
pub struct Workload {
    pub schema_version: u32,
    pub generator: String,
    pub seed: u64,
    pub enemies_per_game: usize,
    pub stats_mode: StatsMode,
    pub games: Vec<OwnedInputGame>,
}

impl Workload {
    pub fn prepare(&self) -> Result<Vec<PreparedGame<'_>>> {
        if self.schema_version != 1 || self.games.is_empty() {
            return Err("expected workload schema 1 and at least one game".into());
        }

        self.games
            .iter()
            .map(|game| {
                if game.enemy_players.len() != self.enemies_per_game {
                    return Err("enemy count does not match workload metadata".into());
                }

                // Explicit borrowed views: deliberately do not use OwnedInputGame's
                // existing transmute-based conversion in the performance harness.
                let enemy_players = game
                    .enemy_players
                    .iter()
                    .map(|enemy| InputMinData {
                        stats: enemy.stats,
                        items: &enemy.items,
                        item_exceptions: &enemy.item_exceptions,
                        stacks: enemy.stacks,
                        level: enemy.level,
                        champion_id: enemy.champion_id,
                        is_mega_gnar: enemy.is_mega_gnar,
                    })
                    .collect();

                Ok(PreparedGame {
                    active_player: (&game.active_player).into(),
                    enemy_players,
                    dragons: game.dragons,
                })
            })
            .collect()
    }
}

// Fixed algorithm rather than a dependency-version-dependent RNG. Generation is
// outside timing, and the JSON is the authoritative input for future comparisons.
struct SplitMix64(u64);

impl SplitMix64 {
    const fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    const fn index(&mut self, len: usize) -> usize {
        (self.next() % len as u64) as usize
    }

    const fn explicit(&mut self, mode: StatsMode) -> bool {
        match mode {
            StatsMode::Explicit => true,
            StatsMode::Inferred => false,
            StatsMode::Mixed => self.next() & 1 == 0,
        }
    }
}

fn random_player<T>(rng: &mut SplitMix64, items: &[ItemId]) -> OwnedInputMinData<T> {
    let champion_id = ChampionId::VARIANTS[rng.index(ChampionId::VARIANTS.len())];
    let count = rng.index(7).min(items.len());
    let mut inventory = Vec::with_capacity(count);

    while inventory.len() < count {
        let item = items[rng.index(items.len())];

        if !inventory.contains(&item) {
            inventory.push(item);
        }
    }

    OwnedInputMinData {
        champion_id,
        level: NonZeroU8::new(6 + rng.index(13) as u8).unwrap(),
        items: inventory.into_boxed_slice(),
        stacks: rng.index(101) as u32,
        is_mega_gnar: matches!(champion_id, ChampionId::Gnar) && rng.next() & 1 == 0,
        ..Default::default()
    }
}

pub fn generate(
    seed: u64,
    games: usize,
    enemies: usize,
    stats_mode: StatsMode,
) -> Result<Workload> {
    if games == 0 {
        return Err("at least one game is required".into());
    }

    let mut rng = SplitMix64(seed);
    let items: Vec<_> = ItemId::VARIANTS
        .iter()
        .copied()
        .filter(|item| {
            let data = item.data();
            data.purchasable
                && data.price > 0
                && data.tier >= 2
                && item.has_map(GameMap::SummonersRift)
        })
        .collect();

    if items.is_empty() {
        return Err("no purchasable items available".into());
    }

    let mut result = Vec::with_capacity(games);

    for _ in 0..games {
        let mut data = random_player::<PlayerStats>(&mut rng, &items);
        let level = data.level.get();
        let r = match level {
            16.. => 3,
            11.. => 2,
            _ => 1,
        };

        let mut ranks = [0u8; 3];
        let mut remaining = level - r;

        while remaining > 0 {
            let i = rng.index(3);
            if ranks[i] < 5 {
                ranks[i] += 1;
                remaining -= 1;
            }
        }

        let abilities = AbilityLevels {
            q: ranks[0],
            w: ranks[1],
            e: ranks[2],
            r,
        };

        // A random damaging keystone and independently selected secondary effects.
        let keystones = [
            RuneId::Electrocute,
            RuneId::DarkHarvest,
            RuneId::ArcaneComet,
            RuneId::PressTheAttack,
        ];

        let secondaries = [
            RuneId::AbsoluteFocus,
            RuneId::Waterwalking,
            RuneId::CoupDeGrace,
            RuneId::LastStand,
            RuneId::AxiomArcanist,
        ];

        let mut runes = vec![keystones[rng.index(keystones.len())]];

        while runes.len() < 4 {
            let rune = secondaries[rng.index(secondaries.len())];
            if !runes.contains(&rune) {
                runes.push(rune);
            }
        }

        let dragons = Dragons {
            ally_fire_dragons: rng.index(5) as u16,
            ally_earth_dragons: rng.index(5) as u16,
            enemy_earth_dragons: rng.index(5) as u16,
            ..Dragons::default()
        };

        if rng.explicit(stats_mode) {
            let mut stats = PlayerStats::infer(InferStats {
                items: &data.items,
                runes: &runes,
                item_exceptions: &[],
                rune_exceptions: &[],
                modifiers: &mut Modifiers::default(),
                dragons,
                ability_levels: abilities,
                stacks: data.stacks,
                level: data.level,
                champion_id: data.champion_id,
                is_mega_gnar: data.is_mega_gnar,
            });

            stats.current_health = stats.max_health * (0.25 + rng.index(76) as f32 / 100.0);
            data.stats = Some(stats);
        }

        let mut enemy_players = Vec::with_capacity(enemies);

        for _ in 0..enemies {
            let mut enemy = random_player::<EnemyStats>(&mut rng, &items);

            if rng.explicit(stats_mode) {
                let state = EnemyState {
                    base_stats: enemy
                        .champion_id
                        .base_stats(enemy.level, enemy.is_mega_gnar)
                        .simple(),
                    items: &enemy.items,
                    champion_id: enemy.champion_id,
                    stacks: enemy.stacks,
                    level: enemy.level,
                    earth_dragons: dragons.enemy_earth_dragons,
                    ..Default::default()
                }
                .full_state(ResistShred::new(&PlayerStats::default()), false);

                let mut stats = state.current_stats;

                stats.current_health = stats.max_health * (0.25 + rng.index(76) as f32 / 100.0);
                stats.missing_health =
                    RiotFormulas::missing_health(stats.current_health, stats.max_health);

                enemy.stats = Some(stats);
            }

            enemy_players.push(enemy);
        }

        result.push(OwnedInputGame {
            active_player: OwnedInputActivePlayer {
                data,
                abilities,
                runes: runes.into_boxed_slice(),
                rune_exceptions: Box::new([]),
            },
            enemy_players: enemy_players.into_boxed_slice(),
            dragons,
        });
    }

    Ok(Workload {
        schema_version: 1,
        generator: "splitmix64-v1".into(),
        seed,
        enemies_per_game: enemies,
        stats_mode,
        games: result,
    })
}

pub struct PreparedGame<'a> {
    active_player: InputActivePlayer<'a>,
    enemy_players: Vec<InputMinData<'a, EnemyStats>>,
    dragons: Dragons,
}

impl PreparedGame<'_> {
    pub const fn input(&self) -> InputGame<'_> {
        InputGame {
            active_player: self.active_player,
            enemy_players: self.enemy_players.as_slice(),
            dragons: self.dragons,
        }
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct Settings {
    pub iterations: usize,
    pub samples: usize,
    pub warmup: usize,
}

#[derive(Serialize)]
pub struct Summary {
    pub mean_ns_per_call: f64,
    pub median_ns_per_call: f64,
    pub p95_batch_mean_ns_per_call: f64,
    pub min_ns_per_call: f64,
    pub max_ns_per_call: f64,
    pub stddev_batch_mean_ns_per_call: f64,
    pub calls_per_second: f64,
}

pub fn summarize(samples: &[f64]) -> Result<Summary> {
    if samples.is_empty() || samples.iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err("samples must contain positive finite timings".into());
    }

    let mut sorted = samples.to_vec();

    sorted.sort_unstable_by(f64::total_cmp);

    let n = sorted.len();
    let mean = sorted.iter().sum::<f64>() / n as f64;
    let median = if n.is_multiple_of(2) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };

    Ok(Summary {
        mean_ns_per_call: mean,
        median_ns_per_call: median,
        p95_batch_mean_ns_per_call: sorted[(n * 95).div_ceil(100) - 1],
        min_ns_per_call: sorted[0],
        max_ns_per_call: sorted[n - 1],
        stddev_batch_mean_ns_per_call: (sorted.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
            / n as f64)
            .sqrt(),
        calls_per_second: 1_000_000_000.0 / mean,
    })
}

#[derive(Serialize)]
pub struct Measurement {
    pub settings: Settings,
    pub measured_calls: usize,
    pub samples_ns_per_call: Vec<f64>,
    pub timer_overhead_mean_ns: f64,
    pub summary: Summary,
}

pub fn run(games: &[PreparedGame<'_>], settings: Settings) -> Result<Measurement> {
    if games.is_empty() || settings.iterations == 0 || settings.samples == 0 {
        return Err("games, iterations, and samples must be nonzero".into());
    }

    let measured_calls = settings
        .iterations
        .checked_mul(settings.samples)
        .ok_or("call count overflow")?;

    let mut cursor = 0;

    for _ in 0..settings.warmup {
        drop(black_box(calculator(black_box(games[cursor].input()))));
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
            let input = black_box(games[cursor].input());
            let start = Instant::now();
            let result = black_box(calculator(input));

            elapsed += start.elapsed();

            // Destruction and corpus selection are deliberately outside timing.
            drop(result);
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

/// Stable, non-cryptographic fingerprints for identifying workloads and binaries.
pub const fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut i = 0;

    while i < bytes.len() {
        hash = (hash ^ bytes[i] as u64).wrapping_mul(0x100000001b3);
        i += 1;
    }

    hash
}
