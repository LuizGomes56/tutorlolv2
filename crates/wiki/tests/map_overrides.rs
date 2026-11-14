use std::collections::BTreeMap;
use types::{CtxVar, GameMap, Key};
use wiki::{
    map_changes::{ChangeTarget, MapChanges, MapEntry, change::MapChange},
    parser::{Effect, Scaling},
};

#[test]
fn item_effects_read_the_ranged_values_for_ranged_champions() {
    use wiki::map_changes::overrides::EffectOverrides;

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
    use wiki::champions::{Stat, WikiMapStats, WikiStats};

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
