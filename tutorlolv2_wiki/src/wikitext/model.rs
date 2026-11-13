//! Formula-ready model of the abilities of a champion, built from the source of
//! their ability data pages. Each effect has a [`Formula`] tree that generators
//! combine structurally, and that displays as the `ctx.*` expressions of today

use {
    serde::{Deserialize, Serialize},
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt,
        ops::{Add, Div, Mul, Neg, Sub},
    },
    tutorlolv2_types::{CtxVar, DamageType, Key, MapMode},
};

/// Abilities of a champion, by key. The `Vec` is ordered by the page files
/// (`Q0`, `Q1`, …), preserving source order for repeated label selections.
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
    /// Visible description text by heading, including continuation fields.
    #[serde(default)]
    pub descriptions: BTreeMap<String, String>,
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
    ByRank {
        rank: CtxVar,
        values: Vec<f64>,
    },
    /// Values by `var` from `{{pp}}`: `keys[i]` is the value of `var` where
    /// `values[i]` starts. `var` is `ctx.level` unless the template names another
    /// axis, such as the rank of another ability
    Table {
        var: CtxVar,
        keys: Vec<u8>,
        values: Vec<f64>,
    },
    /// Bounded numeric axis. Keys retain their original units and precision.
    NumericTable {
        axis: Box<Formula>,
        keys: Vec<f64>,
        values: Vec<f64>,
        interpolated: bool,
    },
    Sum(Vec<Formula>),
    Product(Vec<Formula>),
    Div(Box<Formula>, Box<Formula>),
    /// `a - b`, as written in `f![…]`
    Sub(Box<Formula>, Box<Formula>),
    /// `-a`, as written in `f![…]`
    Neg(Box<Formula>),
    Pow(Box<Formula>, u8),
    /// Expression kept as written, from a `f![...]` override
    Raw(String),
    /// A value that game modes change: `default` everywhere else, and each arm's
    /// formula on its mode, at most one arm per mode
    ByMap {
        default: Box<Formula>,
        maps: Vec<(MapMode, Formula)>,
    },
}

/// Values of the variables a [`Formula`] is evaluated with, and the map
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sample {
    pub vars: BTreeMap<CtxVar, f64>,
    pub map: MapMode,
}

impl Sample {
    /// Sample with `vars`, on the default map
    pub fn new(vars: impl IntoIterator<Item = (CtxVar, f64)>) -> Self {
        Self {
            vars: vars.into_iter().collect(),
            map: MapMode::Default,
        }
    }

    /// The same sample on `map`
    pub fn on(self, map: MapMode) -> Self {
        Self { map, ..self }
    }

    /// Value of `var`, or 0 if the sample does not have it
    pub fn get(&self, var: CtxVar) -> f64 {
        self.vars.get(&var).copied().unwrap_or(0.0)
    }
}

impl Formula {
    /// Multiplies the formula by `factor`, written before it
    pub fn scale(self, factor: f64) -> Formula {
        Formula::Const(factor) * self
    }

    /// `self`, with `value` on `mode`. A [`Formula::ByMap`] gets an arm for `mode`, or
    /// its arm replaced, so both modes changing a value share one branch
    pub fn with_map(self, mode: MapMode, value: Formula) -> Formula {
        let (default, mut maps) = match self {
            Formula::ByMap { default, maps } => (default, maps),
            other => (Box::new(other), Vec::new()),
        };

        maps.retain(|(arm, _)| *arm != mode);
        maps.push((mode, value));
        maps.sort_by_key(|(arm, _)| *arm);

        Formula::ByMap { default, maps }
    }

    /// Values of a constant, of values by rank or of a table. For a value that game
    /// modes change, the values everywhere else
    pub fn default_values(&self) -> Option<Vec<f64>> {
        match self {
            Formula::Const(value) => Some(vec![*value]),
            Formula::ByRank { values, .. }
            | Formula::Table { values, .. }
            | Formula::NumericTable { values, .. } => Some(values.clone()),
            Formula::ByMap { default, .. } => default.default_values(),
            _ => None,
        }
    }

    /// `f` of the formula. When all of it is a [`Formula::ByMap`], `f` applies to its
    /// value everywhere and to each mode's, so it changes every branch alike
    pub fn edit_arms(self, f: impl Fn(Formula) -> Formula) -> Formula {
        match self {
            Formula::ByMap { default, maps } => Formula::ByMap {
                default: Box::new(f(*default)),
                maps: maps
                    .into_iter()
                    .map(|(mode, value)| (mode, f(value)))
                    .collect(),
            },
            other => f(other),
        }
    }

    /// Ratio of a value written in percent, as the 6-8% of Yorick's E. Values are
    /// divided where they are written, so `6%` displays as `0.06`
    pub fn percent(self) -> Formula {
        self.times(0.01)
    }

    /// The formula multiplied by `factor`, applied to the values themselves when it
    /// has them, so `0.2` times 3 displays as `0.6` rather than `3.0 * 0.2`
    pub fn times(self, factor: f64) -> Formula {
        let scale = |values: Vec<f64>| values.into_iter().map(|value| value * factor).collect();

        match self {
            Formula::Const(value) => Formula::Const(value * factor),
            Formula::ByRank { rank, values } => Formula::ByRank {
                rank,
                values: scale(values),
            },
            Formula::Table { var, keys, values } => Formula::Table {
                var,
                keys,
                values: scale(values),
            },
            Formula::NumericTable {
                axis,
                keys,
                values,
                interpolated,
            } => Formula::NumericTable {
                axis,
                keys,
                values: scale(values),
                interpolated,
            },
            Formula::ByMap { default, maps } => Formula::ByMap {
                default: Box::new(default.times(factor)),
                maps: maps
                    .into_iter()
                    .map(|(mode, value)| (mode, value.times(factor)))
                    .collect(),
            },
            other => other.scale(factor),
        }
    }

    /// Top-level terms: the operands of a sum, or the formula itself
    fn terms_mut(&mut self) -> Vec<&mut Formula> {
        match self {
            Formula::Sum(terms) => terms.iter_mut().collect(),
            other => vec![other],
        }
    }

    /// Ratio of each top-level term `ratio × stat`, or `ratio × stat / constant`, as
    /// `(+ 5% per 100 AP)`. Terms with another factor, as a ratio of a stat that
    /// grows with a second stat, have none
    pub fn stat_ratio_mut(&mut self, stat: CtxVar) -> Vec<&mut Formula> {
        self.terms_mut()
            .into_iter()
            .filter_map(|term| {
                // `ratio × stat / constant`, as `per 100`
                let term = match term {
                    Formula::Div(lhs, rhs) => match **rhs {
                        Formula::Const(_) => lhs.as_mut(),
                        _ => return None,
                    },
                    term => term,
                };

                match term {
                    Formula::Product(factors)
                        if factors.len() == 2 && factors[1] == Formula::Var(stat) =>
                    {
                        factors.first_mut()
                    }
                    _ => None,
                }
            })
            .collect()
    }

    /// Top-level term that is the base: the first values by rank, or else the first
    /// constant or table, or one of them that game modes change. Values by rank come
    /// first, as Azir's W rank base after its level table
    pub fn base_mut(&mut self) -> Option<&mut Formula> {
        let base = |term: &Formula| match term {
            Formula::ByMap { default, .. } => default.as_ref().clone(),
            term => term.clone(),
        };

        let terms = self.terms_mut();
        let index = terms
            .iter()
            .position(|term| matches!(base(term), Formula::ByRank { .. }))
            .or_else(|| {
                terms.iter().position(|term| {
                    matches!(
                        base(term),
                        Formula::Const(_) | Formula::Table { .. } | Formula::NumericTable { .. }
                    )
                })
            })?;

        terms.into_iter().nth(index)
    }

    /// Value of the formula at `sample`. `None` for [`Formula::Raw`], which is not
    /// evaluated here
    pub fn eval(&self, sample: &Sample) -> Option<f64> {
        Some(match self {
            Formula::Const(value) => *value,
            Formula::Var(var) => sample.get(*var),
            Formula::ByRank { rank, values } => match Self::linear_step(*rank, values) {
                // As displayed, so the printed formula has the same values
                Some(step) => values[0] + step * (sample.get(*rank) - 1.0),
                None => {
                    let index = (sample.get(*rank) - 1.0).max(0.0) as usize;
                    *values.get(index.min(values.len().checked_sub(1)?))?
                }
            },
            Formula::Table { var, keys, values } => {
                let value = sample.get(*var);
                let index = keys
                    .iter()
                    .rposition(|key| f64::from(*key) <= value)
                    .unwrap_or(0);
                *values.get(index)?
            }
            Formula::NumericTable {
                axis,
                keys,
                values,
                interpolated,
            } => {
                let x = axis.eval(sample)?;
                let i = keys.iter().rposition(|key| *key <= x).unwrap_or(0);
                let a = *values.get(i)?;

                if *interpolated && x > keys[i] && i + 1 < keys.len() {
                    a + (values[i + 1] - a) * (x - keys[i]) / (keys[i + 1] - keys[i])
                } else {
                    a
                }
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
            Formula::Sub(lhs, rhs) => lhs.eval(sample)? - rhs.eval(sample)?,
            Formula::Neg(value) => -value.eval(sample)?,
            Formula::Pow(base, power) => base.eval(sample)?.powi(i32::from(*power)),
            Formula::Raw(_) => return None,
            Formula::ByMap { default, maps } => maps
                .iter()
                .find(|(mode, _)| *mode == sample.map)
                .map_or(default.as_ref(), |(_, value)| value)
                .eval(sample)?,
        })
    }

    /// Step of `values` when they are linear and list every rank of `rank`, which
    /// displays as `(a) + (step * (rank - 1.0))`. Fewer values, such as 4 values of
    /// a Q, display as a `match` that keeps the last value for the ranks after them
    fn linear_step(rank: CtxVar, values: &[f64]) -> Option<f64> {
        let ranks = match rank {
            CtxVar::Level => 18,
            CtxVar::QLevel | CtxVar::WLevel | CtxVar::ELevel => 5,
            CtxVar::RLevel => 3,
            _ => values.len(),
        };

        let step = values.get(1)? - values.first()?;
        let linear = values
            .windows(2)
            .all(|pair| (pair[1] - pair[0] - step).abs() < 1e-9);

        (linear && values.len() >= ranks).then_some(step)
    }

    /// Whether the formula displays without parentheses around it
    fn is_atom(&self) -> bool {
        matches!(self, Formula::Const(_) | Formula::Var(_))
    }

    /// The formula written as the generated code evaluates it: as [`fmt::Display`],
    /// with game mode branches written in Rust, `tutorlolv2_types::MapMode::Aram =>
    /// { core::hint::cold_path(); … }`
    pub fn rust(&self) -> String {
        Written {
            formula: self,
            syntax: Syntax::Rust,
        }
        .to_string()
    }

    /// The formula on `mode`: each value that game modes change is its value on
    /// `mode`, or the one everywhere else
    pub fn on(&self, mode: MapMode) -> Formula {
        let on = |formula: &Formula| Box::new(formula.on(mode));
        let all = |formulas: &[Formula]| formulas.iter().map(|f| f.on(mode)).collect();

        match self {
            Formula::ByMap { default, maps } => maps
                .iter()
                .find(|(arm, _)| *arm == mode)
                .map_or(default.as_ref(), |(_, value)| value)
                .on(mode),
            Formula::NumericTable {
                axis,
                keys,
                values,
                interpolated,
            } => Formula::NumericTable {
                axis: on(axis),
                keys: keys.clone(),
                values: values.clone(),
                interpolated: *interpolated,
            },
            Formula::Sum(terms) => Formula::Sum(all(terms)),
            Formula::Product(factors) => Formula::Product(all(factors)),
            Formula::Div(lhs, rhs) => Formula::Div(on(lhs), on(rhs)),
            Formula::Sub(lhs, rhs) => Formula::Sub(on(lhs), on(rhs)),
            Formula::Neg(value) => Formula::Neg(on(value)),
            Formula::Pow(base, power) => Formula::Pow(on(base), *power),
            other => other.clone(),
        }
    }

    /// The formula with `with` wherever it reads `var`, as Sett's W with his current
    /// Grit instead of his maximum
    pub fn replace_var(&self, var: CtxVar, with: &Formula) -> Formula {
        let go = |formula: &Formula| Box::new(formula.replace_var(var, with));
        let all =
            |formulas: &[Formula]| formulas.iter().map(|f| f.replace_var(var, with)).collect();

        match self {
            Formula::Var(v) if *v == var => with.clone(),
            Formula::Table {
                var: axis,
                keys,
                values,
            } if *axis == var => Formula::NumericTable {
                axis: Box::new(with.clone()),
                keys: keys.iter().map(|key| f64::from(*key)).collect(),
                values: values.clone(),
                interpolated: false,
            },
            Formula::ByRank { rank, values } if *rank == var => {
                if let Formula::Const(value) = with {
                    self.eval(&Sample::new([(var, *value)]))
                        .map(Formula::Const)
                        .unwrap_or_else(|| self.clone())
                } else {
                    Formula::NumericTable {
                        axis: Box::new(with.clone()),
                        keys: (1..=values.len()).map(|i| i as f64).collect(),
                        values: values.clone(),
                        interpolated: Self::linear_step(*rank, values).is_some(),
                    }
                }
            }
            Formula::NumericTable {
                axis,
                keys,
                values,
                interpolated,
            } => Formula::NumericTable {
                axis: go(axis),
                keys: keys.clone(),
                values: values.clone(),
                interpolated: *interpolated,
            },
            Formula::Sum(terms) => Formula::Sum(all(terms)),
            Formula::Product(factors) => Formula::Product(all(factors)),
            Formula::Div(lhs, rhs) => Formula::Div(go(lhs), go(rhs)),
            Formula::Sub(lhs, rhs) => Formula::Sub(go(lhs), go(rhs)),
            Formula::Neg(value) => Formula::Neg(go(value)),
            Formula::Pow(base, power) => Formula::Pow(go(base), *power),
            Formula::ByMap { default, maps } => Formula::ByMap {
                default: go(default),
                maps: maps
                    .iter()
                    .map(|(mode, value)| (*mode, value.replace_var(var, with)))
                    .collect(),
            },
            other => other.clone(),
        }
    }

    /// Whether the formula is 0 everywhere, whatever its shape: `{{ap|0 to 0}}`, `0 * AP`,
    /// or values that game modes change to 0 on every mode. `Raw` is never zero
    pub fn is_zero(&self) -> bool {
        match self {
            Formula::Const(value) => *value == 0.0,
            Formula::ByRank { values, .. }
            | Formula::Table { values, .. }
            | Formula::NumericTable { values, .. } => values.iter().all(|value| *value == 0.0),
            Formula::Sum(terms) => terms.iter().all(Formula::is_zero),
            Formula::Product(factors) => factors.iter().any(Formula::is_zero),
            Formula::Div(lhs, _) | Formula::Neg(lhs) | Formula::Pow(lhs, _) => lhs.is_zero(),
            Formula::Sub(lhs, rhs) => lhs.is_zero() && rhs.is_zero(),
            Formula::ByMap { default, maps } => {
                default.is_zero() && maps.iter().all(|(_, value)| value.is_zero())
            }
            Formula::Var(_) | Formula::Raw(_) => false,
        }
    }

    /// Modes of every value that game modes change, anywhere in the formula
    pub fn modes(&self) -> BTreeSet<MapMode> {
        let mut modes = BTreeSet::new();
        self.collect_modes(&mut modes);
        modes
    }

    fn collect_modes(&self, modes: &mut BTreeSet<MapMode>) {
        match self {
            Formula::ByMap { default, maps } => {
                default.collect_modes(modes);

                for (mode, value) in maps {
                    modes.insert(*mode);
                    value.collect_modes(modes);
                }
            }
            Formula::Sum(formulas) | Formula::Product(formulas) => {
                formulas.iter().for_each(|f| f.collect_modes(modes))
            }
            Formula::Div(lhs, rhs) | Formula::Sub(lhs, rhs) => {
                lhs.collect_modes(modes);
                rhs.collect_modes(modes);
            }
            Formula::Neg(value)
            | Formula::Pow(value, _)
            | Formula::NumericTable { axis: value, .. } => value.collect_modes(modes),
            _ => {}
        }
    }

    /// Writes the formula in `syntax`, in parentheses unless it is an atom
    fn write_operand(&self, f: &mut fmt::Formatter<'_>, syntax: Syntax) -> fmt::Result {
        match self.is_atom() {
            true => self.write(f, syntax),
            false => {
                f.write_str("(")?;
                self.write(f, syntax)?;
                f.write_str(")")
            }
        }
    }

    fn write_joined(
        f: &mut fmt::Formatter<'_>,
        operands: &[Formula],
        separator: &str,
        syntax: Syntax,
    ) -> fmt::Result {
        for (i, operand) in operands.iter().enumerate() {
            if i != 0 {
                f.write_str(separator)?;
            }

            operand.write_operand(f, syntax)?;
        }

        Ok(())
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, syntax: Syntax) -> fmt::Result {
        match self {
            Formula::Const(value) => write!(f, "{}", Number(*value)),
            Formula::Var(var) => write!(f, "{var}"),
            Formula::ByRank { values, .. }
            | Formula::Table { values, .. }
            | Formula::NumericTable { values, .. }
                if values.len() <= 1 =>
            {
                write!(f, "{}", Number(values.first().copied().unwrap_or(0.0)))
            }
            Formula::ByRank { rank, values } => {
                if let Some(step) = Self::linear_step(*rank, values) {
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
            // Built only with a key per value, see `PassiveProgression::new`
            Formula::Table { keys, values, .. } if keys.len() != values.len() => {
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
            Formula::NumericTable {
                axis,
                keys,
                values,
                interpolated,
            } => {
                f.write_str("{ let axis: f32 = ")?;
                axis.write(f, syntax)?;
                f.write_str("; ")?;

                for i in 0..values.len().saturating_sub(1) {
                    write!(f, "if axis < {} {{ ", Number(keys[i + 1]))?;

                    if *interpolated {
                        write!(
                            f,
                            "{} + {} * (axis - {}).max(0.0) / {}",
                            Number(values[i]),
                            Number(values[i + 1] - values[i]),
                            Number(keys[i]),
                            Number(keys[i + 1] - keys[i])
                        )?;
                    } else {
                        write!(f, "{}", Number(values[i]))?;
                    }

                    f.write_str(" } else ")?;
                }

                write!(
                    f,
                    "{{ {} }} }}",
                    Number(values.last().copied().unwrap_or(0.0))
                )
            }
            Formula::Sum(terms) => Self::write_joined(f, terms, " + ", syntax),
            Formula::Product(factors) => Self::write_joined(f, factors, " * ", syntax),
            Formula::Div(lhs, rhs) => {
                f.write_str("(")?;
                lhs.write(f, syntax)?;
                f.write_str(") / (")?;
                rhs.write(f, syntax)?;
                f.write_str(")")
            }
            Formula::Sub(lhs, rhs) => {
                lhs.write_operand(f, syntax)?;
                f.write_str(" - ")?;
                rhs.write_operand(f, syntax)
            }
            Formula::Neg(value) => {
                f.write_str("-")?;
                value.write_operand(f, syntax)
            }
            Formula::Pow(base, power) => {
                let factors = vec![base.as_ref().clone(); usize::from(*power)];
                Self::write_joined(f, &factors, " * ", syntax)
            }
            Formula::Raw(text) => write!(f, "({text})"),
            Formula::ByMap { default, maps } => {
                f.write_str("match ctx.map { ")?;

                for (mode, value) in maps {
                    match syntax {
                        Syntax::Doc => write!(f, "#[cold] {mode:?} => ")?,
                        Syntax::Rust => write!(
                            f,
                            "tutorlolv2_types::MapMode::{mode:?} => {{ core::hint::cold_path(); "
                        )?,
                    }

                    value.write(f, syntax)?;

                    match syntax {
                        Syntax::Doc => f.write_str(", ")?,
                        Syntax::Rust => f.write_str(" } ")?,
                    }
                }

                f.write_str("_ => ")?;
                default.write(f, syntax)?;
                f.write_str(" }")
            }
        }
    }
}

/// Syntax a [`Formula`] is written in
#[derive(Clone, Copy)]
enum Syntax {
    /// The codec's, as `match ctx.map { #[cold] Aram => …, _ => … }`
    Doc,
    /// The generated code's
    Rust,
}

/// A [`Formula`] written in a [`Syntax`]
struct Written<'a> {
    formula: &'a Formula,
    syntax: Syntax,
}

impl fmt::Display for Written<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.formula.write(f, self.syntax)
    }
}

/// Number as written in formulas, such as `5.0` or `0.065`, without the noise of
/// float arithmetic, as in 0.30000000000000004
struct Number(f64);

impl fmt::Display for Number {
    /// Rounded to 10 decimals and written without an exponent, as `0.00006`, with a
    /// `.0` on whole numbers so the generated code reads it as a float
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = ((self.0 * 1e10).round() / 1e10).to_string();

        match text.contains('.') {
            true => f.write_str(&text),
            false => write!(f, "{text}.0"),
        }
    }
}

impl fmt::Display for Formula {
    /// Writes the formula in the codec's syntax
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f, Syntax::Doc)
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

impl Sub for Formula {
    type Output = Formula;

    fn sub(self, rhs: Formula) -> Formula {
        Formula::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Sub<f64> for Formula {
    type Output = Formula;

    fn sub(self, rhs: f64) -> Formula {
        self - Formula::Const(rhs)
    }
}

impl Neg for Formula {
    type Output = Formula;

    /// `-a`, with constants and double negations folded: `-(-0.3)` is `0.3`
    fn neg(self) -> Formula {
        match self {
            Formula::Const(value) => Formula::Const(-value),
            Formula::Neg(value) => *value,
            other => Formula::Neg(Box::new(other)),
        }
    }
}

/// Values that are a [`Formula`] in `f![…]`: numbers, variables and formulas
pub trait IntoFormula {
    fn to_formula(&self) -> Formula;
}

impl IntoFormula for f64 {
    fn to_formula(&self) -> Formula {
        Formula::Const(*self)
    }
}

impl IntoFormula for i32 {
    fn to_formula(&self) -> Formula {
        Formula::Const(f64::from(*self))
    }
}

impl IntoFormula for CtxVar {
    fn to_formula(&self) -> Formula {
        Formula::Var(*self)
    }
}

impl IntoFormula for Formula {
    fn to_formula(&self) -> Formula {
        self.clone()
    }
}

impl<T: IntoFormula + ?Sized> IntoFormula for &T {
    fn to_formula(&self) -> Formula {
        T::to_formula(*self)
    }
}
