use crate::{ItemId, RuneId};
use bincode::{Decode, Encode};
use core::{ops::Index, str::FromStr};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::*;

/// A generic metadata holder for [`AbilityId`], [`ItemId`], or [`RuneId`].
/// Contains its damage type, attributes, and which instance of the enum the value is.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
pub struct TypeMetadata<T> {
    /// Represents a variety of values:
    /// - [`AbilityId`] Which ability key it represents, and its name
    /// - [`ItemId`] Can be casted to [`usize`] and indexes into [`crate::ITEM_CACHE`]
    /// - [`RuneId`] Can be casted to [`usize`] and indexes into [`crate::RUNE_CACHE`]
    pub kind: T,
    /// Represents the damage type of the current instance
    pub damage_type: DamageType,
    /// A variety of possible extra attributes the current instance can have.
    /// See [`Attrs`] for more details
    pub attributes: Attrs,
}

/// Definition of a closure that lives in the generated static variables of
/// cache fields, such as [`crate::CHAMPION_CACHE`], [`crate::ITEM_CACHE`], or
/// [`crate::RUNE_CACHE`]. They all receive a [`Ctx`], which contains
/// more than the necessary information to calculate the damage of some ability,
/// item, passive, or rune, and they return an [`f32`], which represents the calculated
/// damage. All of them must be qualified as `const`, capturing no variables
pub type ConstClosure = fn(&Ctx) -> f32;

/// Generated data about some champion, held in the static variable [`crate::CHAMPION_CACHE`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CachedChampion {
    /// A champion's in-game display name in `English`. This application converts
    /// names from other languages to English, but does not do the opposite, nor
    /// store them internally. If you want to translate the name in English to
    /// some other language, you will have to implement it on your own. Although
    /// not recommended, you can use this field to get the enum [`crate::ChampionId`]
    /// of the current champion by using the `phf map` [`crate::CHAMPION_NAME_TO_ID`].
    pub name: &'static str,

    /// The adaptive force is determined by the current ability power and
    /// bonus damage. However, when they tie (`ability_power == bonus_damage`),
    /// the adaptive force is determined by this pre-assigned value. This is
    /// very common to happen with tanks, who often do not build any items that
    /// give any ability power or bonus damage
    pub adaptive_type: AdaptiveType,

    /// Determines if this champion attacks are considered melee or ranged
    pub attack_type: AttackType,

    /// Array representing what roles that champion commonly plays, sorted by
    /// frequency. A champion that plays mostly on top, but commonly on middle
    /// as well will have be assigned `&[Position::Top, Position::Middle]`
    pub positions: &'static [Position],

    /// Holds basic information about the champion's base stats and growth rate
    pub stats: CachedChampionStats,

    pub combos: &'static [&'static [ComboElement]],

    /// Sorted array that holds basic information about the abilities
    /// of some cached champion, such as their internal id, damage type,
    /// and special attributes, taking up at most 4 bytes of space. The order
    /// of the abilities is based on priority, following the sequence:
    /// - `P`, `Q`, `W`, `E`, `R`
    ///
    /// Note that not all abilities and passives will be represented in this
    /// array because we're only worried about abilities and passives that deal
    /// damage. If it doesn't affect the damage calculation, it will not appear
    /// here, and will not be assigned a `closure`, since it would just waste
    /// memory space by placing the [`zero`] constant as placeholder
    pub metadata: &'static [TypeMetadata<AbilityId>],

    /// Sorted array that have the same length as the field `metadata`. Each
    /// function pointer stored here matches the same index in the `metadata`
    /// field, which means that for example, if the element zero in metadata,
    /// or `metadata[0]` is [`Q::_1`], (alias to [`AbilityId::Q`] with name
    /// [`AbilityName::_1`]), then `closures[0]` is a function that when evaluated
    /// returns the damage of that ability before any modifier is applied, which
    /// means that it is like if the ability dealt true damage
    pub closures: &'static [ConstClosure],

    /// Some abilities have a minimum and maximum damage. The most common way to
    /// identify it is if some ability has two elements of the same key [`AbilityId`]
    /// while having two instances of [`AbilityName`] where one ends with `Min` and the
    /// other one with `Max`.
    ///
    /// For example, if some champion has [`Q::_1Min`] and [`Q::_1Max`], then those
    /// are considered separate abilities when evaluating their damage, but when displaying
    /// in the frontend application, it is merged into a single cell that represents the key
    /// [`Q::_1`], where it will look like `minimum damage - maximum damage`, replaced with
    /// the calculated value. In other words, the user will see `Q::_1Min - Q::_1Max` in the
    /// table cell <strong>Q<sub>1</sub></strong>
    ///
    /// Each tuple contains the offsets in the `metadata` array where those abilities
    /// should be mergede. It is also ordered following the same sequence as defined
    /// in `metadata`, and ignores abilities that deal no damage. Abilities that deal only
    /// one damage will always count towards `minimum_damage` only, and the field
    /// `maximum_damage` will be set to zero, which means it should be ignored
    ///
    /// Lets say suppose the following scenario:
    /// ```rs
    /// let closures = [|ctx| { ctx.ap + 50.0 }; 5];
    /// let merge_data = [(0, 2), (4, 3)];
    /// let metadata = [Q::_1Min, Q::_2, Q::_1Max, Q::_4Max, Q::_4Min];
    /// let eval_ctx = Ctx::default();
    /// for (i, j) in merge_data {
    ///     let min_dmg = closures[i](&eval_ctx);
    ///     let max_dmg = closures[j](&eval_ctx);
    ///     let min_ability = metadata[i].alias;
    ///     let max_ability = metadata[j].alias;
    ///     println!("[tab] {min_ability} - {max_ability}");
    ///     println!("[dmg] {min_dmg} - {max_dmg}");
    /// }
    /// ```
    pub merge_data: &'static [MergeData],
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct CachedChampionStatsMap {
    pub flat: f32,
    pub per_level: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct CachedChampionStats {
    pub health: CachedChampionStatsMap,
    pub mana: CachedChampionStatsMap,
    pub armor: CachedChampionStatsMap,
    pub magic_resist: CachedChampionStatsMap,
    pub attack_damage: CachedChampionStatsMap,
    pub attack_speed: CachedChampionStatsMap,
    pub movespeed: f32,
    pub crit_damage: f32,
    pub crit_damage_mod: f32,
    pub attack_speed_mod: f32,
    pub attack_range: f32,
    pub aram_damage_taken: f32,
    pub aram_damage_dealt: f32,
    pub urf_damage_taken: f32,
    pub urf_damage_dealt: f32,
}

/// Generated data about some item, held in the static variable [`crate::ITEM_CACHE`]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CachedItem {
    pub name: &'static str,
    pub tier: u8,
    pub price: u16,
    pub prettified_stats: &'static [(StatName, u16)],
    pub maps: ItemMaps,
    pub stats: CachedItemStats,
    pub metadata: TypeMetadata<ItemId>,
    pub ranged_damages: [ConstClosure; 2],
    pub melee_damages: [ConstClosure; 2],
    pub deals_damage: (bool, bool),
    pub purchasable: bool,
    pub riot_id: u32,
}

/// Generated data about some rune, held in the static variable [`crate::RUNE_CACHE`]
#[derive(Clone, Copy, Debug)]
pub struct CachedRune {
    pub name: &'static str,
    pub metadata: TypeMetadata<RuneId>,
    pub melee_damage: ConstClosure,
    pub ranged_damage: ConstClosure,
    pub riot_id: u32,
    pub undeclared: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Serialize)]
pub struct CachedItemStats {
    pub ability_power: f32,
    pub adaptive_force: f32,
    pub armor: f32,
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub health: f32,
    pub lifesteal: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_resist: f32,
    pub mana: f32,
    pub movespeed: f32,
    pub omnivamp: f32,
}

/// A constant function that returns `0.0f32`. This is used as a placeholder
/// when some item, rune, or ability doesn't deal any damage, but must be defined
/// in the static variable to meet the requirements of the compiler. In the best-case
/// scenario, this function should never be called, to avoid wasting CPU time with
/// a compile-time known result
pub const fn zero(_: &Ctx) -> f32 {
    0.0
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Encode, Serialize)]
pub struct ItemMaps {
    pub aram: bool,
    pub arena: bool,
    pub nexus_blitz: bool,
    pub summoners_rift: bool,
    pub tft: bool,
    pub unknown_map_33: bool,
    pub unknown_map_35: bool,
}

impl Index<GameMap> for ItemMaps {
    type Output = bool;

    fn index(&self, index: GameMap) -> &Self::Output {
        match index {
            GameMap::Aram => &self.aram,
            GameMap::Arena => &self.arena,
            GameMap::NexusBlitz => &self.nexus_blitz,
            GameMap::SummonersRift => &self.summoners_rift,
            GameMap::Tft => &self.tft,
            GameMap::UnknownMap33 => &self.unknown_map_33,
            GameMap::UnknownMap35 => &self.unknown_map_35,
            _ => &false,
        }
    }
}
