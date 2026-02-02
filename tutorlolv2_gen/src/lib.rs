#![no_std]
#![allow(unused_imports)]

pub mod bitset;
pub mod cache;
pub mod data;
pub mod enums;
pub mod eval;

#[allow(non_upper_case_globals)]
pub(crate) const unknown: f32 = 0.0;
pub(crate) use core::ops::Range;

pub use bitset::*;
pub use cache::*;
pub use data::{
    champions::{
        ABILITY_IDENTS, CHAMPION_CACHE, CHAMPION_NAME_TO_ID, ChampionId, RECOMMENDED_ITEMS,
        RECOMMENDED_RUNES,
    },
    items::{ITEM_CACHE, ItemId},
    runes::{RUNE_CACHE, RuneId},
    *,
};
pub use enums::{Attrs, DamageType};
pub use eval::*;
pub use tutorlolv2_types::*;

pub static RAW_BLOCK: &str = include_str!("block.txt");
pub static BLOCK: &[u8] = include_bytes!("block.br");

pub const BLOCK_LEN: usize = BLOCK.len();

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
        ..
    } = *item;

    tier >= 3 && price > 0 && purchasable && !prettified_stats.is_empty()
}

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
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
            result[j] = unsafe { ItemId::from_u16_unchecked(i as _) };
            j += 1;
        }
        i += 1;
    }
    result
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
            result[j] = unsafe { ItemId::from_u16_unchecked(i as _) };
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
            result[j] = unsafe { RuneId::from_u8_unchecked(i as _) };
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

pub static CHAMPION_ID_TO_NAME: [&str; ChampionId::VARIANTS] = {
    let mut i = 0;
    let mut result = [""; _];
    while i < ChampionId::VARIANTS {
        let champion = CHAMPION_CACHE[i];
        result[i] = champion.name;
        i += 1;
    }
    result
};

pub static CHAMPION_POSITIONS: [&[Position]; ChampionId::VARIANTS] = {
    let mut i = 0;
    let mut result = [unsafe { core::mem::transmute("") }; _];
    while i < ChampionId::VARIANTS {
        let champion = CHAMPION_CACHE[i];
        result[i] = champion.positions;
        i += 1;
    }
    result
};

/// Assert there were no undefined behavior while creating [`CHAMPION_POSITIONS`]
const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion = CHAMPION_CACHE[i].positions;
        let position = CHAMPION_POSITIONS[i];
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

pub static ITEM_ID_TO_NAME: [&str; ItemId::VARIANTS] = {
    let mut i = 0;
    let mut result = [""; _];
    while i < ItemId::VARIANTS {
        let item = ITEM_CACHE[i];
        result[i] = item.name;
        i += 1;
    }
    result
};

pub static ITEM_ID_TO_RIOT_ID: [u32; ItemId::VARIANTS] = {
    let mut i = 0;
    let mut result = [0; _];
    while i < ItemId::VARIANTS {
        let item = ITEM_CACHE[i];
        result[i] = item.riot_id;
        i += 1;
    }
    result
};

pub static RUNE_ID_TO_NAME: [&str; RuneId::VARIANTS] = {
    let mut i = 0;
    let mut result = [""; _];
    while i < RuneId::VARIANTS {
        let rune = RUNE_CACHE[i];
        result[i] = rune.name;
        i += 1;
    }
    result
};

pub static RUNE_ID_TO_RIOT_ID: [u32; RuneId::VARIANTS] = {
    let mut i = 0;
    let mut result = [0; _];
    while i < RuneId::VARIANTS {
        let rune = RUNE_CACHE[i];
        result[i] = rune.riot_id;
        i += 1;
    }
    result
};

macro_rules! const_methods {
    (inner $name:ident, $repr:ident, $($cast:ident),+) => {
        pastey::paste! {
            $(
                impl From<&$name> for $cast {
                    fn from(value: &$name) -> Self {
                        value.index() as _
                    }
                }

                impl From<$name> for $cast {
                    fn from(value: $name) -> Self {
                        value.index() as _
                    }
                }

                impl TryFrom<$cast> for $name {
                    type Error = &'static str;
                    fn try_from(value: $cast) -> Result<Self, Self::Error> {
                        Self::[<from_ $repr>](value as _)
                            .ok_or(concat!(
                                "Index out of bounds when casting ",
                                stringify!($cast),
                                " to ",
                                stringify!($name)
                            ))
                    }
                }
            )+
        }
    };
    ($name:ident, $repr:ident) => {
        pastey::paste! {
            const_methods!(inner $name, $repr, u16, u32, u64, u128, usize);

            impl $name {
                pub const ARRAY: [Self; Self::VARIANTS] = {
                    let mut i = 0;
                    let mut result = [unsafe { Self::[<from_ $repr _unchecked>](0) }; _];
                    while i < Self::VARIANTS {
                        result[i] = unsafe { Self::[<from_ $repr _unchecked>](i as _) };
                        i += 1;
                    }
                    result
                };

                pub const fn get_cache(&self) -> &'static [<Cached $name:replace("Id", "")>] {
                    [<$name:replace("Id", ""):upper _CACHE>][self.index()]
                }

                pub const fn name(&self) -> &'static str {
                    self.get_cache().name
                }

                pub const fn index(&self) -> usize {
                    *self as _
                }

                pub fn [<is_ $name:snake>](value: &core::any::TypeId) -> bool {
                    *value == core::any::TypeId::of::<$name>()
                }
            }

            impl $name {
                pub const fn default() -> Self {
                    unsafe { Self::[<from_ $repr _unchecked>](0) }
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::default()
                }
            }

            impl Into<&'static str> for $name {
                fn into(self) -> &'static str {
                    self.name()
                }
            }

            impl Into<&'static [<Cached $name:replace("Id", "")>]> for $name {
                fn into(self) -> &'static [<Cached $name:replace("Id", "")>] {
                    self.get_cache()
                }
            }

            impl Into<&'static [Self]> for $name {
                fn into(self) -> &'static [Self] {
                    &Self::ARRAY
                }
            }
        }
    };
}

const_methods!(ChampionId, u8);
const_methods!(ItemId, u16);
const_methods!(RuneId, u8);

impl TryFrom<&str> for ChampionId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CHAMPION_NAME_TO_ID
            .get(value)
            .ok_or("Failed to convert value to ChampionId enum")
            .copied()
    }
}

impl ChampionId {
    /// Counts how many damaging abilities a champion has
    pub const fn number_of_abilities(&self) -> usize {
        self.get_cache().closures.len()
    }

    pub const fn recommended_items(&self, position: Position) -> &'static [ItemId] {
        RECOMMENDED_ITEMS[self.index()][position as usize]
    }

    pub const fn recommended_runes(&self, position: Position) -> &'static [RuneId] {
        RECOMMENDED_RUNES[self.index()][position as usize]
    }

    pub const fn idents(&self) -> &'static [EvalIdent] {
        ABILITY_IDENTS[self.index()]
    }
}

macro_rules! riot_id_array {
    ($($enum:ty),*) => {
        $(
            impl $enum {
                pub const RIOT_ID_ARRAY: [u32; Self::VARIANTS] = {
                    let mut result = [0; _];
                    let mut i = 0;
                    while i < Self::VARIANTS {
                        let value = Self::ARRAY[i];
                        result[i] = value.to_riot_id();
                        i += 1;
                    }
                    result
                };
            }
        )*
    };
}

riot_id_array!(ItemId, RuneId);

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
