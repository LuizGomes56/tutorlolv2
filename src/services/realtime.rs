use std::{collections::HashMap, sync::Arc};

use crate::model::{application::GlobalCache, champions::Champion, realtime::*, riot::*};

// Return a massive struct object that will be sent to browser. It will have every
// necessary field to make up tables and display the game data.
pub async fn calculate(cache: &Arc<GlobalCache>, game: &RiotRealtime, _item: String) -> Realtime {
    let active_player = &game.active_player;
    let all_players = &game.all_players;

    let active_player_level = active_player.level;

    let active_player_expanded = all_players
        .iter()
        .find(|player| player.riot_id == active_player.riot_id)
        .expect("Couldn't find active player in all players list");

    let current_player_cache = cache
        .champions
        .get(
            cache
                .champion_names
                .get(&active_player_expanded.champion_name)
                .unwrap(),
        )
        .unwrap();

    let current_player_base_stats = get_base_stats(current_player_cache, active_player_level);

    let current_player = CurrentPlayer {
        team: active_player_expanded.team.to_string(),
        position: active_player_expanded.position.to_string(),
        champion_name: active_player_expanded.champion_name.to_string(),
        current_stats: get_current_stats(&active_player),
        level: active_player_level,
        riot_id: active_player.riot_id.to_string(),
        damaging_abilities: current_player_cache
            .abilities
            .iter()
            .map(|(ability_key, _)| ability_key.to_string())
            .collect(),
        damaging_items: vec![],
        damaging_runes: vec![],
        bonus_stats: get_bonus_stats(&active_player.champion_stats, &current_player_base_stats),
        base_stats: current_player_base_stats,
    };

    let enemy_players: Vec<&RiotAllPlayers> = all_players
        .iter()
        .filter(|player| player.team != active_player_expanded.team)
        .collect();

    let mut enemies = Vec::with_capacity(enemy_players.len());

    for enemy in enemy_players.into_iter() {
        let current_enemy_cache = &cache
            .champions
            .get(cache.champion_names.get(&enemy.champion_name).unwrap())
            .unwrap();
        let enemy_level = enemy.level;
        let enemy_base_stats = get_base_stats(current_enemy_cache, enemy_level);
        let enemy_current_stats = BasicStats {
            armor: 0.0,
            health: 0.0,
            attack_damage: 0.0,
            magic_resist: 0.0,
            mana: 0.0,
        };
        enemies.push(Enemy {
            champion_name: enemy.champion_name.to_string(),
            riot_id: enemy.riot_id.to_string(),
            team: enemy.team.to_string(),
            level: enemy_level,
            position: enemy.position.to_string(),
            damages: Damages {
                abilities: HashMap::new(),
                items: HashMap::new(),
                runes: HashMap::new(),
            },
            bonus_stats: get_bonus_stats(&enemy_current_stats.to_riot_format(), &enemy_base_stats),
            base_stats: enemy_base_stats,
            current_stats: enemy_current_stats,
        });
    }

    Realtime {
        current_player,
        enemies,
        game_information: GameInformation {
            game_time: game.game_data.game_time,
            map_number: game.game_data.map_number,
        },
    }
}

fn get_current_stats(active_player: &RiotActivePlayer) -> Stats {
    Stats {
        ability_power: active_player.champion_stats.ability_power,
        armor: active_player.champion_stats.armor,
        armor_penetration_flat: active_player.champion_stats.physical_lethality,
        armor_penetration_percent: active_player.champion_stats.armor_penetration_percent,
        attack_damage: active_player.champion_stats.attack_damage,
        attack_range: active_player.champion_stats.attack_range,
        attack_speed: active_player.champion_stats.attack_speed,
        crit_chance: active_player.champion_stats.crit_chance,
        crit_damage: active_player.champion_stats.crit_damage,
        current_health: active_player.champion_stats.current_health,
        magic_penetration_flat: active_player.champion_stats.magic_penetration_flat,
        magic_penetration_percent: active_player.champion_stats.magic_penetration_percent,
        magic_resist: active_player.champion_stats.magic_resist,
        max_health: active_player.champion_stats.max_health,
        max_mana: active_player.champion_stats.resource_max,
        current_mana: active_player.champion_stats.resource_value,
    }
}

fn get_bonus_stats(current_stats: &RiotChampionStats, base_stats: &BasicStats) -> BasicStats {
    BasicStats {
        armor: current_stats.armor - base_stats.armor,
        health: current_stats.max_health - base_stats.health,
        attack_damage: current_stats.attack_damage - base_stats.attack_damage,
        magic_resist: current_stats.magic_resist - base_stats.magic_resist,
        mana: current_stats.resource_max - base_stats.mana,
    }
}

fn get_base_stats(champion_cache: &Champion, level: usize) -> BasicStats {
    let stat_formula = |base: f64, growth: f64, level: usize| {
        base + growth * (level as f64 - 1f64) * (0.7025 + 0.0175 * (level as f64 - 1f64))
    };
    BasicStats {
        armor: stat_formula(
            champion_cache.stats.armor.flat,
            champion_cache.stats.armor.per_level,
            level,
        ),
        health: stat_formula(
            champion_cache.stats.health.flat,
            champion_cache.stats.health.per_level,
            level,
        ),
        attack_damage: stat_formula(
            champion_cache.stats.attack_damage.flat,
            champion_cache.stats.attack_damage.per_level,
            level,
        ),
        magic_resist: stat_formula(
            champion_cache.stats.magic_resistance.flat,
            champion_cache.stats.magic_resistance.per_level,
            level,
        ),
        mana: stat_formula(
            champion_cache.stats.mana.flat,
            champion_cache.stats.mana.per_level,
            level,
        ),
    }
}
