use std::collections::BTreeMap;
use types::Key;
use wiki::{items::item_parser::*, parser::Effect};

fn effect(description: &str, variant: Variant) -> (Effect, Option<Effect>) {
    analyze_effect(description, variant, &BTreeMap::new())
}

fn formula(effect: &Effect) -> String {
    effect.simplify_formula(Key::P).unwrap().unwrap_or_default()
}

const RANGED: Variant = Variant {
    ranged: true,
    flip: false,
};

const FLIP: Variant = Variant {
    ranged: false,
    flip: true,
};

#[test]
fn evaluates_expressions() {
    assert_eq!(eval_expr("60/6"), Some(10.0));
    assert_eq!(eval_expr("(60/6)+10"), Some(20.0));
    assert_eq!(eval_expr("150 + 5*(2-1)"), Some(155.0));
    assert_eq!(eval_expr("2^3 - -1"), Some(9.0));
    assert_eq!(eval_expr("10 *"), None);
}

#[test]
fn resolves_level_progressions() {
    let mut series = Vec::new();
    let named = BTreeMap::new();

    assert_eq!(resolve_pp(&["50 to 150".into()], &named, &mut series), "50");
    assert_eq!(series[0].values.len(), MAX_LEVEL);
    assert_eq!(series[0].values[MAX_LEVEL - 1], 150.0);

    // Values of other axes are replaced by their last value
    let named = BTreeMap::from([("type".into(), "Momentum".into())]);
    assert_eq!(
        resolve_pp(&["0 to 40 for 11".into()], &named, &mut series),
        "40"
    );
    assert_eq!(series.len(), 1);
}

#[test]
fn resolves_ranged_and_level_values() {
    let description = "deals {{as|{{rd|20 to 40|10 to 20|pp=true}}|true damage}} \
        {{as|(+ {{rd|25%|20%}} '''bonus''' armor)|armor}} {{as|'''bonus''' true damage}}.";

    let (melee, _) = effect(description, Variant::default());
    let (ranged, _) = effect(description, RANGED);

    assert_eq!(melee.use_values.as_ref().unwrap()[0], 20.0);
    assert_eq!(ranged.use_values.as_ref().unwrap()[MAX_LEVEL - 1], 20.0);
    assert!(formula(&melee).contains("(0.25) * ctx.bonus_armor"));
    assert!(formula(&ranged).contains("(0.2) * ctx.bonus_armor"));
}

#[test]
fn splits_flipped_text_and_totals() {
    let description = "take{{ft|{{as|1% of the target's '''maximum''' health}} every \
        {{fd|0.5}} seconds|{{as|6% of the target's '''maximum''' health}} total}}.";

    assert_eq!(
        formula(&effect(description, Variant::default()).0),
        "(0.01) * ctx.enemy_max_health"
    );
    assert_eq!(
        formula(&effect(description, FLIP).0),
        "(0.06) * ctx.enemy_max_health"
    );

    let description = "take {{as|{{ap|60/6}} {{as|(+ {{ap|6/6}}% AP)}} magic damage}} \
        every {{fd|0.5}} seconds over 3 seconds, for a total of {{as|60|magic damage}} \
        {{as|(+ 6% AP)}}. Against monsters, deal {{as|10 '''bonus''' magic damage}}.";

    let (tick, total) = effect(description, Variant::default());
    assert_eq!(formula(&tick), "(10.0) + ((0.01) * ctx.ability_power)");
    assert_eq!(
        formula(&total.unwrap()),
        "(60.0) + ((0.06) * ctx.ability_power)"
    );
}

#[test]
fn nests_ratios_of_the_target_health() {
    let description = "deal {{as|12% {{as|(+ 5% per 100 '''bonus''' AD)}} of target's \
        '''maximum''' health}} {{as|'''bonus''' physical damage}}";

    assert_eq!(
        formula(&effect(description, Variant::default()).0),
        "((0.12) + ((0.05) * (ctx.bonus_ad * 0.01))) * ctx.enemy_max_health"
    );

    let description = "deal {{as|'''bonus''' true damage}} equal to {{as|{{rd|{{fd|0.7}}%|\
        {{fd|0.5}}%}} (+ {{fd|0.1}}% per target's ''Reap'' stack) of the target's \
        '''maximum''' health}}";

    assert_eq!(
        formula(&effect(description, RANGED).0),
        "((0.005) + ((0.001) * ctx.stacks)) * ctx.enemy_max_health"
    );
}

#[test]
fn effects_without_values_render_as_the_zero() {
    let empty = wiki::parser::Effect::default();
    assert_eq!(
        empty.simplify_formula(Key::P).unwrap().as_deref(),
        Some(wiki::ZERO)
    );
    assert_eq!(wiki::ZERO, "0");
}
