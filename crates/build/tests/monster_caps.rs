//! Caps against monsters read from descriptions and effect labels
use build::generators::parser::monster_caps::{
    CapBound, CapMonsters, CapPart, CapSource, MonsterCap,
};
use std::collections::BTreeMap;
use types::{CtxVar, DamageType, Key};
use wiki::{
    items::item_parser::{ItemEffect, ItemEffects},
    wikitext::{AbilityModel, ChampionModel, EffectModel, Formula, Sample},
};

fn effect(label: &str, formula: Formula) -> EffectModel {
    EffectModel {
        label: label.into(),
        section: 0,
        unit: String::new(),
        formula,
        issues: Vec::new(),
    }
}

fn caps(text: &str, effects: Vec<EffectModel>, extras: Vec<EffectModel>) -> Vec<MonsterCap> {
    let page = AbilityModel {
        page: "P0".into(),
        name: "Ability".into(),
        damage_type: DamageType::Magic,
        attributes: Default::default(),
        descriptions: BTreeMap::from([("Innate".into(), text.into())]),
        effects,
        extras,
    };

    MonsterCap::extract(&ChampionModel {
        champion_id: "Test".into(),
        abilities: BTreeMap::from([(Key::P, vec![page])]),
        pets: BTreeMap::new(),
    })
}

fn at_level(cap: &MonsterCap, level: f64) -> f64 {
    let formula = cap.formula.as_ref().expect("the cap has a value");
    formula
        .eval(&Sample::new([(CtxVar::Level, level)]))
        .unwrap()
}

#[test]
fn a_cap_after_its_value_names_the_monsters_and_the_part() {
    let caps = caps(
        "The damage based on the target's health ratio is capped at 300 against epic monsters.",
        vec![],
        vec![],
    );

    let [cap] = &caps[..] else { panic!("{caps:?}") };
    assert_eq!(cap.bound, CapBound::Max);
    assert_eq!(cap.part, CapPart::HealthRatio);
    assert_eq!(cap.monsters, CapMonsters::Epic);
    assert_eq!(cap.source, CapSource::Description("Innate".into()));
    assert_eq!(cap.formula, Some(Formula::Const(300.0)));
}

#[test]
fn a_range_takes_the_description_table_with_its_numbers() {
    let table = Formula::Table {
        var: CtxVar::Level,
        keys: vec![1, 6, 11],
        values: vec![200.0, 400.0, 600.0],
    };
    let caps = caps(
        "Deals damage every second, capped at 200 to 600 for 3 per second against monsters.",
        vec![],
        vec![
            effect("Other", Formula::Const(1.0)),
            effect("Innate", table.clone()),
        ],
    );

    let [cap] = &caps[..] else { panic!("{caps:?}") };
    assert_eq!(cap.part, CapPart::PerTick);
    assert_eq!(cap.formula, Some(table));
}

#[test]
fn level_expressions_and_scalings_are_parsed() {
    let caps = caps(
        "Deals bonus magic damage, capped at 150 + (300-150)/17*(x-1) for 20 against monsters. \
         Its shock is capped at 15 (+ 100% bonus AD) against monster.",
        vec![],
        vec![],
    );

    let [level, scaling] = &caps[..] else {
        panic!("{caps:?}")
    };
    assert_eq!(level.part, CapPart::Bonus);
    assert!((at_level(level, 1.0) - 150.0).abs() < 1e-9);
    assert!((at_level(level, 18.0) - 300.0).abs() < 1e-9);

    let sample = Sample::new([(CtxVar::BonusAd, 50.0)]);
    let value = scaling.formula.as_ref().unwrap().eval(&sample).unwrap();
    assert!((value - 65.0).abs() < 1e-9);
}

#[test]
fn monsters_named_before_the_cap_and_shared_minimums_count() {
    let caps = caps(
        "Against monsters, the damage is capped at 40 to 200 per second. \
         The damage has a minimum threshold of 15 and is capped at 150 against minions and monsters.",
        vec![],
        vec![],
    );

    let parts: Vec<_> = caps
        .iter()
        .map(|cap| (cap.bound, cap.part, cap.text.as_str()))
        .collect();
    assert_eq!(
        parts,
        [
            (CapBound::Max, CapPart::PerTick, "capped at 40 to 200"),
            (CapBound::Min, CapPart::Damage, "minimum threshold of 15"),
            (CapBound::Max, CapPart::Damage, "capped at 150"),
        ]
    );
    // No table holds 40 and 200, and a range is not one number
    assert_eq!(caps[0].formula, None);
}

#[test]
fn caps_for_others_or_of_stacks_are_not_monster_caps() {
    let caps = caps(
        "It has a minimum of 40 against minions and is not capped. \
         This effect stacks, but is capped at 8 stacks against epic monsters. \
         Up to a maximum of 50 damage.Hemorrhage deals 250% damage against monsters.",
        vec![],
        vec![],
    );

    assert!(caps.is_empty(), "{caps:?}");
}

#[test]
fn effects_labelled_as_caps_are_kept_with_their_formula() {
    let caps = caps(
        "",
        vec![
            effect("Capped Monster Damage", Formula::Const(400.0)),
            effect(
                "Minimum Non-Epic Monster Damage per Hit",
                Formula::Const(10.0),
            ),
            // Damage against monsters, not a cap
            effect("Monster Damage", Formula::Const(1.0)),
        ],
        vec![],
    );

    let found: Vec<_> = caps
        .iter()
        .map(|cap| (cap.source.clone(), cap.bound, cap.part, cap.monsters))
        .collect();
    assert_eq!(
        found,
        [
            (
                CapSource::Effect,
                CapBound::Max,
                CapPart::Damage,
                CapMonsters::All
            ),
            (
                CapSource::Effect,
                CapBound::Min,
                CapPart::PerHit,
                CapMonsters::NonEpic
            ),
        ]
    );
}

#[test]
fn a_cap_continues_with_the_value_for_other_monsters() {
    let stacks = |values: Vec<f64>| Formula::Table {
        var: CtxVar::Stacks,
        keys: vec![1, 2, 3],
        values,
    };
    let caps = caps(
        "This damage is capped at50/4 to (50/4)*3 per tick against non-epic monsters and \
         80/4 to (80/4)*3 per tick against epic monsters. It deals 40% against minions and \
         50% against monsters.",
        vec![],
        vec![
            effect("Innate", stacks(vec![12.5, 25.0, 37.5])),
            effect("Innate", stacks(vec![20.0, 40.0, 60.0])),
        ],
    );

    let found: Vec<_> = caps
        .iter()
        .map(|cap| (cap.bound, cap.part, cap.monsters, cap.formula.clone()))
        .collect();
    assert_eq!(
        found,
        [
            (
                CapBound::Max,
                CapPart::PerTick,
                CapMonsters::NonEpic,
                Some(stacks(vec![12.5, 25.0, 37.5]))
            ),
            (
                CapBound::Max,
                CapPart::PerTick,
                CapMonsters::Epic,
                Some(stacks(vec![20.0, 40.0, 60.0]))
            ),
        ]
    );
}

#[test]
fn items_read_their_caps_from_the_wikitext_of_each_effect() {
    let effect = |name: &str, raw: &str| {
        Some(ItemEffect {
            name: Some(name.into()),
            raw_description: Some(raw.into()),
            ..Default::default()
        })
    };
    let caps = MonsterCap::from_item(&ItemEffects {
        pass: effect(
            "Torment",
            "Burns enemies{{ft|, capped at 20 per tick against monsters.|, capped at 120 \
             against {{tip|monsters}}.}}",
        ),
        pass2: effect(
            "Mist's Edge",
            "Deals damage, with a '''maximum''' of 100 against {{tip|minions}} and {{tip|monsters}}.",
        ),
        ..Default::default()
    });

    let found: Vec<_> = caps
        .iter()
        .map(|cap| (cap.key, cap.ability.as_str(), cap.part, cap.formula.clone()))
        .collect();
    assert_eq!(
        found,
        [
            (
                None,
                "Torment",
                CapPart::PerTick,
                Some(Formula::Const(20.0))
            ),
            (
                None,
                "Torment",
                CapPart::Damage,
                Some(Formula::Const(120.0))
            ),
            (
                None,
                "Mist's Edge",
                CapPart::Damage,
                Some(Formula::Const(100.0))
            ),
        ]
    );
}
