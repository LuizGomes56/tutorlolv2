//! Port of `p.ap` from `Module:Ability progression`, which renders `{{ap}}`.
//! Each positional argument is one of:
//! - `A to B [N]`: `N` values linearly distributed from `A` to `B`. Without `N`,
//!   the first range of the template has 5 values, or 3 in ultimates
//! - A formula with `x`, the rank starting at 1, followed by the number of ranks,
//!   such as `(5 to 80)*(1+(1 to 1.75)) 4`, where each `A to B` is linear in `x`
//! - A single value, such as `70/10`, which is evaluated as arithmetic
//!
//! At most 6 values are produced, and arguments that can't be evaluated are kept
//! as they are written

use {
    super::{Expr, Nodes, Template},
    lazy_regex::regex,
    serde::{Deserialize, Serialize},
    std::fmt,
    tutorlolv2_types::Key,
};

/// One of the values of a [`Progression`]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Value {
    Number(f64),
    /// Argument that is not arithmetic, kept as written
    Text(String),
}

impl Value {
    pub fn number(&self) -> Option<f64> {
        match self {
            Value::Number(value) => Some(*value),
            Value::Text(_) => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Removes the noise of float arithmetic, as in 0.30000000000000004
            Value::Number(value) => write!(f, "{}", (value * 1e10).round() / 1e10),
            Value::Text(text) => f.write_str(text),
        }
    }
}

/// Rounding of each value, given by `round=`
#[derive(Clone, Copy, Debug, PartialEq)]
enum Round {
    Decimals(i32),
    Abs,
    Floor,
    Ceil,
    Trunc,
}

impl Round {
    fn new(text: &str) -> Option<Self> {
        match text.trim() {
            "abs" => Some(Self::Abs),
            "floor" => Some(Self::Floor),
            "ceil" => Some(Self::Ceil),
            "trunc" => Some(Self::Trunc),
            decimals => decimals.parse().ok().map(Self::Decimals),
        }
    }

    fn apply(self, value: f64) -> f64 {
        match self {
            Self::Decimals(decimals) => {
                let factor = 10f64.powi(decimals);
                (value * factor + 0.5).floor() / factor
            }
            Self::Abs => value.abs(),
            Self::Floor => value.floor(),
            Self::Ceil => value.ceil(),
            Self::Trunc => value.trunc(),
        }
    }
}

/// Values of an `{{ap}}` template, one per rank of the ability
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Progression {
    pub values: Vec<Value>,
    /// Suffix of each value, given by `key=`, such as `%`
    pub key: String,
}

impl Progression {
    /// Maximum number of values of a single template
    pub const MAX: usize = 6;

    /// Evaluates the `{{ap}}` `template` of an ability of `skill`. Fails, as the
    /// wiki does, when more than [`Self::MAX`] values would be produced
    pub fn new(template: &Template, skill: Key) -> Result<Self, String> {
        let text = |nodes: &Nodes| nodes.to_string().trim().to_string();
        let round = template
            .named("round")
            .and_then(|value| Round::new(&text(value)));

        let key = template.named("key").map(text).unwrap_or_default();
        let mut builder = Builder {
            values: Vec::new(),
            fill: if skill == Key::R { 3 } else { 5 },
            round,
        };

        // `Template:Ability progression` evaluates each argument as arithmetic
        // before reading it, and stops at the first empty one
        for arg in template.positionals().take(Self::MAX).map(text) {
            if arg.is_empty() {
                break;
            }

            let arg = match Expr::new(&arg).eval() {
                Some(value) => Value::Number(value).to_string(),
                None => arg,
            };

            builder.push(&arg);

            if builder.values.len() > Self::MAX {
                return Err(format!("Maximum size exceeded: {template}"));
            }
        }

        Ok(Self {
            values: builder.values,
            key,
        })
    }

    /// Every value, if all of them are numbers
    pub fn numbers(&self) -> Option<Vec<f64>> {
        self.values.iter().map(Value::number).collect()
    }
}

struct Builder {
    values: Vec<Value>,
    /// Number of values of the first range without an explicit count, after which
    /// it becomes 0 and other ranges without a count are kept as written
    fill: usize,
    round: Option<Round>,
}

impl Builder {
    fn number(&self, value: f64) -> Value {
        Value::Number(self.round.map_or(value, |round| round.apply(value)))
    }

    /// Count of an explicit `N` that may be empty, consuming the default count
    fn times(&mut self, times: &str) -> Option<usize> {
        match times.is_empty() {
            true if self.fill != 0 => Some(std::mem::take(&mut self.fill)),
            true => None,
            false => Expr::new(times).eval().map(|times| times as usize),
        }
    }

    fn push(&mut self, arg: &str) {
        let has_to = arg.contains("to");
        let has_x = arg.contains('x');

        if !(has_to || has_x) || arg.contains('<') {
            let value = match Expr::new(arg).eval() {
                Some(value) => self.number(value),
                None => Value::Text(arg.to_string()),
            };

            return self.values.push(value);
        }

        if has_to && !has_x && self.linear(arg).is_some() {
            return;
        }

        if self.formula(arg).is_none() {
            self.values.push(Value::Text(arg.to_string()));
        }
    }

    /// `A to B [N]`, where `A` and `B` are arithmetic
    fn linear(&mut self, arg: &str) -> Option<()> {
        let (start, _) = arg.split_once("to")?;
        let captures = regex!(r"^.* *to *([^ ]*) *([^ ]*)$").captures(arg)?;
        let start = Expr::new(start).eval()?;
        let finish = Expr::new(&captures[1]).eval()?;
        let times = &captures[2];

        if !times.is_empty() && Expr::new(times).eval().is_none() {
            return None;
        }

        let times = self.times(times)?;

        // Accumulated as the module does, so values match the wiki to the last digit
        let scale = (finish - start) / (times as f64 - 1.0);
        let mut value = start - scale;

        for _ in 0..times {
            value += scale;
            self.values.push(self.number(value));
        }

        Some(())
    }

    /// Formula of `x`, the rank, optionally followed by the number of ranks
    fn formula(&mut self, arg: &str) -> Option<()> {
        let (formula, times) = match regex!(r"^(.*[0-9x)]) +([^ ]*)$").captures(arg) {
            Some(captures) => (captures[1].to_string(), captures[2].to_string()),
            None => (arg.to_string(), String::new()),
        };

        let times = match Expr::new(&times).eval() {
            Some(times) => times as usize,
            None if times.is_empty() => self.times("")?,
            None => return None,
        };

        let formula = match formula.contains("to") {
            true => RangeFormula::from_ranges(&formula).replace("times", &times.to_string()),
            false => formula,
        };

        let values = (1..=times)
            .map(|x| Expr::new(&formula.replace('x', &x.to_string())).eval())
            .collect::<Option<Vec<_>>>()?;

        for value in values {
            self.values.push(self.number(value));
        }

        Some(())
    }
}

/// Port of `string_to_formula`, which rewrites every `A to B` of a formula as
/// `(A)+((B)-(A))/(times-1)*(x-1)`, where `A` is the operand on the left of `to`
/// up to an unmatched `(`, and `B` the one on the right up to an unmatched `)`
pub(super) struct RangeFormula;

impl RangeFormula {
    pub(super) fn from_ranges(formula: &str) -> String {
        let mut value = formula.replace(' ', "");

        while let Some(index) = value.find("to") {
            let left = &value[..index];
            let right = &value[index + 2..];
            let start = Self::left_operand(left);
            let finish = Self::right_operand(right);

            if Expr::new(start).eval().is_none() {
                return formula.to_string();
            }

            let finish_value = match finish.contains("to") {
                true => {
                    let nested = Self::from_ranges(finish);

                    if nested == finish {
                        return formula.to_string();
                    }

                    nested
                }
                false if Expr::new(finish).eval().is_none() => return formula.to_string(),
                false => finish.to_string(),
            };

            let pattern = format!("{start}to{finish}");
            let replacement = format!("({start})+(({finish_value})-({start}))/(times-1)*(x-1)");
            let replaced = value.replace(&pattern, &replacement);

            if replaced == value {
                return formula.to_string();
            }

            value = replaced;
        }

        value
    }

    fn left_operand(left: &str) -> &str {
        let mut depth = 0;

        for (i, c) in left.char_indices().rev() {
            match c {
                ')' => depth += 1,
                '(' if depth == 0 => return &left[i + 1..],
                '(' => depth -= 1,
                _ => {}
            }
        }

        left
    }

    fn right_operand(right: &str) -> &str {
        let mut depth = 0;

        for (i, c) in right.char_indices() {
            match c {
                '(' => depth += 1,
                ')' if depth == 0 => return &right[..i],
                ')' => depth -= 1,
                _ => {}
            }
        }

        right
    }
}
