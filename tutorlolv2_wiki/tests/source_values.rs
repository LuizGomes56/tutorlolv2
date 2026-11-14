use tutorlolv2_types::CtxVar;
use tutorlolv2_wiki::wikitext::{
    Formula, Lookup, PageContext, Sample, SourceDocument, SourceForm, SourceRole, SourceVariant,
};

fn document(source: &str, ranged: bool) -> SourceDocument {
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 200.0,
    };
    SourceDocument::parse(
        "Passive",
        source,
        &context,
        SourceVariant {
            ranged,
            alternate: false,
        },
    )
    .unwrap()
}

#[test]
fn raw_damage_is_distinct_from_conditions_healing_and_stat_grants() {
    let source = "Against targets below {{as|50% of their maximum health}}, deals 30 {{as|(+ 11 per Soul)}} {{as|(+ 10% bonus AD)}} {{as|(+ 5% AP)}} bonus adaptive damage. Then heal for {{as|1.3% of your maximum health}} and grant {{as|5 bonus health}}.";
    let document = document(source, false);
    let damage = document
        .select(SourceRole::Damage, SourceForm::Base)
        .unwrap();
    let sample = Sample::new([
        (CtxVar::Stacks, 10.0),
        (CtxVar::BonusAd, 100.0),
        (CtxVar::AbilityPower, 200.0),
        (CtxVar::EnemyMaxHealth, 2000.0),
        (CtxVar::MaxHealth, 2000.0),
    ]);
    assert_eq!(damage.formula().unwrap().eval(&sample), Some(160.0));
    assert_eq!(
        document
            .select(SourceRole::Condition, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&sample),
        Some(1000.0)
    );
    assert!(
        (document
            .select(SourceRole::Healing, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&sample)
            .unwrap()
            - 26.0)
            .abs()
            < 1e-9
    );
}

#[test]
fn raw_rd_percent_and_maximum_damage_are_named_independently() {
    for (ranged, expected) in [(false, 70.0), (true, 28.0)] {
        let doc = document(
            "Deal bonus magic damage equal to {{as|{{rd|{{fd|3.5}}%|{{fd|1.4}}%}} of your maximum health}}, heal you for {{as|{{rd|1.3%|0.52%}} of your maximum health)}}.",
            ranged,
        );
        assert!(
            (doc.select(SourceRole::Damage, SourceForm::Base)
                .unwrap()
                .formula()
                .unwrap()
                .eval(&Sample::new([(CtxVar::MaxHealth, 2000.0)]))
                .unwrap()
                - expected)
                .abs()
                < 1e-9
        );
    }
    let doc = document(
        "Deals {{pp|15 to 100}} {{as|(+ 10% bonus AD)}} {{as|(+ 5% AP)}} damage, up to a maximum of {{pp|30 to 200}} {{as|(+ 20% bonus AD)}} {{as|(+ 10% AP)}} damage.",
        false,
    );
    let sample = Sample::new([
        (CtxVar::Level, 18.0),
        (CtxVar::BonusAd, 100.0),
        (CtxVar::AbilityPower, 200.0),
    ]);
    assert_eq!(
        doc.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&sample),
        Some(120.0)
    );
    assert_eq!(
        doc.select(SourceRole::Damage, SourceForm::Maximum)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&sample),
        Some(240.0)
    );
}

#[test]
fn named_selection_is_order_independent_and_rejects_ambiguity() {
    let a = document(
        "Deal {{as|15|magic damage}} damage. Heal {{as|20|health}}.",
        false,
    );
    let b = document(
        "Heal {{as|20|health}}. Deal {{as|15|magic damage}} damage.",
        false,
    );
    assert_eq!(
        a.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap(),
        b.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
    );
    let ambiguous = document(
        "Deal {{as|15|magic damage}} damage and deal {{as|20|magic damage}} damage.",
        false,
    );
    assert!(
        ambiguous
            .select(SourceRole::Damage, SourceForm::Base)
            .is_err()
    );
    assert!(a.select(SourceRole::Damage, SourceForm::Maximum).is_err());
}

#[test]
fn raw_catalog_preserves_effect_names_variants_and_shared_variables() {
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 200.0,
    };
    let module = r#"return { ["Example"] = { effects = {
        pass = {name="Burn", description="{{#vardefine:base|15}}Deal {{as|{{#var:base}}|magic damage}} damage."},
        pass2 = {name="Explosion", description="Deal {{as|{{#expr:{{#var:base}}*2}}|magic damage}} damage."}
    } } }"#;
    let effects =
        SourceDocument::item_effects(module, "Example", &context, SourceVariant::default())
            .unwrap();
    assert_eq!(
        effects["Burn"]
            .select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([])),
        Some(15.0)
    );
    assert_eq!(
        effects["Explosion"]
            .select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([])),
        Some(30.0)
    );
    let rune = "{{Rune data|Example|description=Deals {{rd|9 to 30|6 to 20|pp=true}} damage.|description2=Heal {{as|20|health}}.}}";
    let descriptions = SourceDocument::rune_descriptions(
        rune,
        &context,
        SourceVariant {
            ranged: true,
            alternate: false,
        },
    )
    .unwrap();
    assert_eq!(
        descriptions["description"]
            .select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([(CtxVar::Level, 18.0)])),
        Some(20.0)
    );
}

#[test]
fn stat_progressions_use_numeric_domains_without_integer_truncation() {
    let doc = document(
        "Deal {{as|{{pp|0 to 50|0 to 100|type=critical strike chance}}|physical damage}} damage.",
        false,
    );
    let value = doc.select(SourceRole::Damage, SourceForm::Base).unwrap();
    let formula = value.formula().unwrap();
    for (crit, expected) in [(-5.0, 0.0), (25.0, 12.5), (100.0, 50.0), (150.0, 50.0)] {
        assert!(
            (formula
                .eval(&Sample::new([(CtxVar::CritChance, crit)]))
                .unwrap()
                - expected)
                .abs()
                < 0.003
        );
    }
    assert_eq!(value.progressions[0].keys.last(), Some(&100.0));
    let doc = document(
        "Deal {{pp|0 to 100 by 5|0 to 750|key=%|type=distance the comet travelled}} damage.",
        false,
    );
    let value = doc.select(SourceRole::Damage, SourceForm::Base).unwrap();
    assert!(value.formula().is_err());
    assert_eq!(value.progressions[0].keys.last(), Some(&750.0));
}

#[test]
#[ignore = "requires downloaded raw wiki cache"]
fn cached_rune_sources_match_independent_damage_expectations() {
    use std::{fs, path::Path};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let lookup = Lookup::from_cache_at(root).unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 200.0,
    };
    for ranged in [false, true] {
        let sample = Sample::new([
            (CtxVar::Level, 18.0),
            (CtxVar::Stacks, 10.0),
            (CtxVar::BonusAd, 100.0),
            (CtxVar::AbilityPower, 200.0),
            (CtxVar::MaxHealth, 3000.0),
        ]);
        for (id, description, form, expected) in [
            ("ArcaneComet", "description", SourceForm::Base, 120.0),
            ("ArcaneComet", "description", SourceForm::Maximum, 240.0),
            ("DarkHarvest", "description", SourceForm::Base, 160.0),
            (
                "GraspOfTheUndying",
                "description",
                SourceForm::Base,
                if ranged { 42.0 } else { 105.0 },
            ),
            (
                "LethalTempo",
                "description2",
                SourceForm::Base,
                if ranged { 20.01 } else { 30.0 },
            ),
        ] {
            let raw =
                fs::read_to_string(root.join(format!("cache/wiki/runes/{id}/data.lua"))).unwrap();
            let docs = SourceDocument::rune_descriptions(
                &raw,
                &context,
                SourceVariant {
                    ranged,
                    alternate: false,
                },
            )
            .unwrap();
            let actual = docs[description]
                .select(SourceRole::Damage, form)
                .unwrap()
                .formula()
                .unwrap()
                .eval(&sample)
                .unwrap();
            assert!(
                (actual - expected).abs() <= expected.abs() * Formula::LINEAR_RELATIVE_TOLERANCE,
                "{id} ranged={ranged}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
#[ignore = "requires downloaded raw wiki cache"]
fn cached_item_sources_keep_named_effects_and_reject_distinct_damage_events() {
    use std::{fs, path::Path};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let lookup = Lookup::from_cache_at(root).unwrap();
    let context = PageContext {
        lookup: &lookup,
        four_rank_ultimate: false,
        ability_keys: Default::default(),
        crit_damage: 200.0,
    };
    let raw = fs::read_to_string(root.join("cache/wiki/items/data.lua")).unwrap();
    for hp in [1000.0, 3000.0, 5000.0] {
        let sample = Sample::new([
            (CtxVar::MaxHealth, hp),
            (CtxVar::BonusHealth, 1000.0),
            (CtxVar::EnemyMaxHealth, hp),
        ]);
        for (item, effect, role, alternate, expected) in [
            ("Bami's Cinder", "Immolate", SourceRole::Damage, false, 15.0),
            (
                "Liandry's Torment",
                "Torment",
                SourceRole::Damage,
                false,
                0.01 * hp,
            ),
            (
                "Liandry's Torment",
                "Torment",
                SourceRole::Damage,
                true,
                0.06 * hp,
            ),
            (
                "Heartsteel",
                "Colossal Consumption",
                SourceRole::Damage,
                false,
                70.0 + 0.06 * hp,
            ),
            (
                "Overlord's Bloodmail",
                "Tyranny",
                SourceRole::Stat,
                false,
                25.0,
            ),
        ] {
            let docs = SourceDocument::item_effects(
                &raw,
                item,
                &context,
                SourceVariant {
                    ranged: false,
                    alternate,
                },
            )
            .unwrap();
            let actual = docs[effect]
                .select(role, SourceForm::Base)
                .unwrap()
                .formula()
                .unwrap()
                .eval(&sample)
                .unwrap();
            assert!(
                (actual - expected).abs() < 1e-8,
                "{item}: {actual} vs {expected}"
            );
        }
    }
    for (item, effect) in [("Hollow Radiance", "Desolate"), ("Titanic Hydra", "Cleave")] {
        let docs =
            SourceDocument::item_effects(&raw, item, &context, SourceVariant::default()).unwrap();
        assert!(
            docs[effect]
                .select(SourceRole::Damage, SourceForm::Base)
                .is_err()
        );
        assert_eq!(
            docs[effect]
                .values
                .iter()
                .filter(|v| v.role == SourceRole::Damage)
                .count(),
            2
        );
    }
}

#[test]
fn nested_crit_progression_adds_flat_damage_and_retains_caps() {
    let doc = document(
        "Deal {{as|125% base AD}} {{as|(+ {{pp|0 to 50|0 to 100 for 11|key1=%|label1=critical strike chance}} based on critical strike chance)}} bonus physical damage.",
        false,
    );
    let formula = doc
        .select(SourceRole::Damage, SourceForm::Base)
        .unwrap()
        .formula()
        .unwrap();
    for (crit, expected) in [(0.0, 125.0), (25.0, 137.5), (150.0, 175.0)] {
        assert_eq!(
            formula.eval(&Sample::new([
                (CtxVar::BaseAd, 100.0),
                (CtxVar::CritChance, crit)
            ])),
            Some(expected)
        );
    }
}

#[test]
fn missing_health_steps_use_percentage_points_and_cap() {
    let doc = document(
        "Gain {{pp|0 to 10 by 1|0 to 70|key=%|key1=%|type=missing health}} attack damage.",
        false,
    );
    let formula = doc.values[0].formula().unwrap();
    for (missing, expected) in [
        (0.0, 0.0),
        (0.069, 0.0),
        (0.07, 1.0),
        (0.699, 9.0),
        (0.7, 10.0),
        (1.0, 10.0),
    ] {
        assert_eq!(
            formula.eval(&Sample::new([(CtxVar::MissingHealthRatio, missing)])),
            Some(expected)
        );
    }
}

#[test]
fn semantic_selection_requires_unique_target_context() {
    let doc = document(
        "To the primary target deal {{as|15|magic damage}} damage. To secondary targets deal {{as|20|magic damage}} damage.",
        false,
    );
    let value = doc
        .select_where(SourceRole::Damage, SourceForm::Base, |v| {
            v.surrounding.contains("primary target")
        })
        .unwrap();
    assert_eq!(value.formula().unwrap().eval(&Sample::new([])), Some(15.0));
    assert!(
        doc.select_where(SourceRole::Damage, SourceForm::Base, |_| true)
            .is_err()
    );
}

#[test]
fn shield_amount_uses_auxiliary_stacks_input() {
    let doc = document("Deal 0 {{as|(+ 15% shield amount)}} damage.", false);
    // Shield Bash's auxiliary input is the amount of the shield gained.
    assert_eq!(
        doc.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([(CtxVar::Stacks, 200.0)])),
        Some(30.0)
    );
}

#[test]
fn colored_nested_scalars_and_variable_templates_are_formula_values() {
    for source in [
        "Deal {{as|15 {{as|(+ 15% AP)}} bonus magic damage|magic damage}}.",
        "{{#vardefine:hit|15 {{as|(+ 15% AP)}} bonus magic damage}}Deal {{as|{{#var:hit}}|magic damage}}.",
    ] {
        let doc = document(source, false);
        assert_eq!(
            doc.select(SourceRole::Damage, SourceForm::Base)
                .unwrap()
                .formula()
                .unwrap()
                .eval(&Sample::new([(CtxVar::AbilityPower, 100.0)])),
            Some(30.0)
        );
    }
}

#[test]
fn trailing_damage_type_and_split_percentage_preserve_damage_semantics() {
    let doc = document(
        "Allies are healed for {{pp|100 to 200}}, while enemies take {{as|10% of target's maximum health}} as {{as|true damage}}.",
        false,
    );
    assert_eq!(
        doc.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([(CtxVar::EnemyMaxHealth, 2000.0)])),
        Some(200.0)
    );
    let doc = document(
        "Deal damage equal to {{as|3%|hp}} {{as|(+ 2% per 100 AP)}} {{as|of the target's maximum health}}.",
        false,
    );
    assert_eq!(
        doc.select(SourceRole::Damage, SourceForm::Base)
            .unwrap()
            .formula()
            .unwrap()
            .eval(&Sample::new([
                (CtxVar::AbilityPower, 100.0),
                (CtxVar::EnemyMaxHealth, 2000.0)
            ])),
        Some(100.0)
    );
}

#[test]
fn numeric_domains_keep_fractional_knots_and_source_units() {
    let doc = document(
        "Deal {{pp|5;15;20|0.25;0.75;1.5|type=critical strike chance|key1=%|round1=false}} damage.",
        false,
    );
    let value = doc.select(SourceRole::Damage, SourceForm::Base).unwrap();
    assert_eq!(value.progressions[0].keys, vec![0.25, 0.75, 1.5]);
    assert_eq!(value.progressions[0].key_unit, "%");
    for (axis, expected) in [(0.0, 5.0), (0.5, 10.0), (3.0, 20.0)] {
        assert_eq!(
            value
                .formula()
                .unwrap()
                .eval(&Sample::new([(CtxVar::CritChance, axis)])),
            Some(expected)
        );
    }
}

#[test]
fn table_axis_replacement_preserves_steps() {
    use tutorlolv2_wiki::wikitext::Formula;
    let table = Formula::Table {
        var: CtxVar::Stacks,
        keys: vec![1, 3, 5],
        values: vec![10.0, 20.0, 30.0],
    };
    assert_eq!(
        table
            .replace_var(CtxVar::Stacks, &Formula::Const(5.0))
            .eval(&Sample::new([])),
        Some(30.0)
    );
    assert_eq!(
        table
            .replace_var(CtxVar::Stacks, &Formula::Const(4.5))
            .eval(&Sample::new([])),
        Some(20.0)
    );
}

#[test]
fn raw_map_base_values_accept_scalar_and_level_progression() {
    use tutorlolv2_types::Key;
    use tutorlolv2_wiki::{map_changes::change::MapChange, wikitext::MapValue};
    let lookup = Lookup::new("return {}", "return {}").unwrap();
    for (source, level, expected) in [
        ("Base damage changed to 45.", 1.0, 45.0),
        ("Base damage changed to {{pp|10 to 180}}.", 18.0, 180.0),
    ] {
        let change = MapChange::parse(source, false);
        let MapValue::Base(formula) = MapValue::new(&change, Key::P, &lookup).unwrap() else {
            panic!("expected base")
        };
        assert_eq!(
            formula.eval(&Sample::new([(CtxVar::Level, level)])),
            Some(expected)
        );
    }
}

#[test]
fn target_missing_health_numeric_domain_uses_target_percentage() {
    let doc = document(
        "Damage is increased by {{pp|0 to 75 by 5|0 to 100|key=%|key1=%|type=target's missing health}}.",
        false,
    );
    let value = doc.values.first().unwrap();
    assert_eq!(
        value.formula().unwrap().eval(&Sample::new([
            (CtxVar::EnemyMissingHealthFlat, 1000.0),
            (CtxVar::EnemyMaxHealth, 2000.0)
        ])),
        Some(35.0)
    );
}
