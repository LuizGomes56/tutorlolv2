//! This module exports functions related to the `livegame` feature,
//! and data that is obtained through Riot's endpoint for realtime
//! game data. In local port 2999, you can find a json output that
//! look like the struct [`RiotRealtime`], in which you can use
//! `serde_json` library to turn it into a Rust's struct and call the
//! function `realtime` exported in this module, obtaining a [`Realtime`]
//! struct that contains the evaluated damage against every enemy in that
//! live game.
//!
//! Check the module [`math::riot`] for more information about the
//! types extracted from the json file in port 2999

use {
    crate::{
        ChampionId, ItemId, RuneId,
        combo::{ComboError, ComboSource, ComboTarget, MAX_TABLE_BYTES},
        helpers::*,
        model::*,
        riot::*,
    },
    alloc::boxed::Box,
    bincode::{BorrowDecode, Encode},
    core::{num::NonZeroU8, str::FromStr},
    serde::{Deserialize, Serialize},
    strum::{EnumCount, VariantArray},
    types::{AdaptiveType, AttackType, GameMap, Position},
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
    Combo(ComboError),
}

impl core::fmt::Display for RealtimeError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RealtimeError::UnrecognizedCurrentPlayer(id) => write!(
                f,
                "Unable to find current player with `riot_id`: {id} in field `all_players`"
            ),
            RealtimeError::Combo(error) => write!(f, "Invalid combo: {error}"),
        }
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
/// use math::realtime;
///
/// // If feature `livegame` is enabled
/// let port_2999_bytes = Client::get(
///     "http://127.0.0.1:2999/liveclientdata/allgamedata"
/// )
/// .await?
/// .bytes()
/// .await?;
/// let game = serde_json::from_slice(&port_2999_bytes)?;
/// let data = realtime(&game).ok_or("Failed to run `math::realtime`")?;
/// ```
///
/// If feature `livegame` or `serde` are not enabled, it is your job to get a
/// struct [`RiotRealtime`] and call this function
pub fn realtime<'a>(game: &'a RiotRealtime) -> Result<Realtime<'a>, RealtimeError<'a>> {
    realtime_with_combo(game, &[])
}

/// A realtime frame with the combo of the viewer, see [`realtime_with_combo`]
#[derive(Clone, Debug, PartialEq, PartialOrd, Encode, BorrowDecode, Deserialize, Serialize)]
pub struct RealtimeRequest<'a> {
    #[serde(borrow)]
    pub frame: RiotRealtime<'a>,
    /// Steps of the combo, before expansion. May be empty
    #[serde(default)]
    pub combo: Box<[ComboStep]>,
}

impl RealtimeRequest<'_> {
    pub fn eval(&self) -> Result<Realtime<'_>, RealtimeError<'_>> {
        realtime_with_combo(&self.frame, &self.combo)
    }
}

/// Same as [`realtime`], and evaluates `combo` on every row of the table, see
/// [`crate::combo`]. The expanded steps are in [`Realtime::combo`] and each row's
/// results in [`DamageTable::combo`]
pub fn realtime_with_combo<'a>(
    game: &'a RiotRealtime,
    combo: &[ComboStep],
) -> Result<Realtime<'a>, RealtimeError<'a>> {
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

    // Neeko's disguises and Gnar's forms change `champion_name`, while
    // `raw_skin_name` keeps the real champion
    let current_player_champion_id = current_player.champion_id();
    let current_player_cache = current_player_champion_id.data();
    let is_mega_gnar = current_player.is_mega_gnar(current_player_champion_id);

    // Disguised as a non-champion entity, Neeko reports the entity's level. Her
    // ability points can't exceed her real level, so they give a lower bound
    let level = if current_player.is_disguised_neeko(current_player_champion_id) {
        core::hint::cold_path();

        let RiotAbilities { q, w, e, r } = abilities;
        let points = [q, w, e, r].iter().fold(0u8, |sum, ability| {
            sum.saturating_add(ability.ability_level)
        });

        NonZeroU8::new(points).map_or(level, |points| level.max(points))
    } else {
        level
    };

    // Health granted by the map is base health, so bonus health scalings
    // exclude it from the observed health
    let current_player_base_stats =
        current_player_champion_id.map_base_stats(level, is_mega_gnar, game_map);

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

    let self_state = SelfState {
        bonus_attack_speed_ratio: current_player_champion_id
            .stats_on(game_map)
            .bonus_attack_speed_ratio(current_player_stats.attack_speed),
        bonus_move_speed: current_player_champion_id
            .stats_on(game_map)
            .bonus_move_speed(current_player_stats.move_speed),
        current_stats: current_player_stats,
        base_stats: current_player_base_stats,
        adaptive_type,
        ability_levels,
        level,
        stacks: current_player_champion_id.avg_stacks(game_time),
    };

    let owned_runes = general_runes
        .iter()
        .flatten()
        .filter_map(|rune| RuneId::from_riot_id(rune.id))
        .collect::<Box<[_]>>();

    let current_player_runes = get_damaging_runes(&owned_runes);
    let mut base_modifiers = const { Modifiers::default() };

    base_modifiers.apply_loadout(&owned_items, &owned_runes, &current_player_stats);

    let fire = RiotFormulas::get_fire_multiplier(dragons.ally_fire_dragons);
    let earth = RiotFormulas::get_earth_multiplier(dragons.ally_earth_dragons);

    // Prepare attacker-side inputs once; target defenses and contexts remain per enemy.
    let simulated_candidates: [_; ItemId::L_SIML] = core::array::from_fn(|i| {
        let candidate = ItemId::SIML[i];
        let state = self_state.simulated_purchase_with_multipliers(
            candidate,
            &owned_items,
            current_player_champion_id,
            fire,
            earth,
            game_map,
        );

        let shred = ResistShred::new(&state.current_stats);
        let mut buffs = Modifiers::default();

        buffs.apply_loadout(&owned_items, &owned_runes, &state.current_stats);

        if !owned_items.contains(&candidate) {
            buffs.apply_item(candidate);
        }

        let extra_item =
            if candidate.deals_damage() && !current_player_items.contains_const(candidate as _) {
                let item = candidate.data();
                let closures = match current_player_cache_attack_type {
                    AttackType::Melee => &item.melee,
                    AttackType::Ranged => &item.ranged,
                };

                Some((item.metadata, closures))
            } else {
                None
            };

        (state, shred, buffs, extra_item)
    });

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        items: DamageKind::items(&current_player_items, current_player_cache_attack_type),
        runes: DamageKind::runes(&current_player_runes, current_player_cache_attack_type),
        // Rune stacks are not in the live game data
        rune_stacks: RuneStacks::NONE,
        basic_attack: current_player_champion_id.data().basic_attack,
    };

    let mut enemy_idx = ::alloc::vec::Vec::with_capacity(all_players.len());

    let scoreboard = all_players
        .iter()
        .enumerate()
        .map(|(i, player)| {
            let e_champion_id = player.champion_id();
            let team = player.team.parse().unwrap_or_default();

            if team != current_player_team {
                enemy_idx.push(i);
            }

            Scoreboard {
                riot_id: player.riot_id,
                assists: player.scores.assists as _,
                deaths: player.scores.deaths,
                kills: player.scores.kills,
                creep_score: player.scores.creep_score,
                champion_id: e_champion_id,
                position: player
                    .position
                    .parse()
                    .unwrap_or(e_champion_id.main_position()),
                team,
            }
        })
        .collect::<Box<[_]>>();

    let combo_source = ComboSource {
        champion_id: current_player_champion_id,
        data: &eval_data,
        owned_items: &owned_items,
        owned_runes: &owned_runes,
        attack_type: current_player_cache_attack_type,
        level,
    };

    let combo = combo_source.expand(combo).map_err(RealtimeError::Combo)?;
    let layout = DamageLayout::new(&eval_data, true).with_combo(combo.len());
    let table_bytes = enemy_idx.len() * Realtime::ROWS_PER_ENEMY * layout.stride() * 4;

    if !combo.is_empty() && table_bytes > MAX_TABLE_BYTES {
        return Err(RealtimeError::Combo(ComboError::TooLarge(table_bytes)));
    }

    let mut damages = DamageTable::zeroed(layout, enemy_idx.len(), Realtime::ROWS_PER_ENEMY);
    // candidate simulated purchase pair lives at [n_items..n_items + 2]
    let n_items = eval_data.items.closures.len();

    let enemies = enemy_idx
        .iter()
        .enumerate()
        .map(|(e, &i)| {
            let player = &all_players[i];
            let sb = &scoreboard[i];
            let e_champion_id = sb.champion_id;
            let e_level = player.level;
            let e_is_mega_gnar = player.is_mega_gnar(e_champion_id);

            let RiotAllPlayers {
                items: ref e_riot_items,
                riot_id,
                position: e_raw_position,
                ..
            } = *player;

            let e_position = e_raw_position
                .parse()
                .unwrap_or(e_champion_id.main_position());

            // Filter enemy items that we know about
            let e_items = e_riot_items
                .iter()
                .filter_map(|riot_item| ItemId::from_riot_id(riot_item.item_id))
                .collect::<Box<[_]>>();

            // Works the same as the current player basic stats inference, but with less fields
            // since the other ones are unnecessary
            let e_base_stats = e_champion_id
                .base_stats_on(e_level, e_is_mega_gnar, game_map)
                .simple();

            // Holds a collection of more detailed information about this enemy player
            let full_state = EnemyState {
                base_stats: e_base_stats,
                items: &e_items,
                champion_id: e_champion_id,
                level: e_level,
                earth_dragons: enemy_earth_dragons,
                game_map,
                ..Default::default()
            }
            .full_state(shred, false);

            let map_mod = current_player_champion_id.map_damage_modifier(&e_champion_id, game_map);
            let base_row = e * Realtime::ROWS_PER_ENEMY;

            {
                let mut modifiers = base_modifiers;

                modifiers.damages.global_mod *=
                    map_mod * full_state.rune_damage_modifier(&owned_runes);

                let shadowflame = full_state.shadowflame_modifier(&owned_items);

                modifiers.damages.magic_mod *= shadowflame;
                modifiers.damages.true_mod *= shadowflame;

                let modifiers = modifiers.against(&full_state, adaptive_type);
                let ctx = self_state.ctx(&full_state, game_map);
                let row = damages.fill_batch(
                    base_row,
                    ctx,
                    &eval_data,
                    modifiers,
                    current_player_champion_id.data().ability_batch,
                );

                if !combo.is_empty() {
                    combo_source.eval(
                        &combo,
                        ComboTarget {
                            ctx,
                            state: full_state,
                            modifiers,
                            shred,
                            shadowflame_items: &owned_items,
                            extra_item: None,
                            candidate: None,
                        },
                        row.combo,
                    );
                }
            }

            for (k, (state, shred, buffs, extra_item)) in simulated_candidates.iter().enumerate() {
                let target = full_state.with_shred(*shred, false);
                let ctx = state.ctx(&target, game_map);
                let mut buffs = *buffs;

                buffs.damages.global_mod *= map_mod * target.rune_damage_modifier(&owned_runes);

                // The candidate counts as owned while it is being simulated
                let shadowflame_items: &[ItemId] = match ItemId::SIML[k] {
                    ItemId::Shadowflame => &[ItemId::Shadowflame],
                    _ => &owned_items,
                };

                let shadowflame = target.shadowflame_modifier(shadowflame_items);

                buffs.damages.magic_mod *= shadowflame;
                buffs.damages.true_mod *= shadowflame;

                let modifiers = buffs.against(&target, state.adaptive_type);
                let ctx = ctx.with_multipliers(modifiers.damages.multipliers());
                let mut row = damages.fill_batch(
                    base_row + 1 + k,
                    ctx,
                    &eval_data,
                    modifiers,
                    current_player_champion_id.data().ability_batch,
                );

                // Write the candidate pair in place without realloc
                if let Some((metadata, closures)) = extra_item {
                    let modifier = modifiers.damages.modifier(metadata.damage_type);
                    let pair = &mut row.items[n_items..n_items + 2];

                    for (slot, closure) in pair.iter_mut().zip(*closures) {
                        *slot = (modifier * closure(&ctx)) as i32;
                    }

                    let [a, b] = [pair[0], pair[1]];

                    row.inc_onhit(metadata.attributes, a);
                    row.inc_onhit(metadata.attributes, b);
                }

                // Only the candidate's every-hit damage reaches the combo, since
                // a combo can't list an item that isn't owned
                if !combo.is_empty() {
                    combo_source.eval(
                        &combo,
                        ComboTarget {
                            ctx,
                            state: target,
                            modifiers,
                            shred: *shred,
                            shadowflame_items,
                            extra_item: *extra_item,
                            candidate: Some(ItemId::SIML[k]),
                        },
                        row.combo,
                    );
                }
            }

            Enemy {
                champion_id: e_champion_id,
                position: e_position,
                team: sb.team,
                riot_id,
                base_stats: e_base_stats,
                bonus_stats: full_state.bonus_stats,
                current_stats: full_state.current_stats,
                real_armor: full_state.armor_values.real as _,
                real_magic_resist: full_state.magic_values.real as _,
                level: e_level,
            }
        })
        .collect::<Box<[_]>>();

    // F3: the candidates of each enemy, by the total damage of the combo
    let combo_ranking = match combo.is_empty() {
        true => Box::default(),
        false => (0..enemies.len())
            .flat_map(|e| {
                let base_row = e * Realtime::ROWS_PER_ENEMY;
                let total = |k: usize| damages.combo(base_row + 1 + k).map_or(0, |row| row.total);

                let mut ranking = (0..ItemId::L_SIML as u16).collect::<::alloc::vec::Vec<_>>();

                // Stable, so ties keep the order of `ItemId::SIML`
                ranking.sort_by_key(|&k| core::cmp::Reverse(total(k as usize)));
                ranking
            })
            .collect(),
    };

    Ok(Realtime {
        combo_ranking,
        current_player: CurrentPlayer {
            riot_id,
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: current_player_stats,
            level,
            team: current_player_team,
            adaptive_type,
            position: current_player_position,
            champion_id: current_player_champion_id,
            game_map,
        },
        enemies,
        damages,
        combo,
        scoreboard,
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
