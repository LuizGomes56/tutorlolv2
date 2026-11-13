#![cfg(feature = "engine")]
use tutorlolv2::{AttackType, Ctx, ItemId, RuneId};
fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.01, "{actual} != {expected}")
}

#[test]
fn compiled_numeric_progression_interpolates_fractional_inputs_and_caps() {
    for (crit_chance, expected) in [
        (-10.0, 125.0),
        (0.0, 125.0),
        (25.5, 137.75),
        (100.0, 175.0),
        (150.0, 175.0),
    ] {
        let ctx = Ctx {
            base_ad: 100.0,
            attack_damage: 100.0,
            crit_chance,
            ..Default::default()
        };
        close(
            ItemId::EssenceReaver.eval(&ctx, AttackType::Melee)[0],
            expected,
        );
    }
}

#[test]
fn compiled_grasp_uses_percentage_units_for_both_attack_types() {
    let ctx = Ctx {
        max_health: 3000.0,
        ..Default::default()
    };
    close(
        RuneId::GraspOfTheUndying.eval(&ctx, AttackType::Melee)[0],
        105.0,
    );
    close(
        RuneId::GraspOfTheUndying.eval(&ctx, AttackType::Ranged)[0],
        42.0,
    );
}

#[test]
fn shield_bash_uses_the_triggering_shield_as_auxiliary_input() {
    let ctx = Ctx {
        level: 18.0,
        max_health: 3000.0,
        base_health: 2000.0,
        stacks: 200.0,
        ..Default::default()
    };
    close(RuneId::ShieldBash.eval(&ctx, AttackType::Melee)[0], 85.0);
}

#[test]
fn compiled_lethal_tempo_scales_with_the_compact_percentage_input() {
    use tutorlolv2_types::AttackSpeedRatio;
    assert_eq!(size_of::<Ctx>(), 128);
    for (ratio, melee, ranged) in [
        (0.0, 30.0, 20.01),
        (0.75, 52.5, 35.0175),
        (1.0, 60.0, 40.02),
    ] {
        let ctx = Ctx {
            level: 18.0,
            bonus_attack_speed_ratio: AttackSpeedRatio::new(ratio),
            ..Default::default()
        };
        close(RuneId::LethalTempo.eval(&ctx, AttackType::Melee)[0], melee);
        close(
            RuneId::LethalTempo.eval(&ctx, AttackType::Ranged)[0],
            ranged,
        );
    }
}

#[test]
fn compact_ratio_json_uses_the_numeric_ratio_and_accepts_older_contexts() {
    use tutorlolv2_types::AttackSpeedRatio;
    let ctx = Ctx {
        bonus_attack_speed_ratio: AttackSpeedRatio::new(1.5),
        ..Default::default()
    };
    let mut value = serde_json::to_value(ctx).unwrap();
    assert_eq!(value["bonus_attack_speed_ratio"], serde_json::json!(1.5));
    let roundtrip: Ctx = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(roundtrip.bonus_attack_speed_ratio(), 1.5);
    value
        .as_object_mut()
        .unwrap()
        .remove("bonus_attack_speed_ratio");
    let legacy: Ctx = serde_json::from_value(value).unwrap();
    assert_eq!(legacy.bonus_attack_speed_ratio(), 0.0);
}
