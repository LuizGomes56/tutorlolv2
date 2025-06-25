use crate::{
    GLOBAL_CACHE,
    model::{
        base::{AdaptativeType, BasicStats, ComparedItem, DamageExpression, GenericStats, Stats},
        cache::{CachedChampion, CachedItem, EvalContext},
        calculator::AbilitiesX,
        realtime::DragonMultipliers,
    },
    services::riot_formulas::RiotFormulas,
};
use rustc_hash::FxHashMap;

pub const CLAMP_USIZE_MAX: usize = 1 << 32;
pub const CLAMP_F64_MAX: f64 = 1e+300f64;
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
    current_stats: Stats,
    owned_items: &[usize],
    ally_dragon_multipliers: DragonMultipliers,
) -> (FxHashMap<usize, Stats>, FxHashMap<usize, ComparedItem>) {
    GLOBAL_CACHE
        .simulated_items
        .iter()
        .filter_map(move |item_id| {
            if owned_items.contains(item_id) {
                return None;
            }
            let item = GLOBAL_CACHE.items.get(item_id)?;
            let expected_stats =
                simulate_champion_stats(item, current_stats, &ally_dragon_multipliers);
            Some((
                (*item_id, expected_stats),
                (
                    *item_id,
                    ComparedItem {
                        name: &item.name,
                        gold_cost: item.gold,
                        prettified_stats: item
                            .prettified_stats
                            .iter()
                            .map(|(key, val)| (*key, *val))
                            .collect(),
                    },
                ),
            ))
        })
        .collect()
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
            result.$field = RiotFormulas::percent_value(vec![result.$field, stats.$field]);
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
    current_player_items: &[usize],
    is_ranged: bool,
) -> Vec<(usize, DamageExpression)> {
    current_player_items
        .into_iter()
        .filter_map(move |item_id| {
            let item = GLOBAL_CACHE.items.get(item_id)?;
            let item_damage = if is_ranged { &item.ranged } else { &item.melee };
            Some((
                *item_id,
                DamageExpression {
                    level: 0,
                    damage_type: item.damage_type.unwrap_or("UNKNOWN"),
                    minimum_damage: item_damage.minimum_damage,
                    maximum_damage: item_damage.maximum_damage,
                },
            ))
        })
        .collect()
}

pub fn get_runes_damage(
    current_player_runes: &[usize],
    is_ranged: bool,
) -> Vec<(usize, DamageExpression)> {
    current_player_runes
        .into_iter()
        .filter_map(move |rune_id| {
            let rune = GLOBAL_CACHE.runes.get(rune_id)?;
            let minimum_damage = if is_ranged { rune.ranged } else { rune.melee };
            Some((
                *rune_id,
                DamageExpression {
                    level: 0,
                    damage_type: rune.damage_type,
                    minimum_damage,
                    maximum_damage: |_, _| 0.0,
                },
            ))
        })
        .collect()
}

pub fn get_full_stats(
    enemy_state: (&str, usize, f64),
    enemy_stats: (BasicStats, &[usize]),
    magic_val: (f64, f64),
    armor_val: (f64, f64),
) -> (BasicStats, BasicStats, GenericStats) {
    let (enemy_champion_id, enemy_level, earth_dragon_mod) = enemy_state;
    let (enemy_base_stats, enemy_items) = enemy_stats;

    let mut enemy_current_stats =
        get_enemy_current_stats(enemy_base_stats, enemy_items, earth_dragon_mod);
    let mut enemy_bonus_stats = get_bonus_stats(enemy_current_stats, enemy_base_stats);

    macro_rules! real_resist {
        ($tuple:expr, $resist_val:expr) => {{
            let real_val = ($resist_val * $tuple.0 - $tuple.1).clamp(0.0, CLAMP_F64_MAX);
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
        "Kassadin" => {
            // #![manual_impl]
            enemy_magic_mod = 0.9;
        }
        "Ornn" => {
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
        "Malphite" => {
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

    let has_item = |origin: &[usize], check_for: &[usize]| -> bool {
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

pub fn get_damage_multipliers(
    self_mod: (f64, f64, f64, f64),
    enemy_mod: (f64, f64, f64, f64),
    damage_mod: (f64, f64),
    damage_type: &str,
) -> f64 {
    let (enemy_debuff_multiplier, damage_reduction_multiplier, damage_increase_multiplier) =
        match damage_type {
            "PHYSICAL_DAMAGE" => (enemy_mod.0, damage_mod.0, self_mod.0),
            "MAGIC_DAMAGE" => (enemy_mod.1, damage_mod.1, self_mod.1),
            "TRUE_DAMAGE" => (enemy_mod.2, 1.0, self_mod.2),
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
    current_player_level: usize,
    abilities: AbilitiesX,
) -> Vec<(&'static str, DamageExpression)> {
    current_player_cache
        .abilities
        .iter()
        .filter(|(key, _)| !key.contains("MONSTER") && !key.contains("MINION"))
        .filter_map(move |(key, value)| {
            let first_char = key.chars().next()?;
            let level = match first_char {
                'P' => current_player_level,
                'Q' => abilities.q,
                'W' => abilities.w,
                'E' => abilities.e,
                'R' => abilities.r,
                _ => return None,
            };
            Some((
                *key,
                DamageExpression {
                    level,
                    damage_type: value.damage_type,
                    minimum_damage: value.minimum_damage,
                    maximum_damage: value.maximum_damage,
                },
            ))
        })
        .chain(std::iter::once((
            "A",
            DamageExpression {
                level: 0,
                damage_type: "PHYSICAL_DAMAGE",
                minimum_damage: |_, ctx: &EvalContext| ctx.AD * ctx.PHYSICAL_MULTIPLIER,
                maximum_damage: |_, _| 0.0,
            },
        )))
        .chain(std::iter::once((
            "C",
            DamageExpression {
                level: 0,
                damage_type: "PHYSICAL_DAMAGE",
                minimum_damage: |_, ctx: &EvalContext| {
                    ctx.AD * ctx.PHYSICAL_MULTIPLIER * ctx.CRIT_DAMAGE
                },
                maximum_damage: |_, _| 0.0,
            },
        )))
        .collect()
}

// #![unsupported] Champion stacks are ignored.
/// current_player_state: (CurrentStats, BaseStats, BonusStats, Level)
/// enemy_state:(CurrentStats, BonusStats, GenericStats)
pub fn get_eval_ctx(
    current_player_state: (Stats, BasicStats, BasicStats, usize),
    enemy_state: (BasicStats, BasicStats, GenericStats),
) -> EvalContext {
    let (enemy_current_stats, enemy_bonus_stats, generic_stats) = enemy_state;
    let (
        current_player_stats,
        current_player_base_stats,
        current_player_bonus_stats,
        current_player_level,
    ) = current_player_state;
    EvalContext {
        CHOGATH_STACKS: 1.0,
        VEIGAR_STACKS: 1.0,
        NASUS_STACKS: 1.0,
        SMOLDER_STACKS: 1.0,
        AURELION_SOL_STACKS: 1.0,
        THRESH_STACKS: 1.0,
        KINDRED_STACKS: 1.0,
        BELVETH_STACKS: 1.0,
        ADAPTATIVE_DAMAGE: match RiotFormulas::adaptative_type(
            current_player_stats.attack_damage,
            current_player_stats.ability_power,
        ) {
            AdaptativeType::Physical => generic_stats.armor_mod,
            AdaptativeType::Magic => generic_stats.magic_mod,
        },
        LEVEL: current_player_level as f64,
        PHYSICAL_MULTIPLIER: generic_stats.armor_mod,
        MAGIC_MULTIPLIER: generic_stats.magic_mod,
        // #![manual_impl]
        STEELCAPS_EFFECT: if generic_stats.steelcaps { 0.88 } else { 1.0 },
        // #![manual_impl]
        RANDUIN_EFFECT: if generic_stats.randuin { 0.7 } else { 1.0 },
        // #![manual_impl]
        ROCKSOLID_EFFECT: if generic_stats.rocksolid { 0.8 } else { 1.0 },
        ENEMY_BONUS_HEALTH: enemy_bonus_stats.health,
        ENEMY_ARMOR: enemy_current_stats.armor,
        ENEMY_MAX_HEALTH: enemy_current_stats.health,
        ENEMY_HEALTH: enemy_current_stats.health,
        ENEMY_CURRENT_HEALTH: enemy_current_stats.health,
        ENEMY_MISSING_HEALTH: enemy_current_stats.health,
        ENEMY_MAGIC_RESIST: enemy_current_stats.magic_resist,
        BASE_HEALTH: current_player_base_stats.health,
        BASE_AD: current_player_base_stats.attack_damage,
        BASE_ARMOR: current_player_base_stats.armor,
        BASE_MAGIC_RESIST: current_player_base_stats.magic_resist,
        BASE_MANA: current_player_base_stats.mana,
        BONUS_AD: current_player_bonus_stats.attack_damage,
        BONUS_ARMOR: current_player_bonus_stats.armor,
        BONUS_MAGIC_RESIST: current_player_bonus_stats.magic_resist,
        BONUS_HEALTH: current_player_bonus_stats.health,
        BONUS_MANA: current_player_bonus_stats.mana,
        // #![unsupported]
        BONUS_MOVE_SPEED: 1.0,
        ARMOR_PENETRATION_FLAT: current_player_stats.armor_penetration_flat,
        ARMOR_PENETRATION_PERCENT: current_player_stats.armor_penetration_percent,
        MAGIC_PENETRATION_FLAT: current_player_stats.magic_penetration_flat,
        MAGIC_PENETRATION_PERCENT: current_player_stats.magic_penetration_percent,
        MAX_MANA: current_player_stats.max_mana,
        CURRENT_MANA: current_player_stats.current_mana,
        MAX_HEALTH: current_player_stats.max_health,
        CURRENT_HEALTH: current_player_stats.current_health,
        ARMOR: current_player_stats.armor,
        MAGIC_RESIST: current_player_stats.magic_resist,
        CRIT_CHANCE: current_player_stats.crit_chance,
        CRIT_DAMAGE: current_player_stats.crit_damage,
        ATTACK_SPEED: current_player_stats.attack_speed,
        MISSING_HEALTH: 1.0
            - (current_player_stats.current_health
                / current_player_stats.max_health.clamp(1.0, CLAMP_F64_MAX)),
        AP: current_player_stats.ability_power,
        AD: current_player_stats.attack_damage,
    }
}

/// Returns the difference between current stats and base stats
/// current_stats must be a tpe that can be converted to struct `RiotChampionStats`
#[inline]
pub fn get_bonus_stats(current_stats: BasicStats, base_stats: BasicStats) -> BasicStats {
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
pub fn get_base_stats(champion_cache: &&CachedChampion, level: usize) -> BasicStats {
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
    current_items: &[usize],
    earth_dragon_mod: f64,
) -> BasicStats {
    for enemy_item in current_items {
        if let Some(item) = GLOBAL_CACHE.items.get(&enemy_item) {
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
