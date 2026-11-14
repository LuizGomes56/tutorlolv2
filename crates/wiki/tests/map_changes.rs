use types::{CtxVar, StatName};
use wiki::{
    map_changes::change::{Amount, ChangeKind, MapChange, Values},
    parser::Scaling,
};

fn kind(bullet: &str) -> ChangeKind {
    ChangeKind::parse(bullet, false)
}

fn amount(values: Values) -> Amount {
    Amount {
        values,
        ranged: None,
        percent: false,
    }
}

// Bullets of `Module:MapChanges/data/aram` and `/ar` in 16.22.1

#[test]
fn ratios_keep_the_text_before_ratio_and_their_scaling() {
    assert_eq!(
        kind("AP ratio changed to {{as|70% AP}}."),
        ChangeKind::Ratio {
            part: "AP".into(),
            scaling: Scaling::Simple {
                value: 0.7,
                ctx_var: CtxVar::AbilityPower,
            },
            ranged: None,
        }
    );

    assert_eq!(
        kind("AD ratio changed to {{as|{{ap|125 to 185}}% AD}}."),
        ChangeKind::Ratio {
            part: "AD".into(),
            scaling: Scaling::Ranked {
                values: vec![1.25, 1.4, 1.55, 1.7, 1.85],
                ctx_var: CtxVar::AttackDamage,
            },
            ranged: None,
        }
    );
}

#[test]
fn ratios_without_the_word_ratio_and_with_ranged_values() {
    let ChangeKind::Ratio {
        part,
        scaling,
        ranged,
    } = kind(
        "Mist's Edge damage changed to {{as|{{rd|12%|10%}} of target's '''current''' health}}.",
    )
    else {
        panic!("not a ratio");
    };

    assert_eq!(part, "Mist's Edge damage");
    assert_eq!(scaling.ctx_var(), Some(CtxVar::EnemyCurrentHealth));
    assert!(matches!(ranged, Some(Scaling::PercentAttr { value, .. }) if value == 0.1));
}

#[test]
fn base_damage_by_rank_and_by_level() {
    assert_eq!(
        kind("Base damage changed to {{ap|180 to 400 3}}."),
        ChangeKind::Base {
            part: "Base".into(),
            amount: amount(Values::ByRank(vec![180.0, 290.0, 400.0])),
        }
    );

    let ChangeKind::Base { part, amount } =
        kind("Bring It Down damage changed to {{pp|262.5 to 350 for 11|1;9 to 18}} for all users.")
    else {
        panic!("not a base damage");
    };

    assert_eq!(part, "Bring It Down");
    let Values::ByLevel { values, levels } = amount.values else {
        panic!("not by level");
    };
    assert_eq!((values[0], values[10]), (262.5, 350.0));
    assert_eq!(levels, [1, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
}

#[test]
fn values_for_ranged_champions_are_kept_apart() {
    assert_eq!(
        kind("Eminence base bonus attack damage changed to {{rd|15|10}}."),
        ChangeKind::Base {
            part: "Eminence base bonus attack".into(),
            amount: Amount {
                values: Values::Scalar(15.0),
                ranged: Some(Values::Scalar(10.0)),
                percent: false,
            },
        }
    );
}

#[test]
fn item_stats_are_only_read_on_items() {
    assert_eq!(
        ChangeKind::parse("Attack damage changed to 55.", true),
        ChangeKind::ItemStat {
            stat: StatName::AttackDamage,
            amount: amount(Values::Scalar(55.0)),
        }
    );
    assert!(!matches!(
        kind("Attack damage changed to 55."),
        ChangeKind::ItemStat { .. }
    ));
}

#[test]
fn base_stats_gained_by_level() {
    let ChangeKind::BaseStat { stat, amount } = kind(
        "{{sbc|New Effect:}} Now passively gains {{as|{{pp|0 to 187}} increased '''base''' health}}.",
    ) else {
        panic!("not a base stat");
    };

    assert_eq!(stat, StatName::Health);
    let Values::ByLevel { values, .. } = amount.values else {
        panic!("not by level");
    };
    assert_eq!((values.len(), values[0], values[17]), (18, 0.0, 187.0));
}

#[test]
fn modifiers_named_values_and_availability() {
    assert_eq!(
        kind("Damage reduction changed to 55%."),
        ChangeKind::Modifier {
            part: "Damage reduction".into(),
            amount: Amount {
                values: Values::Scalar(55.0),
                ranged: None,
                percent: true,
            },
        }
    );
    assert!(matches!(
        kind("Per-kill stacks changed to {{as|6|Siphoning Strike}}."),
        ChangeKind::Named { part, .. } if part == "Per-kill stacks"
    ));
    assert_eq!(kind("This item is disabled."), ChangeKind::Disabled);
    assert_eq!(
        kind("{{sbc|New Effect:}} Passively generates 1 stack every 5 seconds."),
        ChangeKind::NewEffect
    );
    assert_eq!(
        kind(
            "{{sbc|Removed:}} No longer generates {{as|Courage|buzzword}} while at the allied [[spawn]]."
        ),
        ChangeKind::Removed
    );
}

#[test]
fn changes_that_no_damage_depends_on_are_ignored() {
    for (bullet, keyword) in [
        ("Cooldown changed to {{ap|18 to 14}} seconds.", "cooldown"),
        ("Total cost changed to {{g|2950}}.", "cost"),
        (
            "Base heal ratio changed to {{as|{{fd|1.5}}% of '''maximum''' health}}.",
            "heal",
        ),
        ("Deals 80% damage to minions (60% total).", "minions"),
    ] {
        assert_eq!(
            kind(bullet),
            ChangeKind::Ignored {
                keyword: keyword.into()
            },
            "{bullet}"
        );
    }

    // "health" is not a heal
    assert!(!matches!(
        kind("Health ratio changed to {{as|20% of '''maximum''' health}}."),
        ChangeKind::Ignored { .. }
    ));
}

#[test]
fn lists_keep_notes_and_bugs() {
    let changes = MapChange::parse_list(
        "
    * Bonus attack speed for {{tip|ranged}} users changed to 80%.
    ** ''Attack speed for {{tip|melee}} users unchanged.''
    * Does not trigger {{iis|Mirage Blade|Mirage Blade}} Blur.{{bug}}
    ",
        false,
    );

    assert_eq!(changes.len(), 2);
    assert_eq!(
        changes[0].notes,
        ["''Attack speed for {{tip|melee}} users unchanged.''"]
    );
    assert!(!changes[0].has_bug && changes[1].has_bug);
}
