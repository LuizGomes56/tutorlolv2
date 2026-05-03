//! This module exports functions related to the `livegame` feature,
//! and data that is obtained through Riot's endpoint for realtime
//! game data. In local port 2999, you can find a json output that
//! look like the struct [`RiotRealtime`], in which you can use
//! `serde_json` library to turn it into a Rust's struct and call the
//! function `realtime` exported in this module, obtaining a [`Realtime`]
//! struct that contains the evaluated damage against every enemy in that
//! live game.
//!
//! Check the module [`tutorlolv2::riot`] for more information about the
//! types extracted from the json file in port 2999

use crate::{helpers::*, model::*, riot::*};
use alloc::boxed::Box;
use core::str::FromStr;
use tutorlolv2_gen::*;

/// Ensure that all champions have at least one position, so the unchecked
/// access does not cause a panic or undefined behavior
const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion_id = ChampionId::VALUES[i];
        assert!(!champion_id.cache().positions.is_empty());
        i += 1;
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealtimeError<'a> {
    UnrecognizedCurrentPlayer(&'a str),
}

/// Receives a reference to the current player's game, defined by the struct [`RiotRealtime`]
/// and returns a new [`Option<Realtime>`], which contains all the information that could be
/// extracted from the input struct. See [`Realtime`] for more information
///
/// This function assumes that the data in the input struct is valid, and does several
/// memory assumptions and use unsafe blocks to avoid unnecessary branches, making this
/// code faster, but requires the use of unsafe features.
///
/// You can get these input values from port `2999` in the user's local machine if they're
/// in an active League of Legends game. You may fetch the bytes contained in this route
/// and deserialize them into [`RiotRealtime`] struct, and call this function afterwards
/// to get the calculated data.
///
/// This function takes about 100 micro-seconds to run with an average input for a game
/// at the 20 minutes mark.
///
/// ## Example of usage with `livegame` or `serde` features
/// ```rs
/// use reqwest::Client;
/// use tutorlolv2::realtime;
///
/// // If feature `livegame` is enabled
/// let port_2999_bytes = Client::get(
///     "http://127.0.0.1:2999/liveclientdata/allgamedata"
/// )
/// .await?
/// .bytes()
/// .await?;
/// let game = serde_json::from_slice(&port_2999_bytes)?;
/// let data = realtime(&game).ok_or("Failed to run `tutorlolv2::realtime`")?;
/// ```
///
/// If feature `livegame` or `serde` are not enabled, it is your job to get a
/// struct [`RiotRealtime`] and call this function
pub fn realtime<'a>(game: &'a RiotRealtime) -> Result<Realtime<'a>, RealtimeError<'a>> {
    let RiotRealtime {
        active_player:
            RiotActivePlayer {
                abilities,
                champion_stats,
                full_runes: RiotFullRunes { ref general_runes },
                level,
                riot_id,
            },
        ref all_players,
        events: RiotRealtimeEvents { ref events },
        game_data: RiotRealtimeGameData {
            game_time,
            map_number,
        },
    } = *game;

    let current_player_stats = champion_stats.base100();

    // Defaults to Unknown, which doesn't  make much difference
    let game_map = GameMap::from_u8(map_number);
    let mut ability_modifiers = AbilityModifiers::default();
    let mut base_modifiers = DamageModifiers::default();

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == riot_id)
        .ok_or(RealtimeError::UnrecognizedCurrentPlayer(riot_id))?;

    // If Neeko is disguised as a non-champion entity, the champion name changes
    // to the entity's name. Instead of tracking all possible entities, just make
    // her the default if the champion name is not recognized. Note that
    // [`ChampionId::default`] has a different value
    let current_player_champion_id =
        ChampionId::from_str(current_player.champion_name).unwrap_or(ChampionId::Neeko);
    let current_player_cache = current_player_champion_id.cache();

    // When Gnar is Mega, the current API changes `champion_name`, previously we
    // had to infer it from his attack range
    let is_mega_gnar = current_player.champion_name == "Mega Gnar";

    let current_player_base_stats =
        BasicStats::infer(current_player_champion_id, level, is_mega_gnar);

    // This is the difference between the current stats and the base values
    let current_player_bonus_stats = bonus_stats!(
        BasicStats::<f32>(current_player_stats, current_player_base_stats) {
            armor,
            max_health,
            attack_damage,
            magic_resist,
            max_mana
        }
    );

    // If the player hasn't bought any damaging items, the function defaults,
    // but since we have the champion's adaptive damage, we can use that instead
    let adaptive_type = RiotFormulas::adaptive_type(
        current_player_bonus_stats.attack_damage,
        current_player_stats.ability_power,
    )
    .unwrap_or(current_player_cache.adaptive_type);

    // We ignore items our API don't know they exist, this is a safe approach
    // but may be harder to debug or notice wrong values
    let current_player_items = current_player
        .items
        .iter()
        .filter_map(|riot_item| {
            let riot_id = riot_item.item_id;
            let item_id = ItemId::from_riot_id(riot_id)?;

            // We're not going to iterate through all items again, so we can
            // calculate these damage modifiers before the end of this function
            match item_id {
                // Assume the passive is always active. Damages will be higher than
                // they really are in most situations
                ItemId::Riftmaker | ItemId::RiftmakerArena => {
                    base_modifiers.global_mod *= RiotFormulas::RIFTMAKER_BONUS_DAMAGE
                }
                // Adding this makes it the best ability power item in the game because
                // it will consider that the enemy is always at low health. Removing this
                // block will make it far less effective than it really is
                ItemId::Shadowflame | ItemId::ShadowflameArena => {
                    base_modifiers.magic_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
                    base_modifiers.true_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
                }
                // Similar to Shadowflame, we're considering the item's passive is always
                // fully stacked since there's no way we know what the current stack count is
                ItemId::SpearOfShojin | ItemId::SpearOfShojinArena => {
                    ability_modifiers.q *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    ability_modifiers.w *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    ability_modifiers.e *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    ability_modifiers.r *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                }
                _ => {}
            };

            let item_index = item_id as _;

            // If the item does not define any personalized closure, we ignore it otherwise we
            // would be wasting time evaluating zeros
            DAMAGING_ITEMS
                .contains_const(item_index)
                .then_some(item_index)
        })
        .collect::<ItemsBitSet>();

    let dragons = get_dragons(events, all_players);

    let enemy_earth_dragons = dragons.enemy_earth_dragons;

    // Get an array of stats as if the player had bought all of these items. It is not limited to
    // damaging items. Any item that meets the criteria of `L_SIML` will be included
    let simulated_stats = current_player_stats.get_simulated_stats(dragons);
    let ability_levels = abilities.ability_levels();

    // If we don't know the player's position as in practice mode, we assume he is playing in the
    // most common position for his champion
    let current_player_position = Position::from_str(current_player.position)
        .unwrap_or(current_player_champion_id.main_position());

    // Assume champion attack types cannot change. Mayhem and Arena have augments that change it,
    // but there's no way to know
    let current_player_cache_attack_type = current_player_cache.attack_type;

    // If we don't know his team, put in team `Team::default()`, which by now is `Team::Blue`.
    // Note that it may lead to incorrect results, but it is better than not giving any results
    let current_player_team = Team::from_str(current_player.team).unwrap_or_default();

    let shred = ResistShred::new(&current_player_stats);

    // Scoreboard can't have more entries than the number of players, so we allocate enough space
    // for all of them. We're always assigning every player we find under `all_players` inside this
    // slice, and since we use default values if we can't infer the name, position, or team, there's
    // no chance of non-initialized values.
    let mut scoreboard = Box::new_uninit_slice(all_players.len());

    let self_state = SelfState {
        current_stats: current_player_stats,
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        adaptive_type,
        ability_levels,
        level,
        stacks: get_stacks(current_player_champion_id, game_time),
    };

    let current_player_runes = general_runes
        .as_ref()
        .map(|gr| {
            gr.into_iter()
                .filter_map(|riot_rune| {
                    let riot_id = riot_rune.id;
                    // We ignore runes we don't know they exist
                    let rune_id = RuneId::from_riot_id(riot_id)?;

                    // Similar to items, we're not iterating over all runes again, so this
                    // is the chance to assign modifiers as necessary
                    match rune_id {
                        RuneId::AxiomArcanist => {
                            ability_modifiers.r *= RiotFormulas::AXIOM_ARCANIST_BONUS_DAMAGE
                        }
                        RuneId::CoupDeGrace | RuneId::CutDown => {
                            base_modifiers.global_mod *=
                                RiotFormulas::COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE
                        }
                        RuneId::LastStand => {
                            base_modifiers.global_mod *= RiotFormulas::get_last_stand(
                                1.0 - (self_state.current_stats.current_health
                                    / self_state.current_stats.max_health.max(1.0)),
                            )
                        }
                        _ => {}
                    };

                    let rune_index = rune_id as _;

                    // Most runes will be filtered out, very few ones deal damage in the game
                    DAMAGING_RUNES
                        .contains_const(rune_index)
                        .then_some(rune_index)
                })
                .collect::<RunesBitSet>()
        })
        // If there's no runes field, the final value will be an empty bit set indicating
        // that there are no runes selected
        .unwrap_or_default();

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        // Get closures and metadata for items and runes. Items always have two closures
        // for each, while runes have only a single one. It will have to be changed as
        // now there's one rune whose damage is measured in a range x..1.75x
        items: DamageKind::items(&current_player_items, current_player_cache_attack_type),
        runes: DamageKind::runes(&current_player_runes, current_player_cache_attack_type),
    };

    let enemies = all_players
        .into_iter()
        .enumerate()
        .filter_map(|(i, player)| {
            let RiotAllPlayers {
                items: ref e_riot_items,
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
            } = *player;

            // Similarly, if Neeko is disguised the champion name might change, causing mismatches. The loop cannot
            // return until the scoreboard is fully populated, otherwise we will cause undefined behavior or have to
            // reallocate it at the end
            let e_champion_id = ChampionId::from_str(e_champion_name).unwrap_or(ChampionId::Neeko);
            let e_cache = e_champion_id.cache();
            let e_position =
                Position::from_str(e_raw_position).unwrap_or(e_champion_id.main_position());
            let team = Team::from_str(e_team).unwrap_or_default();

            unsafe {
                scoreboard.get_unchecked_mut(i).write(Scoreboard {
                    riot_id,
                    assists: assists as _,
                    deaths,
                    kills,
                    creep_score,
                    champion_id: e_champion_id,
                    position: e_position,
                    team,
                })
            };

            // We don't care about players in the ally team
            if team == current_player_team {
                return None;
            }

            // Filter enemy items that we know about
            let e_items = e_riot_items
                .iter()
                .filter_map(|riot_item| Some(ItemId::from_riot_id(riot_item.item_id)? as _))
                .collect::<Box<[_]>>();

            // Works the same as the current player basic stats inference, but with less fields
            // since the other ones are unnecessary
            let e_base_stats = SimpleStats::infer(e_champion_id, e_level, false);

            // Holds a collection of more detailed information about this enemy player
            let full_state = get_enemy_full_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: &e_items,
                    champion_id: e_champion_id,
                    level: e_level,
                    earth_dragons: enemy_earth_dragons,
                    ..Default::default()
                },
                shred,
                false,
            );

            // Get our values ready to be used when evaluating all damages
            let ctx = get_eval_ctx(&self_state, &full_state);

            // We will multiply the final damage of each ability, item, and rune
            // with these modifiers according to their damage type. `global_mod` is always
            // multiplied regardless
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
                            // More maps will be added soon
                            _ => 1.0,
                        }
                        * full_state.modifiers.global_mod,
                },
            };

            // Calculate damages of each ability, item, and rune
            let damages = Damages::new(ctx, &eval_data, modifiers);

            // Do the same, but testing a fixed amount of items that the player
            // is likely to buy, it can be used to compare which one would output
            // the greatest amount of damage, or what difference they make
            let siml_items = core::array::from_fn(|i| {
                let siml_stat = simulated_stats[i];
                let siml_ctx = get_eval_ctx(
                    &SelfState {
                        current_stats: siml_stat,
                        ..self_state
                    },
                    &full_state,
                );
                Damages::new(siml_ctx, &eval_data, modifiers)
            });

            // Collect the result, casting `f32` values to `i32` since the game rounds down
            // everything, and it is more compact to send through bincode. `f32` takes up
            // 4 bytes, while an `i32` most of the times will be collapsed to 1 single byte
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
                real_armor: full_state.armor_values.real as _,
                real_magic_resist: full_state.magic_values.real as _,
                level: e_level,
            })
        })
        .collect::<Box<[Enemy<'_>]>>();

    Ok(Realtime {
        current_player: CurrentPlayer {
            riot_id,
            base_stats: BasicStats::from_f32(&current_player_base_stats),
            bonus_stats: BasicStats::from_f32(&current_player_bonus_stats),
            current_stats: Stats::from_f32(&current_player_stats),
            level,
            team: current_player_team,
            adaptive_type,
            position: current_player_position,
            champion_id: current_player_champion_id,
            game_map,
        },
        enemies,
        // At this point, the scoreboard is fully populated and can be used safely
        scoreboard: unsafe { scoreboard.assume_init() },
        items_meta: eval_data.items.metadata,
        runes_meta: eval_data.runes.metadata,
        game_time: game_time as _,
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
        // Both must be defined otherwise we don't know which team got the buff from this event
        if let Some(killer) = event.killer_name
            // We need to know the dragon type
            && let Some(dragon) = event.dragon_type
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
