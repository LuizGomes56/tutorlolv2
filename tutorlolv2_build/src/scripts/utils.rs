use regex::{Captures, Regex};
use serde::Deserialize;
use std::fmt;

pub fn format_damage_type(damage_type: &str) -> String {
    let result = match damage_type {
        "PHYSICAL_DAMAGE" => "DamageType::Physical",
        "MAGIC_DAMAGE" => "DamageType::Magic",
        "MIXED_DAMAGE" => "DamageType::Mixed",
        "TRUE_DAMAGE" => "DamageType::True",
        "ADAPTATIVE_DAMAGE" => "DamageType::Adaptative",
        _ => "DamageType::Unknown",
    };
    result.to_string()
}

pub fn transform_expr(expr: &str) -> (String, bool) {
    let re_num = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();
    let with_f32 = re_num.replace_all(expr, |caps: &Captures| format!("{}f32", &caps[1]));
    let re_up = Regex::new(r"\b([A-Z][A-Z0-9_]*)\b").unwrap();
    let count_ctx = re_up.find_iter(with_f32.as_ref()).count();
    let result = re_up.replace_all(&with_f32, |caps: &Captures| format!("ctx.{}", &caps[1]));
    (result.into_owned(), count_ctx > 0)
}

#[derive(Deserialize)]
pub struct Positions {
    pub jungle: (Vec<String>, Vec<String>),
    pub top: (Vec<String>, Vec<String>),
    pub mid: (Vec<String>, Vec<String>),
    pub adc: (Vec<String>, Vec<String>),
    pub support: (Vec<String>, Vec<String>),
}

impl Positions {
    pub fn make_iterable(&self) -> [(Vec<String>, Vec<String>); 5] {
        [
            (self.top.0.clone(), self.top.1.clone()),
            (self.jungle.0.clone(), self.jungle.1.clone()),
            (self.mid.0.clone(), self.mid.1.clone()),
            (self.adc.0.clone(), self.adc.1.clone()),
            (self.support.0.clone(), self.support.1.clone()),
        ]
    }
}

#[macro_export]
macro_rules! cwd {
    ($path:expr) => {
        format!(
            "{}/../{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            $path
        )
    };
}

#[macro_export]
macro_rules! init_map {
    (file $type_name:ty, $path:literal) => {{
        let content = std::fs::read_to_string(cwd!($path)).unwrap();
        serde_json::from_str::<$type_name>(&content).unwrap()
    }};
    (dir $type_name:ty, $path:literal) => {{
        let mut map = BTreeMap::<String, $type_name>::new();
        if let Ok(dir) = std::fs::read_dir(cwd!($path)) {
            for entry in dir {
                let path = entry.unwrap().path();
                let file_stem = path.file_stem().unwrap();
                let file_name = file_stem.to_str().unwrap();
                let content = std::fs::read_to_string(&path).unwrap();
                let parsed = serde_json::from_str::<$type_name>(&content).unwrap();
                map.insert(file_name.to_owned(), parsed);
            }
        }
        if map.is_empty() {
            panic!("No files found in {}", $path);
        }
        map
    }};
}

pub fn is_valid_math_expression(expr: &str) -> bool {
    if expr.chars().any(|c| c.is_ascii_lowercase()) {
        return false;
    }
    let expr = expr.replace(' ', "");
    let valid_token_re = Regex::new(r"^[\d\.\+\-\*/\(\)A-Z_]+$").unwrap();
    if !valid_token_re.is_match(&expr) {
        return false;
    }

    let mut stack = vec![];
    for c in expr.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[derive(Debug, Clone)]
enum Expr {
    Num(String),
    Var(String),
    Op(Box<Expr>, char, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_prec(f, 0)
    }
}

impl Expr {
    fn fmt_with_prec(&self, f: &mut fmt::Formatter<'_>, parent_prec: u8) -> fmt::Result {
        let (prec, left, op, right) = match self {
            Expr::Op(left, op, right) => {
                let prec = match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    _ => 0,
                };
                (prec, left, op, right)
            }
            Expr::Num(s) | Expr::Var(s) => {
                return write!(f, "{}", s);
            }
        };

        let needs_parens = prec < parent_prec;

        if needs_parens {
            write!(f, "(")?;
        }

        left.fmt_with_prec(f, prec)?;
        write!(f, " {} ", op)?;
        right.fmt_with_prec(f, prec + 1)?;

        if needs_parens {
            write!(f, ")")?;
        }

        Ok(())
    }
}

fn parse_expression(tokens: &[String]) -> (Expr, usize) {
    parse_expression_prec(tokens, 0, 0)
}

fn parse_expression_prec(tokens: &[String], min_prec: u8, mut pos: usize) -> (Expr, usize) {
    let mut lhs = parse_primary(tokens, &mut pos);

    while pos < tokens.len() {
        let op = tokens[pos].chars().next().unwrap();
        let prec = match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => break,
        };

        if prec < min_prec {
            break;
        }

        pos += 1;
        let (rhs, new_pos) = parse_expression_prec(tokens, prec + 1, pos);
        pos = new_pos;
        lhs = Expr::Op(Box::new(lhs), op, Box::new(rhs));
    }

    (lhs, pos)
}

fn parse_primary(tokens: &[String], pos: &mut usize) -> Expr {
    let token = &tokens[*pos];
    *pos += 1;
    if token == "(" {
        let (expr, new_pos) = parse_expression(&tokens[*pos..]);
        *pos += new_pos + 1;
        expr
    } else if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
        Expr::Num(token.clone())
    } else {
        Expr::Var(token.clone())
    }
}

fn tokenize(expr: &str) -> Vec<String> {
    let re = Regex::new(r"[A-Z_]+|\d+\.\d+|\d+|[\+\-\*/\(\)]").unwrap();
    re.find_iter(expr).map(|m| m.as_str().to_string()).collect()
}

pub fn clean_math_expr(expr: &str) -> String {
    if expr.contains(".powf") {
        return expr.to_string();
    }
    if !is_valid_math_expression(expr) {
        return "0.0".to_string();
    }
    let tokens = tokenize(expr);
    let (parsed, _) = parse_expression(&tokens);
    parsed.to_string()
}

pub fn clear_suffixes(input: &str) -> String {
    let re_f32 = Regex::new(r"(\d+(?:\.\d+)?)(f32)").unwrap();
    let no_suffix = re_f32.replace_all(input, "$1");
    let re_decimal = Regex::new(r"\d+\.\d+").unwrap();
    re_decimal
        .replace_all(&no_suffix, |caps: &regex::Captures| {
            let full = &caps[0];
            if let Some((whole, decimal)) = full.split_once('.') {
                if decimal.chars().all(|c| c == '0') && decimal.len() <= 10 {
                    whole.to_string()
                } else {
                    full.to_string()
                }
            } else {
                full.to_string()
            }
        })
        .to_string()
}

pub trait Case {
    fn to_screaming_snake_case(&self) -> String;
}

impl Case for str {
    fn to_screaming_snake_case(&self) -> String {
        #[inline]
        fn is_boundary(c: char) -> bool {
            c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'')
        }
        #[inline]
        fn is_ignorable(c: char) -> bool {
            c == '\''
        }
        let mut out = String::with_capacity(self.len() * 2);
        let mut chars = self.chars().peekable();
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
}

pub trait JoinNumVec {
    fn join(&self, sep: &str) -> String;
}

macro_rules! join_num_vec_trait_impl {
    ($t:ty) => {
        impl<T: ToString + Copy> JoinNumVec for $t {
            fn join(&self, sep: &str) -> String {
                self.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(sep)
            }
        }
    };
}

join_num_vec_trait_impl!(Vec<T>);
join_num_vec_trait_impl!(&[T]);

pub const USE_SUPER: &str = "use super::*;";
