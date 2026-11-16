#![cfg(feature = "engine")]

use math::{
    AbilityLevels, AdaptiveType, AttackSpeedRatio, ChampionId, Ctx, CtxFlags, MapMode,
    model::DamageTable,
    model::{Modifiers, RangeDamage},
};
use strum::EnumCount;

/// A level 18 attacker with items against a tanky target, so that every
/// variable of the formulas has a value
const CTX: Ctx = Ctx {
    ability_power: 620.0,
    armor: 110.0,
    armor_penetration_flat: 18.0,
    armor_penetration_percent: math::Percent::new(30.0),
    attack_damage: 240.0,
    attack_speed: 1.4,
    base_ad: 110.0,
    base_armor: 90.0,
    base_attack_speed: 0.65,
    base_health: 2300.0,
    base_magic_resist: 50.0,
    base_mana: 1300.0,
    bonus_move_speed: 60.0,
    crit_chance: math::Percent::new(40.0),
    crit_damage: math::CritDamage::new(175.0),
    current_health: 2400.0,
    current_mana: 1500.0,
    level: 18.0,
    magic_penetration_flat: 15.0,
    magic_penetration_percent: math::Percent::new(40.0),
    magic_resist: 70.0,
    max_health: 3100.0,
    max_mana: 1900.0,
    stacks: 3.0,
    enemy_armor: 160.0,
    enemy_bonus_health: 1400.0,
    enemy_current_health: 2600.0,
    enemy_magic_resist: 95.0,
    enemy_max_health: 3500.0,
    multipliers: math::DamageMultipliers::ONE,
    ability_levels: AbilityLevels {
        q: 5,
        w: 5,
        e: 5,
        r: 3,
    },
    flags: CtxFlags::attacker(AdaptiveType::Magic),
    map: MapMode::Default,
    bonus_attack_speed_ratio: AttackSpeedRatio::new(0.75),
};
const MODIFIERS: Modifiers = Modifiers::new(&CTX, AdaptiveType::Magic);

#[test]
fn named_batch_is_const_evaluable() {
    const N: usize = ChampionId::Neeko.number_of_abilities();
    const RESULT: ([i32; N], RangeDamage) = {
        let mut values = [0; N];
        let mut onhit = RangeDamage {
            minimum_damage: 0,
            maximum_damage: 0,
        };
        math::libgen::champions::neeko::neeko_batch(&CTX, MODIFIERS, &mut onhit, &mut values);
        (values, onhit)
    };
    let mut values = [0; N];
    let mut onhit = RangeDamage::default();
    (ChampionId::Neeko.data().ability_batch)(&CTX, MODIFIERS, &mut onhit, &mut values);
    assert_eq!(RESULT, (values, onhit));
}

#[test]
fn generated_batches_match_metadata_evaluation() {
    // A populated, valid context also exercises formulas whose own champion differs.
    let ctx = CTX;
    for i in 0..ChampionId::COUNT {
        let id = ChampionId::from_repr(i as _).unwrap();
        let data = id.data();
        for mut modifiers in [
            Modifiers::default(),
            Modifiers::new(&ctx, data.adaptive_type),
        ] {
            modifiers.damages.global_mod = 0.87;
            modifiers.damages.physical_mod = 0.61;
            modifiers.damages.magic_mod = 0.73;
            modifiers.damages.true_mod = 1.13;
            modifiers.abilities.q = 1.17;
            modifiers.abilities.w = 0.83;
            modifiers.abilities.e = 1.29;
            modifiers.abilities.r = 0.91;
            let mut expected = vec![0; data.metadata.len()];
            let mut actual = expected.clone();
            let mut expected_onhit = RangeDamage::default();
            let mut actual_onhit = RangeDamage::default();
            DamageTable::eval_abilities(
                &ctx,
                &mut expected_onhit,
                modifiers,
                data.metadata,
                data.closures,
                &mut expected,
            );
            (data.ability_batch)(&ctx, modifiers, &mut actual_onhit, &mut actual);
            assert_eq!(actual, expected, "{id:?}");
            assert_eq!(actual_onhit, expected_onhit, "{id:?}");
        }
    }
}
