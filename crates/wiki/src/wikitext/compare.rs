//! Evaluates generated Rust formula expressions for validation.

use {
    super::{Formula, Sample},
    std::str::FromStr,
    strum::VariantArray,
    types::{CtxVar, MapMode},
};

/// A formula of the current output, such as
/// `(match ctx.level { ..6 => 0.3, 16.. => 0.6 }) * ctx.bonus_move_speed`
pub struct LegacyFormula(Expr);

enum Expr {
    Number(f64),
    Var(CtxVar),
    Neg(Box<Expr>),
    MaxZero(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
    Match(CtxVar, Vec<(Pattern, Expr)>),
    /// `match ctx.map { … }`: the arm of each mode, and the default
    MapMatch(Vec<(MapMode, Expr)>, Box<Expr>),
}

/// Arm of a `match`: `..=a`, `..a`, `a..b`, `a..=b`, `a` or `a..`
struct Pattern {
    low: Option<f64>,
    high: Option<f64>,
    inclusive: bool,
}

impl Pattern {
    fn matches(&self, value: f64) -> bool {
        let above = self.low.is_none_or(|low| value >= low);
        let below = match (self.high, self.inclusive) {
            (None, _) => true,
            (Some(high), true) => value <= high,
            (Some(high), false) => value < high,
        };

        above && below
    }
}

impl LegacyFormula {
    /// Reads numbers, `ctx.*` variables, `+ - * /`, parentheses, `match` tables
    /// and the lower-bound `.max(0.0)` used by inferred level growth.
    /// `None` for anything else
    pub fn parse(src: &str) -> Option<Self> {
        let mut parser = Parser { src, pos: 0 };
        let expr = parser.sum()?;

        parser.skip();
        (parser.pos == src.len()).then_some(Self(expr))
    }

    pub fn eval(&self, sample: &Sample) -> f64 {
        self.0.eval(sample)
    }
}

impl Formula {
    /// Reads the same expression syntax as legacy validation, rejecting constructs
    /// which cannot be represented faithfully by the structured formula model.
    pub fn parse_expression(source: &str) -> crate::MayFail<Self> {
        LegacyFormula::parse(source)
            .ok_or_else(|| format!("[formula] Unsupported expression: {source}"))?
            .0
            .into_formula()
            .ok_or_else(|| format!("[formula] Unsupported expression tree: {source}").into())
    }
}

impl Expr {
    fn into_formula(self) -> Option<Formula> {
        Some(match self {
            Self::Number(value) => Formula::Const(value),
            Self::Var(var) => Formula::Var(var),
            Self::Neg(value) => -value.into_formula()?,
            Self::Binary(op, lhs, rhs) => {
                let (lhs, rhs) = (lhs.into_formula()?, rhs.into_formula()?);
                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    _ => return None,
                }
            }
            Self::MapMatch(arms, default) => Formula::ByMap {
                default: Box::new(default.into_formula()?),
                maps: arms
                    .into_iter()
                    .map(|(mode, value)| Some((mode, value.into_formula()?)))
                    .collect::<Option<_>>()?,
            },
            // A Table is contiguous with constant arms. General expression matches
            // are not representable by it, and must not silently become raw code.
            Self::Match(_, _) | Self::MaxZero(_) => return None,
        })
    }
}

impl Expr {
    fn eval(&self, sample: &Sample) -> f64 {
        match self {
            Expr::Number(value) => *value,
            Expr::Var(var) => sample.get(*var),
            Expr::Neg(expr) => -expr.eval(sample),
            Expr::MaxZero(expr) => expr.eval(sample).max(0.0),
            Expr::Binary(op, lhs, rhs) => {
                let (lhs, rhs) = (lhs.eval(sample), rhs.eval(sample));

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    _ => lhs / rhs,
                }
            }
            Expr::MapMatch(arms, default) => arms
                .iter()
                .find(|(mode, _)| *mode == sample.map)
                .map_or(default.as_ref(), |(_, value)| value)
                .eval(sample),
            Expr::Match(var, arms) => {
                let value = sample.get(*var);

                arms.iter()
                    .find(|(pattern, _)| pattern.matches(value))
                    .map_or(0.0, |(_, expr)| expr.eval(sample))
            }
        }
    }
}

struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn rest(&self) -> &str {
        &self.src[self.pos..]
    }

    fn skip(&mut self) {
        self.pos = self.src.len() - self.rest().trim_start().len();
    }

    fn eat(&mut self, token: &str) -> bool {
        self.skip();
        let found = self.rest().starts_with(token);

        if found {
            self.pos += token.len();
        }

        found
    }

    fn sum(&mut self) -> Option<Expr> {
        let mut lhs = self.product()?;

        loop {
            let op = match () {
                _ if self.eat("+") => '+',
                // `-` of a sum, but not the `..` of a pattern
                _ if self.eat("-") => '-',
                _ => return Some(lhs),
            };

            lhs = Expr::Binary(op, Box::new(lhs), Box::new(self.product()?));
        }
    }

    fn product(&mut self) -> Option<Expr> {
        let mut lhs = self.unary()?;

        loop {
            let op = match () {
                _ if self.eat("*") => '*',
                _ if self.eat("/") => '/',
                _ => return Some(lhs),
            };

            lhs = Expr::Binary(op, Box::new(lhs), Box::new(self.unary()?));
        }
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.eat("-") {
            return Some(Expr::Neg(Box::new(self.unary()?)));
        }

        let mut value = if self.eat("(") {
            let expr = self.sum()?;
            self.eat(")").then_some(expr)?
        } else if self.eat("match") {
            self.matching()?
        } else if self.eat("ctx.") {
            Expr::Var(self.var()?)
        } else {
            Expr::Number(self.number()?)
        };

        while self.eat(".max") {
            if !self.eat("(") || self.number()? != 0.0 || !self.eat(")") {
                return None;
            }

            value = Expr::MaxZero(Box::new(value));
        }

        Some(value)
    }

    fn var(&mut self) -> Option<CtxVar> {
        let name = self
            .rest()
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .next()?;

        let var = CtxVar::from_str(name).ok()?;
        self.pos += name.len();

        Some(var)
    }

    fn number(&mut self) -> Option<f64> {
        self.skip();

        // `1..` is the number 1 followed by a range, not `1.` and `.`
        let rest = self.rest();
        let mut end = 0;

        for (i, c) in rest.char_indices() {
            match c {
                '0'..='9' => end = i + 1,
                '.' if !rest[i..].starts_with("..") => end = i + 1,
                _ => break,
            }
        }

        let value = rest[..end].parse().ok()?;
        self.pos += end;

        Some(value)
    }

    /// `match ctx.var { pattern => expr, … }`, after `match`
    fn matching(&mut self) -> Option<Expr> {
        if !self.eat("ctx.") {
            return None;
        }

        if self.rest().starts_with("map") {
            self.pos += "map".len();
            return self.map_matching();
        }

        let var = self.var()?;

        if !self.eat("{") {
            return None;
        }

        let mut arms = Vec::new();

        while !self.eat("}") {
            let pattern = self.pattern()?;

            if !self.eat("=>") {
                return None;
            }

            arms.push((pattern, self.sum()?));
            self.eat(",");
        }

        Some(Expr::Match(var, arms))
    }

    /// Arms of `match ctx.map { … }`, after `match ctx.map`, as the generated code
    /// writes them (`types::MapMode::Aram => { core::hint::cold_path(); … }`)
    /// or as the codec does (`#[cold] Aram => …`)
    fn map_matching(&mut self) -> Option<Expr> {
        if !self.eat("{") {
            return None;
        }

        let mut arms = Vec::new();
        let mut default = None;

        while !self.eat("}") {
            self.eat("#[cold]");

            if self.eat("_") {
                if !self.eat("=>") {
                    return None;
                }

                default = Some(self.map_arm()?);
                self.eat(",");
                continue;
            }

            self.eat("types::");
            self.eat("MapMode::");
            self.skip();

            let name = self
                .rest()
                .split(|c: char| !c.is_ascii_alphanumeric())
                .next()?;

            let mode = MapMode::from_str(name).ok()?;
            self.pos += name.len();

            if !self.eat("=>") {
                return None;
            }

            arms.push((mode, self.map_arm()?));
            self.eat(",");
        }

        Some(Expr::MapMatch(arms, Box::new(default?)))
    }

    /// Value of a map arm: an expression, or a block that may start with
    /// `core::hint::cold_path();`
    fn map_arm(&mut self) -> Option<Expr> {
        if !self.eat("{") {
            return self.sum();
        }

        self.eat("core::hint::cold_path();");
        let value = self.sum()?;
        self.eat("}").then_some(value)
    }

    fn pattern(&mut self) -> Option<Pattern> {
        self.skip();

        let low = match self.rest().starts_with("..") {
            true => None,
            false => Some(self.number()?),
        };

        if !self.eat("..") {
            // A single value
            return Some(Pattern {
                low,
                high: low,
                inclusive: true,
            });
        }

        // `..=` is written together, unlike the `=>` that may follow `a..`
        let inclusive = self.rest().starts_with('=') && !self.rest().starts_with("=>");

        if inclusive {
            self.pos += 1;
        }

        self.skip();

        let high = match self.rest().starts_with("=>") {
            true => None,
            false => Some(self.number()?),
        };

        Some(Pattern {
            low,
            high,
            inclusive,
        })
    }
}

impl Sample {
    /// Number of samples formulas are compared at
    const SERIES_LEN: usize = 32;

    /// Deterministic samples where levels are integers in their range (level 1–18,
    /// Q/W/E 1–5, R 1–3) and every other variable is from 0 to 1000
    pub fn series() -> Vec<Sample> {
        let mut state = 0x9E37_79B9_7F4A_7C15_u64;
        let mut random = move || {
            // xorshift64
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64
        };

        (0..Self::SERIES_LEN)
            .map(|_| {
                Sample::new(
                    CtxVar::VARIANTS
                        .iter()
                        .map(|&var| {
                            let mut integer =
                                |max: f64| 1.0 + (random() * max).floor().min(max - 1.0);

                            let value = match var {
                                CtxVar::Level => integer(18.0),
                                CtxVar::QLevel | CtxVar::WLevel | CtxVar::ELevel => integer(5.0),
                                CtxVar::RLevel => integer(3.0),
                                // In game ranges, as a share of the critical strike chance
                                // grows with what items add to the critical damage
                                CtxVar::CritChance => random() * 100.0,
                                CtxVar::CritDamage => 200.0 + random() * 150.0,
                                _ => random() * 1000.0,
                            };

                            (var, value)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
}

/// Whether sampled values agree within wiki rounding precision.
pub fn equal(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(a, b)| match (a.is_finite(), b.is_finite()) {
                (true, true) => (a - b).abs() <= 1e-3 * 1f64.max(a.abs()).max(b.abs()),
                (false, false) => true,
                _ => false,
            })
}
