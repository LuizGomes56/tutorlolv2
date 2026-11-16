//! Unsigned fixed-point numbers in two bytes, see [`Fixed`]
use super::*;

/// A non-negative number in 16 bits, `FRAC` of them for the fraction: it holds
/// `0` up to `65535 / 2^FRAC` in steps of `1 / 2^FRAC`. Values out of range
/// saturate, and NaN reads as `0`. Fields of [`Ctx`] that never need more use it
/// to keep the context in two cache lines
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Decode,
    Deserialize,
    Encode,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(from = "f32", into = "f32")]
#[repr(transparent)]
pub struct Fixed<const FRAC: u32>(u16);

/// Percentages from `0` to `127.998`, in steps of `1/512`: penetrations and
/// critical strike chance
pub type Percent = Fixed<9>;

/// Up to `511.99`, in steps of `1/128`: critical strike damage, about `175` to `300`
pub type CritDamage = Fixed<7>;

impl<const FRAC: u32> Fixed<FRAC> {
    const SCALE: f32 = (1u32 << FRAC) as f32;

    /// Largest value it holds
    pub const MAX: f32 = u16::MAX as f32 / Self::SCALE;

    /// Rounds `value` to the nearest step
    pub const fn new(value: f32) -> Self {
        // `as` saturates and reads NaN as 0
        Self((value * Self::SCALE + 0.5) as u16)
    }

    pub const fn get(self) -> f32 {
        self.0 as f32 / Self::SCALE
    }

    pub const fn to_bits(self) -> u16 {
        self.0
    }

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }
}

impl<const FRAC: u32> From<f32> for Fixed<FRAC> {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl<const FRAC: u32> From<Fixed<FRAC>> for f32 {
    fn from(value: Fixed<FRAC>) -> Self {
        value.get()
    }
}
