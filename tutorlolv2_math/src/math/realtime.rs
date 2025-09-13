use super::*;
use crate::model::{
    SIZE_ITEMS_EXPECTED,
    base::{BasicStats, DamageMultipliers, DamageValue, DragonMultipliers},
    realtime::*,
    riot::*,
};
use smallvec::SmallVec;
use tinyset::SetU32;
use tutorlolv2_generated::{
    CHAMPION_NAME_TO_ID, DAMAGING_ITEMS, DAMAGING_RUNES, INTERNAL_CHAMPIONS, ItemId, Position,
    RuneId,
};

/// Takes a type constructed from port 2999 and returns a new type "Realtime"
// #[tutorlolv2_macros::trace_time]
pub fn realtime<'a>(game: &'a RiotRealtime) -> Result<Realtime<'a>, CalculationError> {
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
        position: current_player_raw_position,
        team: current_player_raw_team,
        riot_id: current_player_riot_id,
        scores: current_player_scores,
        ..
    } = all_players
        .iter()
        .find(|player| &player.riot_id == current_player_riot_id)
        .ok_or(CalculationError::CurrentPlayerNotFound)?;

    let current_player_team = Team::from_raw(current_player_raw_team);
    let current_player_position = Position::from_raw(current_player_raw_position);

    let players_map = all_players
        .iter()
        .map(|player| {
            (
                player.riot_id.split('#').next().unwrap_or_default(),
                player.team,
            )
        })
        .collect::<SmallVec<[(_, _); 10]>>();

    let (enemy_dragon_multipliers, ally_dragon_multipliers) =
        get_dragon_multipliers(game_events, players_map, current_player_raw_team);

    let current_player_stats = current_player_riot_stats.to_stats();
    let current_player_champion_id = CHAMPION_NAME_TO_ID
        .get(current_player_champion_name)
        .ok_or(CalculationError::ChampionNameNotFound)?;

    let current_player_cache = INTERNAL_CHAMPIONS
        .get(*current_player_champion_id as usize)
        .ok_or(CalculationError::ChampionCacheNotFound)?;

    let current_player_base_stats = get_base_stats(current_player_cache, current_player_level);
    let current_player_basic_stats = BasicStats {
        armor: current_player_stats.armor,
        health: current_player_stats.max_health,
        attack_damage: current_player_stats.attack_damage,
        magic_resist: current_player_stats.magic_resist,
        mana: current_player_stats.max_mana,
    };

    let current_player_bonus_stats =
        get_bonus_stats(current_player_basic_stats, current_player_base_stats);

    let current_player_damaging_runes = current_player_riot_runes
        .general_runes
        .as_ref()
        .and_then(|general_runes| {
            Some(
                general_runes
                    .into_iter()
                    .filter_map(|riot_rune| {
                        let rune_id = riot_rune.id;
                        DAMAGING_RUNES
                            .contains(&rune_id)
                            .then_some(RuneId::from_u32(rune_id) as u32)
                    })
                    .collect::<SetU32>(),
            )
        })
        .unwrap_or_default();

    let current_player_items = current_player_riot_items
        .iter()
        .map(|riot_item| riot_item.item_id)
        .collect::<SmallVec<[_; SIZE_ITEMS_EXPECTED]>>();

    let current_player_damaging_items = current_player_items
        .iter()
        .filter_map(|item_id| {
            DAMAGING_ITEMS
                .contains(&item_id)
                .then_some(ItemId::from_u32(*item_id) as u32)
        })
        .collect::<SetU32>();

    let simulated_stats = get_simulated_champion_stats(
        &current_player_stats,
        &current_player_items,
        &ally_dragon_multipliers,
    );

    let current_player_attack_type = current_player_cache.attack_type;
    let current_player_levelings = current_player_abilities.get_levelings();
    let current_player_state = (
        &current_player_stats,
        current_player_bonus_stats,
        current_player_base_stats,
        current_player_level,
    );

    let abilities_iter_expr = get_abilities_damage(
        current_player_cache,
        current_player_level,
        current_player_levelings,
    );
    let items_iter_expr = get_items_damage(
        &current_player_damaging_items,
        current_player_attack_type,
        current_player_level,
    );
    let runes_iter_expr =
        get_runes_damage(&current_player_damaging_runes, current_player_attack_type);

    let enemies = all_players
        .into_iter()
        .filter_map(|player| {
            if &player.team == current_player_raw_team {
                return None;
            }
            let RiotAllPlayers {
                champion_name: enemy_champion_name,
                items: enemy_riot_items,
                riot_id,
                team: enemy_raw_team,
                position: enemy_raw_position,
                ..
            } = player;
            let enemy_team = Team::from_raw(enemy_raw_team);
            let enemy_position = Position::from_raw(enemy_raw_position);
            let enemy_champion_id = CHAMPION_NAME_TO_ID.get(&enemy_champion_name)?;
            let enemy_items = enemy_riot_items
                .iter()
                .map(|value| ItemId::from_u32(value.item_id))
                .collect::<SmallVec<[_; SIZE_ITEMS_EXPECTED]>>();
            let enemy_level = player.level;
            let enemy_cache = INTERNAL_CHAMPIONS.get(*enemy_champion_id as usize)?;
            let enemy_base_stats = get_base_stats(enemy_cache, enemy_level);

            let full_stats = get_full_stats(
                (
                    *enemy_champion_id,
                    enemy_level,
                    enemy_dragon_multipliers.earth,
                    0.0,
                ),
                (None, enemy_base_stats, &enemy_items),
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
                self_mod: full_stats.2.self_mod,
                enemy_mod: full_stats.2.enemy_mod,
                damage_mod: (full_stats.2.armor_mod, full_stats.2.magic_mod),
            };

            let eval_ctx = get_eval_ctx(&current_player_state, &full_stats);
            let mut onhit_effects = DamageValue::<i32>::default();

            let abilities_damage = get_damages(
                &abilities_iter_expr,
                &damage_multipliers,
                &eval_ctx,
                &mut onhit_effects,
            );
            let items_damage = get_damages(
                &items_iter_expr,
                &damage_multipliers,
                &eval_ctx,
                &mut onhit_effects,
            );
            let runes_damage = get_damages(
                &runes_iter_expr,
                &damage_multipliers,
                &eval_ctx,
                &mut onhit_effects,
            );
            let attack_damage = get_attacks(&damage_multipliers, &eval_ctx, onhit_effects);

            let mut compared_items_damage = SmallVec::with_capacity(simulated_stats.len());

            for (siml_item_id, siml_stats) in simulated_stats.iter() {
                let siml_full_stats = get_full_stats(
                    (
                        *enemy_champion_id,
                        enemy_level,
                        enemy_dragon_multipliers.earth,
                        0.0,
                    ),
                    (None, enemy_base_stats, &enemy_items),
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

                let siml_eval_ctx = get_eval_ctx(&siml_current_player_state, &siml_full_stats);
                let mut siml_onhit_effects = DamageValue::<i32>::default();

                let siml_abilities_damage = get_damages(
                    &abilities_iter_expr,
                    &siml_damage_multipliers,
                    &siml_eval_ctx,
                    &mut siml_onhit_effects,
                );
                let siml_items_damage = get_damages(
                    &items_iter_expr,
                    &siml_damage_multipliers,
                    &siml_eval_ctx,
                    &mut siml_onhit_effects,
                );
                let siml_runes_damage = get_damages(
                    &runes_iter_expr,
                    &siml_damage_multipliers,
                    &siml_eval_ctx,
                    &mut siml_onhit_effects,
                );
                let siml_attack_damage =
                    get_attacks(&siml_damage_multipliers, &siml_eval_ctx, siml_onhit_effects);

                compared_items_damage.push((
                    *siml_item_id,
                    SimulatedDamages {
                        attacks: siml_attack_damage,
                        abilities: siml_abilities_damage,
                        items: siml_items_damage,
                        runes: siml_runes_damage,
                    },
                ));
            }

            Some((
                *enemy_champion_id,
                Enemy {
                    riot_id,
                    team: enemy_team,
                    position: enemy_position,
                    damages: Damages {
                        attacks: attack_damage,
                        abilities: abilities_damage,
                        items: items_damage,
                        runes: runes_damage,
                        compared_items: compared_items_damage,
                    },
                    level: player.level,
                    base_stats: enemy_base_stats.cast_i32(),
                    current_stats: full_stats.0.cast_i32(),
                    bonus_stats: full_stats.1.cast_i32(),
                    real_armor: full_stats.2.real_armor as i32,
                    real_magic_resist: full_stats.2.real_magic as i32,
                },
            ))
        })
        .collect::<SmallVec<[(_, Enemy); _]>>();

    Ok(Realtime {
        current_player: CurrentPlayer {
            adaptative_type: RiotFormulas::adaptative_type(
                current_player_bonus_stats.attack_damage,
                current_player_stats.ability_power,
            ),
            damaging_items: current_player_damaging_items.into(),
            damaging_runes: current_player_damaging_runes.into(),
            riot_id: current_player_riot_id,
            level: current_player_level,
            team: current_player_team,
            position: current_player_position,
            champion_id: *current_player_champion_id,
            base_stats: current_player_base_stats.cast_i32(),
            bonus_stats: current_player_bonus_stats.cast_i32(),
            current_stats: current_player_stats.cast_i32(),
        },
        enemies,
        scoreboard: Scoreboard {
            kills: current_player_scores.kills,
            assists: current_player_scores.assists,
            creep_score: current_player_scores.creep_score,
            deaths: current_player_scores.deaths,
            riot_id: current_player_riot_id,
            champion_id: *current_player_champion_id,
            position: current_player_position,
        },
        game_information: GameInformation {
            game_time: game_time as i32,
            map_number,
        },
        enemy_dragon_multipliers,
        ally_dragon_multipliers,
    })
}

fn get_dragon_multipliers<'a>(
    event_list: &[RealtimeEvent],
    players: SmallVec<[(&str, &str); 10]>,
    current_player_team: &str,
) -> (DragonMultipliers, DragonMultipliers) {
    let mut ally_effect = DragonMultipliers::new();
    let mut enemy_effect = DragonMultipliers::new();

    for event in event_list {
        let (Some(killer), Some(dragon_type)) = (&event.killer_name, &event.dragon_type) else {
            continue;
        };
        if let Some((_, team)) = players.iter().find(|(name, _)| name == killer) {
            let target = if *team == current_player_team {
                &mut ally_effect
            } else {
                &mut enemy_effect
            };
            match *dragon_type {
                "Earth" => target.earth += EARTH_DRAGON_MULTIPLIER,
                "Fire" => target.fire += FIRE_DRAGON_MULTIPLIER,
                "Chemtech" => target.chemtech += CHEMTECH_DRAGON_MULTIPLIER,
                _ => {}
            }
        }
    }
    (ally_effect, enemy_effect)
}
