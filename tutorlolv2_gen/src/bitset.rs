use crate::{DAMAGING_ITEMS_BITSET_SIZE, DAMAGING_RUNES_BITSET_SIZE, ItemId, RuneId};

pub type ItemsBitSet = const_sized_bit_set::BitSetArray<{ DAMAGING_ITEMS_BITSET_SIZE }>;
pub type RunesBitSet = const_sized_bit_set::BitSetArray<{ DAMAGING_RUNES_BITSET_SIZE }>;

/// Same as method [`const_sized_bit_set::BitSetArray::pop`], but with the `const` qualifier
pub const fn bit_array_pop<const S: usize>(array: &mut [u64; S]) -> Option<usize> {
    let mut word_index = 0;
    while word_index <= S - 1 {
        let word = array[word_index];
        if word != 0 {
            let tz = word.trailing_zeros();
            let r = tz as usize + (word_index * u64::BITS as usize);
            let t = word & (0u64.wrapping_sub(word));
            array[word_index] ^= t;

            return Some(r);
        }
        word_index += 1;
    }
    None
}

/// Creates a new [`BitSet`] from a known size array of [`ItemId`]
pub const fn bitset_items<const N: usize>(values: [ItemId; N]) -> ItemsBitSet {
    let mut array = ItemsBitSet::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}

/// Creates a new [`BitSet`] from a known size array of [`RuneId`]
pub const fn bitset_runes<const N: usize>(values: [RuneId; N]) -> RunesBitSet {
    let mut array = RunesBitSet::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}
