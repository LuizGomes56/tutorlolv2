use crate::{Monster, model::SimpleStats};

/// How a stat of a [`Monster`] grows during the game, generated from the wiki
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MonsterStat {
    Flat(f32),
    /// Value at each level from `start`. Levels outside of the table are clamped to it
    Levels {
        start: u8,
        values: &'static [f32],
    },
    /// `base + per_upgrade * upgrades`, at most `cap`. Minions gain upgrades over time
    Upgrades {
        base: f32,
        per_upgrade: f32,
        cap: f32,
    },
    /// 0 up to `after` upgrades, then `coefficient * (u - after - 1) / 2 * (u - after)`,
    /// at most `cap`
    TriangularUpgrades {
        coefficient: f32,
        after: u8,
        cap: f32,
    },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MonsterStats {
    pub health: MonsterStat,
    pub armor: MonsterStat,
    pub magic_resist: MonsterStat,
}

impl MonsterStat {
    /// Value of the stat for a monster of `level`
    pub const fn at(self, level: u8) -> f32 {
        match self {
            Self::Flat(value) => value,
            Self::Levels { start, values } => {
                let index = level.saturating_sub(start) as usize;
                let last = values.len() - 1;

                values[if index < last { index } else { last }]
            }
            Self::Upgrades {
                base,
                per_upgrade,
                cap,
            } => (base + per_upgrade * Monster::minion_upgrades(level) as f32).min(cap),
            Self::TriangularUpgrades {
                coefficient,
                after,
                cap,
            } => {
                let upgrades = Monster::minion_upgrades(level);

                if upgrades <= after as u32 {
                    return 0.0;
                }

                let x = (upgrades - after as u32) as f32;

                (coefficient * (x - 1.0) / 2.0 * x).min(cap)
            }
        }
    }
}

impl MonsterStats {
    pub const fn at(self, level: u8) -> SimpleStats {
        SimpleStats {
            armor: self.armor.at(level),
            max_health: self.health.at(level),
            magic_resist: self.magic_resist.at(level),
        }
    }
}

impl Monster {
    /// Stats of the monster when its level is `level`. Starting levels, such as
    /// level 11 of [`Monster::BaronNashor`], are ignored and clamp the value instead
    pub const fn stats(self, level: u8) -> SimpleStats {
        Self::DATA[self as usize].at(level)
    }

    /// Level of every monster, the average level of all champions rounded to the
    /// nearest integer, as camps do when they spawn
    pub fn level(levels: impl IntoIterator<Item = u8>) -> u8 {
        let (sum, count) = levels
            .into_iter()
            .fold((0u32, 0u32), |(sum, count), level| {
                (sum + level as u32, count + 1)
            });

        match count {
            0 => 1,
            _ => ((sum + count / 2) / count) as u8,
        }
    }

    /// Minutes of game assumed for monsters of `level`, since the game time is not
    /// an input of the calculator
    pub const fn game_time(level: u8) -> f32 {
        2.0 * level as f32
    }

    /// Upgrades minions have gained when monsters are of `level`. The first is
    /// gained at 0:30, then another every 90 seconds
    pub const fn minion_upgrades(level: u8) -> u32 {
        let minutes = Self::game_time(level);

        match minutes < 0.5 {
            true => 0,
            false => ((minutes - 0.5) / 1.5) as u32 + 1,
        }
    }
}
