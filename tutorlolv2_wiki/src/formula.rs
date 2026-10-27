#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequencePattern {
    Inconclusive,
    Linear,
    Quadratic,
    Cubic,
    Unknown,
}

fn differences(v: &[f64]) -> Vec<f64> {
    v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}

/// Number of decimal places needed to write `value`, capped at 6
fn decimals(value: f64) -> i32 {
    (0..6)
        .find(|&p| {
            let scaled = value * 10f64.powi(p);
            (scaled - scaled.round()).abs() < 1e-6
        })
        .unwrap_or(6)
}

impl SequencePattern {
    /// Classifies the sequence by the lowest polynomial degree whose differences
    /// are constant. Wiki values are rounded to the decimals they are written with,
    /// so the tolerance is the maximum error that rounding can introduce in the
    /// k-th differences, `2^k * 0.5 * 10^-p`, instead of a fixed threshold
    pub fn new(values: &[f64]) -> Self {
        if values.len() < 3 {
            return Self::Inconclusive;
        }

        let precision = values.iter().copied().map(decimals).max().unwrap_or(0);
        let rounding = 0.5 * 10f64.powi(-precision);

        // Rounding alone can't tell a step table like 0.02, 0.02, 0.03 apart from
        // a slow slope, so the differences must also be consistent with their size
        let constant = |v: &[f64], degree: i32| -> bool {
            let tolerance = 2f64.powi(degree) * rounding + 1e-9;
            let min = v.iter().copied().fold(f64::INFINITY, f64::min);
            let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let mean = v.iter().map(|x| x.abs()).sum::<f64>() / v.len() as f64;
            let spread = max - min;

            !v.is_empty() && spread <= tolerance && spread <= 0.25 * mean + 1e-9
        };

        let mut diffs = values.to_vec();

        for (degree, pattern) in [(1, Self::Linear), (2, Self::Quadratic), (3, Self::Cubic)] {
            diffs = differences(&diffs);

            // A sequence of n values always fits a polynomial of degree n - 1,
            // so it only says something about the pattern with fewer degrees
            if diffs.len() < 2 {
                break;
            }

            if constant(&diffs, degree) {
                return pattern;
            }
        }

        Self::Unknown
    }

    pub fn predict_next(&self, values: &[f64]) -> f64 {
        match self {
            SequencePattern::Linear => {
                let d1 = values[values.len() - 1] - values[values.len() - 2];
                values[values.len() - 1] + d1
            }
            SequencePattern::Quadratic => {
                let d1 = differences(values);
                let d2 = differences(&d1);
                let next_d1 = d1[d1.len() - 1] + d2[d2.len() - 1];
                values[values.len() - 1] + next_d1
            }
            SequencePattern::Cubic => {
                let d1 = differences(values);
                let d2 = differences(&d1);
                let d3 = differences(&d2);
                let next_d2 = d2[d2.len() - 1] + d3[d3.len() - 1];
                let next_d1 = d1[d1.len() - 1] + next_d2;

                values[values.len() - 1] + next_d1
            }
            _ => values[values.len() - 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SequencePattern::{self, *};

    #[test]
    fn classifies_rounded_sequences() {
        assert_eq!(
            SequencePattern::new(&[60.0, 95.0, 130.0, 165.0, 200.0]),
            Linear
        );
        assert_eq!(
            SequencePattern::new(&[0.75, 0.813, 0.875, 0.938, 1.0]),
            Linear
        );
        assert_eq!(
            SequencePattern::new(&[33.33, 50.0, 66.67, 83.33, 100.0]),
            Linear
        );
        assert_eq!(SequencePattern::new(&[10.0, 67.5, 137.5, 220.0]), Quadratic);
    }

    #[test]
    fn rejects_step_tables() {
        // Previously accepted as linear because every difference is below 3
        let steps = [0.02, 0.02, 0.02, 0.03, 0.03, 0.03, 0.04, 0.04, 0.04];
        assert_eq!(SequencePattern::new(&steps), Unknown);
        assert_eq!(SequencePattern::new(&[1.0, 2.0, 4.0, 5.0, 6.0]), Unknown);
    }

    #[test]
    fn two_values_are_inconclusive() {
        assert_eq!(SequencePattern::new(&[1.0, 2.0]), Inconclusive);
    }
}
