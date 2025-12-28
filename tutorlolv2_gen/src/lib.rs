#![no_std]
#![allow(unused_imports)]

#[cfg(feature = "eval")]
pub mod bitset;
pub mod cache;
pub mod data;
pub mod enums;
pub mod eval;

#[cfg(feature = "eval")]
pub use bitset::*;
pub use cache::*;
pub use data::*;
pub use enums::{Attrs, DamageType};
#[cfg(feature = "eval")]
pub use eval::*;
pub use tutorlolv2_types::*;

#[cfg(feature = "glob")]
pub const RAW_BLOCK: &str = include_str!("block.txt");

/// Verifies the following conditions
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
#[cfg(feature = "eval")]
pub const fn is_simulated_item(item: &CachedItem) -> bool {
    let CachedItem {
        purchasable,
        tier,
        price,
        prettified_stats,
        ..
    } = *item;

    tier >= 3 && price > 0 && purchasable && prettified_stats.len() > 0
}

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
#[cfg(feature = "eval")]
pub const NUMBER_OF_SIMULATED_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_ITEMS {
        if is_simulated_item(ITEM_CACHE[i]) {
            sum += 1;
        }
        i += 1;
    }
    sum
};

pub const L_SIML: usize = {
    const N: usize = 118;
    #[cfg(feature = "eval")]
    match NUMBER_OF_SIMULATED_ITEMS == N {
        true => N,
        false => NUMBER_OF_SIMULATED_ITEMS,
        // false => panic!("Number of simulated items is outdated"),
    }
    #[cfg(not(feature = "eval"))]
    N
};

/// Stores the simulated items as [`ItemId`], and only those that follow the rules:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
#[cfg(feature = "eval")]
pub const SIMULATED_ITEMS_ENUM: [ItemId; NUMBER_OF_SIMULATED_ITEMS] = {
    let mut result = [ItemId::AbyssalMask; NUMBER_OF_SIMULATED_ITEMS];
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_ITEMS {
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
#[cfg(feature = "eval")]
pub const NUMBER_OF_DAMAGING_RUNES: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_RUNES {
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
#[cfg(feature = "eval")]
pub const NUMBER_OF_DAMAGING_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_RUNES {
        let item = ITEM_CACHE[i];
        if !item.deals_damage {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// A constant array of all runes that can damage enemies, holding their internal ids,
/// defined by the enum [`RuneId`]
#[cfg(feature = "eval")]
pub const DAMAGING_RUNES_ARRAY: [RuneId; NUMBER_OF_DAMAGING_RUNES] = {
    let mut result = [RuneId::AbilityHaste; NUMBER_OF_DAMAGING_RUNES];
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_RUNES {
        let rune = RUNE_CACHE[i];
        if !rune.undeclared {
            result[j] = unsafe { RuneId::from_u8_unchecked(i as _) };
            j += 1;
        }
        i += 1;
    }
    result
};

/// Stores a bit set of all simulated items, very fast for lookup. Damaging items
/// always have at least one of the following:
/// - `ranged.minimum_damage != "zero"`
/// - `ranged.maximum_damage != "zero"`
/// - `melee.minimum_damage != "zero"`
/// - `melee.maximum_damage != "zero"`
/// Note that comparing the name of two functions and checking if they're equal to each
/// other is still unstable, so the comparison `lhs == zero` does not work
#[cfg(feature = "eval")]
pub const DAMAGING_ITEMS: ItemsBitSet = bitset_items(SIMULATED_ITEMS_ENUM);

#[cfg(feature = "eval")]
pub const DAMAGING_RUNES: RunesBitSet = bitset_runes(DAMAGING_RUNES_ARRAY);

#[cfg(feature = "eval")]
/// How many champions we have in the game in the current patch
pub const NUMBER_OF_CHAMPIONS: usize = ChampionId::VARIANTS;

/// How many items are there in the current patch for the map `SummonersRift`, defined
/// by [`GameMap::SummonersRift`]
#[cfg(feature = "eval")]
pub const NUMBER_OF_ITEMS: usize = ItemId::VARIANTS;

/// How many runes we have currently available in the standard gamemode `SummonersRift`,
/// defined by [`GameMap::SummonersRift`]
#[cfg(feature = "eval")]
pub const NUMBER_OF_RUNES: usize = RuneId::VARIANTS;

/// Counts how many damaging abilities ewe have across all champions. This is used to
/// determine a proper size of how many abilities we should allow to live in the stack
/// before leaking it to the heap to avoid stack overflows
#[cfg(feature = "eval")]
pub const NUMBER_OF_ABILITIES: usize = {
    let mut i = 0;
    let mut sum = 0;
    while i < NUMBER_OF_CHAMPIONS {
        let data = CHAMPION_CACHE[i];
        sum += data.closures.len();
        i += 1;
    }
    sum
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
                        Self::[<from_ $repr>](value as _).ok_or("Index out of bounds")
                    }
                }
            )+
        }
    };
    ($name:ident, $repr:ident) => {
        pastey::paste! {
            const_methods!(inner $name, $repr, u16, u32, u64, u128, usize);

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

            #[cfg(feature = "eval")]
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

                #[cfg(feature = "eval")]
                pub const fn get_cache(&self) -> &'static [<Cached $name:replace("Id", "")>] {
                    [<$name:replace("Id", ""):upper _CACHE>][self.index()]
                }

                #[cfg(feature = "eval")]
                pub const fn name(&self) -> &'static str {
                    self.get_cache().name
                }

                #[cfg(all(not(feature = "eval"), feature = "glob"))]
                pub const fn name(&self) -> &'static str {
                    [<$name:replace("Id", ""):upper _ID_TO_NAME>][self.index()]
                }

                pub const fn index(&self) -> usize {
                    *self as _
                }

                pub fn [<is_ $name:snake>](value: &core::any::TypeId) -> bool {
                    *value == core::any::TypeId::of::<$name>()
                }
            }
        }
    };
}

const_methods!(ChampionId, u8);
const_methods!(ItemId, u16);
const_methods!(RuneId, u8);

impl ChampionId {
    /// Counts how many damaging abilities a champion has
    #[cfg(feature = "eval")]
    pub const fn number_of_abilities(&self) -> usize {
        self.get_cache().closures.len()
    }

    #[cfg(feature = "glob")]
    pub const fn recommended_items(
        champion_id: ChampionId,
        position: Position,
    ) -> &'static [ItemId] {
        RECOMMENDED_ITEMS[champion_id as usize][position as usize]
    }

    #[cfg(feature = "glob")]
    pub const fn recommended_runes(
        champion_id: ChampionId,
        position: Position,
    ) -> &'static [RuneId] {
        RECOMMENDED_RUNES[champion_id as usize][position as usize]
    }
}

macro_rules! riot_id_array {
    ($($enum:ty),*) => {
        $(
            impl $enum {
                #[cfg(feature = "eval")]
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
