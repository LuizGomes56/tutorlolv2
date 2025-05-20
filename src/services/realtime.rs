use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::model::{
    application::GlobalCache, champions::Champion, items::Item, realtime::*, riot::*, runes::Rune,
};

use super::eval::eval_math_expr;

pub enum CalculateError {
    ActivePlayerNotFound,
    ChampionNameNotFound(String),
    ChampionCacheNotFound(String),
    MetaItemsNotFound(String),
}

pub fn calculate<'a>(
    cache: &'a Arc<GlobalCache>,
    game: &'a RiotRealtime,
    _item: String,
) -> Result<Realtime<'a>, CalculateError> {
    let game_time = game.game_data.game_time;
    let map_number = game.game_data.map_number;

    let active_player = &game.active_player;
    let all_players = &game.all_players;

    let active_player_level = active_player.level;

    let active_player_expanded = all_players
        .iter()
        .find(|player| player.riot_id == active_player.riot_id)
        .ok_or(CalculateError::ActivePlayerNotFound)?;

    let current_champion_id = cache
        .champion_names
        .get(&active_player_expanded.champion_name)
        .ok_or_else(|| {
            CalculateError::ChampionNameNotFound(active_player_expanded.champion_name.clone())
        })?;

    let current_player_cache = cache
        .champions
        .get(current_champion_id)
        .ok_or_else(|| CalculateError::ChampionCacheNotFound(current_champion_id.clone()))?;

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

    let current_player_recommended_items = cache
        .meta_items
        .get(current_champion_id)
        .ok_or_else(|| CalculateError::MetaItemsNotFound(current_champion_id.clone()))?;

    let recommended_items_vec = match current_player_position.as_str() {
        "TOP" => &current_player_recommended_items.top,
        "JUNGLE" => &current_player_recommended_items.jungle,
        "MIDDLE" => &current_player_recommended_items.mid,
        "BOTTOM" => &current_player_recommended_items.adc,
        "SUPPORT" => &current_player_recommended_items.support,
        _ => &Vec::new(),
    };

    let owned_items: HashSet<usize> = active_player_expanded
        .items
        .iter()
        .map(|item| item.item_id)
        .collect();

    let recommended_items = recommended_items_vec
        .iter()
        .filter(|item_id| !owned_items.contains(item_id))
        .copied()
        .collect();

    let mut damaging_abilities: HashMap<String, String> = current_player_cache
        .abilities
        .iter()
        .filter_map(|(key, damage)| {
            (!key.contains("MONSTER") && !key.contains("MINION"))
                .then(|| (key.clone(), damage.name.clone()))
        })
        .collect();

    damaging_abilities.extend([
        ("A".to_string(), "Basic Attack".to_string()),
        ("C".to_string(), "Critical Strike".to_string()),
    ]);

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

    let damaging_items: HashMap<usize, String> = active_player_expanded
        .items
        .iter()
        .filter_map(|riot_item| {
            let item = cache.items.get(&riot_item.item_id)?;
            let ok = match attack_type {
                AttackType::Melee => item.melee.is_some(),
                AttackType::Ranged => item.ranged.is_some(),
                AttackType::Other => false,
            };
            ok.then(|| (riot_item.item_id, item.name.clone()))
        })
        .collect();

    let current_player = CurrentPlayer {
        champion_id: current_champion_id.clone(),
        team: &active_player_expanded.team,
        position: &active_player_expanded.position,
        champion_name: &active_player_expanded.champion_name,
        current_stats: get_current_stats(&active_player),
        level: active_player_level,
        riot_id: &active_player.riot_id,
        damaging_abilities,
        damaging_items,
        damaging_runes,
        bonus_stats: get_bonus_stats(&active_player.champion_stats, &current_player_base_stats),
        base_stats: current_player_base_stats,
    };

    let enemy_players: Vec<&RiotAllPlayers> = all_players
        .iter()
        .filter(|player| player.team != active_player_expanded.team)
        .collect();

    let mut enemies = Vec::with_capacity(enemy_players.len());

    for enemy in enemy_players.into_iter() {
        let enemy_champion_id = cache
            .champion_names
            .get(&enemy.champion_name)
            .ok_or_else(|| CalculateError::ChampionNameNotFound(enemy.champion_name.clone()))?;
        let current_enemy_cache = &cache
            .champions
            .get(enemy_champion_id)
            .ok_or_else(|| CalculateError::ChampionCacheNotFound(enemy_champion_id.clone()))?;
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
            champion_id: enemy_champion_id.clone(),
            champion_name: &enemy.champion_name,
            riot_id: &enemy.riot_id,
            team: &enemy.team,
            level: enemy_level,
            position: &enemy.position,
            damages: Damages {
                compared_items: HashMap::new(),
                abilities: get_abilities_damage(
                    current_player_cache,
                    &full_stats,
                    &active_player.abilities,
                ),
                items: get_items_damage(
                    &cache.items,
                    &full_stats,
                    &current_player
                        .damaging_items
                        .iter()
                        .map(|(key, _)| *key)
                        .collect(),
                ),
                runes: get_runes_damage(
                    &cache.runes,
                    &full_stats,
                    &current_player
                        .damaging_runes
                        .iter()
                        .map(|(key, _)| *key)
                        .collect(),
                ),
            },
            bonus_stats: enemy_bonus_stats,
            base_stats: enemy_base_stats,
            current_stats: enemy_current_stats,
        });
    }
    Ok(Realtime {
        current_player,
        enemies,
        recommended_items,
        game_information: GameInformation {
            game_time,
            map_number,
        },
        compared_items: HashMap::new(),
    })
}

fn get_items_damage(
    cache: &HashMap<usize, Item>,
    stats: &FullStats,
    current_player_items_vec: &Vec<usize>,
) -> HashMap<usize, InstanceDamage> {
    let mut item_damages = HashMap::<usize, InstanceDamage>::new();

    let is_ranged = stats.current_player.is_ranged;
    for current_player_item in current_player_items_vec.into_iter() {
        if let Some(item) = cache.get(current_player_item) {
            let item_damage = match is_ranged {
                true => &item.ranged,
                false => &item.melee,
            };
            if let Some(damage) = item_damage {
                let minimum_damage_string = replace_damage_keywords(
                    stats,
                    &damage.minimum_damage.clone().unwrap_or(String::from("0.0")),
                );
                let maximum_damage_string = replace_damage_keywords(
                    stats,
                    &damage.maximum_damage.clone().unwrap_or(String::from("0.0")),
                );
                let minimum_damage = eval_math_expr(&minimum_damage_string).unwrap_or(0.0);
                let maximum_damage = eval_math_expr(&maximum_damage_string).unwrap_or(0.0);
                item_damages.insert(
                    current_player_item.clone(),
                    InstanceDamage {
                        minimum_damage,
                        maximum_damage,
                        damage_type: item.damage_type.clone().unwrap_or(String::from("UNKNOWN")),
                        damages_in_area: false,
                        damages_onhit: false,
                    },
                );
            }
        }
    }
    item_damages
}

fn get_runes_damage(
    cache: &HashMap<usize, Rune>,
    stats: &FullStats,
    current_player_runes_vec: &Vec<usize>,
) -> HashMap<usize, InstanceDamage> {
    let mut rune_damages = HashMap::<usize, InstanceDamage>::new();

    let is_ranged = stats.current_player.is_ranged;
    for current_player_rune in current_player_runes_vec.into_iter() {
        if let Some(rune) = cache.get(current_player_rune) {
            let rune_damage = match is_ranged {
                true => &rune.ranged,
                false => &rune.melee,
            };
            let damage_string = replace_damage_keywords(stats, rune_damage);
            let minimum_damage = eval_math_expr(&damage_string).unwrap_or(0.0);
            rune_damages.insert(
                current_player_rune.clone(),
                InstanceDamage {
                    minimum_damage,
                    maximum_damage: 0.0,
                    damage_type: rune.damage_type.clone(),
                    damages_in_area: false,
                    damages_onhit: false,
                },
            );
        }
    }
    rune_damages
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
            let eval_damage_str = |from_vec: &Vec<String>| {
                let default_value = String::from("0.0");
                let format_str = from_vec.get(indexation - 1).unwrap_or(&default_value);
                let damage_str = replace_damage_keywords(full_stats, format_str);
                let formatted_str = format!("({}) * {}", damage_str, damage_multiplier);
                eval_math_expr(&formatted_str).unwrap_or(0.0)
            };

            let minimum_damage = eval_damage_str(&ability.minimum_damage);
            let maximum_damage = eval_damage_str(&ability.maximum_damage);
            ability_damages.insert(
                keyname.to_string(),
                InstanceDamage {
                    minimum_damage,
                    maximum_damage,
                    damage_type: String::from(damage_type),
                    damages_in_area,
                    damages_onhit: false,
                },
            );
        } else {
            ability_damages.insert(
                keyname.to_string(),
                InstanceDamage {
                    minimum_damage: 0.0,
                    maximum_damage: 0.0,
                    damage_type,
                    damages_in_area,
                    damages_onhit: false,
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
            minimum_damage: basic_attack_damage,
            maximum_damage: 0.0,
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
            damages_onhit: false,
        },
    );
    ability_damages.insert(
        String::from("C"),
        InstanceDamage {
            minimum_damage: basic_attack_damage
                * (full_stats.current_player.current_stats.crit_damage / 100.0),
            maximum_damage: 0.0,
            damage_type: String::from("PHYSICAL_DAMAGE"),
            damages_in_area: false,
            damages_onhit: false,
        },
    );

    ability_damages
}

fn replace_damage_keywords(stats: &FullStats, target_str: &str) -> String {
    let replacements = [
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
        (
            "STEELCAPS_EFFECT",
            if stats.enemy_has_steelcaps { 0.88 } else { 1.0 },
        ),
        (
            "RANDUIN_EFFECT",
            if stats.enemy_has_randuin { 0.7 } else { 1.0 },
        ),
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
        .fold(target_str.to_string(), |acc, (old, new)| {
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
    cache: &HashMap<usize, Item>,
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
        if let Some(item) = cache.get(&enemy_item.item_id) {
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
