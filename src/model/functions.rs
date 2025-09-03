use crate::model::{base::InstanceDamage, realtime::SimulatedDamages};
use bincode::{Encode, enc::Encoder, error::EncodeError};
use internal_comptime::{AbilityLike, ChampionId, ItemId, RuneId};
use smallvec::SmallVec;
use tinyset::SetU32;

pub struct WrapSetU32(pub SetU32);

#[macro_export]
macro_rules! enc {
    ($v:expr) => {
        bincode::encode_to_vec(&$v, bincode::config::standard()).unwrap()
    };
}

pub use crate::enc;

impl From<SetU32> for WrapSetU32 {
    #[inline]
    fn from(set: SetU32) -> Self {
        WrapSetU32(set)
    }
}

impl Encode for WrapSetU32 {
    fn encode<E: Encoder>(&self, enc: &mut E) -> Result<(), EncodeError> {
        self.0.len().encode(enc)?;
        for x in self.0.iter() {
            x.encode(enc)?;
        }
        Ok(())
    }
}

#[inline(always)]
fn size_v(v: u32) -> usize {
    if v <= 251 {
        1
    } else if v <= 0xFFFF {
        3
    } else {
        5
    }
}

pub trait Sizer {
    fn size(&self) -> usize;
}

macro_rules! impl_sizer {
    ($($t:ty),*) => {
        $(
            impl Sizer for $t {
                fn size(&self) -> usize {
                    size_v(*self as u32)
                }
            }
        )+
    };
    (const $t:ty, $ratio:literal) => {
        impl<const N: usize> Sizer for SmallVec<[($t, InstanceDamage); N]> {
            fn size(&self) -> usize {
                let len = self.len();
                len.size() + len * $ratio
            }
        }
    };
}

impl<const N: usize> Sizer for SmallVec<[(ItemId, InstanceDamage); N]> {
    fn size(&self) -> usize {
        self.len().size() + self.iter().map(|(k, _)| k.size() + 9).sum::<usize>()
    }
}

impl<const N: usize> Sizer for SmallVec<[(ItemId, SimulatedDamages); N]> {
    fn size(&self) -> usize {
        self.len().size()
            + self
                .iter()
                .map(|(k, v)| {
                    debug_assert_eq!(enc!(v.abilities).len(), v.abilities.size());
                    debug_assert_eq!(enc!(v.items).len(), v.items.size());
                    debug_assert_eq!(enc!(v.runes).len(), v.runes.size());
                    debug_assert_eq!(enc!(k).len(), k.size());
                    debug_assert_eq!(enc!(v.attacks).len(), 24);
                    k.size() + 24 + v.abilities.size() + v.items.size() + v.runes.size()
                })
                .sum::<usize>()
    }
}

impl_sizer!(u8, u16, u32, u64, usize, ItemId);
impl_sizer!(const ChampionId, 10);
impl_sizer!(const RuneId, 10);
impl_sizer!(const AbilityLike, 11);

impl Sizer for WrapSetU32 {
    fn size(&self) -> usize {
        self.0.len().size() + self.0.iter().map(|x| x.size()).sum::<usize>()
    }
}

impl Sizer for str {
    fn size(&self) -> usize {
        let len = self.len();
        len + size_v(len as u32)
    }
}
