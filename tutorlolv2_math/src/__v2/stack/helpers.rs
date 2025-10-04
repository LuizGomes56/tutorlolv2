use super::model::*;
use crate::__v2::{AbilityLevels, L_ABLT, L_ITEM, L_RUNE, L_SIML, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::*;

/// By 06/07/2025 Earth dragons give +5% resists
// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f32 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f32 = 0.03;
pub const LAST_STAND_CLOSURE: fn(f32) -> f32 =
    |missing_health| 1.0 + (0.05 + 0.2 * (missing_health - 0.4)).clamp(0.0, 0.11);
pub const COUP_DE_GRACE_AND_CUTDOWN_BONUS_DMG: f32 = 1.08;

#[macro_export]
macro_rules! base_stats {
    (@inner $stats:expr, $level:expr) => {
        self::RiotFormulas::stat_growth($stats.flat, $stats.per_level, $level)
    };
    ($struct:ident($stats:expr, $level:expr) { $($field:ident),*}) => {
        $struct {
            $(
                $field: base_stats!(@inner &$stats.$field, $level),
            )*
        }
    };
}

#[macro_export]
macro_rules! bonus_stats {
    ($struct:ident($current_stats:expr, $base_stats:expr) { $($field:ident),*}) => {
        $struct {
            $(
                $field: $current_stats.$field - $base_stats.$field,
            )*
        }
    };
}

pub use {base_stats, bonus_stats};

#[inline]
pub fn get_simulated_stats(
    stats: &RiotChampionStats,
    dragons: Dragons,
) -> [RiotChampionStats; L_SIML] {
    let mut result = MaybeUninit::<[RiotChampionStats; L_SIML]>::uninit();
    let result_ptr = result.as_mut_ptr();

    for (i, item_offset) in SIMULATED_ITEMS_ENUM.into_iter().enumerate() {
        let item_cache = unsafe { INTERNAL_ITEMS.get_unchecked(item_offset as usize) };
        let mut new_stat = *stats;

        macro_rules! add_stat {
            ($field:ident) => {
                new_stat.$field += item_cache.stats.$field;
            };
            (@$field:ident) => {
                new_stat.$field = RiotFormulas::percent_value([new_stat.$field, stats.$field]);
            };
        }

        add_stat!(mana);
        add_stat!(health);
        add_stat!(magic_resist);
        add_stat!(crit_chance);
        add_stat!(crit_damage);
        add_stat!(ability_power);
        add_stat!(attack_damage);
        add_stat!(armor);
        add_stat!(attack_speed);
        add_stat!(armor_penetration_flat);
        add_stat!(magic_penetration_flat);
        add_stat!(@armor_penetration_percent);
        add_stat!(@magic_penetration_percent);

        let fire_mod = 1.0 + dragons.fire as f32 * FIRE_DRAGON_MULTIPLIER;
        let earth_mod = 1.0 + dragons.earth as f32 * EARTH_DRAGON_MULTIPLIER;

        new_stat.ability_power *= fire_mod;
        new_stat.attack_damage *= fire_mod;
        new_stat.magic_resist *= earth_mod;
        new_stat.armor *= earth_mod;

        unsafe {
            core::ptr::addr_of_mut!((*result_ptr)[i]).write(new_stat);
        }
    }

    unsafe { result.assume_init() }
}

#[inline]
pub fn get_abilities_data(
    ability_cache: &'static [(AbilityLike, CachedChampionAbility)],
    ability_levels: AbilityLevels,
    level: u8,
) -> DamageKind<L_ABLT, AbilityLike> {
    let mut metadata = SmallVec::with_capacity(ability_cache.len());
    let mut damages = SmallVec::with_capacity(ability_cache.len());
    for (key, value) in ability_cache.iter() {
        let level = match key {
            AbilityLike::P(_) => level,
            AbilityLike::Q(_) => ability_levels.q,
            AbilityLike::W(_) => ability_levels.w,
            AbilityLike::E(_) => ability_levels.e,
            AbilityLike::R(_) => ability_levels.r,
        };
        damages.push(DamageClosure {
            minimum_damage: value.minimum_damage,
            maximum_damage: value.maximum_damage,
        });
        metadata.push(TypeMetadata {
            level,
            kind: *key,
            meta: Meta::from_bytes(value.damage_type, value.attributes),
        });
    }
    DamageKind {
        metadata,
        closures: damages,
    }
}

#[inline]
pub fn get_runes_data(runes: &SetU32, level: u8) -> DamageKind<L_RUNE, RuneId> {
    let mut metadata = SmallVec::with_capacity(runes.len());
    let mut damages = SmallVec::with_capacity(runes.len());
    for rune_number in runes.iter() {
        let rune_id = unsafe { std::mem::transmute::<u8, RuneId>(rune_number as u8) };
        let rune = unsafe { INTERNAL_RUNES.get_unchecked(rune_number as usize) };
        damages.push(DamageClosure {
            minimum_damage: rune.ranged,
            maximum_damage: zero,
        });
        metadata.push(TypeMetadata {
            level,
            kind: rune_id,
            meta: Meta::from_bytes(rune.damage_type, Attrs::None),
        });
    }
    DamageKind {
        metadata,
        closures: damages,
    }
}

#[inline]
pub fn get_items_data(
    items: &SetU32,
    attack_type: AttackType,
    level: u8,
) -> DamageKind<L_ITEM, ItemId> {
    let mut metadata = SmallVec::with_capacity(items.len());
    let mut damages = SmallVec::with_capacity(items.len());
    for item_number in items.iter() {
        let item_id = unsafe { std::mem::transmute::<u16, ItemId>(item_number as u16) };
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(item_number as usize) };
        let item_damage = match attack_type {
            AttackType::Ranged => &item.ranged,
            AttackType::Melee => &item.melee,
        };
        damages.push(DamageClosure {
            minimum_damage: item_damage.minimum_damage,
            maximum_damage: item_damage.maximum_damage,
        });
        metadata.push(TypeMetadata {
            level,
            kind: item_id,
            meta: Meta::from_bytes(item.damage_type, item.attributes),
        });
    }
    DamageKind {
        metadata,
        closures: damages,
    }
}

#[inline]
pub fn runes_slice_to_set_u32(input: &[RuneId]) -> SetU32 {
    input
        .iter()
        .filter_map(|rune| {
            DAMAGING_RUNES
                .contains(&rune.to_riot_id())
                .then_some(*rune as u32)
        })
        .collect::<SetU32>()
}

#[inline]
pub fn items_slice_to_set_u32(input: &[ItemId]) -> SetU32 {
    input
        .iter()
        .filter_map(|item| {
            DAMAGING_ITEMS
                .contains(&item.to_riot_id())
                .then_some(*item as u32)
        })
        .collect::<SetU32>()
}

#[inline]
pub fn riot_items_to_set_u32(input: &[RiotItems]) -> SetU32 {
    input
        .iter()
        .filter_map(|riot_item| {
            let item_id = riot_item.item_id;
            DAMAGING_ITEMS
                .contains(&item_id)
                .then_some(ItemId::from_riot_id(item_id) as u32)
        })
        .collect::<SetU32>()
}

#[inline]
pub fn get_enemy_current_stats(stats: &mut SimpleStatsF32, items: &SetU32, earth_dragons: u8) {
    for item_id in items.iter() {
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(item_id as usize) };
        macro_rules! add_value {
            ($field:ident) => {
                stats.$field += item.stats.$field;
            };
        }
        add_value!(health);
        add_value!(armor);
        add_value!(magic_resist);
    }
    let dragon_mod = 1.0 + earth_dragons as f32 * EARTH_DRAGON_MULTIPLIER;
    stats.armor *= dragon_mod;
    stats.magic_resist *= dragon_mod;
}

#[inline]
pub fn get_enemy_state(
    state: EnemyState,
    shred: ResistShred,
    earth_dragons: u8,
    accept_negatives: bool,
) -> EnemyFullState {
    let mut e_current_stats = state.base_stats;
    let e_items = &state.items;
    let stacks = state.stacks as f32;

    get_enemy_current_stats(&mut e_current_stats, &e_items, earth_dragons);

    let mut e_modifiers = DamageModifiers::default();

    match state.champion_id {
        ChampionId::Swain => {
            let stack_hp = 12.0 * stacks;
            e_current_stats.health += stack_hp;
        }
        ChampionId::Chogath => {
            let stack_hp = stacks * 80.0
                + 40.0
                    * match state.level {
                        ..6 => 0.0,
                        6..11 => 1.0,
                        11..16 => 2.0,
                        16.. => 3.0,
                    };
            e_current_stats.health += stack_hp;
        }
        ChampionId::Sion => {
            e_current_stats.health += stacks;
        }
        ChampionId::Kassadin => {
            // #![manual_impl]
            e_modifiers.magic_mod *= 0.9;
        }
        ChampionId::Ornn => {
            // Starts game with +10% armor/mr/hp already
            // After level 13, player will start upgrading items
            // At level 18, the maximum bonus must have been reached
            // For every upgrade, a +4% resist is applied.
            // #![manual_impl]
            let ornn_resist_multiplier = match state.level {
                ..13 => 1.1,
                13..18 => (state.level - 12) as f32 * 0.04,
                18.. => 1.3,
            };
            macro_rules! assign_value {
                ($field:ident) => {
                    e_current_stats.$field *= ornn_resist_multiplier;
                };
            }
            assign_value!(armor);
            assign_value!(magic_resist);
            assign_value!(health);
        }
        ChampionId::Malphite => {
            // W upgrade pattern for malphite by 06/07/2025
            // #![manual_impl]
            let malphite_resist_multiplier = match state.level {
                ..3 => 1.0,
                3..14 => 1.1,
                14 => 1.15,
                15..17 => 1.2,
                17 => 1.25,
                18.. => 1.3,
            };
            e_current_stats.armor *= malphite_resist_multiplier;
        }
        _ => {}
    }

    let armor_values = RiotFormulas::real_resist(
        shred.armor_penetration_percent,
        shred.armor_penetration_flat,
        e_current_stats.armor,
        accept_negatives,
    );
    let magic_values = RiotFormulas::real_resist(
        shred.magic_penetration_percent,
        shred.magic_penetration_flat,
        e_current_stats.magic_resist,
        accept_negatives,
    );

    let e_bonus_stats = bonus_stats!(
        SimpleStatsF32(e_current_stats, state.base_stats) {
            armor,
            health,
            magic_resist
        }
    );

    EnemyFullState {
        current_stats: e_current_stats,
        bonus_stats: e_bonus_stats,
        modifiers: e_modifiers,
        armor_values,
        magic_values,
        // #![manual_impl]
        steelcaps: has_item(e_items, [ItemId::PlatedSteelcaps, ItemId::ArmoredAdvance]),
        // #![manual_impl]
        rocksolid: has_item(
            e_items,
            [
                ItemId::RanduinsOmen,
                ItemId::FrozenHeart,
                ItemId::WardensMail,
            ],
        ),
        // #![manual_impl]
        randuin: has_item(e_items, [ItemId::RanduinsOmen]),
    }
}

#[inline]
pub fn get_damage_modifiers(
    enemy_modifiers: DamageModifiers,
    armor_mod: f32,
    magic_mod: f32,
) -> DamageModifiers {
    DamageModifiers {
        physical_mod: armor_mod * enemy_modifiers.physical_mod,
        magic_mod: magic_mod * enemy_modifiers.magic_mod,
        true_mod: enemy_modifiers.true_mod,
        global_mod: enemy_modifiers.global_mod,
    }
}

#[inline]
fn has_item<const N: usize>(origin: &SetU32, check_for: [ItemId; N]) -> bool {
    check_for
        .into_iter()
        .any(|item_id| origin.contains(item_id as u32))
}

#[inline]
pub fn get_eval_ctx(self_state: &SelfState, e_state: &EnemyFullState) -> EvalContext {
    EvalContext {
        chogath_stacks: 1.0,
        veigar_stacks: 1.0,
        nasus_stacks: 1.0,
        smolder_stacks: 1.0,
        aurelion_sol_stacks: 1.0,
        thresh_stacks: 1.0,
        kindred_stacks: 1.0,
        belveth_stacks: 1.0,
        adaptative_damage: match RiotFormulas::adaptative_type(
            self_state.bonus_stats.attack_damage,
            self_state.current_stats.ability_power,
        ) {
            AdaptativeType::Physical => e_state.armor_values.modifier,
            AdaptativeType::Magic => e_state.magic_values.modifier,
        },
        level: self_state.level as f32,
        physical_multiplier: e_state.armor_values.modifier,
        magic_multiplier: e_state.magic_values.modifier,
        // #![manual_impl]
        steelcaps_effect: if e_state.steelcaps { 0.88 } else { 1.0 },
        // #![manual_impl]
        randuin_effect: if e_state.randuin { 0.7 } else { 1.0 },
        // #![manual_impl]
        rocksolid_effect: if e_state.rocksolid { 0.8 } else { 1.0 },
        enemy_bonus_health: e_state.bonus_stats.health,
        enemy_armor: e_state.current_stats.armor,
        enemy_max_health: e_state.current_stats.health,
        enemy_health: e_state.current_stats.health,
        enemy_current_health: e_state.current_stats.health,
        enemy_missing_health: e_state.current_stats.health,
        enemy_magic_resist: e_state.current_stats.magic_resist,
        base_health: self_state.base_stats.health,
        base_ad: self_state.base_stats.attack_damage,
        base_armor: self_state.base_stats.armor,
        base_magic_resist: self_state.base_stats.magic_resist,
        base_mana: self_state.base_stats.mana,
        bonus_ad: self_state.bonus_stats.attack_damage,
        bonus_armor: self_state.bonus_stats.armor,
        bonus_magic_resist: self_state.bonus_stats.magic_resist,
        bonus_health: self_state.bonus_stats.health,
        bonus_mana: self_state.bonus_stats.mana,
        // #![unsupported]
        bonus_move_speed: 1.0,
        armor_penetration_flat: self_state.current_stats.armor_penetration_flat,
        armor_penetration_percent: self_state.current_stats.armor_penetration_percent,
        magic_penetration_flat: self_state.current_stats.magic_penetration_flat,
        magic_penetration_percent: self_state.current_stats.magic_penetration_percent,
        max_mana: self_state.current_stats.mana,
        current_mana: self_state.current_stats.current_mana,
        max_health: self_state.current_stats.health,
        current_health: self_state.current_stats.current_health,
        armor: self_state.current_stats.armor,
        magic_resist: self_state.current_stats.magic_resist,
        crit_chance: self_state.current_stats.crit_chance,
        crit_damage: self_state.current_stats.crit_damage,
        attack_speed: self_state.current_stats.attack_speed,
        missing_health: 1.0
            - (self_state.current_stats.current_health / self_state.current_stats.health.max(1.0)),
        ap: self_state.current_stats.ability_power,
        ad: self_state.current_stats.attack_damage,
    }
}

#[inline]
pub fn eval_damage<const N: usize, T>(
    ctx: &EvalContext,
    onhit: &mut RangeDamageI32,
    damage_kind: &DamageKind<N, T>,
    modifiers: DamageModifiers,
) -> SmallVec<[RangeDamageI32; N]> {
    let mut result = SmallVec::with_capacity(N);
    for i in 0..N {
        let closure = unsafe { damage_kind.closures.get_unchecked(i) };
        let metadata = unsafe { damage_kind.metadata.get_unchecked(i) };
        let damage_type = metadata.meta.damage_type();
        let attributes = metadata.meta.attributes();
        let modifier = match damage_type {
            DamageType::Physical => modifiers.physical_mod,
            DamageType::Magic => modifiers.magic_mod,
            DamageType::True => modifiers.true_mod,
            _ => 1.0,
        } * modifiers.global_mod;

        let minimum_damage = (modifier * (closure.minimum_damage)(metadata.level, ctx)) as i32;
        let maximum_damage = (modifier * (closure.maximum_damage)(metadata.level, ctx)) as i32;

        let sum = minimum_damage + minimum_damage;
        match attributes {
            Attrs::OnhitMin => {
                onhit.minimum_damage += sum;
            }
            Attrs::OnhitMax => {
                onhit.maximum_damage += sum;
            }
            Attrs::Onhit => {
                onhit.minimum_damage += sum;
                onhit.maximum_damage += sum;
            }
            _ => {}
        };

        result.push(RangeDamageI32 {
            minimum_damage,
            maximum_damage,
        });
    }
    result
}

#[inline]
pub fn eval_attacks(ctx: &EvalContext, mut onhit_damage: RangeDamageI32) -> Attacks {
    let basic_attack_damage = (BASIC_ATTACK.minimum_damage)(0, ctx) as i32;
    let critical_strike_damage = (CRITICAL_STRIKE.minimum_damage)(0, ctx) as i32;

    onhit_damage.minimum_damage += basic_attack_damage;
    onhit_damage.maximum_damage += critical_strike_damage;

    Attacks {
        basic_attack: RangeDamageI32 {
            minimum_damage: basic_attack_damage,
            maximum_damage: 0,
        },
        critical_strike: RangeDamageI32 {
            minimum_damage: 0,
            maximum_damage: critical_strike_damage,
        },
        onhit_damage,
    }
}

#[inline]
pub fn get_damages(
    eval_ctx: &EvalContext,
    data: &DamageEvalData,
    modifiers: DamageModifiers,
) -> Damages {
    let mut onhit = RangeDamageI32::default();
    macro_rules! eval_nonempty {
        ($name:ident) => {
            if data.$name.closures.is_empty() {
                SmallVec::new()
            } else {
                eval_damage(&eval_ctx, &mut onhit, &data.$name, modifiers)
            }
        };
    }
    let abilities = eval_nonempty!(abilities);
    let items = eval_nonempty!(items);
    let runes = eval_nonempty!(runes);
    let attacks = eval_attacks(&eval_ctx, onhit);
    Damages {
        abilities,
        items,
        runes,
        attacks,
    }
}
