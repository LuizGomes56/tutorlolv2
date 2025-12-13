use crate::{ChampionId, ItemId, NUMBER_OF_CHAMPIONS, NUMBER_OF_ITEMS, NUMBER_OF_RUNES, RuneId};

/// Compares the number of champions, runes, and items and picks the largest one to
/// determine the appropriate size of the bitsets that the application will use to
/// fast check if some item or rune deals damage or not
pub const BITSET_SIZE: usize = {
    let mut result = NUMBER_OF_CHAMPIONS;
    if NUMBER_OF_ITEMS > result {
        result = NUMBER_OF_ITEMS;
    } else if NUMBER_OF_RUNES > result {
        result = NUMBER_OF_RUNES;
    }
    result.div_ceil(u64::BITS as usize)
};

pub type BitSet<const N: usize = BITSET_SIZE> = const_sized_bit_set::BitSetArray<N>;

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
pub const fn bitset_items<const N: usize, const R: usize>(values: [ItemId; N]) -> BitSet<R> {
    let mut array = BitSet::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}

/// Creates a new [`BitSet`] from a known size array of [`ChampionId`]
pub const fn bitset_champions<const N: usize, const R: usize>(
    values: [ChampionId; N],
) -> BitSet<R> {
    let mut array = BitSet::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}

/// Creates a new [`BitSet`] from a known size array of [`RuneId`]
pub const fn bitset_runes<const N: usize, const R: usize>(values: [RuneId; N]) -> BitSet<R> {
    let mut array = BitSet::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}
