use super::{formulas::*, helpers::*, model::*, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::{
    CHAMPION_NAME_TO_ID, DAMAGING_RUNES, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, ItemId, Position,
    RuneId, SIMULATED_ITEMS,
};

pub fn realtime<'a>(game: &'a RiotRealtime) -> Option<Realtime<'a>> {
    let mut size_counter = 0;

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

    let current_playe_bonus_stats = {
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
        current_playe_bonus_stats.attack_damage,
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
    let ability_levels = abilities.get_levelings();
    let current_player_position = Position::from_raw(current_player.position);

    let DamageKind {
        metadata: abilities_metadata,
        damages: abilities_closures,
    } = get_abilities_data(current_player_cache.abilities, ability_levels, *level);

    let DamageKind {
        metadata: items_metadata,
        damages: items_closures,
    } = get_items_data(
        &current_player_items,
        current_player_cache.attack_type,
        *level,
    );

    let DamageKind {
        metadata: runes_metadata,
        damages: runes_closures,
    } = get_runes_data(&current_player_runes, *level);

    let current_player_team = Team::from_raw(current_player.team);
    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: champion_stats.armor_penetration_percent,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: champion_stats.magic_penetration_percent,
    };

    let enemies = all_players
        .into_iter()
        .filter_map(|player| {
            let team = Team::from_raw(player.team);
            if team == current_player_team {
                return None;
            }
            let e_champion_id = CHAMPION_NAME_TO_ID.get(player.champion_name)?;
            let e_cache = unsafe { INTERNAL_CHAMPIONS.get_unchecked(*e_champion_id as usize) };
            let RiotAllPlayers {
                items: e_riot_items,
                riot_id,
                position: e_raw_position,
                level: e_level,
                // scores: e_scores,
                ..
            } = player;
            let e_position = Position::from_raw(e_raw_position);
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
                    bonus_stats: current_playe_bonus_stats,
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
            let abilities_damage = eval_damage::<L_ABLT, _>(
                &eval_ctx,
                &mut onhit,
                &abilities_closures,
                &abilities_metadata,
                modifiers,
            );
            let items_damage = eval_damage::<L_ITEM, _>(
                &eval_ctx,
                &mut onhit,
                &items_closures,
                &items_metadata,
                modifiers,
            );
            let runes_damage = eval_damage::<L_RUNE, _>(
                &eval_ctx,
                &mut onhit,
                &runes_closures,
                &runes_metadata,
                modifiers,
            );
            let attacks_damage = eval_attacks(&eval_ctx, onhit, modifiers);
            let mut siml_results = MaybeUninit::<[Damages; 118]>::uninit();
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
                let siml_abilities_damage = eval_damage::<L_ABLT, _>(
                    &siml_eval_ctx,
                    &mut siml_onhit,
                    &abilities_closures,
                    &abilities_metadata,
                    modifiers,
                );
                let siml_items_damage = eval_damage::<L_ITEM, _>(
                    &siml_eval_ctx,
                    &mut siml_onhit,
                    &items_closures,
                    &items_metadata,
                    modifiers,
                );
                let siml_runes_damage = eval_damage::<L_RUNE, _>(
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
            Some(Enemy {
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
            })
        })
        .collect();

    Some(Realtime {
        current_player: CurrentPlayer {
            riot_id,
            base_stats: current_player_base_stats.into(),
            bonus_stats: current_playe_bonus_stats.into(),
            current_stats: champion_stats.into(),
            level: *level,
            team: current_player_team,
            adaptative_type,
            position: current_player_position,
            champion_id: *current_player_champion_id,
        },
        enemies,
        scoreboard: SmallVec::new(),
        abilities: abilities_metadata,
        items: items_metadata,
        runes: runes_metadata,
        // TODO: Make constant or static
        siml_items: {
            let mut siml_items = MaybeUninit::<[TypeMetadata<ItemId>; L_SIML]>::uninit();
            let siml_items_ptr = siml_items.as_mut_ptr();
            for (i, item_offset) in SIMULATED_ITEMS.iter().enumerate() {
                let item_id = unsafe { core::mem::transmute::<u16, ItemId>(*item_offset as u16) };
                let item_cache = unsafe { INTERNAL_ITEMS.get_unchecked(item_id as usize) };
                unsafe {
                    core::ptr::addr_of_mut!((*siml_items_ptr)[i]).write(TypeMetadata {
                        level: *level,
                        kind: item_id,
                        meta: Meta::from_bytes(item_cache.damage_type, item_cache.attributes),
                    })
                }
            }
            unsafe { siml_items.assume_init() }
        },
        game_time: *game_time as u32,
        ability_levels,
    })
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
