use super::{formulas::*, helpers_arena::*, model_arena::*, riot::*};
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::{
    CHAMPION_NAME_TO_ID, DAMAGING_RUNES, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, ItemId, Position,
    RuneId, SIMULATED_ITEMS_ENUM,
};

const SIMULATED_ITEMS_METADATA: [ConstItemMetadata; L_SIML] = {
    let mut siml_items = MaybeUninit::<[ConstItemMetadata; L_SIML]>::uninit();
    let siml_items_ptr = siml_items.as_mut_ptr();
    let mut i = 0;
    while i < L_SIML {
        let item_id = unsafe { core::mem::transmute::<u16, ItemId>(SIMULATED_ITEMS_ENUM[i]) };
        let item_cache = INTERNAL_ITEMS[item_id as usize];
        unsafe {
            core::ptr::addr_of_mut!((*siml_items_ptr)[i]).write(ConstItemMetadata {
                kind: item_id,
                meta: Meta::from_bytes(item_cache.damage_type, item_cache.attributes),
            })
        };
        i += 1;
    }
    unsafe { siml_items.assume_init() }
};

// If I decide to add a size counter
// const SIZE_SIMULATED_ITEMS_METADATA: usize = size_u(L_SIML as u32) + L_SIML << 1;

pub fn realtime_arena<'a>(
    arena: &'a bumpalo::Bump,
    game: &'a RiotRealtime,
) -> Option<*const Realtime<'a>> {
    let RiotRealtime {
        active_player:
            RiotActivePlayer {
                abilities,
                champion_stats,
                full_runes: RiotFullRunes { general_runes },
                level,
                riot_id,
            },
        all_players,
        events: RiotRealtimeEvents { events },
        game_data:
            RiotRealtimeGameData {
                game_time,
                // map_number,
                ..
            },
    } = game;

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == *riot_id)?;

    let current_player_champion_id = CHAMPION_NAME_TO_ID.get(current_player.champion_name)?;
    let current_player_cache =
        unsafe { INTERNAL_CHAMPIONS.get_unchecked(*current_player_champion_id as usize) };

    let current_player_base_stats = {
        macro_rules! assign {
            ($field:ident) => {
                get_base_stats(&current_player_cache.stats.$field, *level)
            };
        }
        BasicStatsF32 {
            armor: assign!(armor),
            health: assign!(health),
            attack_damage: assign!(attack_damage),
            magic_resist: assign!(magic_resistance),
            mana: assign!(mana),
        }
    };

    let current_player_bonus_stats = {
        macro_rules! subtract {
            ($field:ident) => {
                champion_stats.$field - current_player_base_stats.$field
            };
        }
        BasicStatsF32 {
            armor: subtract!(armor),
            health: subtract!(health),
            attack_damage: subtract!(attack_damage),
            magic_resist: subtract!(magic_resist),
            mana: subtract!(mana),
        }
    };

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
    );

    let current_player_runes = general_runes
        .as_ref()
        .and_then(|gr| {
            Some(
                gr.into_iter()
                    .filter_map(|riot_rune| {
                        let rune_id = riot_rune.id;
                        DAMAGING_RUNES
                            .contains(&rune_id)
                            .then_some(RuneId::from_riot_id(rune_id) as u32)
                    })
                    .collect::<SetU32>(),
            )
        })
        .unwrap_or_default();

    let current_player_items = items_to_set_u32(&current_player.items);
    let (ally_dragons, enemy_earth_dragons) = get_dragons(&events, &all_players);
    let simulated_stats = get_simulated_stats(&champion_stats, ally_dragons);
    let ability_levels = unsafe { std::mem::transmute(abilities.get_levelings()) };
    let current_player_position = Position::from_raw(current_player.position);

    let DamageKind {
        metadata: abilities_metadata,
        damages: abilities_closures,
    } = get_abilities_data(
        arena,
        current_player_cache.abilities,
        ability_levels,
        *level,
    );

    let DamageKind {
        metadata: items_metadata,
        damages: items_closures,
    } = get_items_data(
        arena,
        &current_player_items,
        current_player_cache.attack_type,
        *level,
    );

    let DamageKind {
        metadata: runes_metadata,
        damages: runes_closures,
    } = get_runes_data(arena, &current_player_runes, *level);

    let current_player_team = Team::from_raw(current_player.team);
    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: champion_stats.armor_penetration_percent,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: champion_stats.magic_penetration_percent,
    };

    let scoreboard = unsafe { alloc_uninit_slice(arena, all_players.len()) };
    let enemies = unsafe {
        alloc_uninit_slice(
            arena,
            all_players.len(), // .iter()
                               // .filter(|player| Team::from_raw(player.team) != current_player_team)
                               // .count(),
        )
    };

    for (index, player) in all_players.into_iter().enumerate() {
        let RiotAllPlayers {
            items: e_riot_items,
            riot_id,
            position: e_raw_position,
            level: e_level,
            scores: e_scores,
            champion_name: e_champion_name,
            team: e_team,
        } = player;

        let e_champion_id = CHAMPION_NAME_TO_ID.get(e_champion_name)?;
        let e_position = Position::from_raw(e_raw_position);
        let team = Team::from_raw(e_team);
        let creep_score = e_scores.creep_score;

        scoreboard[index] = Scoreboard {
            riot_id,
            assists: e_scores.assists,
            deaths: e_scores.deaths,
            kills: e_scores.kills,
            creep_score: creep_score,
            champion_id: *e_champion_id,
            position: e_position,
            team,
        };

        if team == current_player_team {
            continue;
        }

        let e_cache = unsafe { INTERNAL_CHAMPIONS.get_unchecked(*e_champion_id as usize) };
        let e_items = items_to_set_u32(e_riot_items);
        let e_base_stats = {
            macro_rules! assign {
                ($field:ident) => {
                    get_base_stats(&e_cache.stats.$field, *level)
                };
            }
            SimpleStatsF32 {
                armor: assign!(armor),
                health: assign!(health),
                magic_resist: assign!(magic_resistance),
            }
        };

        let full_state = get_enemy_state(
            EnemyState {
                base_stats: e_base_stats,
                items: e_items,
                stacks: 0,
                champion_id: *e_champion_id,
                level: *e_level,
            },
            shred,
            enemy_earth_dragons,
        );

        let eval_ctx = get_eval_ctx(
            SelfState {
                current_stats: *champion_stats,
                bonus_stats: current_player_bonus_stats,
                base_stats: current_player_base_stats,
                level: *level,
            },
            &full_state,
        );
        let mut onhit = RangeDamageI32::default();
        let modifiers = DamageModifiers {
            physical_mod: 1.0,
            magic_mod: 1.0,
            true_mod: 1.0,
            global_mod: 1.0,
        };

        let abilities_damage = eval_damage(
            arena,
            &eval_ctx,
            &mut onhit,
            &abilities_closures,
            &abilities_metadata,
            modifiers,
        );

        let items_damage = eval_damage(
            arena,
            &eval_ctx,
            &mut onhit,
            &items_closures,
            &items_metadata,
            modifiers,
        );

        let runes_damage = eval_damage(
            arena,
            &eval_ctx,
            &mut onhit,
            &runes_closures,
            &runes_metadata,
            modifiers,
        );

        let attacks_damage = eval_attacks(&eval_ctx, onhit, modifiers);

        let mut siml_results = MaybeUninit::<[Damages; L_SIML]>::uninit();
        let siml_result_ptr = siml_results.as_mut_ptr();

        for (i, siml_stat) in simulated_stats.into_iter().enumerate() {
            let siml_eval_ctx = get_eval_ctx(
                SelfState {
                    current_stats: siml_stat,
                    bonus_stats: {
                        macro_rules! subtract {
                            ($field:ident) => {
                                champion_stats.$field - current_player_base_stats.$field
                            };
                        }
                        BasicStatsF32 {
                            armor: subtract!(armor),
                            health: subtract!(health),
                            attack_damage: subtract!(attack_damage),
                            magic_resist: subtract!(magic_resist),
                            mana: subtract!(mana),
                        }
                    },
                    base_stats: current_player_base_stats,
                    level: *level,
                },
                &full_state,
            );
            let mut siml_onhit = RangeDamageI32::default();
            let siml_abilities_damage = eval_damage(
                arena,
                &siml_eval_ctx,
                &mut siml_onhit,
                &abilities_closures,
                &abilities_metadata,
                modifiers,
            );
            let siml_items_damage = eval_damage(
                arena,
                &siml_eval_ctx,
                &mut siml_onhit,
                &items_closures,
                &items_metadata,
                modifiers,
            );
            let siml_runes_damage = eval_damage(
                arena,
                &siml_eval_ctx,
                &mut siml_onhit,
                &runes_closures,
                &runes_metadata,
                modifiers,
            );
            let siml_attacks_damage = eval_attacks(&siml_eval_ctx, siml_onhit, modifiers);
            unsafe {
                core::ptr::addr_of_mut!((*siml_result_ptr)[i]).write(Damages {
                    attacks: siml_attacks_damage,
                    abilities: siml_abilities_damage,
                    items: siml_items_damage,
                    runes: siml_runes_damage,
                });
            }
        }

        enemies[index] = Enemy {
            champion_id: *e_champion_id,
            position: e_position,
            team,
            riot_id,
            damages: Damages {
                attacks: attacks_damage,
                abilities: abilities_damage,
                items: items_damage,
                runes: runes_damage,
            },
            siml_items: unsafe { siml_results.assume_init() },
            base_stats: e_base_stats.into(),
            bonus_stats: full_state.bonus_stats.into(),
            current_stats: full_state.current_stats.into(),
            real_armor: full_state.armor_values.real as i32,
            real_magic_resist: full_state.magic_values.real as i32,
            level: *e_level,
        };
    }

    let data = arena.alloc(Realtime {
        current_player: CurrentPlayer {
            riot_id,
            base_stats: current_player_base_stats.into(),
            bonus_stats: current_player_bonus_stats.into(),
            current_stats: champion_stats.into(),
            level: *level,
            team: current_player_team,
            adaptative_type,
            position: current_player_position,
            champion_id: *current_player_champion_id,
        },
        enemies,
        scoreboard,
        abilities: abilities_metadata,
        items: items_metadata,
        runes: runes_metadata,
        siml_items: SIMULATED_ITEMS_METADATA,
        game_time: *game_time as u32,
        ability_levels,
    });

    Some(data as *mut _ as *const _)
}

fn get_dragons(events: &[RealtimeEvent], players_map: &[RiotAllPlayers]) -> (Dragons, u8) {
    let mut dragons = Dragons::default();
    let mut enemy_earth_dragons = 0;
    for event in events {
        if let (Some(killer), Some(dragon)) =
            (event.killer_name.as_deref(), event.dragon_type.as_deref())
        {
            match dragon {
                "Earth" => {
                    if players_map.iter().any(|player| player.riot_id == killer) {
                        dragons.earth += 1;
                    } else {
                        enemy_earth_dragons += 1;
                    }
                }
                "Fire" => dragons.fire += 1,
                _ => {}
            }
        }
    }
    (dragons, enemy_earth_dragons)
}
