pub mod bitarray;
pub mod cache;
pub mod data;
pub mod enums;
pub mod eval;

pub use bitarray::*;
pub use cache::*;
pub use data::*;
pub use enums::*;
pub use eval::*;

pub const NUMBER_OF_SIMULATED_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_ITEMS {
        let item = INTERNAL_ITEMS[i];
        if item.is_simulated {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// Stores the `riot_id` value for items that meet the following requirements:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
pub const SIMULATED_ITEMS: [u32; NUMBER_OF_SIMULATED_ITEMS] = {
    let mut result = [0; NUMBER_OF_SIMULATED_ITEMS];
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_ITEMS {
        let item = INTERNAL_ITEMS[i];
        if item.is_simulated {
            result[j] = item.riot_id;
            j += 1;
        }
        i += 1;
    }
    result
};

/// Stores the simulated items as `ItemId` instead of `riot_id`. They're identical and can be
/// coerced to each other in constant time
pub const SIMULATED_ITEMS_ENUM: [ItemId; NUMBER_OF_SIMULATED_ITEMS] = {
    let mut result = [ItemId::AbyssalMask; NUMBER_OF_SIMULATED_ITEMS];
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_ITEMS {
        let item = INTERNAL_ITEMS[i];
        if item.is_simulated {
            result[j] = unsafe { ItemId::from_u16_unchecked(j as _) };
            j += 1;
        }
        i += 1;
    }
    result
};

pub const NUMBER_OF_DAMAGING_RUNES: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < NUMBER_OF_RUNES {
        let rune = INTERNAL_RUNES[i];
        if !rune.undeclared {
            sum += 1;
        }
        i += 1;
    }
    sum
};

pub const DAMAGING_RUNES_ARRAY: [RuneId; NUMBER_OF_DAMAGING_RUNES] = {
    let mut result = [RuneId::AbilityHaste; NUMBER_OF_DAMAGING_RUNES];
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_RUNES {
        let rune = INTERNAL_RUNES[i];
        if !rune.undeclared {
            result[j] = unsafe { RuneId::from_u8_unchecked(i as _) };
            j += 1;
        }
        i += 1;
    }
    result
};

pub const DAMAGING_ITEMS_BITSET_SIZE: usize = NUMBER_OF_SIMULATED_ITEMS.div_ceil(64);
/// Stores a bit set of all simulated items, very fast for lookup. Damaging items
/// always have at least one of the following:
/// - `ranged.minimum_damage != "zero"`
/// - `ranged.maximum_damage != "zero"`
/// - `melee.minimum_damage != "zero"`
/// - `melee.maximum_damage != "zero"`
/// Note that comparing the name of two functions and checking if they're equal to each
/// other is still unstable, so the comparison `lhs == zero` does not work
pub const DAMAGING_ITEMS: BitArray<DAMAGING_ITEMS_BITSET_SIZE> =
    bit_array_items(SIMULATED_ITEMS_ENUM);

pub const DAMAGING_RUNES_BITSET_SIZE: usize = NUMBER_OF_DAMAGING_RUNES.div_ceil(64);
pub const DAMAGING_RUNES: BitArray<DAMAGING_RUNES_BITSET_SIZE> =
    bit_array_runes(DAMAGING_RUNES_ARRAY);

pub const NUMBER_OF_CHAMPIONS: usize = INTERNAL_CHAMPIONS.len();
pub const NUMBER_OF_ITEMS: usize = INTERNAL_ITEMS.len();
pub const NUMBER_OF_RUNES: usize = INTERNAL_RUNES.len();
pub const NUMBER_OF_ABILITIES: usize = {
    let mut i = 0;
    let mut sum = 0;
    while i < NUMBER_OF_CHAMPIONS {
        let data = INTERNAL_CHAMPIONS[i];
        sum += data.closures.len();
        i += 1;
    }
    sum
};

impl ItemId {
    pub const fn to_riot_id(&self) -> u32 {
        INTERNAL_ITEMS[*self as usize].riot_id
    }
}

impl RuneId {
    pub const fn to_riot_id(&self) -> u32 {
        INTERNAL_RUNES[*self as usize].riot_id
    }
}
