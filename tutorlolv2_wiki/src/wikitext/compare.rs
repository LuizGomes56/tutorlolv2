//! Compares the formulas of [`ChampionModel`] with the current ones, parsed from the
//! rendered html into `abilities/<page>.json`. Two formulas are equal when they agree
//! at every [`Sample::series`], so the order of their terms does not matter

use {
    super::{ChampionModel, MapChangesReport, Sample},
    crate::{JsonRead, MayFail, champions::abilities::WikiAbility},
    serde::Serialize,
    std::{collections::BTreeMap, path::Path, str::FromStr},
    strum::VariantArray,
    tutorlolv2_types::{CtxVar, Key},
};

/// A formula of the current output, such as
/// `(match ctx.level { ..6 => 0.3, 16.. => 0.6 }) * ctx.bonus_move_speed`
pub struct LegacyFormula(Expr);

enum Expr {
    Number(f64),
    Var(CtxVar),
    Neg(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
    Match(CtxVar, Vec<(Pattern, Expr)>),
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
    /// Reads numbers, `ctx.*` variables, `+ - * /`, parentheses and `match` tables.
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

impl Expr {
    fn eval(&self, sample: &Sample) -> f64 {
        match self {
            Expr::Number(value) => *value,
            Expr::Var(var) => sample.get(*var),
            Expr::Neg(expr) => -expr.eval(sample),
            Expr::Binary(op, lhs, rhs) => {
                let (lhs, rhs) = (lhs.eval(sample), rhs.eval(sample));

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    _ => lhs / rhs,
                }
            }
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

        if self.eat("(") {
            let expr = self.sum()?;
            return self.eat(")").then_some(expr);
        }

        if self.eat("match") {
            return self.matching();
        }

        if self.eat("ctx.") {
            return Some(Expr::Var(self.var()?));
        }

        self.number().map(Expr::Number)
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

/// How the model compares to the current formulas
#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub equal: usize,
    pub different: usize,
    /// Current formulas that are missing or can't be read
    pub unreadable: usize,
    /// Effects of the current output that the model does not have
    pub only_current: usize,
    /// Effects of the model that the current output does not have
    pub only_model: usize,
    /// Equal effects among the ones present in both
    pub percent_equal: f64,
    pub differences: Vec<Difference>,
    /// What happened to the changes of the game modes, from `model_map_changes.json`
    pub map_changes: MapChangesReport,
}

/// An effect whose formulas differ
#[derive(Debug, Serialize)]
pub struct Difference {
    pub champion_id: String,
    pub key: Key,
    /// Page file the ability comes from, such as `Q1`
    pub page: String,
    pub label: String,
    pub current: String,
    pub model: String,
    pub issues: Vec<String>,
}

impl Report {
    /// Relative error within which two values are the same. The wiki rounds values
    /// to 2 decimals, as Maokai's 166.67 against 33.3325 per rank, which is far
    /// below this
    const TOLERANCE: f64 = 1e-3;

    /// Whether two series of values are the same, within [`Self::TOLERANCE`].
    /// Values that are not finite in both series are the same
    pub fn equal(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len()
            && a.iter()
                .zip(b)
                .all(|(a, b)| match (a.is_finite(), b.is_finite()) {
                    (true, true) => {
                        (a - b).abs() <= Self::TOLERANCE * 1f64.max(a.abs()).max(b.abs())
                    }
                    (false, false) => true,
                    _ => false,
                })
    }

    /// Labels are compared without the en dash the current parser replaces
    fn normalize(label: &str) -> String {
        label.replace('–', "-")
    }

    /// Pairs every effect of `<root>/cache/wiki/champions/<id>/abilities/<page>.json`
    /// with the first effect of the same label of the same page in `model.json`
    pub fn new(root: &Path) -> MayFail<Self> {
        let champions = root.join("cache/wiki/champions");
        let models = serde_json::from_slice::<BTreeMap<String, ChampionModel>>(&crate::read(
            champions.join("model.json"),
        )?)?;

        let samples = Sample::series();
        let mut report = Report::default();

        for (champion_id, model) in &models {
            let pages = model
                .abilities
                .iter()
                .flat_map(|(key, abilities)| abilities.iter().map(move |ability| (key, ability)))
                .map(|(key, ability)| (ability.page.as_str(), (*key, ability)))
                .collect::<BTreeMap<_, _>>();

            let dir = champions.join(champion_id).join("abilities");

            for entry in crate::read_dir(&dir)? {
                let path = entry.path();

                if path.extension().is_none_or(|ext| ext != "json") {
                    continue;
                }

                let page = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let current = serde_json::from_slice::<WikiAbility>(&crate::read(&path)?)?;

                let Some((key, ability)) = pages.get(page.as_str()) else {
                    report.only_current += current.effects.len();
                    continue;
                };

                let find = |label: &str| {
                    ability
                        .effects
                        .iter()
                        .find(|effect| Self::normalize(&effect.label) == Self::normalize(label))
                };

                report.only_model += ability
                    .effects
                    .iter()
                    .filter(|effect| {
                        !current
                            .effects
                            .keys()
                            .any(|label| Self::normalize(label) == Self::normalize(&effect.label))
                    })
                    .count();

                for (label, effect) in &current.effects {
                    let Some(new) = find(label) else {
                        report.only_current += 1;
                        continue;
                    };

                    let Some(old) = effect.formula.as_deref().and_then(LegacyFormula::parse) else {
                        report.unreadable += 1;
                        continue;
                    };

                    let old_values = samples.iter().map(|s| old.eval(s)).collect::<Vec<_>>();
                    let new_values = samples
                        .iter()
                        .map(|s| new.formula.eval(s).unwrap_or(f64::NAN))
                        .collect::<Vec<_>>();

                    if Self::equal(&old_values, &new_values) {
                        report.equal += 1;
                        continue;
                    }

                    report.different += 1;
                    report.differences.push(Difference {
                        champion_id: champion_id.clone(),
                        key: *key,
                        page: page.clone(),
                        label: label.clone(),
                        current: effect.formula.clone().unwrap_or_default(),
                        model: new.formula.to_string(),
                        issues: new.issues.clone(),
                    });
                }
            }
        }

        report.differences.sort_by(|a, b| {
            (&a.champion_id, &a.page, &a.label).cmp(&(&b.champion_id, &b.page, &b.label))
        });

        let compared = report.equal + report.different;

        report.percent_equal = match compared {
            0 => 0.0,
            _ => report.equal as f64 / compared as f64 * 100.0,
        };

        let map_changes = champions.join("model_map_changes.json");

        if let Ok(true) = std::fs::exists(&map_changes) {
            report.map_changes = MapChangesReport::from_file(map_changes)?;
        }

        Ok(report)
    }
}

/// Writes `cache/wiki/champions/model_report.json` and prints its counts
pub fn compare_model() -> MayFail {
    println!("[fn] wikitext::compare::compare_model");

    let report = Report::new(Path::new(""))?;

    println!(
        "[report] equal {}, different {}, unreadable {}, only current {}, only model {} ({:.1}% equal)",
        report.equal,
        report.different,
        report.unreadable,
        report.only_current,
        report.only_model,
        report.percent_equal
    );

    println!(
        "[report] map changes: {} applied, {} not placed",
        report.map_changes.applied, report.map_changes.not_placed
    );

    crate::write(
        "cache/wiki/champions/model_report.json",
        serde_json::to_string_pretty(&report)?,
    )
}
