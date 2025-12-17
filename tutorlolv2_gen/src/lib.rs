#![no_std]
pub mod bitset;
pub mod cache;
pub mod data;
pub mod enums;
pub mod eval;

pub use bitset::*;
pub use cache::*;
pub use data::*;
pub use enums::*;
pub use eval::*;

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

    tier >= 3 && price > 0 && purchasable && prettified_stats.len() > 0
}

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
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

/// Stores the simulated items as [`ItemId`], and only those that follow the rules:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
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
pub const DAMAGING_ITEMS: ItemsBitSet = bitset_items(SIMULATED_ITEMS_ENUM);
pub const DAMAGING_RUNES: RunesBitSet = bitset_runes(DAMAGING_RUNES_ARRAY);

/// How many champions we have in the game in the current patch
pub const NUMBER_OF_CHAMPIONS: usize = ChampionId::VARIANTS;

/// How many items are there in the current patch for the map `SummonersRift`, defined
/// by [`GameMap::SummonersRift`]
pub const NUMBER_OF_ITEMS: usize = ItemId::VARIANTS;

/// How many runes we have currently available in the standard gamemode `SummonersRift`,
/// defined by [`GameMap::SummonersRift`]
pub const NUMBER_OF_RUNES: usize = RuneId::VARIANTS;

/// Counts how many damaging abilities ewe have across all champions. This is used to
/// determine a proper size of how many abilities we should allow to live in the stack
/// before leaking it to the heap to avoid stack overflows
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
    ($name:ident, $repr:ident) => {
        pastey::paste! {
            impl $name {
                pub const VARIANTS: usize = [<$name:replace("Id", ""):upper _CACHE>].len();
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
                    [<$name:replace("Id", ""):upper _CACHE>][self.offset()]
                }

                pub const fn name(&self) -> &'static str {
                    self.get_cache().name
                }

                pub const fn offset(&self) -> usize {
                    *self as _
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
    pub const fn number_of_abilities(&self) -> usize {
        self.get_cache().closures.len()
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
