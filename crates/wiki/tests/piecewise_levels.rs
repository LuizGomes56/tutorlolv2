use types::CtxVar::{self, *};
use wiki::wikitext::{Formula, LegacyFormula, Sample};
const VALUES: [f64; 18] = [
    35., 38., 41., 44., 47., 50., 53., 62., 71., 80., 89., 98., 107., 122., 137., 152., 167., 182.,
];
fn at(formula: &Formula, axis: CtxVar, level: f64) -> f64 {
    formula.eval(&Sample::new([(axis, level)])).unwrap()
}

#[test]
fn supplied_level_table_uses_three_growth_ranges() {
    let formula = Formula::Table {
        var: Level,
        keys: (1..=18).collect(),
        values: VALUES.to_vec(),
    };
    let source = formula.to_string();
    assert_eq!(source.matches("=>").count(), 3, "{source}");
    assert!(source.contains("..8 =>"), "{source}");
    assert!(source.contains("8..14 =>"), "{source}");
    assert!(source.contains("14.. =>"), "{source}");
    assert_eq!(at(&formula, Level, 0.), 35.);
    for (i, value) in VALUES.into_iter().enumerate() {
        assert_eq!(at(&formula, Level, (i + 1) as f64), value);
    }
    assert_eq!(at(&formula, Level, 19.), 197.);
    assert_eq!(at(&formula, Level, 255.), 3737.);
    let parsed = LegacyFormula::parse(&source).unwrap();
    for level in 0..=255 {
        let sample = Sample::new([(Level, level as f64)]);
        assert_eq!(formula.eval(&sample), Some(parsed.eval(&sample)));
    }
}

#[test]
fn ranked_tables_share_piecewise_growth_and_lower_bounds() {
    let formula = Formula::ByRank {
        rank: QLevel,
        values: vec![10., 20., 30., 40., 70., 75., 80., 85.],
    };
    assert_eq!(formula.to_string().matches("=>").count(), 2);
    assert_eq!(at(&formula, QLevel, 0.), 10.);
    assert_eq!(at(&formula, QLevel, 4.), 40.);
    assert_eq!(at(&formula, QLevel, 5.), 70.);
    assert_eq!(at(&formula, QLevel, 255.), 1320.);
}

#[test]
fn repeated_values_merge_without_erasing_thresholds() {
    let formula = Formula::Table {
        var: Level,
        keys: (1..=6).collect(),
        values: vec![10., 10., 20., 20., 30., 30.],
    };
    assert_eq!(formula.to_string().matches("=>").count(), 3);
    for level in 0..=255 {
        let expected = if level < 3 {
            10.
        } else if level < 5 {
            20.
        } else {
            30.
        };
        assert_eq!(at(&formula, Level, level as f64), expected);
    }
}

#[test]
fn irregular_keys_and_curves_keep_lookup_semantics() {
    let irregular = Formula::Table {
        var: Level,
        keys: vec![1, 2, 3, 4, 10, 11, 12, 13],
        values: vec![10., 20., 30., 40., 100., 110., 120., 130.],
    };
    assert_eq!(irregular.to_string().matches("=>").count(), 8);
    assert_eq!(at(&irregular, Level, 9.), 40.);
    let curve = Formula::Table {
        var: Level,
        keys: (1..=8).collect(),
        values: vec![1., 4., 9., 16., 25., 36., 49., 64.],
    };
    assert_eq!(curve.to_string().matches("=>").count(), 8);
    assert_eq!(at(&curve, Level, 255.), 64.);
}

#[test]
fn rounded_and_decreasing_runs_preserve_listed_values() {
    for values in [
        vec![0.667, 1.333, 2., 2.667, 4., 5., 6., 7.],
        vec![5., 4., 3., 2., -10., -12., -14., -16.],
    ] {
        let formula = Formula::Table {
            var: Level,
            keys: (1..=8).collect(),
            values: values.clone(),
        };
        assert_eq!(formula.to_string().matches("=>").count(), 2);
        for (i, value) in values.into_iter().enumerate() {
            assert!(
                (at(&formula, Level, (i + 1) as f64) - value).abs()
                    <= value.abs() * Formula::LINEAR_RELATIVE_TOLERANCE + 1e-12
            );
        }
    }
}
