use crate::model::base::Stats;
use smallvec::SmallVec;
use tutorlolv2_gen::{AdaptativeType, CachedChampionStats};

pub struct MonsterResists {
    pub zero: (i8, i8),
    pub super_minion: (i8, i8),
    pub dragon: (i8, i8),
    pub baron: (i8, i8),
    pub atakhan: (i8, i8),
    pub jungle_camp_1: (i8, i8),
    pub jungle_camp_2: (i8, i8),
}

impl MonsterResists {
    #[inline]
    pub const fn iter(&self) -> [(i8, i8); 7] {
        [
            self.zero,
            self.super_minion,
            self.dragon,
            self.baron,
            self.atakhan,
            self.jungle_camp_1,
            self.jungle_camp_2,
        ]
    }
}

pub const MONSTER_RESISTS: MonsterResists = MonsterResists {
    zero: (0, 0),
    dragon: (21, 30),
    baron: (120, 70),
    atakhan: (90, 75),
    super_minion: (100, -30),
    jungle_camp_1: (42, 42),
    jungle_camp_2: (20, 20),
};

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
    pub fn percent_value<const N: usize>(from_vec: SmallVec<[f32; N]>) -> f32 {
        from_vec
            .iter()
            .map(|value: &f32| 100.0 - value)
            .product::<f32>()
            / 10f32.powi((from_vec.len() << 1) as i32)
    }

    #[inline]
    pub const fn real_resist(percent_pen: f32, flat_pen: f32, resist: f32) -> (f32, f32) {
        let real_val = (percent_pen * resist - flat_pen).max(0.0);
        let modf_val = 100.0 / (100.0 + real_val);
        (real_val, modf_val)
    }

    #[inline]
    pub const fn adaptative_type(attack_damage: f32, ability_power: f32) -> AdaptativeType {
        if 0.35 * attack_damage >= 0.2 * ability_power {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }

    #[inline]
    pub const fn full_base_stats(cdn: &CachedChampionStats, level: u8) -> Stats<f32> {
        macro_rules! assign_value {
            ($field:ident) => {
                Self::stat_growth(cdn.$field.flat, cdn.$field.per_level, level)
            };
        }

        Stats {
            ability_power: 0.0,
            armor: assign_value!(armor),
            armor_penetration_flat: 0.0,
            armor_penetration_percent: 0.0,
            attack_damage: assign_value!(attack_damage),
            attack_range: cdn.attack_range,
            attack_speed: assign_value!(attack_speed),
            crit_chance: 0.0,
            crit_damage: cdn.critical_strike_damage * cdn.critical_strike_damage_modifier,
            current_health: assign_value!(health),
            max_health: assign_value!(health),
            current_mana: assign_value!(mana),
            max_mana: assign_value!(mana),
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: assign_value!(magic_resist),
        }
    }
}
