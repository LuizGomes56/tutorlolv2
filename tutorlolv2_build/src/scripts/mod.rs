use regex::{Captures, Regex};
use std::fmt;
use tutorlolv2_fmt::rust_html;

pub mod champions;
pub mod items;
pub mod model;
pub mod runes;

pub static USE_SUPER: &str = "use super::*;";

pub static TOWER_DAMAGE: &str = r#"intrinsic const TOWER_DAMAGE {
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

pub static ONHIT_EFFECT: &str = r#"intrinsic const ONHIT_EFFECT {
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

pub static CRITICAL_STRIKE: &str = r#"intrinsic const CRITICAL_STRIKE {
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad * ctx.crit_damage / 100.0,
};"#;

pub static BASIC_ATTACK: &str = r#"intrinsic const BASIC_ATTACK {
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    damage: |ctx| ctx.ad,
};"#;

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
                return write!(f, "{s}");
            }
        };

        let needs_parens = prec < parent_prec;

        if needs_parens {
            write!(f, "(")?;
        }

        left.fmt_with_prec(f, prec)?;
        write!(f, " {op} ")?;
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

pub trait StringExt: AsRef<str> {
    fn tokenize(&self) -> Vec<String> {
        Regex::new(r"ctx\.[a-z_]+|[A-Z_]+|\d+\.\d+|\d+|[\+\-\*/\(\)]")
            .unwrap()
            .find_iter(self.as_ref())
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn rust_html(&self) -> String {
        rust_html(self.as_ref())
    }

    fn drop_f32s(&self) -> String {
        let input = self.as_ref();
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

    fn rust_fmt(&self, width: usize) -> String {
        tutorlolv2_fmt::rustfmt(self.as_ref(), width)
    }

    fn as_const(&self) -> String {
        self.as_ref()
            .replacen("class=\"type\"", "class=\"constant\"", 1)
    }

    fn to_ssnake(&self) -> String {
        let expr = self.as_ref();
        let mut out = String::with_capacity(expr.len() * 2);
        let mut chars = expr.chars().peekable();
        let mut prev_was_alpha = false;
        let mut prev_was_upper = false;
        let mut prev_was_digit = false;
        while let Some(c) = chars.next() {
            if c == '\'' {
                continue;
            }
            if c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'') {
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

    fn normalize(&self) -> String {
        tutorlolv2_fmt::normalize(self.as_ref())
    }

    fn is_math_expr(&self) -> bool {
        let input = self.as_ref();
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
        if matches!(
            tokens.first().copied(),
            Some("+") | Some("-") | Some("*") | Some("/") | Some(")")
        ) {
            return false;
        }
        let is_op = |t: &str| matches!(t, "+" | "-" | "*" | "/");
        for w in tokens.windows(2) {
            let (a, b) = (w[0], w[1]);
            if is_op(a) && b == ")" {
                return false;
            }
            if a == "(" && is_op(b) {
                return false;
            }
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

    fn as_closure(&self) -> String {
        let expr = self.as_ref();
        if !self.is_math_expr() {
            return "0.0".to_string();
        }
        let tokens = expr.tokenize();
        let (parsed, _) = parse_expression(&tokens);
        parsed.to_string()
    }

    fn add_f32s(&self) -> String {
        let re_num = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();
        let postfixed =
            re_num.replace_all(self.as_ref(), |caps: &Captures| format!("{}f32", &caps[1]));
        postfixed.to_lowercase()
    }

    fn ctx_param(&self) -> &'static str {
        let expr = self.as_ref();
        if expr.contains("ctx.") { "ctx" } else { "_" }
    }
}

impl StringExt for str {
    fn to_ssnake(&self) -> String {
        fn is_boundary(c: char) -> bool {
            c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\'')
        }
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
