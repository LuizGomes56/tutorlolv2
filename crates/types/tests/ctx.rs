use strum::IntoEnumIterator;
use types::{
    AdaptiveType, Ctx, CtxAttacker, CtxFlags, CtxTarget, CtxVar, GameMap, MapMode, ResistValue,
};

#[test]
fn ctx_fits_in_two_cache_lines() {
    assert_eq!(size_of::<Ctx>(), 128);
    // Output tables store these parts instead of a context per row
    assert_eq!(size_of::<CtxAttacker>(), 100);
    assert_eq!(size_of::<CtxTarget>(), 32);
}

#[test]
fn flags_select_the_protections_and_the_adaptive_multiplier() {
    let ctx = Ctx {
        armor_penetration_percent: 150.0,
        enemy_armor: 100.0,
        enemy_magic_resist: 50.0,
        flags: CtxFlags::target(true, false, true, false)
            .union(CtxFlags::attacker(AdaptiveType::Magic)),
        ..Default::default()
    };

    assert_eq!(ctx.steelcaps_effect(), Ctx::STEEL_CAPS_PROTECTION);
    assert_eq!(ctx.randuin_effect(), 1.0);
    assert_eq!(ctx.rocksolid_effect(), Ctx::ROCKSOLID_PROTECTION);
    // Percent penetration is clamped to 100, so all the armor is ignored
    assert_eq!(ctx.physical_multiplier(), 1.0);
    assert_eq!(ctx.magic_multiplier(), 100.0 / 150.0);
    assert_eq!(ctx.adaptive_damage(), ctx.magic_multiplier());
    assert_eq!(ctx.life_steal(), 0.0);
}

#[test]
fn negative_resists_need_their_flag() {
    let resist = |negatives| {
        Ctx {
            armor_penetration_flat: 30.0,
            enemy_armor: 10.0,
            flags: CtxFlags::target(false, false, false, negatives),
            ..Default::default()
        }
        .physical_multiplier()
    };

    assert_eq!(resist(false), 1.0);
    assert_eq!(
        resist(true),
        ResistValue::new(0.0, 30.0, 10.0, true).modifier
    );
    assert!(resist(true) > 1.0);
}

#[test]
fn attacker_and_target_parts_rebuild_the_context() {
    let ctx = Ctx {
        ability_power: 100.0,
        attack_damage: 200.0,
        base_ad: 60.0,
        current_health: 900.0,
        max_health: 1200.0,
        armor_penetration_flat: 18.0,
        level: 11.0,
        stacks: 3.0,
        enemy_armor: 80.0,
        enemy_magic_resist: 40.0,
        enemy_current_health: 700.0,
        enemy_max_health: 2000.0,
        ability_levels: types::AbilityLevels {
            q: 5,
            w: 1,
            e: 3,
            r: 2,
        },
        flags: CtxFlags::target(true, true, false, true)
            .union(CtxFlags::attacker(AdaptiveType::Magic)),
        ..Default::default()
    };
    let rebuilt = Ctx::new(&ctx.attacker(), &ctx.target());

    assert_eq!(rebuilt, ctx);
    for var in CtxVar::iter() {
        assert_eq!(
            rebuilt.get(var).to_bits(),
            ctx.get(var).to_bits(),
            "{var:?}"
        );
    }
    // Each part only keeps its own flags
    assert_eq!(
        ctx.attacker().flags,
        CtxFlags::attacker(AdaptiveType::Magic)
    );
    assert_eq!(
        ctx.target().flags,
        CtxFlags::target(true, true, false, true)
    );
}

#[test]
fn the_map_comes_with_the_attacker() {
    let attacker = CtxAttacker {
        map: GameMap::AramMayhem.mode(),
        ..Default::default()
    };
    let ctx = Ctx::new(&attacker, &CtxTarget::default());

    assert_eq!(ctx.map, MapMode::Aram);
    assert_eq!(ctx.attacker().map, MapMode::Aram);
    assert_eq!(GameMap::SummonersRift.mode(), MapMode::Default);
    assert_eq!(GameMap::Unknown.mode(), MapMode::Default);
}

#[test]
fn a_zeroed_context_has_no_map_changes() {
    // Output tables start zeroed, and their rows must read as the default context
    let zeroed = unsafe { core::mem::zeroed::<Ctx>() };

    assert_eq!(zeroed.map, MapMode::Default);
    assert_eq!(zeroed, Ctx::default());
}
