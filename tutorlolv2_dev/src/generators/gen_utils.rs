use crate::MayFail;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::{fmt::Display, str::FromStr};
use tutorlolv2_gen::eval::CtxVar::*;

static CAPTURE_NUMBERS_SLASH_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\d+(?:\s*/\s*\d+)+").unwrap());
static CAPTURE_NUMBERS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+(?:\.\d+)?").unwrap());
static REPLACE_PERCENTAGES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+(?:\.\d+)?)%").unwrap());
static REMOVE_PARENTHESIS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(\+\s*[^)]*\)").unwrap());
static GET_SCALINGS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(([^)]+)\)").unwrap());
static GET_INTERVAL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").unwrap());
static GET_DAMAGE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{as\|([^\}]+)\}\}").unwrap());
static GET_DAMAGE_NESTED_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{[^}]+\|([^}]+)\}\}").unwrap());
static CLEAN_FORMULA_TOKEN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ctx\.[a-zA-Z_][a-zA-Z0-9_]*|\d+(?:\.\d+)?|[()+\-*/]").unwrap());
static FROM_SCALED_STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\([^\)]*\)").unwrap());

static REPLACEMENTS_MAP: [(&str, &(dyn Display + Send + Sync)); 45] = [
    ("per 100", &"0.01 *"),
    ("of damage dealt", &"100.0"),
    ("of damage stored", &"100.0"),
    ("of expended Grit", &"0.0"),
    ("of the original damage", &"100.0"),
    ("per Overwhelm stack on the target", &"1.0"),
    ("of primary target's bonus health", &EnemyBonusHealth),
    ("of his bonus health", &BonusHealth),
    ("Pantheon's bonus health", &BonusHealth),
    ("bonus attack speed", &AttackSpeed),
    ("based on critical strike chance", &CritChance),
    ("critical strike chance", &CritChance),
    ("of Ivern's AP", &AbilityPower),
    ("of Sona's AP", &AbilityPower),
    ("per Feast stack", &Stacks),
    ("of Siphoning Strike stacks", &Stacks),
    ("Stardust", &Stacks),
    ("per Mark", &Stacks),
    ("per mark", &Stacks),
    ("bonus movement speed", &BonusMoveSpeed),
    ("bonus mana", &BonusMana),
    ("bonus AD", &BonusAd),
    ("bonus armor", &BonusArmor),
    ("bonus magic resistance", &BonusMagicResist),
    ("bonus health", &BonusHealth),
    ("of the target's maximum health", &EnemyMaxHealth),
    ("of target's maximum health", &EnemyMaxHealth),
    ("of the target's current health", &EnemyCurrentHealth),
    ("of target's current health", &EnemyCurrentHealth),
    ("target's current health", &EnemyCurrentHealth),
    ("of the target's missing health", &MissingHealth),
    ("of target's missing health", &MissingHealth),
    ("target's missing health", &MissingHealth),
    ("of Zac's maximum health", &MaxHealth),
    ("of Braum's maximum health", &MaxHealth),
    ("of her maximum health", &MaxHealth),
    ("of his maximum health", &MaxHealth),
    ("of maximum health", &MaxHealth),
    ("maximum health", &MaxHealth),
    ("maximum mana", &MaxMana),
    ("armor", &Armor),
    ("AP", &AbilityPower),
    ("base AD", &BaseAd),
    ("AD", &AttackDamage),
    ("\u{00D7}", &"*"),
];

pub trait F64Ext {
    /// Removes the `.0` or any other fractional part
    /// and transforms the float number to a string
    fn trim(&self) -> String;
}

impl F64Ext for f64 {
    fn trim(&self) -> String {
        match self.fract() {
            0.0 => format!("{:.0}", self),
            _ => self.to_string(),
        }
    }
}

/// Provides several methods to extract data from strings using Regex patterns,
/// specially inside generator functions
pub trait RegExtractor {
    /// Capture numbers separated by a `/` symbol.
    /// ```txt
    /// 100 / 200 / 300 / 400 -> [100.0, 200.0, 300.0, 400.0]
    /// ```
    /// Any data that can't be parsed to a `f64` will be ignored
    fn capture_numbers_slash(&self) -> Vec<f64>;

    /// Capture the numbers that are followed by a `%` symbol
    /// ```txt
    /// 100% -> 100.0
    /// 200% -> 200.0
    /// 300% -> 300.0
    /// ```
    fn capture_percent(&self, number: usize) -> MayFail<f64>;

    /// Returns a vector of all the numbers that could be extracted
    /// from some string, preserving the order that they were found
    fn capture_numbers<T: FromStr>(&self) -> Vec<T>;

    /// Captures only the numbers inside a set of parenthesis. Parameter
    /// `number` indicates how many appearences of the regex match to skip
    /// ```txt
    /// (100%) -> 100.0
    /// (+ 84 * ATTACK_DAMAGE) -> 84.0
    /// ```
    fn capture_parens(&self, number: usize) -> MayFail<String>;
    fn mul(&self, value: f64) -> String;
    fn half(&self) -> String;

    /// Replaces several string matches to values that are mathematically
    /// evaluable.
    /// For example "per Soul collected" becomes [`ThreshStacks`]
    fn replace_keys(&self) -> String;
    fn replace_percentages(&self) -> String;

    /// Removes all the parentheses from a string
    fn remove_parenthesis(&self) -> String;

    /// Extracts tuples of scalings from a string`
    /// ```txt
    /// (+ 80% AP) -> (0.8 * ctx.ap)
    /// ```
    fn get_scalings(&self) -> String;

    /// Finds a match of `:` separated pairs of numbers and returns a
    /// tuple if that value was found. This is useful for extracting
    /// passive damage intervals
    /// ```txt
    /// 200 : 380 -> (200.0, 380.0)
    /// ```
    fn get_interval(&self) -> Option<(f64, f64)>;

    fn get_damage(&self) -> String;
    fn clean_formula(&self) -> String;
    fn from_scaled_string(&self) -> String;
}

impl RegExtractor for str {
    fn capture_parens(&self, number: usize) -> MayFail<String> {
        Ok(
            Regex::new(&format!(r"^(?:.*?(\([^()]*\))){{{}}}", number + 1))
                .unwrap()
                .captures(self)
                .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                .ok_or(format!(
                    "There are no parenthesis in #{number} for '{self}'"
                ))?
                .to_string(),
        )
    }

    fn capture_percent(&self, number: usize) -> MayFail<f64> {
        Ok(Regex::new(&format!(r"^(?:.*?(\d+)%){{{}}}", number + 1))?
            .captures(self)
            .and_then(|cap| cap.get(1).map(|m| m.as_str()))
            .ok_or(format!("No percent value in #{number} for '{self}'"))?
            .parse::<f64>()
            .map_err(|e| format!("Unable to convert all numbers to f64: {e:?}"))?)
    }

    fn capture_numbers_slash(&self) -> Vec<f64> {
        let mut nums = Vec::<f64>::new();

        for m in CAPTURE_NUMBERS_SLASH_RE.find_iter(self) {
            let slice = m.as_str();
            for part in slice.split('/') {
                if let Ok(n) = part.trim().parse::<f64>() {
                    nums.push(n);
                }
            }
        }

        nums
    }

    fn mul(&self, value: f64) -> String {
        format!("{value} * ({self})")
    }

    fn half(&self) -> String {
        self.mul(0.5)
    }

    fn capture_numbers<T: FromStr>(&self) -> Vec<T> {
        CAPTURE_NUMBERS_RE
            .find_iter(self)
            .filter_map(|m| m.as_str().parse().ok())
            .collect()
    }

    fn replace_keys(&self) -> String {
        REPLACEMENTS_MAP
            .into_iter()
            .fold(self.to_string(), |acc, (old, new)| {
                let pattern = format!(r"\b{}\b", regex::escape(old));
                regex::Regex::new(&pattern)
                    .unwrap()
                    .replace_all(&acc, &new.to_string())
                    .replace("per Soul collected", &format!(" * {Stacks}"))
                    .replace_percentages()
            })
    }

    fn replace_percentages(&self) -> String {
        REPLACE_PERCENTAGES_RE
            .replace_all(self, |caps: &Captures| {
                let num: f64 = caps[1].parse().unwrap();
                format!("{} *", num / 100.0)
            })
            .to_string()
    }

    fn remove_parenthesis(&self) -> String {
        REMOVE_PARENTHESIS_RE.replace_all(self, "").to_string()
    }

    fn get_scalings(&self) -> String {
        let mut result = Vec::<String>::new();
        for cap in GET_SCALINGS_RE.captures_iter(self) {
            let content = cap[1].trim();
            if content.to_lowercase().contains("based on level") {
                continue;
            }
            let cleaned = content.trim_start_matches('+').trim();
            let parts = cleaned.split_whitespace().collect::<Vec<&str>>();
            if parts.len() >= 2 && parts[0].ends_with('%') {
                if let Ok(percent) = parts[0].trim_end_matches('%').parse::<f64>() {
                    let decimal = percent / 100.0;
                    let rest = parts[1..].join(" ");
                    result.push(format!("({decimal} * {rest})"));
                    continue;
                }
            }
            result.push(format!("({cleaned})"));
        }
        result
            .into_iter()
            .map(|value| value.replace_keys())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn get_interval(&self) -> Option<(f64, f64)> {
        let caps = GET_INTERVAL_RE.captures(self)?;

        let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

        let first_is_percent = caps.get(2).is_some();
        let second_is_percent = caps.get(4).is_some();

        let denom1 = if first_is_percent { 100.0 } else { 1.0 };
        let denom2 = if second_is_percent { 100.0 } else { 1.0 };

        Some((first / denom1, second / denom2))
    }

    fn get_damage(&self) -> String {
        let mut results = Vec::<String>::new();
        for cap in GET_DAMAGE_RE.captures_iter(self) {
            let mut content = cap[1].to_string();
            content = GET_DAMAGE_NESTED_RE.replace_all(&content, "$1").to_string();
            results.push(content);
        }
        let scaled_input = results.join(" ").replace("{{as|", "").replace("'''", "");
        Self::from_scaled_string(&scaled_input)
            .replace_keys()
            .clean_formula()
    }

    fn clean_formula(&self) -> String {
        #[derive(Debug, Clone, PartialEq)]
        enum Token {
            Value(String),
            Op(String),
            LParen,
            RParen,
        }

        let tokens: Vec<Token> = CLEAN_FORMULA_TOKEN_RE
            .find_iter(self)
            .map(|m| {
                let s = m.as_str();
                match s {
                    "(" => Token::LParen,
                    ")" => Token::RParen,
                    "+" | "-" | "*" | "/" => Token::Op(s.to_string()),
                    _ => Token::Value(s.to_string()),
                }
            })
            .collect();

        let mut out: Vec<Token> = Vec::new();
        let mut expect_value = true;
        let mut open_parens = 0usize;

        for token in tokens {
            match token {
                Token::Value(v) => {
                    if !expect_value {
                        out.push(Token::Op("+".to_string()));
                    }
                    out.push(Token::Value(v));
                    expect_value = false;
                }

                Token::LParen => {
                    if !expect_value {
                        out.push(Token::Op("+".to_string()));
                    }
                    out.push(Token::LParen);
                    open_parens += 1;
                    expect_value = true;
                }

                Token::RParen => {
                    if open_parens > 0 && !expect_value {
                        out.push(Token::RParen);
                        open_parens -= 1;
                        expect_value = false;
                    }
                }

                Token::Op(op) => {
                    if expect_value {
                        if op == "-" || op == "+" {
                            out.push(Token::Op(op));
                        }
                    } else {
                        out.push(Token::Op(op));
                        expect_value = true;
                    }
                }
            }
        }

        while matches!(out.last(), Some(Token::Op(_))) {
            out.pop();
        }

        while open_parens > 0 {
            out.push(Token::RParen);
            open_parens -= 1;
        }

        if out.is_empty() {
            return "0".to_string();
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum TokenKind {
            Value,
            Op,
            LParen,
            RParen,
        }

        trait AsRefToken {
            fn kind(&self) -> TokenKind;
            fn text(&self) -> &str;
        }

        impl AsRefToken for Token {
            fn kind(&self) -> TokenKind {
                match self {
                    Token::Value(_) => TokenKind::Value,
                    Token::Op(_) => TokenKind::Op,
                    Token::LParen => TokenKind::LParen,
                    Token::RParen => TokenKind::RParen,
                }
            }

            fn text(&self) -> &str {
                match self {
                    Token::Value(v) => v,
                    Token::Op(op) => op,
                    Token::LParen => "(",
                    Token::RParen => ")",
                }
            }
        }

        fn render_tokens(tokens: &[impl AsRefToken]) -> String {
            let mut result = String::new();
            let mut prev: Option<TokenKind> = None;

            for token in tokens {
                let kind = token.kind();
                let text = token.text();

                let needs_space = matches!(
                    (prev, kind),
                    (Some(TokenKind::Value), TokenKind::Value)
                        | (Some(TokenKind::Value), TokenKind::Op)
                        | (Some(TokenKind::Value), TokenKind::LParen)
                        | (Some(TokenKind::Op), TokenKind::Value)
                        | (Some(TokenKind::Op), TokenKind::LParen)
                        | (Some(TokenKind::RParen), TokenKind::Value)
                        | (Some(TokenKind::RParen), TokenKind::Op)
                        | (Some(TokenKind::RParen), TokenKind::LParen)
                );

                if needs_space {
                    result.push(' ');
                }

                result.push_str(text);
                prev = Some(kind);
            }

            result
        }

        render_tokens(&out)
    }

    fn from_scaled_string(&self) -> String {
        let paren_part = FROM_SCALED_STRING_RE
            .find(self)
            .map(|m| m.as_str())
            .unwrap_or("");
        let base = self.replace(paren_part, "").trim().to_string();
        let scaled = paren_part.get_scalings();
        if !scaled.is_empty() {
            format!("{base} + {scaled}")
        } else {
            base
        }
    }
}

pub fn process_linear_scalings(
    bounds: (f64, f64),
    size: usize,
    postfix: Option<String>,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let (start, end) = bounds;
    for i in 0..size {
        let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
        if let Some(ref postfix) = postfix {
            result.push(format!("({value}{postfix})"));
            continue;
        }
        result.push(value.to_string());
    }
    result
}

impl<T: AsRef<str>> RegExtractor for T {
    fn capture_numbers_slash(&self) -> Vec<f64> {
        self.as_ref().capture_numbers_slash()
    }
    fn capture_percent(&self, number: usize) -> MayFail<f64> {
        self.as_ref().capture_percent(number)
    }
    fn capture_numbers<U: FromStr>(&self) -> Vec<U> {
        self.as_ref().capture_numbers::<U>()
    }
    fn capture_parens(&self, number: usize) -> MayFail<String> {
        self.as_ref().capture_parens(number)
    }
    fn mul(&self, value: f64) -> String {
        self.as_ref().mul(value)
    }
    fn half(&self) -> String {
        self.as_ref().half()
    }
    fn replace_keys(&self) -> String {
        self.as_ref().replace_keys()
    }
    fn replace_percentages(&self) -> String {
        self.as_ref().replace_percentages()
    }
    fn remove_parenthesis(&self) -> String {
        self.as_ref().remove_parenthesis()
    }
    fn get_scalings(&self) -> String {
        self.as_ref().get_scalings()
    }
    fn get_interval(&self) -> Option<(f64, f64)> {
        self.as_ref().get_interval()
    }
    fn get_damage(&self) -> String {
        self.as_ref().get_damage()
    }
    fn clean_formula(&self) -> String {
        self.as_ref().clean_formula()
    }
    fn from_scaled_string(&self) -> String {
        self.as_ref().from_scaled_string()
    }
}
