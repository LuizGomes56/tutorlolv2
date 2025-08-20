#![allow(unreachable_code, unused_variables)]
use super::*;
use crate::model::{base::*, calculator::*};
use internal_comptime::{
    AbilityLike, AdaptativeType, AttackType, Attrs, BASIC_ATTACK, CRITICAL_STRIKE, ChampionId,
    DAMAGING_ITEMS, DAMAGING_RUNES, EvalContext, INTERNAL_CHAMPIONS, INTERNAL_ITEMS, ItemId,
    META_ITEMS, Position, RuneId,
};
use smallvec::SmallVec;
use tinyset::SetU32;

/// If user opted not to dictate the active player's stats, this function is called
/// it reads all items present in cache and evaluates what the game condition would be
/// if all items were owned in a real game. Stacks and champion exceptions are partially
/// taken into consideration when calculation is made. It is less accurate than Realtime.
fn infer_champion_stats(
    stats: &mut Stats,
    active_player: &InputActivePlayer,
    base_stats: BasicStats,
    dragon_mod: DragonMultipliers,
) -> BasicStats {
    let stacks = active_player.stacks as f32;
    let owned_items = &active_player.items;

    let mut armor_penetration = SmallVec::<[f32; 4]>::with_capacity(4);
    let mut magic_penetration = SmallVec::<[f32; 4]>::with_capacity(4);

    for item_id in owned_items {
        if let Some(cached_item) = INTERNAL_ITEMS.get(*item_id as usize) {
            let item_stats = &cached_item.stats;

            macro_rules! add_stat {
                ($field:ident) => {
                    stats.$field += item_stats.$field;
                };
                ($field:ident, $field2:ident) => {
                    stats.$field += item_stats.$field2;
                };
                (@$mul:ident $field:ident) => {
                    stats.$field += dragon_mod.$mul * item_stats.$field;
                };
                (@$mul:ident $field:ident, $field2:ident) => {
                    stats.$field += dragon_mod.$mul * item_stats.$field2;
                };
            }

            add_stat!(@fire ability_power);
            add_stat!(@fire attack_damage);
            add_stat!(@earth armor);
            add_stat!(@earth magic_resist, magic_resistance);
            add_stat!(max_health, health);
            add_stat!(crit_chance, critical_strike_chance);
            add_stat!(crit_damage, critical_strike_damage);
            add_stat!(max_mana, mana);
            add_stat!(attack_speed);
            add_stat!(current_mana, mana);
            add_stat!(current_health, health);

            armor_penetration.push(item_stats.armor_penetration_percent);
            magic_penetration.push(item_stats.magic_penetration_percent);
        }
    }

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);

    let bonus_stats = get_bonus_stats(
        BasicStats {
            armor: stats.armor,
            health: stats.max_health,
            attack_damage: stats.attack_damage,
            magic_resist: stats.magic_resist,
            mana: stats.max_mana,
        },
        base_stats,
    );

    // #![manual_impl]
    // #![todo] File generator
    // Depend on bonus stats, that has to be evaluated later
    for item_id in owned_items {
        match item_id {
            ItemId::OverlordsBloodmail => {
                stats.attack_damage += 0.025 * (bonus_stats.health + 500.0)
            }
            ItemId::ArchangelsStaff => stats.ability_power += 0.01 * bonus_stats.mana,
            ItemId::Manamune | ItemId::Muramana => stats.attack_damage += 0.025 * bonus_stats.mana,
            ItemId::SeraphsEmbrace => stats.ability_power += 0.02 * bonus_stats.mana,
            ItemId::WintersApproach => stats.max_health += 0.15 * (bonus_stats.health + 500.0),
            ItemId::Fimbulwinter => stats.max_health += 0.15 * (bonus_stats.health + 860.0),
            ItemId::Riftmaker | ItemId::DemonicEmbrace => {
                stats.ability_power += 0.02 * (bonus_stats.health + stats.max_health)
            }
            _ => {}
        }
    }

    // #![manual_impl]
    // #![todo] Create exception file that is generated automatically
    match active_player.champion_id {
        ChampionId::Veigar => stats.ability_power += stacks,
        ChampionId::Chogath => {
            stats.max_health += stacks * 80.0 + 40.0 * active_player.abilities.r.clamp(0, 3) as f32;
        }
        ChampionId::Sion => stats.max_health += stacks,
        ChampionId::Darius => {
            armor_penetration.push(15.0 + 5.0 * active_player.abilities.e as f32);
        }
        ChampionId::Pantheon => {
            armor_penetration.push(10.0 * active_player.abilities.r as f32);
        }
        ChampionId::Nilah => {
            armor_penetration.push(stats.crit_chance / 3.0);
        }
        ChampionId::Mordekaiser => {
            magic_penetration.push(2.5 + 2.5 * active_player.abilities.e as f32);
        }
        _ => {}
    }

    stats.armor_penetration_percent = RiotFormulas::percent_value(armor_penetration);
    stats.magic_penetration_percent = RiotFormulas::percent_value(magic_penetration);

    bonus_stats
}

// #![manual_impl]
// #![unsupported]
fn rune_exceptions(
    champion_stats: &mut Stats,
    owned_runes: &[RuneId],
    level: f32,
    value_types: (AdaptativeType, AttackType),
    // exception_map: &FxHashMap<u32, u32>,
) {
    return;
    let this_stack = 0x0000;
    for rune in owned_runes {
        // let this_stack = *exception_map.get(&rune).unwrap_or(&0);
        match rune.to_u32() {
            // Lethal Tempo
            8008 => match value_types.1 {
                AttackType::Melee => {
                    champion_stats.attack_speed +=
                        (this_stack as f32) * (5.0 + 11.0 / 17.0 * (level - 1.0));
                }
                AttackType::Ranged => {
                    champion_stats.attack_speed +=
                        (this_stack as f32) * (3.6 + 4.4 / 17.0 * (level - 1.0));
                }
            },
            // Conqueror
            8010 => {
                let formula: f32 = (this_stack as f32) * (1.8 + 2.2 / 17.0 * (level - 1.0));
                match value_types.0 {
                    AdaptativeType::Physical => {
                        champion_stats.attack_damage += 0.6 * formula;
                    }
                    AdaptativeType::Magic => {
                        champion_stats.ability_power += formula;
                    }
                }
            }
            // Eyeball Collection | Ghost Poro | Zombie Ward :: Removed Runes
            8120 | 8136 | 8138 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += match this_stack {
                        0..10 => 1.2 * (this_stack as f32),
                        _ => 18.0,
                    };
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += match this_stack {
                        0..10 => (this_stack << 1) as f32,
                        _ => 30.0,
                    };
                }
            },
            // Waterwalking
            8232 => {
                champion_stats.ability_power += 12.0 + level;
                champion_stats.attack_damage += 7.2 + 0.6 * level
            }
            // Absolute Focus
            8233 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += 1.8 + 16.2 / 17.0 * (level - 1.0);
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += 3.0 + 27.0 / 17.0 * (level - 1.0);
                }
            },
            // Gathering Storm
            8236 => {
                let formula: f32 = ((this_stack * (this_stack + 1)) << 2) as f32;
                match value_types.0 {
                    AdaptativeType::Physical => {
                        champion_stats.attack_damage += 0.6 * formula;
                    }
                    AdaptativeType::Magic => {
                        champion_stats.ability_power += formula;
                    }
                }
            }
            // Adaptative damage shard
            9000 => match value_types.0 {
                AdaptativeType::Physical => {
                    champion_stats.attack_damage += 5.4 * (this_stack as f32);
                }
                AdaptativeType::Magic => {
                    champion_stats.ability_power += 9.0 * (this_stack as f32);
                }
            },
            // Max health shard
            9001 => champion_stats.max_health += 65.0 * (this_stack as f32),
            // Health per level shard
            9002 => champion_stats.max_health += 10.0 * level * (this_stack as f32),
            // Attack speed shard
            9003 => champion_stats.attack_speed += 10.0 * (this_stack as f32),
            _ => {}
        }
    }
}

// #![manual_impl]
// #![unsupported]
fn item_exceptions(
    champion_stats: &mut Stats,
    owned_items: &[ItemId],
    // exception_map: &FxHashMap<u32, u32>,
) {
    return;
    let this_stack = 0x0000;
    for item_id in owned_items {
        // let this_stack = *exception_map.get(&item_id).unwrap_or(&0);
        match item_id {
            ItemId::DarkSeal => champion_stats.ability_power += (this_stack.max(1) << 2) as f32,
            ItemId::MejaisSoulstealer => {
                champion_stats.ability_power += (5 * this_stack.max(1)) as f32
            }
            ItemId::RabadonsDeathcap => champion_stats.ability_power *= 1.3,
            ItemId::Hubris => champion_stats.attack_damage += (15 + this_stack.max(1) << 1) as f32,
            ItemId::WoogletsWitchcap => champion_stats.ability_power *= 1.5,
            _ => {}
        }
    }
}

// #![todo] Stats are not assigned correctly
// #![todo] Review exceptions
// #[generator_macros::trace_time]
pub fn calculator(game: InputGame) -> Result<OutputGame, CalculationError> {
    let InputGame {
        active_player,
        enemy_players,
        enemy_earth_dragons,
        ally_fire_dragons,
        ally_earth_dragons,
        // stack_exceptions,
    } = game;

    let InputActivePlayer {
        level: current_player_level,
        champion_id: current_player_champion_id,
        abilities: current_player_abilities,
        champion_stats: current_player_input_stats,
        runes: ref current_player_full_runes,
        items: ref current_player_full_items,
        infer_stats: current_player_infer_stats,
        ..
    } = active_player;

    let current_player_damaging_items = current_player_full_items
        .into_iter()
        .filter_map(|item_id| {
            DAMAGING_ITEMS
                .contains(&item_id.to_u32())
                .then_some(*item_id as u32)
        })
        .collect::<SetU32>();

    let current_player_damaging_runes = current_player_full_runes
        .into_iter()
        .filter_map(|rune_id| {
            DAMAGING_RUNES
                .contains(&rune_id.to_u32())
                .then_some(*rune_id as u32)
        })
        .collect::<SetU32>();

    let current_player_cache = INTERNAL_CHAMPIONS
        .get(current_player_champion_id as usize)
        .ok_or(CalculationError::ChampionCacheNotFound)?;

    let mut current_player_stats = if current_player_infer_stats {
        RiotFormulas::full_base_stats(&current_player_cache.stats, current_player_level)
    } else {
        current_player_input_stats
    };

    let current_player_base_stats = get_base_stats(current_player_cache, current_player_level);
    let current_player_basic_stats = BasicStats {
        armor: current_player_stats.armor,
        health: current_player_stats.max_health,
        attack_damage: current_player_stats.attack_damage,
        magic_resist: current_player_stats.magic_resist,
        mana: current_player_stats.max_mana,
    };

    let ally_dragon_multipliers = DragonMultipliers {
        fire: 1.0 + FIRE_DRAGON_MULTIPLIER * ally_fire_dragons as f32,
        earth: 1.0 + EARTH_DRAGON_MULTIPLIER * ally_earth_dragons as f32,
        chemtech: 1.0,
    };

    let current_player_bonus_stats = if current_player_infer_stats {
        infer_champion_stats(
            &mut current_player_stats,
            &active_player,
            current_player_base_stats,
            ally_dragon_multipliers,
        )
    } else {
        get_bonus_stats(current_player_basic_stats, current_player_base_stats)
    };

    let current_player_attack_type = current_player_cache.attack_type;
    let current_player_champion_id = active_player.champion_id;

    item_exceptions(
        &mut current_player_stats,
        current_player_full_items.as_slice(),
        // &stack_exceptions,
    );

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        current_player_stats.ability_power,
    );

    rune_exceptions(
        &mut current_player_stats,
        &current_player_full_runes.as_slice(),
        current_player_level as f32,
        (adaptative_type, current_player_attack_type),
        // &stack_exceptions,
    );

    let enemy_dragon_multipliers = DragonMultipliers {
        fire: 1.0,
        earth: 1.0 + EARTH_DRAGON_MULTIPLIER * enemy_earth_dragons as f32,
        chemtech: 1.0,
    };

    let current_player_state = (
        &current_player_stats,
        current_player_bonus_stats,
        current_player_base_stats,
        current_player_level,
    );

    let abilities_iter_expr = get_abilities_damage(
        current_player_cache,
        current_player_level,
        current_player_abilities,
    );
    let items_iter_expr = get_items_damage(
        &current_player_damaging_items,
        current_player_attack_type,
        current_player_level,
    );
    let runes_iter_expr =
        get_runes_damage(&current_player_damaging_runes, current_player_attack_type);

    let output_enemy = |player| {
        let InputEnemyPlayers {
            champion_id: enemy_champion_id,
            items: enemy_items,
            level: enemy_level,
            // #![todo]
            infer_stats: _infer_enemy_stats,
            // #![todo]
            stats: _enemy_stats,
        } = player;
        let enemy_cache = INTERNAL_CHAMPIONS
            .get(enemy_champion_id as usize)
            .ok_or(CalculationError::ChampionCacheNotFound)?;
        let enemy_base_stats = get_base_stats(enemy_cache, enemy_level);
        let full_stats = get_full_stats(
            (
                enemy_champion_id,
                enemy_level,
                enemy_dragon_multipliers.earth,
            ),
            (enemy_base_stats, &enemy_items),
            (
                current_player_stats.armor_penetration_percent,
                current_player_stats.armor_penetration_flat,
            ),
            (
                current_player_stats.magic_penetration_percent,
                current_player_stats.magic_penetration_flat,
            ),
        );

        let damage_multipliers = DamageMultipliers {
            self_mod: full_stats.2.self_mod,
            enemy_mod: full_stats.2.enemy_mod,
            damage_mod: (full_stats.2.armor_mod, full_stats.2.magic_mod),
        };

        let eval_ctx = get_eval_ctx(&current_player_state, &full_stats);
        let mut onhit_effects = DamageValue::default();

        let abilities_damage = get_damages(
            &abilities_iter_expr,
            &damage_multipliers,
            &eval_ctx,
            &mut onhit_effects,
        );
        let items_damage = get_damages(
            &items_iter_expr,
            &damage_multipliers,
            &eval_ctx,
            &mut onhit_effects,
        );
        let runes_damage = get_damages(
            &runes_iter_expr,
            &damage_multipliers,
            &eval_ctx,
            &mut onhit_effects,
        );
        let attack_damage = get_attacks(&damage_multipliers, &eval_ctx, onhit_effects);

        Ok((
            enemy_champion_id,
            OutputEnemy {
                damages: CalculatorDamages {
                    attacks: attack_damage,
                    abilities: abilities_damage,
                    items: items_damage,
                    runes: runes_damage,
                },
                level: player.level,
                base_stats: enemy_base_stats,
                current_stats: full_stats.0,
                bonus_stats: full_stats.1,
                real_armor: full_stats.2.real_armor,
                real_magic_resist: full_stats.2.real_magic,
            },
        ))
    };

    let enemies = enemy_players
        .into_iter()
        .map(output_enemy)
        .collect::<Result<SmallVec<[(_, OutputEnemy); 1]>, CalculationError>>()?;

    let make_state = |tuple: (i8, i8)| {
        get_full_stats(
            (ChampionId::Zyra, 0, 1.0),
            (
                BasicStats {
                    armor: tuple.0 as f32,
                    magic_resist: tuple.1 as f32,
                    attack_damage: 0.0,
                    health: 2000.0,
                    mana: 0.0,
                },
                &[],
            ),
            (
                current_player_stats.armor_penetration_percent,
                current_player_stats.armor_penetration_flat,
            ),
            (
                current_player_stats.magic_penetration_percent,
                current_player_stats.magic_penetration_flat,
            ),
        )
    };

    let monster_damages = MONSTER_RESISTS.iter_enumerate().map(|(index, resists)| {
        let mut onhit_damage = DamageValue::default();
        let abilities = abilities_iter_expr
            .iter()
            .map(|(ability_name, dmg_expr)| {
                let monster_state = make_state(resists);
                let eval_ctx = get_eval_ctx(&current_player_state, &monster_state);
                let damage_multipliers = DamageMultipliers {
                    self_mod: monster_state.2.self_mod,
                    enemy_mod: monster_state.2.enemy_mod,
                    damage_mod: (monster_state.2.armor_mod, monster_state.2.magic_mod),
                };
                let damage_mod = get_damage_multipliers(&damage_multipliers, dmg_expr.damage_type);
                let ability_level = match ability_name {
                    AbilityLike::P(_) => current_player_level,
                    AbilityLike::Q(_) => current_player_abilities.q,
                    AbilityLike::W(_) => current_player_abilities.w,
                    AbilityLike::E(_) => current_player_abilities.e,
                    AbilityLike::R(_) => current_player_abilities.r,
                };
                let minimum_damage =
                    damage_mod * (dmg_expr.minimum_damage)(ability_level, &eval_ctx);
                let maximum_damage =
                    damage_mod * (dmg_expr.maximum_damage)(ability_level, &eval_ctx);
                match dmg_expr.attributes {
                    Attrs::OnhitMin => {
                        onhit_damage.minimum_damage += minimum_damage;
                    }
                    Attrs::OnhitMax => {
                        onhit_damage.maximum_damage += maximum_damage;
                    }
                    Attrs::Onhit => {
                        onhit_damage.minimum_damage += minimum_damage;
                        onhit_damage.maximum_damage += maximum_damage;
                    }
                    _ => {}
                };
                InstanceDamage {
                    damage_type: dmg_expr.damage_type,
                    minimum_damage,
                    maximum_damage,
                }
            })
            .collect::<SmallVec<_>>();
        let items = items_iter_expr
            .iter()
            .map(|(item_name, dmg_expr)| {
                let monster_state = make_state(resists);
                let eval_ctx = get_eval_ctx(&current_player_state, &monster_state);
                let damage_multipliers = DamageMultipliers {
                    self_mod: monster_state.2.self_mod,
                    enemy_mod: monster_state.2.enemy_mod,
                    damage_mod: (monster_state.2.armor_mod, monster_state.2.magic_mod),
                };
                let damage_mod = get_damage_multipliers(&damage_multipliers, dmg_expr.damage_type);
                let minimum_damage =
                    damage_mod * (dmg_expr.minimum_damage)(current_player_level, &eval_ctx);
                let maximum_damage =
                    damage_mod * (dmg_expr.maximum_damage)(current_player_level, &eval_ctx);
                match dmg_expr.attributes {
                    Attrs::OnhitMin => {
                        onhit_damage.minimum_damage += minimum_damage;
                    }
                    Attrs::OnhitMax => {
                        onhit_damage.maximum_damage += maximum_damage;
                    }
                    Attrs::Onhit => {
                        onhit_damage.minimum_damage += minimum_damage;
                        onhit_damage.maximum_damage += maximum_damage;
                    }
                    _ => {}
                };
                InstanceDamage {
                    damage_type: dmg_expr.damage_type,
                    minimum_damage,
                    maximum_damage,
                }
            })
            .collect::<SmallVec<_>>();
        let attacks = {
            let eval_context = EvalContext {
                ad: current_player_stats.attack_damage,
                crit_damage: current_player_stats.crit_damage,
                physical_multiplier: RiotFormulas::real_resist(
                    current_player_stats.armor_penetration_percent,
                    current_player_stats.armor_penetration_flat,
                    resists.1 as f32,
                )
                .1,
                ..Default::default()
            };
            Attacks {
                basic_attack: DamageValue {
                    minimum_damage: (BASIC_ATTACK.minimum_damage)(0, &eval_context),
                    maximum_damage: 0.0,
                },
                critical_strike: DamageValue {
                    minimum_damage: (CRITICAL_STRIKE.minimum_damage)(0, &eval_context),
                    maximum_damage: 0.0,
                },
                onhit_damage,
            }
        };
        MonsterExpr {
            attacks,
            abilities,
            items,
        }
    });

    let mut tower_damages: [f32; 6] = [0.0; 6];
    let base_tower_damage = |i| match adaptative_type {
        AdaptativeType::Physical => {
            current_player_base_stats.attack_damage
                + current_player_bonus_stats.attack_damage
                + current_player_stats.ability_power
                    * 0.6
                    * (100.0
                        / (140.0
                            + (-25.0 + 50.0 * i as f32)
                                * current_player_stats.armor_penetration_percent
                            - current_player_stats.armor_penetration_flat))
        }
        AdaptativeType::Magic => {
            current_player_base_stats.attack_damage
                + current_player_bonus_stats.attack_damage
                + current_player_stats.ability_power
                    * 0.6
                    * (100.0
                        / (140.0
                            + (-25.0 + 50.0 * i as f32)
                                * current_player_stats.magic_penetration_percent
                            - current_player_stats.magic_penetration_flat))
        }
    };

    for i in 0..6 {
        tower_damages[i] = base_tower_damage(i);
    }

    Ok(OutputGame {
        monster_damages,
        tower_damages,
        current_player: OutputCurrentPlayer {
            adaptative_type,
            damaging_items: current_player_damaging_items.into(),
            damaging_runes: current_player_damaging_runes.into(),
            level: current_player_level,
            champion_id: current_player_champion_id,
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: current_player_stats,
        },
        enemies,
    })
}
