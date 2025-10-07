use super::{helpers::*, model::*};
use crate::__v2::{AbilityLevels, L_CENM, L_MSTR, L_STCK, L_TWRD, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::{
    AdaptativeType, AttackType, ChampionId, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, ItemId, RuneId,
};

const MONSTER_RESISTS: [(f32, f32); L_MSTR] = [
    (0f32, 0f32),
    (21f32, 30f32),
    (120f32, 70f32),
    (90f32, 75f32),
    (100f32, -30f32),
    (42f32, 42f32),
    (20f32, 20f32),
];

fn infer_champion_stats(items: &[ItemId], dragons: Dragons) -> Stats<f32> {
    let mut stats = Stats::<f32>::default();

    let mut armor_penetration = SmallVec::<[f32; 5]>::new();
    let mut magic_penetration = SmallVec::<[f32; 5]>::new();

    for item_id in items {
        let item = unsafe { INTERNAL_ITEMS.get_unchecked(*item_id as usize) };
        let item_stats = &item.stats;

        macro_rules! add_stat {
            ($field:ident) => {
                stats.$field += item_stats.$field;
            };
            (@$mul:ident $field:ident) => {
                stats.$field += $mul * item_stats.$field;
            };
        }

        let fire = GET_FIRE_MULTIPLIER(dragons.fire);
        let earth = GET_EARTH_MULTIPLIER(dragons.earth);

        add_stat!(@fire ability_power);
        add_stat!(@fire attack_damage);
        add_stat!(@earth armor);
        add_stat!(@earth magic_resist);
        add_stat!(health);
        add_stat!(crit_chance);
        add_stat!(crit_damage);
        add_stat!(mana);
        add_stat!(attack_speed);

        stats.current_health = stats.health;
        stats.current_mana = stats.mana;

        armor_penetration.push(item_stats.armor_penetration_percent);
        magic_penetration.push(item_stats.magic_penetration_percent);
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);
    stats.armor_penetration_percent = RiotFormulas::percent_value(&armor_penetration);
    stats.magic_penetration_percent = RiotFormulas::percent_value(&magic_penetration);

    stats
}

pub struct AssignExceptionData<'a> {
    pub ability_levels: AbilityLevels,
    pub current_player_stats: &'a mut Stats<f32>,
    pub current_player_bonus_stats: &'a mut BasicStats<f32>,
    pub enemy_players: &'a mut [InputMinData<SimpleStats<i32>>],
    pub attack_type: AttackType,
    pub adaptative_type: AdaptativeType,
    pub level: u8,
}

fn assign_exceptions(data: AssignExceptionData, exceptions: SmallVec<[StackException; L_STCK]>) {
    let AssignExceptionData {
        ability_levels,
        current_player_stats,
        current_player_bonus_stats,
        enemy_players,
        attack_type,
        adaptative_type,
        level,
    } = data;

    macro_rules! add_self {
        ($field:ident, $stat:expr) => {{
            current_player_stats.$field += $stat;
            current_player_bonus_stats.$field += $stat;
        }};
    }

    for stack_exception in exceptions.into_iter() {
        match stack_exception {
            StackException::Champion(exception) => {
                let stacks = exception.stacks;

                match exception.kind {
                    ChampionId::Veigar => current_player_stats.ability_power += stacks as f32,
                    ChampionId::Swain => current_player_stats.health += (12 * stacks) as f32,
                    ChampionId::Chogath => {
                        current_player_stats.health +=
                            (stacks * 80 + 40 * ability_levels.r.clamp(0, 3) as u16) as f32
                    }
                    ChampionId::Sion => current_player_stats.health += stacks as f32,
                    ChampionId::Darius => {
                        current_player_stats.armor_penetration_percent =
                            RiotFormulas::percent_value(&[
                                current_player_stats.armor_penetration_percent,
                                (15 + 5 * ability_levels.e) as f32,
                            ])
                    }
                    ChampionId::Pantheon => {
                        current_player_stats.armor_penetration_percent =
                            RiotFormulas::percent_value(&[
                                current_player_stats.armor_penetration_percent,
                                (10 * ability_levels.r) as f32,
                            ])
                    }
                    ChampionId::Nilah => {
                        current_player_stats.armor_penetration_percent =
                            RiotFormulas::percent_value(&[
                                current_player_stats.armor_penetration_percent,
                                current_player_stats.crit_chance / 3.0,
                            ])
                    }
                    ChampionId::Mordekaiser => {
                        current_player_stats.magic_penetration_percent =
                            RiotFormulas::percent_value(&[
                                current_player_stats.magic_penetration_percent,
                                2.5 + 2.5 * ability_levels.e as f32,
                            ])
                    }
                    _ => {}
                }
            }
            StackException::Item(exception) => {
                let stacks = exception.stacks;
                let e_stats = enemy_players.get_mut(exception.offset as usize);

                macro_rules! exc_add {
                    ($field:ident, $stat:expr) => {
                        if let Some(e_st) = e_stats {
                            e_st.stats.$field += $stat as i32;
                        } else {
                            current_player_stats.$field += $stat;
                            current_player_bonus_stats.$field += $stat;
                        }
                    };
                }

                match exception.kind {
                    ItemId::DarkSeal => {
                        current_player_stats.ability_power += (stacks.max(1) << 2) as f32
                    }
                    ItemId::MejaisSoulstealer => {
                        current_player_stats.ability_power += (5 * stacks.max(1)) as f32
                    }
                    ItemId::RabadonsDeathcap => current_player_stats.ability_power *= 1.3,
                    ItemId::WoogletsWitchcap => current_player_stats.ability_power *= 1.5,
                    ItemId::ArchangelsStaff => {
                        current_player_stats.ability_power += 0.01 * current_player_bonus_stats.mana
                    }
                    ItemId::SeraphsEmbrace => {
                        current_player_stats.ability_power += 0.02 * current_player_bonus_stats.mana
                    }
                    ItemId::Riftmaker | ItemId::DemonicEmbrace => {
                        current_player_stats.ability_power +=
                            0.02 * (current_player_bonus_stats.health + current_player_stats.health)
                    }
                    ItemId::BlackCleaver => {
                        current_player_stats.armor_penetration_percent =
                            RiotFormulas::percent_value(&[
                                current_player_stats.armor_penetration_percent,
                                (6 * stacks) as f32,
                            ])
                    }
                    ItemId::Hubris => add_self!(attack_damage, (15 + stacks.max(1) << 1) as f32),
                    ItemId::OverlordsBloodmail => add_self!(
                        attack_damage,
                        0.025 * (current_player_bonus_stats.health + 500.0)
                    ),
                    ItemId::Manamune | ItemId::Muramana => {
                        add_self!(attack_damage, 0.025 * current_player_bonus_stats.mana)
                    }
                    ItemId::WintersApproach => {
                        exc_add!(health, 0.15 * (current_player_bonus_stats.health + 500.0))
                    }
                    ItemId::Fimbulwinter => {
                        exc_add!(health, 0.15 * (current_player_bonus_stats.health + 860.0))
                    }
                    _ => {}
                }
            }
            StackException::Rune(exception) => {
                let stacks = exception.stacks;
                match exception.kind {
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
                        let formula: f32 =
                            (stacks as f32) * (1.8 + 2.2 / 17.0 * (level - 1) as f32);
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
                                    0..10 => 1.2 * (stacks as f32),
                                    _ => 18.0,
                                };
                            }
                            AdaptativeType::Magic => {
                                current_player_stats.ability_power += match stacks {
                                    0..10 => (stacks << 1) as f32,
                                    _ => 30.0,
                                };
                            }
                        }
                    }
                    RuneId::Waterwalking => {
                        current_player_stats.ability_power += (12 + level) as f32;
                        current_player_stats.attack_damage += 7.2 + 0.6 * level as f32
                    }
                    RuneId::AbsoluteFocus => match adaptative_type {
                        AdaptativeType::Physical => {
                            current_player_stats.attack_damage +=
                                1.8 + 16.2 / 17.0 * (level - 1) as f32;
                        }
                        AdaptativeType::Magic => {
                            current_player_stats.ability_power +=
                                3.0 + 27.0 / 17.0 * (level - 1) as f32;
                        }
                    },
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
                    RuneId::AttackSpeed => {
                        current_player_stats.attack_speed += 10.0 * (stacks as f32)
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn calculator(game: InputGame) -> Option<OutputGame> {
    let InputGame {
        active_player:
            InputActivePlayer {
                abilities: ability_levels,
                runes: current_player_raw_runes,
                data:
                    InputMinData {
                        stats: champion_raw_stats_i32,
                        level,
                        items: current_player_items,
                        infer_stats,
                        champion_id: current_player_champion_id,
                        is_mega_gnar,
                        ..
                    },
            },
        mut enemy_players,
        enemy_earth_dragons,
        stack_exceptions,
        ally_dragons,
    } = game;

    let champion_raw_stats: Stats<f32> = champion_raw_stats_i32.into();

    let current_player_runes = runes_slice_to_set_u32(&current_player_raw_runes);
    let current_player_cache =
        unsafe { INTERNAL_CHAMPIONS.get_unchecked(current_player_champion_id as usize) };

    let current_player_base_stats =
        base_stats_bf32(&current_player_cache.stats, level, is_mega_gnar);

    let mut current_player_bonus_stats = bonus_stats!(
        BasicStats::<f32>(champion_raw_stats, current_player_base_stats) {
            armor,
            health,
            attack_damage,
            magic_resist,
            mana
        }
    );

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        champion_raw_stats.ability_power as f32,
    );

    let mut champion_stats = match infer_stats {
        true => champion_raw_stats,
        false => infer_champion_stats(&current_player_items, ally_dragons),
    };

    if !stack_exceptions.is_empty() {
        assign_exceptions(
            AssignExceptionData {
                ability_levels,
                current_player_stats: &mut champion_stats,
                current_player_bonus_stats: &mut current_player_bonus_stats,
                enemy_players: &mut enemy_players,
                attack_type: current_player_cache.attack_type,
                adaptative_type,
                level,
            },
            stack_exceptions,
        );
    }

    let current_player_items = items_slice_to_set_u32(&current_player_items);

    let eval_data = DamageEvalData {
        abilities: get_abilities_data(current_player_cache.abilities, ability_levels, level),
        items: get_items_data(
            &current_player_items,
            current_player_cache.attack_type,
            level,
        ),
        runes: get_runes_data(&current_player_runes, level),
    };

    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: champion_stats.armor_penetration_percent,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: champion_stats.magic_penetration_percent,
    };

    let self_state = SelfState {
        current_stats: champion_stats.into(),
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        level,
    };

    let enemies = enemy_players
        .into_iter()
        .filter_map(|player| {
            let InputMinData {
                infer_stats: e_infer_stats,
                items: e_raw_items,
                stacks: e_stacks,
                stats: e_raw_stats,
                level: e_level,
                champion_id: e_champion_id,
                is_mega_gnar: e_is_mega_gnar,
            } = player;

            let e_stats: SimpleStats<f32> = e_raw_stats.into();
            let e_cache = unsafe { INTERNAL_CHAMPIONS.get_unchecked(e_champion_id as usize) };
            let e_items = items_slice_to_set_u32(&e_raw_items);
            let e_base_stats = base_stats_sf32(&e_cache.stats, e_level, e_is_mega_gnar);
            let mut full_state = get_enemy_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: e_items,
                    stacks: e_stacks,
                    champion_id: e_champion_id,
                    level: e_level,
                },
                shred,
                enemy_earth_dragons,
                false,
            );

            if e_infer_stats {
                full_state.current_stats = e_stats;
            }

            let eval_ctx = get_eval_ctx(&self_state, &full_state);
            let modifiers = {
                let mut global_mod = full_state.modifiers.global_mod;

                if current_player_raw_runes.contains(&RuneId::LastStand) {
                    global_mod *= LAST_STAND_CLOSURE(eval_ctx.missing_health);
                }
                if current_player_raw_runes.contains(&RuneId::CoupdeGrace)
                    || current_player_raw_runes.contains(&RuneId::CutDown)
                {
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

            Some(OutputEnemy {
                champion_id: e_champion_id,
                damages,
                base_stats: e_base_stats.into(),
                bonus_stats: full_state.bonus_stats.into(),
                current_stats: full_state.current_stats.into(),
                real_armor: full_state.armor_values.real as i32,
                real_magic_resist: full_state.magic_values.real as i32,
                level: e_level,
            })
        })
        .collect::<SmallVec<[OutputEnemy; L_CENM]>>();

    let mut monster_results = MaybeUninit::<[MonsterDamage; L_MSTR]>::uninit();
    let monster_result_ptr = monster_results.as_mut_ptr();

    for (index, (armor, magic_resist)) in MONSTER_RESISTS.into_iter().enumerate() {
        let full_state = get_enemy_state(
            EnemyState {
                base_stats: SimpleStats::<f32> {
                    armor,
                    health: 1.0,
                    magic_resist,
                },
                items: SetU32::new(),
                stacks: 0,
                champion_id: ChampionId::Aatrox,
                level: 0,
            },
            shred,
            enemy_earth_dragons,
            true,
        );
        let eval_ctx = get_eval_ctx(&self_state, &full_state);
        let damages = get_damages(&eval_ctx, &eval_data, DamageModifiers::default());
        unsafe {
            core::ptr::addr_of_mut!((*monster_result_ptr)[index]).write(MonsterDamage {
                attacks: damages.attacks,
                abilities: damages.abilities,
                items: damages.items,
            });
        }
    }

    let mut tower_damages_results = MaybeUninit::<[i32; 6]>::uninit();
    let tower_damages_ptr = tower_damages_results.as_mut_ptr();

    for i in 0..L_TWRD {
        let formula = |pen_percent, pen_flat| {
            (current_player_base_stats.attack_damage
                + current_player_bonus_stats.attack_damage
                + champion_stats.ability_power
                    * 0.6
                    * (100.0 / (140.0 + (-25.0 + 50.0 * i as f32) * pen_percent - pen_flat)))
                as i32
        };
        unsafe {
            core::ptr::addr_of_mut!((*tower_damages_ptr)[i]).write(match adaptative_type {
                AdaptativeType::Physical => formula(
                    champion_stats.armor_penetration_percent,
                    champion_stats.armor_penetration_flat,
                ),
                AdaptativeType::Magic => formula(
                    champion_stats.magic_penetration_percent,
                    champion_stats.magic_penetration_flat,
                ),
            });
        };
    }

    Some(OutputGame {
        current_player: OutputCurrentPlayer {
            base_stats: current_player_base_stats.into(),
            bonus_stats: current_player_bonus_stats.into(),
            current_stats: champion_stats.into(),
            level,
            adaptative_type,
            champion_id: current_player_champion_id,
        },
        enemies,
        abilities_meta: eval_data.abilities.metadata,
        items_meta: eval_data.items.metadata,
        runes_meta: eval_data.runes.metadata,
        monster_damages: unsafe { monster_results.assume_init() },
        tower_damages: unsafe { tower_damages_results.assume_init() },
    })
}
