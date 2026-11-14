#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use math::{ChampionId, GameMap, ItemId, calculator::InferStats, model::*};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

#[test]
fn retribution_grows_one_percent_every_seven_percent_missing_health() {
    for (missing, expected) in [
        (-0.5, 0.0),
        (0.0, 0.0),
        (0.069, 0.0),
        (0.07, 0.01),
        (0.14, 0.02),
        (0.5, 0.07),
        (0.7, 0.1),
        (1.0, 0.1),
    ] {
        close(
            RiotFormulas::get_bloodmail_retribution(missing, GameMap::SummonersRift),
            expected,
        );
    }
}

fn infer(current_health: f32) -> PlayerStats {
    let mut modifiers = Modifiers::default();

    InferStats {
        item_exceptions: &[],
        rune_exceptions: &[],
        items: &[ItemId::OverlordsBloodmail],
        runes: &[],
        modifiers: &mut modifiers,
        dragons: Dragons::default(),
        ability_levels: AbilityLevels::default(),
        stacks: 0,
        level: NonZeroU8::new(18).unwrap(),
        champion_id: ChampionId::Garen,
        is_mega_gnar: false,
        current_health,
        game_map: GameMap::SummonersRift,
    }
    .infer()
}

/// Bloodmail adds no health, so the maximum health at full health is final
fn infer_attack_damage(missing_percent: f32) -> f32 {
    let max_health = infer(0.0).max_health;
    infer(max_health * (1.0 - missing_percent / 100.0)).attack_damage
}

#[test]
fn calculator_applies_retribution_from_the_missing_health() {
    let full = infer_attack_damage(0.0);

    close(infer_attack_damage(70.0), full * 1.10);
    close(infer_attack_damage(20.0), full * 1.02);
    close(infer_attack_damage(99.0), full * 1.10);
}

fn state(current_health: f32) -> SelfState {
    SelfState {
        bonus_attack_speed_ratio: types::AttackSpeedRatio::new(0.0),
        stacks: 0.0,
        ability_levels: AbilityLevels::default(),
        current_stats: PlayerStats {
            attack_damage: 200.0,
            current_health,
            max_health: 2000.0,
            ..PlayerStats::default()
        },
        base_stats: BasicStats {
            attack_damage: 100.0,
            max_health: 1000.0,
            ..BasicStats::default()
        },
        level: NonZeroU8::new(18).unwrap(),
        adaptive_type: math::AdaptiveType::Physical,
    }
}

#[test]
fn owned_bloodmail_amplifies_only_the_purchased_attack_damage() {
    let purchase = |current_health| {
        state(current_health)
            .simulated_purchase(
                ItemId::LongSword,
                &[ItemId::OverlordsBloodmail],
                ChampionId::Garen,
                Dragons::default(),
                GameMap::SummonersRift,
            )
            .current_stats
            .attack_damage
    };

    let full = purchase(2000.0);
    // 50% missing health grants 7% of the attack damage from other sources
    close(purchase(1000.0), 200.0 + (full - 200.0) * 1.07);
}

#[test]
fn purchased_bloodmail_amplifies_all_attack_damage() {
    let purchase = |current_health| {
        state(current_health)
            .simulated_purchase(
                ItemId::OverlordsBloodmail,
                &[],
                ChampionId::Garen,
                Dragons::default(),
                GameMap::SummonersRift,
            )
            .current_stats
            .attack_damage
    };

    close(purchase(1000.0), purchase(2000.0) * 1.07);
}
