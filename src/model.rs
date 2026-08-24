//! Declaration of structs, constants, enums and methods that are
//! used in all other modules of this library.
//!
//! ### Features
//! - `bincode` - implements `Encode` `Decode` or `BorrowDecode` traits
//!   to all eligible structs and enums
//! - `serde` - implements `Serialize` `Deserialize` traits to all eligible
//!   structs and enums. Structs that have `'static` lifetimes do not implement
//!   `Deserialize`

use {
    crate::{
        ChampionId, ItemId, RuneId,
        libgen::{Closure, L_MSTR, L_TWRD},
    },
    alloc::boxed::Box,
    bincode::{BorrowDecode, Decode, Encode},
    core::str::FromStr,
    serde::{Deserialize, Serialize},
    strum::EnumCount,
    tutorlolv2_types::{
        AbilityId, AbilityName, AdaptiveType, Attrs, Ctx, DamageType, GameMap, Position,
        TypeMetadata,
    },
};

/// Holds the compile-time known metadata and closures of the current champion,
/// obtained from the static variable [`CHAMPION_CACHE`]. Note that
/// the current champion is only known at runtime, but since the application knowns
/// all the possible champion's data at compile-time, the length of both metadata
/// and closures is known and lives forever, unlike the ones in variant [`DamageKind`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct StaticDamageKind<T: 'static> {
    pub metadata: &'static [TypeMetadata<T>],
    pub closures: &'static [Closure],
}

/// Runtime-known values that depend on how many damaging items or runes the
/// current player has. Holds the metadata and closures of the given `T` parameter.
/// See [`TypeMetadata`] and [`Closure`] for more details
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DamageKind<T> {
    /// Vector of the [`TypeMetadata`] of either [`ItemId`] or [`RuneId`]
    /// Order is not guaranteed and none of these values are knonwn at compile time
    /// since they depend on how many damaging items the current player has
    pub metadata: Box<[TypeMetadata<T>]>,
    /// Vector of closures of some [`ItemId`] or [`RuneId`], defined by the generic
    /// parameter `T`. See [`Closure`] to learn more of how to evaluate these
    /// functions, and what they return
    pub closures: Box<[Closure]>,
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

    /// Vector of the [`TypeMetadata<ItemId>`] of all damaging items the player had
    pub items_meta: Box<[TypeMetadata<ItemId>]>,

    /// Vector of the [`TypeMetadata<RuneId>`] of all damaging runes the player had
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,

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
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: PlayerStats,
    pub level: u8,
    pub team: Team,
    pub adaptive_type: AdaptiveType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub game_map: GameMap,
    // pub _padding: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Encode, Serialize)]
pub struct EnemyState<'a> {
    pub current_stats: Option<EnemyStats>,
    pub base_stats: SimpleStats,
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
    pub stacks: f32,
    pub ability_levels: AbilityLevels,
    pub current_stats: PlayerStats,
    pub bonus_stats: BasicStats,
    pub base_stats: BasicStats,
    pub level: u8,
    pub adaptive_type: AdaptiveType,
    // _padding: u16
}

/// Holds the intermediary data of some enemy champion, including fields
/// necessary to have a more precise calculation of the damage against this
/// target. Each enemy champion should have their own instance of this struct
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct EnemyFullState {
    pub current_stats: EnemyStats,
    pub bonus_stats: SimpleStats,
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
    pub siml_items: [Damages; ItemId::L_SIML],
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: EnemyStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub team: Team,
    pub position: Position,
}

/// Enum that defines the team of some player.
/// - `CHAOS` is converted to [`Team::Red`],
/// - `ORDER` and any other variant matches [`Team::Blue`]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Encode,
    Decode,
    Serialize,
    Deserialize,
)]
pub enum Team {
    #[default]
    Blue,
    Red,
}

impl FromStr for Team {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ORDER" => Ok(Team::Blue),
            "CHAOS" => Ok(Team::Red),
            _ => Err("No matches when calling Team::from_str"),
        }
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct RangeDamage {
    pub minimum_damage: i32,
    pub maximum_damage: i32,
}

impl RangeDamage {
    /// Takes an attribute and adds the appropriate damage value based on it.
    /// [`Attrs::Onhit`] and any other variant like this one indicates that the
    /// current instance deals a constant onhit damage, not being defined as
    /// minimum-only, or maximum-only
    pub const fn inc_attr(&mut self, attr: Attrs, damage: i32) {
        match attr {
            Attrs::OnhitMin | Attrs::AreaOnhitMin => self.minimum_damage += damage,
            Attrs::OnhitMax | Attrs::AreaOnhitMax => self.maximum_damage += damage,
            Attrs::Onhit | Attrs::AreaOnhit => {
                self.minimum_damage += damage;
                self.maximum_damage += damage;
            }
            _ => {}
        };
    }
}

/// Holds the damage of the basic attack, critical strike damage, and onhits
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Decode,
    Deserialize,
    Encode,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
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

/// Struct that holds the values that can reduce the enemie's armor or
/// magic resistence benefits
#[derive(
    Copy, Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
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
    pub const fn new(stats: &PlayerStats) -> Self {
        let PlayerStats {
            armor_penetration_flat,
            armor_penetration_percent,
            magic_penetration_flat,
            magic_penetration_percent,
            ..
        } = *stats;
        Self {
            armor_penetration_flat,
            armor_penetration_percent: armor_penetration_percent.clamp(0.0, 100.0),
            magic_penetration_flat,
            magic_penetration_percent: magic_penetration_percent.clamp(0.0, 100.0),
        }
    }
}

#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: Box<[i32]>,
    pub items: Box<[i32]>,
    pub runes: Box<[i32]>,
    pub ctx: Ctx,
}

impl Damages {
    pub fn sum(&self) -> i32 {
        self.attacks.onhit_damage.minimum_damage
            + self.attacks.onhit_damage.maximum_damage
            + self.abilities.iter().sum::<i32>()
            + self.items.iter().sum::<i32>()
            + self.runes.iter().sum::<i32>()
    }
}

/// Wrapper around the type [`u32`], whose first [`Self::DISC_BITS`] are used to
/// identify the enum type of the current value, which is either [`ItemId`] or [`RuneId`],
/// and the remaining [`Self::VAL_BITS`] are used to store the actual number of stacks held
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Decode,
    Deserialize,
    Encode,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(transparent)]
pub struct ValueException(u32);

impl ValueException {
    pub const DISC_BITS: u32 = Self::find_disc_bits(ItemId::COUNT as u32, RuneId::COUNT as u32);
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
        RuneId::from_repr(self.enum_id() as _)
    }

    /// Returns if the current value is an [`ItemId`]
    pub const fn get_item_id(&self) -> Option<ItemId> {
        ItemId::from_repr(self.enum_id() as _)
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

    pub const fn pack_items<const N: usize>(items: &[(ItemId, u32); N]) -> [Self; N] {
        let mut result: [Self; N] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        while i < N {
            let (item_id, v) = items[i];
            result[i] = Self::pack_item_id(item_id, v);
            i += 1;
        }
        result
    }

    pub const fn pack_runes<const N: usize>(runes: &[(RuneId, u32); N]) -> [Self; N] {
        let mut result: [Self; N] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        while i < N {
            let (rune_id, v) = runes[i];
            result[i] = Self::pack_rune_id(rune_id, v);
            i += 1;
        }
        result
    }
}

/// Holds the number of dragons and their types, associated to the ally or enemy team.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct Dragons {
    pub ally_fire_dragons: u16,
    pub ally_earth_dragons: u16,
    pub ally_chemtech_dragons: u16,
    pub enemy_earth_dragons: u16,
}

#[derive(Copy, Clone, Debug, Default, Encode, PartialEq, PartialOrd, Serialize)]
pub struct InputGame<'a> {
    pub active_player: InputActivePlayer<'a>,
    pub enemy_players: &'a [InputMinData<'a, EnemyStats>],
    pub dragons: Dragons,
}

#[derive(Copy, Clone, Debug, Default, Encode, PartialEq, PartialOrd, Serialize)]
pub struct InputActivePlayer<'a> {
    pub runes: &'a [RuneId],
    pub rune_exceptions: &'a [ValueException],
    pub abilities: AbilityLevels,
    pub data: InputMinData<'a, PlayerStats>,
}

#[derive(Copy, Clone, Debug, Default, Encode, PartialEq, PartialOrd, Serialize)]
pub struct InputMinData<'a, T> {
    pub stats: Option<T>,
    pub items: &'a [ItemId],
    pub item_exceptions: &'a [ValueException],
    pub stacks: u32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub is_mega_gnar: bool,
}

#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct OwnedInputGame {
    pub active_player: OwnedInputActivePlayer,
    pub enemy_players: Box<[OwnedInputMinData<EnemyStats>]>,
    pub dragons: Dragons,
}

#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct OwnedInputActivePlayer {
    pub runes: Box<[RuneId]>,
    pub rune_exceptions: Box<[ValueException]>,
    pub abilities: AbilityLevels,
    pub data: OwnedInputMinData<PlayerStats>,
}

#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct OwnedInputMinData<T> {
    pub stats: Option<T>,
    pub items: Box<[ItemId]>,
    pub item_exceptions: Box<[ValueException]>,
    pub stacks: u32,
    pub level: u8,
    pub is_mega_gnar: bool,
    pub champion_id: ChampionId,
}

impl<'a> From<&'a OwnedInputActivePlayer> for InputActivePlayer<'a> {
    fn from(de: &'a OwnedInputActivePlayer) -> Self {
        InputActivePlayer {
            runes: &de.runes,
            rune_exceptions: &de.rune_exceptions,
            abilities: de.abilities,
            data: InputMinData {
                stats: de.data.stats,
                items: &de.data.items,
                item_exceptions: &de.data.item_exceptions,
                stacks: de.data.stacks,
                level: de.data.level,
                is_mega_gnar: de.data.is_mega_gnar,
                champion_id: de.data.champion_id,
            },
        }
    }
}

impl<'a> From<&'a OwnedInputGame> for InputGame<'a> {
    fn from(de: &'a OwnedInputGame) -> Self {
        InputGame {
            active_player: (&de.active_player).into(),
            enemy_players: unsafe {
                core::mem::transmute::<
                    &'a [OwnedInputMinData<EnemyStats>],
                    &'a [InputMinData<'a, EnemyStats>],
                >(&de.enemy_players)
            },
            dragons: de.dragons,
        }
    }
}

/// Returned data by the function [`crate::calculator::calculator`]
#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: EnemyStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub champion_id: ChampionId,
    pub level: u8,
}

/// Holds values that will be multiplied by all damages, depending on their
/// damage types, defined by the metadata [`crate::DamageType`]. Note
/// that the value `1.0` means no modifiers, and `global_mod` is applied regardless
/// of the damage type provided
#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct DamageModifiers {
    pub adaptive_type: AdaptiveType,
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
                DamageType::Adaptive => match self.adaptive_type {
                    AdaptiveType::Physical => self.physical_mod,
                    AdaptiveType::Magic => self.magic_mod,
                },
                _ => 1.0,
            }
    }
}

#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct Modifiers {
    pub damages: DamageModifiers,
    pub abilities: AbilityModifiers,
}

impl Modifiers {
    pub const fn new(ctx: &Ctx, adaptive_type: AdaptiveType) -> Self {
        Self {
            damages: DamageModifiers {
                adaptive_type,
                physical_mod: ctx.physical_multiplier,
                magic_mod: ctx.magic_multiplier,
                true_mod: 1.0,
                global_mod: 1.0,
            },
            ..Self::default()
        }
    }
}

/// Holds float values that will be multiplied by the damages of each ability
/// depending on their letters, which can be obtained through the method
/// [`AbilityId::as_char`] with simple branching. Values of `1.0` mean no modifiers.
/// Also, [`Key`] can also be used to identify it
#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct AbilityModifiers {
    pub q: f32,
    pub w: f32,
    pub e: f32,
    pub r: f32,
}

impl AbilityModifiers {
    pub const fn modifier(&self, ability_id: AbilityId) -> f32 {
        match match ability_id {
            AbilityId::Q(v) => Some((v, self.q)),
            AbilityId::W(v) => Some((v, self.w)),
            AbilityId::E(v) => Some((v, self.e)),
            AbilityId::R(v) => Some((v, self.r)),
            _ => None,
        } {
            Some((v, mul)) if v as u8 <= AbilityName::Mega as u8 => mul,
            _ => 1.0,
        }
    }
}

#[derive(
    Copy, Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
pub struct OutputCurrentPlayer {
    pub current_stats: PlayerStats,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub adaptive_type: AdaptiveType,
    pub champion_id: ChampionId,
    pub level: u8,
}

#[derive(Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct OutputGame {
    pub monster_damages: [Damages; L_MSTR],
    pub current_player: OutputCurrentPlayer,
    pub enemies: Box<[OutputEnemy]>,
    pub tower_damages: [i32; L_TWRD],
    pub items_meta: Box<[TypeMetadata<ItemId>]>,
    pub runes_meta: Box<[TypeMetadata<RuneId>]>,
}

/// Holds the levels of the abilities of a champion
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

/// Holds the real resistence that a enemy has, after applying the appropriate
/// armor or magic penetrations that the current player has, and other buffs or
/// penalties applied. Field `modifier` is a value between `0.0` and `1.0` that
/// represents the number that will be multiplied by the raw damage to obtain
/// the precise final damage
#[derive(
    Copy, Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
pub struct ResistValue {
    pub real: f32,
    pub modifier: f32,
}

/// ZST with constant methods to calculate some values for the game
/// League of Legends. These formulas may change over time
pub struct RiotFormulas;

impl RiotFormulas {
    /// Rune [`RuneId::AxiomArcanist`] gives +12% bonus damage to `R`
    /// if it deals single target damage. The -3% penalty is not yet
    /// supported for area-damaging ultimates
    pub const AXIOM_ARCANIST_BONUS_DAMAGE: f32 = 1.12;

    /// By 06/07/2025 Earth dragons give +5% resists
    pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;

    /// By 06/07/2025 Fire dragons give +3% bonus attack stats
    pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;

    /// Item [`ItemId::PlatedSteelcaps`] gives 12% damage reduction for basic attacks
    pub const STEEL_CAPS_PROTECTION: f32 = 0.88;

    /// Item [`ItemId::RanduinsOmen`] gives 30% damage reduction against critical hits
    pub const RANDUIN_CRIT_PROTECTION: f32 = 0.7;

    /// Items with Rocksolid passive give 20% damage reduction in some cases
    pub const ROCKSOLID_PROTECTION: f32 = 0.8;

    pub const SHOJIN_BONUS_DAMAGE: f32 = 1.12;
    pub const SHADOWFLAME_BONUS_DAMAGE: f32 = 1.2;
    pub const RIFTMAKER_BONUS_DAMAGE: f32 = 1.08;
    pub const COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE: f32 = 1.08;

    /// Constant array containing the armor and magic resistences of jungle monsters.
    /// Note that there's no specific name to each monster since the damage against
    /// most of them repeats since their armor and magic resistence values are the same
    pub const MONSTER_RESISTS: [(f32, f32); L_MSTR] = [
        // Minions
        (0f32, 0f32),
        // Dragons
        (21f32, 30f32),
        // Red, Blue, Gromp, Wolves
        (42f32, 42f32),
        // Krug, Raptor
        (20f32, 20f32),
        // Baron Nashor
        (120f32, 70f32),
        // Atakhan
        (90f32, 75f32),
        // Super Minion
        (100f32, -30f32),
    ];

    pub const fn missing_health(current_health: f32, max_health: f32) -> f32 {
        1.0 - (current_health / max_health.max(1.0))
    }

    /// Formula to get the bonus damage of the rune [`RuneId::LastStand`], where
    /// missing health is a ratio of the current health and the maximum health.
    /// ```rs
    /// let missing_health = 1.0 - (
    ///     current_player_stats.current_health /
    ///         current_player_stats.health.max(1.0)
    /// );
    /// ```
    /// Note that it uses [`f32::max`] to avoid division by zero.
    ///
    /// - Current Health = 800
    /// - Maximum Health = 1000
    ///
    /// Then you're missing 200 health, which represents 20% of the total HP,
    /// which should translate to 0.2.
    /// Check the formula
    /// `1.0 - (800.0 / 1000.0) = 0.2`
    ///
    /// Also, this formula has a range from 1.0 to 1.11, since in game the
    /// maximum damage increase is of `11%`
    pub const fn get_last_stand(missing_health: f32) -> f32 {
        1.0 + (0.05 + 0.2 * (missing_health - 0.4)).clamp(0.0, 0.11)
    }

    /// Receives the number of Mountain dragons and returns a multiplier that will
    /// be applied to increase some target's armor and magic resistences
    pub const fn get_earth_multiplier(x: u16) -> f32 {
        1.0 + x as f32 * Self::EARTH_DRAGON_MULTIPLIER
    }

    /// Receives the number of fire dragons and returns a number that can be multiplied
    /// by the current ability power and attack damage to obtain the expected current
    /// player's numeric value for those fields
    pub const fn get_fire_multiplier(x: u16) -> f32 {
        1.0 + x as f32 * Self::FIRE_DRAGON_MULTIPLIER
    }

    /// Takes in a slice of numbers between `0.0` and `100.0` and treats them as percentage
    /// armor or magic penetration, returning a number that represents the final penetration
    /// value of the current player. This happens because two `30%` penetrations do not sum
    /// up to `60%` directly, instead they return `51%` penetration.
    pub const fn percent_value(values: &[f32]) -> f32 {
        let mut i = 0;
        let mut result = 1.0;

        while i < values.len() {
            let value = values[i].clamp(0.0, 100.0);
            result *= 1.0 - (value * 0.01);
            i += 1;
        }

        (1.0 - result) * 100.0
    }

    pub const fn combine_percentage(a: f32, b: f32) -> f32 {
        let a = a.clamp(0.0, 100.0);
        let b = b.clamp(0.0, 100.0);
        a + b - (a * b * 0.01)
    }

    /// Returns the real resistence value in a struct [`ResistValue`], taking into account
    /// if it can go below zero or not. If `accept_negatives` is set to `false`, then the
    /// `real` field will be clamped to `0.0`, and the modifier will be set to `1.0`, meaning
    /// that the enemy would have zero resistences after applying the percentage and flat
    /// penetration values
    pub const fn real_resist(
        percent_pen: f32,
        flat_pen: f32,
        resist: f32,
        accept_negatives: bool,
    ) -> ResistValue {
        let real = ((1.0 - percent_pen / 100.0) * resist - flat_pen).max(if accept_negatives {
            f32::NEG_INFINITY
        } else {
            0.0
        });
        let modifier = 100.0 / (100.0 + real);
        ResistValue { real, modifier }
    }

    /// Returns an [`i32`] that represents the damage against some turret
    pub const fn tower_damage(
        plates: u32,
        base_attack_damage: f32,
        bonus_attack_damage: f32,
        ability_power: f32,
        pen_percent: f32,
        pen_flat: f32,
    ) -> i32 {
        let base = base_attack_damage + bonus_attack_damage + ability_power * 0.6;
        let bonus_resist = match plates == 0 {
            true => 0.0,
            false => -25.0 + 50.0 * (plates as f32 - 1.0),
        };
        let raw_resist = 40.0 + bonus_resist;
        let resist = raw_resist * (1.0 - pen_percent / 100.0) - pen_flat;
        let mult = 100.0 / (100.0 + resist);
        (base * mult) as _
    }
}

#[derive(
    Copy, Clone, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
pub struct EnemyStats {
    pub armor: f32,
    pub current_health: f32,
    pub magic_resist: f32,
    pub max_health: f32,
    pub missing_health: f32,
}

#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct SimpleStats {
    pub armor: f32,
    pub max_health: f32,
    pub magic_resist: f32,
}

#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
pub struct BasicStats {
    pub armor: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub magic_resist: f32,
    pub max_health: f32,
    pub max_mana: f32,
}

#[derive(Copy, Clone, Debug, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub ability_power: f32,
    pub armor: f32,
    #[serde(rename = "physicalLethality")]
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub current_health: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_resist: f32,
    pub max_health: f32,
    #[serde(rename = "resourceMax")]
    pub max_mana: f32,
    #[serde(rename = "resourceValue")]
    pub current_mana: f32,
}

macro_rules! impl_default {
    ($ty:ty, $init:literal, $typedef:ty) => {
        impl $ty {
            pub const fn default() -> Self {
                unsafe {
                    core::mem::transmute(
                        [$init as $typedef; size_of::<$ty>() / size_of::<$typedef>()],
                    )
                }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                <$ty>::default()
            }
        }
    };
}

impl_default!(PlayerStats, 0, f32);
impl_default!(BasicStats, 0, f32);
impl_default!(SimpleStats, 0, f32);
impl_default!(DamageModifiers, 1, f32);
impl_default!(AbilityModifiers, 1, f32);
impl_default!(Modifiers, 1, f32);
impl_default!(Dragons, 0, u8);
impl_default!(RangeDamage, 0, i32);
impl_default!(AbilityLevels, 0, u8);
