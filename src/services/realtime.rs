use std::{collections::HashMap, sync::Arc};

use crate::model::{
    application::GlobalCache, champions::Champion, items::Item, realtime::*, riot::*,
};

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
        let enemy_current_stats =
            get_enemy_current_stats(&cache.items, &enemy_base_stats, &enemy.items);
        let enemy_bonus_stats =
            get_bonus_stats(&enemy_current_stats.to_riot_format(), &enemy_base_stats);
        let full_stats = get_full_stats(
            &current_player,
            (&enemy_base_stats, &enemy_bonus_stats, &enemy_current_stats),
            &enemy.items.iter().map(|x| x.item_id).collect(),
        );
        enemies.push(Enemy {
            champion_name: enemy.champion_name.to_string(),
            riot_id: enemy.riot_id.to_string(),
            team: enemy.team.to_string(),
            level: enemy_level,
            position: enemy.position.to_string(),
            damages: Damages {
                abilities: get_abilities_damage(
                    current_player_cache,
                    &full_stats,
                    &active_player.abilities,
                ),
                items: HashMap::new(),
                runes: HashMap::new(),
            },
            bonus_stats: enemy_bonus_stats,
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

fn get_full_stats<'a>(
    current_player: &'a CurrentPlayer,
    enemy_state: (&'a BasicStats, &'a BasicStats, &'a BasicStats),
    enemy_items: &Vec<usize>,
) -> FullStats<'a> {
    let (enemy_base_stats, enemy_bonus_stats, enemy_current_stats) = enemy_state;
    let mut real_armor = enemy_current_stats.armor
        * current_player.current_stats.armor_penetration_percent
        - current_player.current_stats.armor_penetration_flat;
    let mut real_magic_resist = enemy_current_stats.magic_resist
        * current_player.current_stats.magic_penetration_percent
        - current_player.current_stats.magic_penetration_flat;
    if real_armor < 0.0 {
        real_armor = 0.0;
    }
    if real_magic_resist < 0.0 {
        real_magic_resist = 0.0;
    }

    let physical_damage_multiplier = 100.0 / (100.0 + real_armor);
    let magic_damage_multiplier = 100.0 / (100.0 + real_magic_resist);

    let has_item = |item: &[usize], check_for: &[usize]| -> bool {
        check_for.iter().any(|id| item.contains(id))
    };

    FullStats {
        physical_damage_multiplier,
        magic_damage_multiplier,
        missing_health: 1.0
            - (current_player.current_stats.current_health
                / current_player.current_stats.max_health),
        enemy_has_steelcaps: has_item(enemy_items, &[3047]),
        enemy_has_rocksolid: has_item(enemy_items, &[3082, 3110, 3143]),
        enemy_has_randuin: has_item(enemy_items, &[3143]),
        current_player: SelfFullStats {
            current_stats: &current_player.current_stats,
            base_stats: &current_player.base_stats,
            bonus_stats: &current_player.bonus_stats,
            is_ranged: current_player.current_stats.attack_range > 350.0,
            level: current_player.level,
            deals_extra_damage_from: DamageMultipliers {
                magic_damage: 1.0,
                physical_damage: 1.0,
                true_damage: 1.0,
                all_sources: 1.0,
            },
            is_physical_adaptative_type: 0.35 * current_player.bonus_stats.attack_damage
                >= 0.2 * current_player.current_stats.ability_power,
        },
        enemy_player: EnemyFullStats {
            current_stats: &enemy_current_stats,
            base_stats: &enemy_base_stats,
            bonus_stats: &enemy_bonus_stats,
            takes_extra_damage_from: DamageMultipliers {
                magic_damage: 1.0,
                physical_damage: 1.0,
                true_damage: 1.0,
                all_sources: 1.0,
            },
            real_resistances: RealResistances {
                armor: real_armor,
                magic_resistance: real_magic_resist,
            },
        },
    }
}

fn get_abilities_damage(
    current_player_cache: &Champion,
    full_stats: &FullStats,
    abilities: &RiotAbilities,
) -> HashMap<String, InstanceDamage> {
    let mut ability_damages = HashMap::<String, InstanceDamage>::new();
    let default_value = String::from("0.0");
    for (keyname, ability) in &current_player_cache.abilities {
        let first_char = keyname.chars().next().unwrap_or_default();
        let indexation = match first_char {
            'P' => full_stats.current_player.level,
            _ => match first_char {
                'Q' => abilities.q.ability_level,
                'W' => abilities.w.ability_level,
                'E' => abilities.e.ability_level,
                'R' => abilities.r.ability_level,
                _ => return ability_damages,
            },
        };
        let damages_in_area = ability.damages_in_area;
        let damage_type = ability.damage_type.clone();
        if indexation > 0 {
            let damage_multiplier = match ability.damage_type.as_str() {
                "MAGIC_DAMAGE" => full_stats.magic_damage_multiplier,
                "PHYSICAL_DAMAGE" => full_stats.physical_damage_multiplier,
                _ => 1.0,
            };
            let minimum_damage = format!(
                "({}) * {}",
                replace_damage_keywords(
                    full_stats,
                    ability
                        .minimum_damage
                        .get(indexation - 1)
                        .unwrap_or(&default_value),
                ),
                damage_multiplier
            );
            let maximum_damage = format!(
                "({}) * {}",
                replace_damage_keywords(
                    full_stats,
                    &ability
                        .maximum_damage
                        .get(indexation - 1)
                        .unwrap_or(&default_value),
                ),
                damage_multiplier
            );
            ability_damages.insert(
                keyname.clone(),
                InstanceDamage {
                    minimum_damage: minimum_damage.clone(),
                    maximum_damage: maximum_damage.clone(),
                    damage_type,
                    damages_in_area,
                },
            );
        } else {
            ability_damages.insert(
                keyname.clone(),
                InstanceDamage {
                    minimum_damage: default_value.clone(),
                    maximum_damage: default_value.clone(),
                    damage_type,
                    damages_in_area,
                },
            );
        }
    }
    let basic_attack_damage = full_stats.current_player.current_stats.attack_damage
        * full_stats.physical_damage_multiplier
        * full_stats
            .current_player
            .deals_extra_damage_from
            .physical_damage;
    ability_damages.insert(
        String::from("A"),
        InstanceDamage {
            minimum_damage: basic_attack_damage.to_string(),
            maximum_damage: String::from(default_value.clone()),
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
        },
    );
    ability_damages.insert(
        String::from("C"),
        InstanceDamage {
            minimum_damage: (basic_attack_damage
                * (full_stats.current_player.current_stats.crit_damage / 100.0))
                .to_string(),
            maximum_damage: String::from(default_value),
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
        },
    );

    ability_damages
}

fn replace_damage_keywords(stats: &FullStats, target_string: &str) -> String {
    let replacements = [
        ("ENEMY_MAX_HEALTH", stats.enemy_player.current_stats.health),
        (
            "ENEMY_CURRENT_HEALTH",
            stats.enemy_player.current_stats.health,
        ),
        (
            "ENEMY_MISSING_HEALTH",
            stats.enemy_player.current_stats.health,
        ),
        ("CHOGATH_STACKS", 1.0),
        ("BONUS_AD", stats.current_player.bonus_stats.attack_damage),
        ("BONUS_HEALTH", stats.current_player.bonus_stats.health),
        ("AP", stats.current_player.current_stats.ability_power),
        ("AD", stats.current_player.current_stats.attack_damage),
    ];

    replacements
        .iter()
        .fold(target_string.to_string(), |acc, (old, new)| {
            acc.replace(old, &new.to_string())
        })
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

fn get_enemy_current_stats(
    cache: &HashMap<String, Item>,
    base_stats: &BasicStats,
    current_items: &Vec<RiotItems>,
) -> BasicStats {
    let mut base = BasicStats {
        armor: base_stats.armor,
        health: base_stats.health,
        attack_damage: base_stats.attack_damage,
        magic_resist: base_stats.magic_resist,
        mana: base_stats.mana,
    };
    let add_if_some = |target: &mut f64, value: Option<f64>| {
        if let Some(v) = value {
            *target += v;
        }
    };
    for enemy_item in current_items {
        if let Some(item) = cache.get(&enemy_item.item_id.to_string()) {
            let stats = &item.stats;
            add_if_some(&mut base.armor, stats.armor);
            add_if_some(&mut base.health, stats.health);
            add_if_some(&mut base.attack_damage, stats.attack_damage);
            add_if_some(&mut base.magic_resist, stats.magic_resistance);
            add_if_some(&mut base.mana, stats.mana);
        }
    }
    base
}
