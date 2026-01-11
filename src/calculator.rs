//! This module has functions related to the `calculator` section,
//! which receives custom game information to evaluate damages.
//! The functions exported in this module are more optimized than
//! the ones in the [`crate::realtime`] module, and have more
//! information, which means their results are more accurate.
//!
//! Note that this module have no dependencies on Riot's API,
//! so all the function inputs have to be obtained through your
//! own mechanism

use crate::helpers::*;
use crate::model::*;
use alloc::boxed::Box;
use tutorlolv2_gen::*;

/// Constant array containing the armor and magic resistences of jungle monsters.
/// Note that there's no specific name to each monster since the damage against
/// most of them repeats since their armor and magic resistence values are the same
pub const MONSTER_RESISTS: [(f32, f32); L_MSTR] = [
    (0f32, 0f32),
    (21f32, 30f32),
    (120f32, 70f32),
    (90f32, 75f32),
    (100f32, -30f32),
    (42f32, 42f32),
    (20f32, 20f32),
];

/// Counts how many items have either armor or magic resistence
/// in its stats. Items that give those attributes conditionally
/// such as [`ItemId::BlackCleaver`] are also included.
pub const NUMBER_OF_ITEMS_WITH_PEN: usize = {
    let mut i = 0;
    let mut j = 0;
    while i < NUMBER_OF_ITEMS {
        let CachedItem {
            stats,
            prettified_stats,
            ..
        } = ITEM_CACHE[i];

        if stats.armor_penetration_percent > 0.0 || stats.magic_penetration_percent > 0.0 {
            j += 1;
        } else {
            let mut k = 0;
            while k < prettified_stats.len() {
                match prettified_stats[k] {
                    StatName::ArmorPenetration(_) | StatName::MagicPenetration(_) => j += 1,
                    _ => {}
                }
                k += 1;
            }
        }
        i += 1;
    }
    j
};

/// Array with [`ItemId`] definition of all items that have some
/// kind of armor or magic penetration, including those that give
/// these attributes conditionally such as [`ItemId::BlackCleaver`]
pub const ITEMS_WITH_PEN: [ItemId; NUMBER_OF_ITEMS_WITH_PEN] = {
    let mut i = 0;
    let mut j = 0;
    let mut items = [ItemId::AbyssalMask; NUMBER_OF_ITEMS_WITH_PEN];
    while i < NUMBER_OF_ITEMS {
        let CachedItem {
            stats,
            prettified_stats,
            metadata,
            ..
        } = ITEM_CACHE[i];

        if stats.armor_penetration_percent > 0.0 || stats.magic_penetration_percent > 0.0 {
            items[j] = metadata.kind;
            j += 1;
        } else {
            let mut k = 0;
            while k < prettified_stats.len() {
                match prettified_stats[k] {
                    StatName::ArmorPenetration(_) | StatName::MagicPenetration(_) => {
                        items[j] = metadata.kind;
                        j += 1
                    }
                    _ => {}
                }
                k += 1;
            }
        }
        i += 1;
    }
    items
};

pub const fn get_item_bonus_stats(
    items: &[ItemId],
    modifiers: &mut Modifiers,
    dragons: Dragons,
) -> Stats<f32> {
    let mut stats = Stats::<f32>::default();

    let mut armor_pen_count = 0;
    let mut magic_pen_count = 0;
    let mut armor_penetration = [0.0; NUMBER_OF_ITEMS_WITH_PEN];
    let mut magic_penetration = [0.0; NUMBER_OF_ITEMS_WITH_PEN];

    let mut i = 0;
    while i < items.len() {
        let item_id = items[i];
        let item = ITEM_CACHE[item_id as usize];
        let item_stats = &item.stats;

        let fire = get_fire_multiplier(dragons.ally_fire_dragons);
        let earth = get_earth_multiplier(dragons.ally_earth_dragons);

        stats.ability_power += fire * item_stats.ability_power;
        stats.attack_damage += fire * item_stats.attack_damage;
        stats.magic_resist += earth * item_stats.magic_resist;
        stats.attack_speed += item_stats.attack_speed;
        stats.crit_chance += item_stats.crit_chance;
        stats.crit_damage += item_stats.crit_damage;
        stats.armor += earth * item_stats.armor;
        stats.current_health = stats.health;
        stats.health += item_stats.health;
        stats.current_mana = stats.mana;
        stats.mana += item_stats.mana;

        armor_penetration[armor_pen_count] = item_stats.armor_penetration_percent;
        magic_penetration[magic_pen_count] = item_stats.magic_penetration_percent;

        armor_pen_count += 1;
        magic_pen_count += 1;

        match item_id {
            ItemId::RabadonsDeathcap => stats.ability_power *= 1.3,
            ItemId::WoogletsWitchcap => stats.ability_power *= 1.5,
            ItemId::WarmogsArmor => stats.health *= 1.12,
            ItemId::ElixirOfIron => stats.health += 300.0,
            ItemId::JuiceOfVitality => stats.health += 300.0 + 0.1 * stats.health,
            ItemId::Shadowflame => {
                let bonus = 1.2;

                modifiers.damages.magic_mod *= bonus;
                modifiers.damages.true_mod *= bonus;
            }
            ItemId::SpearOfShojin => {
                let bonus = 1.12;

                modifiers.abilities.q *= bonus;
                modifiers.abilities.w *= bonus;
                modifiers.abilities.e *= bonus;
                modifiers.abilities.r *= bonus;
            }
            ItemId::JuiceOfPower => {
                stats.attack_damage += 18.0 + 0.1 * stats.attack_damage;
                stats.ability_power += 30.0 + 0.1 * stats.ability_power;
            }
            // ItemId::OverlordsBloodmail => {
            //     stats.attack_damage += 0.025 * current_player_bonus_stats.health;
            // }
            // ItemId::Riftmaker => {
            //     modifiers.damages.global_mod *= 1.08;
            //     stats.ability_power += 0.02 * (current_player_bonus_stats.health + stats.health);
            // }
            // ItemId::ArchangelsStaff => {
            //     stats.ability_power += 0.01 * current_player_bonus_stats.mana
            // }
            // ItemId::SeraphsEmbrace => stats.ability_power += 0.02 * current_player_bonus_stats.mana,
            // ItemId::DemonicEmbrace => {
            //     stats.ability_power += 0.02 * (current_player_bonus_stats.health + stats.health)
            // }
            // ItemId::Manamune | ItemId::Muramana => {
            //     stats.attack_damage += 0.025 * current_player_bonus_stats.mana;
            // }
            // ItemId::WintersApproach | ItemId::Fimbulwinter => {
            //     stats.health += 0.15 * current_player_bonus_stats.mana;
            // }
            _ => {}
        }

        i += 1;
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);
    stats.armor_penetration_percent = RiotFormulas::percent_value(&armor_penetration);
    stats.magic_penetration_percent = RiotFormulas::percent_value(&magic_penetration);

    // match rune_id {
    //     RuneId::Waterwalking => {
    //         current_player_stats.ability_power += (12 + level) as f32;
    //         current_player_stats.attack_damage += 7.2 + 0.6 * level as f32
    //     }
    //     RuneId::AbsoluteFocus => match adaptative_type {
    //         AdaptativeType::Physical => {
    //             current_player_stats.attack_damage +=
    //                 1.8 + 16.2 / 17.0 * (level - 1) as f32;
    //         }
    //         AdaptativeType::Magic => {
    //             current_player_stats.ability_power +=
    //                 3.0 + 27.0 / 17.0 * (level - 1) as f32;
    //         }
    //     },
    //     RuneId::CoupDeGrace | RuneId::CutDown => {
    //         modifiers.damages.global_mod *= COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE
    //     }
    //     RuneId::LastStand => {
    //         modifiers.damages.global_mod *= get_last_stand(
    //             1.0 - (current_player_stats.current_health
    //                 / current_player_stats.health.max(1.0)),
    //         )
    //     }
    //     RuneId::AxiomArcanist => modifiers.abilities.r *= 1.12,
    // }

    stats
}

pub struct ChampionExceptionData<'a> {
    pub ability_levels: AbilityLevels,
    pub stacks: u32,
    pub current_player_stats: &'a mut Stats<f32>,
}

/// Receives basic information about the current player ability levels and its identifier,
/// and modifies the current stats based on the number of stacks. Note that depending on the
/// value of the enum [`ChampionId`], this function might do nothing
pub const fn assign_champion_exceptions(data: ChampionExceptionData, champion_id: ChampionId) {
    let ChampionExceptionData {
        ability_levels,
        stacks,
        current_player_stats,
    } = data;

    match champion_id {
        ChampionId::Veigar => current_player_stats.ability_power += stacks as f32,
        ChampionId::Swain => current_player_stats.health += (12 * stacks) as f32,
        ChampionId::Chogath => {
            current_player_stats.health +=
                (stacks * 80 + 40 * const_clamp(ability_levels.r, 0..=3) as u32) as f32
        }
        ChampionId::Sion => current_player_stats.health += stacks as f32,
        ChampionId::Darius => {
            current_player_stats.armor_penetration_percent = RiotFormulas::percent_value(&[
                current_player_stats.armor_penetration_percent,
                (15 + 5 * ability_levels.e) as f32,
            ])
        }
        ChampionId::Pantheon => {
            current_player_stats.armor_penetration_percent = RiotFormulas::percent_value(&[
                current_player_stats.armor_penetration_percent,
                (10 * ability_levels.r) as f32,
            ])
        }
        ChampionId::Nilah => {
            current_player_stats.armor_penetration_percent = RiotFormulas::percent_value(&[
                current_player_stats.armor_penetration_percent,
                current_player_stats.crit_chance / 3.0,
            ])
        }
        ChampionId::Mordekaiser => {
            current_player_stats.magic_penetration_percent = RiotFormulas::percent_value(&[
                current_player_stats.magic_penetration_percent,
                2.5 + 2.5 * ability_levels.e as f32,
            ])
        }
        _ => {}
    };
}

pub struct RuneExceptionData<'a> {
    pub current_player_stats: &'a mut Stats<f32>,
    pub attack_type: AttackType,
    pub adaptative_type: AdaptativeType,
    pub level: u8,
}

/// Receives a struct containing mutable references to the player's stats and modifiers,
/// applying them based on the received `exceptions` slice
pub const fn assign_rune_exceptions(data: RuneExceptionData, exceptions: &[ValueException]) {
    if exceptions.is_empty() {
        return;
    }

    let RuneExceptionData {
        current_player_stats,
        attack_type,
        adaptative_type,
        level,
    } = data;

    let mut i = 0;
    while i < exceptions.len() {
        let rune_exception = exceptions[i];
        let stacks = rune_exception.stacks();
        if let Some(rune_id) = rune_exception.get_rune_id() {
            match rune_id {
                RuneId::LethalTempo => match attack_type {
                    AttackType::Melee => {
                        current_player_stats.attack_speed +=
                            (stacks as f32) * (5.0 + 11.0 / 17.0 * (level - 1) as f32);
                    }
                    AttackType::Ranged => {
                        current_player_stats.attack_speed +=
                            (stacks as f32) * (3.6 + 4.4 / 17.0 * (level - 1) as f32);
                    }
                },
                RuneId::Conqueror => {
                    let formula: f32 = (stacks as f32) * (1.8 + 2.2 / 17.0 * (level - 1) as f32);
                    match adaptative_type {
                        AdaptativeType::Physical => {
                            current_player_stats.attack_damage += (0.6 * formula) as f32;
                        }
                        AdaptativeType::Magic => {
                            current_player_stats.ability_power += formula as f32;
                        }
                    }
                }
                RuneId::EyeballCollection | RuneId::GhostPoro | RuneId::ZombieWard => {
                    match adaptative_type {
                        AdaptativeType::Physical => {
                            current_player_stats.attack_damage += match stacks {
                                ..10 => 1.2 * (stacks as f32),
                                10.. => 18.0,
                            };
                        }
                        AdaptativeType::Magic => {
                            current_player_stats.ability_power += match stacks {
                                ..10 => (stacks << 1) as f32,
                                10.. => 30.0,
                            };
                        }
                    }
                }
                RuneId::GatheringStorm => {
                    let formula = ((stacks * (stacks + 1)) << 2) as f32;
                    match adaptative_type {
                        AdaptativeType::Physical => {
                            current_player_stats.attack_damage += 0.6 * formula;
                        }
                        AdaptativeType::Magic => {
                            current_player_stats.ability_power += formula;
                        }
                    }
                }
                RuneId::AdaptiveForce => match adaptative_type {
                    AdaptativeType::Physical => {
                        current_player_stats.attack_damage += 5.4 * (stacks as f32);
                    }
                    AdaptativeType::Magic => {
                        current_player_stats.ability_power += 9.0 * stacks as f32;
                    }
                },
                RuneId::Health => current_player_stats.health += 65.0 * (stacks as f32),
                RuneId::HealthScaling => {
                    current_player_stats.health += 10.0 * level as f32 * (stacks as f32)
                }
                RuneId::AttackSpeed => current_player_stats.attack_speed += 10.0 * (stacks as f32),
                _ => {}
            }
        }
        i += 1;
    }
}

pub struct ItemExceptionData<'a> {
    pub current_player_stats: &'a mut Stats<f32>,
    pub current_player_bonus_stats: &'a mut BasicStats<f32>,
}

/// Receives mutable references to the champion's current stats, bonus stats and modifiers,
/// modifying their values based on the `exceptions` slice. Only the items that
/// depend on some stack count should be added to the match arms of this function
pub const fn assign_item_exceptions(data: ItemExceptionData, exceptions: &[ValueException]) {
    if exceptions.is_empty() {
        return;
    }

    let ItemExceptionData {
        current_player_stats,
        current_player_bonus_stats,
    } = data;

    let mut i = 0;
    while i < exceptions.len() {
        let item_exception = exceptions[i];
        let stacks = item_exception.stacks();

        if let Some(item_id) = item_exception.get_item_id() {
            match item_id {
                ItemId::DarkSeal => current_player_stats.ability_power += (stacks << 2) as f32,
                ItemId::Dragonheart => {
                    let modifier = 1.0 + 0.04 * stacks as f32;
                    current_player_stats.ability_power *= modifier;
                    current_player_stats.attack_speed *= modifier;

                    current_player_stats.attack_damage *= modifier;
                    current_player_bonus_stats.attack_damage *= modifier;
                    current_player_stats.health *= modifier;
                    current_player_bonus_stats.health *= modifier;
                    current_player_stats.armor *= modifier;
                    current_player_bonus_stats.armor *= modifier;
                    current_player_stats.magic_resist *= modifier;
                    current_player_bonus_stats.magic_resist *= modifier;
                }
                ItemId::DemonKingsCrown => {
                    let modifier = 1.0 + 0.01 * stacks as f32;
                    current_player_stats.ability_power *= modifier;
                    current_player_stats.attack_speed *= modifier;

                    current_player_stats.attack_damage *= modifier;
                    current_player_bonus_stats.attack_damage *= modifier;
                    current_player_stats.health *= modifier;
                    current_player_bonus_stats.health *= modifier;
                    current_player_stats.armor *= modifier;
                    current_player_bonus_stats.armor *= modifier;
                    current_player_stats.magic_resist *= modifier;
                    current_player_bonus_stats.magic_resist *= modifier;
                }
                ItemId::RiteOfRuin => current_player_stats.crit_chance += stacks as f32 * 2.5,
                ItemId::MejaisSoulstealer => {
                    current_player_stats.ability_power += (5 * stacks) as f32
                }
                ItemId::BlackCleaver => {
                    current_player_stats.armor_penetration_percent = RiotFormulas::percent_value(&[
                        current_player_stats.armor_penetration_percent,
                        (6 * stacks) as f32,
                    ])
                }
                ItemId::BloodlettersCurse => {
                    current_player_stats.magic_penetration_percent = RiotFormulas::percent_value(&[
                        current_player_stats.magic_penetration_percent,
                        (7.5 * stacks as f32) as f32,
                    ])
                }
                ItemId::Hubris => {
                    let bonus = (15 + stacks << 1) as f32;

                    current_player_stats.attack_damage += bonus;
                    current_player_bonus_stats.attack_damage += bonus;
                }
                _ => {}
            }
        }
        i += 1;
    }
}

/// Receives data about some custom game, containing the minimum information about the
/// current player and the enemy players, returning a new struct containing the calculated
/// damages against several entities. This function is generally safe to use, but it assumes
/// that the received struct [`InputGame`] is valid. There's no undefined behavior checks.
pub fn calculator(game: InputGame) -> OutputGame {
    let InputGame {
        active_player:
            InputActivePlayer {
                abilities: ability_levels,
                runes: current_player_raw_runes,
                rune_exceptions,
                data:
                    InputMinData {
                        stats: champion_raw_stats_i32,
                        level,
                        items: current_player_raw_items,
                        infer_stats,
                        champion_id: current_player_champion_id,
                        is_mega_gnar,
                        stacks,
                        item_exceptions,
                    },
            },
        dragons,
        enemy_players,
    } = game;

    let champion_raw_stats = Stats::from_i32(&champion_raw_stats_i32);
    let mut modifiers = Modifiers::default();

    let current_player_cache = CHAMPION_CACHE[current_player_champion_id as usize];

    let current_player_base_stats =
        base_stats_bf32(current_player_champion_id, level, is_mega_gnar);

    let mut champion_stats = match infer_stats {
        true => get_item_bonus_stats(&current_player_raw_items, &mut modifiers, dragons),
        false => champion_raw_stats,
    };

    let mut current_player_bonus_stats = bonus_stats!(
        BasicStats::<f32>(champion_stats, current_player_base_stats) {
            armor,
            health,
            attack_damage,
            magic_resist,
            mana
        }
    );

    let adaptative_type = match RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
    ) {
        Some(data) => data,
        None => current_player_cache.adaptative_type,
    };

    let attack_type = current_player_cache.attack_type;

    assign_champion_exceptions(
        ChampionExceptionData {
            ability_levels,
            stacks,
            current_player_stats: &mut champion_stats,
        },
        current_player_champion_id,
    );

    assign_rune_exceptions(
        RuneExceptionData {
            current_player_stats: &mut champion_stats,
            adaptative_type,
            attack_type,
            level,
        },
        &rune_exceptions,
    );

    assign_item_exceptions(
        ItemExceptionData {
            current_player_stats: &mut champion_stats,
            current_player_bonus_stats: &mut current_player_bonus_stats,
        },
        &item_exceptions,
    );

    let shred = ResistShred::new(&champion_stats);
    let tower_damages = get_tower_damages(
        adaptative_type,
        current_player_base_stats.attack_damage,
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
        shred,
    );

    let current_player_runes = get_damaging_runes(&current_player_raw_runes);
    let current_player_items = get_damaging_items(&current_player_raw_items);

    let self_state = SelfState {
        current_stats: champion_stats,
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        adaptative_type,
        ability_levels,
        level,
    };

    // Everything up to this point was const-evaluable. The functions below require
    // heap allocation, therefore they're not constant qualified

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        items: get_items_data(&current_player_items, attack_type),
        runes: get_runes_data(&current_player_runes, attack_type),
    };

    let monster_damages = get_monster_damages(&self_state, &eval_data, shred);
    let enemies = get_calculator_enemies(
        enemy_players,
        &self_state,
        &eval_data,
        modifiers,
        shred,
        game.dragons.enemy_earth_dragons,
    );

    OutputGame {
        current_player: OutputCurrentPlayer {
            base_stats: BasicStats::from_f32(&current_player_base_stats),
            bonus_stats: BasicStats::from_f32(&current_player_bonus_stats),
            current_stats: Stats::from_f32(&champion_stats),
            champion_id: current_player_champion_id,
            adaptative_type,
            level,
        },
        abilities_to_merge: current_player_cache.merge_data,
        abilities_meta: eval_data.abilities.metadata,
        items_meta: eval_data.items.metadata,
        runes_meta: eval_data.runes.metadata,
        monster_damages,
        tower_damages,
        enemies,
    }
}

pub fn get_calculator_enemies(
    enemy_players: Box<[InputMinData<SimpleStats<i32>>]>,
    self_state: &SelfState,
    eval_data: &DamageEvalData,
    modifiers: Modifiers,
    shred: ResistShred,
    enemy_earth_dragons: u16,
) -> Box<[OutputEnemy]> {
    enemy_players
        .into_iter()
        .map(|player| {
            let InputMinData {
                infer_stats: e_infer_stats,
                items: e_items,
                stacks: e_stacks,
                stats: e_raw_stats_i32,
                level: e_level,
                champion_id: e_champion_id,
                is_mega_gnar: e_is_mega_gnar,
                item_exceptions: e_item_exceptions,
            } = player;

            let e_stats = SimpleStats::from_i32(&e_raw_stats_i32);
            let e_base_stats = base_stats_sf32(e_champion_id, e_level, e_is_mega_gnar);
            let mut full_state = get_enemy_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: &e_items,
                    stacks: e_stacks,
                    champion_id: e_champion_id,
                    level: e_level,
                    item_exceptions: &e_item_exceptions,
                    earth_dragons: enemy_earth_dragons,
                },
                shred,
                false,
            );

            if e_infer_stats {
                full_state.current_stats = e_stats;
            }

            let eval_ctx = get_eval_ctx(self_state, &full_state);

            let modifiers = Modifiers {
                damages: DamageModifiers {
                    physical_mod: modifiers.damages.physical_mod
                        * full_state.armor_values.modifier
                        * full_state.modifiers.physical_mod,
                    magic_mod: modifiers.damages.magic_mod
                        * full_state.magic_values.modifier
                        * full_state.modifiers.magic_mod,
                    true_mod: modifiers.damages.true_mod * full_state.modifiers.true_mod,
                    global_mod: modifiers.damages.global_mod * full_state.modifiers.global_mod,
                },
                ..modifiers
            };

            // Everything const up to this point

            let damages = get_damages(&eval_ctx, eval_data, modifiers);

            OutputEnemy {
                current_stats: SimpleStats::from_f32(&full_state.current_stats),
                bonus_stats: SimpleStats::from_f32(&full_state.bonus_stats),
                base_stats: SimpleStats::from_f32(&e_base_stats),
                real_magic_resist: full_state.magic_values.real as _,
                real_armor: full_state.armor_values.real as _,
                champion_id: e_champion_id,
                level: e_level,
                damages,
                eval_ctx,
            }
        })
        .collect::<Box<[OutputEnemy]>>()
}
