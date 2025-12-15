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

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
pub const NUMBER_OF_SIMULATED_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_ITEMS {
        let item = ITEM_CACHE[i];
        if item.is_simulated {
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
        let item = ITEM_CACHE[i];
        if item.is_simulated {
            result[j] = unsafe { ItemId::from_u16_unchecked(j as _) };
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
        if !item.is_damaging {
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

/// Calculates the appropriate size of the bitset that will store thte simulated items.
/// A higher value would waste a few bytes of memory
pub const DAMAGING_ITEMS_BITSET_SIZE: usize = NUMBER_OF_DAMAGING_ITEMS.div_ceil(64);

/// Stores a bit set of all simulated items, very fast for lookup. Damaging items
/// always have at least one of the following:
/// - `ranged.minimum_damage != "zero"`
/// - `ranged.maximum_damage != "zero"`
/// - `melee.minimum_damage != "zero"`
/// - `melee.maximum_damage != "zero"`
/// Note that comparing the name of two functions and checking if they're equal to each
/// other is still unstable, so the comparison `lhs == zero` does not work
pub const DAMAGING_ITEMS: ItemsBitSet = bitset_items(SIMULATED_ITEMS_ENUM);

/// Calculation of the appropriate size of the bitset that will store the damaging runes
pub const DAMAGING_RUNES_BITSET_SIZE: usize = NUMBER_OF_DAMAGING_RUNES.div_ceil(64);
pub const DAMAGING_RUNES: RunesBitSet = bitset_runes(DAMAGING_RUNES_ARRAY);

/// How many champions we have in the game in the current patch
pub const NUMBER_OF_CHAMPIONS: usize = CHAMPION_CACHE.len();

/// How many items are there in the current patch for the map `SummonersRift`, defined
/// by [`GameMap::SummonersRift`]
pub const NUMBER_OF_ITEMS: usize = ITEM_CACHE.len();

/// How many runes we have currently available in the standard gamemode `SummonersRift`,
/// defined by [`GameMap::SummonersRift`]
pub const NUMBER_OF_RUNES: usize = RUNE_CACHE.len();

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
