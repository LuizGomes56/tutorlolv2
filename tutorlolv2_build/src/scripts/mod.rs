use regex::{Captures, Regex};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    sync::LazyLock,
};
use tutorlolv2_fmt::rust_html;

pub mod champions;
pub mod items;
pub mod model;
pub mod runes;

static RE_CAST_F32: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<before>^|[^.\d])(?P<num>\d+)(?P<after>[^.\d]|$)").unwrap());
static RE_DROP_F32: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+(?:\.\d+)?)(f32)").unwrap());
static RE_DROP_F32_DECIMAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+\.\d+|\d+").unwrap());
static RE_SIMPLIFY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[-+]?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+)").unwrap());
static RE_IDENTS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"ctx\.([a-z_][a-z0-9_]*)").unwrap());

pub const ZERO_FN: &str = r#"fn zero(_: &Ctx) -> f32 {
    0.0 /* No damage */
}"#;

pub const DEFAULT_ITEM_GENERATOR: &str = r#"use super::*;

impl Generator for Item {
    fn generate(&mut self) -> MayFail {
        /* No implementation */
        self.infer_stats_ifdef().end()
    }
}"#;

pub const TOWER_DAMAGE: &str = r#"intrinsic TOWER_DAMAGE {
    damage_type: RiotFormulas::adaptive_type(
        bonus_stats.attack_damage,
        current_stats.ability_power,
    ),
    definition: const fn get_tower_damages(
        AdaptiveType, 
        ResistShred, 
        ...f32
    ) -> [i32; L_TWRD]
}"#;

pub const IGNITE_FN: &str = r#"fn ignite(level: i32) -> i32 {
    70 + 20 * level + 5 
      * if level > 4 { level - 4 } 
        else { 0 }
}"#;

pub const TOWER_DAMAGE_FN: &str = r#"fn tower_damage(_: f32, ...) -> i32 {
    let base = base_attack_damage 
        + bonus_attack_damage 
        + ability_power * 0.6;
    let bonus_resist = match plates == 0 {
        true => 0.0,
        false => -25 + 50 * (plates - 1),
    };
    let raw_resist = 40 + bonus_resist;
    let resist = raw_resist 
        * (1 - pen_percent / 100) 
        - pen_flat;
    let mult = 100 / (100 + resist);
    base * mult
}"#;

pub const ONHIT_EFFECT: &str = r#"intrinsic ONHIT_EFFECT {
    damage_type: Mixed,
    definition: const fn eval_attacks(
        &Ctx, 
        RangeDamage, 
        f32
    ) -> Attacks
};"#;

pub const ONHIT_EFFECT_FN: &str = r#"fn eval_attacks(
    ctx: &Ctx, 
    mut onhit_damage: RangeDamage, 
    physical_mod: f32
) -> Attacks {
    intrinsic
}"#;

pub const CRITICAL_STRIKE: &str = r#"intrinsic CRITICAL_STRIKE {
    attributes: Attrs::OnhitMax,
    damage_type: Physical,
    damage: |ctx| {
        ctx.ad * ctx.crit_damage / 100
    }
};"#;

pub const CRITICAL_STRIKE_FN: &str = r#"fn critical_strike(ctx: &Ctx) -> f32 {
    ctx.ad * ctx.crit_damage / 100
}"#;

pub const BASIC_ATTACK: &str = r#"intrinsic BASIC_ATTACK {
    attributes: Attrs::OnhitMin,
    damage_type: Physical,
    damage: |ctx| ctx.ad,
};"#;

pub const BASIC_ATTACK_FN: &str = r#"fn basic_attack(ctx: &Ctx) -> f32 {
    ctx.ad /* mul ctx.physical_mod */
}"#;

pub trait StringExt: AsRef<str> {
    fn get_idents(&self, damage_type: &str) -> BTreeSet<String> {
        RE_IDENTS
            .captures_iter(self.as_ref())
            .map(|cap| format!("CtxVar::{},", cap[1].pascal_case()))
            .chain(
                match damage_type {
                    "Physical" => Some("CtxVar::PhysicalMultiplier,"),
                    "Magic" => Some("CtxVar::MagicMultiplier,"),
                    _ => None,
                }
                .map(str::to_string),
            )
            .collect()
    }

    fn rust_html(&self) -> String {
        rust_html(self.as_ref())
    }

    fn rust_fmt(&self) -> String {
        tutorlolv2_fmt::rustfmt(self.as_ref(), Some(48))
    }

    fn as_const(&self) -> String {
        self.as_ref().replacen("class=\"_t\"", "class=\"_c\"", 1)
    }

    fn to_ssnake(&self) -> String {
        tutorlolv2_fmt::to_ssnake(self.as_ref())
    }

    fn pascal_case(&self) -> String {
        tutorlolv2_fmt::pascal_case(self.as_ref())
    }

    fn cast_f32(&self) -> String {
        let result = RE_CAST_F32.replace_all(self.as_ref(), |caps: &Captures| {
            let before = &caps["before"];
            let num = &caps["num"];
            let after = &caps["after"];

            format!("{before}{num}.0{after}")
        });

        result.into_owned()
    }

    fn ctx_param(&self) -> &'static str {
        if self.as_ref().contains("ctx.") {
            "ctx"
        } else {
            "_"
        }
    }

    fn clean(&self) -> String {
        let input = self.as_ref();
        let tokens = tokenize(input);
        match parse_expr(&tokens, 0) {
            Some((expr, _)) => format_expr(&expr, None, false),
            None => input.to_string(),
        }
    }

    fn drop_f32s(&self) -> String {
        let input = self.as_ref();
        let no_suffix = RE_DROP_F32.replace_all(input, "$1");

        RE_DROP_F32_DECIMAL
            .replace_all(&no_suffix, |caps: &regex::Captures| {
                let full = &caps[0];
                match full.parse::<f64>() {
                    Ok(num) => {
                        let rounded = (num * 1000.0).round() / 1000.0;
                        let mut s = rounded.to_string();
                        if s.ends_with(".0") {
                            s.truncate(s.len() - 2);
                        }
                        s
                    }
                    Err(_) => full.to_string(),
                }
            })
            .to_string()
    }
}

impl StringExt for str {}

#[derive(Debug, Clone)]
enum Expr {
    Term(String),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    fn is_associative(self) -> bool {
        matches!(self, Op::Add | Op::Mul)
    }
}

impl Expr {
    fn precedence(&self) -> u8 {
        match self {
            Expr::Term(_) => 3,
            Expr::Binary { op, .. } => op.precedence(),
        }
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut prev_was_op = true;

    while let Some(c) = chars.peek().copied() {
        match c {
            ' ' => {
                chars.next();
            }
            '(' | ')' | '+' | '*' | '/' => {
                tokens.push(c.to_string());
                prev_was_op = true;
                chars.next();
            }
            '-' => {
                chars.next();
                match prev_was_op {
                    true => {
                        let mut num = String::from("-");
                        while let Some(nc) = chars.peek() {
                            if !(nc.is_alphanumeric() || *nc == '.') {
                                break;
                            }
                            num.push(*nc);
                            chars.next();
                        }
                        tokens.push(num);
                        prev_was_op = false;
                    }
                    false => {
                        tokens.push("-".into());
                        prev_was_op = true;
                    }
                }
            }
            _ => {
                let mut term = String::new();
                while let Some(nc) = chars.peek() {
                    if !matches!(*nc, ' ' | '(' | ')' | '+' | '-' | '*' | '/') {
                        term.push(*nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(term);
                prev_was_op = false;
            }
        }
    }

    tokens
}

fn parse_expr(tokens: &[String], min_prec: u8) -> Option<(Expr, &[String])> {
    let (mut left, mut rest) = parse_primary(tokens)?;

    while let Some(op) = rest.first().and_then(parse_op) {
        if op.precedence() < min_prec {
            break;
        }

        let (right, next) = parse_expr(&rest[1..], op.precedence() + 1)?;
        left = Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
        rest = next;
    }

    Some((left, rest))
}

fn parse_primary(tokens: &[String]) -> Option<(Expr, &[String])> {
    let tok = tokens.first()?;

    if tok == "(" {
        let (expr, rest) = parse_expr(&tokens[1..], 0)?;
        if rest.first()? == ")" {
            return Some((expr, &rest[1..]));
        }
        None
    } else {
        Some((Expr::Term(tok.clone()), &tokens[1..]))
    }
}

fn parse_op(tok: &String) -> Option<Op> {
    match tok.as_str() {
        "+" => Some(Op::Add),
        "-" => Some(Op::Sub),
        "*" => Some(Op::Mul),
        "/" => Some(Op::Div),
        _ => None,
    }
}

const SIMPLIFY_EPS: f64 = 1e-10;

#[derive(Debug, Clone, Default)]
struct Poly {
    terms: BTreeMap<Vec<String>, f64>,
}

impl Poly {
    fn constant(value: f64) -> Self {
        let mut out = Self::default();
        if value.abs() > SIMPLIFY_EPS {
            out.terms.insert(Vec::new(), value);
        }
        out
    }

    fn variable(name: String) -> Self {
        let mut out = Self::default();
        out.terms.insert(vec![name], 1.0);
        out
    }

    fn normalize_key(mut vars: Vec<String>) -> Vec<String> {
        vars.sort();
        vars
    }

    fn clean(&mut self) {
        self.terms.retain(|_, value| value.abs() > SIMPLIFY_EPS);
    }

    fn add(mut self, other: Self) -> Self {
        for (key, value) in other.terms {
            *self.terms.entry(key).or_insert(0.0) += value;
        }
        self.clean();
        self
    }

    fn sub(mut self, other: Self) -> Self {
        for (key, value) in other.terms {
            *self.terms.entry(key).or_insert(0.0) -= value;
        }
        self.clean();
        self
    }

    fn mul(self, other: Self) -> Self {
        let mut out = BTreeMap::<Vec<String>, f64>::new();

        for (k1, c1) in self.terms {
            for (k2, c2) in &other.terms {
                let mut key = k1.clone();
                key.extend(k2.clone());
                let key = Self::normalize_key(key);
                *out.entry(key).or_insert(0.0) += c1 * *c2;
            }
        }

        let mut poly = Self { terms: out };
        poly.clean();
        poly
    }

    fn div(self, other: Self) -> Option<Self> {
        let value = other.as_constant()?;
        if value.abs() <= SIMPLIFY_EPS {
            return None;
        }

        let mut out = self;
        for coeff in out.terms.values_mut() {
            *coeff /= value;
        }
        out.clean();
        Some(out)
    }

    fn as_constant(&self) -> Option<f64> {
        if self.terms.is_empty() {
            return Some(0.0);
        }
        if self.terms.len() != 1 {
            return None;
        }
        let (key, value) = self.terms.iter().next()?;
        if key.is_empty() { Some(*value) } else { None }
    }

    fn render(&self) -> String {
        if self.terms.is_empty() {
            return "0".into();
        }

        let mut items = self
            .terms
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect::<Vec<_>>();

        items.sort_by(|(ka, _), (kb, _)| {
            let da = ka.len();
            let db = kb.len();
            da.cmp(&db).then_with(|| ka.cmp(kb))
        });

        let mut out = String::new();

        for (index, (vars, coeff)) in items.into_iter().enumerate() {
            let negative = coeff < 0.0;
            let abs = coeff.abs();

            let term = if vars.is_empty() {
                format_simplified_number(abs)
            } else if (abs - 1.0).abs() <= SIMPLIFY_EPS {
                vars.join(" * ")
            } else {
                format!("{} * {}", format_simplified_number(abs), vars.join(" * "))
            };

            if index == 0 {
                if negative {
                    out.push('-');
                }
                out.push_str(&term);
            } else {
                if negative {
                    out.push_str(" - ");
                } else {
                    out.push_str(" + ");
                }
                out.push_str(&term);
            }
        }

        out
    }
}

fn format_simplified_number(value: f64) -> String {
    let value = if value.abs() <= SIMPLIFY_EPS {
        0.0
    } else {
        value
    };
    let s = format!("{value:.6}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    match s {
        "" | "-0" => "0".into(),
        _ => s.to_string(),
    }
}

fn expr_to_poly(expr: &Expr) -> Option<Poly> {
    match expr {
        Expr::Term(term) => match term.parse::<f64>() {
            Ok(number) => Some(Poly::constant(number)),
            Err(_) => Some(Poly::variable(term.clone())),
        },
        Expr::Binary { left, op, right } => {
            let left = expr_to_poly(left)?;
            let right = expr_to_poly(right)?;

            match op {
                Op::Add => Some(left.add(right)),
                Op::Sub => Some(left.sub(right)),
                Op::Mul => Some(left.mul(right)),
                Op::Div => left.div(right),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct ExprCost {
    adds: usize,
    subs: usize,
    muls: usize,
    divs: usize,
}

impl ExprCost {
    fn score(self) -> usize {
        self.adds + self.subs + self.muls * 3 + self.divs * 6
    }
}

fn expr_cost(expr: &Expr) -> ExprCost {
    match expr {
        Expr::Term(_) => ExprCost::default(),
        Expr::Binary { left, op, right } => {
            let mut cost = expr_cost(left);
            let rhs = expr_cost(right);

            cost.adds += rhs.adds;
            cost.subs += rhs.subs;
            cost.muls += rhs.muls;
            cost.divs += rhs.divs;

            match op {
                Op::Add => cost.adds += 1,
                Op::Sub => cost.subs += 1,
                Op::Mul => cost.muls += 1,
                Op::Div => cost.divs += 1,
            }

            cost
        }
    }
}

fn parse_full_expr(input: &str) -> Option<Expr> {
    let tokens = tokenize(input);
    let (expr, rest) = parse_expr(&tokens, 0)?;
    if rest.is_empty() { Some(expr) } else { None }
}

fn simplify_formula(input: String) -> String {
    let Some(original_expr) = parse_full_expr(&input) else {
        return input;
    };

    let Some(poly) = expr_to_poly(&original_expr) else {
        return input;
    };

    let simplified = poly.render();

    let Some(simplified_expr) = parse_full_expr(&simplified) else {
        return input;
    };

    let original_cost = expr_cost(&original_expr);
    let simplified_cost = expr_cost(&simplified_expr);

    match simplified_cost.score().cmp(&original_cost.score()) {
        std::cmp::Ordering::Less => simplified,
        std::cmp::Ordering::Greater => input,
        std::cmp::Ordering::Equal => {
            if simplified.len() < input.len() {
                simplified
            } else {
                input
            }
        }
    }
}

fn format_expr(expr: &Expr, parent: Option<Op>, is_right: bool) -> String {
    let out = match expr {
        Expr::Term(t) => t.clone(),
        Expr::Binary { left, op, right } => {
            let l = format_expr(left, Some(*op), false);
            let r = format_expr(right, Some(*op), true);
            let sym = match op {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            };
            format!("{l} {sym} {r}")
        }
    };

    if needs_parens(expr, parent, is_right) {
        format!("({out})")
    } else {
        out
    }
}

fn needs_parens(expr: &Expr, parent: Option<Op>, is_right: bool) -> bool {
    let Some(p) = parent else { return false };

    let ep = expr.precedence();
    let pp = p.precedence();

    if ep < pp {
        return true;
    }

    if ep == pp && is_right && !p.is_associative() {
        return true;
    }

    false
}

pub enum Simplified {
    Unknown,
    Impossible,
    Progression(String),
    Independent(String),
    Unrecognized,
}

impl Simplified {
    pub const fn arm(&self) -> char {
        match self {
            Simplified::Progression(_) => 'n',
            _ => '_',
        }
    }

    pub fn expr(&self) -> String {
        match self {
            Simplified::Unknown => "unknown".into(),
            Simplified::Impossible => "impossible".into(),
            Simplified::Unrecognized => "unrecognized".into(),
            Simplified::Progression(s) => s.clean(),
            Simplified::Independent(s) => s.clean(),
        }
    }
}

impl Display for Simplified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arm = self.arm();
        let expr = self.expr();
        write!(f, "{arm} => {expr}")
    }
}

/// AI-generated
pub fn simplify(values: &[String]) -> Simplified {
    fn normalize_expr(expr: &String) -> String {
        let mut out = String::with_capacity(expr.len() + 8);
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if i + 4 <= chars.len()
                && chars[i] == 'c'
                && chars[i + 1] == 't'
                && chars[i + 2] == 'x'
                && chars[i + 3] == '.'
            {
                let mut j = out.chars().rev().skip_while(|c| c.is_whitespace());
                let needs_one = match j.next() {
                    None => true,
                    Some('*') => false,
                    Some(c) if c.is_ascii_digit() => false,
                    Some('.') => false,
                    _ => true,
                };
                if needs_one {
                    out.push_str("1.0 * ");
                }
                while i < chars.len() {
                    let c = chars[i];
                    out.push(c);
                    i += 1;
                    if !c.is_alphanumeric() && c != '_' && c != '.' {
                        break;
                    }
                }
                continue;
            }
            out.push(chars[i]);
            i += 1;
        }

        out
    }

    fn approx_eq_eps(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    fn decimal_places(s: &str) -> usize {
        s.split_once('.').map(|(_, frac)| frac.len()).unwrap_or(0)
    }

    fn infer_column_epsilon(column_text: &[String]) -> f64 {
        let max_dp = column_text
            .iter()
            .map(|s| decimal_places(s))
            .max()
            .unwrap_or(0);

        let base = 0.5 * 10f64.powi(-(max_dp as i32));

        match max_dp {
            0..=2 => base.max(0.01),
            3 => base.max(0.001),
            _ => base.max(0.0001),
        }
    }

    fn trim_f64(value: f64) -> String {
        let s = format!("{value:.6}");
        match s.split_once('.') {
            Some((int, frac)) => {
                let frac = frac.trim_end_matches('0');
                if frac.is_empty() {
                    return int.to_string();
                }
                format!("{int}.{frac}")
            }
            None => s,
        }
    }

    fn render_linear(intercept: f64, slope: f64, var: &str) -> String {
        let intercept_zero = intercept.abs() < 1e-10;
        let slope_zero = slope.abs() < 1e-10;

        match (intercept_zero, slope_zero) {
            (true, true) => "0".into(),
            (false, true) => trim_f64(intercept),
            (true, false) => format!("({} * {var})", trim_f64(slope)),
            (false, false) => format!("({} + {} * {var})", trim_f64(intercept), trim_f64(slope)),
        }
    }

    fn render_quadratic(a: f64, b: f64, c: f64, var: &str) -> String {
        if a.abs() < 1e-10 {
            return render_linear(c, b, var);
        }

        format!(
            "(({} * {var} + {}) * {var} + {})",
            trim_f64(a),
            trim_f64(b),
            trim_f64(c)
        )
    }

    fn fit_linear_ls(column: &[f64]) -> Option<(f64, f64)> {
        if column.len() < 2 {
            return None;
        }

        let n = column.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xx = 0.0;
        let mut sum_xy = 0.0;

        for (i, y) in column.iter().enumerate() {
            let x = (i + 1) as f64;
            sum_x += x;
            sum_y += *y;
            sum_xx += x * x;
            sum_xy += x * *y;
        }

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < 1e-12 {
            return None;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denom;
        let intercept = (sum_y - slope * sum_x) / n;

        Some((intercept, slope))
    }

    fn validate_linear(column: &[f64], intercept: f64, slope: f64, eps: f64) -> bool {
        column.iter().enumerate().all(|(i, actual)| {
            let x = (i + 1) as f64;
            let predicted = intercept + slope * x;
            approx_eq_eps(predicted, *actual, eps)
        })
    }

    fn snap_rational(value: f64, max_den: i64, eps: f64) -> Option<f64> {
        let mut best: Option<(f64, f64)> = None;

        for den in 1..=max_den {
            let num = (value * den as f64).round();
            let snapped = num / den as f64;
            let err = (snapped - value).abs();

            if err <= eps {
                match best {
                    None => best = Some((err, snapped)),
                    Some((best_err, _)) if err < best_err => best = Some((err, snapped)),
                    _ => {}
                }
            }
        }

        best.map(|(_, snapped)| snapped)
    }

    fn solve_3x3(mut m: [[f64; 4]; 3]) -> Option<(f64, f64, f64)> {
        for col in 0..3 {
            let mut pivot = col;
            for row in (col + 1)..3 {
                if m[row][col].abs() > m[pivot][col].abs() {
                    pivot = row;
                }
            }

            if m[pivot][col].abs() < 1e-12 {
                return None;
            }

            if pivot != col {
                m.swap(pivot, col);
            }

            let div = m[col][col];
            for k in col..4 {
                m[col][k] /= div;
            }

            for row in 0..3 {
                if row == col {
                    continue;
                }

                let factor = m[row][col];
                if factor.abs() < 1e-12 {
                    continue;
                }

                for k in col..4 {
                    m[row][k] -= factor * m[col][k];
                }
            }
        }

        Some((m[0][3], m[1][3], m[2][3]))
    }

    fn fit_quadratic_ls(column: &[f64]) -> Option<(f64, f64, f64)> {
        if column.len() < 3 {
            return None;
        }

        let mut n = 0.0;
        let mut sx = 0.0;
        let mut sx2 = 0.0;
        let mut sx3 = 0.0;
        let mut sx4 = 0.0;
        let mut sy = 0.0;
        let mut sxy = 0.0;
        let mut sx2y = 0.0;

        for (i, y) in column.iter().enumerate() {
            let x = (i + 1) as f64;
            let x2 = x * x;
            let x3 = x2 * x;
            let x4 = x2 * x2;

            n += 1.0;
            sx += x;
            sx2 += x2;
            sx3 += x3;
            sx4 += x4;
            sy += *y;
            sxy += x * *y;
            sx2y += x2 * *y;
        }

        solve_3x3([[sx4, sx3, sx2, sx2y], [sx3, sx2, sx, sxy], [sx2, sx, n, sy]])
    }

    fn validate_quadratic(column: &[f64], a: f64, b: f64, c: f64, eps: f64) -> bool {
        column.iter().enumerate().all(|(i, actual)| {
            let x = (i + 1) as f64;
            let predicted = (a * x + b) * x + c;
            approx_eq_eps(predicted, *actual, eps)
        })
    }

    fn try_soft_linear(column: &[f64], column_text: &[String], var: &str) -> Option<String> {
        let eps = infer_column_epsilon(column_text);

        let (raw_intercept, raw_slope) = fit_linear_ls(column)?;

        if !validate_linear(column, raw_intercept, raw_slope, eps) {
            return None;
        }

        let snapped_intercept = snap_rational(raw_intercept, 240, eps);
        let snapped_slope = snap_rational(raw_slope, 240, eps);

        if let (Some(i), Some(s)) = (snapped_intercept, snapped_slope) {
            if validate_linear(column, i, s, eps) {
                return Some(render_linear(i, s, var));
            }
        }

        Some(render_linear(raw_intercept, raw_slope, var))
    }

    fn try_soft_quadratic(column: &[f64], column_text: &[String], var: &str) -> Option<String> {
        let eps = infer_column_epsilon(column_text);

        let (raw_a, raw_b, raw_c) = fit_quadratic_ls(column)?;

        if !validate_quadratic(column, raw_a, raw_b, raw_c, eps) {
            return None;
        }

        let snapped_a = snap_rational(raw_a, 240, eps);
        let snapped_b = snap_rational(raw_b, 240, eps);
        let snapped_c = snap_rational(raw_c, 240, eps);

        if let (Some(a), Some(b), Some(c)) = (snapped_a, snapped_b, snapped_c) {
            if validate_quadratic(column, a, b, c, eps) {
                return Some(render_quadratic(a, b, c, var));
            }
        }

        Some(render_quadratic(raw_a, raw_b, raw_c, var))
    }

    let values = values.iter().map(normalize_expr).collect::<Vec<_>>();

    if values.is_empty() {
        return Simplified::Impossible;
    }
    if values.len() == 1 {
        return Simplified::Unrecognized;
    }

    let mut count = 0;
    let template = RE_SIMPLIFY
        .replace_all(&values[0], |_caps: &Captures| {
            let marker = format!("[[{count}]]");
            count += 1;
            marker
        })
        .into_owned();

    let mut text_matrix = Vec::<Vec<String>>::new();
    let mut value_matrix = Vec::<Vec<f64>>::new();

    for s in &values {
        let text_row = RE_SIMPLIFY
            .find_iter(s)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<_>>();

        let value_row = text_row
            .iter()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect::<Vec<_>>();

        text_matrix.push(text_row);
        value_matrix.push(value_row);
    }

    let num_constants = value_matrix[0].len();

    for row in &value_matrix {
        if row.len() != num_constants {
            return Simplified::Unknown;
        }
    }

    let mut depends_on_n = false;
    let mut formulas = Vec::new();

    for col in 0..num_constants {
        let column = value_matrix.iter().map(|row| row[col]).collect::<Vec<_>>();
        let column_text = text_matrix
            .iter()
            .map(|row| row[col].clone())
            .collect::<Vec<_>>();

        let eps = infer_column_epsilon(&column_text);

        let all_equal = column.windows(2).all(|w| approx_eq_eps(w[0], w[1], eps));
        if all_equal {
            formulas.push(trim_f64(column[0]));
            continue;
        }

        let diffs = column.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let first_diff = diffs[0];
        let is_linear = diffs.iter().all(|d| approx_eq_eps(*d, first_diff, eps));

        if is_linear {
            depends_on_n = true;
            let intercept = column[0] - first_diff;
            formulas.push(render_linear(intercept, first_diff, "context_level"));
            continue;
        }

        if let Some(expr) = try_soft_linear(&column, &column_text, "context_level") {
            depends_on_n = true;
            formulas.push(expr);
            continue;
        }

        if let Some(expr) = try_soft_quadratic(&column, &column_text, "context_level") {
            depends_on_n = true;
            formulas.push(expr);
            continue;
        }

        return Simplified::Unknown;
    }

    let mut final_expression = template;
    for (i, formula) in formulas.iter().enumerate() {
        let marker = format!("[[{i}]]");
        final_expression = final_expression.replace(&marker, formula);
    }

    let expr = final_expression
        .replace("+ -", "- ")
        .replace("( ", "(")
        .replace(" )", ")");

    let result = simplify_formula(expr);

    match depends_on_n {
        true => Simplified::Progression(result),
        false => Simplified::Independent(result),
    }
}

pub fn rustfmt_batch(snips: &[String]) -> Vec<String> {
    if snips.is_empty() {
        return Vec::new();
    }

    let mut input = String::new();
    input.push_str("// @generated\n\n");

    for (i, s) in snips.iter().enumerate() {
        input.push_str(&format!("//__SNIP_{i:06}__"));
        input.push('\n');
        input.push_str(
            &s.replace("ctx.", "")
                .replace("|ctx| ", "")
                .replace("(ctx: &Ctx) -> f32", "()"),
        );
        if !s.ends_with('\n') {
            input.push('\n');
        }
        input.push('\n');
    }

    let formatted = input.rust_fmt();

    let mut out = vec![String::new(); snips.len()];
    let mut cur = None;

    for line in formatted.lines() {
        if let Some(num) = line
            .strip_prefix("//__SNIP_")
            .and_then(|s| s.strip_suffix("__"))
        {
            cur = num.parse::<usize>().ok();
            continue;
        }

        if let Some(i) = cur {
            out[i].push_str(line);
            out[i].push('\n');
        }
    }

    for s in &mut out {
        while s.ends_with("\n\n") {
            s.pop();
        }
    }

    // #[cfg(debug_assertions)]
    // {
    //     let systime = SystemTime::now();
    //     let now = systime
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .unwrap()
    //         .as_nanos();
    //     let _ = std::fs::write(
    //         format!("batch_{now}.rs"),
    //         out.iter()
    //             .map(|v| v.as_bytes())
    //             .flatten()
    //             .copied()
    //             .collect::<Vec<_>>(),
    //     );
    // }

    out
}
