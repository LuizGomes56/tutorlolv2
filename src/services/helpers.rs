use super::riot_formulas::RiotFormulas;
use crate::{
    BASIC_ATTACK, CRITICAL_STRIKE, INTERNAL_ITEMS, INTERNAL_RUNES, SIMULATED_ITEMS,
    SIZE_DAMAGING_RUNES, SIZE_SIMULATED_ITEMS,
    model::{SIZE_ABILITIES, SIZE_ITEMS_EXPECTED, base::*},
};
use internal_comptime::{
    AbilityLike, AdaptativeType, AttackType, Attrs, CachedChampion, CachedItem, ChampionId,
    DamageExpression, DamageType, EvalContext, ItemId, RuneId, zero,
};
use smallvec::SmallVec;
use tinyset::SetU32;

/// By 06/07/2025 Earth dragons give +5% resists
// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f64 = 0.05;
/// By 06/07/2025 Fire dragons give +3% bonus attack stats
// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f64 = 0.03;
/// Chemtech Dragons will be used to calculate shields/healing/vamp
// #![unsupported]
// #![manual_impl]
pub const CHEMTECH_DRAGON_MULTIPLIER: f64 = 0.06;

pub fn get_simulated_champion_stats<'a>(
    current_stats: &Stats,
    owned_items: &SetU32,
    ally_dragon_multipliers: &DragonMultipliers,
) -> SmallVec<[(ItemId, Stats); SIZE_SIMULATED_ITEMS]> {
    let mut simulated_stats = SmallVec::with_capacity(SIZE_SIMULATED_ITEMS);
    for item_id in SIMULATED_ITEMS.iter() {
        if owned_items.contains(*item_id) {
            continue;
        }
        let item_id_enum = ItemId::from_u32(*item_id);
        if let Some(item) = INTERNAL_ITEMS.get(item_id_enum as usize) {
            simulated_stats.push((
                item_id_enum,
                simulate_champion_stats(item, *current_stats, ally_dragon_multipliers),
            ));
        }
    }
    simulated_stats
}

pub fn simulate_champion_stats(
    item_cache: &&CachedItem,
    cloned_stats: Stats,
    ally_dragon_multipliers: &DragonMultipliers,
) -> Stats {
    let stats = &item_cache.stats;
    let mut result = cloned_stats;

    macro_rules! add_stat {
        ($field:ident) => {
            result.$field += stats.$field;
        };
        ($field:ident, $field2:ident) => {
            result.$field += stats.$field2;
        };
        (#$field:ident) => {
            result.$field = RiotFormulas::percent_value(SmallVec::<[f64; 2]>::from([
                result.$field,
                stats.$field,
            ]));
        };
    }

    add_stat!(max_mana, mana);
    add_stat!(max_health, health);
    add_stat!(magic_resist, magic_resistance);
    add_stat!(crit_chance, critical_strike_chance);
    add_stat!(crit_damage, critical_strike_damage);
    add_stat!(ability_power);
    add_stat!(attack_damage);
    add_stat!(armor);
    add_stat!(attack_speed);
    add_stat!(armor_penetration_flat);
    add_stat!(magic_penetration_flat);
    add_stat!(#armor_penetration_percent);
    add_stat!(#magic_penetration_percent);
    result.ability_power *= ally_dragon_multipliers.fire;
    result.attack_damage *= ally_dragon_multipliers.fire;
    result.armor *= ally_dragon_multipliers.earth;
    result.magic_resist *= ally_dragon_multipliers.earth;

    result
}

pub fn get_items_damage(
    current_player_damaging_items: &SetU32,
    attack_type: AttackType,
    level: u8,
) -> SmallVec<[(ItemId, DamageExpression); SIZE_ITEMS_EXPECTED]> {
    let mut result = SmallVec::with_capacity(current_player_damaging_items.len());
    for item_id in current_player_damaging_items.iter() {
        if let Some(item) = INTERNAL_ITEMS.get(item_id as usize) {
            let item_damage = match attack_type {
                AttackType::Ranged => &item.ranged,
                AttackType::Melee => &item.melee,
            };
            result.push((
                unsafe { std::mem::transmute(item_id as u16) },
                DamageExpression {
                    level,
                    attributes: item.attributes,
                    damage_type: item.damage_type,
                    minimum_damage: item_damage.minimum_damage,
                    maximum_damage: item_damage.maximum_damage,
                },
            ));
        }
    }
    result
}

pub fn get_runes_damage(
    current_player_damaging_runes: &SetU32,
    attack_type: AttackType,
) -> SmallVec<[(RuneId, DamageExpression); SIZE_DAMAGING_RUNES]> {
    let mut result = SmallVec::with_capacity(current_player_damaging_runes.len());
    for rune_id in current_player_damaging_runes.iter() {
        if let Some(rune) = INTERNAL_RUNES.get(rune_id as usize) {
            let minimum_damage = match attack_type {
                AttackType::Ranged => rune.ranged,
                AttackType::Melee => rune.melee,
            };
            result.push((
                unsafe { std::mem::transmute(rune_id as u8) },
                DamageExpression {
                    level: 0,
                    attributes: Attrs::None,
                    damage_type: rune.damage_type,
                    minimum_damage,
                    maximum_damage: zero,
                },
            ));
        }
    }
    result
}

pub fn get_full_stats(
    enemy_state: (ChampionId, u8, f64),
    enemy_stats: (BasicStats, &[u32]),
    armor_val: (f64, f64),
    magic_val: (f64, f64),
) -> (BasicStats, BasicStats, GenericStats) {
    let (enemy_champion_id, enemy_level, earth_dragon_mod) = enemy_state;
    let (enemy_base_stats, enemy_items) = enemy_stats;

    let mut enemy_current_stats =
        get_enemy_current_stats(enemy_base_stats, enemy_items, earth_dragon_mod);
    let mut enemy_bonus_stats = get_bonus_stats(enemy_current_stats, enemy_base_stats);

    macro_rules! real_resist {
        ($tuple:expr, $resist_val:expr) => {{
            let real_val = ($resist_val * $tuple.0 - $tuple.1).max(0.0);
            let modf_val = 100.0 / (100.0 + real_val);
            (real_val, modf_val)
        }};
    }

    let (real_armor, armor_mod) = real_resist!(armor_val, enemy_current_stats.armor);
    let (real_magic, magic_mod) = real_resist!(magic_val, enemy_current_stats.magic_resist);
    let (enemy_physical_mod, mut enemy_magic_mod, enemy_true_mod, enemy_global_mod) =
        (1.0, 1.0, 1.0, 1.0);
    let (self_physical_mod, self_magic_mod, self_true_mod, self_global_mod) = (1.0, 1.0, 1.0, 1.0);

    match enemy_champion_id {
        ChampionId::Kassadin => {
            // #![manual_impl]
            enemy_magic_mod -= 0.1;
        }
        ChampionId::Ornn => {
            // Starts game with +10% armor/mr/hp already
            // After level 13, player will start upgrading items
            // At level 18, the maximum bonus must have been reached
            // For every upgrade, a +4% resist is applied.
            // #![manual_impl]
            let ornn_resist_multiplier: f64 = match enemy_level {
                13..18 => (enemy_level - 12) as f64 * 0.04,
                18 => 1.3,
                _ => 1.1,
            };
            macro_rules! assign_value {
                ($field:ident) => {
                    enemy_current_stats.$field *= ornn_resist_multiplier;
                    enemy_bonus_stats.$field *= ornn_resist_multiplier;
                };
            }
            assign_value!(armor);
            assign_value!(magic_resist);
            assign_value!(health);
        }
        ChampionId::Malphite => {
            // W upgrade pattern for malphite by 06/07/2025
            // #![manual_impl]
            let malphite_resist_multiplier: f64 = match enemy_level {
                0..3 => 1.0,
                3..14 => 1.1,
                14 => 1.15,
                15..17 => 1.2,
                17 => 1.25,
                _ => 1.3,
            };
            enemy_current_stats.armor *= malphite_resist_multiplier;
            enemy_bonus_stats.armor *= malphite_resist_multiplier;
        }
        _ => {}
    }

    let has_item = |origin: &[u32], check_for: &[u32]| -> bool {
        check_for.iter().any(|id| origin.contains(id))
    };

    let generic_stats = GenericStats {
        real_armor,
        real_magic,
        armor_mod,
        magic_mod,
        enemy_mod: (
            enemy_physical_mod,
            enemy_magic_mod,
            enemy_true_mod,
            enemy_global_mod,
        ),
        self_mod: (
            self_physical_mod,
            self_magic_mod,
            self_true_mod,
            self_global_mod,
        ),
        // #![manual_impl]
        steelcaps: has_item(enemy_items, &[3047]),
        // #![manual_impl]
        rocksolid: has_item(enemy_items, &[3082, 3110, 3143]),
        // #![manual_impl]
        randuin: has_item(enemy_items, &[3143]),
    };

    (enemy_current_stats, enemy_bonus_stats, generic_stats)
}

#[inline]
pub fn get_damage_multipliers(modifiers: &DamageMultipliers, damage_type: DamageType) -> f64 {
    let DamageMultipliers {
        self_mod,
        enemy_mod,
        damage_mod,
    } = modifiers;
    let (enemy_debuff_multiplier, damage_reduction_multiplier, damage_increase_multiplier) =
        match damage_type {
            DamageType::Physical => (enemy_mod.0, damage_mod.0, self_mod.0),
            DamageType::Magic => (enemy_mod.1, damage_mod.1, self_mod.1),
            DamageType::True => (enemy_mod.2, 1.0, self_mod.2),
            _ => (1.0, 1.0, 1.0),
        };
    damage_reduction_multiplier
        * enemy_debuff_multiplier
        * damage_increase_multiplier
        * self_mod.3
        * enemy_mod.3
}

pub fn get_abilities_damage(
    current_player_cache: &&CachedChampion,
    current_player_level: u8,
    abilities: AbilityLevels,
) -> SmallVec<[(AbilityLike, DamageExpression); SIZE_ABILITIES]> {
    let mut result = SmallVec::with_capacity(current_player_cache.abilities.len() + 2);
    for (key, value) in current_player_cache.abilities.iter() {
        let level = match key {
            AbilityLike::P(_) => current_player_level,
            AbilityLike::Q(_) => abilities.q,
            AbilityLike::W(_) => abilities.w,
            AbilityLike::E(_) => abilities.e,
            AbilityLike::R(_) => abilities.r,
            _ => continue,
        };
        result.push((
            *key,
            DamageExpression {
                level,
                attributes: value.attributes,
                damage_type: value.damage_type,
                minimum_damage: value.minimum_damage,
                maximum_damage: value.maximum_damage,
            },
        ));
    }

    result.push((AbilityLike::A, BASIC_ATTACK));
    result.push((AbilityLike::C, CRITICAL_STRIKE));
    result
}

// #![unsupported] Champion stacks are ignored.
/// current_player_state: (CurrentStats, BaseStats, BonusStats, Level)
/// enemy_state:(CurrentStats, BonusStats, GenericStats)
pub fn get_eval_ctx(
    current_player_state: &(&Stats, BasicStats, BasicStats, u8),
    enemy_state: &(BasicStats, BasicStats, GenericStats),
) -> EvalContext {
    let (enemy_current_stats, enemy_bonus_stats, generic_stats) = enemy_state;
    let (
        current_player_stats,
        current_player_base_stats,
        current_player_bonus_stats,
        current_player_level,
    ) = current_player_state;
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
            current_player_stats.attack_damage,
            current_player_stats.ability_power,
        ) {
            AdaptativeType::Physical => generic_stats.armor_mod,
            AdaptativeType::Magic => generic_stats.magic_mod,
        },
        level: *current_player_level as f64,
        physical_multiplier: generic_stats.armor_mod,
        magic_multiplier: generic_stats.magic_mod,
        // #![manual_impl]
        steelcaps_effect: if generic_stats.steelcaps { 0.88 } else { 1.0 },
        // #![manual_impl]
        randuin_effect: if generic_stats.randuin { 0.7 } else { 1.0 },
        // #![manual_impl]
        rocksolid_effect: if generic_stats.rocksolid { 0.8 } else { 1.0 },
        enemy_bonus_health: enemy_bonus_stats.health,
        enemy_armor: enemy_current_stats.armor,
        enemy_max_health: enemy_current_stats.health,
        enemy_health: enemy_current_stats.health,
        enemy_current_health: enemy_current_stats.health,
        enemy_missing_health: enemy_current_stats.health,
        enemy_magic_resist: enemy_current_stats.magic_resist,
        base_health: current_player_base_stats.health,
        base_ad: current_player_base_stats.attack_damage,
        base_armor: current_player_base_stats.armor,
        base_magic_resist: current_player_base_stats.magic_resist,
        base_mana: current_player_base_stats.mana,
        bonus_ad: current_player_bonus_stats.attack_damage,
        bonus_armor: current_player_bonus_stats.armor,
        bonus_magic_resist: current_player_bonus_stats.magic_resist,
        bonus_health: current_player_bonus_stats.health,
        bonus_mana: current_player_bonus_stats.mana,
        // #![unsupported]
        bonus_move_speed: 1.0,
        armor_penetration_flat: current_player_stats.armor_penetration_flat,
        armor_penetration_percent: current_player_stats.armor_penetration_percent,
        magic_penetration_flat: current_player_stats.magic_penetration_flat,
        magic_penetration_percent: current_player_stats.magic_penetration_percent,
        max_mana: current_player_stats.max_mana,
        current_mana: current_player_stats.current_mana,
        max_health: current_player_stats.max_health,
        current_health: current_player_stats.current_health,
        armor: current_player_stats.armor,
        magic_resist: current_player_stats.magic_resist,
        crit_chance: current_player_stats.crit_chance,
        crit_damage: current_player_stats.crit_damage,
        attack_speed: current_player_stats.attack_speed,
        missing_health: 1.0
            - (current_player_stats.current_health / current_player_stats.max_health.max(1.0)),
        ap: current_player_stats.ability_power,
        ad: current_player_stats.attack_damage,
    }
}

/// Returns the difference between current stats and base stats
/// current_stats must be a tpe that can be converted to struct `RiotChampionStats`
#[inline]
pub const fn get_bonus_stats(current_stats: BasicStats, base_stats: BasicStats) -> BasicStats {
    BasicStats {
        armor: current_stats.armor - base_stats.armor,
        health: current_stats.health - base_stats.health,
        attack_damage: current_stats.attack_damage - base_stats.attack_damage,
        magic_resist: current_stats.magic_resist - base_stats.magic_resist,
        mana: current_stats.mana - base_stats.mana,
    }
}

/// Reads cached values for a given champion and assigns its base stats at a given level
#[inline]
pub const fn get_base_stats(champion_cache: &&CachedChampion, level: u8) -> BasicStats {
    macro_rules! assign_value {
        ($field:ident) => {
            RiotFormulas::stat_growth(
                champion_cache.stats.$field.flat,
                champion_cache.stats.$field.per_level,
                level,
            )
        };
    }
    BasicStats {
        armor: assign_value!(armor),
        health: assign_value!(health),
        attack_damage: assign_value!(attack_damage),
        magic_resist: assign_value!(magic_resistance),
        mana: assign_value!(mana),
    }
}

/// Reads enemy player's items and base stats
/// Return value may not match the in-game value due to runes/stacks
/// There are several other situations where enemy current stats
/// Can't be evaluated precisely.
#[inline]
pub fn get_enemy_current_stats(
    mut basic_stats: BasicStats,
    current_items: &[u32],
    earth_dragon_mod: f64,
) -> BasicStats {
    for enemy_item in current_items {
        if let Some(item) = INTERNAL_ITEMS.get(ItemId::from_u32(*enemy_item) as usize) {
            macro_rules! add_value {
                ($field:ident) => {
                    basic_stats.$field += item.stats.$field;
                };
            }
            add_value!(attack_damage);
            add_value!(health);
            add_value!(armor);
            basic_stats.magic_resist += item.stats.magic_resistance;
            add_value!(mana);
        }
    }
    basic_stats.armor *= earth_dragon_mod;
    basic_stats.magic_resist *= earth_dragon_mod;
    basic_stats
}

fn transform_expr<T: Copy + 'static>(
    tuple: (T, DamageExpression),
    onhit_effects: &mut DamageValue,
    damage_mlt: &DamageMultipliers,
    eval_ctx: &EvalContext,
) -> (T, InstanceDamage) {
    let damage_type = tuple.1.damage_type;
    let damage_mod = get_damage_multipliers(damage_mlt, damage_type);
    let minimum_damage = damage_mod * (tuple.1.minimum_damage)(tuple.1.level, eval_ctx);
    let maximum_damage = damage_mod * (tuple.1.maximum_damage)(tuple.1.level, eval_ctx);
    match tuple.1.attributes {
        Attrs::OnhitMin => {
            onhit_effects.minimum_damage += maximum_damage + minimum_damage;
        }
        Attrs::OnhitMax => {
            onhit_effects.maximum_damage += maximum_damage + minimum_damage;
        }
        Attrs::Onhit => {
            onhit_effects.minimum_damage += minimum_damage + minimum_damage;
            onhit_effects.maximum_damage += minimum_damage + minimum_damage;
        }
        Attrs::None => {}
    };
    (
        tuple.0,
        InstanceDamage {
            damage_type,
            minimum_damage,
            maximum_damage,
        },
    )
}

pub fn get_damages<const N: usize, T: Copy + 'static>(
    tuples: &[(T, DamageExpression)],
    damage_multipliers: &DamageMultipliers,
    eval_ctx: &EvalContext,
    onhit_effects: &mut DamageValue,
) -> DamageLike<N, T> {
    let mut result = DamageLike::<N, T>::with_capacity(tuples.len());
    for tuple in tuples.iter().copied() {
        let (key, val) = transform_expr(tuple, onhit_effects, damage_multipliers, eval_ctx);
        result.push((key, val));
    }
    result
}
