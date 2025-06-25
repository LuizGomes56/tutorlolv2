use super::*;
use crate::{
    GLOBAL_CACHE,
    model::{
        base::{BasicStats, DamageLike, Damages, InstanceDamage, SimulatedDamages},
        realtime::*,
        riot::*,
    },
};
use rustc_hash::FxHashMap;

/// Takes a type constructed from port 2999 and returns a new type "Realtime"
#[writer_macros::trace_time]
pub fn realtime<'a>(game: &'a RiotRealtime) -> Result<Realtime<'a>, String> {
    let __start_time = std::time::Instant::now();
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
        // events: RiotRealtimeEvents {
        //     events: game_events,
        // },
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
        .ok_or(format!(
            "Current player not found in allPlayers. No matches for {}",
            current_player_riot_id
        ))?;

    // __print_time("Step 1");

    let enemy_dragon_multipliers = DragonMultipliers {
        earth: 1.0,
        fire: 1.0,
        chemtech: 1.0,
    };

    let ally_dragon_multipliers = DragonMultipliers {
        earth: 1.0,
        fire: 1.0,
        chemtech: 1.0,
    };

    let current_player_stats = current_player_riot_stats.to_stats();

    // __print_time("Step 2");

    let current_player_champion_id = GLOBAL_CACHE
        .champion_names
        .get(current_player_champion_name)
        .ok_or_else(|| {
            format!(
                "Champion name {} does not have a matching ID in champion_names cache",
                current_player_champion_name
            )
        })?;

    // __print_time("Step 3");

    let current_player_cache = GLOBAL_CACHE
        .champions
        .get(current_player_champion_id)
        .ok_or_else(|| format!("Champion {} not found on cache", current_player_champion_id))?;

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
        current_player_stats,
        &current_player_items,
        ally_dragon_multipliers,
    );

    // __print_time("Step 11");

    let current_player_is_ranged = current_player_cache.attack_type == "RANGED";
    let current_player_levelings = current_player_abilities.get_levelings();

    // __print_time("Step 12");

    let abilities_iter_expr = get_abilities_damage(
        current_player_cache,
        current_player_level,
        current_player_levelings,
    );

    // __print_time("Step 13");

    let current_player_items_expr =
        get_items_damage(&current_player_items, current_player_is_ranged);

    // __print_time("Step 14");

    let current_player_runes_expr =
        get_runes_damage(&current_player_runes, current_player_is_ranged);

    // __print_time("Step 15");

    let (scoreboard, player_values) = all_players
        .into_iter()
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
                    (player_champion_id, enemy_level, 1.0),
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

                let eval_ctx = get_eval_ctx(
                    (
                        current_player_stats,
                        current_player_bonus_stats,
                        current_player_base_stats,
                        current_player_level,
                    ),
                    (enemy_current_stats, enemy_bonus_stats, generic_stats),
                );

                // __print_time("[ITERATOR] Step 18");

                macro_rules! eval_expr {
                    ($lvl:expr, $cmp:expr) => {
                        $cmp($lvl, &eval_ctx)
                    };
                    ($lvl:expr, $cmp:expr, $siml:expr, $genr:expr) => {
                        $cmp(
                            $lvl,
                            &get_eval_ctx(
                                (
                                    $siml.0,
                                    $siml.1,
                                    current_player_base_stats,
                                    current_player_level,
                                ),
                                $genr,
                            ),
                        )
                    };
                }
                macro_rules! transform_expr {
                    (($key:expr, $val:expr)) => {{
                        let damage_mod = get_damage_multipliers(
                            generic_stats.self_mod,
                            generic_stats.enemy_mod,
                            (generic_stats.armor_mod, generic_stats.magic_mod),
                            $val.damage_type,
                        );
                        (
                            $key,
                            InstanceDamage {
                                damage_type: $val.damage_type,
                                minimum_damage: damage_mod
                                    * eval_expr!($val.level, $val.minimum_damage),
                                maximum_damage: damage_mod
                                    * eval_expr!($val.level, $val.maximum_damage),
                            },
                        )
                    }};
                    (($key:expr, $val:expr), $siml:expr, $genr:expr) => {{
                        let damage_mod = get_damage_multipliers(
                            $genr.2.self_mod,
                            $genr.2.enemy_mod,
                            ($genr.2.armor_mod, $genr.2.magic_mod),
                            $val.damage_type,
                        );
                        (
                            $key,
                            InstanceDamage {
                                damage_type: $val.damage_type,
                                minimum_damage: damage_mod
                                    * eval_expr!($val.level, $val.minimum_damage, $siml, $genr),
                                maximum_damage: damage_mod
                                    * eval_expr!($val.level, $val.maximum_damage, $siml, $genr),
                            },
                        )
                    }};
                }

                let abilities_damage = abilities_iter_expr
                    .iter()
                    .copied()
                    .map(|(ability_key, ability_expr)| transform_expr!((ability_key, ability_expr)))
                    .collect::<DamageLike<&'static str>>();

                // __print_time("[ITERATOR] Step 19");

                let items_damage = current_player_items_expr
                    .iter()
                    .copied()
                    .map(|(item_id, item_expr)| transform_expr!((item_id, item_expr)))
                    .collect::<DamageLike<usize>>();

                // __print_time("[ITERATOR] Step 20");

                let runes_damage = current_player_runes_expr
                    .iter()
                    .copied()
                    .map(|(rune_id, rune_expr)| transform_expr!((rune_id, rune_expr)))
                    .collect::<DamageLike<usize>>();

                // __print_time("[ITERATOR] Step 21");

                let compared_items_damage = simulated_stats
                    .iter()
                    .map(|(sim_item_id, sim_stats)| {
                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 22", sim_item_id));

                        let sim_gen_stats = get_full_stats(
                            (player_champion_id, enemy_level, 1.0),
                            (enemy_base_stats, &enemy_items),
                            (
                                sim_stats.armor_penetration_percent,
                                sim_stats.armor_penetration_flat,
                            ),
                            (
                                sim_stats.magic_penetration_percent,
                                sim_stats.magic_penetration_flat,
                            ),
                        );

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 23", sim_item_id));

                        let sim_bonus_stats = get_bonus_stats(
                            BasicStats {
                                armor: sim_stats.armor,
                                health: sim_stats.max_health,
                                attack_damage: sim_stats.attack_damage,
                                magic_resist: sim_stats.magic_resist,
                                mana: sim_stats.max_mana,
                            },
                            current_player_basic_stats,
                        );

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 24", sim_item_id));

                        let sim_abilities_damage = abilities_iter_expr
                            .iter()
                            .copied()
                            .map(|(ability_key, ability_expr)| {
                                transform_expr!(
                                    (ability_key, ability_expr),
                                    (*sim_stats, sim_bonus_stats),
                                    sim_gen_stats
                                )
                            })
                            .collect::<DamageLike<&'static str>>();

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 25", sim_item_id));

                        let sim_items_damage = current_player_items_expr
                            .iter()
                            .copied()
                            .map(|(item_id, item_expr)| {
                                transform_expr!(
                                    (item_id, item_expr),
                                    (*sim_stats, sim_bonus_stats),
                                    sim_gen_stats
                                )
                            })
                            .collect::<DamageLike<usize>>();

                        // __print_time(&format!("[CMP] [ITERATOR] [{}] Step 26", sim_item_id));

                        let sim_runes_damage = current_player_runes_expr
                            .iter()
                            .copied()
                            .map(|(rune_id, rune_expr)| {
                                transform_expr!(
                                    (rune_id, rune_expr),
                                    (*sim_stats, sim_bonus_stats),
                                    sim_gen_stats
                                )
                            })
                            .collect::<DamageLike<usize>>();
                        (
                            *sim_item_id,
                            SimulatedDamages {
                                abilities: sim_abilities_damage,
                                items: sim_items_damage,
                                runes: sim_runes_damage,
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
                    if let Some(item) = GLOBAL_CACHE.items.get(&item_id) {
                        Some((item_id, item.name))
                    } else {
                        None
                    }
                })
                .collect(),
            damaging_runes: current_player_runes
                .into_iter()
                .filter_map(|rune_id| {
                    if let Some(rune) = GLOBAL_CACHE.runes.get(&rune_id) {
                        Some((rune_id, rune.name))
                    } else {
                        None
                    }
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

fn get_dragon_multipliers(
    event_list: &[RealtimeEvent],
    scoreboard: &[Scoreboard],
    current_player_team: &str,
) -> (DragonMultipliers, DragonMultipliers) {
    let mut ally_team = DragonMultipliers::new();
    let mut enemy_team = DragonMultipliers::new();

    for event in event_list {
        let (Some(killer), Some(dragon_type)) = (&event.killer_name, &event.dragon_type) else {
            continue;
        };
        if let Some(player) = scoreboard
            .iter()
            .find(|p| &p.riot_id.split('#').next().unwrap_or_default() == killer)
        {
            let target = if player.team == current_player_team {
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
