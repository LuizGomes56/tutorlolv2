use std::{collections::HashMap, sync::Arc};

use crate::model::{
    application::GlobalCache,
    base::{AttackType, BasicStats, ComparedItem, Stats},
    calculator::{ActivePlayerX, Calculator, CurrentPlayerX, EnemyX, GameX},
    champions::Champion,
    items::Item,
    realtime::*,
    runes::Rune,
};

use super::*;

fn apply_auto_stats(
    active_player: &ActivePlayerX,
    items_cache: &HashMap<usize, Item>,
) -> Result<Stats, String> {
    let mut stats = Stats::default();
    let stacks = active_player.stacks as f64;

    match active_player.champion_id.as_str() {
        "Veigar" => stats.ability_power += stacks,
        "Chogath" => {
            let scallings = [0.0, 80.0, 120.0, 160.0];
            let mut r_ability_level = active_player.abilities.r;
            if r_ability_level > 3 {
                r_ability_level = 3
            }
            stats.max_health += stacks * scallings[r_ability_level]
        }
        "Sion" => stats.max_health += stacks,
        "Darius" => {
            stats.armor_penetration_percent += 15.0 + 5.0 * active_player.abilities.e as f64
        }
        "Pantheon" => stats.armor_penetration_percent += 10.0 * active_player.abilities.r as f64,
        "Nilah" => stats.armor_penetration_percent += stats.crit_chance / 3.0,
        "Mordekaiser" => {
            stats.magic_penetration_percent += 2.5 + 2.5 * active_player.abilities.e as f64
        }
        _ => {}
    }

    let add_if_some = |target: &mut f64, value: Option<f64>| {
        if let Some(v) = value {
            *target += v;
        }
    };

    for item_id in active_player.items.iter() {
        let cached_item = items_cache
            .get(&item_id)
            .ok_or_else(|| format!("Item {} not found on cache", item_id.to_string()))?;

        let item_stats = &cached_item.stats;

        add_if_some(&mut stats.ability_power, item_stats.ability_power);
        add_if_some(&mut stats.attack_damage, item_stats.attack_damage);
        add_if_some(&mut stats.armor, item_stats.armor);
        add_if_some(&mut stats.magic_resist, item_stats.magic_resistance);
        add_if_some(&mut stats.max_health, item_stats.health);
        add_if_some(&mut stats.crit_chance, item_stats.critical_strike_chance);
        add_if_some(&mut stats.crit_damage, item_stats.critical_strike_damage);
        add_if_some(&mut stats.max_mana, item_stats.mana);
        add_if_some(&mut stats.attack_speed, item_stats.attack_speed);
    }

    if stats.crit_chance > 100.0 {
        stats.crit_chance = 100.0;
    }

    Ok(stats)
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
        earth: EARTH_DRAGON_MULTIPLIER * game.ally_earth_dragons as f64,
        fire: FIRE_DRAGON_MULTIPLIER * game.ally_fire_dragons as f64,
        chemtech: 1.0,
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

    let recommended_items: Vec<usize> = recommended_items_vec
        .iter()
        .filter(|item_id: &&usize| !owned_items.contains(item_id))
        .copied()
        .collect();

    let damaging_abilities: HashMap<String, String> = get_damaging_abilities(current_player_cache);

    let damaging_runes: HashMap<usize, String> = active_player
        .runes
        .iter()
        .filter_map(|riot_rune: &usize| {
            cache
                .runes
                .get(&riot_rune)
                .map(|cached: &Rune| (*riot_rune, cached.name.clone()))
        })
        .collect();

    let attack_type: AttackType = AttackType::from(current_player_cache.attack_type.as_str());

    let damaging_items: HashMap<usize, String> =
        get_damaging_items(&cache.items, attack_type, &owned_items);

    let active_player_champion_stats: Stats = active_player.champion_stats.clone();
    // apply_auto_stats(active_player, &cache.items)?

    let current_player: CurrentPlayerX = CurrentPlayerX {
        bonus_stats: get_bonus_stats(&active_player_champion_stats, &current_player_base_stats),
        current_stats: active_player_champion_stats,
        level: active_player_level,
        damaging_abilities,
        damaging_items,
        damaging_runes,
        base_stats: current_player_base_stats,
        champion_id: active_player.champion_id.clone(),
    };

    let mut compared_items_info: HashMap<usize, ComparedItem> =
        HashMap::<usize, ComparedItem>::new();

    let simulated_champion_stats: HashMap<usize, Stats> = get_simulated_champion_stats(
        &simulated_items,
        &owned_items,
        &current_player.current_stats,
        &cache.items,
        ally_dragon_multipliers,
        &mut compared_items_info,
    )?;

    let mut enemies: Vec<EnemyX> = Vec::with_capacity(1 << 3);
    let mut best_item: (usize, f64) = (0usize, 0f64);

    for player in game.enemy_players.iter() {
        let player_champion_id: String = player.champion_id.clone();
        let current_enemy_cache: &&Champion =
            &cache.champions.get(&player_champion_id).ok_or_else(|| {
                format!(
                    "ChampionID {} not found in champions cache",
                    player_champion_id
                )
            })?;
        let champion_name: String = current_enemy_cache.name.clone();
        let enemy_level: usize = player.level;
        let mut enemy_base_stats: BasicStats = get_base_stats(current_enemy_cache, enemy_level);
        let enemy_items: &Vec<usize> = &player.items;
        let mut enemy_current_stats: BasicStats = player.stats.clone();
        /*
            get_enemy_current_stats(
                &cache.items,
                &enemy_base_stats,
                &enemy_items,
                EARTH_DRAGON_MULTIPLIER * game.enemy_earth_dragons as f64,
            )
        */
        let (damages, real_resists, bonus_stats) = calculate_enemy_state(GameState {
            cache: GameStateCache {
                items: &cache.items,
                runes: &cache.runes,
            },
            current_player: GameStateCurrentPlayer {
                thisv: &current_player,
                cache: &current_player_cache,
                items: &keys_as_vec(&current_player.damaging_items),
                runes: &keys_as_vec(&current_player.damaging_runes),
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
            best_item: &mut best_item,
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
        best_item: best_item.0,
    })
}
