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

    let applied = Outcome::Applied {
        effects: vec![r#"E[0] "Magic Damage""#.into()],
        rule: "the only selected effect",
    };
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
fn the_closest_value_takes_the_change_and_its_multiples_follow() {
    // A total of two hits, so its ratio and base are twice the ones of a hit
    let champion = kassadin(Some(effect(
        &[140.0, 200.0, 260.0, 320.0, 380.0],
        vec![ap(1.4)],
    )));
    let both = selected(&["Magic Damage", "Total Damage"]);
    let overrides = ChampionOverrides::new(
        &champion,
        &changes(&["AP ratio changed to {{as|75% AP}}."]),
        &both,
    );

    assert_eq!(
        overrides.outcomes[0].2,
        Outcome::Applied {
            effects: vec![
                r#"E[0] "Magic Damage""#.into(),
                r#"E[0] "Total Damage""#.into()
            ],
            rule: "the closest value",
        }
    );
    assert_eq!(overrides.effects[&both[0]].scalings, [ap(0.75)]);
    assert_eq!(overrides.effects[&both[1]].scalings, [ap(1.5)]);
}

#[test]
fn effects_as_close_that_are_not_multiples_are_ambiguous() {
    // 70% and 80% are as close to 75%, and the bases show 80% is no multiple of 70%
    let champion = kassadin(Some(effect(&[1.0; 5], vec![ap(0.8)])));
    let overrides = ChampionOverrides::new(
        &champion,
        &changes(&["AP ratio changed to {{as|75% AP}}."]),
        &selected(&["Magic Damage", "Total Damage"]),
    );

    assert_eq!(
        overrides.outcomes[0].2,
        Outcome::Manual("the selected effects are ambiguous")
    );
    assert!(overrides.effects.is_empty());
}

#[test]
fn a_named_part_only_changes_the_effects_named_like_it() {
    // The total is a multiple of the hit, but the change names the total only
    let champion = kassadin(Some(effect(
        &[140.0, 200.0, 260.0, 320.0, 380.0],
        vec![ap(1.4)],
    )));
    let both = selected(&["Magic Damage", "Total Damage"]);
    let overrides = ChampionOverrides::new(
        &champion,
        &changes(&["Total AP ratio changed to {{as|150% AP}}."]),
        &both,
    );

    assert!(matches!(
        &overrides.outcomes[0].2,
        Outcome::Applied {
            rule: "the effects named like it",
            ..
        }
    ));
    assert_eq!(overrides.effects[&both[1]].scalings, [ap(1.5)]);
    assert!(!overrides.effects.contains_key(&both[0]));
}

#[test]
fn three_ultimate_ranks_fill_five_values() {
    let mut champion = kassadin(None);
    let e = champion.wiki_abilities.remove(&Key::E).unwrap();
    champion.wiki_abilities.insert(Key::R, e);

    let mut change = changes(&["Base damage changed to {{ap|100 to 300 3}}."]);
    let entry = change.entries.remove("Kassadin E").unwrap();
    change.entries.insert(
        "Kassadin R".into(),
        tutorlolv2_wiki::map_changes::MapEntry {
            target: ChangeTarget::Ability {
                champion_id: "Kassadin".into(),
                key: Key::R,
            },
            ..entry
        },
    );

    let selected = vec![SelectedEffect {
        key: Key::R,
        nth: 0,
        name: "Magic Damage".into(),
    }];
    let overrides = ChampionOverrides::new(&champion, &change, &selected);

    assert_eq!(
        overrides.effects[&selected[0]].base,
        Some(vec![100.0, 150.0, 200.0, 250.0, 300.0])
    );
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
        Outcome::Applied {
            effects: vec![
                r#"E[0] "Magic Damage""#.into(),
                r#"E[0] "Total Damage""#.into()
            ],
            rule: "the effects that share the value",
        }
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
        Outcome::Manual("it names a part that no selected effect is named after")
    );
    assert_eq!(
        outcome("AP ratio changed to {{as|70% AP}}."),
        Outcome::AlreadyApplied("the value is the same")
    );
    assert_eq!(
        outcome("Cooldown changed to 10 seconds."),
        Outcome::NotRelevant
    );
}

#[test]
fn values_far_from_the_change_keep_theirs() {
    // 90% is much farther from 75% than 70% is
    let champion = kassadin(Some(effect(&[1.0; 5], vec![ap(0.9)])));
    let both = selected(&["Magic Damage", "Total Damage"]);
    let overrides = ChampionOverrides::new(
        &champion,
        &changes(&["AP ratio changed to {{as|75% AP}}."]),
        &both,
    );

    assert_eq!(overrides.effects[&both[0]].scalings, [ap(0.75)]);
    assert!(!overrides.effects.contains_key(&both[1]));
}

#[test]
fn item_effects_read_the_ranged_values_for_ranged_champions() {
    use tutorlolv2_wiki::map_changes::overrides::EffectOverrides;

    let percent = |value| Scaling::PercentAttr {
        value,
        debug: "of target's current health".into(),
        ctx_var: CtxVar::EnemyCurrentHealth,
    };
    let target = ChangeTarget::Item {
        item_id: "BladeOfTheRuinedKing".into(),
    };
    let changes = MapChanges {
        maps: vec![GameMap::Aram, GameMap::AramMayhem],
        entries: BTreeMap::from([(
            "Blade of the Ruined King".to_string(),
            MapEntry {
                target: target.clone(),
                changes: vec![MapChange::parse(
                    "Mist's Edge damage changed to {{as|{{rd|12%|10%}} of target's '''current''' health}}.",
                    true,
                )],
            },
        )]),
    };

    let effect = |value| {
        let mut effect = Effect {
            scalings: vec![percent(value)],
            ..Default::default()
        };
        effect.formula = effect.simplify_formula(Key::P).unwrap();
        effect
    };

    for (ranged, old, new) in [(false, 0.09, 0.12), (true, 0.06, 0.1)] {
        let effects =
            BTreeMap::from([("pass".to_string(), ("Mist's Edge".to_string(), effect(old)))]);
        let overrides = EffectOverrides::new(&target, &changes, &effects, ranged);

        assert_eq!(overrides.effects["pass"].scalings, [percent(new)]);
    }
}

#[test]
fn arena_stats_replace_the_ones_the_template_changes() {
    use tutorlolv2_wiki::champions::{Stat, WikiMapStats, WikiStats};

    let stats = WikiStats {
        health: Stat {
            base: 600.0,
            per_level: 119.0,
        },
        ..Default::default()
    };
    let values = BTreeMap::from([("hp_lvl".to_string(), 121.0)]);

    let arena = WikiMapStats::new("ar", 1.0, &values, &stats).unwrap();
    assert_eq!(arena.maps, [GameMap::Arena]);
    assert_eq!(
        arena.stats.health,
        Stat {
            base: 600.0,
            per_level: 121.0
        }
    );

    // A mode that changes nothing has no stats
    assert!(WikiMapStats::new("aram", 1.0, &BTreeMap::new(), &stats).is_none());
}
