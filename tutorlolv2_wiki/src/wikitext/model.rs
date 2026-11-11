//! Formula-ready model of the abilities of a champion, built from the source of
//! their ability data pages. Each effect has a [`Formula`] tree that generators
//! combine structurally, and that displays as the `ctx.*` expressions of today

use {
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt,
        ops::{Add, Div, Mul},
    },
    tutorlolv2_types::{CtxVar, DamageType, Key},
};

/// Abilities of a champion, by key. The `Vec` is ordered by the page files
/// (`Q0`, `Q1`, …), so its index is the same `nth` that v1 generators use
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChampionModel {
    pub champion_id: String,
    pub abilities: BTreeMap<Key, Vec<AbilityModel>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AbilityModel {
    /// Page file the ability comes from, such as `Q1` for `abilities/Q1.lua`
    pub page: String,
    pub name: String,
    pub damage_type: DamageType,
    /// One per `{{st}}` label and value of the `leveling*` fields, in order
    pub effects: Vec<EffectModel>,
    /// Tables written with `{{pp}}` in the `description*` fields, such as the
    /// Flux bonus of Ryze's Overload, which depends on the rank of his R
    pub extras: Vec<EffectModel>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EffectModel {
    /// `{{st}}` label, such as `Magic Damage`. For extras, the field name
    pub label: String,
    /// Index of the field the effect comes from, `leveling` is 0, `leveling2` is 1
    pub section: u8,
    /// Text right after the base values, such as `%` or `seconds`
    pub unit: String,
    pub formula: Formula,
    /// Pieces of the value that could not be understood, as written. An effect
    /// with issues still has a formula, built from the parts that were understood
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Formula {
    Const(f64),
    /// `ctx.ability_power`, `ctx.enemy_max_health`, …
    Var(CtxVar),
    /// Values by the rank `rank` (`ctx.q_level` … `ctx.r_level`), from `{{ap}}`
    ByRank { rank: CtxVar, values: Vec<f64> },
    /// Values by `var` from `{{pp}}`: `keys[i]` is the value of `var` where
    /// `values[i]` starts. `var` is `ctx.level` unless the template names another
    /// axis, such as the rank of another ability
    Table {
        var: CtxVar,
        keys: Vec<u8>,
        values: Vec<f64>,
    },
    Sum(Vec<Formula>),
    Product(Vec<Formula>),
    Div(Box<Formula>, Box<Formula>),
    Pow(Box<Formula>, u8),
    /// Expression kept as written, from a `f![...]` override
    Raw(String),
}

/// Values of the variables a [`Formula`] is evaluated with
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sample(pub BTreeMap<CtxVar, f64>);

impl Sample {
    /// Value of `var`, or 0 if the sample does not have it
    pub fn get(&self, var: CtxVar) -> f64 {
        self.0.get(&var).copied().unwrap_or(0.0)
    }
}

impl Formula {
    /// Multiplies the formula by `factor`, written before it
    pub fn scale(self, factor: f64) -> Formula {
        Formula::Const(factor) * self
    }

    /// Value of the formula at `sample`. `None` for [`Formula::Raw`], which is not
    /// evaluated here
    pub fn eval(&self, sample: &Sample) -> Option<f64> {
        Some(match self {
            Formula::Const(value) => *value,
            Formula::Var(var) => sample.get(*var),
            Formula::ByRank { rank, values } => {
                let index = (sample.get(*rank) - 1.0).max(0.0) as usize;
                *values.get(index.min(values.len().checked_sub(1)?))?
            }
            Formula::Table { var, keys, values } => {
                let value = sample.get(*var);
                let index = keys
                    .iter()
                    .rposition(|key| f64::from(*key) <= value)
                    .unwrap_or(0);
                *values.get(index)?
            }
            Formula::Sum(terms) => terms
                .iter()
                .map(|term| term.eval(sample))
                .sum::<Option<f64>>()?,
            Formula::Product(factors) => factors
                .iter()
                .map(|factor| factor.eval(sample))
                .product::<Option<f64>>()?,
            Formula::Div(lhs, rhs) => lhs.eval(sample)? / rhs.eval(sample)?,
            Formula::Pow(base, power) => base.eval(sample)?.powi(i32::from(*power)),
            Formula::Raw(_) => return None,
        })
    }

    /// Whether the formula displays without parentheses around it
    fn is_atom(&self) -> bool {
        matches!(self, Formula::Const(_) | Formula::Var(_))
    }

    /// Writes the formula, in parentheses unless it is a constant or a variable
    fn fmt_operand(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_atom() {
            true => write!(f, "{self}"),
            false => write!(f, "({self})"),
        }
    }

    fn fmt_joined(f: &mut fmt::Formatter<'_>, operands: &[Formula], separator: &str) -> fmt::Result {
        for (i, operand) in operands.iter().enumerate() {
            if i != 0 {
                f.write_str(separator)?;
            }

            operand.fmt_operand(f)?;
        }

        Ok(())
    }
}

/// Number as written in formulas, such as `5.0` or `0.065`, without the noise of
/// float arithmetic, as in 0.30000000000000004
struct Number(f64);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", (self.0 * 1e10).round() / 1e10)
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Const(value) => write!(f, "{}", Number(*value)),
            Formula::Var(var) => write!(f, "{var}"),
            Formula::ByRank { values, .. } if values.len() == 1 => {
                write!(f, "{}", Number(values[0]))
            }
            Formula::ByRank { rank, values } => {
                let step = values[1] - values[0];
                let linear = values
                    .windows(2)
                    .all(|pair| (pair[1] - pair[0] - step).abs() < 1e-9);

                if linear {
                    return write!(
                        f,
                        "({}) + ({} * ({rank} - 1.0))",
                        Number(values[0]),
                        Number(step)
                    );
                }

                let last = values.len();
                let arms = values
                    .iter()
                    .enumerate()
                    .map(|(i, value)| match i + 1 {
                        1 => format!("..=1 => {}", Number(*value)),
                        n if n == last => format!("{n}.. => {}", Number(*value)),
                        n => format!("{n} => {}", Number(*value)),
                    })
                    .collect::<Vec<_>>();

                write!(f, "match {rank} {{{}}}", arms.join(", "))
            }
            Formula::Table { values, .. } if values.len() == 1 => {
                write!(f, "{}", Number(values[0]))
            }
            Formula::Table { var, keys, values } => {
                let last = values.len() - 1;
                let arms = values
                    .iter()
                    .enumerate()
                    .map(|(i, value)| match i {
                        0 => format!("..{} => {}", keys[1], Number(*value)),
                        i if i == last => format!("{}.. => {}", keys[i], Number(*value)),
                        i => format!("{}..{} => {}", keys[i], keys[i + 1], Number(*value)),
                    })
                    .collect::<Vec<_>>();

                write!(f, "match {var} {{ {} }}", arms.join(", "))
            }
            Formula::Sum(terms) => Self::fmt_joined(f, terms, " + "),
            Formula::Product(factors) => Self::fmt_joined(f, factors, " * "),
            Formula::Div(lhs, rhs) => write!(f, "({lhs}) / ({rhs})"),
            Formula::Pow(base, power) => {
                let factors = vec![base.as_ref().clone(); usize::from(*power)];
                Self::fmt_joined(f, &factors, " * ")
            }
            Formula::Raw(text) => write!(f, "({text})"),
        }
    }
}

impl Add for Formula {
    type Output = Formula;

    /// Sums are flattened, so `a + b + c` is a single [`Formula::Sum`]
    fn add(self, rhs: Formula) -> Formula {
        let mut terms = match self {
            Formula::Sum(terms) => terms,
            other => vec![other],
        };

        match rhs {
            Formula::Sum(rhs) => terms.extend(rhs),
            rhs => terms.push(rhs),
        }

        Formula::Sum(terms)
    }
}

impl Mul for Formula {
    type Output = Formula;

    /// Products are flattened, so `a * b * c` is a single [`Formula::Product`]
    fn mul(self, rhs: Formula) -> Formula {
        let mut factors = match self {
            Formula::Product(factors) => factors,
            other => vec![other],
        };

        match rhs {
            Formula::Product(rhs) => factors.extend(rhs),
            rhs => factors.push(rhs),
        }

        Formula::Product(factors)
    }
}

impl Div for Formula {
    type Output = Formula;

    fn div(self, rhs: Formula) -> Formula {
        Formula::Div(Box::new(self), Box::new(rhs))
    }
}

impl Add<f64> for Formula {
    type Output = Formula;

    fn add(self, rhs: f64) -> Formula {
        self + Formula::Const(rhs)
    }
}

impl Mul<f64> for Formula {
    type Output = Formula;

    fn mul(self, rhs: f64) -> Formula {
        self * Formula::Const(rhs)
    }
}

impl Div<f64> for Formula {
    type Output = Formula;

    fn div(self, rhs: f64) -> Formula {
        self / Formula::Const(rhs)
    }
}
