use super::*;
use crate::{
    model::{
        application::GlobalCache,
        base::{AdaptativeType, AttackType, BasicStats, ComparedItem, Stats},
        calculator::{ActivePlayerX, Calculator, CurrentPlayerX, EnemyX, GameX},
        champions::Champion,
        items::{Item, PartialStats},
        realtime::*,
        runes::Rune,
    },
    services::eval::{ConditionalAddition, RiotFormulas},
};
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// If user opted not to dictate the active player's stats, this function is called
/// it reads all items present in cache and evaluates what the game condition would be
/// if all items were owned in a real game. Stacks and champion exceptions are partially
/// taken into consideration when calculation is made. It is less accurate than Realtime.
fn apply_auto_stats(
    stats: &mut Stats,
    items_cache: &FxHashMap<usize, Item>,
    active_player: &ActivePlayerX,
    base_stats: &BasicStats,
) -> Result<BasicStats, String> {
    let stacks: f64 = active_player.stacks as f64;
    let owned_items: &Vec<usize> = &active_player.items;

    let mut armor_penetration: Vec<f64> = vec![];
    let mut magic_penetration: Vec<f64> = vec![];

    for item_id in owned_items {
        let cached_item = items_cache
            .get(&item_id)
            .ok_or_else(|| format!("Item {} not found on cache", item_id))?;

        let item_stats: &PartialStats = &cached_item.stats;

        macro_rules! add_stat {
            ($field:ident) => {
                stats.$field.add_if_some(item_stats.$field);
            };
            ($field:ident, $field2:ident) => {
                stats.$field.add_if_some(item_stats.$field2);
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

        if let Some(armor_penetration_percent) = item_stats.armor_penetration_percent {
            armor_penetration.push(armor_penetration_percent);
        }
        if let Some(magic_penetration_percent) = item_stats.magic_penetration_percent {
            magic_penetration.push(magic_penetration_percent);
        }
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);

    let bonus_stats: BasicStats = get_bonus_stats(stats, base_stats);

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
    owned_runes: &Vec<usize>,
    level: f64,
    exception_map: &FxHashMap<usize, usize>,
    value_types: (AdaptativeType, AttackType),
) {
    for rune in owned_runes {
        let this_stack: usize = *exception_map.get(&rune).unwrap_or(&0);
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
                _ => {}
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
                let formula: f64 = (this_stack << 2 * (this_stack + 1)) as f64;
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
    owned_items: &Vec<usize>,
    exception_map: &FxHashMap<usize, usize>,
) {
    for item_id in owned_items {
        let this_stack: usize = *exception_map.get(&item_id).unwrap_or(&0);
        match item_id {
            // Dark Seal
            1082 => {
                champion_stats.ability_power += (this_stack.clamp(1, CLAMP_USIZE_MAX) << 2) as f64
            }
            // Mejai's Soulstealer
            3041 => {
                champion_stats.ability_power += (5 * this_stack.clamp(1, CLAMP_USIZE_MAX)) as f64
            }
            // Rabadon's Deathcap
            3089 | 223089 => champion_stats.ability_power *= 1.3,
            // Hubris
            6697 | 7008 => {
                champion_stats.attack_damage +=
                    (15 + this_stack.clamp(1, CLAMP_USIZE_MAX) << 1) as f64
            }
            // Wooglet's Witchcap
            8002 => champion_stats.ability_power *= 1.5,
            _ => {}
        }
    }
}

// #![todo] Comparison tool is not working;
// #![todo] Stats are not being assigned correctly
// #![todo] Untreated item/champion exceptions
pub fn calculator<'a>(
    cache: &'a Arc<GlobalCache>,
    game: &'a GameX,
    simulated_items: &'a Vec<usize>,
) -> Result<Calculator, String> {
    let active_player: &ActivePlayerX = &game.active_player;
    let active_player_level: usize = active_player.level;

    let ally_dragon_multipliers: &DragonMultipliers = &DragonMultipliers {
        earth: 1.0 + EARTH_DRAGON_MULTIPLIER * game.ally_earth_dragons as f64,
        fire: 1.0 + FIRE_DRAGON_MULTIPLIER * game.ally_fire_dragons as f64,
        // #![unsupported]
        chemtech: 1.0 + CHEMTECH_DRAGON_MULTIPLIER * 0.0,
    };

    let current_champion_id: &String = &active_player.champion_id;
    let current_player_cache: &Champion = cache
        .champions
        .get(current_champion_id)
        .ok_or_else(|| format!("Champion {} not found on cache", current_champion_id))?;

    let current_player_base_stats: BasicStats =
        get_base_stats(current_player_cache, active_player_level);
    let current_player_position: String = {
        let default_position: String = String::from("MIDDLE");
        current_player_cache
            .positions
            .get(0)
            .unwrap_or(&default_position)
            .clone()
    };

    let current_player_recommended_items =
        cache.meta_items.get(current_champion_id).ok_or_else(|| {
            format!(
                "Champion {} not found when trying to access meta_items",
                current_champion_id
            )
        })?;

    let recommended_items_fallback: Vec<usize> = Vec::new();
    let recommended_items_vec: &Vec<usize> =
        get_recommended_items(&current_player_position, &current_player_recommended_items)
            .unwrap_or(&recommended_items_fallback);

    let owned_items: &Vec<usize> = &active_player.items;
    let owned_runes: &Vec<usize> = &active_player.runes;

    let recommended_items: Vec<usize> = recommended_items_vec
        .iter()
        .filter(|item_id: &&usize| !owned_items.contains(item_id))
        .copied()
        .collect();

    let damaging_abilities: FxHashMap<String, String> =
        get_damaging_abilities(current_player_cache);
    let damaging_runes: FxHashMap<usize, String> = owned_runes
        .iter()
        .filter_map(|riot_rune: &usize| {
            cache
                .runes
                .get(&riot_rune)
                .map(|cached: &Rune| (*riot_rune, cached.name.clone()))
        })
        .collect();

    let attack_type: AttackType = AttackType::from(current_player_cache.attack_type.as_str());

    let damaging_items: FxHashMap<usize, String> =
        get_damaging_items(&cache.items, attack_type, &owned_items);

    let mut current_stats: Stats =
        RiotFormulas::full_base_stats(&current_player_cache.stats, active_player_level);

    let bonus_stats: BasicStats = if active_player.infer_stats {
        apply_auto_stats(
            &mut current_stats,
            &cache.items,
            active_player,
            &current_player_base_stats,
        )?
    } else {
        get_bonus_stats(&active_player.champion_stats, &current_player_base_stats)
    };

    item_exceptions(&mut current_stats, &owned_items, &game.stack_exceptions);

    let adaptative_type: AdaptativeType =
        RiotFormulas::adaptative_type(bonus_stats.attack_damage, current_stats.ability_power);

    rune_exceptions(
        &mut current_stats,
        &owned_runes,
        active_player_level as f64,
        &game.stack_exceptions,
        (
            adaptative_type,
            AttackType::from(current_player_cache.attack_type.as_str()),
        ),
    );

    let current_player: CurrentPlayerX = CurrentPlayerX {
        bonus_stats,
        current_stats,
        level: active_player_level,
        damaging_abilities,
        damaging_items,
        damaging_runes,
        base_stats: current_player_base_stats,
        champion_id: active_player.champion_id.clone(),
    };

    let mut compared_items_info: FxHashMap<usize, ComparedItem> =
        FxHashMap::<usize, ComparedItem>::default();

    let simulated_champion_stats: FxHashMap<usize, Stats> = get_simulated_champion_stats(
        &simulated_items,
        &owned_items,
        &current_player.current_stats,
        &cache.items,
        ally_dragon_multipliers,
        &mut compared_items_info,
    )?;

    let mut enemies: Vec<EnemyX> = Vec::with_capacity(game.enemy_players.len());
    for player in game.enemy_players.iter() {
        let player_champion_id: String = player.champion_id.clone();
        let current_enemy_cache: &Champion =
            cache.champions.get(&player_champion_id).ok_or_else(|| {
                format!(
                    "ChampionID {} not found in champions cache",
                    player_champion_id
                )
            })?;
        let champion_name: String = current_enemy_cache.name.clone();
        let enemy_level: usize = player.level;
        let mut enemy_base_stats: BasicStats = get_base_stats(current_enemy_cache, enemy_level);
        let enemy_items: &Vec<usize> = &player.items;
        // #![todo] Let user define enemy stats manually instead of predicting it from its items
        let mut enemy_current_stats: BasicStats = get_enemy_current_stats(
            &cache.items,
            &enemy_base_stats,
            &enemy_items,
            EARTH_DRAGON_MULTIPLIER * game.enemy_earth_dragons as f64,
        );
        let (damages, real_resists, bonus_stats) = calculate_enemy_state(GameState {
            cache: GameStateCache {
                items: &cache.items,
                runes: &cache.runes,
            },
            current_player: GameStateCurrentPlayer {
                thisv: &current_player,
                cache: &current_player_cache,
                items: &clone_keys(&current_player.damaging_items),
                runes: &clone_keys(&current_player.damaging_runes),
                abilities: &active_player.abilities,
                simulated_stats: &simulated_champion_stats,
            },
            enemy_player: GameStateEnemyPlayer {
                base_stats: &mut enemy_base_stats,
                current_stats: &mut enemy_current_stats,
                items: &enemy_items,
                champion_id: &player_champion_id,
                level: enemy_level,
            },
        });
        enemies.push(EnemyX {
            champion_name,
            champion_id: player_champion_id,
            level: enemy_level,
            damages,
            real_resists,
            bonus_stats,
            base_stats: enemy_base_stats,
            current_stats: enemy_current_stats,
        });
    }

    Ok(Calculator {
        current_player,
        enemies,
        recommended_items,
        compared_items: compared_items_info,
    })
}
