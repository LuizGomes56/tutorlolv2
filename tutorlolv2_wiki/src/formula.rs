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

impl SequencePattern {
    pub fn new(values: &[f64]) -> Self {
        const TOLERANCE: f64 = 3.0;

        let len = values.len();

        let approx_constant = |v: &[f64]| -> bool {
            if v.is_empty() {
                return false;
            }

            let mut min = v[0];
            let mut max = min;

            for &v in v.iter().skip(1) {
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
            }

            (max - min).abs() < TOLERANCE
        };

        if len < 3 {
            return Self::Inconclusive;
        }

        let diff_1 = differences(values);

        if approx_constant(&diff_1) {
            return Self::Linear;
        }

        let diff_2 = differences(&diff_1);

        match approx_constant(&diff_2) {
            true if len == 3 => Self::Quadratic,
            false if len == 3 => Self::Unknown,
            _ => {
                let diff_3 = differences(&diff_2);

                match approx_constant(&diff_3) {
                    true if len == 4 => Self::Cubic,
                    false if len == 4 => Self::Unknown,
                    _ => Self::Unknown,
                }
            }
        }
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
