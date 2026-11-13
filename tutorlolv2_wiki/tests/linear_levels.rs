use tutorlolv2_types::CtxVar::{self, *};
use tutorlolv2_wiki::wikitext::{Formula, Sample};
fn at(formula: &Formula, var: CtxVar, value: f64) -> f64 {
    formula.eval(&Sample::new([(var, value)])).unwrap()
}

#[test]
fn rank_progressions_continue_beyond_normal_and_listed_ranks() {
    for var in [QLevel, WLevel, ELevel, RLevel, Level] {
        let formula = Formula::ByRank {
            rank: var,
            values: vec![10.0, 20.0, 30.0, 40.0],
        };
        assert!(!formula.to_string().contains("match"));
        for rank in [6.0, 19.0, 255.0] {
            assert_eq!(at(&formula, var, rank), 10.0 * rank);
        }
    }
}
#[test]
fn contiguous_level_matches_become_growth_with_their_lower_bound_intact() {
    let formula = Formula::Table {
        var: Level,
        keys: vec![1, 2, 3, 4],
        values: vec![10.0, 20.0, 30.0, 40.0],
    };
    assert!(!formula.to_string().contains("match"));
    assert_eq!(at(&formula, Level, 0.0), 10.0);
    assert_eq!(at(&formula, Level, 18.0), 180.0);
    assert_eq!(at(&formula, Level, 255.0), 2550.0);
    let offset = Formula::Table {
        var: Level,
        keys: vec![6, 7, 8],
        values: vec![100.0, 120.0, 140.0],
    };
    assert_eq!(at(&offset, Level, 1.0), 100.0);
    assert_eq!(at(&offset, Level, 20.0), 380.0);
}
#[test]
fn rounded_arithmetic_progressions_use_endpoint_growth() {
    let values = vec![0.667, 1.333, 2.0, 2.667];
    let formula = Formula::Table {
        var: Level,
        keys: vec![1, 2, 3, 4],
        values: values.clone(),
    };
    assert!(!formula.to_string().contains("match"));
    for (i, expected) in values.into_iter().enumerate() {
        assert!((at(&formula, Level, (i + 1) as f64) - expected).abs() <= expected * 0.001);
    }
    assert!(at(&formula, Level, 255.0) > 169.0);
}
#[test]
fn nonlinear_threshold_and_stat_caps_are_not_removed() {
    for (keys, values) in [
        (vec![1, 6, 11, 16], vec![0.3, 0.4, 0.5, 0.6]),
        (vec![1, 2, 3, 4], vec![10.0, 10.0, 20.0, 20.0]),
        (vec![1, 2, 3], vec![10.0, 20.0, 40.0]),
        (vec![1, 2], vec![10.0, 20.0]),
    ] {
        let formula = Formula::Table {
            var: Level,
            keys,
            values: values.clone(),
        };
        assert!(formula.to_string().contains("match"));
        assert_eq!(at(&formula, Level, 255.0), *values.last().unwrap());
    }
    let stat = Formula::NumericTable {
        axis: Box::new(Formula::Var(CritChance)),
        keys: vec![0.0, 50.0, 100.0],
        values: vec![0.0, 25.0, 50.0],
        interpolated: true,
    };
    assert_eq!(at(&stat, CritChance, 150.0), 50.0);
}

#[test]
fn converted_partial_rank_tables_keep_their_lower_bound() {
    let formula = Formula::ByRank {
        rank: QLevel,
        values: vec![100.0, 150.0, 200.0, 250.0],
    };
    assert_eq!(at(&formula, QLevel, 0.0), 100.0);
    assert_eq!(at(&formula, QLevel, 6.0), 350.0);
    let existing = Formula::ByRank {
        rank: RLevel,
        values: vec![100.0, 200.0, 300.0],
    };
    assert_eq!(at(&existing, RLevel, 0.0), 0.0);
}

#[test]
fn numeric_level_tables_and_map_wrappers_agree_with_printed_growth() {
    use tutorlolv2_types::MapMode;
    use tutorlolv2_wiki::wikitext::LegacyFormula;
    let base = Formula::NumericTable {
        axis: Box::new(Formula::Var(Level)),
        keys: vec![1.0, 2.0, 3.0],
        values: vec![10.0, 20.0, 30.0],
        interpolated: false,
    };
    let formula = base.with_map(
        MapMode::Arena,
        Formula::Table {
            var: Level,
            keys: vec![1, 2, 3],
            values: vec![20.0, 40.0, 60.0],
        },
    );
    let printed = formula.to_string();
    assert!(printed.contains("match ctx.map"));
    assert!(!printed.contains("match ctx.level"));
    let parsed = LegacyFormula::parse(&printed).unwrap();
    for mode in [MapMode::Default, MapMode::Arena] {
        for level in [0.0, 1.0, 18.0, 255.0] {
            let sample = Sample::new([(Level, level)]).on(mode);
            assert_eq!(formula.eval(&sample), Some(parsed.eval(&sample)));
        }
    }
    let fractional = Formula::NumericTable {
        axis: Box::new(Formula::Var(Level)),
        keys: vec![0.5, 1.5, 2.5],
        values: vec![10.0, 20.0, 30.0],
        interpolated: false,
    };
    assert_eq!(at(&fractional, Level, 1.0), 10.0);
    assert!(fractional.to_string().contains("if axis"));
}

#[test]
fn tolerance_does_not_hide_plateaus_turning_points_or_changing_slopes() {
    for values in [
        vec![0.02, 0.02, 0.02, 0.03, 0.03, 0.03],
        vec![1.0, 1.0001, 1.0],
        vec![1000.0, 1000.01, 1000.4],
    ] {
        let formula = Formula::ByRank {
            rank: QLevel,
            values,
        };
        assert!(formula.to_string().contains("match"));
    }
}

#[test]
fn small_but_precise_curvature_is_not_mistaken_for_rounding() {
    // An exponential can fit within a value-relative tolerance over three ranks,
    // but its varying increments cannot be explained by its written precision.
    for values in [
        vec![1.0, 1.04, 1.0816],
        vec![0.0001, 0.000104, 0.00010816],
        vec![1e6, 1e6 + 1.0, 1e6 + 2.04],
    ] {
        let last = *values.last().unwrap();
        let formula = Formula::ByRank {
            rank: RLevel,
            values,
        };
        assert!(formula.to_string().contains("match"));
        assert_eq!(at(&formula, RLevel, 255.0), last);
    }
}
