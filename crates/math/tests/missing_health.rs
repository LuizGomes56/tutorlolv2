#![cfg(feature = "engine")]

use math::{AbilityId::R, AbilityName::Void, ChampionId};
use types::{AbilityLevels, Ctx, CtxVar};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
}

#[test]
fn derived_variables_are_read_through_methods() {
    let ctx = Ctx {
        current_health: 750.0,
        max_health: 1000.0,
        enemy_current_health: 1200.0,
        enemy_max_health: 2000.0,
        ..Default::default()
    };

    close(ctx.missing_health_ratio(), 0.25);
    close(ctx.enemy_missing_health_flat(), 800.0);
    close(ctx.get(CtxVar::MissingHealthRatio), 0.25);
    close(ctx.get(CtxVar::EnemyMissingHealthFlat), 800.0);
    close(ctx.get(CtxVar::EnemyMaxHealth), 2000.0);

    assert!(CtxVar::MissingHealthRatio.is_method());
    assert!(!CtxVar::MaxHealth.is_method());
    // Formulas spell methods as fields
    assert_eq!(
        CtxVar::EnemyMissingHealthFlat.as_var(),
        "ctx.enemy_missing_health_flat"
    );
}

#[test]
fn derived_health_stays_in_range() {
    let full = Ctx {
        current_health: 1500.0,
        max_health: 1000.0,
        enemy_current_health: 2500.0,
        enemy_max_health: 2000.0,
        ..Default::default()
    };
    close(full.missing_health_ratio(), 0.0);
    close(full.enemy_missing_health_flat(), 0.0);

    // The maximum health is at least 1 to avoid division by zero
    close(Ctx::default().missing_health_ratio(), 1.0);
}

#[test]
fn garen_ultimate_scales_with_the_flat_missing_health() {
    let at = |enemy_current_health| {
        let ctx = Ctx {
            ability_levels: AbilityLevels {
                r: 1,
                ..AbilityLevels::default()
            },
            enemy_current_health,
            enemy_max_health: 2000.0,
            ..Default::default()
        };
        ChampionId::Garen.eval(&ctx, R(Void))
    };

    // 25% of the target's missing health at rank 1
    close(at(1000.0) - at(2000.0), 250.0);
}
