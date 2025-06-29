use super::*;
use crate::{
    INTERNAL_CHAMPIONS, INTERNAL_ITEMS, INTERNAL_NAMES, INTERNAL_RUNES, META_ITEMS,
    model::{base::*, calculator::*},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;

/// If user opted not to dictate the active player's stats, this function is called
/// it reads all items present in cache and evaluates what the game condition would be
/// if all items were owned in a real game. Stacks and champion exceptions are partially
/// taken into consideration when calculation is made. It is less accurate than Realtime.
fn apply_auto_stats(
    stats: &mut Stats,
    active_player: &InputActivePlayer,
    base_stats: BasicStats,
) -> Result<BasicStats, CalculationError> {
    let stacks = active_player.stacks as f64;
    let owned_items = &active_player.items;

    let mut armor_penetration = Vec::<f64>::new();
    let mut magic_penetration = Vec::<f64>::new();

    for item_id in owned_items {
        let cached_item = INTERNAL_ITEMS
            .get(&item_id)
            .ok_or_else(|| CalculationError::ItemCacheNotFound(*item_id))?;

        let item_stats = &cached_item.stats;

        macro_rules! add_stat {
            ($field:ident) => {
                stats.$field += item_stats.$field;
            };
            ($field:ident, $field2:ident) => {
                stats.$field += item_stats.$field2;
            };
        }

        add_stat!(ability_power);
        add_stat!(attack_damage);
        add_stat!(armor);
        add_stat!(magic_resist, magic_resistance);
        add_stat!(max_health, health);
        add_stat!(crit_chance, critical_strike_chance);
        add_stat!(crit_damage, critical_strike_damage);
        add_stat!(max_mana, mana);
        add_stat!(attack_speed);

        armor_penetration.push(item_stats.armor_penetration_percent);
        magic_penetration.push(item_stats.magic_penetration_percent);
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);

    let bonus_stats = get_bonus_stats(
        BasicStats {
            armor: stats.armor,
            health: stats.max_health,
            attack_damage: stats.attack_damage,
            magic_resist: stats.magic_resist,
            mana: stats.max_mana,
        },
        base_stats,
    );

    // #![manual_impl]
    // #![todo] File generator
    // Depend on bonus stats, that has to be evaluated later
    for item_id in owned_items {
        match item_id {
            // Overlord's Bloodmail
            2501 | 222501 => stats.attack_damage += 0.025 * (bonus_stats.health + 500.0),
            // Archangel's Staff
            3003 | 223003 => stats.ability_power += 0.01 * bonus_stats.mana,
            // Manamune | Muramana
            3004 | 3042 | 223004 | 223042 => stats.attack_damage += 0.025 * bonus_stats.mana,
            // Seraph's Embrace
            3040 | 223040 => stats.ability_power += 0.02 * bonus_stats.mana,
            // Winter's Approach
            3119 | 223119 => stats.max_health += 0.15 * (bonus_stats.health + 500.0),
            // Fimbulwinter
            3121 | 223121 => stats.max_health += 0.15 * (bonus_stats.health + 860.0),
            // Riftmaker | Demonic Embrace
            4633 | 4637 | 224633 | 224637 => {
                stats.ability_power += 0.02 * (bonus_stats.health + stats.max_health)
            }
            _ => {}
        }
    }

    // #![manual_impl]
    // #![todo] Create exception file that is generated automatically
    match active_player.champion_id.as_str() {
        "Veigar" => stats.ability_power += stacks,
        "Chogath" => {
            stats.max_health += stacks * 80.0 + 40.0 * active_player.abilities.r.clamp(0, 3) as f64;
        }
        "Sion" => stats.max_health += stacks,
        "Darius" => {
            armor_penetration.push(15.0 + 5.0 * active_player.abilities.e as f64);
        }
        "Pantheon" => {
            armor_penetration.push(10.0 * active_player.abilities.r as f64);
        }
        "Nilah" => {
            armor_penetration.push(stats.crit_chance / 3.0);
        }
        "Mordekaiser" => {
            magic_penetration.push(2.5 + 2.5 * active_player.abilities.e as f64);
        }
        _ => {}
    }

    stats.armor_penetration_percent = RiotFormulas::percent_value(armor_penetration);
    stats.magic_penetration_percent = RiotFormulas::percent_value(magic_penetration);

    Ok(bonus_stats)
}

// #![manual_impl]
// #![unsupported]
fn rune_exceptions(
    champion_stats: &mut Stats,
    owned_runes: &[usize],
    level: f64,
    exception_map: &FxHashMap<usize, usize>,
    value_types: (AdaptativeType, AttackType),
) {
    for rune in owned_runes {
        let this_stack = *exception_map.get(&rune).unwrap_or(&0);
        match rune {
            // Lethal Tempo
            8008 => match value_types.1 {
                AttackType::Melee => {
                    champion_stats.attack_speed +=
                        (this_stack as f64) * (5.0 + 11.0 / 17.0 * (level - 1.0));
                }
                AttackType::Ranged => {
                    champion_stats.attack_speed +=
                        (this_stack as f64) * (3.6 + 4.4 / 17.0 * (level - 1.0));
                }
            },
            // Conqueror
            8010 => {
                let formula: f64 = (this_stack as f64) * (1.8 + 2.2 / 17.0 * (level - 1.0));
                match value_types.0 {
                    AdaptativeType::Physical => {
                        champion_stats.attack_damage += 0.6 * formula;
                    }
                    AdaptativeType::Magic => {
                        champion_stats.ability_power += formula;
                    }
                }
            }
            // Eyeball Collection | Ghost Poro | Zombie Ward :: Removed Runes
            8120 | 8136 | 8138 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += match this_stack {
                        0..10 => 1.2 * (this_stack as f64),
                        _ => 18.0,
                    };
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += match this_stack {
                        0..10 => (this_stack << 1) as f64,
                        _ => 30.0,
                    };
                }
            },
            // Waterwalking
            8232 => {
                champion_stats.ability_power += 12.0 + level;
                champion_stats.attack_damage += 7.2 + 0.6 * level
            }
            // Absolute Focus
            8233 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += 1.8 + 16.2 / 17.0 * (level - 1.0);
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += 3.0 + 27.0 / 17.0 * (level - 1.0);
                }
            },
            // Gathering Storm
            8236 => {
                let formula: f64 = ((this_stack * (this_stack + 1)) << 2) as f64;
                match value_types.0 {
                    AdaptativeType::Physical => {
                        champion_stats.attack_damage += 0.6 * formula;
                    }
                    AdaptativeType::Magic => {
                        champion_stats.ability_power += formula;
                    }
                }
            }
            // Adaptative damage shard
            9000 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += 5.4 * (this_stack as f64);
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += 9.0 * (this_stack as f64);
                }
            },
            // Max health shard
            9001 => champion_stats.max_health += 65.0 * (this_stack as f64),
            // Health per level shard
            9002 => champion_stats.max_health += 10.0 * level * (this_stack as f64),
            // Attack speed shard
            9003 => champion_stats.attack_speed += 10.0 * (this_stack as f64),
            _ => {}
        }
    }
}

// #![manual_impl]
// #![unsupported]
fn item_exceptions(
    champion_stats: &mut Stats,
    owned_items: &[usize],
    exception_map: &FxHashMap<usize, usize>,
) {
    for item_id in owned_items {
        let this_stack = *exception_map.get(&item_id).unwrap_or(&0);
        match item_id {
            // Dark Seal
            1082 => champion_stats.ability_power += (this_stack.max(1) << 2) as f64,
            // Mejai's Soulstealer
            3041 => champion_stats.ability_power += (5 * this_stack.max(1)) as f64,
            // Rabadon's Deathcap
            3089 | 223089 => champion_stats.ability_power *= 1.3,
            // Hubris
            6697 | 7008 => champion_stats.attack_damage += (15 + this_stack.max(1) << 1) as f64,
            // Wooglet's Witchcap
            8002 => champion_stats.ability_power *= 1.5,
            _ => {}
        }
    }
}

// #![todo] Comparison tool is not reliable;
// #![todo] Stats are not assigned correctly
// #![todo] Review exceptions
#[writer_macros::trace_time]
pub fn calculator(game: InputGame) -> Result<OutputGame, CalculationError> {
    let InputGame {
        active_player,
        enemy_players,
        ally_fire_dragons,
        ally_earth_dragons,
        enemy_earth_dragons,
        stack_exceptions,
    } = game;

    let InputActivePlayer {
        level: current_player_level,
        champion_id: ref current_player_champion_id,
        abilities: current_player_abilities,
        // #![todo]
        champion_stats: ref _current_player_stats,
        runes: ref current_player_runes,
        items: ref current_player_items,
        infer_stats: current_player_infer_stats,
        // #![todo]
        stacks: _current_player_stacks,
    } = active_player;

    let current_player_cache = INTERNAL_CHAMPIONS.get(current_player_champion_id).ok_or(
        CalculationError::ChampionCacheNotFound(format!(
            "[INTERNAL_CHAMPIONS]: {}",
            current_player_champion_id
        )),
    )?;

    let mut current_player_stats =
        RiotFormulas::full_base_stats(&current_player_cache.stats, current_player_level);

    let current_player_base_stats = get_base_stats(current_player_cache, current_player_level);
    let current_player_basic_stats = BasicStats {
        armor: current_player_stats.armor,
        health: current_player_stats.max_health,
        attack_damage: current_player_stats.attack_damage,
        magic_resist: current_player_stats.magic_resist,
        mana: current_player_stats.max_mana,
    };

    let current_player_bonus_stats = if current_player_infer_stats {
        apply_auto_stats(
            &mut current_player_stats,
            &active_player,
            current_player_base_stats,
        )?
    } else {
        get_bonus_stats(current_player_basic_stats, current_player_base_stats)
    };

    let current_player_attack_type = AttackType::from_str(current_player_cache.attack_type);
    let current_player_champion_id = active_player.champion_id;

    item_exceptions(
        &mut current_player_stats,
        current_player_items,
        &stack_exceptions,
    );

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        current_player_stats.ability_power,
    );

    rune_exceptions(
        &mut current_player_stats,
        current_player_runes,
        current_player_level as f64,
        &stack_exceptions,
        (adaptative_type, current_player_attack_type),
    );

    let current_player_recommended_items = META_ITEMS
        .get(&current_player_champion_id)
        .and_then(|meta_items| {
            current_player_cache
                .positions
                .get(0)
                .map(|position| match *position {
                    "TOP" => meta_items.top,
                    "JUNGLE" => meta_items.jungle,
                    "MIDDLE" => meta_items.mid,
                    "BOTTOM" => meta_items.adc,
                    "SUPPORT" => meta_items.support,
                    _ => &[],
                })
        })
        .unwrap_or(&[]);

    let ally_dragon_multipliers = DragonMultipliers {
        fire: 1.0 + FIRE_DRAGON_MULTIPLIER * ally_fire_dragons as f64,
        earth: 1.0 + EARTH_DRAGON_MULTIPLIER * ally_earth_dragons as f64,
        chemtech: 1.0,
    };
    let enemy_dragon_multipliers = DragonMultipliers {
        fire: 1.0,
        earth: 1.0 + EARTH_DRAGON_MULTIPLIER * enemy_earth_dragons as f64,
        chemtech: 1.0,
    };

    let (simulated_stats, compared_items) = get_simulated_champion_stats(
        &current_player_stats,
        current_player_items,
        &ally_dragon_multipliers,
    );

    let current_player_state = (
        &current_player_stats,
        current_player_bonus_stats,
        current_player_base_stats,
        current_player_level,
    );

    let abilities_iter_expr = get_abilities_damage(
        current_player_cache,
        current_player_level,
        current_player_abilities,
    );
    let items_iter_expr = get_items_damage(current_player_items, current_player_attack_type);
    let runes_iter_expr = get_runes_damage(current_player_runes, current_player_attack_type);

    let enemies = enemy_players
        .into_par_iter()
        .map(|player| {
            let InputEnemyPlayers {
                champion_name: enemy_champion_name,
                items: enemy_items,
                level: enemy_level,
                // #![todo]
                infer_stats: _infer_enemy_stats,
                // #![todo]
                stats: _enemy_stats,
            } = player;
            let enemy_champion_id = INTERNAL_NAMES.get(&enemy_champion_name).ok_or(
                CalculationError::ChampionCacheNotFound(format!(
                    "[enemy_players.into_par_iter()]: {}",
                    enemy_champion_name
                )),
            )?;
            let enemy_cache = INTERNAL_CHAMPIONS.get(enemy_champion_id).ok_or(
                CalculationError::ChampionCacheNotFound(format!(
                    "[enemy_players.into_par_iter()]: {}",
                    enemy_champion_id
                )),
            )?;
            let enemy_base_stats = get_base_stats(enemy_cache, enemy_level);
            let full_stats = get_full_stats(
                (
                    enemy_champion_id,
                    enemy_level,
                    enemy_dragon_multipliers.earth,
                ),
                (enemy_base_stats, &enemy_items),
                (
                    current_player_stats.armor_penetration_percent,
                    current_player_stats.armor_penetration_flat,
                ),
                (
                    current_player_stats.magic_penetration_percent,
                    current_player_stats.magic_penetration_flat,
                ),
            );

            let damage_multipliers = DamageMultipliers {
                self_mod: full_stats.2.self_mod,
                enemy_mod: full_stats.2.enemy_mod,
                damage_mod: (full_stats.2.armor_mod, full_stats.2.magic_mod),
            };

            let eval_ctx = get_eval_ctx(&current_player_state, &full_stats);

            let abilities_damage =
                get_damages(&abilities_iter_expr, &damage_multipliers, &eval_ctx);
            let items_damage = get_damages(&items_iter_expr, &damage_multipliers, &eval_ctx);
            let runes_damage = get_damages(&runes_iter_expr, &damage_multipliers, &eval_ctx);

            let mut compared_items_damage =
                FxHashMap::with_capacity_and_hasher(simulated_stats.len(), Default::default());

            for (siml_item_id, siml_stats) in simulated_stats.iter() {
                let siml_full_stats = get_full_stats(
                    (
                        enemy_champion_id,
                        enemy_level,
                        enemy_dragon_multipliers.earth,
                    ),
                    (enemy_base_stats, &enemy_items),
                    (
                        siml_stats.armor_penetration_percent,
                        siml_stats.armor_penetration_flat,
                    ),
                    (
                        siml_stats.magic_penetration_percent,
                        siml_stats.magic_penetration_flat,
                    ),
                );

                let siml_damage_multipliers = DamageMultipliers {
                    self_mod: siml_full_stats.2.self_mod,
                    enemy_mod: siml_full_stats.2.enemy_mod,
                    damage_mod: (siml_full_stats.2.armor_mod, siml_full_stats.2.magic_mod),
                };

                let siml_bonus_stats = get_bonus_stats(
                    BasicStats {
                        armor: siml_stats.armor,
                        health: siml_stats.max_health,
                        attack_damage: siml_stats.attack_damage,
                        magic_resist: siml_stats.magic_resist,
                        mana: siml_stats.max_mana,
                    },
                    current_player_basic_stats,
                );

                let siml_current_player_state = (
                    siml_stats,
                    siml_bonus_stats,
                    current_player_state.2,
                    current_player_state.3,
                );

                let siml_eval_ctx = get_eval_ctx(&siml_current_player_state, &siml_full_stats);

                let siml_abilities_damage = get_damages(
                    &abilities_iter_expr,
                    &siml_damage_multipliers,
                    &siml_eval_ctx,
                );
                let siml_items_damage =
                    get_damages(&items_iter_expr, &siml_damage_multipliers, &siml_eval_ctx);
                let siml_runes_damage =
                    get_damages(&runes_iter_expr, &siml_damage_multipliers, &siml_eval_ctx);

                compared_items_damage.insert(
                    *siml_item_id,
                    SimulatedDamages {
                        abilities: siml_abilities_damage,
                        items: siml_items_damage,
                        runes: siml_runes_damage,
                    },
                );
            }

            Ok(OutputEnemy {
                champion_id: enemy_champion_id,
                champion_name: enemy_champion_name,
                damages: Damages {
                    abilities: abilities_damage,
                    items: items_damage,
                    runes: runes_damage,
                    compared_items: compared_items_damage,
                },
                level: player.level,
                base_stats: enemy_base_stats,
                current_stats: full_stats.0,
                bonus_stats: full_stats.1,
                real_armor: full_stats.2.real_armor,
                real_magic_resist: full_stats.2.real_magic,
            })
        })
        .collect::<Result<Vec<OutputEnemy>, CalculationError>>()?;

    Ok(OutputGame {
        current_player: OutputCurrentPlayer {
            damaging_abilities: current_player_cache
                .abilities
                .into_iter()
                .map(|(key, val)| (*key, val.name))
                .chain(std::iter::once(("A", "Basic Attack")))
                .chain(std::iter::once(("C", "Critical Strike")))
                .collect(),
            damaging_items: current_player_items
                .into_iter()
                .filter_map(|item_id| {
                    let item = INTERNAL_ITEMS.get(item_id)?;
                    Some((*item_id, item.name))
                })
                .collect(),
            damaging_runes: current_player_runes
                .into_iter()
                .filter_map(|rune_id| {
                    let rune = INTERNAL_RUNES.get(rune_id)?;
                    Some((*rune_id, rune.name))
                })
                .collect(),
            level: current_player_level,
            champion_id: current_player_champion_id,
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: current_player_stats,
        },
        enemies,
        compared_items,
        recommended_items: current_player_recommended_items,
    })
}
