use super::*;

/// Bonus attack speed as a ratio (for example, `0.5` for 50%).
///
/// IEEE 754 binary16 storage fits in the evaluation context's existing padding.
/// Values round to the nearest representable value, with ties to even. The
/// relative rounding error for normal values is at most 1/2048.
#[derive(
    Clone, Copy, Debug, Default, Decode, Deserialize, Encode, PartialEq, PartialOrd, Serialize,
)]
#[repr(transparent)]
#[serde(from = "f32", into = "f32")]
pub struct AttackSpeedRatio(u16);

impl AttackSpeedRatio {
    pub const fn new(value: f32) -> Self {
        let bits = value.to_bits();
        let sign = ((bits >> 16) & 0x8000) as u16;
        let exponent = ((bits >> 23) & 0xff) as i32;
        let fraction = bits & 0x7f_ffff;
        if exponent == 255 {
            // Keep NaNs distinct from infinity even when their payload is tiny.
            return Self(
                sign | 0x7c00
                    | if fraction == 0 {
                        0
                    } else {
                        ((fraction >> 13) as u16) | 0x0200
                    },
            );
        }
        let half_exponent = exponent - 127 + 15;
        if half_exponent >= 31 {
            return Self(sign | 0x7c00);
        }
        if half_exponent < -10 {
            return Self(sign);
        }
        let (significand, shift) = if half_exponent <= 0 {
            (fraction | 0x80_0000, (14 - half_exponent) as u32)
        } else {
            (fraction, 13)
        };
        let truncated = significand >> shift;
        let remainder = significand & ((1 << shift) - 1);
        let midpoint = 1 << (shift - 1);
        let rounded = truncated
            + (remainder > midpoint || (remainder == midpoint && truncated & 1 != 0)) as u32;
        // Addition carries into the exponent at a rounded mantissa boundary.
        let magnitude = if half_exponent <= 0 {
            rounded
        } else {
            ((half_exponent as u32) << 10) + rounded
        };
        Self(sign | magnitude as u16)
    }

    pub const fn get(self) -> f32 {
        let sign = ((self.0 & 0x8000) as u32) << 16;
        let exponent = (self.0 >> 10) & 0x1f;
        let mut fraction = (self.0 & 0x03ff) as u32;
        let magnitude = if exponent == 0 {
            if fraction == 0 {
                0
            } else {
                let mut exponent = 113_u32;
                while fraction & 0x0400 == 0 {
                    fraction <<= 1;
                    exponent -= 1;
                }
                (exponent << 23) | ((fraction & 0x03ff) << 13)
            }
        } else if exponent == 31 {
            0x7f80_0000 | (fraction << 13)
        } else {
            (((exponent as u32) + 112) << 23) | (fraction << 13)
        };
        f32::from_bits(sign | magnitude)
    }
}

impl From<f32> for AttackSpeedRatio {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<AttackSpeedRatio> for f32 {
    fn from(value: AttackSpeedRatio) -> Self {
        value.get()
    }
}
