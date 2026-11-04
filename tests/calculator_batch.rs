#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use strum::EnumCount;
use tutorlolv2::{AttackType, ChampionId, L_MSTR, calculator::EnemyMap, model::*};

#[test]
fn calculator_batch_matches_legacy_enemy_and_monster_rows() {
    let level = NonZeroU8::new(18).unwrap();
    let enemy_players = [InputMinData {
        stats: Some(EnemyStats {
            armor: 125.0,
            magic_resist: 85.0,
            max_health: 2500.0,
            current_health: 1200.0,
            missing_health: 0.0,
        }),
        items: &[],
        item_exceptions: &[],
        stacks: 0,
        level,
        champion_id: ChampionId::Ornn,
        is_mega_gnar: false,
    }];

    for index in 0..ChampionId::COUNT {
        let champion_id = ChampionId::from_repr(index as _).unwrap();
        let champion = champion_id.data();
        let base_stats = champion_id.base_stats(level, false);
        let self_state = SelfState {
            level,
            stacks: 150.0,
            ability_levels: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            current_stats: PlayerStats {
                max_health: 3000.0,
                current_health: 1700.0,
                max_mana: 1500.0,
                attack_damage: 200.0,
                ability_power: 180.0,
                ..PlayerStats::default()
            },
            base_stats,
            bonus_stats: BasicStats::default(),
            adaptive_type: champion.adaptive_type,
        };
        let eval_data = DamageEvalData {
            abilities: StaticDamageKind {
                metadata: champion.metadata,
                closures: champion.closures,
            },
            items: DamageKind::items(&tutorlolv2::bitset::BitSet::EMPTY, AttackType::Melee),
            runes: DamageKind::runes(&tutorlolv2::bitset::BitSet::EMPTY, AttackType::Melee),
        };
        let modifiers = Modifiers::default();
        // Flat penetration exercises the monsters' negative-resistance path.
        let shred = ResistShred {
            armor_penetration_flat: 100.0,
            magic_penetration_flat: 100.0,
            ..ResistShred::default()
        };
        let layout = DamageLayout::new(&eval_data, false);
        let mut legacy = DamageTable::zeroed(layout, L_MSTR + enemy_players.len());
        let mut batched = legacy.clone();
        for (i, &(armor, magic_resist)) in RiotFormulas::MONSTER_RESISTS.iter().enumerate() {
            let full_state = EnemyState {
                base_stats: SimpleStats {
                    armor,
                    magic_resist,
                    max_health: 1000.0,
                },
                ..Default::default()
            }
            .full_state(shred, true);
            legacy.fill(
                i,
                self_state.ctx(&full_state),
                &eval_data,
                modifiers.against(&full_state, self_state.adaptive_type),
            );
        }
        batched.fill_monster_damages(
            &self_state,
            &eval_data,
            shred,
            modifiers,
            champion.ability_batch,
        );
        let legacy_enemies = legacy.calculator_enemies(
            &enemy_players,
            &self_state,
            &eval_data,
            modifiers,
            shred,
            2,
            EnemyMap::default(),
        );
        let batch_enemies = batched.calculator_enemies_with(
            &enemy_players,
            &self_state,
            &eval_data,
            modifiers,
            shred,
            2,
            EnemyMap::default(),
            |damages, r, ctx, data, modifiers| {
                damages.fill_batch(r, ctx, data, modifiers, champion.ability_batch);
            },
        );
        assert_eq!(legacy_enemies, batch_enemies, "{champion_id:?}");
        assert_eq!(legacy, batched, "{champion_id:?}");
    }
}
