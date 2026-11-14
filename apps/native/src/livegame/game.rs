//! A calculated live game: the web app's `livegame/mod.rs` and `utils/transport::realtime*`.
//!
//! The engine's `Realtime` borrows names from its input. [`Game`] owns them, so it can be built
//! on a background thread and kept after the input is gone.
use crate::components::stack::StackGame;
use gpui::SharedString;
use math::{
    AdaptiveType, ChampionId, ItemId, Position, RuneId, TypeMetadata,
    model::{
        AbilityLevels, BasicStats, DamageTable, Damages, Dragons, EnemyStats, PlayerStats, Team,
    },
    riot::RiotRealtime,
};
use std::num::NonZeroU8;

pub type Error = Box<dyn core::error::Error + Send + Sync>;

pub struct CurrentPlayer {
    pub riot_id: SharedString,
    pub base_stats: BasicStats,
    pub current_stats: PlayerStats,
    pub level: NonZeroU8,
    pub team: Team,
    pub adaptive_type: AdaptiveType,
    pub champion_id: ChampionId,
}

pub struct Enemy {
    pub current_stats: EnemyStats,
    pub champion_id: ChampionId,
}

pub struct Score {
    pub riot_id: SharedString,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

pub struct Game {
    pub current_player: CurrentPlayer,
    pub enemies: Vec<Enemy>,
    pub damages: DamageTable,
    pub scoreboard: Vec<Score>,
    pub items_meta: Vec<TypeMetadata<ItemId>>,
    pub runes_meta: Vec<TypeMetadata<RuneId>>,
    pub game_time: u32,
    pub ability_levels: AbilityLevels,
    pub dragons: Dragons,
}

impl Game {
    /// `Realtime::ROWS_PER_ENEMY`: each enemy's row, then one row per simulated purchase.
    pub const ROWS_PER_ENEMY: usize = math::model::Realtime::ROWS_PER_ENEMY;

    pub fn enemy_damages(&self, e: usize) -> Damages<'_> {
        self.damages.row(e * Self::ROWS_PER_ENEMY)
    }

    pub fn siml_damages(&self, e: usize, i: usize) -> Damages<'_> {
        self.damages.row(e * Self::ROWS_PER_ENEMY + 1 + i)
    }

    pub fn total_damage(&self, e: usize) -> i32 {
        self.enemy_damages(e).sum()
    }

    /// Total damage against enemy `e` after buying each simulated item, best first.
    pub fn item_scores(&self, e: usize) -> [(i32, ItemId); ItemId::L_SIML] {
        let mut list = core::array::from_fn(|i| (self.siml_damages(e, i).sum(), ItemId::SIML[i]));
        list.sort_unstable_by(|left, right| right.cmp(left));
        list
    }

    /// Calculates the Riot JSON of port 2999, as the recorder keeps it.
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        let input: RiotRealtime<'_> = serde_json::from_slice(bytes)?;
        Self::from_frame(&input)
    }

    /// Calculates an already decoded frame (replays).
    pub fn from_frame(input: &RiotRealtime<'_>) -> Result<Self, Error> {
        #[cfg(feature = "local-calculation")]
        {
            let output = math::realtime::realtime(input)
                .map_err(|error| format!("Unable to calculate frame: {error}"))?;

            Ok(output.into())
        }

        #[cfg(all(not(feature = "local-calculation"), feature = "api"))]
        {
            let response = crate::api::send::<::api::Realtime>(input)?;
            Ok(response.decode()?.into())
        }
    }
}

impl From<math::model::Realtime<'_>> for Game {
    fn from(data: math::model::Realtime<'_>) -> Self {
        let player = data.current_player;

        Self {
            current_player: CurrentPlayer {
                riot_id: SharedString::from(player.riot_id.to_owned()),
                base_stats: player.base_stats,
                current_stats: player.current_stats,
                level: player.level,
                team: player.team,
                adaptive_type: player.adaptive_type,
                champion_id: player.champion_id,
            },
            enemies: data
                .enemies
                .iter()
                .map(|enemy| Enemy {
                    current_stats: enemy.current_stats,
                    champion_id: enemy.champion_id,
                })
                .collect(),
            damages: data.damages,
            scoreboard: data
                .scoreboard
                .iter()
                .map(|score| Score {
                    riot_id: SharedString::from(score.riot_id.to_owned()),
                    assists: score.assists,
                    creep_score: score.creep_score,
                    deaths: score.deaths,
                    kills: score.kills,
                    champion_id: score.champion_id,
                    position: score.position,
                    team: score.team,
                })
                .collect(),
            items_meta: data.items_meta.into_vec(),
            runes_meta: data.runes_meta.into_vec(),
            game_time: data.game_time,
            ability_levels: data.ability_levels,
            dragons: data.dragons,
        }
    }
}

/// The stack sums each enemy's own row, skipping the simulated purchase rows.
impl StackGame for Game {
    fn champion_id(&self) -> ChampionId {
        self.current_player.champion_id
    }

    fn level(&self) -> NonZeroU8 {
        self.current_player.level
    }

    fn items_meta(&self) -> &[TypeMetadata<ItemId>] {
        &self.items_meta
    }

    fn runes_meta(&self) -> &[TypeMetadata<RuneId>] {
        &self.runes_meta
    }

    fn targets(&self) -> Vec<(ChampionId, f32, Damages<'_>)> {
        self.enemies
            .iter()
            .enumerate()
            .map(|(e, enemy)| {
                (
                    enemy.champion_id,
                    enemy.current_stats.max_health,
                    self.enemy_damages(e),
                )
            })
            .collect()
    }
}
