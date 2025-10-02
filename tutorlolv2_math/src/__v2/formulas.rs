use super::model::*;
use tutorlolv2_gen::AdaptativeType;

pub struct RiotFormulas;

impl RiotFormulas {
    /// Uses wiki's formula to return base stats for a given champion
    #[inline]
    pub const fn stat_growth(base: f32, growth_per_level: f32, level: u8) -> f32 {
        base + growth_per_level * (level as f32 - 1.0) * (0.7025 + 0.0175 * (level as f32 - 1.0))
    }
    /// Percentage values are entered in this section as a number in range 0-100
    ///
    /// 30% and 30% penetration should yield 49% penetration (0.51 true value)
    ///
    /// ```
    /// for x in vec yield x / 10.pow(len(x) << 1)
    /// let from_vec = [30, 30];
    /// return 0.51
    ///
    /// ```
    pub fn percent_value<const N: usize>(from_vec: [f32; N]) -> f32 {
        from_vec
            .iter()
            .map(|value: &f32| 100.0 - value)
            .product::<f32>()
            / 10f32.powi((from_vec.len() << 1) as i32)
    }

    #[inline]
    pub const fn real_resist(percent_pen: f32, flat_pen: f32, resist: f32) -> ResistValue {
        let real_val = (percent_pen * resist - flat_pen).max(0.0);
        let modf_val = 100.0 / (100.0 + real_val);
        ResistValue {
            real: real_val,
            modifier: modf_val,
        }
    }

    #[inline]
    pub const fn adaptative_type(attack_damage: f32, ability_power: f32) -> AdaptativeType {
        if 0.35 * attack_damage >= 0.2 * ability_power {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }
}

#[inline(always)]
pub const fn size_i(v: i32) -> usize {
    let u = ((v << 1) ^ (v >> 31)) as u32;
    if u < 251 {
        1
    } else if u < (1 << 16) {
        3
    } else {
        5
    }
}

macro_rules! impl_size_counter {
    ($($ty:ty),*) => {
        $(
            impl SizeCounter for $ty {
                #[inline(always)]
                fn size(&self) -> usize {
                    size_u(*self as u32)
                }
            }
        )*
    };
}

impl_size_counter!(u32, usize);

#[inline(always)]
pub const fn size_u(v: u32) -> usize {
    if v <= 250 {
        1
    } else if v <= 0xFFFF {
        3
    } else {
        5
    }
}

pub trait SizeCounter {
    fn size(&self) -> usize;
}

impl SizeCounter for i32 {
    #[inline(always)]
    fn size(&self) -> usize {
        size_i(*self)
    }
}

impl SizeCounter for SimpleStatsI32 {
    #[inline(always)]
    fn size(&self) -> usize {
        self.armor.size() + self.health.size() + self.magic_resist.size()
    }
}

impl SizeCounter for str {
    #[inline(always)]
    fn size(&self) -> usize {
        let len = self.len();
        len + (len as u32).size()
    }
}
