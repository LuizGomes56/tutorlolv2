use super::{helpers::*, model::*};
use crate::{GameMap, L_SIML, L_TEAM, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::{
    CHAMPION_NAME_TO_ID, ChampionId, DAMAGING_RUNES, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, ItemId,
    Position, RuneId, SIMULATED_ITEMS_ENUM,
};

const LAST_STAND: u32 = RuneId::LastStand.to_riot_id_const();
const COUP_DE_GRACE: u32 = RuneId::CoupdeGrace.to_riot_id_const();
const CUT_DOWN: u32 = RuneId::CutDown.to_riot_id_const();
const AXIOM_ARCANIST: u32 = RuneId::AxiomArcanist.to_riot_id_const();
const SIMULATED_ITEMS_METADATA: [TypeMetadata<ItemId>; L_SIML] = {
    let mut siml_items = MaybeUninit::<[TypeMetadata<ItemId>; L_SIML]>::uninit();
    let siml_items_ptr = siml_items.as_mut_ptr();
    let mut i = 0;
    while i < L_SIML {
        let item_id = unsafe { core::mem::transmute::<u16, ItemId>(SIMULATED_ITEMS_ENUM[i]) };
        let item_cache = INTERNAL_ITEMS[item_id as usize];
        unsafe {
            core::ptr::addr_of_mut!((*siml_items_ptr)[i]).write(TypeMetadata::<ItemId> {
                kind: item_id,
                damage_type: item_cache.damage_type,
                attributes: item_cache.attributes,
            })
        };
        i += 1;
    }
    unsafe { siml_items.assume_init() }
};

#[derive(Default)]
struct RuneExceptions {
    has_last_stand: bool,
    has_coup_de_grace: bool,
    has_cut_down: bool,
    has_axiom_arcanist: bool,
}

pub fn realtime<'a>(game: &'a RiotRealtime) -> Option<Realtime<'a>> {
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
        game_data: RiotRealtimeGameData {
            game_time,
            map_number,
        },
    } = game;

    let game_map = GameMap::from(*map_number);

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == *riot_id)?;

    let current_player_champion_id = *CHAMPION_NAME_TO_ID.get(current_player.champion_name)?;
    let current_player_cache =
        unsafe { INTERNAL_CHAMPIONS.get_unchecked(current_player_champion_id as usize) };

    let is_mega_gnar =
        current_player_champion_id == ChampionId::Gnar && champion_stats.attack_range > 275.0;

    let current_player_base_stats =
        base_stats_bf32(&current_player_cache.stats, *level, is_mega_gnar);

    let current_player_bonus_stats = bonus_stats!(
        BasicStats::<f32>(champion_stats, current_player_base_stats) {
            armor,
            health,
            attack_damage,
            magic_resist,
            mana
        }
    );

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
    );

    let mut rune_exceptions = RuneExceptions::default();

    let current_player_runes = general_runes
        .as_ref()
        .and_then(|gr| {
            Some(
                gr.into_iter()
                    .filter_map(|riot_rune| {
                        let rune_id = riot_rune.id;
                        match rune_id {
                            LAST_STAND => rune_exceptions.has_last_stand = true,
                            COUP_DE_GRACE => rune_exceptions.has_coup_de_grace = true,
                            CUT_DOWN => rune_exceptions.has_cut_down = true,
                            AXIOM_ARCANIST => rune_exceptions.has_axiom_arcanist = true,
                            _ => {}
                        };
                        DAMAGING_RUNES
                            .contains(&rune_id)
                            .then_some(RuneId::from_riot_id(rune_id) as u32)
                    })
                    .collect::<SetU32>(),
            )
        })
        .unwrap_or_default();

    let current_player_items = riot_items_to_set_u32(&current_player.items);
    let (ally_dragons, enemy_earth_dragons) = get_dragons(&events, &all_players);
    let simulated_stats = get_simulated_stats(&champion_stats, ally_dragons);
    let ability_levels = abilities.get_levelings();
    let current_player_position = Position::from_raw(current_player.position)
        .unwrap_or(unsafe { *current_player_cache.positions.get_unchecked(0) });

    let eval_data = DamageEvalData {
        abilities: get_abilities_data(current_player_cache.abilities),
        items: get_items_data(&current_player_items, current_player_cache.attack_type),
        runes: get_runes_data(&current_player_runes),
    };

    let current_player_team = Team::from(current_player.team);
    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: champion_stats.armor_penetration_percent,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: champion_stats.magic_penetration_percent,
    };

    let mut scoreboard = SmallVec::with_capacity(all_players.len());
    let self_state = SelfState {
        current_stats: *champion_stats,
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        ability_levels,
        level: *level,
    };

    let enemies = all_players
        .into_iter()
        .filter_map(|player| {
            let RiotAllPlayers {
                items: e_riot_items,
                riot_id,
                position: e_raw_position,
                level: e_level,
                scores:
                    RiotScoreboard {
                        kills,
                        deaths,
                        assists,
                        creep_score,
                        ..
                    },
                champion_name: e_champion_name,
                team: e_team,
            } = player;

            let e_champion_id = *CHAMPION_NAME_TO_ID.get(e_champion_name)?;
            let e_cache = unsafe { INTERNAL_CHAMPIONS.get_unchecked(e_champion_id as usize) };
            let e_position = Position::from_raw(e_raw_position)
                .unwrap_or(unsafe { *e_cache.positions.get_unchecked(0) });
            let team = Team::from(*e_team);

            scoreboard.push(Scoreboard {
                riot_id,
                assists: *assists,
                deaths: *deaths,
                kills: *kills,
                creep_score: *creep_score,
                champion_id: e_champion_id,
                position: e_position,
                team,
            });

            if team == current_player_team {
                return None;
            }

            let e_items = riot_items_to_set_u32(e_riot_items);
            let e_base_stats = base_stats_sf32(&e_cache.stats, *e_level, false);
            let full_state = get_enemy_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: e_items,
                    stacks: 0,
                    champion_id: e_champion_id,
                    level: *e_level,
                },
                shred,
                enemy_earth_dragons,
                false,
            );
            let eval_ctx = get_eval_ctx(&self_state, &full_state);
            let modifiers = {
                let map_modifier = match game_map {
                    GameMap::Aram => {
                        current_player_cache.stats.aram_damage_dealt
                            * e_cache.stats.aram_damage_taken
                    }
                    GameMap::Urf => {
                        current_player_cache.stats.urf_damage_dealt * e_cache.stats.urf_damage_taken
                    }
                    _ => 1.0,
                };

                let mut global_mod = map_modifier * full_state.modifiers.global_mod;

                if rune_exceptions.has_last_stand {
                    global_mod *= LAST_STAND_CLOSURE(eval_ctx.missing_health);
                }
                if rune_exceptions.has_coup_de_grace || rune_exceptions.has_cut_down {
                    global_mod *= COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE;
                }

                DamageModifiers {
                    physical_mod: full_state.armor_values.modifier
                        * full_state.modifiers.physical_mod,
                    magic_mod: full_state.magic_values.modifier * full_state.modifiers.magic_mod,
                    true_mod: full_state.modifiers.true_mod,
                    global_mod,
                }
            };
            let damages = get_damages(&eval_ctx, &eval_data, modifiers);
            let mut siml_results = MaybeUninit::<[Damages; L_SIML]>::uninit();
            let siml_result_ptr = siml_results.as_mut_ptr();

            for (i, siml_stat) in simulated_stats.into_iter().enumerate() {
                let siml_eval_ctx = get_eval_ctx(
                    &SelfState {
                        current_stats: siml_stat,
                        bonus_stats: self_state.bonus_stats,
                        base_stats: self_state.base_stats,
                        ability_levels: self_state.ability_levels,
                        level: *level,
                    },
                    &full_state,
                );
                let siml_damages = get_damages(&siml_eval_ctx, &eval_data, modifiers);
                unsafe {
                    core::ptr::addr_of_mut!((*siml_result_ptr)[i]).write(siml_damages);
                }
            }

            Some(Enemy {
                champion_id: e_champion_id,
                position: e_position,
                team,
                riot_id,
                damages,
                siml_items: unsafe { siml_results.assume_init() },
                base_stats: e_base_stats.into(),
                bonus_stats: full_state.bonus_stats.into(),
                current_stats: full_state.current_stats.into(),
                real_armor: full_state.armor_values.real as i32,
                real_magic_resist: full_state.magic_values.real as i32,
                level: *e_level,
            })
        })
        .collect::<SmallVec<[Enemy<'_>; L_TEAM]>>();

    Some(Realtime {
        current_player: CurrentPlayer {
            riot_id,
            base_stats: current_player_base_stats.into(),
            bonus_stats: current_player_bonus_stats.into(),
            current_stats: (*champion_stats).into(),
            level: *level,
            team: current_player_team,
            adaptative_type,
            position: current_player_position,
            champion_id: current_player_champion_id,
            game_map,
        },
        enemies,
        scoreboard,
        abilities_meta: eval_data.abilities.metadata,
        items_meta: eval_data.items.metadata,
        runes_meta: eval_data.runes.metadata,
        siml_meta: SIMULATED_ITEMS_METADATA,
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
