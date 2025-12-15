use super::{const_model::*, model::*};
use crate::{
    AbilityLevels, ChampionExceptionData, ItemExceptionData, L_MSTR, L_TWRD, MONSTER_RESISTS,
    RiotFormulas, RuneExceptionData, Stats, ability_id_mod, assign_champion_exceptions,
    assign_item_exceptions, assign_rune_exceptions, bonus_stats, eval_attacks, get_enemy_state,
    get_eval_ctx, get_item_bonus_stats,
};
use core::mem::MaybeUninit;
use tutorlolv2_gen::*;

/// Constant version of the [`eval_damage`] function for the enum [`AbilityId`].
pub const fn const_ability_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
    onhit: &mut RangeDamage,
    champion_id: ChampionId,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let CachedChampion { metadata, .. } = CHAMPION_CACHE[champion_id as usize];
        let TypeMetadata {
            kind,
            damage_type,
            attributes,
        } = metadata[i];
        let modifier = ability_id_mod(kind, damage_type, modifiers);
        let damage = (modifier * ability_const_eval(ctx, champion_id, kind)) as i32;
        onhit.inc_attr(attributes, damage);
        result[i] = damage;
        i += 1;
    }
    result
}

pub const fn const_item_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
    onhit: &mut RangeDamage,
    item_ids: [ItemId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let item_id = item_ids[i];
        let CachedItem {
            attributes,
            damage_type,
            ..
        } = ITEM_CACHE[item_id as usize];
        let modifier = modifiers.damages.modifier(*damage_type);
        let damages = item_const_eval(ctx, item_id, attack_type);
        let mut j = 0;
        while j < 2 {
            let damage = (modifier * damages[j]) as i32;
            onhit.inc_attr(*attributes, damage);
            result[i + j] = damage;
            j += 1;
        }
        i += 2;
    }
    result
}

pub const fn const_rune_id_eval_damage<const N: usize>(
    ctx: &EvalContext,
    rune_ids: [RuneId; N],
    attack_type: AttackType,
    modifiers: Modifiers,
) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        let rune_id = rune_ids[i];
        let CachedRune { damage_type, .. } = RUNE_CACHE[rune_id as usize];
        let modifier = modifiers.damages.modifier(*damage_type);
        result[i] = (modifier * rune_const_eval(ctx, rune_id, attack_type)) as i32;
        i += 1;
    }
    result
}

/// Constructs a new [`Damages`] struct that holds all the damage values against some entity
/// that could be calculated. This function will cause undefined behavior if any
/// metadata of closures vectors do not have the same length
pub const fn const_get_damages<const A: usize, const I: usize, const R: usize>(
    eval_ctx: &EvalContext,
    attack_type: AttackType,
    champion_id: ChampionId,
    item_ids: [ItemId; I],
    rune_ids: [RuneId; R],
    modifiers: Modifiers,
) -> ConstDamages<A, I, R> {
    let mut onhit = RangeDamage::default();

    let abilities = const_ability_id_eval_damage(&eval_ctx, &mut onhit, champion_id, modifiers);
    let items = const_item_id_eval_damage(&eval_ctx, &mut onhit, item_ids, attack_type, modifiers);
    let runes = const_rune_id_eval_damage(&eval_ctx, rune_ids, attack_type, modifiers);
    let attacks = eval_attacks(&eval_ctx, onhit, modifiers.damages.physical_mod);

    ConstDamages {
        abilities,
        items,
        runes,
        attacks,
    }
}

pub const fn const_calculator<
    const _ABILITIES: usize,
    const _ENEMIES: usize,
    const _RUNES: usize,
    const _RUNES_ECP: usize,
    const _ITEMS: usize,
    const _ITEM_EXC: usize,
>(
    game: ConstInputGame<_ENEMIES, _RUNES, _RUNES_ECP, _ITEMS, _ITEM_EXC>,
) -> ConstOutputGame<_ABILITIES, _ENEMIES, _ITEMS, _RUNES> {
    let ConstInputGame {
        active_player:
            ConstInputActivePlayer {
                abilities: ability_levels,
                runes: current_player_runes,
                rune_exceptions,
                data:
                    ConstInputMinData {
                        stats: champion_raw_stats_i32,
                        level,
                        items: current_player_items,
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

    let mut modifiers = Modifiers::default();
    let enemy_earth_dragons = dragons.enemy_earth_dragons;
    let champion_raw_stats = Stats::from_i32(champion_raw_stats_i32);

    let current_player_cache = CHAMPION_CACHE[current_player_champion_id as usize];

    let current_player_base_stats =
        BasicStats::base_stats(current_player_champion_id, level, is_mega_gnar);

    let mut champion_stats = match infer_stats {
        true => champion_raw_stats,
        false => get_item_bonus_stats(&current_player_items, &mut modifiers, dragons),
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
        Some(value) => value,
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

    let shred = ResistShred {
        armor_penetration_flat: champion_stats.armor_penetration_flat,
        armor_penetration_percent: 1.0 - champion_stats.armor_penetration_percent / 100.0,
        magic_penetration_flat: champion_stats.magic_penetration_flat,
        magic_penetration_percent: 1.0 - champion_stats.magic_penetration_percent / 100.0,
    };

    let self_state = SelfState {
        current_stats: champion_stats,
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        adaptative_type,
        ability_levels,
        level,
    };

    let mut enemies = MaybeUninit::<[ConstOutputEnemy<_ABILITIES, _ITEMS, _RUNES>; _]>::uninit();

    {
        let enemies_ptr = enemies.as_mut_ptr();
        let mut i = 0;
        while i < enemy_players.len() {
            let player = enemy_players[i];
            let ConstInputMinData {
                infer_stats: e_infer_stats,
                items: e_items,
                stacks: e_stacks,
                stats: e_raw_stats,
                level: e_level,
                champion_id: e_champion_id,
                is_mega_gnar: e_is_mega_gnar,
                item_exceptions: e_item_exceptions,
            } = player;

            let e_stats = SimpleStats::from_i32(e_raw_stats);
            let e_base_stats = SimpleStats::base_stats(e_champion_id, e_level, e_is_mega_gnar);
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

            let eval_ctx = get_eval_ctx(&self_state, &full_state);

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

            let damages = const_get_damages(
                &eval_ctx,
                attack_type,
                current_player_champion_id,
                current_player_items,
                current_player_runes,
                modifiers,
            );

            unsafe {
                core::ptr::addr_of_mut!((*enemies_ptr)[i]).write(ConstOutputEnemy {
                    champion_id: e_champion_id,
                    damages,
                    base_stats: SimpleStats::from_f32(e_base_stats),
                    bonus_stats: SimpleStats::from_f32(full_state.bonus_stats),
                    current_stats: SimpleStats::from_f32(full_state.current_stats),
                    real_armor: full_state.armor_values.real as i32,
                    real_magic_resist: full_state.magic_values.real as i32,
                    level: e_level,
                })
            };
            i += 1;
        }
    }

    let mut monster_damages = MaybeUninit::<[ConstMonsterDamage<_, _>; L_MSTR]>::uninit();

    {
        let monster_damages_ptr = monster_damages.as_mut_ptr();
        let mut i = 0;
        while i < L_MSTR {
            let (armor, magic_resist) = MONSTER_RESISTS[i];
            let full_state = get_enemy_state(
                EnemyState {
                    base_stats: SimpleStats::<f32> {
                        armor,
                        health: 1.0,
                        magic_resist,
                    },
                    items: &[],
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
            let damages = const_get_damages::<_, _, 0>(
                &eval_ctx,
                attack_type,
                current_player_champion_id,
                current_player_items,
                [],
                Modifiers::default(),
            );

            unsafe {
                core::ptr::addr_of_mut!((*monster_damages_ptr)[i]).write(ConstMonsterDamage {
                    attacks: damages.attacks,
                    abilities: damages.abilities,
                    items: damages.items,
                })
            };
            i += 1;
        }
    }

    let mut tower_damages = [0; L_TWRD];

    {
        let mut i = 0;
        while i < L_TWRD {
            let damage = match adaptative_type {
                AdaptativeType::Physical => {
                    (current_player_base_stats.attack_damage
                        + current_player_bonus_stats.attack_damage
                        + champion_stats.ability_power
                            * 0.6
                            * (100.0
                                / (140.0
                                    + (-25.0 + 50.0 * i as f32)
                                        * champion_stats.armor_penetration_percent
                                    - champion_stats.armor_penetration_flat)))
                        as i32
                }
                AdaptativeType::Magic => {
                    (current_player_base_stats.attack_damage
                        + current_player_bonus_stats.attack_damage
                        + champion_stats.ability_power
                            * 0.6
                            * (100.0
                                / (140.0
                                    + (-25.0 + 50.0 * i as f32)
                                        * champion_stats.magic_penetration_percent
                                    - champion_stats.magic_penetration_flat)))
                        as i32
                }
            };
            tower_damages[i] = damage;
            i += 1;
        }
    }

    ConstOutputGame::<_, _, _, _> {
        current_player: OutputCurrentPlayer {
            base_stats: BasicStats::from_f32(current_player_base_stats),
            bonus_stats: BasicStats::from_f32(current_player_bonus_stats),
            current_stats: Stats::from_f32(champion_stats),
            level,
            adaptative_type,
            champion_id: current_player_champion_id,
        },
        enemies: unsafe { enemies.assume_init() },
        abilities_meta: current_player_cache.metadata,
        items_meta: {
            let mut items_meta = MaybeUninit::<[TypeMetadata<_>; _]>::uninit();
            let items_ptr = items_meta.as_mut_ptr();
            let mut i = 0;
            while i < current_player_items.len() {
                let item = current_player_items[i];
                let CachedItem { metadata, .. } = *ITEM_CACHE[item as usize];
                unsafe {
                    core::ptr::addr_of_mut!((*items_ptr)[i]).write(metadata);
                }
                i += 1;
            }
            unsafe { items_meta.assume_init() }
        },
        runes_meta: {
            let mut runes_meta = MaybeUninit::<[TypeMetadata<_>; _]>::uninit();
            let runes_ptr = runes_meta.as_mut_ptr();
            let mut i = 0;
            while i < current_player_runes.len() {
                let rune = current_player_runes[i];
                let CachedRune { metadata, .. } = *RUNE_CACHE[rune as usize];
                unsafe {
                    core::ptr::addr_of_mut!((*runes_ptr)[i]).write(metadata);
                }
                i += 1;
            }
            unsafe { runes_meta.assume_init() }
        },
        abilities_to_merge: current_player_cache.merge_data,
        monster_damages: unsafe { monster_damages.assume_init() },
        tower_damages,
    }
}

pub const _ABILITIES: usize = ChampionId::Neeko.number_of_spells();
pub const _CALC_TEST: ConstOutputGame<_ABILITIES, 1, 0, 2> =
    const_calculator::<_ABILITIES, 1, 2, 0, 0, 0>(ConstInputGame {
        active_player: ConstInputActivePlayer {
            runes: [RuneId::Electrocute, RuneId::AbsoluteFocus],
            rune_exceptions: [],
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            data: ConstInputMinData {
                stats: Stats {
                    ability_power: 1000,
                    armor: 1000,
                    armor_penetration_flat: 1000,
                    armor_penetration_percent: 1000,
                    attack_damage: 1000,
                    attack_range: 1000,
                    attack_speed: 1000,
                    crit_chance: 1000,
                    crit_damage: 1000,
                    current_health: 1000,
                    magic_penetration_flat: 10,
                    magic_penetration_percent: 30,
                    magic_resist: 1000,
                    health: 1000,
                    mana: 1000,
                    current_mana: 1000,
                },
                items: [],
                item_exceptions: [],
                stacks: 0,
                level: 18,
                infer_stats: true,
                is_mega_gnar: false,
                champion_id: ChampionId::Neeko,
            },
        },
        enemy_players: [ConstInputMinData {
            stats: SimpleStats {
                armor: 100,
                health: 1000,
                magic_resist: 100,
            },
            items: [],
            item_exceptions: [],
            stacks: 0,
            level: 18,
            infer_stats: true,
            is_mega_gnar: false,
            champion_id: ChampionId::Aatrox,
        }],
        dragons: Dragons {
            ally_fire_dragons: 0,
            ally_earth_dragons: 0,
            ally_chemtech_dragons: 0,
            enemy_earth_dragons: 0,
        },
    });
