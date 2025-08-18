use super::{SIZE_ABILITIES, riot::RiotChampionStats};
use bincode::{Decode, Encode};
use internal_comptime::DamageType;
use smallvec::SmallVec;

#[derive(Encode)]
pub struct InstanceDamage {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
    pub damage_type: DamageType,
}

#[derive(Encode, Copy, Clone, Decode)]
pub struct Stats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub armor_penetration_percent: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub max_mana: f64,
    pub current_mana: f64,
}

impl RiotChampionStats {
    pub fn to_stats(&self) -> Stats {
        Stats {
            ability_power: self.ability_power,
            armor: self.armor,
            armor_penetration_flat: self.physical_lethality,
            armor_penetration_percent: self.armor_penetration_percent,
            attack_damage: self.attack_damage,
            attack_range: self.attack_range,
            attack_speed: self.attack_speed,
            crit_chance: self.crit_chance,
            crit_damage: self.crit_damage,
            current_health: self.current_health,
            magic_penetration_flat: self.magic_penetration_flat,
            magic_penetration_percent: self.magic_penetration_percent,
            magic_resist: self.magic_resist,
            max_health: self.max_health,
            max_mana: self.resource_max,
            current_mana: self.resource_value,
        }
    }
}

pub type DamageLike<const N: usize, T> = SmallVec<[(T, InstanceDamage); N]>;

#[derive(Encode, Copy, Clone, Decode)]
pub struct BasicStats {
    pub armor: f64,
    pub health: f64,
    pub attack_damage: f64,
    pub magic_resist: f64,
    pub mana: f64,
}

#[derive(Clone, Copy)]
pub struct GenericStats {
    pub real_armor: f64,
    pub real_magic: f64,
    pub armor_mod: f64,
    pub magic_mod: f64,
    pub enemy_mod: (f64, f64, f64, f64),
    pub self_mod: (f64, f64, f64, f64),
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
}

pub struct DamageMultipliers {
    pub self_mod: (f64, f64, f64, f64),
    pub enemy_mod: (f64, f64, f64, f64),
    pub damage_mod: (f64, f64),
}

#[derive(Copy, Clone, Decode)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

#[derive(Clone, Copy, Encode)]
pub struct DragonMultipliers {
    pub earth: f64,
    pub fire: f64,
    pub chemtech: f64,
}

impl DragonMultipliers {
    #[inline]
    pub const fn new() -> DragonMultipliers {
        DragonMultipliers {
            earth: 1.0,
            fire: 1.0,
            chemtech: 1.0,
        }
    }
}

#[derive(Encode, Default)]
pub struct DamageValue {
    pub minimum_damage: f64,
    pub maximum_damage: f64,
}

#[derive(Encode)]
pub struct Attacks {
    pub basic_attack: DamageValue,
    pub critical_strike: DamageValue,
    pub onhit_damage: DamageValue,
}

#[derive(Encode)]
pub struct MonsterExpr {
    pub attacks: Attacks,
    pub abilities: SmallVec<[InstanceDamage; SIZE_ABILITIES]>,
    pub items: SmallVec<[InstanceDamage; 5]>,
}

pub type MonsterDamages = [MonsterExpr; 7];
