use crate::model::{
    base::Stats,
    cache::{AdaptativeType, CachedChampionStats},
};
use smallvec::SmallVec;

pub struct MonsterResists {
    pub zero: (i8, i8),
    pub dragon: (i8, i8),
    pub baron: (i8, i8),
    pub atakhan: (i8, i8),
    pub super_minion: (i8, i8),
    pub jungle_camp_1: (i8, i8),
    pub jungle_camp_2: (i8, i8),
}

impl MonsterResists {
    pub fn iter_enumerate(&self) -> [(u8, (i8, i8)); 7] {
        [
            (0, self.zero),
            (1, self.dragon),
            (2, self.baron),
            (3, self.atakhan),
            (4, self.super_minion),
            (5, self.jungle_camp_1),
            (6, self.jungle_camp_2),
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
    pub const fn stat_growth(base: f64, growth_per_level: f64, level: u8) -> f64 {
        base + growth_per_level * (level as f64 - 1.0) * (0.7025 + 0.0175 * (level as f64 - 1.0))
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
    pub fn percent_value<const N: usize>(from_vec: SmallVec<[f64; N]>) -> f64 {
        from_vec
            .iter()
            .map(|value: &f64| 100.0 - value)
            .product::<f64>()
            / 10f64.powi((from_vec.len() << 1) as i32)
    }

    pub const fn adaptative_type(attack_damage: f64, ability_power: f64) -> AdaptativeType {
        if 0.35 * attack_damage >= 0.2 * ability_power {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }

    pub const fn full_base_stats(cdn: &CachedChampionStats, level: u8) -> Stats {
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
            attack_range: assign_value!(attack_range),
            attack_speed: assign_value!(attack_speed),
            crit_chance: 0.0,
            crit_damage: assign_value!(critical_strike_damage)
                * cdn.critical_strike_damage_modifier.flat,
            current_health: assign_value!(health),
            max_health: assign_value!(health),
            current_mana: assign_value!(mana),
            max_mana: assign_value!(mana),
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: assign_value!(magic_resistance),
        }
    }
}
