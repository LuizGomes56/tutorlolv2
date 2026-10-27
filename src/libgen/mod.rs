#![allow(unreachable_patterns)]

pub mod check;
pub mod globals;
pub mod impls;

pub use {
    champions_code::ChampionId,
    globals::*,
    impls::traits::{CastId, ValueId},
    items_code::ItemId,
    runes_code::RuneId,
};

include!(concat!(env!("OUT_DIR"), "/libgen.rs"));

/// Encodes each id as its `repr` primitive. The bincode derive ignores `repr` and emits
/// one match arm per variant, which is hundreds of arms per encoder/decoder instance.
/// The wire format is unchanged under varint encoding while every variant index is
/// below 251 for `u8` ids, since the derive writes the index as a varint `u32`.
///
/// Serde preserves named unit variants using strum's sorted `VARIANTS` table for
/// serialization and binary-search deserialization instead of per-variant matches.
macro_rules! impl_repr_codec {
    ($($ty:ty => $repr:ty),* $(,)?) => {$(
        impl bincode::Encode for $ty {
            fn encode<E: bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), bincode::error::EncodeError> {
                (*self as $repr).encode(encoder)
            }
        }

        impl<Context> bincode::Decode<Context> for $ty {
            fn decode<D: bincode::de::Decoder<Context = Context>>(
                decoder: &mut D,
            ) -> Result<Self, bincode::error::DecodeError> {
                let repr = <$repr as bincode::Decode<Context>>::decode(decoder)?;

                Self::from_repr(repr).ok_or(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: stringify!($ty),
                    allowed: const {
                        &bincode::error::AllowedEnumVariants::Range {
                            min: 0,
                            max: <$ty as strum::EnumCount>::COUNT as u32 - 1,
                        }
                    },
                    found: repr as u32,
                })
            }
        }

        bincode::impl_borrow_decode!($ty);

        const _: () = assert!(
            is_sorted(<$ty as strum::VariantNames>::VARIANTS),
            concat!(stringify!($ty), "::VARIANTS must be sorted for its serde impl"),
        );

        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let index = *self as usize;
                serializer.serialize_unit_variant(
                    stringify!($ty),
                    index as u32,
                    <Self as strum::VariantNames>::VARIANTS[index],
                )
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use serde::de::{self, EnumAccess, Unexpected, VariantAccess};

                const VARIANTS: &[&str] = <$ty as strum::VariantNames>::VARIANTS;

                struct Variant;
                struct Identifier;

                impl<'de> de::Visitor<'de> for Identifier {
                    type Value = $ty;

                    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                        f.write_str(concat!("a variant identifier of ", stringify!($ty)))
                    }

                    fn visit_u64<E: de::Error>(self, value: u64) -> Result<$ty, E> {
                        <$repr>::try_from(value)
                            .ok()
                            .and_then(<$ty>::from_repr)
                            .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(value), &self))
                    }

                    fn visit_str<E: de::Error>(self, value: &str) -> Result<$ty, E> {
                        VARIANTS
                            .binary_search(&value)
                            .ok()
                            .and_then(|index| <$ty>::from_repr(index as $repr))
                            .ok_or_else(|| E::unknown_variant(value, VARIANTS))
                    }

                    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<$ty, E> {
                        match core::str::from_utf8(value) {
                            Ok(value) => self.visit_str(value),
                            Err(_) => Err(E::invalid_value(Unexpected::Bytes(value), &self)),
                        }
                    }
                }

                impl<'de> de::DeserializeSeed<'de> for Identifier {
                    type Value = $ty;

                    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<$ty, D::Error> {
                        deserializer.deserialize_identifier(self)
                    }
                }

                impl<'de> de::Visitor<'de> for Variant {
                    type Value = $ty;

                    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                        f.write_str(concat!("enum ", stringify!($ty)))
                    }

                    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<$ty, A::Error> {
                        let (id, variant) = data.variant_seed(Identifier)?;
                        variant.unit_variant()?;
                        Ok(id)
                    }
                }

                deserializer.deserialize_enum(stringify!($ty), VARIANTS, Variant)
            }
        }
    )*};
}

impl_repr_codec!(ChampionId => u8, ItemId => u16, RuneId => u8);

/// Whether `names` is strictly ascending in byte order, the order `binary_search` expects.
const fn is_sorted(names: &[&str]) -> bool {
    let mut i = 1;

    while i < names.len() {
        let (a, b) = (names[i - 1].as_bytes(), names[i].as_bytes());
        let mut j = 0;

        while j < a.len() && j < b.len() && a[j] == b[j] {
            j += 1;
        }

        let ascending = match (j < a.len(), j < b.len()) {
            (true, true) => a[j] < b[j],
            (false, true) => true,
            _ => false,
        };

        if !ascending {
            return false;
        }

        i += 1;
    }

    true
}

#[cfg(feature = "engine")]
pub const fn zero(_: &tutorlolv2_types::Ctx) -> f32 {
    0.0
}

pub const fn ignite(level: core::num::NonZeroU8) -> i32 {
    let n = level.get() as i32;
    let nth = if n > 4 { n - 4 } else { 0 };
    70 + 20 * n + 5 * nth
}

pub const L_MSTR: usize = 7;
pub const L_TWRD: usize = 6;
