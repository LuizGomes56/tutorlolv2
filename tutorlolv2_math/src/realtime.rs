use super::{helpers::*, model::*};
use crate::{L_ITEM, L_SIML, L_TEAM, RiotFormulas, riot::*};
use core::mem::MaybeUninit;
use smallvec::SmallVec;
use tutorlolv2_gen::{
    CHAMPION_CACHE, CHAMPION_NAME_TO_ID, ChampionId, DAMAGING_ITEMS, DAMAGING_RUNES, GameMap,
    ITEM_CACHE, ItemId, ItemsBitSet, Position, RuneId, RunesBitSet, SIMULATED_ITEMS_ENUM,
    TypeMetadata,
};

/// Contains the metadata of all items that have their stats compared to choose
/// which one is best to buy considering the current game state. See [`TypeMetadata`]
/// for more details
pub const SIMULATED_ITEMS_METADATA: [TypeMetadata<ItemId>; L_SIML] = {
    let mut siml_items = MaybeUninit::<[TypeMetadata<ItemId>; L_SIML]>::uninit();
    let siml_items_ptr = siml_items.as_mut_ptr();
    let mut i = 0;
    while i < L_SIML {
        let item_id = SIMULATED_ITEMS_ENUM[i];
        let item_cache = ITEM_CACHE[item_id as usize];
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

/// Receives a reference to the current player's game, defined by the struct [`RiotRealtime`]
/// and returns a new [`Option<Realtime>`], which contains all the information that could be
/// extracted from the input struct. See [`Realtime`] for more information
/// This function assumes that the data in the input struct is valid, and does several
/// memory assumptions and use unsafe blocks to avoid unnecessary branches, making this
/// code faster, but requires the use of unsafe features
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

    let game_map = GameMap::from_u8(*map_number);
    let mut ability_modifiers = AbilityModifiers::default();
    let mut base_modifiers = DamageModifiers::default();

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == *riot_id)?;

    let current_player_champion_id = *CHAMPION_NAME_TO_ID.get(current_player.champion_name)?;
    let current_player_cache =
        unsafe { CHAMPION_CACHE.get_unchecked(current_player_champion_id as usize) };

    let is_mega_gnar =
        current_player_champion_id == ChampionId::Gnar && champion_stats.attack_range >= 400.0;

    let current_player_base_stats =
        BasicStats::base_stats(current_player_champion_id, *level, is_mega_gnar);

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
    )
    .unwrap_or(current_player_cache.adaptative_type);

    const SHADOWFLAME: u32 = ItemId::Shadowflame.to_riot_id();
    const RIFTMAKER: u32 = ItemId::Riftmaker.to_riot_id();
    const SPEAR_OF_SHOJIN: u32 = ItemId::SpearOfShojin.to_riot_id();

    let current_player_items = current_player
        .items
        .iter()
        .filter_map(|riot_item| {
            let riot_id = riot_item.item_id;

            match riot_id {
                RIFTMAKER => base_modifiers.global_mod *= 1.08,
                SHADOWFLAME => {
                    base_modifiers.magic_mod *= 1.2;
                    base_modifiers.true_mod *= 1.2;
                }
                SPEAR_OF_SHOJIN => {
                    ability_modifiers.q *= 1.12;
                    ability_modifiers.w *= 1.12;
                    ability_modifiers.e *= 1.12;
                    ability_modifiers.r *= 1.12;
                }
                _ => {}
            };

            let item_id = ItemId::from_riot_id(riot_id)? as _;
            DAMAGING_ITEMS.contains(item_id).then_some(item_id)
        })
        .collect::<ItemsBitSet>();

    let dragons = get_dragons(&events, &all_players);

    let enemy_earth_dragons = dragons.enemy_earth_dragons;
    let simulated_stats = get_simulated_stats(&champion_stats, dragons);
    let ability_levels = abilities.get_ability_levels();
    let current_player_position = Position::from_raw(current_player.position)
        .unwrap_or(unsafe { *current_player_cache.positions.get_unchecked(0) });
    let current_player_cache_attack_type = current_player_cache.attack_type;

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
        adaptative_type,
        ability_levels,
        level: *level,
    };

    const LAST_STAND: u32 = RuneId::LastStand.to_riot_id();
    const COUP_DE_GRACE: u32 = RuneId::CoupDeGrace.to_riot_id();
    const CUT_DOWN: u32 = RuneId::CutDown.to_riot_id();
    const AXIOM_ARCANIST: u32 = RuneId::AxiomArcanist.to_riot_id();

    let current_player_runes = general_runes
        .as_ref()
        .and_then(|gr| {
            Some(
                gr.into_iter()
                    .filter_map(|riot_rune| {
                        let riot_id = riot_rune.id;

                        match riot_id {
                            AXIOM_ARCANIST => ability_modifiers.r *= AXIOM_ARCANIST_BONUS_DAMAGE,
                            COUP_DE_GRACE | CUT_DOWN => {
                                base_modifiers.global_mod *= COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE
                            }
                            LAST_STAND => {
                                base_modifiers.global_mod *= get_last_stand(
                                    1.0 - (self_state.current_stats.current_health
                                        / self_state.current_stats.health.max(1.0)),
                                )
                            }
                            _ => {}
                        };

                        let rune_id = RuneId::from_riot_id(riot_id)? as _;
                        DAMAGING_RUNES.contains(rune_id).then_some(rune_id)
                    })
                    .collect::<RunesBitSet>(),
            )
        })
        .unwrap_or_default();

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        items: get_items_data(&current_player_items, current_player_cache_attack_type),
        runes: get_runes_data(&current_player_runes, current_player_cache_attack_type),
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
                    },
                champion_name: e_champion_name,
                team: e_team,
            } = player;

            let e_champion_id = *CHAMPION_NAME_TO_ID.get(e_champion_name)?;
            let e_cache = unsafe { CHAMPION_CACHE.get_unchecked(e_champion_id as usize) };
            let e_position = Position::from_raw(e_raw_position)
                .unwrap_or(unsafe { *e_cache.positions.get_unchecked(0) });
            let team = Team::from(*e_team);

            scoreboard.push(Scoreboard {
                riot_id,
                assists: *assists as _,
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

            let e_items = e_riot_items
                .iter()
                .filter_map(|riot_item| Some(ItemId::from_riot_id(riot_item.item_id)? as _))
                .collect::<SmallVec<[_; L_ITEM]>>();

            let e_base_stats = SimpleStats::base_stats(e_champion_id, *e_level, false);
            let full_state = get_enemy_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: &e_items,
                    stacks: 0,
                    champion_id: e_champion_id,
                    level: *e_level,
                    item_exceptions: &[],
                    earth_dragons: enemy_earth_dragons,
                },
                shred,
                false,
            );
            let eval_ctx = get_eval_ctx(&self_state, &full_state);
            let modifiers = Modifiers {
                abilities: ability_modifiers,
                damages: DamageModifiers {
                    physical_mod: base_modifiers.physical_mod
                        * full_state.armor_values.modifier
                        * full_state.modifiers.physical_mod,
                    magic_mod: base_modifiers.magic_mod
                        * full_state.magic_values.modifier
                        * full_state.modifiers.magic_mod,
                    true_mod: base_modifiers.true_mod * full_state.modifiers.true_mod,
                    global_mod: base_modifiers.global_mod
                        * match game_map {
                            GameMap::Aram => {
                                current_player_cache.stats.aram_damage_dealt
                                    * e_cache.stats.aram_damage_taken
                            }
                            GameMap::Urf => {
                                current_player_cache.stats.urf_damage_dealt
                                    * e_cache.stats.urf_damage_taken
                            }
                            _ => 1.0,
                        }
                        * full_state.modifiers.global_mod,
                },
            };
            let damages = get_damages(&eval_ctx, &eval_data, modifiers);

            let siml_items = core::array::from_fn(|i| {
                let siml_stat = simulated_stats[i];
                let siml_eval_ctx = get_eval_ctx(
                    &SelfState {
                        current_stats: siml_stat,
                        ..self_state
                    },
                    &full_state,
                );
                get_damages(&siml_eval_ctx, &eval_data, modifiers)
            });

            Some(Enemy {
                champion_id: e_champion_id,
                position: e_position,
                team,
                riot_id,
                damages,
                siml_items,
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
        abilities_to_merge: current_player_cache.merge_data,
        game_time: *game_time as u32,
        ability_levels,
        dragons,
    })
}

/// Reads all game events and looks for dragon kills, returning a struct with
/// the number of killed dragons for each team. A slice with information about
/// all players in the game needs to be provided so the dragon kill per team
/// can be assigned correctly. Dragons that do not guarantee buffs that can
/// affect damage calculations are ignored.
pub fn get_dragons(events: &[RealtimeEvent], players: &[RiotAllPlayers]) -> Dragons {
    let mut dragons = Dragons::default();
    for event in events {
        if let (Some(killer), Some(dragon)) =
            (event.killer_name.as_deref(), event.dragon_type.as_deref())
        {
            match dragon {
                "Earth" => match players.iter().any(|player| player.riot_id == killer) {
                    true => dragons.ally_earth_dragons += 1,
                    false => dragons.enemy_earth_dragons += 1,
                },
                "Fire" => dragons.ally_fire_dragons += 1,
                "Chemtech" => dragons.ally_chemtech_dragons += 1,
                _ => {}
            }
        }
    }

    dragons
}
