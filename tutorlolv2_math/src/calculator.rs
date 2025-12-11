use super::{helpers::*, model::*};
use crate::{AbilityLevels, L_CENM, L_MSTR, RiotFormulas, riot::*};
use smallvec::SmallVec;
use tutorlolv2_gen::{
    AdaptativeType, AttackType, BitSet, CHAMPION_CACHE, CachedItem, ChampionId, ITEM_CACHE, ItemId,
    NUMBER_OF_ITEMS, RuneId,
};
use tutorlolv2_types::StatName;

pub const MONSTER_RESISTS: [(f32, f32); L_MSTR] = [
    (0f32, 0f32),
    (21f32, 30f32),
    (120f32, 70f32),
    (90f32, 75f32),
    (100f32, -30f32),
    (42f32, 42f32),
    (20f32, 20f32),
];

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

pub const fn infer_champion_stats(items: &[ItemId], dragons: Dragons) -> Stats<f32> {
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
        i += 1;
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);
    stats.armor_penetration_percent = RiotFormulas::percent_value(&armor_penetration);
    stats.magic_penetration_percent = RiotFormulas::percent_value(&magic_penetration);

    stats
}

pub struct ChampionExceptionData<'a> {
    pub ability_levels: AbilityLevels,
    pub stacks: u32,
    pub current_player_stats: &'a mut Stats<f32>,
}

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

pub const fn assign_rune_exceptions(data: RuneExceptionData, exceptions: &[ValueException]) {
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

pub const fn assign_item_exceptions(data: ItemExceptionData, exceptions: &[ValueException]) {
    let ItemExceptionData {
        current_player_stats,
        current_player_bonus_stats,
    } = data;

    macro_rules! change_stat {
        ($field:ident $op:tt $stat:expr) => {{
            current_player_stats.$field $op $stat;
            current_player_bonus_stats.$field $op $stat;
        }};
    }

    let mut i = 0;
    while i < exceptions.len() {
        let item_exception = exceptions[i];
        let stacks = item_exception.stacks();

        if let Some(item_id) = item_exception.get_item_id() {
            match item_id {
                ItemId::WarmogsArmor => current_player_stats.health *= 1.12,
                ItemId::DarkSeal => current_player_stats.ability_power += (stacks << 2) as f32,
                ItemId::Dragonheart => {
                    let modifier = 1.0 + 0.04 * stacks as f32;
                    current_player_stats.ability_power *= modifier;
                    current_player_stats.attack_speed *= modifier;

                    change_stat!(attack_damage *= modifier);
                    change_stat!(health *= modifier);
                    change_stat!(armor *= modifier);
                    change_stat!(magic_resist *= modifier)
                }
                ItemId::DemonKingsCrown => {
                    let modifier = 1.0 + 0.01 * stacks as f32;
                    current_player_stats.ability_power *= modifier;
                    current_player_stats.attack_speed *= modifier;

                    change_stat!(attack_damage *= modifier);
                    change_stat!(health *= modifier);
                    change_stat!(armor *= modifier);
                    change_stat!(magic_resist *= modifier)
                }
                ItemId::RiteOfRuin => current_player_stats.crit_chance += stacks as f32 * 2.5,
                ItemId::ElixirOfIron => change_stat!(health += 300.0),
                ItemId::JuiceOfVitality => {
                    change_stat!(health += 300.0 + 0.1 * current_player_stats.health)
                }
                ItemId::JuiceOfPower => {
                    change_stat!(attack_damage += 18.0 + 0.1 * current_player_stats.attack_damage);
                    current_player_stats.ability_power +=
                        30.0 + 0.1 * current_player_stats.ability_power;
                }
                ItemId::MejaisSoulstealer => {
                    current_player_stats.ability_power += (5 * stacks) as f32
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
                ItemId::Hubris => change_stat!(attack_damage += (15 + stacks << 1) as f32),
                ItemId::OverlordsBloodmail => {
                    change_stat!(attack_damage += 0.025 * current_player_bonus_stats.health)
                }
                ItemId::Manamune | ItemId::Muramana => {
                    change_stat!(attack_damage += 0.025 * current_player_bonus_stats.mana)
                }
                ItemId::WintersApproach | ItemId::Fimbulwinter => {
                    change_stat!(health += 0.15 * current_player_bonus_stats.mana)
                }
                _ => {}
            }
        }
        i += 1;
    }
}

pub fn calculator(game: InputGame) -> Option<OutputGame> {
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
        enemy_players,
        dragons,
    } = game;

    let enemy_earth_dragons = dragons.enemy_earth_dragons;
    let mut ability_modifiers = AbilityModifiers::default();
    let champion_raw_stats: Stats<f32> = champion_raw_stats_i32.into();

    let current_player_runes = get_damaging_runes(&current_player_raw_runes);
    let current_player_cache =
        unsafe { CHAMPION_CACHE.get_unchecked(current_player_champion_id as usize) };

    let current_player_base_stats =
        BasicStats::base_stats(current_player_champion_id, level, is_mega_gnar);

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
        champion_raw_stats.ability_power,
    );

    let mut champion_stats = match infer_stats {
        true => champion_raw_stats,
        false => infer_champion_stats(&current_player_raw_items, dragons),
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

    if !rune_exceptions.is_empty() {
        assign_rune_exceptions(
            RuneExceptionData {
                current_player_stats: &mut champion_stats,
                adaptative_type,
                attack_type,
                level,
            },
            &rune_exceptions,
        );
    }

    if !item_exceptions.is_empty() {
        assign_item_exceptions(
            ItemExceptionData {
                current_player_stats: &mut champion_stats,
                current_player_bonus_stats: &mut current_player_bonus_stats,
            },
            &item_exceptions,
        );
    }

    let current_player_items = get_damaging_items(&current_player_raw_items);
    let (items_data, items_to_merge) = get_items_data(&current_player_items, attack_type);

    let eval_data = DamageEvalData {
        abilities: ConstDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        items: items_data,
        runes: get_runes_data(&current_player_runes, attack_type),
    };

    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: 1.0 - champion_stats.armor_penetration_percent / 100.0,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: 1.0 - champion_stats.magic_penetration_percent / 100.0,
    };

    let self_state = SelfState {
        current_stats: champion_stats.into(),
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        ability_levels,
        level,
    };

    let mut base_modifiers = DamageModifiers::default();

    for rune_id in current_player_raw_runes {
        match rune_id {
            RuneId::CoupDeGrace | RuneId::CutDown => {
                base_modifiers.global_mod *= COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE
            }
            RuneId::LastStand => {
                base_modifiers.global_mod *= get_last_stand(
                    1.0 - (self_state.current_stats.current_health
                        / self_state.current_stats.health.max(1.0)),
                )
            }
            RuneId::AxiomArcanist => ability_modifiers.r *= 1.12,
            _ => {}
        };
    }

    for item_id in current_player_raw_items {
        match item_id {
            ItemId::Shadowflame => {
                base_modifiers.magic_mod *= 1.2;
                base_modifiers.true_mod *= 1.2;
            }
            ItemId::Riftmaker => base_modifiers.global_mod *= 1.08,
            ItemId::SpearOfShojin => {
                ability_modifiers.q *= 1.12;
                ability_modifiers.w *= 1.12;
                ability_modifiers.e *= 1.12;
                ability_modifiers.r *= 1.12;
            }
            _ => {}
        };
    }

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
                item_exceptions: e_item_exceptions,
            } = player;

            let e_stats: SimpleStats<f32> = e_raw_stats.into();
            let e_items = get_damaging_items(&e_raw_items);
            let e_base_stats = SimpleStats::base_stats(e_champion_id, e_level, e_is_mega_gnar);
            let mut full_state = get_enemy_state(
                EnemyState {
                    base_stats: e_base_stats,
                    items: e_items,
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
                    global_mod: base_modifiers.global_mod * full_state.modifiers.global_mod,
                },
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

    let monster_damages = std::array::from_fn(|i| {
        let (armor, magic_resist) = MONSTER_RESISTS[i];
        let full_state = get_enemy_state(
            EnemyState {
                base_stats: SimpleStats::<f32> {
                    armor,
                    health: 1.0,
                    magic_resist,
                },
                items: BitSet::EMPTY,
                stacks: 0,
                champion_id: ChampionId::Aatrox,
                level: 0,
                earth_dragons: 0,
                item_exceptions: &[],
            },
            shred,
            true,
        );
        let eval_ctx = get_eval_ctx(&self_state, &full_state);
        let damages = get_damages(&eval_ctx, &eval_data, Modifiers::default());
        MonsterDamage {
            attacks: damages.attacks,
            abilities: damages.abilities,
            items: damages.items,
        }
    });

    let tower_damage_formula = |plates, pen_percent, pen_flat| {
        (current_player_base_stats.attack_damage
            + current_player_bonus_stats.attack_damage
            + champion_stats.ability_power
                * 0.6
                * (100.0 / (140.0 + (-25.0 + 50.0 * plates) * pen_percent - pen_flat)))
            as i32
    };

    let tower_damages = std::array::from_fn(|i| match adaptative_type {
        AdaptativeType::Physical => tower_damage_formula(
            i as f32,
            champion_stats.armor_penetration_percent,
            champion_stats.armor_penetration_flat,
        ),
        AdaptativeType::Magic => tower_damage_formula(
            i as f32,
            champion_stats.magic_penetration_percent,
            champion_stats.magic_penetration_flat,
        ),
    });

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
        abilities_to_merge: current_player_cache.merge_data,
        items_to_merge: items_to_merge,
        monster_damages,
        tower_damages,
    })
}
