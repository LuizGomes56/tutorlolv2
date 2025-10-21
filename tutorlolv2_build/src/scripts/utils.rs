use regex::{Captures, Regex};
use serde::Deserialize;
use std::fmt;

pub fn add_f32_postfix<'a>(expr: &'a str) -> (String, bool) {
    let re_num = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();
    let postfixed = re_num.replace_all(expr, |caps: &Captures| format!("{}f32", &caps[1]));
    let uses_ctx = postfixed.contains("ctx.");
    (postfixed.into_owned(), uses_ctx)
}

#[derive(Deserialize)]
pub struct Positions {
    #[serde(default)]
    pub jungle: (Vec<String>, Vec<String>),
    #[serde(default)]
    pub top: (Vec<String>, Vec<String>),
    #[serde(default)]
    pub mid: (Vec<String>, Vec<String>),
    #[serde(default)]
    pub adc: (Vec<String>, Vec<String>),
    #[serde(default)]
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

pub fn is_valid_math_expression(input: &str) -> bool {
    let expr = input.trim();
    if expr.is_empty() {
        return false;
    }

    let expr_ns = expr
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let token_re = Regex::new(r"(?:ctx\.[a-z_][a-z0-9_]*)|\d+\.\d+|\d+|[+\-*/()]").unwrap();

    let tokens = token_re
        .find_iter(&expr_ns)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();
    if tokens.is_empty() {
        return false;
    }

    if tokens.concat() != expr_ns {
        return false;
    }

    if matches!(
        tokens.last().copied(),
        Some("+") | Some("-") | Some("*") | Some("/") | Some("(")
    ) {
        return false;
    }

    let mut depth = 0_i32;
    for t in &tokens {
        match *t {
            "(" => depth += 1,
            ")" => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    depth == 0
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
    let re = Regex::new(r"ctx\.[a-z_]+|[A-Z_]+|\d+\.\d+|\d+|[\+\-\*/\(\)]").unwrap();
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
