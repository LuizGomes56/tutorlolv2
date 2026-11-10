#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use tutorlolv2::{
    AttackType, ChampionId, GameMap, ItemId, calculator::InferStats, calculator::calculator,
    model::*,
};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

#[test]
fn mayhem_and_aram_have_their_own_map_codes() {
    assert_eq!(GameMap::from_code(12), GameMap::AramMayhem);
    assert_eq!(GameMap::from_code(14), GameMap::Aram);
    assert_eq!(GameMap::from_code(11), GameMap::SummonersRift);
}

#[test]
fn mayhem_stats_grow_with_level_up_to_18() {
    let mayhem = GameMap::AramMayhem;

    close(mayhem.base_health(AttackType::Melee, 1), 20.0);
    close(mayhem.base_health(AttackType::Melee, 18), 600.0);
    close(mayhem.base_health(AttackType::Ranged, 18), 400.0);
    close(mayhem.base_health(AttackType::Ranged, 255), 400.0);

    assert_eq!(mayhem.bonus_resists(AttackType::Melee, 1), (15.0, 15.0));
    assert_eq!(mayhem.bonus_resists(AttackType::Ranged, 1), (2.0, 2.0));
    assert_eq!(mayhem.bonus_resists(AttackType::Ranged, 18), (25.0, 25.0));

    assert_eq!(
        GameMap::Aram.bonus_resists(AttackType::Melee, 10),
        (0.0, 15.0)
    );
    assert_eq!(
        GameMap::Aram.bonus_resists(AttackType::Ranged, 10),
        (0.0, 0.0)
    );
    close(GameMap::Aram.base_health(AttackType::Melee, 18), 0.0);
    close(
        GameMap::SummonersRift.base_health(AttackType::Melee, 18),
        0.0,
    );
}

fn level() -> NonZeroU8 {
    NonZeroU8::new(18).unwrap()
}

fn enemy(game_map: GameMap) -> EnemyFullState {
    let champion_id = ChampionId::Garen;

    EnemyState {
        base_stats: champion_id.base_stats(level(), false).simple(),
        items: &[ItemId::ClothArmor],
        champion_id,
        level: level(),
        game_map,
        ..Default::default()
    }
    .full_state(ResistShred::default(), false)
}

#[test]
fn inferred_enemies_gain_the_stats_of_the_map() {
    let standard = enemy(GameMap::SummonersRift);
    let mayhem = enemy(GameMap::AramMayhem);
    let aram = enemy(GameMap::Aram);

    let (s, m) = (standard.current_stats, mayhem.current_stats);
    close(m.max_health, s.max_health + 600.0);
    close(m.armor, s.armor + 15.0);
    close(m.magic_resist, s.magic_resist + 15.0);
    close(aram.current_stats.magic_resist, s.magic_resist + 15.0);
    close(aram.current_stats.armor, s.armor);

    // Mayhem health is base health, while its resists are bonus resists
    close(
        mayhem.bonus_stats.max_health,
        standard.bonus_stats.max_health,
    );
    close(mayhem.bonus_stats.armor, standard.bonus_stats.armor + 15.0);
}

fn infer(game_map: GameMap, item_exceptions: &[ValueException]) -> PlayerStats {
    let mut modifiers = Modifiers::default();

    InferStats {
        item_exceptions,
        rune_exceptions: &[],
        items: &[],
        runes: &[],
        modifiers: &mut modifiers,
        dragons: Dragons::default(),
        ability_levels: AbilityLevels::default(),
        stacks: 0,
        level: level(),
        champion_id: ChampionId::Garen,
        is_mega_gnar: false,
        current_health: 0.0,
        game_map,
    }
    .infer()
}

#[test]
fn inferred_current_player_gains_the_stats_of_the_map() {
    let standard = infer(GameMap::SummonersRift, &[]);
    let mayhem = infer(GameMap::AramMayhem, &[]);

    close(mayhem.max_health, standard.max_health + 600.0);
    close(mayhem.current_health, mayhem.max_health);
    close(mayhem.armor, standard.armor + 15.0);
    close(mayhem.magic_resist, standard.magic_resist + 15.0);
}

#[test]
fn mayhem_converts_critical_strike_chance_above_100() {
    // Rite of Ruin grants 2.5% critical strike chance per stack: 150% in total
    let crit = [ValueException::pack_item_id(ItemId::RiteOfRuin, 60)];
    let aram = infer(GameMap::Aram, &crit);
    let mayhem = infer(GameMap::AramMayhem, &crit);

    close(aram.crit_chance, 100.0);
    close(mayhem.crit_chance, 100.0);
    // Garen's adaptive type is physical: 0.45 attack damage per 1% above 100%
    close(mayhem.attack_damage - aram.attack_damage, 0.45 * 50.0);
}

#[test]
fn calculator_uses_the_game_map() {
    let enemies = [InputMinData {
        champion_id: ChampionId::Garen,
        level: level(),
        ..Default::default()
    }];

    let game = |game_map| InputGame {
        active_player: InputActivePlayer {
            data: InputMinData {
                champion_id: ChampionId::Garen,
                level: level(),
                ..Default::default()
            },
            ..Default::default()
        },
        enemy_players: &enemies,
        game_map,
        ..Default::default()
    };

    let standard = calculator(game(GameMap::SummonersRift));
    let mayhem = calculator(game(GameMap::AramMayhem));

    let (s, m) = (&standard.current_player, &mayhem.current_player);
    close(m.base_stats.max_health, s.base_stats.max_health + 600.0);
    close(
        m.current_stats.max_health,
        s.current_stats.max_health + 600.0,
    );
    close(m.bonus_stats.max_health, s.bonus_stats.max_health);
}
