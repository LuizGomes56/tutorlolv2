use std::hash::Hash;

use super::*;
use crate::{
    GLOBAL_CACHE,
    model::{
        base::{
            BasicStats, DamageExpression, DamageLike, DamageMultipliers, Damages, InstanceDamage,
            SimulatedDamages,
        },
        cache::EvalContext,
        realtime::*,
        riot::*,
    },
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;

pub enum RealtimeError {
    CurrentPlayerNotFound,
    ChampionNameNotFound,
    ChampionCacheNotFound,
}

/// Takes a type constructed from port 2999 and returns a new type "Realtime"
#[writer_macros::trace_time]
pub fn realtime<'a>(game: &'a RiotRealtime) -> Result<Realtime<'a>, RealtimeError> {
    // let __start_time = std::time::Instant::now();
    // let __print_time = |msg: &str| println!("{}: {:#?}", msg, __start_time.elapsed().as_micros());

    let current_player_level = game.active_player.level;
    let game_time = game.game_data.game_time;
    let map_number = game.game_data.map_number;
    let RiotRealtime {
        active_player:
            RiotActivePlayer {
                abilities: current_player_abilities,
                champion_stats: current_player_riot_stats,
                full_runes: current_player_riot_runes,
                riot_id: current_player_riot_id,
                ..
            },
        all_players,
        events: RiotRealtimeEvents {
            events: game_events,
        },
        ..
    } = game;

    let RiotAllPlayers {
        champion_name: current_player_champion_name,
        items: current_player_riot_items,
        position: current_player_position,
        team: current_player_team,
        ..
    } = all_players
        .iter()
        .find(|player| &player.riot_id == current_player_riot_id)
        .ok_or(RealtimeError::CurrentPlayerNotFound)?;

    let players_map = all_players
        .iter()
        .map(|player| {
            (
                player.riot_id.split('#').next().unwrap_or_default(),
                &player.team,
            )
        })
        .collect::<FxHashMap<&str, &String>>();

    let (enemy_dragon_multipliers, ally_dragon_multipliers) =
        get_dragon_multipliers(game_events, players_map, current_player_team);

    // __print_time("Step 1");

    let current_player_stats = current_player_riot_stats.to_stats();

    // __print_time("Step 2");

    let current_player_champion_id = GLOBAL_CACHE
        .champion_names
        .get(current_player_champion_name)
        .ok_or(RealtimeError::ChampionNameNotFound)?;

    // __print_time("Step 3");

    let current_player_cache = GLOBAL_CACHE
        .champions
        .get(current_player_champion_id)
        .ok_or(RealtimeError::ChampionCacheNotFound)?;

    // __print_time("Step 4");

    let current_player_base_stats = get_base_stats(current_player_cache, current_player_level);

    // __print_time("Step 5");

    let current_player_basic_stats = BasicStats {
        armor: current_player_stats.armor,
        health: current_player_stats.max_health,
        attack_damage: current_player_stats.attack_damage,
        magic_resist: current_player_stats.magic_resist,
        mana: current_player_stats.max_mana,
    };

    // __print_time("Step 6");

    let current_player_bonus_stats =
        get_bonus_stats(current_player_basic_stats, current_player_base_stats);

    // __print_time("Step 7");

    let current_player_recommended_items = {
        if let Some(meta_items) = GLOBAL_CACHE.meta_items.get(current_player_champion_id) {
            match current_player_position.as_str() {
                "TOP" => meta_items.top,
                "JUNGLE" => meta_items.jungle,
                "MIDDLE" => meta_items.mid,
                "BOTTOM" => meta_items.adc,
                "SUPPORT" => meta_items.support,
                _ => &[],
            }
        } else {
            &[]
        }
    };

    // __print_time("Step 8");

    let current_player_runes = current_player_riot_runes
        .general_runes
        .iter()
        .map(|riot_rune| riot_rune.id)
        .collect::<Vec<usize>>();

    // __print_time("Step 9");

    let current_player_items = current_player_riot_items
        .iter()
        .map(|value| value.item_id)
        .collect::<Vec<usize>>();

    // __print_time("Step 10");

    let (simulated_stats, compared_items) = get_simulated_champion_stats(
        &current_player_stats,
        &current_player_items,
        ally_dragon_multipliers,
    );

    // __print_time("Step 11");

    let current_player_is_ranged = current_player_cache.attack_type == "RANGED";
    let current_player_levelings = current_player_abilities.get_levelings();
    let current_player_state = (
        &current_player_stats,
        current_player_bonus_stats,
        current_player_base_stats,
        current_player_level,
    );

    // __print_time("Step 12");

    let abilities_iter_expr = get_abilities_damage(
        current_player_cache,
        current_player_level,
        current_player_levelings,
    );

    // __print_time("Step 13");

    let items_iter_expr = get_items_damage(&current_player_items, current_player_is_ranged);

    // __print_time("Step 14");

    let runes_iter_expr = get_runes_damage(&current_player_runes, current_player_is_ranged);

    // __print_time("Step 15");

    let (scoreboard, player_values) = all_players
        .into_par_iter()
        .filter_map(|player| {
            let player_champion_id = GLOBAL_CACHE.champion_names.get(&player.champion_name)?;
            let scoreboard = Scoreboard {
                assists: player.scores.assists,
                creep_score: player.scores.creep_score,
                deaths: player.scores.deaths,
                kills: player.scores.kills,
                riot_id: &player.riot_id,
                champion_id: player_champion_id,
                champion_name: &player.champion_name,
                team: &player.team,
                position: &player.position,
            };

            // __print_time("[ITERATOR] Step 16");

            let enemy_data = if &player.team != current_player_team {
                let RiotAllPlayers {
                    champion_name: enemy_champion_name,
                    items: enemy_riot_items,
                    ..
                } = player;
                let enemy_items = enemy_riot_items
                    .iter()
                    .map(|value| value.item_id)
                    .collect::<Vec<usize>>();
                let enemy_level = player.level;
                let enemy_cache = GLOBAL_CACHE.champions.get(player_champion_id)?;
                let enemy_base_stats = get_base_stats(enemy_cache, enemy_level);

                // __print_time("[ITERATOR] Step 17");

                let (enemy_current_stats, enemy_bonus_stats, generic_stats) = get_full_stats(
                    (
                        player_champion_id,
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
                    self_mod: generic_stats.self_mod,
                    enemy_mod: generic_stats.enemy_mod,
                    damage_mod: (generic_stats.armor_mod, generic_stats.magic_mod),
                };

                let eval_ctx = get_eval_ctx(
                    &current_player_state,
                    (enemy_current_stats, enemy_bonus_stats, generic_stats),
                );

                // __print_time("[ITERATOR] Step 18");

                let abilities_damage =
                    get_damages(&abilities_iter_expr, &damage_multipliers, &eval_ctx);

                // __print_time("[ITERATOR] Step 19");

                let items_damage = get_damages(&items_iter_expr, &damage_multipliers, &eval_ctx);

                // __print_time("[ITERATOR] Step 20");

                let runes_damage = get_damages(&runes_iter_expr, &damage_multipliers, &eval_ctx);

                // __print_time("[ITERATOR] Step 21");

                let compared_items_damage = simulated_stats
                    .iter()
                    .map(|(siml_item_id, siml_stats)| {
                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 22", siml_item_id));

                        let siml_full_stats = get_full_stats(
                            (
                                player_champion_id,
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

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 23", siml_item_id));

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

                        let siml_eval_ctx =
                            get_eval_ctx(&siml_current_player_state, siml_full_stats);

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 24", siml_item_id));

                        let siml_abilities_damage = get_damages(
                            &abilities_iter_expr,
                            &siml_damage_multipliers,
                            &siml_eval_ctx,
                        );

                        // __print_time("[ITERATOR] Step 19");

                        let siml_items_damage =
                            get_damages(&items_iter_expr, &siml_damage_multipliers, &siml_eval_ctx);

                        // __print_time("[ITERATOR] Step 20");

                        let siml_runes_damage =
                            get_damages(&runes_iter_expr, &siml_damage_multipliers, &siml_eval_ctx);

                        (
                            *siml_item_id,
                            SimulatedDamages {
                                abilities: siml_abilities_damage,
                                items: siml_items_damage,
                                runes: siml_runes_damage,
                            },
                        )
                    })
                    .collect::<FxHashMap<usize, SimulatedDamages>>();

                // __print_time("[ITERATOR] Step 27");

                Some(Enemy {
                    champion_id: player_champion_id,
                    champion_name: &enemy_champion_name,
                    riot_id: &player.riot_id,
                    team: &player.team,
                    position: &player.position,
                    damages: Damages {
                        abilities: abilities_damage,
                        items: items_damage,
                        runes: runes_damage,
                        compared_items: compared_items_damage,
                    },
                    level: player.level,
                    base_stats: enemy_base_stats,
                    current_stats: enemy_current_stats,
                    bonus_stats: enemy_bonus_stats,
                    real_armor: generic_stats.real_armor,
                    real_magic_resist: generic_stats.real_magic,
                })
            } else {
                None
            };
            Some((scoreboard, enemy_data))
        })
        .collect::<(Vec<Scoreboard>, Vec<Option<Enemy>>)>();

    // __print_time("[FINAL] Step 28");

    Ok(Realtime {
        current_player: CurrentPlayer {
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
                    let item = GLOBAL_CACHE.items.get(&item_id)?;
                    Some((item_id, item.name))
                })
                .collect(),
            damaging_runes: current_player_runes
                .into_iter()
                .filter_map(|rune_id| {
                    let rune = GLOBAL_CACHE.runes.get(&rune_id)?;
                    Some((rune_id, rune.name))
                })
                .collect(),
            riot_id: &current_player_riot_id,
            level: current_player_level,
            team: &current_player_team,
            position: &current_player_position,
            champion_name: &current_player_champion_name,
            champion_id: &current_player_champion_id,
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: current_player_stats,
        },
        enemies: player_values
            .into_iter()
            .filter_map(|enemy| enemy)
            .collect::<Vec<Enemy>>(),
        game_information: GameInformation {
            game_time,
            map_number,
        },
        recommended_items: current_player_recommended_items,
        compared_items,
        scoreboard,
        enemy_dragon_multipliers,
        ally_dragon_multipliers,
    })
}

fn get_dragon_multipliers<'a>(
    event_list: &[RealtimeEvent],
    players: FxHashMap<&str, &String>,
    current_player_team: &str,
) -> (DragonMultipliers, DragonMultipliers) {
    let mut ally_effect = DragonMultipliers::new();
    let mut enemy_effect = DragonMultipliers::new();

    for event in event_list {
        let (Some(killer), Some(dragon_type)) = (&event.killer_name, &event.dragon_type) else {
            continue;
        };
        if let Some(&team) = players.get(killer.as_str()) {
            let target = if team == current_player_team {
                &mut ally_effect
            } else {
                &mut enemy_effect
            };
            match dragon_type.as_str() {
                "Earth" => target.earth += EARTH_DRAGON_MULTIPLIER,
                "Fire" => target.fire += FIRE_DRAGON_MULTIPLIER,
                "Chemtech" => target.chemtech += CHEMTECH_DRAGON_MULTIPLIER,
                _ => {}
            }
        }
    }
    (ally_effect, enemy_effect)
}

fn transform_expr<T: Copy + 'static>(
    tuple: (T, DamageExpression),
    damage_mlt: &DamageMultipliers,
    eval_ctx: &EvalContext,
) -> (T, InstanceDamage) {
    let damage_mod = get_damage_multipliers(damage_mlt, tuple.1.damage_type);
    (
        tuple.0,
        InstanceDamage {
            damage_type: tuple.1.damage_type,
            minimum_damage: damage_mod * (tuple.1.minimum_damage)(tuple.1.level, eval_ctx),
            maximum_damage: damage_mod * (tuple.1.maximum_damage)(tuple.1.level, eval_ctx),
        },
    )
}

fn get_damages<T: Copy + Eq + Hash + 'static>(
    tuples: &[(T, DamageExpression)],
    damage_multipliers: &DamageMultipliers,
    eval_ctx: &EvalContext,
) -> DamageLike<T> {
    tuples
        .iter()
        .copied()
        .map(|tuple| transform_expr(tuple, damage_multipliers, eval_ctx))
        .collect()
}
