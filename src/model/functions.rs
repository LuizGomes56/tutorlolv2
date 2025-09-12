use crate::model::{
    base::{Attacks, DragonMultipliers, InstanceDamage, MonsterExpr},
    realtime::{SimulatedDamages, Team},
};
use bincode::{Encode, enc::Encoder, error::EncodeError};
use internal_comptime::{
    AbilityLike, AdaptativeType, ChampionId, DamageType, ItemId, Position, RuneId,
};
use smallvec::SmallVec;
use tinyset::SetU32;

pub struct WrapSetU32(pub SetU32);

#[macro_export]
macro_rules! enc {
    ($v:expr) => {
        bincode::encode_to_vec(&$v, bincode::config::standard()).unwrap()
    };
}

impl From<SetU32> for WrapSetU32 {
    #[inline(always)]
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
fn find_size(v: u32) -> usize {
    if v <= 250 {
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

impl Sizer for i32 {
    #[inline(always)]
    fn size(&self) -> usize {
        let v = *self;
        let u = ((v << 1) ^ (v >> 31)) as u32;
        if u < 251 {
            1
        } else if u < (1 << 16) {
            3
        } else {
            5
        }
    }
}

macro_rules! impl_sizer {
    ($($t:ty),*) => {
        $(
            impl Sizer for $t {
                #[inline(always)]
                fn size(&self) -> usize {
                    find_size(*self as u32)
                }
            }
        )+
    };
    (const($value:literal) $($t:ty),*) => {
        $(
            impl Sizer for $t {
                #[inline(always)]
                fn size(&self) -> usize {
                    $value
                }
            }
        )+
    };
}

impl<const N: usize> Sizer for SmallVec<[(ItemId, SimulatedDamages); N]> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.len().size()
            + self
                .iter()
                .map(|(k, v)| {
                    k.size()
                        + v.attacks.size()
                        + v.abilities.size()
                        + v.items.size()
                        + v.runes.size()
                })
                .sum::<usize>()
    }
}

impl_sizer!(u16, u32, u64, usize, ItemId);
impl_sizer!(const(1) ChampionId, RuneId, u8, Position, Team, bool, DamageType, AdaptativeType);
impl_sizer!(const(2) AbilityLike);
impl_sizer!(const(4) f32);

impl<T: Sizer> Sizer for Attacks<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.basic_attack.minimum_damage.size()
            + self.basic_attack.maximum_damage.size()
            + self.critical_strike.minimum_damage.size()
            + self.critical_strike.maximum_damage.size()
            + self.onhit_damage.minimum_damage.size()
            + self.onhit_damage.maximum_damage.size()
    }
}

impl<T: Sizer> Sizer for InstanceDamage<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.damage_type.size() + self.maximum_damage.size() + self.minimum_damage.size()
    }
}

impl<const N: usize, T: Sizer, U: Sizer> Sizer for SmallVec<[(T, InstanceDamage<U>); N]> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.len().size()
            + self
                .iter()
                .map(|(key, val)| key.size() + val.size())
                .sum::<usize>()
    }
}

impl<const N: usize, T: Sizer> Sizer for SmallVec<[InstanceDamage<T>; N]> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.len().size() + self.iter().map(|val| val.size()).sum::<usize>()
    }
}

impl Sizer for WrapSetU32 {
    #[inline(always)]
    fn size(&self) -> usize {
        self.0.len().size() + self.0.iter().map(|x| x.size()).sum::<usize>()
    }
}

impl Sizer for str {
    #[inline(always)]
    fn size(&self) -> usize {
        let len = self.len();
        len + find_size(len as u32)
    }
}

impl<const N: usize, T: Sizer> Sizer for [MonsterExpr<T>; N] {
    #[inline(always)]
    fn size(&self) -> usize {
        self.iter()
            .map(|expr| expr.abilities.size() + expr.attacks.size() + expr.items.size())
            .sum::<usize>()
    }
}

impl<const N: usize, T: Sizer> Sizer for [T; N] {
    #[inline(always)]
    fn size(&self) -> usize {
        self.iter().map(|x| x.size()).sum::<usize>()
    }
}

impl Sizer for DragonMultipliers {
    #[inline(always)]
    fn size(&self) -> usize {
        size_of::<Self>()
    }
}
