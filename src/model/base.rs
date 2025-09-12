use super::{SIZE_ABILITIES, Sizer, riot::RiotChampionStats};
use bincode::{Decode, Encode};
use internal_comptime::DamageType;
use smallvec::SmallVec;

macro_rules! castable {
    (#[derive($($derives:tt),+)] pub struct $name:ident { $($fields:tt),+ }) => {
        #[derive($($derives),+)]
        pub struct $name<T = f32> {
            $(pub $fields: T),+
        }

        impl $name<f32> {
            #[inline(always)]
            pub fn cast_i32(&self) -> $name<i32> {
                $name {
                    $( $fields: self.$fields as i32 ),+
                }
            }
        }

        impl Sizer for $name<i32> {
            #[inline(always)]
            fn size(&self) -> usize {
                let mut sum = 0;
                $(
                    sum += self.$fields.size();
                )+
                sum
            }
        }
    };
}

#[derive(Encode)]
pub struct InstanceDamage<T = f32> {
    pub minimum_damage: T,
    pub maximum_damage: T,
    pub damage_type: DamageType,
}

castable!(
    #[derive(Encode, Copy, Clone, Decode)]
    pub struct Stats {
        ability_power,
        armor,
        armor_penetration_flat,
        armor_penetration_percent,
        attack_damage,
        attack_range,
        attack_speed,
        crit_chance,
        crit_damage,
        current_health,
        magic_penetration_flat,
        magic_penetration_percent,
        magic_resist,
        max_health,
        max_mana,
        current_mana
    }
);

impl RiotChampionStats {
    pub fn to_stats(&self) -> Stats<f32> {
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

pub type DamageLike<const N: usize, T> = SmallVec<[(T, InstanceDamage<i32>); N]>;

castable!(
    #[derive(Encode, Copy, Clone, Decode)]
    pub struct BasicStats {
        armor,
        health,
        attack_damage,
        magic_resist,
        mana
    }
);

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

#[derive(Encode)]
pub struct Attacks<T = f32> {
    pub basic_attack: DamageValue<T>,
    pub critical_strike: DamageValue<T>,
    pub onhit_damage: DamageValue<T>,
}

#[derive(Encode)]
pub struct MonsterExpr<T = f32> {
    pub attacks: Attacks<T>,
    pub abilities: SmallVec<[InstanceDamage<T>; SIZE_ABILITIES]>,
    pub items: SmallVec<[InstanceDamage<T>; 5]>,
}

pub type MonsterDamages<T> = [MonsterExpr<T>; 7];

#[derive(Encode, Default)]
pub struct DamageValue<T = f32> {
    pub minimum_damage: T,
    pub maximum_damage: T,
}
