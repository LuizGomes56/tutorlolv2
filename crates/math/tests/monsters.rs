use math::{L_MSTR, Monster, model::MonsterStat};
use strum::{EnumCount, VariantArray};

#[test]
fn every_monster_has_a_row() {
    assert_eq!(L_MSTR, Monster::COUNT);
    assert_eq!(Monster::VARIANTS.len(), Monster::DATA.len());
}

#[test]
fn level_is_rounded_average() {
    assert_eq!(Monster::level([]), 1);
    assert_eq!(Monster::level([1]), 1);
    assert_eq!(Monster::level([10, 11]), 11);
    assert_eq!(Monster::level([10, 10, 11]), 10);
    assert_eq!(Monster::level([18; 10]), 18);
}

#[test]
fn levels_clamp_to_the_table() {
    let baron = Monster::BaronNashor;

    // Starts at level 11, so earlier levels use its first value
    assert_eq!(baron.stats(1), baron.stats(11));
    assert_eq!(baron.stats(11).max_health, 16300.0);
    assert_eq!(baron.stats(18).max_health, 16300.0 + 7.0 * 170.0);
    assert_eq!(baron.stats(18).armor, 34.0 + 7.0 * 8.0);

    // Rift Scuttler's table ends at level 17
    assert_eq!(Monster::RiftScuttler.stats(18).max_health, 5270.0);
}

#[test]
fn camps_scale_with_level() {
    let gromp = Monster::Gromp;
    assert_eq!(gromp.stats(1).max_health, 2050.0);
    assert!((gromp.stats(18).max_health - 2050.0 * 2.35).abs() < 0.1);
    assert_eq!(gromp.stats(18).armor, 42.0);
}

#[test]
fn minions_scale_with_game_time() {
    // Level 1 is minute 2: upgrades at 0:30 and 2:00
    assert_eq!(Monster::minion_upgrades(1), 2);
    // Level 10 is minute 20
    assert_eq!(Monster::minion_upgrades(10), 14);

    let melee = Monster::MeleeMinion.stats(10);
    assert_eq!(melee.max_health, 430.0 + 35.0 * 14.0);
    assert!((melee.armor - 0.085 * 8.0 / 2.0 * 9.0).abs() < 1e-4);
    assert_eq!(Monster::MeleeMinion.stats(2).armor, 0.0);

    assert_eq!(Monster::SuperMinion.stats(18).magic_resist, -30.0);
    assert_eq!(
        MonsterStat::Upgrades {
            base: 0.0,
            per_upgrade: 1000.0,
            cap: 5.0
        }
        .at(18),
        5.0
    );
}
