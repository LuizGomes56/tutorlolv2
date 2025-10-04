use super::{helpers::*, model::*};
use crate::__v2::{L_CENM, L_MSTR, L_TWRD, RiotFormulas, riot::*};
use smallvec::SmallVec;
use std::mem::MaybeUninit;
use tinyset::SetU32;
use tutorlolv2_gen::{AdaptativeType, ChampionId, INTERNAL_CHAMPIONS, RuneId};

const MONSTER_RESISTS: [(i8, i8); L_MSTR] = [
    (0, 0),
    (21, 30),
    (120, 70),
    (90, 75),
    (100, -30),
    (42, 42),
    (20, 20),
];

pub fn calculator(game: InputGame) -> Option<OutputGame> {
    let InputGame {
        active_player:
            InputActivePlayer {
                abilities: ability_levels,
                runes: current_player_raw_runes,
                data:
                    InputMinData::<StatsI32> {
                        stats: champion_stats_i32,
                        level,
                        stacks: current_player_stacks,
                        items: current_player_items,
                        infer_stats,
                        attack_form,
                        champion_id: current_player_champion_id,
                    },
            },
        enemy_players,
        enemy_earth_dragons,
        stack_exceptions,
        ally_dragons,
    } = game;

    let current_player_runes = runes_slice_to_set_u32(&current_player_raw_runes);
    let champion_stats: RiotChampionStats = champion_stats_i32.into();
    let current_player_cache =
        unsafe { INTERNAL_CHAMPIONS.get_unchecked(current_player_champion_id as usize) };

    let current_player_base_stats = base_stats!(
        BasicStatsF32(&current_player_cache.stats, level) {
            armor,
            health,
            attack_damage,
            magic_resist,
            mana
        }
    );

    let current_player_bonus_stats = bonus_stats!(
        BasicStatsF32(champion_stats, current_player_base_stats) {
            armor,
            health,
            attack_damage,
            magic_resist,
            mana
        }
    );

    let adaptative_type = RiotFormulas::adaptative_type(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power as f32,
    );

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
            let InputMinData::<SimpleStatsI32> {
                infer_stats: e_infer_stats,
                items: e_raw_items,
                stacks: e_stacks,
                stats: e_stats,
                level: e_level,
                attack_form: e_attack_form,
                champion_id: e_champion_id,
            } = player;

            let e_cache = unsafe { INTERNAL_CHAMPIONS.get_unchecked(e_champion_id as usize) };
            let e_items = items_slice_to_set_u32(&e_raw_items);
            let e_base_stats = base_stats!(
                SimpleStatsF32(&e_cache.stats, e_level) {
                    health,
                    armor,
                    magic_resist
                }
            );
            let full_state = get_enemy_state(
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

            let eval_ctx = get_eval_ctx(&self_state, &full_state);
            let modifiers = {
                let mut global_mod = full_state.modifiers.global_mod;

                if current_player_raw_runes.contains(&RuneId::LastStand) {
                    global_mod *= LAST_STAND_CLOSURE(eval_ctx.missing_health);
                }
                if current_player_raw_runes.contains(&RuneId::CoupdeGrace)
                    || current_player_raw_runes.contains(&RuneId::CutDown)
                {
                    global_mod *= COUP_DE_GRACE_AND_CUTDOWN_BONUS_DMG;
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
                base_stats: SimpleStatsF32 {
                    armor: armor as f32,
                    health: 1.0,
                    magic_resist: magic_resist as f32,
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
            current_stats: (&champion_stats).into(),
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
