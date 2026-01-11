use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::fmt::Display;
use tutorlolv2_fmt::rust_html;

pub mod champions;
pub mod items;
pub mod model;
pub mod runes;

static RE_CAST_F32: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<before>^|[^.\d])(?P<num>\d+)(?P<after>[^.\d]|$)").unwrap());
static RE_DROP_F32: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+(?:\.\d+)?)(f32)").unwrap());
static RE_DROP_F32_DECIMAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+\.\d+|\d+").unwrap());
static RE_SIMPLIFY: Lazy<Regex> = Lazy::new(|| Regex::new(r"([-+]?\d*\.?\d+)").unwrap());

pub static TOWER_DAMAGE: &str = r#"const intrinsic TOWER_DAMAGE {
    damage_type: RiotFormulas::adaptative_type(
        bonus_stats.attack_damage,
        current_stats.ability_power,
    ),
    damage: intrinsic |plates, percent, flat| {
        base_stats.attack_damage
            + bonus_stats.attack_damage
            + current_stats.ability_power
                * 0.6
                * (100 / (140 + (-25 + 50 * plates) * percent - flat))
    }
}"#;

pub static ONHIT_EFFECT: &str = r#"const intrinsic ONHIT_EFFECT {
    damage_type: DamageType::Mixed,
    definition: |damage, [min, max], attr| match attr {
        Attrs::OnhitMin | Attrs::AreaOnhitMin => *min += damage,
        Attrs::OnhitMax | Attrs::AreaOnhitMax => *max += damage,
        Attrs::Onhit | Attrs::AreaOnhit => {
            *min += damage;
            *max += damage;
        }
    }
};"#;

pub static CRITICAL_STRIKE: &str = r#"const intrinsic CRITICAL_STRIKE {
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad * ctx.crit_damage / 100.0,
};"#;

pub static BASIC_ATTACK: &str = r#"const intrinsic BASIC_ATTACK {
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad,
};"#;

pub trait StringExt: AsRef<str> {
    fn rust_html(&self) -> String {
        rust_html(self.as_ref())
    }

    fn rust_fmt(&self) -> String {
        tutorlolv2_fmt::rustfmt(self.as_ref())
    }

    fn as_const(&self) -> String {
        self.as_ref()
            .replacen("class=\"type\"", "class=\"constant\"", 1)
    }

    fn to_ssnake(&self) -> String {
        fn is_boundary(c: char) -> bool {
            c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'')
        }
        fn is_ignorable(c: char) -> bool {
            c == '\''
        }
        let value = self.as_ref();
        let mut out = String::with_capacity(value.len() * 2);
        let mut chars = value.chars().peekable();
        let mut prev_was_alpha = false;
        let mut prev_was_upper = false;
        let mut prev_was_digit = false;
        while let Some(c) = chars.next() {
            if is_ignorable(c) {
                continue;
            }
            if is_boundary(c) {
                if !out.is_empty() && !out.ends_with('_') {
                    out.push('_');
                }
                prev_was_alpha = false;
                prev_was_upper = false;
                prev_was_digit = false;
                continue;
            }
            let is_upper = c.is_ascii_uppercase();
            let is_alpha = c.is_ascii_alphabetic();
            let is_digit = c.is_ascii_digit();
            let next_is_lower = chars
                .peek()
                .map(|n| n.is_ascii_lowercase())
                .unwrap_or(false);
            let need_us = if out.is_empty() || out.ends_with('_') {
                false
            } else if is_upper {
                (prev_was_alpha && !prev_was_upper)
                    || prev_was_digit
                    || (prev_was_upper && next_is_lower)
            } else if is_alpha {
                prev_was_digit
            } else if is_digit {
                prev_was_alpha
            } else {
                false
            };
            if need_us && !out.ends_with('_') {
                out.push('_');
            }
            out.push(c.to_ascii_uppercase());
            prev_was_alpha = is_alpha;
            prev_was_upper = is_upper;
            prev_was_digit = is_digit;
        }
        if out.ends_with('_') {
            out.pop();
        }
        out
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

    let values = values.iter().map(normalize_expr).collect::<Vec<_>>();

    if values.is_empty() {
        return Simplified::Unknown;
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

    let mut value_matrix = Vec::<Vec<f64>>::new();
    for s in values {
        let nums = RE_SIMPLIFY
            .find_iter(&s)
            .filter_map(|m| m.as_str().parse::<f64>().ok())
            .collect::<Vec<_>>();
        value_matrix.push(nums);
    }

    let num_constants = value_matrix[0].len();
    for row in &value_matrix {
        if row.len() != num_constants {
            return Simplified::Impossible;
        }
    }

    let mut depends_on_n = false;
    let mut formulas = Vec::new();

    for col in 0..num_constants {
        let v1 = value_matrix[0][col];
        let v2 = value_matrix[1][col];
        let diff = v2 - v1;

        match diff.abs() < 0.0001 {
            true => {
                formulas.push(v1.to_string());
            }
            false => {
                depends_on_n = true;
                let start_offset = v1 - diff;

                let pa = match start_offset.abs() < 0.0001 {
                    true => format!("({diff} * context_level)"),
                    false => format!("({start_offset} + {diff} * context_level)"),
                };
                formulas.push(pa);
            }
        }
    }

    let mut final_expression = template;
    for (i, formula) in formulas.iter().enumerate() {
        let marker = format!("[[{i}]]");
        final_expression = final_expression.replace(&marker, formula);
    }

    let result = final_expression
        .replace("+ -", "- ")
        .replace("( ", "(")
        .replace(" )", ")");

    match depends_on_n {
        true => Simplified::Progression(result),
        false => Simplified::Independent(result),
    }
}
