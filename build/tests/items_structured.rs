use tutorlolv2_build_dep::{
    formula as f,
    generators::parser::items::{Item, Source},
};
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex, MapMode};
use tutorlolv2_wiki::{
    items::item_parser::{ItemEffect, WikiItem},
    wikitext::{Formula, Sample},
};

#[test]
fn composed_item_damage_keeps_map_branches_until_assignment() {
    let mut item = Item::try_from(WikiItem::default()).unwrap();
    let damage = Formula::Const(20.0).with_map(MapMode::Arena, Formula::Const(30.0));
    let attack_damage = CtxVar::AttackDamage;
    let combined = f![damage + 0.5 * attack_damage];
    let sample = Sample::new([(CtxVar::AttackDamage, 100.0)]);
    assert_eq!(combined.eval(&sample), Some(70.0));
    assert_eq!(combined.eval(&sample.on(MapMode::Arena)), Some(80.0));
    item.set_min(combined.clone());
    assert_eq!(item.melee.min_dmg, combined.on(MapMode::Default).rust());
    assert_eq!(item.ranged.min_dmg, item.melee.min_dmg);
    assert_eq!(
        item.map_damages[0],
        vec![(MapMode::Arena, combined.on(MapMode::Arena).rust())]
    );
    assert_eq!(item.map_damages[2], item.map_damages[0]);
}

#[test]
fn item_queries_keep_distinct_melee_and_ranged_effects() {
    let mut data = WikiItem::default();
    data.effects.pass = Some(ItemEffect {
        name: Some("Fixture".into()),
        raw_description: Some(
            "Deal {{as|{{rd|100|60}}|magic damage}} damage and heal {{as|20|health}} health."
                .into(),
        ),
        ..ItemEffect::default()
    });
    let mut item = Item::try_from(data).unwrap();
    for attack_type in [AttackType::Melee, AttackType::Ranged] {
        let damage = item.formula_for(Source::Passive, attack_type).unwrap();
        item.assign(attack_type, DamageIndex::Min, f![damage * 2]);
    }
    assert_eq!(item.melee.min_dmg, f![100 * 2].rust());
    assert_eq!(item.ranged.min_dmg, f![60 * 2].rust());
}

#[test]
fn native_damage_rejects_ambiguous_events() {
    let mut data = WikiItem::default();
    data.effects.pass = Some(ItemEffect {
        name: Some("Fixture".into()),
        raw_description: Some(
            "Deal {{as|10|magic damage}} damage. Deal {{as|20|magic damage}} damage.".into(),
        ),
        ..Default::default()
    });
    let item = Item::try_from(data).unwrap();
    assert!(item.formula(Source::Passive).is_err());
}

fn native_fixture(source: &str) -> Item {
    let mut data = WikiItem::default();
    data.effects.pass = Some(ItemEffect {
        name: Some("Fixture".into()),
        raw_description: Some(source.into()),
        ..Default::default()
    });
    Item::try_from(data).unwrap()
}

#[test]
fn named_target_selection_ignores_the_secondary_damage_event() {
    use tutorlolv2_build_dep::generators::parser::items::ItemEvent;
    let item = native_fixture(
        "Basic attacks deal {{as|1% maximum health|hp}} {{as|bonus physical damage}} to the target and {{as|9% maximum health|hp}} {{as|physical damage}} to other enemies.",
    );
    let damage = item
        .formula(Source::Event("Fixture", ItemEvent::PrimaryTarget))
        .unwrap();
    assert_eq!(
        damage.eval(&Sample::new([(CtxVar::MaxHealth, 2500.0)])),
        Some(25.0)
    );
}

#[test]
fn native_effects_share_source_variables_across_slots() {
    let mut item =
        native_fixture("Deal {{as|{{#vardefineecho:base|15}} magic damage|magic damage}}.");
    item.data.effects.pass2 = Some(ItemEffect {
        name: Some("Eruption".into()),
        raw_description: Some(
            "Deal {{as|{{ap|{{#var:base}}*2}} magic damage|magic damage}}.".into(),
        ),
        ..Default::default()
    });
    let damage = item.formula(Source::Named("Eruption")).unwrap();
    assert_eq!(damage.eval(&Sample::default()), Some(30.0));
}

#[test]
fn maximum_presentation_uses_the_source_progression_cap() {
    let mut item =
        native_fixture("Deal {{as|{{pp|0 to 43|0 to 100|type=Momentum}}|magic damage}}.");
    item.maximum_damage(Source::Named("Fixture")).unwrap();
    assert_eq!(item.melee.min_dmg, Formula::Const(43.0).rust());
}

#[test]
fn cached_map_values_keep_source_damage_units_and_stats() {
    use tutorlolv2_build_dep::generators::parser::{Parser, items::ItemParser};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let parser = ItemParser::from_root(root).unwrap();
    let sample = Sample::new([
        (CtxVar::AbilityPower, 200.0),
        (CtxVar::MaxHealth, 3000.0),
        (CtxVar::BonusHealth, 1000.0),
        (CtxVar::EnemyMaxHealth, 2000.0),
        (CtxVar::Level, 18.0),
    ]);
    for (id, effect, alternate, form, mode, expected) in [
        (
            "BlackfireTorch",
            "Baleful Blaze",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Arena,
            14.0,
        ),
        (
            "BlackfireTorch",
            "Baleful Blaze",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Total,
            MapMode::Arena,
            84.0,
        ),
        (
            "Malignance",
            "Hatefog",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Arena,
            31.25,
        ),
        (
            "Malignance",
            "Hatefog",
            true,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Arena,
            375.0,
        ),
        (
            "Malignance",
            "Hatefog",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Aram,
            45.6,
        ),
        (
            "Malignance",
            "Hatefog",
            true,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Aram,
            547.2,
        ),
        (
            "LiandrysTorment",
            "Torment",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Aram,
            18.0,
        ),
        (
            "LiandrysTorment",
            "Torment",
            true,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Aram,
            108.0,
        ),
        (
            "Heartsteel",
            "Colossal Consumption",
            false,
            tutorlolv2_wiki::wikitext::SourceForm::Base,
            MapMode::Arena,
            245.0,
        ),
    ] {
        let item = Item::try_from(parser.data[id].clone()).unwrap();
        let source = if id == "Malignance" {
            Source::Event(
                effect,
                tutorlolv2_build_dep::generators::parser::items::ItemEvent::FogDamage,
            )
        } else {
            Source::Named(effect)
        };
        let formula = item
            .native_formula(source, AttackType::Melee, alternate, form)
            .unwrap();
        let actual = formula.eval(&sample.clone().on(mode)).unwrap();
        assert!(
            (actual - expected).abs() < 1e-8,
            "{id} {mode:?}: {actual} != {expected}"
        );
    }
    let echo = Item::try_from(parser.data["LudensEcho"].clone())
        .unwrap()
        .echo_total()
        .unwrap();
    assert_eq!(echo.eval(&sample.clone().on(MapMode::Arena)), Some(247.5));
    let kraken = Item::try_from(parser.data["KrakenSlayer"].clone()).unwrap();
    let amplification = kraken
        .maximum_amplification_formula(Source::Named("Bring It Down"))
        .unwrap();
    for attack in [AttackType::Melee, AttackType::Ranged] {
        let base = kraken
            .formula_for(
                Source::Event(
                    "Bring It Down",
                    tutorlolv2_build_dep::generators::parser::items::ItemEvent::BaseProc,
                ),
                attack,
            )
            .unwrap();
        let actual = base.eval(&sample.clone().on(MapMode::Aram)).unwrap();
        assert_eq!(actual, 350.0);
        assert_eq!(
            actual
                * (1.0
                    + amplification
                        .eval(&sample.clone().on(MapMode::Aram))
                        .unwrap()),
            612.5
        );
        let expected_base = if attack == AttackType::Melee {
            200.0
        } else {
            160.0
        };
        let arena_base = base.eval(&sample.clone().on(MapMode::Arena)).unwrap();
        assert_eq!(arena_base, expected_base);
        assert_eq!(
            arena_base
                * (1.0
                    + amplification
                        .eval(&sample.clone().on(MapMode::Arena))
                        .unwrap()),
            expected_base * 1.75
        );
    }
    let mut count = 0;
    for id in parser.data.keys() {
        if tutorlolv2_build_dep::generators::items::item_gen_fn(id).is_some() {
            parser
                .run_fn(id)
                .unwrap_or_else(|error| panic!("{id}: {error}"));
            count += 1;
        }
    }
    assert_eq!(count, 101);
}

#[test]
fn native_named_effects_reject_duplicate_slots() {
    let mut item = native_fixture("Deal {{as|10|magic damage}}.");
    item.data.effects.pass2 = item.data.effects.pass.clone();
    assert!(
        item.description(Source::Named("Fixture"))
            .unwrap_err()
            .to_string()
            .contains("found 2")
    );
    assert!(
        item.formula(Source::Named("Fixture"))
            .unwrap_err()
            .to_string()
            .contains("found 2")
    );
}

#[test]
fn timed_damage_requires_an_unambiguous_source_clock() {
    let mut item = native_fixture("Deal {{as|10|magic damage}} every 0.5 seconds.");
    item.data.effects.pass.as_mut().unwrap().name = Some("Baleful Blaze".into());
    assert!(
        item.formula(Source::Named("Baleful Blaze"))
            .unwrap_err()
            .to_string()
            .contains("interval and duration")
    );
}
