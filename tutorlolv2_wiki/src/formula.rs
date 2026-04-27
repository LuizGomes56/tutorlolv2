use crate::{client::MayFail, parser::Effect};
use serde::{Deserialize, Serialize};
use tutorlolv2_types::Key;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Formula {
    PerLevel(Vec<String>),
    Linear(String),
}

impl Formula {
    pub fn new(champion_id: &str, key: Key, effect: &Effect) -> MayFail<Self> {
        let Effect {
            index,
            inner,
            scalings,
            use_formula,
            use_values,
            base,
        } = effect;

        if let Some(b) = base {
            let pat = SequencePattern::new(b);

            if !matches!(
                pat,
                SequencePattern::Linear | SequencePattern::Quadratic | SequencePattern::Cubic
            ) {
                println!("[{champion_id}][{key:?}][{index}][{pat:?}]: {b:?}");
            }
        };

        let _ = match base {
            Some(b) if let Some(v) = use_values => {
                if b.len() != v.len() {
                    return Err(format!(
                        "[{champion_id}][::1] [b != v] Base: {b:?}; UseValues: {v:?}"
                    )
                    .into());
                }
                b.len()
            }
            Some(b)
                if let len = b.len()
                    && key != Key::P
                    && !matches!(len, 3 | 5 | 6) =>
            {
                // Nidalee scales with R level for all {1} abilities with n = 4
                return Err(format!("[{champion_id}][::2] base[{len}][{key:?}]: {b:?}").into());
            }
            Some(b) => b.len(),
            None if let Some(v) = use_values => v.len(),
            _ => 0,
        };

        Ok(Self::Linear("0".into()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequencePattern {
    Inconclusive,
    Linear,
    Quadratic,
    Cubic,
    Unknown,
}

impl SequencePattern {
    pub fn new(values: &[f64]) -> Self {
        const TOLERANCE: f64 = 3.0;

        let len = values.len();

        let differences = |v: &[f64]| v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
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
}
