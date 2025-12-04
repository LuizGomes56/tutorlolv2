use crate::{NUMBER_OF_CHAMPIONS, NUMBER_OF_ITEMS, NUMBER_OF_RUNES};
use tutorlolv2_gen::{ChampionId, ItemId, RuneId};

pub const BITSET_SIZE: usize = {
    let mut result = NUMBER_OF_CHAMPIONS;
    if NUMBER_OF_ITEMS > result {
        result = NUMBER_OF_ITEMS;
    } else if NUMBER_OF_RUNES > result {
        result = NUMBER_OF_RUNES;
    }
    result.div_ceil(u64::BITS as usize)
};

pub type BitArray = const_sized_bit_set::BitSetArray<BITSET_SIZE>;

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

pub const fn bit_array_items<const N: usize>(values: [ItemId; N]) -> BitArray {
    let mut array = BitArray::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}

pub const fn bit_array_champions<const N: usize>(values: [ChampionId; N]) -> BitArray {
    let mut array = BitArray::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}

pub const fn bit_array_runes<const N: usize>(values: [RuneId; N]) -> BitArray {
    let mut array = BitArray::EMPTY;
    let mut i = 0;
    while i < N {
        array.insert(values[i] as usize);
        i += 1;
    }
    array
}
