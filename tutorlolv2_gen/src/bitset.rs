use crate::{ItemId, RuneId};
pub use const_sized_bit_set::BitSetArray;

pub const fn max_usize(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

pub const fn bitset_size<const N: usize>(array: [usize; N]) -> usize {
    let mut i = 0;
    let mut max = 0;
    while i < N {
        max = max_usize(max, array[i]);
        i += 1;
    }
    sizeof_bitset(max)
}

pub const fn sizeof_bitset(n: usize) -> usize {
    n.div_ceil(64)
}

pub type ItemsBitSet = BitSetArray<{ sizeof_bitset(ItemId::VARIANTS) }>;
pub type RunesBitSet = BitSetArray<{ sizeof_bitset(RuneId::VARIANTS) }>;

pub type ItemsExcSet = BitSetArray<{ ItemId::SIZE_OF_EXCEPTIONS }>;
pub type RunesExcSet = BitSetArray<{ RuneId::SIZE_OF_EXCEPTIONS }>;

/// Same as method [`BitSetArray::pop`], but with the `const` qualifier
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

pub const fn make_bitset<const N: usize, const L: usize>(values: [usize; L]) -> BitSetArray<N> {
    let mut array = BitSetArray::EMPTY;
    let mut i = 0;
    while i < L {
        array.insert(values[i]);
        i += 1;
    }
    array
}

#[macro_export]
macro_rules! bitset {
    ([$($value:expr),*$(,)*]) => {
        const { make_bitset::<_, L>([$($value.index()),*]) }
    };
    ($array:expr => [$ty:ty]) => {
        const {
            const LEN: usize = $array.len();
            let mut array = [0 as $ty; LEN];
            let mut i = 0;
            while i < LEN {
                array[i] = $array[i].index();
                i += 1;
            }
            array
        }
    };
    ($array:expr) => {
        const {
            let cast = $crate::bitset!($array => [usize]);
            make_bitset(cast)
        }
    };
}
