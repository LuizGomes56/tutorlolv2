use std::{collections::HashMap, hash::Hash, sync::Arc};

use crate::model::{
    application::GlobalCache,
    base::{AttackType, ComparedDamage, ComparedItem, Damages, SimulatedDamages, Stats},
    calculator::{AbilitiesX, Calculator, CurrentPlayerX, EnemyX, GameX},
    champions::Champion,
    items::Item,
    realtime::*,
    riot::*,
    runes::Rune,
};

use super::*;

pub fn calculator<'a>(
    cache: &'a Arc<GlobalCache>,
    game: &'a GameX,
    simulated_items: &'a Vec<usize>,
) -> Result<Calculator, String> {
    let active_player = &game.active_player;
    let enemy_players = &game.enemy_players;

    let active_player_level = active_player.level;

    // let (ally_dragon_multipliers, enemy_dragon_multipliers) = get_dragon_multipliers(
    //     &game.events.events,
    //     &scoreboard,
    //     &active_player_expanded.team,
    // );

    let current_champion_id = &active_player.champion_id;

    let current_player_cache = cache.champions.get(current_champion_id).ok_or_else(|| {
        format!(
            "Champion {} not found on cache",
            current_champion_id.clone()
        )
    })?;

    let current_player_base_stats = get_base_stats(current_player_cache, active_player_level);

    let current_player_position = {
        let default_position = String::from("MIDDLE");
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
                current_champion_id.clone()
            )
        })?;

    let ri_vec = Vec::new();
    let recommended_items_vec =
        get_recommended_items(&current_player_position, &current_player_recommended_items)
            .unwrap_or(&ri_vec);

    let owned_items = &active_player.items;

    let recommended_items = recommended_items_vec
        .iter()
        .filter(|item_id| !owned_items.contains(item_id))
        .copied()
        .collect();

    let damaging_abilities = get_damaging_abilities(current_player_cache);

    let damaging_runes: HashMap<usize, String> = active_player
        .runes
        .iter()
        .filter_map(|riot_rune| {
            cache
                .runes
                .get(&riot_rune)
                .map(|cached| (*riot_rune, cached.name.clone()))
        })
        .collect();

    let attack_type = AttackType::from(current_player_cache.attack_type.as_str());

    let damaging_items = get_damaging_items(&cache.items, attack_type, &owned_items);

    let current_player = CurrentPlayerX {
        current_stats: get_current_stats(&active_player.champion_stats),
        level: active_player_level,
        damaging_abilities,
        damaging_items,
        damaging_runes,
        bonus_stats: get_bonus_stats(&active_player.champion_stats, &current_player_base_stats),
        base_stats: current_player_base_stats,
    };

    let mut compared_items_info = HashMap::<usize, ComparedItem>::new();

    let simulated_champion_stats = get_simulated_champion_stats(
        &simulated_items,
        &owned_items,
        &current_player.current_stats,
        &cache.items,
        &DragonMultipliers {
            earth: 1.0,
            fire: 1.0,
            chemtech: 1.0,
        },
        &mut compared_items_info,
    )?;

    let mut enemies = Vec::with_capacity(1 << 3);
    let mut best_item = (0usize, 0f64);

    for player in enemy_players.into_iter() {
        let player_champion_id = &player.champion_id;
        let current_enemy_cache = &cache.champions.get(player_champion_id).ok_or_else(|| {
            format!(
                "ChampionID {} not found in champions cache",
                player_champion_id.clone()
            )
        })?;
        let enemy_level = player.level;
        let enemy_base_stats = get_base_stats(current_enemy_cache, enemy_level);
        let enemy_items = &player.items;
        let enemy_current_stats = player.stats.clone().unwrap_or(get_enemy_current_stats(
            &cache.items,
            &enemy_base_stats,
            &enemy_items,
            1.0,
        ));
        let (damages, real_resists, bonus_stats) = calculate_enemy_state(
            &cache.items,
            &cache.runes,
            &current_player_cache,
            &current_player,
            &get_damaging_vec(&current_player.damaging_items),
            &get_damaging_vec(&current_player.damaging_runes),
            &enemy_base_stats,
            &enemy_current_stats,
            &enemy_items,
            &active_player.abilities,
            &mut best_item,
            &simulated_champion_stats,
        );
        enemies.push(EnemyX {
            champion_id: player_champion_id.clone(),
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
