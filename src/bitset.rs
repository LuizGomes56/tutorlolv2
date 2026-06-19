use crate::{ChampionId, ItemId, RuneId};
pub use const_sized_bit_set::{self, prelude::BitSetArray};

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

pub const BITSET_SIZE: usize = max_usize(
    ChampionId::VARIANTS,
    max_usize(ItemId::VARIANTS, RuneId::VARIANTS),
);

pub type BitSet = BitSetArray<{ sizeof_bitset(BITSET_SIZE) }>;

pub const BITSET_EXC_SIZE: usize = {
    let ie = max_usize(
        bitset_size(crate::bitset!(ItemId::ALLY_EXCEPTIONS => [usize])),
        bitset_size(crate::bitset!(ItemId::ENEMY_EXCEPTIONS => [usize])),
    );
    let re = bitset_size(crate::bitset!(RuneId::EXCEPTIONS => [usize]));
    max_usize(ie, re)
};

pub type BitSetExc = BitSetArray<{ BITSET_EXC_SIZE }>;

pub const fn make_bitset<const N: usize, const L: usize>(values: [u32; L]) -> BitSetArray<N> {
    let mut array = BitSetArray::EMPTY;
    let mut i = 0;
    while i < L {
        array.insert_const(values[i]);
        i += 1;
    }
    array
}

pub const fn unmake_bitset<const N: usize, const L: usize>(bitset: BitSetArray<N>) -> [u32; L] {
    let mut result = [0; L];

    let mut iter = bitset.iter_const();
    let mut i = 0;

    while let Some(value) = iter.next_const() {
        result[i] = value;
        i += 1;
    }

    result
}

#[macro_export]
macro_rules! bitset {
    ([$($value:expr),*$(,)*]) => {
        const { $crate::make_bitset([$($value.index() as _),*]) }
    };
    ($bitset:expr => [$ty:ty] => [$enum:ty]) => {
        const {
            const LEN: usize = $bitset.count_const() as usize;
            let array = $crate::unmake_bitset::<_, LEN>($bitset);
            let mut i = 0;
            let mut result: [$enum; LEN] = unsafe { core::mem::zeroed() };
            while i < array.len() {
                unsafe {
                    result[i] = <$enum>::from_u32(array[i]).unwrap();
                }
                i += 1;
            }
            result
        }
    };
    ($array:expr => [$ty:ty]) => {
        const {
            const LEN: usize = $array.len();
            let mut array = [0 as $ty; LEN];
            let mut i = 0;
            while i < LEN {
                array[i] = $array[i] as $ty;
                i += 1;
            }
            array
        }
    };
    ($array:expr) => {
        const {
            let cast = $crate::bitset!($array => [u32]);
            $crate::bitset::make_bitset(cast)
        }
    };
}
