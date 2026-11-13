use tutorlolv2_types::CtxVar;
use tutorlolv2_wiki::{
    parser::{Effect, LevelArm, Scaling},
    wikitext::{Formula, LegacyFormula, Sample},
};

fn equivalent(scaling: Scaling, axis: CtxVar) {
    let legacy = LegacyFormula::parse(&scaling.render(axis).unwrap()).unwrap();
    let typed = scaling.to_formula(axis).unwrap();
    for level in 1..=20 {
        let sample = Sample::new([(axis, f64::from(level)), (CtxVar::AbilityPower, 123.0)]);
        assert!(
            (legacy.eval(&sample) - typed.eval(&sample).unwrap()).abs() < 1e-6,
            "{scaling:?}, {level}: {typed}"
        );
    }
}

#[test]
fn structured_scalings_preserve_progressions_and_targets() {
    let axis = CtxVar::QLevel;
    for values in [
        vec![10.0],
        vec![10.0, 20.0],
        vec![0.75, 0.813, 0.875, 0.938, 1.0],
        vec![1.0, 4.0, 9.0, 16.0, 25.0],
        vec![1.0, 8.0, 27.0, 64.0, 125.0],
        vec![10.0, 20.0, 30.0, 40.0],
    ] {
        equivalent(
            Scaling::Flat {
                values: values.clone(),
            },
            axis,
        );
        equivalent(
            Scaling::Ranked {
                values: values.clone(),
                ctx_var: CtxVar::AbilityPower,
            },
            axis,
        );
        equivalent(
            Scaling::RankedPer100 {
                values,
                ctx_var: CtxVar::AbilityPower,
            },
            axis,
        );
    }
    let outer = Scaling::BasedOnLevel {
        level_var: CtxVar::Level,
        arms: vec![LevelArm::new(..6, 0.1), LevelArm::new(6.., 0.2)],
        ctx_var: CtxVar::AbilityPower,
        debug: String::new(),
    };
    equivalent(outer.clone(), CtxVar::Level);
    equivalent(
        Scaling::Nested {
            outer: Box::new(outer),
            inner: vec![Scaling::Flat { values: vec![0.2] }],
            raw: String::new(),
        },
        CtxVar::Level,
    );
}

#[test]
fn explicit_formula_is_structured_and_binds_only_x() {
    let effect = Effect {
        use_formula: Some("3 + x * 2".into()),
        ..Default::default()
    };
    let formula = effect.use_formula_tree(CtxVar::Level).unwrap();
    assert_eq!(
        formula.eval(&Sample::new([(CtxVar::Level, 4.0)])),
        Some(11.0)
    );
    assert!(Formula::parse_expression("unknown(2)").is_err());
    assert!(Formula::parse_expression("match ctx.level { 1.. => ctx.ability_power }").is_err());
}

#[test]
fn structured_ranges_and_multipliers_preserve_metadata_rules() {
    for debug in ["based on level", "based on rank", "based on ability power"] {
        equivalent(
            Scaling::RangePercentAttr {
                min: 0.2,
                max: 0.8,
                debug: debug.into(),
                ctx_var: CtxVar::AbilityPower,
            },
            CtxVar::QLevel,
        );
    }
    equivalent(
        Scaling::Multiplier {
            raw: "1 + 0.3 per 100% bonus attack speed".into(),
            base: 1.0,
            inner: vec![Scaling::Per100 {
                value: 0.003,
                ctx_var: CtxVar::AbilityPower,
            }],
        },
        CtxVar::QLevel,
    );
    equivalent(
        Scaling::Multiplier {
            raw: String::new(),
            base: 1.0,
            inner: vec![Scaling::Simple {
                value: 0.2,
                ctx_var: CtxVar::AbilityPower,
            }],
        },
        CtxVar::QLevel,
    );
    let effect = Effect {
        use_values: Some(vec![5.0, 10.0, 15.0]),
        use_levels: Some(vec![1, 6, 11]),
        ..Default::default()
    };
    let formula = effect.to_formula(CtxVar::Level).unwrap().unwrap();
    assert_eq!(
        formula.eval(&Sample::new([(CtxVar::Level, 8.0)])),
        Some(10.0)
    );
    assert_eq!(
        formula.eval(&Sample::new([(CtxVar::Level, 18.0)])),
        Some(15.0)
    );
}

#[test]
fn multiplier_without_base_does_not_create_damage() {
    let mut effect = Effect {
        scalings: vec![Scaling::Multiplier {
            raw: String::new(),
            base: 2.0,
            inner: vec![],
        }],
        ..Default::default()
    };
    assert_eq!(
        effect
            .to_formula(CtxVar::Level)
            .unwrap()
            .unwrap()
            .eval(&Sample::default()),
        Some(0.0)
    );
    effect.scalings.push(Scaling::Flat { values: vec![7.0] });
    assert_eq!(
        effect
            .to_formula(CtxVar::Level)
            .unwrap()
            .unwrap()
            .eval(&Sample::default()),
        Some(7.0)
    );
}
