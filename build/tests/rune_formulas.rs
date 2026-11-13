use std::collections::BTreeMap;
use tutorlolv2_build_dep::{
    formula as f,
    generators::parser::{load_map_changes, runes::Rune},
};
use tutorlolv2_types::{
    AttackType::{Melee, Ranged},
    CtxVar::AbilityPower,
    DamageIndex::{Max, Min},
    GameMap, MapMode,
};
use tutorlolv2_wiki::{
    map_changes::{
        self, ChangeTarget, MapChanges, MapEntry,
        change::{ChangeKind, MapChange},
    },
    parser::{Effect, Scaling},
    runes::{RuneKeystone, RuneSlot, WikiRune},
    wikitext::{Formula, Sample},
};

fn rune() -> Rune {
    Rune::try_from(WikiRune {
        name: "Test".into(),
        rune_id: "StructuredRune".into(),
        path: RuneKeystone::Inspiration,
        slot: RuneSlot::Keystone,
        riot_id: 42,
        custom: false,
        descriptions: vec![],
        effects: BTreeMap::from([(
            "Damage".into(),
            Effect {
                base: Some(vec![30.0]),
                scalings: vec![Scaling::Simple {
                    value: 0.5,
                    ctx_var: AbilityPower,
                }],
                ..Default::default()
            },
        )]),
    })
    .unwrap()
}

#[test]
fn rune_queries_compose_trees_and_preserve_attack_type_map_overrides() {
    let root =
        std::env::temp_dir().join(format!("tutorlolv2-rune-formulas-{}", std::process::id()));
    let path = root.join(map_changes::full_path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let text = "Base damage changed to {{rd|50|40}}.";
    let changes = BTreeMap::from([(
        "aram",
        MapChanges {
            maps: vec![GameMap::Aram],
            entries: BTreeMap::from([(
                "Test".into(),
                MapEntry {
                    target: ChangeTarget::Rune {
                        rune_id: "StructuredRune".into(),
                    },
                    changes: vec![MapChange {
                        text: text.into(),
                        kind: ChangeKind::parse(text, false),
                        notes: vec![],
                        has_bug: false,
                    }],
                },
            )]),
        },
    )]);
    std::fs::write(&path, serde_json::to_vec(&changes).unwrap()).unwrap();
    load_map_changes(&root).unwrap();
    let mut rune = rune();
    let sample = Sample::new([(AbilityPower, 100.0)]);
    for (attack_type, expected) in [(Melee, 200.0), (Ranged, 180.0)] {
        let damage = rune.formula_for(0, attack_type).unwrap();
        assert!(matches!(damage, Formula::ByMap { .. }));
        let damage = f![damage * 2];
        assert!(matches!(damage, Formula::Product(_)));
        assert_eq!(damage.eval(&sample), Some(160.0));
        assert_eq!(
            damage.eval(&sample.clone().on(MapMode::Aram)),
            Some(expected)
        );
        rune.assign(attack_type, Min, &damage)
            .assign(attack_type, Max, f![damage + 1]);
    }
    rune.end().unwrap();
    assert_eq!(rune.riot_id, 42);
    for (slot, value) in [(0, "50"), (1, "50"), (2, "40"), (3, "40")] {
        assert_eq!(rune.map_damages[slot][0].0, MapMode::Aram);
        assert!(rune.map_damages[slot][0].1.contains(value));
    }
    assert!(rune.melee.max_dmg.contains("+ 1"));
    assert!(
        rune.scaling(0, 0..2).is_err(),
        "missing selected scalings must fail"
    );
    std::fs::remove_dir_all(root).unwrap();
}
