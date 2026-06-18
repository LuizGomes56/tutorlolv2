use {
    crate::{ChampionId, ItemId, RuneId, impls::traits::CastId},
    core::fmt::Debug,
    tutorlolv2_types::{
        AbilityId, AdaptiveType, AttackType, ComboElement, Ctx, CtxVar, GameMap, MergeData,
        Position, StatName, TypeMetadata,
    },
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EntityId {
    Champion(ChampionId),
    Item(ItemId),
    Rune(RuneId),
}

impl<T: CastId> From<T> for EntityId {
    fn from(value: T) -> Self {
        value.entity()
    }
}

impl EntityId {
    pub const fn is_champion(&self) -> bool {
        matches!(self, EntityId::Champion(_))
    }

    pub const fn is_item(&self) -> bool {
        matches!(self, EntityId::Item(_))
    }

    pub const fn is_rune(&self) -> bool {
        matches!(self, EntityId::Rune(_))
    }
}

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
    #[cfg(feature = "docs")]
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
    pub metadata: TypeMetadata<ItemId>,
    pub ranged: [Closure; 2],
    pub melee: [Closure; 2],
    pub deals_damage: [bool; 4],
    pub purchasable: bool,
    pub riot_id: u32,
    #[cfg(feature = "docs")]
    pub identifiers: &'static [CtxVar],
    pub custom: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Rune {
    pub name: &'static str,
    pub metadata: TypeMetadata<RuneId>,
    pub ranged: [Closure; 2],
    pub melee: [Closure; 2],
    pub deals_damage: [bool; 4],
    pub riot_id: u32,
    #[cfg(feature = "docs")]
    pub identifiers: &'static [CtxVar],
    pub custom: bool,
}

impl Item {
    pub const fn is_siml(&self) -> bool {
        let Self {
            purchasable,
            tier,
            price,
            maps,
            metadata: TypeMetadata { kind, .. },
            ..
        } = *self;

        let check = [
            StatName::AbilityPower,
            StatName::AttackDamage,
            StatName::AdaptiveForce,
            StatName::Lethality,
            StatName::ArmorPenetration,
            StatName::MagicPenetration,
        ];

        let mut allow = false;
        let mut i = 0;

        while i < check.len() {
            if kind.has_stat(check[i]) {
                allow = true;
            }

            i += 1;
        }

        tier >= 3 && price > 0 && purchasable && allow && {
            let mut j = 0;
            while j < maps.len() {
                if matches!(maps[j], GameMap::SummonersRift) {
                    return true;
                }
                j += 1;
            }
            false
        }
    }
}
