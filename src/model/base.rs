use super::{SIZE_ABILITIES, riot::RiotChampionStats};
use bincode::{Decode, Encode};
use internal_comptime::DamageType;
use smallvec::SmallVec;

#[derive(Encode)]
pub struct InstanceDamage {
    pub minimum_damage: f32,
    pub maximum_damage: f32,
    pub damage_type: DamageType,
}

#[derive(Encode, Copy, Clone, Decode)]
pub struct Stats {
    pub ability_power: f32,
    pub armor: f32,
    pub armor_penetration_flat: f32,
    pub armor_penetration_percent: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub current_health: f32,
    pub magic_penetration_flat: f32,
    pub magic_penetration_percent: f32,
    pub magic_resist: f32,
    pub max_health: f32,
    pub max_mana: f32,
    pub current_mana: f32,
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
    pub armor: f32,
    pub health: f32,
    pub attack_damage: f32,
    pub magic_resist: f32,
    pub mana: f32,
}

#[derive(Clone, Copy)]
pub struct GenericStats {
    pub real_armor: f32,
    pub real_magic: f32,
    pub armor_mod: f32,
    pub magic_mod: f32,
    pub enemy_mod: (f32, f32, f32, f32),
    pub self_mod: (f32, f32, f32, f32),
    pub steelcaps: bool,
    pub rocksolid: bool,
    pub randuin: bool,
}

pub struct DamageMultipliers {
    pub self_mod: (f32, f32, f32, f32),
    pub enemy_mod: (f32, f32, f32, f32),
    pub damage_mod: (f32, f32),
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
    pub earth: f32,
    pub fire: f32,
    pub chemtech: f32,
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
    pub minimum_damage: f32,
    pub maximum_damage: f32,
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
