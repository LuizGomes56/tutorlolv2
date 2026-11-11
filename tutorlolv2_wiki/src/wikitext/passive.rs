//! Port of `p.pp` from `Module:Ability progression`, which renders `{{pp}}` and
//! `{{pplevel}}`: values by champion level, or by another quantity named with `type=`.
//!
//! The first argument is the bottom row of values and the second one the top row
//! of keys (levels by default). Each row is a `;` separated list of segments:
//! - `A to B for N` or `A to B by S`: `N` values linearly distributed from `A` to `B`
//! - `A to B`: a linear filling, sized when both rows are known
//! - A formula of `x`, as `8+2*x for 6`, where `then` is the previous value. Without
//!   `for N` it is a filling too
//! - A single value
//!
//! A row with a filling grows to the length of the other row, or both grow to
//! 18 values when both have one

use {
    super::{
        Expr, Nodes, Template, Value,
        progression::{RangeFormula, Round},
    },
    lazy_regex::regex,
};

/// Values of a `{{pp}}`, with the key where each one starts
#[derive(Clone, Debug, PartialEq)]
pub struct PassiveProgression {
    pub values: Vec<f64>,
    /// Top row, `1..=values.len()` when the template has none
    pub keys: Vec<f64>,
    /// Text of `type=`, else `label1=`, which names what the keys are
    pub axis: Option<String>,
    /// Suffix of each value, given by `key=`, such as `%`
    pub key: String,
}

impl PassiveProgression {
    /// Number of values of rows that fill up to the default size
    const DEFAULT_SIZE: usize = 18;

    pub fn new(template: &Template) -> Result<Self, String> {
        let text = |nodes: &Nodes| nodes.to_string().trim().to_string();
        let named = |name: &str| template.named(name).map(text);

        // 2 decimals unless `round=false`, or `ceil`, `floor`, `abs` and `trunc`
        let round = |name: &str| match named(name).as_deref() {
            Some("false") => None,
            Some(round) => Round::new(round).or(Some(Round::Decimals(2))),
            None => Some(Round::Decimals(2)),
        };

        let bottom = template.positional(0).map(text).unwrap_or_default();
        let top = template.positional(1).map(text).unwrap_or_default();

        let mut rows = [
            Row::new(&bottom, round("round"))?,
            Row::new(&top, round("round1"))?,
        ];

        // Size of each filling, as `fill` in the module
        let lens = rows.each_ref().map(|row| row.values.len());
        let mut fill = lens.map(|len| Self::DEFAULT_SIZE.saturating_sub(len));

        match (
            lens[0] >= lens[1],
            rows[0].filling.is_none(),
            rows[1].filling.is_none(),
        ) {
            (true, true, _) => fill[1] = lens[0] - lens[1],
            (false, _, true) => fill[0] = lens[1] - lens[0],
            _ => {}
        }

        for (row, times) in rows.iter_mut().zip(fill) {
            row.fill(times)?;
        }

        let [bottom, top] = rows;
        let keys = match top.values.is_empty() {
            true => (1..=bottom.values.len()).map(|key| key as f64).collect(),
            false => top.values,
        };

        if bottom.values.is_empty() {
            return Err(format!("No values in {template}"));
        }

        // The module shows "Variables are unmatched" for rows of different sizes
        if keys.len() != bottom.values.len() {
            return Err(format!("Variables are unmatched in {template}"));
        }

        let axis = named("type")
            .or_else(|| named("label1"))
            .map(|axis| Nodes::parse(&axis).plain().trim().to_string());

        Ok(Self {
            values: bottom.values,
            keys,
            axis,
            key: named("key").unwrap_or_default(),
        })
    }
}

/// Number of values of a [`Range`]
enum Count {
    /// `A to B for N`
    Times(f64),
    /// `A to B by S`, as the number of values the step `S` gives
    Step(f64),
}

/// `A to B`, `A to B for N` or `A to B by S`. Without a count, it is a filling
struct Range {
    start: f64,
    finish: f64,
    count: Option<Count>,
}

impl Range {
    /// `None` when a part is not arithmetic
    fn new(segment: &str) -> Option<Self> {
        let (start, rest) = segment.split_once("to")?;
        let start = Expr::new(start).eval()?;

        let (finish, count) = match (rest.split_once("by"), rest.split_once("for")) {
            (Some((finish, step)), _) => {
                let finish = Expr::new(finish).eval()?;
                let step = Expr::new(step).eval()?;
                let times = ((finish - start).abs() / step + 1.0).round();
                (finish, Some(Count::Step(times)))
            }
            (None, Some((finish, times))) => (
                Expr::new(finish).eval()?,
                Some(Count::Times(Expr::new(times).eval()?)),
            ),
            (None, None) => (Expr::new(rest).eval()?, None),
        };

        Some(Self {
            start,
            finish,
            count,
        })
    }
}

/// Range or formula of a row without a count, sized after both rows are read
#[derive(Clone, Debug)]
enum Filling {
    Linear { start: f64, finish: f64 },
    Formula(String),
}

/// One row of a `{{pp}}`
#[derive(Default)]
struct Row {
    values: Vec<f64>,
    /// Index where the filling goes, its kind, and the value before it
    filling: Option<(usize, Filling, Option<f64>)>,
    round: Option<Round>,
}

impl Row {
    fn new(src: &str, round: Option<Round>) -> Result<Self, String> {
        let mut row = Self {
            round,
            ..Default::default()
        };

        let segments = src.split(';').collect::<Vec<_>>();
        let mut last_value = None;

        for (i, segment) in segments.iter().enumerate() {
            if segment.is_empty() {
                break;
            }

            let invalid = || format!("Invalid segment {segment:?} of {src:?}");

            if !(segment.contains("to") || segment.contains('x')) || segment.contains('<') {
                // A single value, where `then` is the previous one
                let value = match last_value {
                    Some(last) => segment.replacen("then", &Value::Number(last).to_string(), 1),
                    None => segment.to_string(),
                };
                let value = Expr::new(&value).eval().ok_or_else(invalid)?;

                row.push(value);

                let next_is_range = segments.get(i + 1).is_some_and(|next| next.contains("to"));
                last_value = (!next_is_range).then_some(value);
                continue;
            }

            if !segment.contains('x')
                && let Some(Range {
                    start,
                    finish,
                    count,
                }) = Range::new(segment)
            {
                match count {
                    None => {
                        row.set_filling(Filling::Linear { start, finish }, last_value)
                            .ok_or_else(invalid)?;
                    }
                    Some(Count::Times(times)) => row.linear(start, finish, times, last_value),
                    // A count derived from a step includes the previous value
                    Some(Count::Step(times)) if last_value == Some(start) => {
                        row.linear(start, finish, times - 1.0, last_value)
                    }
                    Some(Count::Step(times)) => row.linear(start, finish, times, last_value),
                }

                last_value = Some(finish);
                continue;
            }

            let (formula, times) = match regex!(r"^ *(.*?) *for *(.*?) *$").captures(segment) {
                Some(captures) => (captures[1].to_string(), Some(captures[2].to_string())),
                None => (segment.to_string(), None),
            };

            let times = match times {
                Some(times) => Some(Expr::new(&times).eval().ok_or_else(invalid)? as usize),
                None => None,
            };

            let mut formula = match last_value {
                Some(last) => formula.replacen("then", &Value::Number(last).to_string(), 1),
                None => formula,
            };

            if formula.contains("to") {
                formula = RangeFormula::from_ranges(&formula);

                if let Some(times) = times {
                    formula = formula.replace("times", &times.to_string());
                }
            }

            let probe = formula.replace("times", "2").replace('x', "1");
            Expr::new(&probe).eval().ok_or_else(invalid)?;

            match times {
                None => {
                    row.set_filling(Filling::Formula(formula), last_value)
                        .ok_or_else(invalid)?;
                    last_value = None;
                }
                Some(times) => {
                    for x in 1..=times {
                        let value = Self::eval_at(&formula, x).ok_or_else(invalid)?;
                        row.push(value);
                        last_value = Some(value);
                    }
                }
            }
        }

        Ok(row)
    }

    fn eval_at(formula: &str, x: usize) -> Option<f64> {
        Expr::new(&formula.replace('x', &x.to_string())).eval()
    }

    fn push(&mut self, value: f64) {
        self.values
            .push(self.round.map_or(value, |round| round.apply(value)));
    }

    /// Only one filling is allowed per row
    fn set_filling(&mut self, filling: Filling, last_value: Option<f64>) -> Option<()> {
        if self.filling.is_some() {
            return None;
        }

        self.filling = Some((self.values.len(), filling, last_value));
        Some(())
    }

    /// Pushes `times` values from `start` to `finish`. When `start` is the previous
    /// value, the range continues from it with `times` new values
    fn linear(&mut self, start: f64, finish: f64, times: f64, last_value: Option<f64>) {
        let mut times = times as usize;

        if last_value != Some(start) {
            self.push(start);
            times = times.saturating_sub(1);
        }

        for x in 1..=times {
            self.push(start + (finish - start) * x as f64 / times as f64);
        }
    }

    /// Inserts the filling with `times` values where it was written
    fn fill(&mut self, times: usize) -> Result<(), String> {
        let Some((index, filling, last_value)) = self.filling.take() else {
            return Ok(());
        };

        let after = self.values.split_off(index);

        match filling {
            Filling::Linear { start, finish } => {
                self.linear(start, finish, times as f64, last_value)
            }
            Filling::Formula(formula) => {
                let formula = formula.replace("times", &times.to_string());

                for x in 1..=times {
                    let value = Self::eval_at(&formula, x)
                        .ok_or_else(|| format!("Invalid formula {formula:?}"))?;
                    self.push(value);
                }
            }
        }

        self.values.extend(after);
        Ok(())
    }
}
