use crate::*;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Decode, Deserialize)]
pub struct Realtime {
    /// Contains the information about the current player
    pub current_player: CurrentPlayer,

    /// Holds the recovered information about each enemy in the opposing team. including
    /// how much damage they will take for each item, rune, or ability of the current player
    pub enemies: Box<[Enemy]>,

    /// The game's scoreboard. Contains meaningful information about each champion
    /// in both teams, including the current player. This data is not guaranteed
    /// to be ordered based on team. Sorting this vector has to be done by the caller
    pub scoreboard: Box<[Scoreboard]>,

    /// Constant array of the [`TypeMetadata<AbilityId>`] of all abilities of the
    /// current champion. Note that this value lives at the static variable
    /// [`CHAMPION_CACHE`] at the index [`ChampionId`] when casting
    /// this enum to type [`usize`]
    pub abilities_meta: Box<[TypeMetadata<AbilityId>]>,

    /// Vector of the [`TypeMetadata<ItemId>`] of all damaging items the player had
    pub items_meta: Box<[TypeMetadata<ItemId>]>,

    /// Vector of the [`TypeMetadata<RuneId>`] of all damaging runes the player had
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,

    /// Constant array containing the [`TypeMetadata<ItemId>`] of all items that
    /// were chosen to have their damages compared among each other, to determine
    /// which one is mathematically besst to buy next. Note that this value lives
    /// at the constant [`crate::realtime::SIMULATED_ITEMS_METADATA`]
    #[serde(with = "serde_arrays")]
    pub siml_meta: [TypeMetadata<ItemId>; L_SIML],

    /// Constant array of tuples that determines which abilities should
    /// be displayed in a single cell, in the format `{min} - {max}`. Doing it
    /// this way allow that when summing up the damage of each ability, the user
    /// has more flexibility in which it can choose to insert only the minimum damage
    /// of some ability, the maximum damage, or both, while maintaining the table
    /// display in a very deterministic format
    pub abilities_to_merge: Box<[(usize, usize)]>,

    /// Game time in seconds
    pub game_time: u32,

    /// Level of each ability of the current player
    pub ability_levels: AbilityLevels,

    /// Contains which dragons were associated, and to which team.
    /// Only `Earth`, `Fire` and `Chemtech` dragons are considered,
    /// since the others doesn't give any damage-related bonus
    pub dragons: Dragons,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct Scoreboard {
    pub riot_id: Box<str>,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct CurrentPlayer {
    pub riot_id: Box<str>,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub current_stats: Stats<i32>,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub game_map: GameMap,
    // pub _padding: u16,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct Enemy {
    pub riot_id: Box<str>,
    pub damages: Damages,
    #[serde(with = "serde_arrays")]
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub team: Team,
    pub position: Position,
}
