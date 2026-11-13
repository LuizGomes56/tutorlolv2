#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use tutorlolv2::{ChampionId, GameMap, ItemId, calculator::InferStats, model::*};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

fn infer_with_health(items: &[ItemId], current_health: f32) -> PlayerStats {
    let mut modifiers = Modifiers::default();

    InferStats {
        item_exceptions: &[],
        rune_exceptions: &[],
        items,
        runes: &[],
        modifiers: &mut modifiers,
        dragons: Dragons::default(),
        ability_levels: AbilityLevels::default(),
        stacks: 0,
        level: NonZeroU8::new(18).unwrap(),
        champion_id: ChampionId::Ahri,
        is_mega_gnar: false,
        current_health,
        game_map: GameMap::SummonersRift,
    }
    .infer()
}

/// Infers the stats missing `missing_percent` of the maximum health inferred
/// at full health
fn infer(items: &[ItemId], missing_percent: f32) -> PlayerStats {
    let max_health = infer_with_health(items, 0.0).max_health;
    infer_with_health(items, max_health * (1.0 - missing_percent / 100.0))
}

#[test]
fn riftmaker_converts_two_percent_of_bonus_health() {
    let alone = infer(&[ItemId::Riftmaker], 0.0);
    let with_belt = infer(&[ItemId::Riftmaker, ItemId::GiantsBelt], 0.0);

    let added_health = with_belt.max_health - alone.max_health;
    assert!(added_health > 0.0);
    close(
        with_belt.ability_power - alone.ability_power,
        0.02 * added_health,
    );
}

#[test]
fn inferred_current_health_is_clamped_to_the_maximum_health() {
    let full = infer_with_health(&[ItemId::GiantsBelt], 0.0);
    close(full.current_health, full.max_health);
    close(full.missing_health_ratio(), 0.0);

    let stats = infer_with_health(&[ItemId::GiantsBelt], 1000.0);
    close(stats.current_health, 1000.0);
    close(
        stats.missing_health_ratio(),
        1.0 - 1000.0 / stats.max_health,
    );

    // Zero, negative, NaN and values above the maximum health mean full health
    for current_health in [-50.0, f32::NAN, full.max_health + 500.0] {
        let stats = infer_with_health(&[ItemId::GiantsBelt], current_health);
        close(stats.current_health, full.max_health);
    }
}

#[test]
fn missing_health_ratio_ranges_from_zero_to_one() {
    for (current_health, max_health, expected) in [
        (1000.0, 1000.0, 0.0),
        (750.0, 1000.0, 0.25),
        (0.0, 1000.0, 1.0),
        (1500.0, 1000.0, 0.0),
        (-10.0, 1000.0, 1.0),
        // The maximum health is at least 1 to avoid division by zero
        (0.0, 0.0, 1.0),
    ] {
        close(
            RiotFormulas::missing_health_ratio(current_health, max_health),
            expected,
        );
    }
}

#[test]
fn sinister_pact_grows_every_125_missing_health() {
    for (missing, expected) in [
        (0.0, 0.0),
        (124.0, 0.0),
        (125.0, 1.875),
        (1000.0, 15.0),
        (3000.0, 45.0),
        (9000.0, 45.0),
    ] {
        close(RiotFormulas::get_sinister_pact(missing), expected);
    }
}

#[test]
fn demonic_embrace_uses_the_flat_missing_health() {
    let full = infer(&[ItemId::DemonicEmbrace], 0.0);
    let half = infer(&[ItemId::DemonicEmbrace], 50.0);
    let pact = RiotFormulas::get_sinister_pact(half.max_health * 0.5);

    assert!(pact > 0.0);
    close(half.ability_power - full.ability_power, pact);

    // Rabadon's Deathcap amplifies it like any other ability power
    let items = [ItemId::DemonicEmbrace, ItemId::RabadonsDeathcap];
    let amplified = infer(&items, 50.0).ability_power - infer(&items, 0.0).ability_power;
    close(amplified, 1.3 * pact);

    // Health does not grant ability power anymore
    let with_belt = infer(&[ItemId::DemonicEmbrace, ItemId::GiantsBelt], 0.0);
    close(with_belt.ability_power, full.ability_power);
}

fn state(current_health: f32) -> SelfState {
    SelfState {
        bonus_attack_speed_ratio: tutorlolv2_types::AttackSpeedRatio::new(0.0),
        stacks: 0.0,
        ability_levels: AbilityLevels::default(),
        current_stats: PlayerStats {
            ability_power: 100.0,
            current_health,
            max_health: 2000.0,
            ..PlayerStats::default()
        },
        base_stats: BasicStats {
            max_health: 1000.0,
            ..BasicStats::default()
        },
        level: NonZeroU8::new(18).unwrap(),
        adaptive_type: tutorlolv2::AdaptiveType::Magic,
    }
}

#[test]
fn purchased_demonic_embrace_uses_the_observed_missing_health() {
    let purchase = |current_health, owned: &[ItemId], item| {
        state(current_health)
            .simulated_purchase(
                item,
                owned,
                ChampionId::Ahri,
                Dragons::default(),
                GameMap::SummonersRift,
            )
            .current_stats
            .ability_power
    };

    let full = purchase(2000.0, &[], ItemId::DemonicEmbrace);
    close(purchase(1000.0, &[], ItemId::DemonicEmbrace), full + 15.0);

    // Owned, its ability power is already observed and purchases keep the
    // flat missing health
    let belt = |current_health| {
        purchase(
            current_health,
            &[ItemId::DemonicEmbrace],
            ItemId::GiantsBelt,
        )
    };
    close(belt(1000.0), belt(2000.0));
}
