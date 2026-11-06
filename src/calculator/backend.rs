//! Runs a calculation: the web app's `utils/transport.rs`.
//!
//! With `local-calculation` (the default) the engine runs in process. Otherwise, with `api`,
//! the input goes to the HTTP API. Either way it runs on a background thread.
use crate::calculator::state::{Enemy, Player};
use std::num::NonZeroU8;
use tutorlolv2::{
    ChampionId, GameMap, ItemId, RuneId,
    model::{
        AbilityLevels, Dragons, EnemyStats, InputActivePlayer, InputGame, InputMinData, OutputGame,
        PlayerStats, ValueException,
    },
};

#[cfg(not(any(feature = "local-calculation", feature = "api")))]
compile_error!("enable `local-calculation` or `api` to calculate damages");

pub type Error = Box<dyn core::error::Error + Send + Sync>;

/// Owned copy of the inputs, which [`InputGame`] borrows.
pub struct Snapshot {
    runes: Vec<RuneId>,
    rune_exceptions: Vec<ValueException>,
    abilities: AbilityLevels,
    current_health: f32,
    player: MinData<PlayerStats>,
    enemies: Vec<MinData<EnemyStats>>,
    dragons: Dragons,
    game_map: GameMap,
}

struct MinData<T> {
    stats: Option<T>,
    items: Vec<ItemId>,
    item_exceptions: Vec<ValueException>,
    stacks: u32,
    level: NonZeroU8,
    champion_id: ChampionId,
    is_mega_gnar: bool,
}

impl<T> MinData<T> {
    fn input(&self) -> InputMinData<'_, T>
    where
        T: Copy,
    {
        InputMinData {
            stats: self.stats,
            items: &self.items,
            item_exceptions: &self.item_exceptions,
            stacks: self.stacks,
            level: self.level,
            champion_id: self.champion_id,
            is_mega_gnar: self.is_mega_gnar,
        }
    }
}

impl<T: Copy> From<&crate::calculator::state::PlayerData<T>> for MinData<T> {
    fn from(data: &crate::calculator::state::PlayerData<T>) -> Self {
        Self {
            // Inferred stats are computed by the engine and sent back.
            stats: (!data.infer_stats).then_some(data.stats),
            items: data.items.values().to_vec(),
            item_exceptions: data.item_exceptions.values().to_vec(),
            stacks: data.stacks,
            level: data.level,
            champion_id: data.champion_id,
            is_mega_gnar: data.is_mega_gnar,
        }
    }
}

impl Snapshot {
    pub fn new(player: &Player, enemies: &[Enemy], dragons: Dragons, game_map: GameMap) -> Self {
        Self {
            runes: player.runes.values().to_vec(),
            rune_exceptions: player.rune_exceptions.values().to_vec(),
            abilities: player.abilities,
            current_health: player.current_health,
            player: (&player.data).into(),
            enemies: enemies.iter().map(MinData::from).collect(),
            dragons,
            game_map,
        }
    }

    fn with_input<R>(&self, f: impl FnOnce(InputGame<'_>) -> R) -> R {
        let enemies = self.enemies.iter().map(MinData::input).collect::<Vec<_>>();

        f(InputGame {
            active_player: InputActivePlayer {
                runes: &self.runes,
                rune_exceptions: &self.rune_exceptions,
                abilities: self.abilities,
                data: self.player.input(),
                current_health: self.current_health,
            },
            enemy_players: &enemies,
            dragons: self.dragons,
            game_map: self.game_map,
        })
    }

    /// Blocks until the damages are calculated.
    pub fn calculate(self) -> Result<OutputGame, Error> {
        #[cfg(feature = "local-calculation")]
        {
            Ok(self.with_input(tutorlolv2::calculator::calculator))
        }

        #[cfg(all(not(feature = "local-calculation"), feature = "api"))]
        {
            self.with_input(|input| {
                Ok(crate::api::send::<tutorlolv2_api::Calculator>(&input)?.decode()?)
            })
        }
    }
}
