use tutorlolv2_types::CtxVar;
use tutorlolv2_wiki::parser::*;

#[test]
fn level_scalings_bind_to_the_stat_inside_their_group() {
    let raw = r#"deals <span class="pp-tooltip" data-bot-values="68;72;77" data-top-values="1;2;3">68 - 77 (based on level)</span> (+ 60% bonus AD) (+ <span class="pp-tooltip" data-bot-values="70;80;90;100" data-top-values="1;6;11;16" data-bot-key="%">70% / 80% / 90% / 100% (based on level)</span> AP) magic damage"#;
    let scalings = parse_scalings(raw);

    let levels = scalings
        .iter()
        .filter_map(|scaling| match scaling {
            Scaling::BasedOnLevel { arms, ctx_var, .. } => Some((arms, ctx_var)),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(levels.len(), 1, "{scalings:?}");
    assert_eq!(*levels[0].1, CtxVar::AbilityPower);
    assert_eq!(
        levels[0].0.iter().map(LevelArm::value).collect::<Vec<_>>(),
        [0.7, 0.8, 0.9, 1.0]
    );
    assert!(scalings.iter().any(|s| matches!(
        s,
        Scaling::Simple {
            value: 0.6,
            ctx_var: CtxVar::BonusAd
        }
    )));

    let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();
    assert!(
        effects
            .iter()
            .flat_map(|(_, e)| &e.scalings)
            .any(|s| matches!(
                s,
                Scaling::BasedOnLevel {
                    ctx_var: CtxVar::AbilityPower,
                    ..
                }
            )),
        "{effects:?}"
    );
}

#[test]
fn level_scalings_are_not_duplicated_by_other_scaling_blocks() {
    let raw = r#"gains <span class="pp-tooltip" data-bot-values="3;6;9" data-top-values="1;6;11" data-bot-key="%">3% / 6% / 9% (based on level)</span> bonus movement speed and deals 5 (+ 30% AP) magic damage"#;
    let scalings = parse_scalings(raw);

    assert_eq!(
        scalings
            .iter()
            .filter(|s| matches!(s, Scaling::BasedOnLevel { .. }))
            .count(),
        1,
        "{scalings:?}"
    );
}

#[test]
fn monster_caps_are_not_used_as_passive_base_damage() {
    let raw = r#"deals 1% (+ 2.7% per 100 AP) of the target's maximum health, capped at <span class="pp-tooltip" data-useformula="90+10*x" data-bot-values="100;110;120">100 - 120 (based on level)</span> against monsters"#;
    let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();

    assert!(effects[0].1.use_formula.is_none(), "{effects:?}");
    assert!(effects[0].1.use_values.is_none(), "{effects:?}");
    assert!(
        effects[0]
            .1
            .scalings
            .iter()
            .any(|s| matches!(s, Scaling::Nested { .. }))
    );
}

#[test]
fn movement_speed_tooltips_are_not_used_as_passive_base_damage() {
    let raw = r#"gains <span class="pp-tooltip" data-bot-values="3;6;9" data-top-values="1;6;11" data-bot-key="%">3% / 6% / 9% (based on level)</span> bonus movement speed and deals 5 (+ 30% AP) magic damage"#;
    let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();

    assert!(effects[0].1.use_values.is_none(), "{effects:?}");
    assert_eq!(effects[0].1.base, Some(vec![5.0]));
}

#[test]
fn per_100_scalings_increase_the_ratio_they_belong_to() {
    let raw = "7 / 7.5 / 8 / 8.5 / 9% <span>(+ 2.5% per 100 <b>bonus</b> AD)</span> of target's <b>maximum</b> health";
    let scalings = parse_scalings(raw);

    assert_eq!(scalings.len(), 1, "{scalings:?}");
    let Scaling::Nested { outer, inner, .. } = &scalings[0] else {
        panic!("expected a nested scaling: {scalings:?}");
    };

    assert!(matches!(
        outer.as_ref(),
        Scaling::Ranked {
            ctx_var: CtxVar::EnemyMaxHealth,
            ..
        }
    ));
    assert!(matches!(
        inner.as_slice(),
        [Scaling::Per100 {
            ctx_var: CtxVar::BonusAd,
            ..
        }]
    ));
}

#[test]
fn level_ratios_use_the_tooltip_values_per_level() {
    let raw = r#"equal to <span class="pp-tooltip" data-bot-values="6;9;12" data-top-values="1;7;13" data-bot-key="%">6% – 12% (based on level)</span> <span>(+ 2% per 100 AP)</span> of their maximum health"#;
    let scalings = parse_scalings(raw);

    let [Scaling::Nested { outer, inner, .. }] = scalings.as_slice() else {
        panic!("expected a single nested scaling: {scalings:?}");
    };

    let Scaling::BasedOnLevel { arms, ctx_var, .. } = outer.as_ref() else {
        panic!("expected a level ratio: {outer:?}");
    };

    assert_eq!(*ctx_var, CtxVar::EnemyMaxHealth);
    assert_eq!(
        arms.iter().map(LevelArm::value).collect::<Vec<_>>(),
        [0.06, 0.09, 0.12]
    );
    assert!(matches!(
        inner.as_slice(),
        [Scaling::Per100 {
            ctx_var: CtxVar::AbilityPower,
            ..
        }]
    ));
}

#[test]
fn ratios_inside_groups_are_left_to_the_group() {
    let text = "50 / 80 (+ 8% (+ 3% per 100 AP) of target's missing health)";
    assert!(nested_ratio_scalings(text).is_empty());
}

#[test]
fn alternative_representations_are_removed() {
    let html = r#"a<span class="flipText1 active">「 x <span>1</span> 」</span><span class="flipText2">「 y <span>2</span> 」</span>b"#;
    let cleaned = remove_elements_with_class(html, "flipText2");

    assert!(cleaned.contains("x <span>1</span>"));
    assert!(!cleaned.contains('y'));
    assert!(cleaned.ends_with('b'));
}

#[test]
fn variables_come_from_the_text_after_the_value() {
    let text = "deal 60% / 70% / 80% (based on level) (+ 0% - 80% (based on critical strike chance)) AD bonus physical damage";
    let following = following_text(text, "60% / 70% / 80% (based on level)");

    assert_eq!(leading_ctx_var(&following), CtxVar::AttackDamage);
}

#[test]
fn unknown_variables_are_explicit() {
    assert_eq!(assign_ctx_var("of target's missing mana"), CtxVar::Unknown);
    assert_eq!(
        assign_ctx_var("bonus attack speed"),
        CtxVar::BonusAttackSpeed
    );
    assert_eq!(assign_ctx_var("the marked enemy"), CtxVar::Unknown);
    assert_eq!(assign_ctx_var("150% base AD"), CtxVar::BaseAd);
}

#[test]
fn values_per_100_without_percent_are_per_100() {
    assert_eq!(
        Scaling::per_ctx("0.3 per 100 bonus health"),
        Some(Scaling::Per100 {
            value: 0.3,
            ctx_var: CtxVar::BonusHealth,
        })
    );
    assert_eq!(
        Scaling::per_ctx("25 per 1 lethality"),
        Some(Scaling::Simple {
            value: 25.0,
            ctx_var: CtxVar::ArmorPenetrationFlat,
        })
    );
}
