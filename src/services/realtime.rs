use std::{collections::HashMap, hash::Hash, sync::Arc};

use crate::model::{
    application::GlobalCache,
    base::{AttackType, ComparedDamage, ComparedItem, Damages, SimulatedDamages, Stats},
    champions::Champion,
    items::Item,
    realtime::*,
    riot::*,
    runes::Rune,
};

use super::*;

pub fn realtime<'a>(
    cache: &'a Arc<GlobalCache>,
    game: &'a RiotRealtime,
    simulated_items: &'a Vec<usize>,
) -> Result<Realtime<'a>, String> {
    let game_time = game.game_data.game_time;
    let map_number = game.game_data.map_number;

    let active_player = &game.active_player;
    let all_players = &game.all_players;

    let mut scoreboard = all_players
        .iter()
        .map(|player| Scoreboard {
            riot_id: player.riot_id.clone(),
            team: player.team.clone(),
            kills: player.scores.kills,
            deaths: player.scores.deaths,
            assists: player.scores.assists,
            creep_score: player.scores.creep_score,
            champion_id: None,
            position: player.position.clone(),
            champion_name: player.champion_name.clone(),
        })
        .collect::<Vec<Scoreboard>>();

    let active_player_level = active_player.level;

    let active_player_expanded = all_players
        .iter()
        .find(|player| player.riot_id == active_player.riot_id)
        .ok_or("Current player not found in allPlayers".to_string())?;

    let (ally_dragon_multipliers, enemy_dragon_multipliers) = get_dragon_multipliers(
        &game.events.events,
        &scoreboard,
        &active_player_expanded.team,
    );

    let current_champion_id = cache
        .champion_names
        .get(&active_player_expanded.champion_name)
        .ok_or_else(|| {
            format!(
                "Champion name {} does not have a matching ID in champion_names cache",
                active_player_expanded.champion_name.clone()
            )
        })?;

    let current_player_cache = cache.champions.get(current_champion_id).ok_or_else(|| {
        format!(
            "Champion {} not found on cache",
            current_champion_id.clone()
        )
    })?;

    let current_player_base_stats = get_base_stats(current_player_cache, active_player_level);

    let current_player_position = if !active_player_expanded.position.is_empty() {
        active_player_expanded.position.clone()
    } else {
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

    let owned_items: Vec<usize> = active_player_expanded
        .items
        .iter()
        .map(|item| item.item_id)
        .collect();

    let recommended_items = recommended_items_vec
        .iter()
        .filter(|item_id| !owned_items.contains(item_id))
        .copied()
        .collect();

    let damaging_abilities = get_damaging_abilities(current_player_cache);

    let damaging_runes: HashMap<usize, String> = active_player
        .full_runes
        .general_runes
        .iter()
        .filter_map(|riot_rune| {
            cache
                .runes
                .get(&riot_rune.id)
                .map(|cached| (riot_rune.id, cached.name.clone()))
        })
        .collect();

    let attack_type = AttackType::from(current_player_cache.attack_type.as_str());

    let damaging_items = get_damaging_items(&cache.items, attack_type, &owned_items);

    let current_player = CurrentPlayer {
        champion_id: current_champion_id.clone(),
        team: &active_player_expanded.team,
        position: &active_player_expanded.position,
        champion_name: &active_player_expanded.champion_name,
        current_stats: get_current_stats(&active_player.champion_stats),
        level: active_player_level,
        riot_id: &active_player.riot_id,
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
        &ally_dragon_multipliers,
        &mut compared_items_info,
    )?;

    let mut enemies = Vec::with_capacity(1 << 3);
    let mut best_item = (0usize, 0f64);

    for player in all_players.into_iter() {
        let player_champion_id =
            cache
                .champion_names
                .get(&player.champion_name)
                .ok_or_else(|| {
                    format!(
                        "Champion {} does not have a matching ID in champion_names cache",
                        player.champion_name.clone()
                    )
                })?;
        scoreboard
            .iter_mut()
            .find(|this_player| this_player.riot_id == player.riot_id)
            .unwrap()
            .champion_id = Some(player_champion_id.clone());
        if player.team != active_player_expanded.team {
            let current_enemy_cache =
                &cache.champions.get(player_champion_id).ok_or_else(|| {
                    format!(
                        "ChampionID {} not found in champions cache",
                        player_champion_id.clone()
                    )
                })?;
            let enemy_level = player.level;
            let enemy_base_stats = get_base_stats(current_enemy_cache, enemy_level);
            let enemy_items = &player
                .items
                .iter()
                .map(|x| x.item_id)
                .collect::<Vec<usize>>();
            let enemy_current_stats = get_enemy_current_stats(
                &cache.items,
                &enemy_base_stats,
                enemy_items,
                enemy_dragon_multipliers.earth,
            );
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
                &active_player.abilities.get_levelings(),
                &mut best_item,
                &simulated_champion_stats,
            );
            enemies.push(Enemy {
                champion_id: player_champion_id.clone(),
                champion_name: &player.champion_name,
                riot_id: &player.riot_id,
                team: &player.team,
                level: enemy_level,
                position: &player.position,
                damages,
                real_resists,
                bonus_stats,
                base_stats: enemy_base_stats,
                current_stats: enemy_current_stats,
            });
        }
    }

    Ok(Realtime {
        current_player,
        enemies,
        recommended_items,
        game_information: GameInformation {
            game_time,
            map_number,
        },
        compared_items: compared_items_info,
        best_item: best_item.0,
        scoreboard,
        enemy_dragon_multipliers,
        ally_dragon_multipliers,
    })
}
