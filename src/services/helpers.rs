use super::eval::{ConditionalAddition, MathEval};
use crate::{
    model::{
        base::{
            AttackType, BasicStats, ComparedDamage, ComparedItem, CurrentPlayerLike, DamageLike,
            DamageMultipliers, Damages, EnemyFullStats, FullStats, InstanceDamage, RealResists,
            SelfFullStats, SimulatedDamages, Stats, ToRiotFormat,
        },
        calculator::AbilitiesX,
        champions::Champion,
        internal::Positions,
        items::{Item, PartialStats},
        realtime::{DamageObject, DragonMultipliers},
        riot::RiotChampionStats,
        runes::Rune,
    },
    services::eval::RiotFormulas,
};
use std::{collections::HashMap, hash::Hash};

// By 06/07/2025 Earth dragons give +5% resists
// #![manual_impl]
pub const EARTH_DRAGON_MULTIPLIER: f64 = 0.05;
// By 06/07/2025 Fire dragons give +3% bonus attack stats
// #![manual_impl]
pub const FIRE_DRAGON_MULTIPLIER: f64 = 0.03;
// Chemtech Dragons will be used to calculate shields/healing/vamp
// #![unsupported]
// #![manual_impl]
pub const CHEMTECH_DRAGON_MULTIPLIER: f64 = 0.06;

pub struct GameStateCache<'a> {
    pub items: &'a HashMap<usize, Item>,
    pub runes: &'a HashMap<usize, Rune>,
}

pub struct GameStateCurrentPlayer<'a, T: CurrentPlayerLike> {
    pub thisv: &'a T,
    pub cache: &'a Champion,
    pub items: &'a Vec<usize>,
    pub runes: &'a Vec<usize>,
    pub abilities: &'a AbilitiesX,
    pub simulated_stats: &'a HashMap<usize, Stats>,
}

pub struct GameStateEnemyPlayer<'a> {
    pub base_stats: &'a mut BasicStats,
    pub current_stats: &'a mut BasicStats,
    pub items: &'a Vec<usize>,
    pub champion_id: &'a str,
    pub level: usize,
}

pub struct GameState<'a, T: CurrentPlayerLike> {
    pub cache: GameStateCache<'a>,
    pub current_player: GameStateCurrentPlayer<'a, T>,
    pub enemy_player: GameStateEnemyPlayer<'a>,
}

// Returns a cloned vector with the keys of the map.
pub fn clone_keys(map: &HashMap<usize, String>) -> Vec<usize> {
    map.keys().cloned().collect()
}

// All references may be dropped after this function is called.
// Removed excessive argument function
pub fn calculate_enemy_state<T: CurrentPlayerLike>(
    state: GameState<'_, T>,
) -> (Damages, RealResists, BasicStats) {
    let GameState {
        cache:
            GameStateCache {
                items: items_cache,
                runes: runes_cache,
            },
        current_player:
            GameStateCurrentPlayer {
                thisv: current_player,
                cache: current_player_cache,
                items: current_player_items_vec,
                runes: current_player_runes_vec,
                abilities,
                simulated_stats: simulated_champion_stats,
            },
        enemy_player:
            GameStateEnemyPlayer {
                base_stats: enemy_base_stats,
                current_stats: mut enemy_current_stats,
                items: enemy_items,
                champion_id: enemy_champion_id,
                level: enemy_level,
            },
    } = state;

    let mut enemy_bonus_stats: BasicStats = get_bonus_stats(enemy_current_stats, &enemy_base_stats);
    let full_stats: FullStats<'_> = get_full_stats(
        current_player,
        &current_player.get_current_stats(),
        (
            enemy_champion_id,
            enemy_level,
            &mut enemy_bonus_stats,
            &mut enemy_current_stats,
        ),
        &enemy_items,
    );
    let normal_abilities_damage: HashMap<String, InstanceDamage> =
        get_abilities_damage(current_player_cache, &full_stats, &abilities);
    let normal_items_damage: HashMap<usize, InstanceDamage> =
        get_items_damage(&items_cache, &full_stats, current_player_items_vec);
    let normal_runes_damage: HashMap<usize, InstanceDamage> =
        get_runes_damage(&runes_cache, &full_stats, current_player_runes_vec);
    let real_resists: RealResists = full_stats.enemy_player.real_resists;
    let compared_items: HashMap<usize, SimulatedDamages> = simulated_champion_stats
        .iter()
        .map(|(simulated_item_id, simulated_stats)| {
            let simulated_full_stats: FullStats<'_> = get_full_stats(
                current_player,
                &simulated_stats,
                (
                    enemy_champion_id,
                    enemy_level,
                    &mut enemy_bonus_stats,
                    &mut enemy_current_stats,
                ),
                &enemy_items,
            );
            let mut simulated_ability_damage: HashMap<String, InstanceDamage> =
                get_abilities_damage(&current_player_cache, &simulated_full_stats, &abilities);
            let mut simulated_item_damage: HashMap<usize, InstanceDamage> = get_items_damage(
                &items_cache,
                &simulated_full_stats,
                &current_player_items_vec,
            );
            let mut simulated_rune_damage: HashMap<usize, InstanceDamage> = get_runes_damage(
                &runes_cache,
                &simulated_full_stats,
                &current_player_runes_vec,
            );
            let (total_abilities_damage, change_abilities_damage) = get_comparison_total_damage(
                &normal_abilities_damage,
                &mut simulated_ability_damage,
            );
            let (total_items_damage, change_items_damage) =
                get_comparison_total_damage(&normal_items_damage, &mut simulated_item_damage);
            let (total_runes_damage, change_runes_damage) =
                get_comparison_total_damage(&normal_runes_damage, &mut simulated_rune_damage);
            (
                *simulated_item_id,
                SimulatedDamages {
                    abilities: ComparedDamage {
                        total: total_abilities_damage,
                        change: change_abilities_damage,
                        damages: simulated_ability_damage,
                    },
                    items: ComparedDamage {
                        total: total_items_damage,
                        change: change_items_damage,
                        damages: simulated_item_damage,
                    },
                    runes: ComparedDamage {
                        total: total_runes_damage,
                        change: change_runes_damage,
                        damages: simulated_rune_damage,
                    },
                },
            )
        })
        .collect::<HashMap<usize, SimulatedDamages>>();
    let damages: Damages = Damages {
        compared_items,
        abilities: normal_abilities_damage,
        items: normal_items_damage,
        runes: normal_runes_damage,
    };
    (damages, real_resists, enemy_bonus_stats)
}

// Takes each simulated item and clones the current champions stats
// and adds to these stats the value as if the player owned the item
// Supports multiple inputs, being that the reason why a HashMap is returned
// Mutates compared_items_info map, inserting the prettified stats, name and gold
// Dragon effects are taken into consideration
pub fn get_simulated_champion_stats(
    simulated_items: &Vec<usize>,
    owned_items: &Vec<usize>,
    current_stats: &Stats,
    items_cache: &HashMap<usize, Item>,
    ally_dragon_multipliers: &DragonMultipliers,
    compared_items_info: &mut HashMap<usize, ComparedItem>,
) -> Result<HashMap<usize, Stats>, String> {
    simulated_items
        .iter()
        .filter(|this_item: &&usize| !owned_items.contains(this_item))
        .map(|simulated_item_id: &usize| {
            let simulated_item_cache: &Item =
                items_cache.get(simulated_item_id).ok_or_else(|| {
                    format!(
                        "Simulated item {} not found in items cache",
                        simulated_item_id
                    )
                })?;
            let mut cloned_champion_stats: Stats = current_stats.clone();
            compared_items_info.insert(
                *simulated_item_id,
                ComparedItem {
                    name: simulated_item_cache.name.clone(),
                    gold_cost: simulated_item_cache.gold,
                    prettified_stats: simulated_item_cache.pretiffied_stats.clone(),
                },
            );
            simulate_champion_stats(
                *simulated_item_id,
                simulated_item_cache,
                &mut cloned_champion_stats,
                &owned_items,
                &ally_dragon_multipliers,
            );
            Ok((*simulated_item_id, cloned_champion_stats))
        })
        .collect()
}

// Checks cache and returns a map with the matching keys and their names.
// Basic attacks and critical strikes are always included.
// Ignores key that is related to Monster or Minion damage
// #![unsupported] Minions and Monster damages are not evaluated
// In the future a table to represent damage on monsters/towers may be created
pub fn get_damaging_abilities(champion_cache: &Champion) -> HashMap<String, String> {
    let mut damaging_abilities: HashMap<String, String> = champion_cache
        .abilities
        .iter()
        .filter_map(|(key, ability)| {
            (!key.contains("MONSTER") && !key.contains("MINION"))
                .then(|| (key.clone(), ability.name.clone()))
        })
        .collect();
    damaging_abilities.extend([
        ("A".to_string(), "Basic Attack".to_string()),
        ("C".to_string(), "Critical Strike".to_string()),
    ]);
    damaging_abilities
}

// Items may or may not contain damage information. Returns items that do
// Returns a map with their matching keys and item name.
// Every item should have a matching value for both ranged and melee champions
// For safety, damaging items are evaluated only if item contains a key that matches attack type
// Common exceptions for this error would be in items that are not purchasable for melee champions
// Even in this situation, it is recommended to add damage value for both attack types
pub fn get_damaging_items(
    items_cache: &HashMap<usize, Item>,
    attack_type: AttackType,
    owned_items: &Vec<usize>,
) -> HashMap<usize, String> {
    owned_items
        .iter()
        .filter_map(|item_id: &usize| {
            let item: &Item = items_cache.get(&item_id)?;
            let ok: bool = match attack_type {
                AttackType::Melee => item.melee.is_some(),
                AttackType::Ranged => item.ranged.is_some(),
                AttackType::Other => false,
            };
            ok.then(|| (*item_id, item.name.clone()))
        })
        .collect()
}

// Returns ref that should live longer than the function
// Takes player position and checks `meta_items.json`
// May return None. When there are not enough data to evaluate
// Most common items for a champion in a given position, it may be empty
// In case it returns None, an empty <Vec> should be used.
// This function can't return an owned Vec since it moves `recommendations`
pub fn get_recommended_items<'a>(
    positon: &str,
    recommendations: &'a Positions,
) -> Option<&'a Vec<usize>> {
    match positon {
        "TOP" => Some(&recommendations.top),
        "JUNGLE" => Some(&recommendations.jungle),
        "MIDDLE" => Some(&recommendations.mid),
        "BOTTOM" => Some(&recommendations.adc),
        "SUPPORT" => Some(&recommendations.support),
        _ => None,
    }
}

// Receives a deep copy of the original champion stats and mutates its reference
// Takes an item and adds to this cloned mut ref the stats as if the player owned it
// If player already owns the item, this function does nothing
pub fn simulate_champion_stats(
    simulated_item_id: usize,
    simulated_item_cache: &Item,
    cloned_stats: &mut Stats,
    current_owned_items: &Vec<usize>,
    ally_dragon_multipliers: &DragonMultipliers,
) {
    let stats: &PartialStats = &simulated_item_cache.stats;
    if current_owned_items.contains(&simulated_item_id) {
        return;
    }
    cloned_stats.ability_power.add_if_some(stats.ability_power);
    cloned_stats.attack_damage.add_if_some(stats.attack_damage);
    cloned_stats.max_health.add_if_some(stats.health);
    cloned_stats.armor.add_if_some(stats.armor);
    cloned_stats
        .magic_resist
        .add_if_some(stats.magic_resistance);
    cloned_stats.max_mana.add_if_some(stats.mana);
    cloned_stats.attack_speed.add_if_some(stats.attack_speed);
    cloned_stats
        .crit_chance
        .add_if_some(stats.critical_strike_chance);
    cloned_stats
        .crit_damage
        .add_if_some(stats.critical_strike_damage);
    cloned_stats
        .armor_penetration_flat
        .add_if_some(stats.armor_penetration_flat);
    cloned_stats
        .armor_penetration_percent
        .add_if_some(stats.armor_penetration_percent);
    cloned_stats
        .magic_penetration_flat
        .add_if_some(stats.magic_penetration_flat);
    cloned_stats
        .magic_penetration_percent
        .add_if_some(stats.magic_penetration_percent);
    cloned_stats.ability_power *= ally_dragon_multipliers.fire;
    cloned_stats.attack_damage *= ally_dragon_multipliers.fire;
    cloned_stats.armor *= ally_dragon_multipliers.earth;
    cloned_stats.magic_resist *= ally_dragon_multipliers.earth;
}

// Reads items from cache and evaluates its damages, if some.
pub fn get_items_damage(
    items_cache: &HashMap<usize, Item>,
    stats: &FullStats,
    current_player_items_vec: &Vec<usize>,
) -> DamageLike<usize> {
    let mut item_damages: DamageLike<usize> = DamageLike::<usize>::new();

    let is_ranged: bool = stats.current_player.is_ranged;
    for current_player_item in current_player_items_vec.into_iter() {
        if let Some(item) = items_cache.get(current_player_item) {
            let item_damage: &Option<DamageObject> = match is_ranged {
                true => &item.ranged,
                false => &item.melee,
            };
            if let Some(damage) = item_damage {
                let damage_type: String =
                    item.damage_type.clone().unwrap_or(String::from("UNKNOWN"));
                let (damage_reduction_multiplier, damage_increase_multiplier) =
                    get_damage_multipliers(stats, &damage_type);
                let minimum_damage_string: String = format!(
                    "({}) * {}",
                    replace_damage_keywords(
                        stats,
                        &damage.minimum_damage.clone().unwrap_or_default(),
                    ),
                    damage_reduction_multiplier,
                );
                let maximum_damage_string: String = format!(
                    "({}) * {}",
                    replace_damage_keywords(
                        stats,
                        &damage.maximum_damage.clone().unwrap_or_default(),
                    ),
                    damage_reduction_multiplier,
                );
                let minimum_damage: f64 =
                    damage_increase_multiplier * &minimum_damage_string.eval().unwrap_or_default();
                let maximum_damage: f64 =
                    damage_increase_multiplier * &maximum_damage_string.eval().unwrap_or_default();
                item_damages.insert(
                    *current_player_item,
                    InstanceDamage {
                        minimum_damage,
                        maximum_damage,
                        damage_type,
                        damages_in_area: false,
                        damages_onhit: item.damages_onhit,
                        min_dmg_change: None,
                        max_dmg_change: None,
                    },
                );
            }
        }
    }
    item_damages
}

// Reads runes from cache and evaluates its damages, if some.
pub fn get_runes_damage(
    runes_cache: &HashMap<usize, Rune>,
    stats: &FullStats,
    current_player_runes_vec: &Vec<usize>,
) -> DamageLike<usize> {
    let mut rune_damages: DamageLike<usize> = DamageLike::<usize>::new();

    let is_ranged: bool = stats.current_player.is_ranged;
    for current_player_rune in current_player_runes_vec.into_iter() {
        if let Some(rune) = runes_cache.get(current_player_rune) {
            let rune_damage: &String = match is_ranged {
                true => &rune.ranged,
                false => &rune.melee,
            };
            let damage_type: String = rune.damage_type.clone();
            let (damage_reduction_multiplier, damage_increase_multiplier) =
                get_damage_multipliers(stats, &damage_type);
            let damage_string: String = format!(
                "({}) * {}",
                replace_damage_keywords(stats, rune_damage),
                damage_reduction_multiplier
            );
            let minimum_damage: f64 =
                damage_increase_multiplier * &damage_string.eval().unwrap_or_default();
            rune_damages.insert(
                *current_player_rune,
                InstanceDamage {
                    minimum_damage,
                    maximum_damage: 0.0,
                    damage_type,
                    damages_in_area: false,
                    damages_onhit: false,
                    min_dmg_change: None,
                    max_dmg_change: None,
                },
            );
        }
    }
    rune_damages
}

// Sums all the damages in every key of a DamageLike expression
// Returns the total and the change in total damage respectively
pub fn get_comparison_total_damage<T: Eq + Hash>(
    prev: &DamageLike<T>,
    next: &mut DamageLike<T>,
) -> (f64, f64) {
    let mut sum: f64 = 0f64;
    for (key, val) in next.iter_mut() {
        sum += val.minimum_damage + val.maximum_damage;
        if let Some(prev_val) = prev.get(key) {
            val.min_dmg_change = Some(val.minimum_damage - prev_val.minimum_damage);
            val.max_dmg_change = Some(val.maximum_damage - prev_val.maximum_damage);
        }
    }
    (
        sum,
        sum - prev
            .iter()
            .map(|(_, value)| value.maximum_damage + value.minimum_damage)
            .sum::<f64>(),
    )
}

// Creates a big struct containing all the stats of the current player and the enemy
// Part of the returned value contains reference to the arguments passed to this function
pub fn get_full_stats<'a, T: CurrentPlayerLike>(
    current_player: &'a T,
    current_stats: &'a Stats,
    enemy_state: (&'a str, usize, &'a mut BasicStats, &'a mut BasicStats),
    enemy_items: &'a Vec<usize>,
) -> FullStats<'a> {
    let (enemy_champion_id, enemy_level, enemy_bonus_stats, enemy_current_stats) = enemy_state;
    let mut real_armor: f64 = enemy_current_stats.armor * current_stats.armor_penetration_percent
        - current_stats.armor_penetration_flat;
    let mut real_magic_resist: f64 = enemy_current_stats.magic_resist
        * current_stats.magic_penetration_percent
        - current_stats.magic_penetration_flat;

    real_armor = real_armor.clamp(0.0, (1 << 32) as f64);
    real_magic_resist = real_magic_resist.clamp(0.0, (1 << 32) as f64);

    let physical_damage_multiplier: f64 = 100.0 / (100.0 + real_armor);
    let magic_damage_multiplier: f64 = 100.0 / (100.0 + real_magic_resist);

    let has_item = |item: &[usize], check_for: &[usize]| -> bool {
        check_for.iter().any(|id: &usize| item.contains(id))
    };

    // Translates to (physical, magic, true, all_sources)
    let mut enemy_dmg_mod: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 1.0);
    let self_dmg_mod: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 1.0);

    match enemy_champion_id {
        "Kassadin" => {
            // #![manual_impl]
            enemy_dmg_mod.1 = 0.9;
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
            enemy_current_stats.armor *= ornn_resist_multiplier;
            enemy_current_stats.magic_resist *= ornn_resist_multiplier;
            enemy_current_stats.health *= ornn_resist_multiplier;
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
        }
        _ => {}
    }

    FullStats {
        physical_damage_multiplier,
        magic_damage_multiplier,
        missing_health: 1.0 - (current_stats.current_health / current_stats.max_health),
        // #![manual_impl]
        enemy_has_steelcaps: has_item(enemy_items, &[3047]),
        // #![manual_impl]
        enemy_has_rocksolid: has_item(enemy_items, &[3082, 3110, 3143]),
        // #![manual_impl]
        enemy_has_randuin: has_item(enemy_items, &[3143]),
        current_player: SelfFullStats {
            current_stats: &current_stats,
            base_stats: current_player.get_base_stats(),
            bonus_stats: current_player.get_bonus_stats(),
            // #![manual_impl]
            // This may not be valid anymore.
            // Melee champions today can reach an attack range > 350.0
            // In this scenario, they will be falsely marked as ranged
            // #![unsafe_impl]
            is_ranged: current_stats.attack_range > 350.0,
            level: current_player.get_level(),
            // #![unsupported] only permanent modifiers are supported
            damage_mod: DamageMultipliers {
                magic_damage: self_dmg_mod.0,
                physical_damage: self_dmg_mod.1,
                true_damage: self_dmg_mod.2,
                all_sources: self_dmg_mod.3,
            },
            is_physical_adaptative_type: 0.35 * current_player.get_bonus_stats().attack_damage
                >= 0.2 * current_stats.ability_power,
        },
        enemy_player: EnemyFullStats {
            current_stats: enemy_current_stats,
            bonus_stats: enemy_bonus_stats,
            // #![unsupported] only permanent modifiers are supported
            damage_mod: DamageMultipliers {
                magic_damage: enemy_dmg_mod.0,
                physical_damage: enemy_dmg_mod.1,
                true_damage: enemy_dmg_mod.2,
                all_sources: enemy_dmg_mod.3,
            },
            real_resists: RealResists {
                armor: real_armor,
                magic_resist: real_magic_resist,
            },
        },
    }
}

// Checks damage type and returns its multipliers. Usually a number
// If a debuff is found, values returned will be < 1.0, else >= 1.0
// return[0]: value that will replace PHYSICAL_MULTIPLIER or MAGIC_MULTIPLIER
// return[1]: value in decimal form that represents percentage of bonus/debuff damage
pub fn get_damage_multipliers(full_stats: &FullStats, damage_type: &str) -> (f64, f64) {
    let enemy_damage_multipliers: &DamageMultipliers = &full_stats.enemy_player.damage_mod;
    let (enemy_debuff_multiplier, damage_reduction_multiplier, damage_increase_multiplier) =
        match damage_type {
            "MAGIC_DAMAGE" => (
                full_stats.enemy_player.damage_mod.magic_damage,
                full_stats.magic_damage_multiplier,
                full_stats.current_player.damage_mod.magic_damage,
            ),
            "PHYSICAL_DAMAGE" => (
                full_stats.enemy_player.damage_mod.physical_damage,
                full_stats.physical_damage_multiplier,
                full_stats.current_player.damage_mod.physical_damage,
            ),
            "TRUE_DAMAGE" => (
                full_stats.enemy_player.damage_mod.true_damage,
                1.0,
                full_stats.current_player.damage_mod.true_damage,
            ),
            _ => (1.0, 1.0, 1.0),
        };
    let damage_bonus: f64 = full_stats.current_player.damage_mod.all_sources;
    (
        damage_reduction_multiplier,
        enemy_debuff_multiplier
            * damage_increase_multiplier
            * damage_bonus
            * enemy_damage_multipliers.all_sources,
    )
}

// Evaluates abilities damages to a given champion, whose stats are in `full_stats`
// Intentionally ignores Monster and Minion damages
// Invalid inputs will cause the function to return an empty HashMap
pub fn get_abilities_damage(
    current_player_cache: &Champion,
    full_stats: &FullStats,
    abilities: &AbilitiesX,
) -> DamageLike<String> {
    let mut ability_damages: DamageLike<String> = DamageLike::<String>::new();
    for (keyname, ability) in current_player_cache
        .abilities
        .iter()
        .filter_map(|(key, val)| {
            (!key.contains("MONSTER") && !key.contains("MINION")).then(|| (key.clone(), val))
        })
    {
        let first_char: char = keyname.chars().next().unwrap_or_default();
        let indexation: usize = match first_char {
            'P' => full_stats.current_player.level,
            _ => match first_char {
                'Q' => abilities.q,
                'W' => abilities.w,
                'E' => abilities.e,
                'R' => abilities.r,
                _ => return ability_damages,
            },
        };
        let damages_in_area: bool = ability.damages_in_area;
        let damage_type: String = ability.damage_type.clone();
        if indexation > 0 {
            let (damage_reduction_multiplier, damage_increase_multiplier) =
                get_damage_multipliers(full_stats, &damage_type);
            let eval_damage_str = |from_vec: &Vec<String>| {
                let default_value: String = String::from("0.0");
                let format_str: &String = from_vec.get(indexation - 1).unwrap_or(&default_value);
                let damage_str: String = replace_damage_keywords(full_stats, format_str);
                let formatted_str: String =
                    format!("({}) * {}", damage_str, damage_reduction_multiplier);
                (damage_increase_multiplier) * &formatted_str.eval().unwrap_or_default()
            };

            let minimum_damage: f64 = eval_damage_str(&ability.minimum_damage);
            let maximum_damage: f64 = eval_damage_str(&ability.maximum_damage);
            ability_damages.insert(
                keyname.to_string(),
                InstanceDamage {
                    minimum_damage,
                    maximum_damage,
                    damage_type: String::from(damage_type),
                    damages_in_area,
                    damages_onhit: false,
                    min_dmg_change: None,
                    max_dmg_change: None,
                },
            );
        } else {
            // For abilities at level zero, return default values right away
            // Avoids processing unnecessary math expressions
            ability_damages.insert(
                keyname.to_string(),
                InstanceDamage {
                    minimum_damage: 0.0,
                    maximum_damage: 0.0,
                    damage_type,
                    damages_in_area,
                    damages_onhit: false,
                    min_dmg_change: None,
                    max_dmg_change: None,
                },
            );
        }
    }
    // Some champions may have a range value in their basic attacks, or a different damage type
    // Champions listed below are considered to be "normal" while exceptions are not implemented
    // #![unsupported] Senna, Akshan, Corki, Kalista
    let basic_attack_damage: f64 = full_stats.current_player.current_stats.attack_damage
        * full_stats.physical_damage_multiplier
        * full_stats.current_player.damage_mod.physical_damage;
    ability_damages.insert(
        String::from("A"),
        InstanceDamage {
            minimum_damage: basic_attack_damage,
            maximum_damage: 0.0,
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
            damages_onhit: false,
            min_dmg_change: None,
            max_dmg_change: None,
        },
    );
    // Same issue as basic attack description above
    ability_damages.insert(
        String::from("C"),
        InstanceDamage {
            minimum_damage: basic_attack_damage
                * (full_stats.current_player.current_stats.crit_damage / 100.0),
            maximum_damage: 0.0,
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
            damages_onhit: false,
            min_dmg_change: None,
            max_dmg_change: None,
        },
    );

    ability_damages
}

// Takes a string and replaces keywords defined manually.
// Expected to return a string that can be evaluted mathematically, but not guaranteed
// Stacks information is not accessible unless sent from client.
// #![unsupported] Champion stacks are ignored.
pub fn replace_damage_keywords(stats: &FullStats, target_str: &str) -> String {
    let replacements: [(&'static str, f64); 48] = [
        ("CHOGATH_STACKS", 1.0),
        ("VEIGAR_STACKS", 1.0),
        ("NASUS_STACKS", 1.0),
        ("SMOLDER_STACKS", 1.0),
        ("AURELION_SOL_STACKS", 1.0),
        ("THRESH_STACKS", 1.0),
        ("KINDRED_STACKS", 1.0),
        ("BELVETH_STACKS", 1.0),
        (
            "ADAPTATIVE_DAMAGE",
            if stats.current_player.is_physical_adaptative_type {
                stats.physical_damage_multiplier
            } else {
                stats.magic_damage_multiplier
            },
        ),
        ("MISSING_HEALTH", stats.missing_health),
        ("LEVEL", stats.current_player.level as f64),
        ("PHYSICAL_MULTIPLIER", stats.physical_damage_multiplier),
        ("MAGIC_MULTIPLIER", stats.magic_damage_multiplier),
        // #![manual_impl]
        (
            "STEELCAPS_EFFECT",
            if stats.enemy_has_steelcaps { 0.88 } else { 1.0 },
        ),
        // #![manual_impl]
        (
            "RANDUIN_EFFECT",
            if stats.enemy_has_randuin { 0.7 } else { 1.0 },
        ),
        // #![manual_impl]
        (
            "ROCKSOLID_EFFECT",
            if stats.enemy_has_rocksolid { 0.8 } else { 1.0 },
        ),
        ("ENEMY_BONUS_HEALTH", stats.enemy_player.bonus_stats.health),
        ("ENEMY_ARMOR", stats.enemy_player.current_stats.armor),
        ("ENEMY_HEALTH", stats.enemy_player.current_stats.health),
        (
            "ENEMY_CURRENT_HEALTH",
            stats.enemy_player.current_stats.health,
        ),
        (
            "ENEMY_MISSING_HEALTH",
            stats.enemy_player.current_stats.health,
        ),
        (
            "ENEMY_MAGIC_RESIST",
            stats.enemy_player.current_stats.magic_resist,
        ),
        ("BASE_HEALTH", stats.current_player.base_stats.health),
        ("BASE_AD", stats.current_player.base_stats.attack_damage),
        ("BASE_ARMOR", stats.current_player.base_stats.armor),
        (
            "BASE_MAGIC_RESIST",
            stats.current_player.base_stats.magic_resist,
        ),
        ("BASE_MANA", stats.current_player.base_stats.mana),
        ("BONUS_AD", stats.current_player.bonus_stats.attack_damage),
        ("BONUS_ARMOR", stats.current_player.bonus_stats.armor),
        (
            "BONUS_MAGIC_RESIST",
            stats.current_player.bonus_stats.magic_resist,
        ),
        ("BONUS_HEALTH", stats.current_player.bonus_stats.health),
        ("BONUS_MANA", stats.current_player.bonus_stats.mana),
        // #![unsupported]
        ("BONUS_MOVE_SPEED", 1.0),
        ("AP", stats.current_player.current_stats.ability_power),
        ("AD", stats.current_player.current_stats.attack_damage),
        (
            "ARMOR_PENETRATION_FLAT",
            stats.current_player.current_stats.armor_penetration_flat,
        ),
        (
            "ARMOR_PENETRATION_PERCENT",
            stats.current_player.current_stats.armor_penetration_percent,
        ),
        (
            "MAGIC_PENETRATION_FLAT",
            stats.current_player.current_stats.magic_penetration_flat,
        ),
        (
            "MAGIC_PENETRATION_PERCENT",
            stats.current_player.current_stats.magic_penetration_percent,
        ),
        ("MAX_MANA", stats.current_player.current_stats.max_mana),
        (
            "CURRENT_MANA",
            stats.current_player.current_stats.current_mana,
        ),
        ("MAX_HEALTH", stats.current_player.current_stats.max_health),
        (
            "CURRENT_HEALTH",
            stats.current_player.current_stats.current_health,
        ),
        ("ARMOR", stats.current_player.current_stats.armor),
        (
            "MAGIC_RESIST",
            stats.current_player.current_stats.magic_resist,
        ),
        (
            "CRIT_CHANCE",
            stats.current_player.current_stats.crit_chance,
        ),
        (
            "CRIT_DAMAGE",
            stats.current_player.current_stats.crit_damage,
        ),
        (
            "ATTACK_SPEED",
            stats.current_player.current_stats.attack_speed,
        ),
    ];

    replacements
        .iter()
        .fold(target_str.to_string(), |acc: String, (old, new)| {
            acc.replace(old, &new.to_string())
        })
}

// Returns the difference between current stats and base stats
// current_stats must be a tpe that can be converted to struct `RiotChampionStats`
pub fn get_bonus_stats<T: ToRiotFormat>(current_stats: &T, base_stats: &BasicStats) -> BasicStats {
    let value: RiotChampionStats = current_stats.format();
    BasicStats {
        armor: value.armor - base_stats.armor,
        health: value.max_health - base_stats.health,
        attack_damage: value.attack_damage - base_stats.attack_damage,
        magic_resist: value.magic_resist - base_stats.magic_resist,
        mana: value.resource_max - base_stats.mana,
    }
}

// Reads cached values for a given champion and assigns its base stats at a given level
pub fn get_base_stats(champion_cache: &Champion, level: usize) -> BasicStats {
    BasicStats {
        armor: RiotFormulas::stat_growth(
            champion_cache.stats.armor.flat,
            champion_cache.stats.armor.per_level,
            level,
        ),
        health: RiotFormulas::stat_growth(
            champion_cache.stats.health.flat,
            champion_cache.stats.health.per_level,
            level,
        ),
        attack_damage: RiotFormulas::stat_growth(
            champion_cache.stats.attack_damage.flat,
            champion_cache.stats.attack_damage.per_level,
            level,
        ),
        magic_resist: RiotFormulas::stat_growth(
            champion_cache.stats.magic_resistance.flat,
            champion_cache.stats.magic_resistance.per_level,
            level,
        ),
        mana: RiotFormulas::stat_growth(
            champion_cache.stats.mana.flat,
            champion_cache.stats.mana.per_level,
            level,
        ),
    }
}

// Reads enemy player's items and base stats
// Return value may not match the in-game value due to runes/stacks
// There are several other situations where enemy current stats
// Can't be evaluated precisely.
pub fn get_enemy_current_stats(
    champion_cache: &HashMap<usize, Item>,
    base_stats: &BasicStats,
    current_items: &Vec<usize>,
    earth_dragon: f64,
) -> BasicStats {
    let mut base: BasicStats = BasicStats {
        armor: base_stats.armor,
        health: base_stats.health,
        attack_damage: base_stats.attack_damage,
        magic_resist: base_stats.magic_resist,
        mana: base_stats.mana,
    };
    for enemy_item in current_items {
        if let Some(item) = champion_cache.get(&enemy_item) {
            let stats: &PartialStats = &item.stats;
            base.armor.add_if_some(stats.armor);
            base.health.add_if_some(stats.health);
            base.attack_damage.add_if_some(stats.attack_damage);
            base.magic_resist.add_if_some(stats.magic_resistance);
            base.mana.add_if_some(stats.mana);
        }
    }
    base.armor *= earth_dragon;
    base.magic_resist *= earth_dragon;
    base
}
