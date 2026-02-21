#![no_std]
#![allow(unused_imports)]

pub mod bitset;
pub mod cache;
pub mod data;
pub mod enums;
pub mod eval;

use core::{any::TypeId, mem::MaybeUninit, ops::Index, ops::Range, str::FromStr};

#[allow(non_upper_case_globals)]
pub(crate) const unknown: f32 = 0.0;
pub(crate) use tutorlolv2_types::{AbilityId::*, AbilityName::*};

pub use bitset::*;
pub use cache::*;
pub use data::{
    champions::{
        ABILITY_CLOSURES, ABILITY_FORMULAS, ABILITY_IDENTS, ABILITY_IDENTS_INDEX, CHAMPION_CACHE,
        CHAMPION_FORMULAS, CHAMPION_NAME_TO_ID, ChampionId, RECOMMENDED_ITEMS, RECOMMENDED_RUNES,
    },
    items::{ITEM_CACHE, ITEM_CLOSURES, ITEM_FORMULAS, ITEM_IDENTS, ItemId},
    runes::{RUNE_CACHE, RUNE_CLOSURES, RUNE_FORMULAS, RUNE_IDENTS, RuneId},
    *,
};
pub use enums::{Attrs, DamageType};
pub use eval::*;
pub use tutorlolv2_types::*;

use crate::data::{champions::CHAMPION_GENERATOR, items::ITEM_GENERATOR};

pub static RAW_BLOCK: &str = include_str!("block.txt");
const BR_BLOCK: &[u8] = include_bytes!("block.br");
pub static mut BLOCK: &[u8] = BR_BLOCK;

pub const BLOCK_LEN: usize = BR_BLOCK.len();

pub const fn ignite(level: u8) -> i32 {
    let n = level as i32;
    let nth = if n > 4 { n - 4 } else { 0 };
    70 + 20 * n + 5 * nth
}

/// Verifies the following conditions
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
pub const fn is_simulated_item(item: &CachedItem) -> bool {
    let CachedItem {
        purchasable,
        tier,
        price,
        prettified_stats,
        maps,
        ..
    } = *item;

    tier >= 3 && price > 0 && purchasable && !prettified_stats.is_empty() && maps.summoners_rift
}

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
/// - `maps.summoners_rift`
pub const L_SIML: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < ItemId::VARIANTS {
        if is_simulated_item(ITEM_CACHE[i]) {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// Exact number of resistence variations for jungle monsters
pub const L_MSTR: usize = 7;

/// Number of different plates a tower can have. Each tower can have `0..=5` plates
pub const L_TWRD: usize = 6;

/// Stores the simulated items as [`ItemId`], and only those that follow the rules:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
pub const SIMULATED_ITEMS_ENUM: [ItemId; L_SIML] = {
    let mut result = [ItemId::AbyssalMask; _];
    let mut i = 0;
    let mut j = 0;
    while i < ItemId::VARIANTS {
        if is_simulated_item(ITEM_CACHE[i]) {
            result[j] = ItemId::from_repr(i as _).unwrap();
            j += 1;
        }
        i += 1;
    }
    result
};

impl FromStr for ChampionId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CHAMPION_NAME_TO_ID
            .get(s)
            .copied()
            .ok_or("No matches when calling ChampionId::from_str")
    }
}

/// Contains the metadata of all items that have their stats compared to choose
/// which one is best to buy considering the current game state. See [`TypeMetadata`]
/// for more details
pub const SIMULATED_ITEMS_METADATA: [TypeMetadata<ItemId>; L_SIML] = {
    let mut siml_items = MaybeUninit::<[TypeMetadata<ItemId>; L_SIML]>::uninit();
    let siml_items_ptr = siml_items.as_mut_ptr();
    let mut i = 0;
    while i < L_SIML {
        let item_id = SIMULATED_ITEMS_ENUM[i];
        let CachedItem {
            metadata:
                TypeMetadata {
                    damage_type,
                    attributes,
                    ..
                },
            ..
        } = *ITEM_CACHE[item_id as usize];
        unsafe {
            core::ptr::addr_of_mut!((*siml_items_ptr)[i]).write(TypeMetadata::<ItemId> {
                kind: item_id,
                damage_type,
                attributes,
            })
        };
        i += 1;
    }
    unsafe { siml_items.assume_init() }
};

/// Number of runes that can damage enemies. Currently they're generated manually and
/// might be outdated. Also, they're stored in a single `.json` file, instead of containing
/// a dedicated file for each rune
pub const NUMBER_OF_DAMAGING_RUNES: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < RuneId::VARIANTS {
        let rune = RUNE_CACHE[i];
        if !rune.undeclared {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// Number of items that can damage enemies. All items have their own files
/// and access to the `MerakiCdn` collected data, which can be used to create
/// their damage closures and insert in a static variable, replacing the [`zero`] constant
pub const NUMBER_OF_DAMAGING_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < ItemId::VARIANTS {
        let item = ITEM_CACHE[i];
        if !item.deals_damage {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// A constant array of all items that can damage enemies, holding their internal ids,
/// defined by the enum [`ItemId`]
pub const DAMAGING_ITEMS_ARRAY: [ItemId; NUMBER_OF_DAMAGING_ITEMS] = {
    let mut result = [ItemId::AbyssalMask; _];
    let mut i = 0;
    let mut j = 0;
    while i < ItemId::VARIANTS {
        let item = ITEM_CACHE[i];
        if item.deals_damage {
            result[j] = ItemId::from_repr(i as _).unwrap();
            j += 1;
        }
        i += 1;
    }
    result
};

/// A constant array of all runes that can damage enemies, holding their internal ids,
/// defined by the enum [`RuneId`]
pub const DAMAGING_RUNES_ARRAY: [RuneId; NUMBER_OF_DAMAGING_RUNES] = {
    let mut result = [RuneId::AbilityHaste; _];
    let mut i = 0;
    let mut j = 0;
    while i < RuneId::VARIANTS {
        let rune = RUNE_CACHE[i];
        if !rune.undeclared {
            result[j] = RuneId::from_repr(i as _).unwrap();
            j += 1;
        }
        i += 1;
    }
    result
};

pub const DAMAGING_ITEMS: ItemsBitSet = bitset_items(DAMAGING_ITEMS_ARRAY);
pub const DAMAGING_RUNES: RunesBitSet = bitset_runes(DAMAGING_RUNES_ARRAY);

/// Counts how many damaging abilities ewe have across all champions. This is used to
/// determine a proper size of how many abilities we should allow to live in the stack
/// before leaking it to the heap to avoid stack overflows
pub const NUMBER_OF_ABILITIES: usize = {
    let mut i = 0;
    let mut sum = 0;
    while i < ChampionId::VARIANTS {
        let data = CHAMPION_CACHE[i];
        sum += data.closures.len();
        i += 1;
    }
    sum
};

pub static CHAMPION_POSITIONS: [&[Position]; ChampionId::VARIANTS] = {
    let mut i = 0;
    let mut result = [&[] as &[_]; _];
    while i < ChampionId::VARIANTS {
        let champion = CHAMPION_CACHE[i];
        result[i] = champion.positions;
        i += 1;
    }
    result
};

const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion_id = ChampionId::VALUES[i];
        let cache = champion_id.cache();

        let merge_data = cache.merge_data;
        let len = cache.metadata.len();

        assert!(len == champion_id.closures().len());
        assert!(len == champion_id.ident_indexes().len());

        let mut j = 0;
        while j < merge_data.len() {
            let m = &merge_data[j];
            assert!((m.minimum_damage as usize) < len);
            assert!((m.maximum_damage as usize) < len);
            assert!(m.minimum_damage < m.maximum_damage);
            if j + 1 < merge_data.len() {
                let a = &merge_data[j];
                let b = &merge_data[j + 1];
                assert!(a.maximum_damage < b.maximum_damage);
            }
            j += 1;
        }
        i += 1;
    }
};

/// Assert there were no undefined behavior while creating [`CHAMPION_POSITIONS`]
const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion = CHAMPION_CACHE[i].positions;
        let position = CHAMPION_POSITIONS[i];
        assert!(!position.is_empty());
        assert!(champion.len() == position.len());
        let mut j = 0;
        while j < champion.len() {
            let pos_a = champion[j];
            let pos_b = position[j];
            assert!(pos_a as u8 == pos_b as u8);
            j += 1;
        }
        i += 1;
    }
};

impl TryFrom<&str> for ChampionId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CHAMPION_NAME_TO_ID
            .get(value)
            .copied()
            .ok_or("Failed to convert &str to ChampionId on TryFrom trait")
    }
}

impl ChampionId {
    pub const CLOSURES: &[&[Range<usize>]; Self::VARIANTS] = &ABILITY_CLOSURES;
    pub const ABILITIES: &[&[Range<usize>]; Self::VARIANTS] = &ABILITY_FORMULAS;

    pub const fn number_of_abilities(&self) -> usize {
        self.cache().closures.len()
    }

    pub const fn recommended_items(&self, position: Position) -> &'static [ItemId] {
        RECOMMENDED_ITEMS[self.index()][position as usize]
    }

    pub const fn recommended_runes(&self, position: Position) -> &'static [RuneId] {
        RECOMMENDED_RUNES[self.index()][position as usize]
    }

    pub const fn positions(&self) -> &'static [Position] {
        self.cache().positions
    }

    pub const fn main_position(&self) -> Position {
        self.positions()[0]
    }

    pub const fn closures(&self) -> &'static [Range<usize>] {
        Self::CLOSURES[self.index()]
    }

    pub const fn ident_indexes(&self) -> &'static [Range<usize>] {
        ABILITY_IDENTS_INDEX[self.index()]
    }

    pub const fn ability_formulas(&self) -> &'static [Range<usize>] {
        ABILITY_FORMULAS[self.index()]
    }

    pub const fn get_ability_formula(&self, index: usize) -> &'static Range<usize> {
        &self.ability_formulas()[index]
    }

    pub const fn generator(&self) -> &'static Range<usize> {
        &CHAMPION_GENERATOR[self.index()]
    }
}

impl ItemId {
    pub const CLOSURES: &[Range<usize>; Self::VARIANTS] = &ITEM_CLOSURES;
    pub const RIOT_IDS: [u32; Self::VARIANTS] = {
        let mut result = [0; _];
        let mut i = 0;
        while i < Self::VARIANTS {
            let value = Self::VALUES[i];
            result[i] = value.to_riot_id();
            i += 1;
        }
        result
    };

    pub const fn has_stat(&self, stat_name: StatName) -> bool {
        let mut i = 0;
        let stats = self.cache().prettified_stats;
        while i < stats.len() {
            if stats[i].0 as u8 == stat_name as u8 {
                return true;
            }
            i += 1;
        }
        false
    }

    pub const fn find_variants<const N: usize>(stat_name: StatName) -> [ItemId; N] {
        let mut i = 0;
        let mut j = 0;
        let mut result = [Self::default(); N];
        while i < Self::VARIANTS {
            let item = Self::VALUES[i];
            if item.has_stat(stat_name) {
                result[j] = item;
                j += 1;
            }
            i += 1;
        }
        result
    }

    pub const FILTERS: [&[Self]; StatName::VARIANTS] = [
        &Self::ITEMS_WITH_ABILITY_HASTE,
        &Self::ITEMS_WITH_ABILITY_POWER,
        &Self::ITEMS_WITH_ADAPTIVE_FORCE,
        &Self::ITEMS_WITH_ARMOR,
        &Self::ITEMS_WITH_ARMOR_PENETRATION,
        &Self::ITEMS_WITH_ATTACK_DAMAGE,
        &Self::ITEMS_WITH_ATTACK_SPEED,
        &Self::ITEMS_WITH_BASE_HEALTH_REGEN,
        &Self::ITEMS_WITH_BASE_MANA_REGEN,
        &Self::ITEMS_WITH_CRITICAL_STRIKE_CHANCE,
        &Self::ITEMS_WITH_CRITICAL_STRIKE_DAMAGE,
        &Self::ITEMS_WITH_GOLD_PER10_SECONDS,
        &Self::ITEMS_WITH_HEAL_AND_SHIELD_POWER,
        &Self::ITEMS_WITH_HEALTH,
        &Self::ITEMS_WITH_LETHALITY,
        &Self::ITEMS_WITH_LIFE_STEAL,
        &Self::ITEMS_WITH_MAGIC_PENETRATION,
        &Self::ITEMS_WITH_MAGIC_RESIST,
        &Self::ITEMS_WITH_MANA,
        &Self::ITEMS_WITH_MOVE_SPEED,
        &Self::ITEMS_WITH_OMNIVAMP,
        &Self::ITEMS_WITH_TENACITY,
    ];

    pub const ITEMS_WITH_ABILITY_HASTE: [Self; Self::count_variants(StatName::AbilityHaste)] =
        Self::find_variants(StatName::AbilityHaste);
    pub const ITEMS_WITH_ABILITY_POWER: [Self; Self::count_variants(StatName::AbilityPower)] =
        Self::find_variants(StatName::AbilityPower);
    pub const ITEMS_WITH_ADAPTIVE_FORCE: [Self; Self::count_variants(StatName::AdaptiveForce)] =
        Self::find_variants(StatName::AdaptiveForce);
    pub const ITEMS_WITH_ARMOR: [Self; Self::count_variants(StatName::Armor)] =
        Self::find_variants(StatName::Armor);
    pub const ITEMS_WITH_ARMOR_PENETRATION: [Self;
        Self::count_variants(StatName::ArmorPenetration)] =
        Self::find_variants(StatName::ArmorPenetration);
    pub const ITEMS_WITH_ATTACK_DAMAGE: [Self; Self::count_variants(StatName::AttackDamage)] =
        Self::find_variants(StatName::AttackDamage);
    pub const ITEMS_WITH_ATTACK_SPEED: [Self; Self::count_variants(StatName::AttackSpeed)] =
        Self::find_variants(StatName::AttackSpeed);
    pub const ITEMS_WITH_BASE_HEALTH_REGEN: [Self;
        Self::count_variants(StatName::BaseHealthRegen)] =
        Self::find_variants(StatName::BaseHealthRegen);
    pub const ITEMS_WITH_BASE_MANA_REGEN: [Self; Self::count_variants(StatName::BaseManaRegen)] =
        Self::find_variants(StatName::BaseManaRegen);
    pub const ITEMS_WITH_CRITICAL_STRIKE_CHANCE: [Self;
        Self::count_variants(StatName::CriticalStrikeChance)] =
        Self::find_variants(StatName::CriticalStrikeChance);
    pub const ITEMS_WITH_CRITICAL_STRIKE_DAMAGE: [Self;
        Self::count_variants(StatName::CriticalStrikeDamage)] =
        Self::find_variants(StatName::CriticalStrikeDamage);
    pub const ITEMS_WITH_GOLD_PER10_SECONDS: [Self;
        Self::count_variants(StatName::GoldPer10Seconds)] =
        Self::find_variants(StatName::GoldPer10Seconds);
    pub const ITEMS_WITH_HEAL_AND_SHIELD_POWER: [Self;
        Self::count_variants(StatName::HealAndShieldPower)] =
        Self::find_variants(StatName::HealAndShieldPower);
    pub const ITEMS_WITH_HEALTH: [Self; Self::count_variants(StatName::Health)] =
        Self::find_variants(StatName::Health);
    pub const ITEMS_WITH_LETHALITY: [Self; Self::count_variants(StatName::Lethality)] =
        Self::find_variants(StatName::Lethality);
    pub const ITEMS_WITH_LIFE_STEAL: [Self; Self::count_variants(StatName::LifeSteal)] =
        Self::find_variants(StatName::LifeSteal);
    pub const ITEMS_WITH_MAGIC_PENETRATION: [Self;
        Self::count_variants(StatName::MagicPenetration)] =
        Self::find_variants(StatName::MagicPenetration);
    pub const ITEMS_WITH_MAGIC_RESIST: [Self; Self::count_variants(StatName::MagicResist)] =
        Self::find_variants(StatName::MagicResist);
    pub const ITEMS_WITH_MANA: [Self; Self::count_variants(StatName::Mana)] =
        Self::find_variants(StatName::Mana);
    pub const ITEMS_WITH_MOVE_SPEED: [Self; Self::count_variants(StatName::MoveSpeed)] =
        Self::find_variants(StatName::MoveSpeed);
    pub const ITEMS_WITH_OMNIVAMP: [Self; Self::count_variants(StatName::Omnivamp)] =
        Self::find_variants(StatName::Omnivamp);
    pub const ITEMS_WITH_TENACITY: [Self; Self::count_variants(StatName::Tenacity)] =
        Self::find_variants(StatName::Tenacity);

    pub const fn filter(stat_name: StatName) -> &'static [Self] {
        Self::FILTERS[stat_name as usize]
    }

    pub const fn to_riot_id(&self) -> u32 {
        self.cache().riot_id
    }

    pub const fn closure(&self) -> &'static Range<usize> {
        &Self::CLOSURES[self.index()]
    }

    pub const fn generator(&self) -> &'static Range<usize> {
        &ITEM_GENERATOR[self.index()]
    }

    pub const fn count_variants(stat_name: StatName) -> usize {
        let mut result = 0;
        let mut i = 0;
        while i < Self::VARIANTS {
            if Self::VALUES[i].has_stat(stat_name) {
                result += 1;
            }
            i += 1;
        }
        result
    }
}

impl RuneId {
    pub const CLOSURES: &[Range<usize>; Self::VARIANTS] = &RUNE_CLOSURES;
    pub const RIOT_IDS: [u32; Self::VARIANTS] = {
        let mut result = [0; _];
        let mut i = 0;
        while i < Self::VARIANTS {
            let value = Self::VALUES[i];
            result[i] = value.to_riot_id();
            i += 1;
        }
        result
    };

    pub const fn to_riot_id(&self) -> u32 {
        self.cache().riot_id
    }

    pub const fn closure(&self) -> &'static Range<usize> {
        &Self::CLOSURES[self.index()]
    }
}

macro_rules! impl_methods {
    (inner $stru:ident, $($repr:ty),*) => {
        pastey::paste! {
            $(
                impl TryFrom<$repr> for $stru {
                    type Error = &'static str;
                    fn try_from(value: $repr) -> Result<Self, Self::Error> {
                        Self::from_repr(value as _).ok_or(concat!(
                            "Index out of bounds when converting ",
                            stringify!($repr),
                            " to ",
                            stringify!($stru)
                        ))
                    }
                }

                impl TryFrom<&$repr> for $stru {
                    type Error = &'static str;
                    fn try_from(value: &$repr) -> Result<Self, Self::Error> {
                        Self::from_repr(*value as _).ok_or(concat!(
                            "Index out of bounds when converting ",
                            stringify!($repr),
                            " to ",
                            stringify!($stru)
                        ))
                    }
                }

                impl $stru {
                    pub const unsafe fn [<from_ $repr _unchecked>](id: $repr) -> Self {
                        unsafe { Self::from_repr_unchecked(id as _) }
                    }

                    pub const fn [<from_ $repr>](id: $repr) -> Option<Self> {
                        match id < Self::VARIANTS as _ {
                            true => unsafe { Some(Self::from_repr_unchecked(id as _)) },
                            false => None
                        }
                    }
                }
            )*
        }
    };
    ($($stru:ident => $repr:ty),+$(,)*) => {
        pastey::paste! {
            $(
                impl Default for $stru {
                    fn default() -> Self {
                        Self::default()
                    }
                }

                impl_methods!(inner $stru, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

                impl $stru {
                    pub const VALUES: [Self; Self::VARIANTS] = {
                        let mut i = 0;
                        let mut result = [Self::default(); _];
                        while i < Self::VARIANTS {
                            result[i] = Self::from_repr(i as _).unwrap();
                            i += 1;
                        }
                        result
                    };

                    pub const NAMES: [&str; Self::VARIANTS] = {
                        let mut i = 0;
                        let mut result = [""; _];
                        while i < Self::VARIANTS {
                            result[i] = Self::VALUES[i].name();
                            i += 1;
                        }
                        result
                    };

                    pub const FORMULAS: &[Range<usize>; Self::VARIANTS] = &[<$stru:replace("Id", ""):upper _FORMULAS>];

                    pub const unsafe fn from_repr_unchecked(id: $repr) -> Self {
                        unsafe { core::mem::transmute(id) }
                    }

                    pub const fn from_repr(id: $repr) -> Option<Self> {
                        match id < Self::VARIANTS as _ {
                            true => unsafe { Some(Self::from_repr_unchecked(id as _)) },
                            false => None
                        }
                    }

                    pub const fn default() -> Self {
                        unsafe { Self::from_repr_unchecked(0) }
                    }

                    pub const fn cache(&self) -> &'static [<Cached $stru:replace("Id", "")>] {
                        [<$stru:replace("Id", ""):upper _CACHE>][self.index()]
                    }

                    pub const fn name(&self) -> &'static str {
                        self.cache().name
                    }

                    pub const fn index(&self) -> usize {
                        *self as _
                    }

                    pub const fn idents(&self) -> &'static [CtxVar] {
                        [<$stru:replace("Id", ""):replace("Champion", "Ability"):upper _IDENTS>][self.index()]
                    }
                }

                impl CastId for $stru {
                    const VARIANTS: usize = Self::VARIANTS;
                    const NAMES: &'static [&'static str] = &Self::NAMES;
                    const VALUES: &'static [Self] = &Self::VALUES;
                    const FORMULAS: &'static [Range<usize>] = Self::FORMULAS;

                    fn name(&self) -> &'static str {
                        self.name()
                    }

                    fn index(&self) -> usize {
                        self.index()
                    }

                    fn idents(&self) -> &'static [CtxVar] {
                        self.idents()
                    }
                }
            )+
        }
    };
}

impl_methods!(
    ChampionId => u8,
    ItemId => u16,
    RuneId => u8
);

pub const ZEROED_STATS: CachedItemStats = CachedItemStats {
    ability_power: 0.0,
    armor: 0.0,
    armor_penetration_percent: 0.0,
    armor_penetration_flat: 0.0,
    magic_penetration_percent: 0.0,
    magic_penetration_flat: 0.0,
    attack_damage: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    crit_damage: 0.0,
    health: 0.0,
    lifesteal: 0.0,
    magic_resist: 0.0,
    mana: 0.0,
    movespeed: 0.0,
    omnivamp: 0.0,
};

pub trait CastId
where
    Self: Sized + 'static,
{
    const VARIANTS: usize;
    const NAMES: &'static [&'static str];
    const VALUES: &'static [Self];
    const FORMULAS: &'static [Range<usize>];

    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn idents(&self) -> &'static [CtxVar];
    fn formula(&self) -> &'static Range<usize> {
        &Self::FORMULAS[self.index()]
    }
    fn is(value: &TypeId) -> bool {
        *value == TypeId::of::<Self>()
    }
}
