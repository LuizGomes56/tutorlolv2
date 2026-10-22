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

use {
    crate::{ChampionId, ItemId, RuneId, helpers::*, model::*, riot::*},
    alloc::boxed::Box,
    core::str::FromStr,
    strum::{EnumCount, VariantArray},
    tutorlolv2_types::{AdaptiveType, AttackType, GameMap, Position},
};

/// Ensure that all champions have at least one position, so the unchecked
/// access does not cause a panic or undefined behavior
const _: () = {
    let mut i = 0;
    while i < ChampionId::COUNT {
        let champion_id = ChampionId::VARIANTS[i];
        assert!(!champion_id.positions().is_empty());
        i += 1;
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealtimeError<'a> {
    UnrecognizedCurrentPlayer(&'a str),
}

impl core::fmt::Display for RealtimeError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id = match self {
            RealtimeError::UnrecognizedCurrentPlayer(s) => s,
        };

        write!(
            f,
            "Unable to find current player with `riot_id`: {id} in field `all_players`"
        )
    }
}

impl core::error::Error for RealtimeError<'_> {}

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
    let game_map = GameMap::from_code(map_number);

    let current_player = all_players
        .iter()
        .find(|player| player.riot_id == riot_id)
        .ok_or(RealtimeError::UnrecognizedCurrentPlayer(riot_id))?;

    // If Neeko is disguised as a non-champion entity, the champion name changes
    // to the entity's name. Instead of tracking all possible entities, just make
    // her the default if the champion name is not recognized. Note that
    // [`ChampionId::default`] has a different value
    let current_player_champion_id = current_player
        .champion_name
        .parse()
        .unwrap_or(ChampionId::Neeko);

    let current_player_cache = current_player_champion_id.data();

    // When Gnar is Mega, the current API changes `champion_name`, previously we
    // had to infer it from his attack range
    let is_mega_gnar = current_player.champion_name == "Mega Gnar";

    let current_player_base_stats =
        BasicStats::base_stats(current_player_champion_id, level, is_mega_gnar);

    // This is the difference between the current stats and the base values
    let current_player_bonus_stats = current_player_stats.bonus_stats(current_player_base_stats);

    // If the player hasn't bought any damaging items, the function defaults,
    // but since we have the champion's adaptive damage, we can use that instead
    let adaptive_type = AdaptiveType::try_infer(
        current_player_bonus_stats.attack_damage,
        current_player_stats.ability_power,
    )
    .unwrap_or(current_player_cache.adaptive_type);

    let owned_items = current_player
        .items
        .iter()
        .filter_map(|item| ItemId::from_riot_id(item.item_id))
        .collect::<Box<[_]>>();

    let current_player_items = get_damaging_items(&owned_items);

    // If we don't know his team, put in team `Team::default()`, which by now is `Team::Blue`.
    // Note that it may lead to incorrect results, but it is better than not giving any results
    let current_player_team = current_player.team.parse().unwrap_or_default();

    let dragons = get_dragons(events, all_players, current_player_team);

    let enemy_earth_dragons = dragons.enemy_earth_dragons;

    // Get an array of stats as if the player had bought all of these items. It is not limited to
    // damaging items. Any item that meets the criteria of `L_SIML` will be included
    let ability_levels = abilities.ability_levels();

    // If we don't know the player's position as in practice mode, we assume he is playing in the
    // most common position for his champion
    let current_player_position = Position::from_str(current_player.position)
        .unwrap_or(current_player_champion_id.main_position());

    // Assume champion attack types cannot change. Mayhem and Arena have augments that change it,
    // but there's no way to know
    let current_player_cache_attack_type =
        current_player_champion_id.effective_attack_type(is_mega_gnar);

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

    let owned_runes = general_runes
        .iter()
        .flatten()
        .filter_map(|rune| RuneId::from_riot_id(rune.id))
        .collect::<Box<[_]>>();

    let current_player_runes = get_damaging_runes(&owned_runes);
    let mut base_modifiers = Modifiers::default();

    base_modifiers.apply_loadout(&owned_items, &owned_runes, &current_player_stats);

    // Candidate self stats are independent of the target; defenses are recalculated below.
    let simulated_states: [_; ItemId::L_SIML] = core::array::from_fn(|i| {
        self_state.simulated_purchase(
            ItemId::SIML[i],
            &owned_items,
            current_player_champion_id,
            dragons,
        )
    });

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
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
            let e_champion_id = e_champion_name.parse().unwrap_or(ChampionId::Neeko);
            let team = e_team.parse().unwrap_or_default();

            let e_position = e_raw_position
                .parse()
                .unwrap_or(e_champion_id.main_position());

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
            let e_base_stats = SimpleStats::base_stats(e_champion_id, e_level, false);

            // Holds a collection of more detailed information about this enemy player
            let full_state = EnemyState {
                base_stats: e_base_stats,
                items: &e_items,
                champion_id: e_champion_id,
                level: e_level,
                earth_dragons: enemy_earth_dragons,
                ..Default::default()
            }
            .full_state(shred, false);

            // Get our values ready to be used when evaluating all damages
            let ctx = self_state.ctx(&full_state);

            // We will multiply the final damage of each ability, item, and rune
            // with these modifiers according to their damage type. `global_mod` is always
            // multiplied regardless
            let mut attacker_modifiers = base_modifiers;

            attacker_modifiers.damages.global_mod *=
                current_player_champion_id.map_damage_modifier(&e_champion_id, game_map);

            let modifiers = attacker_modifiers.against(&full_state, adaptive_type);

            // Calculate damages of each ability, item, and rune
            let damages = Damages::new(ctx, &eval_data, modifiers);

            // Do the same, but testing a fixed amount of items that the player
            // is likely to buy, it can be used to compare which one would output
            // the greatest amount of damage, or what difference they make
            let siml_items = core::array::from_fn(|i| {
                let candidate = ItemId::SIML[i];
                let state = &simulated_states[i];
                let target = full_state.with_shred(ResistShred::new(&state.current_stats), false);
                let ctx = state.ctx(&target);
                let mut buffs = Modifiers::default();

                buffs.apply_loadout(&owned_items, &owned_runes, &state.current_stats);

                if !owned_items.contains(&candidate) {
                    buffs.apply_item(candidate);
                }

                buffs.damages.global_mod *=
                    current_player_champion_id.map_damage_modifier(&e_champion_id, game_map);

                let modifiers = buffs.against(&target, state.adaptive_type);
                let mut damage = Damages::new(ctx, &eval_data, modifiers);

                // Existing pairs retain items_meta order. A new candidate's pair is
                // appended, identified by SIML[i], rather than silently omitted.
                if candidate.deals_damage() && !current_player_items.contains_const(candidate as _)
                {
                    let item = candidate.data();
                    let closures = match current_player_cache_attack_type {
                        AttackType::Melee => &item.melee,
                        AttackType::Ranged => &item.ranged,
                    };

                    let extra = Damages::eval(
                        &ctx,
                        &mut damage.attacks.onhit_damage,
                        modifiers,
                        &[item.metadata],
                        closures,
                    );

                    let mut items = damage.items.into_vec();

                    items.extend_from_slice(&extra);
                    damage.items = items.into_boxed_slice();
                }

                damage
            });

            Some(Enemy {
                champion_id: e_champion_id,
                position: e_position,
                team,
                riot_id,
                damages,
                siml_items,
                base_stats: e_base_stats,
                bonus_stats: full_state.bonus_stats,
                current_stats: full_state.current_stats,
                real_armor: full_state.armor_values.real as _,
                real_magic_resist: full_state.magic_values.real as _,
                level: e_level.get(),
            })
        })
        .collect::<Box<[Enemy<'_>]>>();

    Ok(Realtime {
        current_player: CurrentPlayer {
            riot_id,
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: current_player_stats,
            level: level.get(),
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
pub fn get_dragons(events: &[RealtimeEvent], players: &[RiotAllPlayers], team: Team) -> Dragons {
    let mut dragons = Dragons::default();

    for RealtimeEvent {
        dragon_type,
        killer_name,
    } in events
    {
        // Both must be defined otherwise we don't know which team got the buff from this event
        if let Some(killer) = *killer_name
            // We need to know the dragon type
            && let Some(dragon) = *dragon_type
            // We need to know the player's team
            && let Some(player) = players.iter().find(|p| p.riot_id == killer)
        {
            let player_team = Team::from_str(player.team).unwrap_or_default();
            let ally = player_team == team;

            match dragon {
                "Earth" if ally => dragons.ally_earth_dragons += 1,
                "Earth" if !ally => dragons.enemy_earth_dragons += 1,
                "Fire" if ally => dragons.ally_fire_dragons += 1,
                "Chemtech" if ally => dragons.ally_chemtech_dragons += 1,
                _ => {}
            }
        }
    }

    dragons
}
