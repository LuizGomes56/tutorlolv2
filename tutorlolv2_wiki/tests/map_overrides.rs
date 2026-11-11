use std::collections::BTreeMap;
use tutorlolv2_types::{CtxVar, GameMap, Key};
use tutorlolv2_wiki::{
    champions::{WikiChampion, abilities::WikiAbility},
    map_changes::{
        ChangeTarget, MapChanges, MapEntry,
        change::MapChange,
        overrides::{ChampionOverrides, Outcome, SelectedEffect},
    },
    parser::{Effect, Scaling},
};

fn ap(value: f64) -> Scaling {
    Scaling::Simple {
        value,
        ctx_var: CtxVar::AbilityPower,
    }
}

/// An effect with its formula, as the parser leaves it
fn effect(base: &[f64], scalings: Vec<Scaling>) -> Effect {
    let mut effect = Effect {
        base: Some(base.to_vec()),
        scalings,
        ..Default::default()
    };
    effect.formula = effect.simplify_formula(Key::E).unwrap();
    effect
}

/// Kassadin's E of 16.22.1, with a second effect for the ambiguous cases
fn kassadin(other: Option<Effect>) -> WikiChampion {
    let mut effects = BTreeMap::from([(
        "Magic Damage".to_string(),
        effect(&[70.0, 100.0, 130.0, 160.0, 190.0], vec![ap(0.7)]),
    )]);

    if let Some(other) = other {
        effects.insert("Total Damage".into(), other);
    }

    WikiChampion {
        name: "Kassadin".into(),
        champion_id: "Kassadin".into(),
        adaptive_type: Default::default(),
        attack_type: Default::default(),
        positions: Default::default(),
        stats: Default::default(),
        modifiers: Default::default(),
        wiki_abilities: BTreeMap::from([(
            Key::E,
            vec![WikiAbility {
                skill: Key::E,
                effects,
                ..Default::default()
            }],
        )]),
        exceptions: Default::default(),
        map_stats: Default::default(),
    }
}

fn changes(bullets: &[&str]) -> MapChanges {
    MapChanges {
        maps: vec![GameMap::Aram, GameMap::AramMayhem],
        entries: BTreeMap::from([(
            "Kassadin E".to_string(),
            MapEntry {
                target: ChangeTarget::Ability {
                    champion_id: "Kassadin".into(),
                    key: Key::E,
                },
                changes: bullets.iter().map(|b| MapChange::parse(b, false)).collect(),
            },
        )]),
    }
}

fn selected(names: &[&str]) -> Vec<SelectedEffect> {
    names
        .iter()
        .map(|name| SelectedEffect {
            key: Key::E,
            nth: 0,
            name: name.to_string(),
        })
        .collect()
}

#[test]
fn changes_of_the_same_effect_add_up() {
    let overrides = ChampionOverrides::new(
        &kassadin(None),
        &changes(&[
            "Base damage changed to {{ap|80 to 220}}.",
            "AP ratio changed to {{as|75% AP}}.",
        ]),
        &selected(&["Magic Damage"]),
    );

    let applied = Outcome::Applied(selected(&["Magic Damage"]));
    assert_eq!(overrides.outcomes[0].2, applied);
    assert_eq!(overrides.outcomes[1].2, applied);

    let changed = &overrides.effects[&selected(&["Magic Damage"])[0]];
    assert_eq!(changed.base, Some(vec![80.0, 115.0, 150.0, 185.0, 220.0]));
    assert_eq!(changed.scalings, [ap(0.75)]);
    assert_eq!(
        changed.formula,
        effect(&[80.0, 115.0, 150.0, 185.0, 220.0], vec![ap(0.75)]).formula
    );
}

#[test]
fn only_selected_effects_are_candidates() {
    let champion = kassadin(Some(effect(&[1.0; 5], vec![ap(1.4)])));
    let change = changes(&["AP ratio changed to {{as|75% AP}}."]);

    // The second effect scales differently, but no generator selects it
    let unselected = ChampionOverrides::new(&champion, &change, &selected(&["Magic Damage"]));
    assert!(matches!(unselected.outcomes[0].2, Outcome::Applied(_)));

    let both = ChampionOverrides::new(
        &champion,
        &change,
        &selected(&["Magic Damage", "Total Damage"]),
    );
    assert_eq!(
        both.outcomes[0].2,
        Outcome::Manual("the selected effects scale with it differently")
    );
    assert!(both.effects.is_empty());
}

#[test]
fn effects_that_repeat_the_scaling_change_together() {
    let champion = kassadin(Some(effect(&[10.0; 5], vec![ap(0.7)])));
    let overrides = ChampionOverrides::new(
        &champion,
        &changes(&["AP ratio changed to {{as|75% AP}}."]),
        &selected(&["Magic Damage", "Total Damage"]),
    );

    assert_eq!(
        overrides.outcomes[0].2,
        Outcome::Applied(selected(&["Magic Damage", "Total Damage"]))
    );
}

#[test]
fn changes_that_name_a_part_or_repeat_the_value_are_not_overrides() {
    let champion = kassadin(None);
    let outcome = |bullet| {
        ChampionOverrides::new(&champion, &changes(&[bullet]), &selected(&["Magic Damage"]))
            .outcomes[0]
            .2
            .clone()
    };

    assert_eq!(
        outcome("First dash AP ratio changed to {{as|40% AP}}."),
        Outcome::Manual("it names a part of the ability")
    );
    assert_eq!(
        outcome("AP ratio changed to {{as|70% AP}}."),
        Outcome::AlreadyApplied("the scaling is the same")
    );
    assert_eq!(
        outcome("Cooldown changed to 10 seconds."),
        Outcome::NotRelevant
    );
}
