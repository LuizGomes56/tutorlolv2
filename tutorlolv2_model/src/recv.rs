use crate::*;
use bincode::{BorrowDecode, Decode, Encode};
use serde::{Deserialize, Serialize};

/// Holds the compile-time known metadata and closures of the current champion,
/// obtained from the static variable [`CHAMPION_CACHE`]. Note that
/// the current champion is only known at runtime, but since the application knowns
/// all the possible champion's data at compile-time, the length of both metadata
/// and closures is known and lives forever, unlike the ones in variant [`DamageKind`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct StaticDamageKind<T: 'static> {
    pub metadata: &'static [TypeMetadata<T>],
    pub closures: &'static [ConstClosure],
}

/// Runtime-known values that depend on how many damaging items or runes the
/// current player has. Holds the metadata and closures of the given `T` parameter.
/// See [`TypeMetadata`] and [`ConstClosure`] for more details
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DamageKind<T> {
    /// Vector of the [`TypeMetadata`] of either [`ItemId`] or [`RuneId`]
    /// Order is not guaranteed and none of these values are knonwn at compile time
    /// since they depend on how many damaging items the current player has
    pub metadata: Box<[TypeMetadata<T>]>,
    /// Vector of closures of some [`ItemId`] or [`RuneId`], defined by the generic
    /// parameter `T`. See [`ConstClosure`] to learn more of how to evaluate these
    /// functions, and what they return
    pub closures: Box<[ConstClosure]>,
}

/// Contains all the results from function [`crate::realtime::realtime`] that could be extracted
/// from the input game data. It includes all the information about items, runes,
/// and abilities damage against each champion in the enemy team, inferring their
/// stats and applying the appropriate damage modifiers. This struct is not
/// organized in a very convenient way in order to reduce memory usage. Note that
/// the size of this struct is often larger than `70 KB`, and it is created every
/// second, while part of its fields live in the caller function, which is why it
/// has lifetime annotations.
#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct Realtime<'a> {
    /// Contains the information about the current player
    pub current_player: CurrentPlayer<'a>,

    /// Holds the recovered information about each enemy in the opposing team. including
    /// how much damage they will take for each item, rune, or ability of the current player
    pub enemies: Box<[Enemy<'a>]>,

    /// The game's scoreboard. Contains meaningful information about each champion
    /// in both teams, including the current player. This data is not guaranteed
    /// to be ordered based on team. Sorting this vector has to be done by the caller
    pub scoreboard: Box<[Scoreboard<'a>]>,

    /// Constant array of the [`TypeMetadata<AbilityId>`] of all abilities of the
    /// current champion. Note that this value lives at the static variable
    /// [`CHAMPION_CACHE`] at the index [`ChampionId`] when casting
    /// this enum to type [`usize`]
    pub abilities_meta: &'static [TypeMetadata<AbilityId>],

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
    pub abilities_to_merge: &'static [(usize, usize)],

    /// Game time in seconds
    pub game_time: u32,

    /// Level of each ability of the current player
    pub ability_levels: AbilityLevels,

    /// Contains which dragons were associated, and to which team.
    /// Only `Earth`, `Fire` and `Chemtech` dragons are considered,
    /// since the others doesn't give any damage-related bonus
    pub dragons: Dragons,
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, BorrowDecode, Serialize, Deserialize,
)]
pub struct Scoreboard<'a> {
    #[serde(borrow)]
    pub riot_id: &'a str,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, BorrowDecode, Serialize, Deserialize)]
pub struct CurrentPlayer<'a> {
    #[serde(borrow)]
    pub riot_id: &'a str,
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct EnemyState<'a> {
    pub base_stats: SimpleStats<f32>,
    pub items: &'a [ItemId],
    pub stacks: u32,
    pub champion_id: ChampionId,
    pub earth_dragons: u16,
    pub level: u8,
    pub item_exceptions: &'a [ValueException],
    // _padding: u32
}

/// Defines the state of the current player, that will be used
/// to calculate the final necessary evaluation data of the enemy
/// player. This struct should be the same used for all champions
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct SelfState {
    pub ability_levels: AbilityLevels,
    pub current_stats: Stats<f32>,
    pub bonus_stats: BasicStats<f32>,
    pub base_stats: BasicStats<f32>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    // _padding: u16
}

/// Holds the intermediary data of some enemy champion, including fields
/// necessary to have a more precise calculation of the damage against this
/// target. Each enemy champion should have their own instance of this struct
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct EnemyFullState {
    pub current_stats: SimpleStats<f32>,
    pub bonus_stats: SimpleStats<f32>,
    pub modifiers: DamageModifiers,
    pub armor_values: ResistValue,
    pub magic_values: ResistValue,
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
    // _padding: u8
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DamageEvalData {
    pub abilities: StaticDamageKind<AbilityId>,
    pub items: DamageKind<ItemId>,
    pub runes: DamageKind<RuneId>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, BorrowDecode, Serialize, Deserialize)]
pub struct Enemy<'a> {
    #[serde(borrow)]
    pub riot_id: &'a str,
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
