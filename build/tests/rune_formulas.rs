use tutorlolv2_build_dep::generators::parser::runes::Rune;
use tutorlolv2_types::{
    AttackType::{Melee, Ranged},
    CtxVar::MaxHealth,
};
use tutorlolv2_wiki::{
    runes::{RuneKeystone, RuneSlot, WikiRune},
    wikitext::{Sample, SourceForm},
};
fn rune(source: &str) -> Rune {
    Rune::try_from(WikiRune {
        name: "Test".into(),
        rune_id: "Test".into(),
        path: RuneKeystone::Resolve,
        slot: RuneSlot::Keystone,
        riot_id: 42,
        custom: false,
        effects: Default::default(),
        descriptions: vec![],
        raw_source: source.into(),
    })
    .unwrap()
}
#[test]
fn native_rune_selects_damage_without_healing_and_converts_percent_units() {
    let rune = rune(
        "{{Rune data|Test|description=Deal {{as|{{rd|3.5%|1.4%}} maximum health}} magic damage, heal {{as|1.3% maximum health}}.}}",
    );
    let sample = Sample::new([(MaxHealth, 3000.)]);
    for (attack, expected) in [(Melee, 105.), (Ranged, 42.)] {
        assert!(
            (rune
                .native("description", attack, SourceForm::Base)
                .unwrap()
                .eval(&sample)
                .unwrap()
                - expected)
                .abs()
                < 1e-8
        );
    }
}
#[test]
fn native_rune_ambiguity_fails() {
    let rune = rune(
        "{{Rune data|Test|description=Deal {{pp|10 to 20}} magic damage, then deal {{pp|30 to 40}} magic damage.}}",
    );
    assert!(rune.native("description", Melee, SourceForm::Base).is_err());
}
#[test]
fn cached_declared_rune_damage_sources_are_supported() {
    use tutorlolv2_build_dep::generators::{
        parser::{Parser, runes::RuneParser},
        runes::rune_gen_fn,
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let parser = RuneParser::from_root(root).unwrap();
    let mut errors = vec![];
    for (id, data) in parser.data {
        if let Some(make) = rune_gen_fn(&id) {
            let mut generator = make(data).unwrap();
            if let Err(error) = generator.generate() {
                errors.push(format!("{id}: {error}"));
            }
        }
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn cached_rune_values_match_independent_source_expectations() {
    use tutorlolv2_types::CtxVar::*;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let sample = Sample::new([
        (Level, 18.),
        (BonusAd, 100.),
        (AbilityPower, 200.),
        (Stacks, 10.),
        (MaxHealth, 3000.),
        (BonusHealth, 1000.),
    ]);
    for (id, section, expected) in [
        ("Aftershock", "description2", 200.),
        ("ArcaneComet", "description", 120.),
        ("CheapShot", "description", 45.),
        ("DarkHarvest", "description", 160.),
        ("DeathfireTouch", "description", 12.),
        ("Electrocute", "description", 260.),
        ("GraspOfTheUndying", "description", 105.),
        ("HailOfBlades", "description2", 52.),
        ("LethalTempo", "description2", 30.),
        ("Predator", "description3", 235.),
        ("PressTheAttack", "description", 160.),
        ("Scorch", "description", 40.),
        ("SuddenImpact", "description", 80.),
        ("SummonAery", "description", 70.),
    ] {
        let source =
            std::fs::read_to_string(root.join("cache/wiki/runes").join(id).join("data.lua"))
                .unwrap();
        let formula = rune(&source)
            .native(section, Melee, SourceForm::Base)
            .unwrap();
        assert!(
            (formula.eval(&sample).unwrap() - expected).abs() < 0.01,
            "{id}: {:?}",
            formula.eval(&sample)
        );
    }
}

#[test]
fn cached_rune_map_changes_replace_named_terms() {
    use tutorlolv2_build_dep::generators::parser::{Parser, runes::RuneParser};
    use tutorlolv2_types::{CtxVar::*, MapMode};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let parser = RuneParser::from_root(root).unwrap();
    let sample = Sample::new([
        (Level, 18.),
        (BonusAd, 100.),
        (AbilityPower, 200.),
        (Stacks, 10.),
    ]);
    for (id, default, aram) in [("DarkHarvest", 160., 100.), ("Electrocute", 260., 275.)] {
        let rune = Rune::try_from(parser.data[id].clone()).unwrap();
        let formula = rune.native("description", Melee, SourceForm::Base).unwrap();
        assert_eq!(formula.eval(&sample), Some(default), "{id}");
        assert_eq!(
            formula.eval(&sample.clone().on(MapMode::Aram)),
            Some(aram),
            "{id}"
        );
    }
}

#[test]
fn cached_arena_rune_values_follow_raw_changes() {
    use tutorlolv2_build_dep::generators::parser::{Parser, runes::RuneParser};
    use tutorlolv2_types::{CtxVar::*, MapMode};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let parser = RuneParser::from_root(root).unwrap();
    let sample = Sample::new([
        (Level, 18.),
        (BonusAd, 100.),
        (AbilityPower, 200.),
        (Stacks, 10.),
    ])
    .on(MapMode::Arena);
    for (id, expected) in [
        ("DarkHarvest", 385.),
        ("Electrocute", 435.),
        ("PressTheAttack", 360.),
    ] {
        let rune = Rune::try_from(parser.data[id].clone()).unwrap();
        let formula = rune.native("description", Melee, SourceForm::Base).unwrap();
        assert_eq!(formula.eval(&sample), Some(expected), "{id}");
    }
}

#[test]
fn lethal_tempo_uses_source_attack_speed_amplification() {
    use tutorlolv2_build_dep::generators::runes::LethalTempo;
    use tutorlolv2_types::CtxVar;
    use tutorlolv2_wiki::wikitext::LegacyFormula;
    let source = "{{Rune data|Test|description=Gain {{as|{{rd|6%|4.8%}} bonus attack speed|as}} for each stack.|description2=Basic attacks at full stacks deal {{as|{{rd|30|20.01}}|adaptive damage}} bonus adaptive damage, increased by 2% per {{as|1% bonus attack speed}}.}}";
    let data = rune(source).data;
    let generated = LethalTempo::try_from(data).unwrap().call().unwrap();
    let sample = Sample::new([(CtxVar::BonusAttackSpeedRatio, 1.0)]);
    for (damage, expected) in [
        (&generated.melee.min_dmg, 90.0),
        (&generated.ranged.min_dmg, 60.03),
    ] {
        let actual = LegacyFormula::parse(damage).unwrap().eval(&sample);
        assert!((actual - expected).abs() < 1e-8, "{actual} != {expected}");
    }
}

#[test]
fn lethal_tempo_stack_speed_selects_per_stack_instead_of_maximum() {
    let rune = rune(
        "{{Rune data|Test|description=Gain {{as|{{rd|6%|{{ap|6*0.8}}%}} '''bonus''' attack speed|as}} for each stack, up to {{as|{{rd|36%|{{ap|36*0.8}}%}}|as}} at maximum stacks.}}",
    );
    for (attack, expected) in [(Melee, 6.), (Ranged, 4.8)] {
        let formula = rune.attack_speed_per_stack(attack).unwrap();
        assert!((formula.eval(&Sample::new([])).unwrap() - expected).abs() < 1e-6);
    }
}

#[test]
fn cached_lethal_tempo_stack_speed_follows_arena_note() {
    use tutorlolv2_build_dep::generators::parser::{Parser, runes::RuneParser};
    use tutorlolv2_types::{CtxVar::Level, MapMode};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let parser = RuneParser::from_root(root).unwrap();
    let rune = Rune::try_from(parser.data["LethalTempo"].clone()).unwrap();
    for (attack, normal, first, last) in [(Melee, 6., 13., 18.), (Ranged, 4.8, 8., 12.)] {
        let formula = rune.attack_speed_per_stack(attack).unwrap();
        assert_eq!(formula.eval(&Sample::new([(Level, 1.)])), Some(normal));
        assert_eq!(
            formula.eval(&Sample::new([(Level, 1.)]).on(MapMode::Arena)),
            Some(first)
        );
        assert_eq!(
            formula.eval(&Sample::new([(Level, 18.)]).on(MapMode::Arena)),
            Some(last)
        );
    }
}
