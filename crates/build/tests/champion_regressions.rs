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
