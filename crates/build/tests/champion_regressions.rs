//! Standalone regression tests of the structured champion generators.
use build::{
    MayFail,
    generators::champions::{CHAMPIONS, champion_gen_fn},
    generators::{
        GeneratorExt,
        parser::{
            Parser,
            champions::{Champion, ChampionParser},
        },
    },
};
use types::{AbilityId, MapMode};
use wiki::{
    champions::WikiChampion,
    wikitext::{LegacyFormula, Sample},
};
type GeneratorFn = fn(WikiChampion) -> MayFail<Box<dyn GeneratorExt<Champion>>>;
const MODES: [MapMode; 3] = [MapMode::Default, MapMode::Aram, MapMode::Arena];

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn every_cached_champion_has_a_structured_generator_and_builds() {
    let parser = ChampionParser::new().unwrap();
    for id in parser.map().keys() {
        assert!(
            CHAMPIONS.contains(&id.as_str()),
            "{id} has no structured generator"
        );
        let champion = v2_champion(&parser, id);
        assert_eq!(&champion.champion_id, id);
    }
    assert_eq!(CHAMPIONS.len(), parser.map().len());
}

/// v2's output of `champion_id`
fn v2_champion(parser: &ChampionParser, champion_id: &str) -> Champion {
    let make: GeneratorFn = champion_gen_fn(champion_id).unwrap();
    make(parser.map()[champion_id].clone())
        .unwrap()
        .call()
        .unwrap()
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn aurelion_sol_breath_burst_is_a_percent_of_stardust() {
    use types::{AbilityName::_2, CtxVar::*};
    // "(+ (3.1% Stardust)% of target's maximum health)": 100 Stardust is 3.1%
    let parser = ChampionParser::new().unwrap();
    let v2 = v2_champion(&parser, "AurelionSol");
    let burst = LegacyFormula::parse(&v2.abilities[&AbilityId::Q(_2)].damage).unwrap();
    let s = Sample::new([(QLevel, 1.0), (Stacks, 100.0), (EnemyMaxHealth, 1000.0)]);
    assert!(
        (burst.eval(&s) - (60.0 + 31.0)).abs() < 1e-6,
        "{}",
        burst.eval(&s)
    );
}

/// v2's damage `id` of `champion_id` at `sample`
fn v2_value(champion: &Champion, id: AbilityId, sample: &Sample) -> f64 {
    LegacyFormula::parse(&champion.abilities[&id].damage)
        .unwrap()
        .eval(sample)
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn ryze_flux_bonus_only_increases_the_empowered_overload() {
    use types::{AbilityName::*, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    let ryze = v2_champion(&parser, "Ryze");
    let at = |r: f64| Sample::new([(QLevel, 1.0), (RLevel, r)]);

    // Overload's 75 at rank 1, and its Flux bonus of 10 / 40 / 70 / 100% by R rank 0 to 3
    assert_eq!(v2_value(&ryze, AbilityId::Q(Void), &at(3.0)), 75.0);
    for (r, bonus) in [(0.0, 1.1), (1.0, 1.4), (3.0, 2.0)] {
        let empowered = v2_value(&ryze, AbilityId::Q(_1), &at(r));
        assert!(
            (empowered - 75.0 * bonus).abs() < 1e-6,
            "R {r}: {empowered}"
        );
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn sett_haymaker_reads_current_grit_within_its_range() {
    use types::{AbilityName::*, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    let sett = v2_champion(&parser, "Sett");
    // Grit is his mana: the maximum is resourceMax and the current one his mana
    let s = Sample::new([(WLevel, 1.0), (MaxMana, 400.0), (CurrentMana, 200.0)]);

    assert_eq!(v2_value(&sett, AbilityId::W(Void), &s), 80.0 + 0.25 * 200.0);
    assert_eq!(v2_value(&sett, AbilityId::W(_1Min), &s), 80.0);
    assert_eq!(
        v2_value(&sett, AbilityId::W(_1Max), &s),
        80.0 + 0.25 * 400.0
    );
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn smolder_passive_bonus_multiplies_whole_by_stacks() {
    use types::{AbilityName::*, CtxVar::*};
    // Q's passive bonus is (0.25 + 0.35 * crit chance) per stack, all of it, and its
    // 60 physical damage is increased by 75% at 100% crit chance
    let parser = ChampionParser::new().unwrap();
    let smolder = v2_champion(&parser, "Smolder");
    let s = Sample::new([
        (QLevel, 1.0),
        (CritChance, 100.0),
        (CritDamage, 200.0),
        (Stacks, 10.0),
        (PhysicalMultiplier, 1.0),
        (MagicMultiplier, 1.0),
    ]);
    let q = v2_value(&smolder, AbilityId::Q(Min), &s);
    assert!((q - (60.0 * 1.75 + 0.6 * 10.0)).abs() < 1e-6, "{q}");
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn mel_and_twitch_use_the_complete_wiki_formulas() {
    use types::{AbilityName::*, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    let mel = v2_champion(&parser, "Mel");
    for (rank, expected) in [(1.0, 235.0), (2.0, 340.0), (3.0, 445.0)] {
        let s = Sample::new([(RLevel, rank), (AbilityPower, 100.0), (Stacks, 10.0)]);
        assert!((v2_value(&mel, AbilityId::R(Void), &s) - expected).abs() < 1e-6);
    }
    let twitch = v2_champion(&parser, "Twitch");
    for level in [1.0, 5.0, 18.0] {
        let s = Sample::new([(Level, level), (AbilityPower, 100.0)]);
        let tick = v2_value(&twitch, AbilityId::P(Min), &s);
        for (id, times) in [(Max, 6.0), (_1Min, 6.0), (_1Max, 36.0)] {
            assert!((v2_value(&twitch, AbilityId::P(id), &s) - tick * times).abs() < 1e-6);
        }
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn hecarim_wiki_increase_matches_the_manual_reference_on_every_map() {
    use build::formula as f;
    use types::{AbilityName::Void, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    let hecarim = v2_champion(&parser, "Hecarim");
    for map in MODES {
        for rank in 1..=5 {
            for ad in [0.0, 100.0, 350.0] {
                for stacks in [0.0, 1.0, 2.0, 3.0] {
                    let s = Sample::new([(QLevel, rank as f64), (BonusAd, ad), (Stacks, stacks)])
                        .on(map);
                    let base = match map {
                        MapMode::Arena => 80.0 + 25.0 * (rank - 1) as f64,
                        _ => 60.0 + 30.0 * (rank - 1) as f64,
                    };
                    let q = base + 0.9 * ad;
                    let expected = f![q * (1 + Stacks * (0.03 + 0.03 * BonusAd / 100))];
                    assert!(
                        (v2_value(&hecarim, AbilityId::Q(Void), &s) - expected.eval(&s).unwrap())
                            .abs()
                            < 1e-6,
                        "{map:?}, rank {rank}, AD {ad}, stacks {stacks}"
                    );
                }
            }
        }
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn lucian_shots_come_from_the_wiki_with_critical_damage_scaling() {
    use types::{AbilityName::*, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    let lucian = v2_champion(&parser, "Lucian");
    for (chance, crit, shots) in [
        (0.0, 200.0, 22.0),
        (25.0, 200.0, 27.5555),
        (100.0, 200.0, 44.222),
        (100.0, 230.0, 50.8886),
    ] {
        let s = Sample::new([
            (RLevel, 1.0),
            (AttackDamage, 100.0),
            (AbilityPower, 100.0),
            (CritChance, chance),
            (CritDamage, crit),
        ]);
        let shot = v2_value(&lucian, AbilityId::R(Min), &s);
        assert!((v2_value(&lucian, AbilityId::R(Max), &s) - shot * shots).abs() < 1e-6);
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn neeko_qiyana_taliyah_map_damage_overrides() {
    use types::{AbilityName::*, CtxVar::*};
    let parser = ChampionParser::new().unwrap();
    for champion_id in ["Neeko", "Qiyana", "Taliyah"] {
        let champion = v2_champion(&parser, champion_id);
        for map in MODES {
            for rank in 1..=5 {
                for ap in [0.0, 200.0] {
                    let arena = map == MapMode::Arena;
                    let r = (rank - 1) as f64;
                    let sample = Sample::new([
                        (QLevel, rank as f64),
                        (WLevel, rank as f64),
                        (AbilityPower, ap),
                        (BonusAd, 100.0),
                    ])
                    .on(map);
                    let cases = match champion_id {
                        "Neeko" => {
                            let initial = if arena {
                                90.0 + 50.0 * r
                            } else {
                                60.0 + 50.0 * r
                            } + 0.6 * ap;
                            let subsequent = if arena {
                                75.0 + 25.0 * r
                            } else {
                                35.0 + 25.0 * r
                            } + 0.25 * ap;
                            vec![
                                (AbilityId::Q(Min), initial),
                                (AbilityId::Q(_1), subsequent),
                                (AbilityId::Q(Max), initial + 2.0 * subsequent),
                                (
                                    AbilityId::W(Void),
                                    if arena {
                                        70.0 + 35.0 * r + 0.7 * ap
                                    } else {
                                        30.0 + 35.0 * r + 0.6 * ap
                                    },
                                ),
                            ]
                        }
                        "Qiyana" => vec![(
                            AbilityId::W(Void),
                            8.0 + 8.0 * r
                                + if arena {
                                    35.0 + 0.55 * ap
                                } else {
                                    20.0 + 0.45 * ap
                                },
                        )],
                        _ => {
                            let initial = if arena {
                                70.0 + 20.0 * r + 0.6 * ap
                            } else {
                                55.0 + 17.5 * r + 0.5 * ap
                            };
                            vec![
                                (AbilityId::Q(Min), initial),
                                (AbilityId::Q(_2), 0.4 * initial),
                                (AbilityId::Q(Max), 2.6 * initial),
                                (AbilityId::Q(_1), 1.8 * initial),
                            ]
                        }
                    };
                    for (id, expected) in cases {
                        let actual = v2_value(&champion, id, &sample);
                        assert!(
                            (actual - expected).abs() < 1e-6,
                            "{champion_id} {id:?} {map:?} rank {rank} AP {ap}: {actual} != {expected}"
                        );
                    }
                }
            }
        }
    }
}

// Values of the samples of `assert_map_values`
const BONUS_AD: f64 = 100.0;
const AD: f64 = 160.0;
const STACKS: f64 = 4.0;
const BONUS_ATTACK_SPEED: f64 = 50.0;
const MAX_HEALTH: f64 = 2000.0;
const CURRENT_HEALTH: f64 = 1500.0;
const MISSING_HEALTH: f64 = 500.0;

/// `arena` on Arena, `other` elsewhere
fn pick(map: MapMode, arena: f64, other: f64) -> f64 {
    match map {
        MapMode::Arena => arena,
        _ => other,
    }
}

/// Checks each damage of `champion_id` against `expected`, written from the wiki's
/// map change bullets, on every mode, at ranks 1 to 5 and with and without AP.
/// `expected` gets the mode, the rank minus 1, the AP and the default mode's sample
fn assert_map_values(
    parser: &ChampionParser,
    champion_id: &str,
    expected: impl Fn(MapMode, f64, f64, &dyn Fn(AbilityId) -> f64) -> Vec<(AbilityId, f64)>,
) {
    use types::CtxVar::*;
    let champion = v2_champion(parser, champion_id);
    for map in MODES {
        for rank in 1..=5 {
            for ap in [0.0, 200.0] {
                let sample = Sample::new([
                    (Level, 9.0),
                    (QLevel, rank as f64),
                    (WLevel, rank as f64),
                    (ELevel, rank as f64),
                    (RLevel, rank as f64),
                    (AbilityPower, ap),
                    (AttackDamage, AD),
                    (BonusAd, BONUS_AD),
                    (Stacks, STACKS),
                    (BonusAttackSpeed, BONUS_ATTACK_SPEED),
                    (CurrentMana, 300.0),
                    (MaxMana, 600.0),
                    (EnemyMaxHealth, MAX_HEALTH),
                    (EnemyCurrentHealth, CURRENT_HEALTH),
                    (EnemyMissingHealthFlat, MISSING_HEALTH),
                ]);
                let default = |id| v2_value(&champion, id, &sample);
                let r = (rank - 1) as f64;
                for (id, expected) in expected(map, r, ap, &default) {
                    let actual = v2_value(&champion, id, &sample.clone().on(map));
                    assert!(
                        (actual - expected).abs() < 1e-6,
                        "{champion_id} {id:?} {map:?} rank {rank} AP {ap}: {actual} != {expected}"
                    );
                }
            }
        }
    }
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn camille_to_yuumi_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Camille", |map, r, _, _| {
        let per_100 = pick(map, 0.03, 0.025);
        vec![(
            W(Max),
            60.0 + 25.0 * r
                + 0.6 * BONUS_AD
                + (0.07 + 0.005 * r + per_100 * BONUS_AD / 100.0) * MAX_HEALTH,
        )]
    });

    assert_map_values(&parser, "Evelynn", |map, r, ap, _| {
        let dart = pick(map, 35.0 + 10.0 * r + 0.35 * ap, 25.0 + 5.0 * r + 0.25 * ap);
        let spike = pick(map, 20.0 + 10.0 * r + 0.4 * ap, 25.0 + 5.0 * r + 0.25 * ap);
        let bonus = 15.0 + 10.0 * r + 0.25 * ap;
        vec![
            (Q(Min), dart),
            (Q(Max), dart + 3.0 * spike + 3.0 * bonus),
            (
                E(Void),
                60.0 + 30.0 * r + (pick(map, 0.04, 0.03) + 0.015 * ap / 100.0) * MAX_HEALTH,
            ),
            (
                E(_1),
                80.0 + 40.0 * r + (pick(map, 0.06, 0.04) + 0.025 * ap / 100.0) * MAX_HEALTH,
            ),
        ]
    });

    assert_map_values(&parser, "Kaisa", |map, r, ap, default| {
        let bonus_ad = match map {
            MapMode::Aram => 0.65,
            MapMode::Arena => 0.5,
            _ => 0.55,
        };
        let missile = 40.0 + 15.0 * r + bonus_ad * BONUS_AD + pick(map, 0.15, 0.2) * ap;
        vec![
            (P(Void), pick(map, 0.5, 1.0) * default(P(Void))),
            (
                P(_1),
                (pick(map, 0.1, 0.15) + 0.06 * ap / 100.0) * MISSING_HEALTH,
            ),
            (Q(Min), missile),
            (Q(Max), 2.25 * missile),
            (Q(_1), 3.75 * missile),
        ]
    });

    assert_map_values(&parser, "Kindred", |map, r, ap, _| {
        vec![
            (
                W(Void),
                pick(map, 10.0, 25.0)
                    + 5.0 * r
                    + pick(map, 0.1, 0.2) * BONUS_AD
                    + 0.2 * ap
                    + (pick(map, 0.01, 0.015) + pick(map, 0.005, 0.01) * STACKS) * CURRENT_HEALTH,
            ),
            (
                E(Void),
                80.0 + 30.0 * r
                    + BONUS_AD
                    + (pick(map, 0.02, 0.05) + 0.005 * STACKS) * MISSING_HEALTH,
            ),
        ]
    });

    assert_map_values(&parser, "KogMaw", |map, r, ap, _| {
        let health = match map {
            MapMode::Aram => 0.025 + 0.005 * r,
            MapMode::Arena => 0.02 + 0.0075 * r,
            _ => 0.03 + 0.0075 * r,
        };
        vec![
            (Q(Void), 80.0 + 45.0 * r + pick(map, 1.0, 0.9) * ap),
            (W(Void), (health + 0.015 * ap / 100.0) * MAX_HEALTH),
            (E(Void), 70.0 + 40.0 * r + pick(map, 1.0, 0.65) * ap),
        ]
    });

    assert_map_values(&parser, "Lillia", |map, _, ap, _| {
        vec![(
            P(Void),
            (pick(map, 0.04, 0.05) + 0.0125 * ap / 100.0) * MAX_HEALTH,
        )]
    });

    assert_map_values(&parser, "Naafiri", |map, r, _, _| {
        let initial = pick(map, 40.0 + 10.0 * r, 35.0 + 5.0 * r) + 0.2 * BONUS_AD;
        let bleed = pick(map, 40.0 + 40.0 * r, 35.0 + 25.0 * r) + 0.8 * BONUS_AD;
        let minimum = pick(map, 40.0 + 15.0 * r, 30.0 + 12.5 * r) + pick(map, 0.45, 0.4) * BONUS_AD;
        let maximum = pick(map, 80.0 + 30.0 * r, 60.0 + 25.0 * r) + pick(map, 1.6, 1.4) * BONUS_AD;
        let dash = pick(map, 30.0, 15.0) + 10.0 * r + pick(map, 0.55, 0.4) * BONUS_AD;
        let flurry = pick(map, 70.0, 60.0) + 25.0 * r + pick(map, 1.0, 0.8) * BONUS_AD;
        vec![
            (
                P(Void),
                10.0 + 10.0 / 17.0 * 8.0 + pick(map, 0.06, 0.04) * BONUS_AD,
            ),
            (Q(_1), bleed),
            (Q(Min), initial + bleed + minimum),
            (Q(Max), initial + bleed + maximum),
            (E(Min), dash),
            (E(Max), dash + flurry),
        ]
    });

    assert_map_values(&parser, "Sett", |map, r, _, _| {
        let grit = match map {
            MapMode::Aram => 0.2,
            _ => 0.25,
        } + 0.25 * BONUS_AD / 100.0;
        vec![
            (W(Void), 80.0 + 20.0 * r + grit * 300.0),
            (W(_1Max), 80.0 + 20.0 * r + grit * 600.0),
        ]
    });

    assert_map_values(&parser, "Viktor", |map, r, ap, _| {
        let device = pick(map, 50.0 + 12.5 * r + 0.3 * ap, 60.0 + 15.0 * r + 0.4 * ap);
        let discharge = pick(map, 20.0 + 15.0 * r, 20.0 + 25.0 * r) + AD + 0.5 * ap;
        let laser = 70.0 + 40.0 * r + pick(map, 0.4, 0.5) * ap;
        let aftershock = 20.0 + 30.0 * r + 0.8 * ap;
        let initial = pick(map, 75.0 + 50.0 * r, 100.0 + 75.0 * r) + 0.6 * ap;
        vec![
            (Q(Min), device),
            (Q(Void), discharge),
            (Q(Max), device + discharge),
            (E(Max), laser + aftershock),
            (R(Void), initial),
            (R(Max), initial + 6.0 * (65.0 + 40.0 * r + 0.35 * ap)),
        ]
    });

    assert_map_values(&parser, "Yuumi", |map, r, ap, _| {
        vec![(
            Q(Max),
            pick(map, 100.0 + 75.0 * r + 0.6 * ap, 80.0 + 55.0 * r + 0.3 * ap),
        )]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn brand_to_urgot_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Brand", |map, _, _, _| {
        vec![(P(Void), pick(map, 0.015, 0.02) * MAX_HEALTH)]
    });

    assert_map_values(&parser, "Ekko", |map, r, ap, _| {
        let initial = pick(map, 90.0, 80.0) + 15.0 * r + 0.3 * ap;
        let secondary = pick(map, 50.0, 40.0) + 25.0 * r + 0.7 * ap;
        vec![
            // Level 9: {{pp|33 to 154}} on Arena, 85 + 5 per level from 7 elsewhere
            (
                P(Void),
                pick(map, 33.0 + 121.0 / 17.0 * 8.0, 95.0) + 0.8 * ap,
            ),
            (Q(_1Min), initial),
            (Q(_1Max), secondary),
            (Q(Max), initial + secondary),
        ]
    });

    assert_map_values(&parser, "Heimerdinger", |map, r, ap, _| {
        let initial = 50.0 + 25.0 * r + pick(map, 0.65, 0.55) * ap;
        vec![
            (
                Q(Void),
                pick(map, 4.0 + 2.0 * r, 7.0 + 4.0 * r) + pick(map, 0.25, 0.35) * ap,
            ),
            (
                Q(_1),
                pick(map, 50.0 + 15.0 * r, 80.0 + 20.0 * r) + 0.35 * ap,
            ),
            (W(Min), initial),
            (W(Max), initial + 4.0 * (10.0 + 5.0 * r + 0.12 * ap)),
            (W(_1Min), 135.0 + 45.0 * r + 0.45 * ap),
        ]
    });

    assert_map_values(&parser, "Malzahar", |map, _, _, default| {
        vec![(
            W(Void),
            default(W(Void)) + pick(map, 0.6 - 0.4, 0.0) * BONUS_AD,
        )]
    });

    assert_map_values(&parser, "MonkeyKing", |map, r, _, _| {
        let tick = pick(map, 0.0075 + 0.00375 * r, 0.01 + 0.005 * r) * MAX_HEALTH
            + pick(map, 1.1 / 4.0, 2.75 / 8.0) * AD;
        vec![
            (R(Min), tick),
            (R(Max), 8.0 * tick),
            (R(_1Max), 16.0 * tick),
        ]
    });

    assert_map_values(&parser, "Rengar", |map, r, ap, _| {
        let ad = match map {
            MapMode::Aram => 0.1 + 0.0375 * r,
            MapMode::Arena => 0.05 + 0.05 * r,
            _ => 0.05,
        };
        let bonus_ad = match map {
            MapMode::Aram => 1.0,
            _ => 0.8,
        };
        vec![
            (
                Q(Void),
                pick(map, 40.0 + 40.0 * r, 20.0 + 35.0 * r) + ad * AD,
            ),
            (
                W(Void),
                pick(map, 60.0 + 40.0 * r, 50.0 + 30.0 * r) + pick(map, 0.95, 0.8) * ap,
            ),
            (E(Void), 55.0 + 45.0 * r + bonus_ad * BONUS_AD),
        ]
    });

    assert_map_values(&parser, "Samira", |map, r, _, _| {
        let shot = pick(map, 15.0 + 15.0 * r, 20.0 + 20.0 * r) + 0.3 * AD;
        vec![(R(Min), shot), (R(Max), 10.0 * shot)]
    });

    assert_map_values(&parser, "Twitch", |map, r, ap, default| {
        let venom = |id, ticks: f64| default(id) + pick(map, 0.01 * ticks, 0.0) * ap;
        vec![
            (P(Min), venom(P(Min), 1.0)),
            (P(Max), venom(P(Max), 6.0)),
            (P(_1Min), venom(P(_1Min), 6.0)),
            (P(_1Max), venom(P(_1Max), 36.0)),
            (
                E(Min),
                35.0 + 15.0 * r + pick(map, 0.4, 0.35) * BONUS_AD + 0.35 * ap,
            ),
            (
                E(Max),
                110.0 + 40.0 * r + pick(map, 2.4, 2.1) * BONUS_AD + 2.1 * ap,
            ),
        ]
    });

    assert_map_values(&parser, "Udyr", |map, r, ap, _| {
        let tick = pick(map, 12.5 + 10.0 * r, 10.0 + 8.0 * r) + 0.175 * ap;
        vec![
            (
                Q(Min),
                (0.03 + 0.01 * r + pick(map, 0.015, 0.035) * BONUS_AD / 100.0) * MAX_HEALTH,
            ),
            (
                Q(Max),
                (0.06 + 0.02 * r + pick(map, 0.03, 0.07) * BONUS_AD / 100.0) * MAX_HEALTH,
            ),
            (R(Min), tick),
            (R(Max), 8.0 * tick),
        ]
    });

    assert_map_values(&parser, "Urgot", |map, r, _, _| {
        vec![(W(Void), pick(map, 17.0, 12.0) + (0.2 + 0.035 * r) * AD)]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn akali_to_trundle_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Akali", |map, r, ap, _| {
        let ratio = match map {
            MapMode::Default => 0.3,
            _ => 0.4,
        };
        vec![
            (R(_1), 110.0 + 110.0 * r + 0.5 * BONUS_AD + ratio * ap),
            (R(Min), 70.0 + 70.0 * r + ratio * ap),
            (R(Max), 210.0 + 210.0 * r + 3.0 * ratio * ap),
        ]
    });

    assert_map_values(&parser, "Irelia", |map, r, ap, _| {
        vec![
            // Level 9: {{pp|15 to 100}} on Arena
            (
                P(Void),
                pick(map, 15.0 + 5.0 * 8.0, 10.0 + 3.0 * 8.0) + 0.2 * BONUS_AD,
            ),
            (R(Void), 125.0 + 75.0 * r + ap),
        ]
    });

    assert_map_values(&parser, "Lulu", |map, r, ap, _| {
        let primary = 60.0 + 35.0 * r + 0.5 * ap;
        let secondary = pick(map, 0.75, 0.5) * primary;
        vec![(Q(_1), secondary), (Q(Max), primary + secondary)]
    });

    assert_map_values(&parser, "Maokai", |map, r, ap, _| {
        let aram = map == MapMode::Aram;
        vec![
            (E(Min), 50.0 + 25.0 * r + if aram { 0.2 } else { 0.25 } * ap),
            (E(Max), 100.0 + 50.0 * r + if aram { 0.4 } else { 0.5 } * ap),
        ]
    });

    assert_map_values(&parser, "MissFortune", |map, r, ap, _| {
        vec![
            (E(Min), 11.25 + 3.75 * r + pick(map, 0.225, 0.15) * ap),
            (E(Max), 90.0 + 30.0 * r + pick(map, 1.8, 1.2) * ap),
            (
                R(Max),
                280.0
                    + 220.0 * r
                    + (8.4 + 1.2 * r) * AD
                    + (14.0 + 2.0 * r) * pick(map, 0.35, 0.25) * ap,
            ),
        ]
    });

    assert_map_values(&parser, "Trundle", |map, r, ap, _| {
        let total = pick(map, 0.1 + 0.05 * r, 0.2 + 0.05 * r);
        vec![
            (R(Max), (total + 0.02 * ap / 100.0) * MAX_HEALTH),
            (R(Min), (total / 2.0 + 0.01 * ap / 100.0) * MAX_HEALTH),
            (R(Void), (total / 8.0 + 0.0025 * ap / 100.0) * MAX_HEALTH),
        ]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn akshan_to_zoe_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Akshan", |map, r, _, _| {
        let shot = match map {
            MapMode::Arena => {
                (8.0 + 8.0 * r + 0.2459 * BONUS_AD) * (1.0 + 0.45 * BONUS_ATTACK_SPEED / 100.0)
            }
            _ => (8.0 + 8.0 * r + 0.25 * AD) * (1.0 + 0.3 * BONUS_ATTACK_SPEED / 100.0),
        };
        vec![(E(Void), shot)]
    });

    assert_map_values(&parser, "Cassiopeia", |map, r, ap, default| {
        vec![
            (E(Min), 20.0 + 25.0 * r + pick(map, 0.4, 0.45) * ap),
            (E(Max), default(E(Max)) + pick(map, -0.05, 0.0) * ap),
        ]
    });

    assert_map_values(&parser, "Kayle", |map, r, ap, default| {
        vec![
            (
                P(Void),
                default(P(Void)) + pick(map, -0.03 * BONUS_AD - 0.08 * ap, 0.0),
            ),
            (
                E(Void),
                (pick(map, 0.065, 0.08) + 0.005 * r + 0.015 * ap / 100.0) * MISSING_HEALTH,
            ),
        ]
    });

    assert_map_values(&parser, "Kayn", |map, r, _, _| {
        let aram = map == MapMode::Aram;
        vec![
            (
                Q(Min),
                75.0 + 30.0 * r + if aram { 0.95 } else { 0.85 } * BONUS_AD,
            ),
            (
                Q(Max),
                150.0 + 60.0 * r + if aram { 1.9 } else { 1.7 } * BONUS_AD,
            ),
        ]
    });

    assert_map_values(&parser, "Mel", |map, r, _, default| {
        let per_stack = pick(map, (5.0 + r) - (4.0 + 3.0 * r), 0.0);
        vec![(R(Void), default(R(Void)) + per_stack * STACKS)]
    });

    assert_map_values(&parser, "Milio", |map, _, _, default| {
        // Level 9: {{pp|20 to 100}} on Arena, from {{pp|10 to 50}}
        let burn = pick(map, 10.0 + 40.0 / 17.0 * 8.0, 0.0);
        vec![(P(Void), default(P(Void)) + burn)]
    });

    assert_map_values(&parser, "Tristana", |map, r, ap, _| {
        vec![
            (
                E(Min),
                60.0 + 25.0 * r
                    + pick(map, 0.75 + 0.1 * r, 0.8) * BONUS_AD
                    + pick(map, 0.8, 0.5) * ap,
            ),
            (
                E(Max),
                120.0
                    + 50.0 * r
                    + pick(map, 1.5 + 0.2 * r, 1.6) * BONUS_AD
                    + pick(map, 1.6, 1.0) * ap,
            ),
        ]
    });

    assert_map_values(&parser, "Zoe", |map, r, ap, default| {
        vec![
            (Q(Min), default(Q(Min)) + pick(map, 0.1, 0.0) * ap),
            (Q(Max), default(Q(Max)) + pick(map, 0.25, 0.0) * ap),
            (
                W(Min),
                pick(map, 20.0 + 15.0 * r, 15.0 + 10.0 * r) + 0.1 * ap,
            ),
            (
                W(Max),
                pick(map, 60.0 + 45.0 * r, 45.0 + 30.0 * r) + 0.3 * ap,
            ),
        ]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn annie_to_zyra_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Annie", |map, r, ap, _| {
        vec![
            (
                R(_1),
                pick(map, 25.0 + 25.0 * r, 30.0 + 15.0 * r) + 0.1 * ap,
            ),
            (
                R(_2),
                pick(map, (15.0 + 5.0 * r) / 4.0, 2.0 + r) + 0.01 * ap,
            ),
        ]
    });

    assert_map_values(&parser, "Ashe", |map, r, _, _| {
        let volley = match map {
            MapMode::Arena => 25.0 + 20.0 * r + 1.2 * AD,
            _ => 60.0 + 35.0 * r + BONUS_AD,
        };
        vec![(W(Void), volley)]
    });

    assert_map_values(&parser, "AurelionSol", |map, r, ap, default| {
        let per_second = pick(map, 0.6, 0.55);
        vec![
            (Q(_1), 45.0 + 15.0 * r + per_second * ap),
            (Q(Min), (45.0 + 15.0 * r + per_second * ap) / 8.0),
            (
                Q(Max),
                default(Q(Max)) + (per_second - 0.55) * ap * 26.0 / 8.0,
            ),
            (
                Q(_2),
                default(Q(_2)) + pick(map, 0.04 - 0.031, 0.0) * STACKS / 100.0 * MAX_HEALTH,
            ),
        ]
    });

    assert_map_values(&parser, "Leblanc", |map, r, ap, _| {
        let sigil = pick(map, 85.0, 65.0) + 25.0 * r + 0.4 * ap;
        let mimic = pick(map, 90.0 + 70.0 * r + 0.5 * ap, 70.0 + 80.0 * r + 0.4 * ap);
        vec![
            (Q(Min), sigil),
            (Q(Max), 2.0 * sigil),
            (R(_1), mimic),
            (R(_2), 2.0 * mimic),
            (R(_4), mimic),
            (R(_6), mimic + 140.0 + 160.0 * r + 0.85 * ap),
        ]
    });

    assert_map_values(&parser, "Pantheon", |map, r, ap, _| {
        vec![(
            W(Void),
            (0.06 + 0.005 * r + pick(map, 0.02, 0.015) * ap / 100.0) * MAX_HEALTH,
        )]
    });

    assert_map_values(&parser, "Skarner", |map, r, _, default| {
        let per_hit = pick(map, 10.0 + 10.0 * r, 0.0);
        vec![
            (Q(Min), default(Q(Min)) + per_hit),
            (Q(Max), default(Q(Max)) + 3.0 * per_hit),
            (
                Q(Void),
                default(Q(Void)) + per_hit + pick(map, 0.01, 0.0) * MAX_HEALTH,
            ),
        ]
    });

    assert_map_values(&parser, "Talon", |map, r, _, default| {
        let aram = map == MapMode::Aram;
        let out =
            pick(map, 60.0 + 15.0 * r, 50.0 + 10.0 * r) + if aram { 0.5 } else { 0.4 } * BONUS_AD;
        let back =
            pick(map, 75.0 + 37.5 * r, 60.0 + 30.0 * r) + if aram { 1.0 } else { 0.9 } * BONUS_AD;
        vec![
            (P(Void), default(P(Void)) + pick(map, 0.5, 0.0) * BONUS_AD),
            (W(Min), back),
            (W(Max), out + back),
        ]
    });

    assert_map_values(&parser, "Zyra", |map, _, ap, default| {
        vec![(P(Void), default(P(Void)) + pick(map, -0.05, 0.0) * ap)]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn ambessa_to_ziggs_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Ambessa", |map, r, _, default| {
        // Of the increased damage, halved for the others
        let health = |id, share: f64, default_health: f64| {
            default(id) + pick(map, share * (0.01 + 0.00375 * r) - default_health, 0.0) * MAX_HEALTH
        };
        vec![
            (
                Q(Max),
                health(Q(Max), 1.0, 0.04 + 0.005 * r) + pick(map, -0.15, 0.0) * BONUS_AD,
            ),
            (
                Q(_1Max),
                health(Q(_1Max), 1.0, 0.04 + 0.005 * r) + pick(map, -0.2, 0.0) * BONUS_AD,
            ),
            (
                Q(Min),
                health(Q(Min), 0.5, 0.02 + 0.0025 * r) + pick(map, -0.075, 0.0) * BONUS_AD,
            ),
            (
                Q(_1Min),
                health(Q(_1Min), 0.5, 0.02 + 0.0025 * r) + pick(map, -0.1, 0.0) * BONUS_AD,
            ),
        ]
    });

    assert_map_values(&parser, "Blitzcrank", |map, _, ap, default| {
        vec![
            (R(Void), default(R(Void)) + pick(map, 0.3, 0.0) * ap),
            (R(_1), default(R(_1)) + pick(map, 10.0 + 0.1 * ap, 0.0)),
        ]
    });

    assert_map_values(&parser, "Fiora", |map, _, _, _| {
        let vital = (pick(map, 0.02, 0.03) + pick(map, 0.03, 0.04) * BONUS_AD / 100.0) * MAX_HEALTH;
        vec![(P(Void), vital), (R(Void), 4.0 * vital)]
    });

    assert_map_values(&parser, "KSante", |map, _, _, default| {
        // Level 9: 2% to 3% on Arena
        let arena = 12.0 + (0.02 + 0.01 / 17.0 * 8.0) * MAX_HEALTH;
        vec![(P(Void), pick(map, arena, default(P(Void))))]
    });

    assert_map_values(&parser, "Katarina", |map, _, _, default| {
        vec![(P(Void), default(P(Void)) + pick(map, 0.4, 0.0) * BONUS_AD)]
    });

    assert_map_values(&parser, "RekSai", |map, r, ap, _| {
        vec![(
            W(Void),
            pick(map, 50.0, 30.0) + 25.0 * r + pick(map, 1.0, 0.8) * ap,
        )]
    });

    assert_map_values(&parser, "Ryze", |map, r, _, default| {
        // Realm Warp's rank is r + 1 in the samples
        let arena = default(Q(Void)) * (1.0 + (10.0 + 20.0 * (r + 1.0)) / 100.0);
        vec![(Q(_1), pick(map, arena, default(Q(_1))))]
    });

    assert_map_values(&parser, "Singed", |map, r, ap, _| {
        let total = pick(map, 0.65, 0.85);
        vec![
            (Q(Void), 40.0 + 20.0 * r + total * ap),
            (Q(Min), 20.0 + 10.0 * r + total / 2.0 * ap),
            (Q(Max), 5.0 + 2.5 * r + total / 8.0 * ap),
        ]
    });

    assert_map_values(&parser, "Swain", |map, r, ap, _| {
        vec![(
            R(Void),
            pick(map, 3.75 + 2.5 * r, 7.5 + 5.0 * r) + 0.02 * ap,
        )]
    });

    assert_map_values(&parser, "Ziggs", |map, r, ap, default| {
        let first = match map {
            MapMode::Aram => 0.25 + 0.025 * r,
            _ => 0.25 + 0.05 * r,
        };
        vec![
            (P(Void), default(P(Void)) + pick(map, 0.25, 0.0) * ap),
            (E(Void), 30.0 + 40.0 * r + first * ap),
            (E(Min), 12.0 + 16.0 * r + 0.4 * first * ap),
            (E(Max), 150.0 + 200.0 * r + 5.0 * first * ap),
        ]
    });
}

#[test]
#[ignore = "needs the cache of wiki pages and model.json"]
fn jax_and_sivir_map_damage_overrides() {
    use AbilityId::*;
    use types::AbilityName::*;
    let parser = ChampionParser::new().unwrap();

    assert_map_values(&parser, "Jax", |map, r, ap, _| {
        vec![
            (R(Void), 100.0 + 75.0 * r + ap),
            (
                R(_1),
                pick(map, 50.0 + 40.0 * r, 75.0 + 55.0 * r) + 0.6 * ap,
            ),
        ]
    });

    assert_map_values(&parser, "Sivir", |map, r, ap, _| {
        let blade = 60.0
            + 25.0 * r
            + pick(map, (0.8 + 0.05 * r) * AD, 0.7 * BONUS_AD)
            + pick(map, 0.8, 0.6) * ap;
        vec![
            (Q(Void), blade),
            (Q(Min), 0.4 * blade),
            (Q(Max), 2.0 * blade),
        ]
    });
}
