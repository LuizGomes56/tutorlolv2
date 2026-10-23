#![allow(unreachable_patterns)]

pub mod check;
pub mod impls;
pub mod model;

use {
    bincode::{Decode, Encode},
    core::fmt::Debug,
    serde::{Deserialize, Serialize},
    strum::{EnumCount, FromRepr, VariantArray, VariantNames},
    tutorlolv2_types::{
        AbilityId::{self, *},
        AbilityName::*,
        AdaptiveType,
        AttackType::{self, *},
        Attrs::*,
        Ctx,
        DamageType::*,
        GameMap::*,
        Position,
        Position::*,
        StatName, TypeMetadata,
    },
};

pub use {
    champions_code::ChampionId,
    impls::traits::{CastId, ValueId},
    items_code::ItemId,
    model::*,
    runes_code::RuneId,
};

#[cfg(feature = "yew")]
pub(self) use tutorlolv2_types::{ComboElement::*, CtxVar::*, MergeData};

pub mod champions_code {
    use super::{champions::*, *};
    include!(concat!(env!("OUT_DIR"), "/champions_code.rs"));
}

pub mod champions {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/champions.rs"));
    type X = Champion;
}

pub mod items_code {
    use super::{items::*, *};
    include!(concat!(env!("OUT_DIR"), "/items_code.rs"));
}

pub mod items {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/items.rs"));
    type X = Item;
}

pub mod runes_code {
    use super::{runes::*, *};
    include!(concat!(env!("OUT_DIR"), "/runes_code.rs"));
}

pub mod runes {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/runes.rs"));
    type X = Rune;
}

/// Encodes each id as its `repr` primitive. The bincode derive ignores `repr` and emits
/// one match arm per variant, which is hundreds of arms per encoder/decoder instance.
/// The wire format is unchanged under varint encoding while every variant index is
/// below 251 for `u8` ids, since the derive writes the index as a varint `u32`.
macro_rules! impl_repr_codec {
    ($($ty:ty => $repr:ty),* $(,)?) => {$(
        impl Encode for $ty {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                (*self as $repr).encode(encoder)
            }
        }

        impl<Context> Decode<Context> for $ty {
            fn decode<D: bincode::de::Decoder<Context = Context>>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let repr = <$repr as Decode<Context>>::decode(decoder)?;

                Self::from_repr(repr).ok_or(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: stringify!($ty),
                    allowed: const {
                        &bincode::error::AllowedEnumVariants::Range {
                            min: 0,
                            max: <$ty>::COUNT as u32 - 1,
                        }
                    },
                    found: repr as u32,
                })
            }
        }

        bincode::impl_borrow_decode!($ty);
    )*};
}

impl_repr_codec!(ChampionId => u8, ItemId => u16, RuneId => u8);

pub const fn zero(_: &Ctx) -> f32 {
    0.0
}

pub const fn ignite(level: u8) -> i32 {
    let n = level as i32;
    let nth = if n > 4 { n - 4 } else { 0 };
    70 + 20 * n + 5 * nth
}

pub const L_MSTR: usize = 7;
pub const L_TWRD: usize = 6;
