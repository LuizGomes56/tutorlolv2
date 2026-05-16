pub mod exports;

use bincode::{Decode, Encode};
use core::ops::Range;
use serde::{Deserialize, Serialize};
use tutorlolv2_types::{
    AbilityId::{self, *},
    AbilityName::*,
    AdaptiveType,
    AttackType::{self, *},
    Attrs::*,
    ComboElement::{self, *},
    Ctx,
    CtxVar::{self, *},
    DamageType::*,
    GameMap::{self, *},
    MergeData,
    Position::{self, *},
    StatName, TypeMetadata,
};

pub type Closure = fn(&Ctx) -> f32;

#[derive(Clone, Copy, Debug)]
pub struct Champion {
    pub name: &'static str,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub positions: &'static [Position],
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub combos: &'static [&'static [ComboElement]],
    pub metadata: &'static [TypeMetadata<AbilityId>],
    pub merge_data: &'static [MergeData],
    pub identifiers: &'static [&'static [CtxVar]],
    pub closures: &'static [Closure],
}

#[derive(Clone, Copy, Debug)]
pub struct WikiStats {
    pub health: Stat,
    pub mana: Stat,
    pub armor: Stat,
    pub magic_resist: Stat,
    pub attack_damage: Stat,
    pub attack_speed: Stat,
    pub attack_speed_ratio: f32,
    pub crit_modifier: f32,
    pub crit_base: f32,
    pub move_speed: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Stat {
    pub base: f32,
    pub per_level: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Modifier {
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct WikiModifiers {
    pub ofa: Modifier,
    pub usb: Modifier,
    pub aram: Modifier,
    pub ar: Modifier,
    pub nb: Modifier,
    pub swift: Modifier,
    pub urf: Modifier,
}

#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub name: &'static str,
    pub tier: u8,
    pub price: u16,
    pub stats: &'static [(StatName, u16)],
    pub maps: &'static [GameMap],
    pub metadata: TypeMetadata<exports::ItemId>,
    pub ranged: [Closure; 2],
    pub melee: [Closure; 2],
    pub deals_damage: [bool; 4],
    pub purchasable: bool,
    pub riot_id: u32,
    pub identifiers: [[&'static [CtxVar]; 2]; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct Rune {
    pub name: &'static str,
    pub metadata: TypeMetadata<exports::RuneId>,
    pub ranged: [Closure; 2],
    pub melee: [Closure; 2],
    pub deals_damage: [bool; 4],
    pub riot_id: u32,
    pub identifiers: [[&'static [CtxVar]; 2]; 2],
}

pub const fn zero(_: &Ctx) -> f32 {
    0.0
}
