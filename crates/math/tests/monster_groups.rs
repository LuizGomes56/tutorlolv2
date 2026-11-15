#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use math::{ChampionId, L_MSTR, Monster, calculator::calculator, model::*};
use strum::VariantArray;

fn game() -> OutputGame {
    let enemies = [InputMinData {
        champion_id: ChampionId::Garen,
        level: NonZeroU8::new(18).unwrap(),
        ..Default::default()
    }];

    calculator(InputGame {
        active_player: InputActivePlayer {
            data: InputMinData {
                champion_id: ChampionId::Ahri,
                level: NonZeroU8::new(18).unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        enemy_players: &enemies,
        ..Default::default()
    })
}

/// Every damage of `row`, without its context
fn values(game: &OutputGame, row: usize) -> Vec<i32> {
    let damages = game.damages.row(row);
    let attacks = damages.attacks;

    [
        attacks.basic_attack,
        attacks.critical_strike,
        attacks.onhit_damage.minimum_damage,
        attacks.onhit_damage.maximum_damage,
    ]
    .into_iter()
    .chain(damages.abilities.iter().copied())
    .chain(damages.items.iter().copied())
    .chain(damages.runes.iter().copied())
    .collect()
}

#[test]
fn monster_groups_hold_every_monster_once_in_order() {
    let game = game();
    let groups = game.damages.monster_groups();
    let monsters = groups
        .iter()
        .flat_map(|group| group.monsters.iter().copied())
        .collect::<Vec<_>>();

    let mut sorted = monsters.clone();
    sorted.sort();
    assert_eq!(sorted, Monster::VARIANTS);
    assert_eq!(monsters.len(), L_MSTR);

    // Each group is shown by the row of its first monster
    for group in &groups {
        assert_eq!(Monster::VARIANTS[group.row], group.monsters[0]);
    }
}

#[test]
fn monster_groups_merge_only_equal_damages() {
    let game = game();
    let groups = game.damages.monster_groups();

    for group in &groups {
        for &monster in &group.monsters {
            let row = Monster::VARIANTS
                .iter()
                .position(|&m| m == monster)
                .unwrap();
            assert_eq!(values(&game, row), values(&game, group.row), "{monster:?}");
        }
    }

    for (i, a) in groups.iter().enumerate() {
        for b in &groups[i + 1..] {
            assert_ne!(values(&game, a.row), values(&game, b.row));
        }
    }
}

#[test]
fn tower_damage_type_compares_bonus_ad_with_the_ap_ratio() {
    use math::AdaptiveType::*;

    let ratio = math::libgen::rules::turret::AP_RATIO;

    assert_eq!(
        RiotFormulas::tower_damage_type(61.0, 100.0, Magic),
        Physical
    );
    assert_eq!(
        RiotFormulas::tower_damage_type(59.0, 100.0, Physical),
        Magic
    );
    // A tie keeps the champion's own adaptive type
    assert_eq!(
        RiotFormulas::tower_damage_type(ratio * 100.0, 100.0, Magic),
        Magic
    );
    assert_eq!(
        RiotFormulas::tower_damage_type(0.0, 0.0, Physical),
        Physical
    );
}
