use crate::*;
use bincode::{Decode, Encode};
use tutorlolv2_gen::{
    AdaptativeType, Attrs, ChampionId, ConstClosure, DamageType, GameMap, ItemId, NUMBER_OF_ITEMS,
    NUMBER_OF_RUNES, Position, RuneId, TypeMetadata,
};
use tutorlolv2_types::AbilityId;

/// Enum that defines the team of some player.
/// - `CHAOS` is converted to [`Team::Red`],
/// - `ORDER` and any other variant matches [`Team::Blue`]
#[derive(Clone, Copy, Debug, Encode, PartialEq)]
pub enum Team {
    Blue,
    Red,
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        match value {
            "ORDER" => Team::Blue,
            _ => Team::Red,
        }
    }
}

#[derive(Debug, Encode)]
pub struct RangeDamage {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

impl RangeDamage {
    pub const fn inc_attr(&mut self, attr: Attrs, damage: i32) {
        match attr {
            Attrs::OnhitMin => self.minimum_damage += damage,
            Attrs::OnhitMax => self.maximum_damage += damage,
            Attrs::Onhit => {
                self.minimum_damage += damage;
                self.maximum_damage += damage;
            }
            _ => {}
        };
    }
}

/// Struct holding the core champion stats of a player, where `T` is a
/// numeric type
#[derive(Clone, Copy, Debug, Encode)]
pub struct BasicStats<T> {
    pub armor: T,
    pub health: T,
    pub attack_damage: T,
    pub magic_resist: T,
    pub mana: T,
}

/// Holds the damage of the basic attack, critical strike damage, and onhits
#[derive(Debug, Encode)]
pub struct Attacks {
    /// Damage of the basic attack hit
    pub basic_attack: i32,
    /// Damage of the critical strike. For most champions, it represents a
    /// multipler of 1.75x the damage of the basic attack.
    pub critical_strike: i32,
    /// The onhit damage variant, containing the necessary information to
    /// display it as a range `{min} - {max}`
    pub onhit_damage: RangeDamage,
}

/// Holds the compile-time known metadata and closures of the current champion,
/// obtained from the static variable [`tutorlolv2_gen::CHAMPION_CACHE`]. Note that
/// the current champion is only known at runtime, but since the application knowns
/// all the possible champion's data at compile-time, the length of both metadata
/// and closures is known and lives forever, unlike the ones in variant [`DamageKind`]
#[derive(Debug)]
pub struct StaticDamageKind<T: 'static> {
    pub metadata: &'static [TypeMetadata<T>],
    pub closures: &'static [ConstClosure],
}

/// Runtime-known values that depend on how many damaging items or runes the
/// current player has. Holds the metadata and closures of the given `T` parameter.
/// See [`TypeMetadata`] and [`ConstClosure`] for more details
#[derive(Debug)]
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

/// Contains all the results from function [`realtime::realtime`] that could be extracted
/// from the input game data. It includes all the information about items, runes,
/// and abilities damage against each champion in the enemy team, inferring their
/// stats and applying the appropriate damage modifiers. This struct is not
/// organized in a very convenient way in order to reduce memory usage. Note that
/// the size of this struct is often larger than `70 KB`, and it is created every
/// second, while part of its fields live in the caller function, which is why it
/// has lifetime annotations.
#[derive(Debug, Encode)]
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
    /// [`tutorlolv2_gen::CHAMPION_CACHE`] at the index [`ChampionId`] when casting
    /// this enum to type [`usize`]
    pub abilities_meta: &'static [TypeMetadata<AbilityId>],

    /// Vector of the [`TypeMetadata<ItemId>`] of all damaging items the player had
    pub items_meta: Box<[TypeMetadata<ItemId>]>,

    /// Vector of the [`TypeMetadata<RuneId>`] of all damaging runes the player had
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,

    /// Constant array containing the [`TypeMetadata<ItemId>`] of all items that
    /// were chosen to have their damages compared among each other, to determine
    /// which one is mathematically besst to buy next. Note that this value lives
    /// at the constant [`SIMULATED_ITEMS_METADATA`]
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

#[derive(Debug, Encode)]
pub struct Scoreboard<'a> {
    pub riot_id: &'a str,
    pub assists: u8,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
    pub team: Team,
}

#[derive(Debug, Encode)]
pub struct CurrentPlayer<'a> {
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

/// Struct that holds the values that can reduce the enemie's armor or
/// magic resistence benefits
#[derive(Clone, Copy, Debug)]
pub struct ResistShred {
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
}

impl ResistShred {
    /// Creates a [`ResistShred`] struct from the current player stats. This API
    /// uses percent penetration as a value in range `[0.0, 100.0]`. Values over
    /// `100.0` will be clamped
    pub const fn new(stats: &Stats<f32>) -> Self {
        let Stats {
            armor_penetration_flat,
            armor_penetration_percent,
            magic_penetration_flat,
            magic_penetration_percent,
            ..
        } = *stats;
        Self {
            armor_penetration_flat: armor_penetration_flat,
            armor_penetration_percent: armor_penetration_percent.clamp(0.0, 100.0),
            magic_penetration_flat: magic_penetration_flat,
            magic_penetration_percent: magic_penetration_percent.clamp(0.0, 100.0),
        }
    }
}

#[derive(Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Debug)]
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

/// Holds the most simple stats that need to be used to calculate
/// the damage against this enemy. Note that it is similar to struct
/// [`BasicStats`], but without the `attack_damage` and `mana` fields,
/// which are fields that do not quantify any damage reduction the enemy
/// champion may take. Generic parameter `T` is supposed to be a numeric type
#[derive(Clone, Copy, Debug, Decode, Encode)]
pub struct SimpleStats<T> {
    pub armor: T,
    pub health: T,
    pub magic_resist: T,
}

#[derive(Debug)]
pub struct DamageEvalData {
    pub abilities: StaticDamageKind<AbilityId>,
    pub items: DamageKind<ItemId>,
    pub runes: DamageKind<RuneId>,
}

#[derive(Debug, Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub damages: Damages,
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

#[derive(Debug, Encode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: Box<[i32]>,
    pub items: Box<[i32]>,
    pub runes: Box<[i32]>,
}

/// Wrapper around the type [`u32`], whose first [`Self::DISC_BITS`] are used to
/// identify the enum type of the current value, which is either [`ItemId`] or [`RuneId`],
/// and the remaining [`Self::VAL_BITS`] are used to store the actual number of stacks held
#[derive(Clone, Copy, Debug, Decode)]
#[repr(transparent)]
pub struct ValueException(u32);

impl ValueException {
    pub const DISC_BITS: u32 = Self::find_disc_bits(NUMBER_OF_ITEMS as u32, NUMBER_OF_RUNES as u32);
    pub const VAL_BITS: u32 = 32 - Self::DISC_BITS;
    pub const VAL_MASK: u32 = (1u32 << Self::VAL_BITS) - 1;
    pub const DISC_MASK: u32 = !Self::VAL_MASK;
    pub const DISC_LOW_MASK: u32 = (1u32 << Self::DISC_BITS) - 1;

    /// Returns a u32 with the number of leading zeros of the maximum between `a` and `b`
    const fn find_disc_bits(a: u32, b: u32) -> u32 {
        u32::BITS - if a > b { a } else { b }.leading_zeros()
    }

    /// Returns how many stacks are stored. Note that it returns an [`u32`] but whose
    /// maximum value is [`Self::VAL_MASK`]
    pub const fn stacks(&self) -> u32 {
        self.0 & Self::VAL_MASK
    }

    /// Returns an [`u16`], which is large enough to represent both [`ItemId`] and [`RuneId`]
    /// enums. This value is taken from the first [`Self::DISC_BITS`] bits
    const fn enum_id(&self) -> u16 {
        ((self.0 >> Self::VAL_BITS) & ((1u32 << Self::DISC_BITS) - 1)) as u16
    }

    /// Returns if the current value is a [`RuneId`]
    pub const fn get_rune_id(&self) -> Option<RuneId> {
        RuneId::from_u8(self.enum_id() as u8)
    }

    /// Returns if the current value is an [`ItemId`]
    pub const fn get_item_id(&self) -> Option<ItemId> {
        ItemId::from_u16(self.enum_id())
    }

    /// If the value to be stored is greater than [`Self::VAL_MASK`],
    /// the value is truncated
    const fn truncate_value(v: u32) -> u32 {
        v & Self::VAL_MASK
    }

    /// Creates a new instance of [`Self`] from a [`RuneId`] and a number of stacks
    pub const fn pack_rune_id(r: RuneId, v: u32) -> Self {
        let disc = (r as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }

    /// Creates a new instance of [`Self`] from an [`ItemId`] and a number of stacks
    pub const fn pack_item_id(i: ItemId, v: u32) -> Self {
        let disc = (i as u32) & Self::DISC_LOW_MASK;
        Self((disc << Self::VAL_BITS) | Self::truncate_value(v))
    }
}

/// Holds the number of dragons and their types, associated to the ally or enemy team.
#[derive(Clone, Copy, Debug, Decode, Encode)]
pub struct Dragons {
    pub ally_fire_dragons: u16,
    pub ally_earth_dragons: u16,
    pub ally_chemtech_dragons: u16,
    pub enemy_earth_dragons: u16,
}

#[derive(Debug, Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: Box<[InputMinData<SimpleStats<i32>>]>,
    pub dragons: Dragons,
}

#[derive(Debug, Decode)]
pub struct InputActivePlayer {
    pub runes: Box<[RuneId]>,
    pub rune_exceptions: Box<[ValueException]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData<Stats<i32>>,
}

/// Minimum required data to qualify a valid enemy player, and calculate
/// damages against this target. Field `stats` is required, but if `infer_stats`
/// is set to true, the enemy's stats will be inferred and this field will be ignored.
/// The same happens with `is_mega_gnar`, which can be set to true, but will only
/// have effect if field `champion_id` is also of type [`ChampionId::Gnar`].
/// Field `stacks` is useless if the associated champion does not have any special
/// characteristics that are related to stack-scaling
#[derive(Debug, Decode)]
pub struct InputMinData<T> {
    pub stats: T,
    pub items: Box<[ItemId]>,
    pub item_exceptions: Box<[ValueException]>,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

/// Returned data by the function [`calculator::calculator`]
#[derive(Debug, Encode)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStats<i32>,
    pub bonus_stats: SimpleStats<i32>,
    pub current_stats: SimpleStats<i32>,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

/// Holds values that will be multiplied by all damages, depending on their
/// damage types, defined by the metadata [`tutorlolv2_gen::DamageType`]. Note
/// that the value `1.0` means no modifiers, and `global_mod` is applied regardless
/// of the damage type provided
#[derive(Clone, Copy, Debug)]
pub struct DamageModifiers {
    pub physical_mod: f32,
    pub magic_mod: f32,
    pub true_mod: f32,
    pub global_mod: f32,
}

impl DamageModifiers {
    pub const fn modifier(&self, damage_type: DamageType) -> f32 {
        self.global_mod
            * match damage_type {
                DamageType::Physical => self.physical_mod,
                DamageType::Magic => self.magic_mod,
                DamageType::True => self.true_mod,
                _ => 1.0,
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Modifiers {
    pub damages: DamageModifiers,
    pub abilities: AbilityModifiers,
}

/// Holds float values that will be multiplied by the damages of each ability
/// depending on their letters, which can be obtained through the method
/// [`AbilityId::as_char`] with simple branching. Values of `1.0` mean no modifiers
#[derive(Clone, Copy, Debug)]
pub struct AbilityModifiers {
    pub q: f32,
    pub w: f32,
    pub e: f32,
    pub r: f32,
}

#[derive(Debug, Encode)]
pub struct OutputCurrentPlayer {
    pub current_stats: Stats<i32>,
    pub base_stats: BasicStats<i32>,
    pub bonus_stats: BasicStats<i32>,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Debug, Encode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: Box<[i32]>,
    pub items: Box<[i32]>,
}

#[derive(Debug, Encode)]
pub struct OutputGame {
    pub monster_damages: [MonsterDamage; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[OutputEnemy]>,
    pub tower_damages: [i32; L_TWRD],
    pub abilities_meta: &'static [TypeMetadata<AbilityId>],
    pub abilities_to_merge: &'static [(usize, usize)],
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}

/// Implements traits From<T<f32>> and From<T<i32>> for some struct
/// in which all of its fields are numeric types with simple and
/// deterministic castings from between [`f32`] and [`i32`]
macro_rules! impl_cast_from {
    ($stru:ident, $($fields:ident),*) => {
        impl $stru<f32> {
            pub const fn from_i32(value: &$stru<i32>) -> Self {
                $stru {
                    $($fields: value.$fields as f32),*
                }
            }
        }

        impl $stru<i32> {
            pub const fn from_f32(value: &$stru<f32>) -> Self {
                $stru {
                    $($fields: value.$fields as i32),*
                }
            }
        }

        impl From<$stru<f32>> for $stru<i32> {
            fn from(value: $stru<f32>) -> Self {
                $stru::from_f32(&value)
            }
        }

        impl From<$stru<i32>> for $stru<f32> {
            fn from(value: $stru<i32>) -> Self {
                $stru::from_i32(&value)
            }
        }

        impl From<&$stru<f32>> for $stru<i32> {
            fn from(value: &$stru<f32>) -> Self {
                $stru::from_f32(value)
            }
        }

        impl From<&$stru<i32>> for $stru<f32> {
            fn from(value: &$stru<i32>) -> Self {
                $stru::from_i32(value)
            }
        }
    };
}

impl_cast_from!(SimpleStats, health, armor, magic_resist);
impl_cast_from!(BasicStats, armor, health, attack_damage, magic_resist, mana);
impl_cast_from!(
    Stats,
    ability_power,
    armor,
    armor_penetration_flat,
    armor_penetration_percent,
    attack_damage,
    attack_range,
    attack_speed,
    crit_chance,
    crit_damage,
    current_health,
    magic_penetration_flat,
    magic_penetration_percent,
    magic_resist,
    health,
    mana,
    current_mana
);

/// Implements trait [`Default`] and a constant method [`Self::default`] for some struct.
/// The first argument of this macro is the struct in which these functions will be generated,
/// the second one is the default value. Often `0` or `1`, and the third one is the type
/// of the default value, that could not be inferred. This macro does not need to know the
/// fields of the target struct since it uses [`core::mem::transmute`]
macro_rules! impl_default {
    ($ty:ty, $init:literal, $typedef:ty) => {
        impl $ty {
            pub const fn default() -> Self {
                unsafe { core::mem::transmute([$init; size_of::<$ty>() / size_of::<$typedef>()]) }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                <$ty>::default()
            }
        }
    };
}

impl_default!(Stats<f32>, 0.0f32, f32);
impl_default!(SimpleStats<f32>, 0.0f32, f32);
impl_default!(BasicStats<f32>, 0.0f32, f32);
impl_default!(DamageModifiers, 1.0f32, f32);
impl_default!(AbilityModifiers, 1.0f32, f32);
impl_default!(Modifiers, 1.0f32, f32);
impl_default!(Dragons, 0u8, u8);
impl_default!(RangeDamage, 0i32, i32);
