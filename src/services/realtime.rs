use super::*;
use crate::model::{
    application::GlobalCache,
    base::{AttackType, BasicStats, ComparedItem, Stats},
    champions::Champion,
    realtime::*,
    riot::*,
    runes::Rune,
};
use std::{collections::HashMap, sync::Arc};

/// Takes a type constructed from port 2999 and returns a new type "Realtime"
/// `simulated_items` dictates how many clones of player current stats will be created
/// for each clone, abilities, runes and items damages will be recalculated
/// Returns a HashMap with results for each enemy, and the best item in general.
pub fn realtime<'a>(
    cache: &'a Arc<GlobalCache>,
    game: &'a RiotRealtime,
    // Find the best item possible instead of the best one in the array specified
    // _: &'a Vec<usize>,
    simulated_items: &'a Vec<usize>,
) -> Result<Realtime<'a>, String> {
    // #![todo] Filter legendary items that are available to be purchased [game][game_data][map_number]
    // Calculating every item in the game is too expensive. Normally, it takes 200ns to run this function
    // If this option is enabled, the average time of execution rises up to 10ms to 20ms (Similar to NodeJS)
    // JSON size rises from (6 * NUM_ENEMIES) to (572 * NUM_ENEMIES) KB. Too large to be sent efficiently to the client
    // Benchmarks with this option showed nearly 8 times more memory usage, and 1.5 times CPU usage
    // Execution time is still really good, but not worth it to leave this feature on.

    // let simulated_items = &cache
    //     .items
    //     .iter()
    //     .map(|(item_id, _)| item_id.clone())
    //     .collect::<Vec<usize>>();

    let game_time: f64 = game.game_data.game_time;
    let map_number: usize = game.game_data.map_number;

    let active_player: &RiotActivePlayer = &game.active_player;
    let all_players: &Vec<RiotAllPlayers> = &game.all_players;

    let mut scoreboard: Vec<Scoreboard> = all_players
        .iter()
        .map(|player: &RiotAllPlayers| Scoreboard {
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

    let active_player_level: usize = active_player.level;

    let active_player_expanded: &RiotAllPlayers = all_players
        .iter()
        .find(|player: &&RiotAllPlayers| player.riot_id == active_player.riot_id)
        .ok_or("Current player not found in allPlayers".to_string())?;

    let (ally_dragon_multipliers, enemy_dragon_multipliers) = get_dragon_multipliers(
        &game.events.events,
        &scoreboard,
        &active_player_expanded.team,
    );

    let current_champion_id: &String = cache
        .champion_names
        .get(&active_player_expanded.champion_name)
        .ok_or_else(|| {
            format!(
                "Champion name {} does not have a matching ID in champion_names cache",
                &active_player_expanded.champion_name
            )
        })?;

    let current_player_cache: &Champion = cache
        .champions
        .get(current_champion_id)
        .ok_or_else(|| format!("Champion {} not found on cache", &current_champion_id))?;

    let current_player_base_stats: BasicStats =
        get_base_stats(current_player_cache, active_player_level);

    let current_player_position: String = if !active_player_expanded.position.is_empty() {
        active_player_expanded.position.clone()
    } else {
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
                &current_champion_id
            )
        })?;

    let recommended_items_fallback: Vec<usize> = Vec::new();
    let recommended_items_vec: &Vec<usize> =
        get_recommended_items(&current_player_position, &current_player_recommended_items)
            .unwrap_or(&recommended_items_fallback);

    let owned_items: Vec<usize> = active_player_expanded
        .items
        .iter()
        .map(|item: &RiotItems| item.item_id)
        .collect();

    let recommended_items: Vec<usize> = recommended_items_vec
        .iter()
        .filter(|item_id: &&usize| !owned_items.contains(item_id))
        .copied()
        .collect();

    let damaging_abilities: HashMap<String, String> = get_damaging_abilities(current_player_cache);

    let damaging_runes: HashMap<usize, String> = active_player
        .full_runes
        .general_runes
        .iter()
        .filter_map(|riot_rune: &RiotGeneralRunes| {
            cache
                .runes
                .get(&riot_rune.id)
                .map(|cached: &Rune| (riot_rune.id, cached.name.clone()))
        })
        .collect();

    let attack_type: AttackType = AttackType::from(current_player_cache.attack_type.as_str());

    let damaging_items: HashMap<usize, String> =
        get_damaging_items(&cache.items, attack_type, &owned_items);

    let current_player: CurrentPlayer<'_> = CurrentPlayer {
        champion_id: current_champion_id.clone(),
        team: &active_player_expanded.team,
        bonus_stats: get_bonus_stats(&active_player.champion_stats, &current_player_base_stats),
        position: &active_player_expanded.position,
        champion_name: &active_player_expanded.champion_name,
        level: active_player_level,
        riot_id: &active_player.riot_id,
        damaging_abilities,
        damaging_items,
        damaging_runes,
        current_stats: active_player.champion_stats.transform(),
        base_stats: current_player_base_stats,
    };

    let mut compared_items_info: HashMap<usize, ComparedItem> =
        HashMap::<usize, ComparedItem>::new();

    let simulated_champion_stats: HashMap<usize, Stats> = get_simulated_champion_stats(
        &simulated_items,
        &owned_items,
        &current_player.current_stats,
        &cache.items,
        &ally_dragon_multipliers,
        &mut compared_items_info,
    )?;

    let mut enemies: Vec<Enemy<'_>> = Vec::with_capacity(1 << 3);

    for player in all_players.into_iter() {
        let player_champion_id: &String = cache
            .champion_names
            .get(&player.champion_name)
            .ok_or_else(|| {
                format!(
                    "Champion {} does not have a matching ID in champion_names cache",
                    &player.champion_name
                )
            })?;
        scoreboard
            .iter_mut()
            .find(|this_player: &&mut Scoreboard| this_player.riot_id == player.riot_id)
            .unwrap()
            .champion_id = Some(player_champion_id.clone());
        if player.team != active_player_expanded.team {
            let current_enemy_cache: &&Champion =
                &cache.champions.get(player_champion_id).ok_or_else(|| {
                    format!(
                        "ChampionID {} not found in champions cache",
                        &player_champion_id
                    )
                })?;
            let enemy_level: usize = player.level;
            let mut enemy_base_stats: BasicStats = get_base_stats(current_enemy_cache, enemy_level);
            let enemy_items: &Vec<usize> = &player
                .items
                .iter()
                .map(|x: &RiotItems| x.item_id)
                .collect::<Vec<usize>>();
            let mut enemy_current_stats: BasicStats = get_enemy_current_stats(
                &cache.items,
                &enemy_base_stats,
                enemy_items,
                enemy_dragon_multipliers.earth,
            );
            let (damages, real_resists, bonus_stats) = calculate_enemy_state(GameState {
                cache: GameStateCache {
                    items: &cache.items,
                    runes: &cache.runes,
                },
                current_player: GameStateCurrentPlayer {
                    thisv: &current_player,
                    cache: current_player_cache,
                    items: &clone_keys(&current_player.damaging_items),
                    runes: &clone_keys(&current_player.damaging_runes),
                    abilities: &active_player.abilities.get_levelings(),
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
        scoreboard,
        enemy_dragon_multipliers,
        ally_dragon_multipliers,
    })
}

fn get_dragon_multipliers(
    event_list: &[RealtimeEvent],
    scoreboard: &[Scoreboard],
    current_player_team: &str,
) -> (DragonMultipliers, DragonMultipliers) {
    let mut ally_team: DragonMultipliers = DragonMultipliers::new();
    let mut enemy_team: DragonMultipliers = DragonMultipliers::new();

    for event in event_list {
        let (Some(killer), Some(dragon_type)) = (&event.killer_name, &event.dragon_type) else {
            continue;
        };
        if let Some(player) = scoreboard
            .iter()
            .find(|p: &&Scoreboard| &p.riot_id.split('#').next().unwrap_or_default() == killer)
        {
            let target: &mut DragonMultipliers = if player.team == current_player_team {
                &mut ally_team
            } else {
                &mut enemy_team
            };
            match dragon_type.as_str() {
                "Earth" => target.earth += EARTH_DRAGON_MULTIPLIER,
                "Fire" => target.fire += FIRE_DRAGON_MULTIPLIER,
                "Chemtech" => target.chemtech += CHEMTECH_DRAGON_MULTIPLIER,
                _ => {}
            }
        }
    }
    (ally_team, enemy_team)
}
