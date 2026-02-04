use crate::MayFail;
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use tutorlolv2_gen::eval::*;

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

    /// Replaces several string matches to values that are mathematically
    /// evaluable.
    /// For example "per Soul collected" becomes [`ThreshStacks`]
    fn replace_keys(&self) -> String;

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
    fn from_scaled_string(&self) -> String;
    fn process_linear_scalings(
        bounds: (f64, f64),
        size: usize,
        postfix: Option<String>,
    ) -> Vec<String>;
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
        let re = Regex::new(r"\d+(?:\s*/\s*\d+)+").unwrap();
        let mut nums = Vec::<f64>::new();

        for m in re.find_iter(self) {
            let slice = m.as_str();
            for part in slice.split('/') {
                if let Ok(n) = part.trim().parse::<f64>() {
                    nums.push(n);
                }
            }
        }

        nums
    }

    fn capture_numbers<T: FromStr>(&self) -> Vec<T> {
        Regex::new(r"\d+")
            .unwrap()
            .find_iter(self)
            .filter_map(|m| m.as_str().to_string().parse().ok())
            .collect()
    }

    fn replace_keys(&self) -> String {
        let mut replacements = Vec::<(&'static str, Box<dyn Display>)>::new();

        fn push_tuple(
            vector: &mut Vec<(&'static str, Box<dyn Display>)>,
            from: &'static str,
            to: impl Display + 'static,
        ) {
            vector.push((from, Box::new(to)));
        }

        macro_rules! tuple_rep {
            ($($from:literal => $to:expr),*$(,)?) => {
                $(push_tuple(&mut replacements, $from, $to);)*
            };
        }
        tuple_rep! {
            "per 100" => "0.01 *",
            "of damage dealt" => "100.0",
            "of damage stored" => "100.0",
            "of expended Grit" => "0.0",
            "of the original damage" => "100.0",
            "per Overwhelm stack on the target" => "1.0",
            "of primary target's bonus health" => EnemyBonusHealth,
            "of his bonus health" => BonusHealth,
            "Pantheon's bonus health" => BonusHealth,
            "bonus attack speed" => AttackSpeed,
            "based on critical strike chance" => CritChance,
            "critical strike chance" => CritChance,
            "of Ivern's AP" => AbilityPower,
            "of Sona's AP" => AbilityPower,
            "per Feast stack" => Stacks,
            "of Siphoning Strike stacks" => Stacks,
            "Stardust" => Stacks,
            "per Mark" => Stacks,
            "per mark" => Stacks,
            "bonus movement speed" => BonusMoveSpeed,
            "bonus mana" => BonusMana,
            "bonus AD" => BonusAd,
            "bonus armor" => BonusArmor,
            "bonus magic resistance" => BonusMagicResist,
            "bonus health" => BonusHealth,
            "of the target's maximum health" => EnemyMaxHealth,
            "of target's maximum health" => EnemyMaxHealth,
            "of the target's current health" => CurrentHealth,
            "of target's current health" => CurrentHealth,
            "target's current health" => CurrentHealth,
            "of the target's missing health" => MissingHealth,
            "of target's missing health" => MissingHealth,
            "target's missing health" => MissingHealth,
            "of Zac's maximum health" => MaxHealth,
            "of Braum's maximum health" => MaxHealth,
            "of her maximum health" => MaxHealth,
            "of his maximum health" => MaxHealth,
            "of maximum health" => MaxHealth,
            "maximum health" => MaxHealth,
            "maximum mana" => MaxMana,
            "armor" => Armor,
            "AP" => AbilityPower,
            "AD" => AttackDamage,
            "\u{00D7}" => "*"
        };

        replacements
            .into_iter()
            .fold(self.to_string(), |acc, (old, new)| {
                let pattern = format!(r"\b{}\b", regex::escape(old));
                let re = regex::Regex::new(&pattern).unwrap();
                re.replace_all(&acc, &new.to_string())
                    .replace("per Soul collected", &format!(" * {Stacks}"))
            })
    }

    fn remove_parenthesis(&self) -> String {
        Regex::new(r"\(\+\s*[^)]*\)")
            .unwrap()
            .replace_all(self, "")
            .to_string()
    }

    fn get_scalings(&self) -> String {
        let re = Regex::new(r"\(([^)]+)\)").unwrap();
        let mut result = Vec::<String>::new();
        for cap in re.captures_iter(self) {
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
        let re = Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").ok()?;
        let caps = re.captures(self)?;

        let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

        let first_is_percent = caps.get(2).is_some();
        let second_is_percent = caps.get(4).is_some();

        let denom1 = if first_is_percent { 100.0 } else { 1.0 };
        let denom2 = if second_is_percent { 100.0 } else { 1.0 };

        Some((first / denom1, second / denom2))
    }

    fn get_damage(&self) -> String {
        let re = Regex::new(r"\{\{as\|([^\}]+)\}\}").unwrap();
        let mut results = Vec::<String>::new();
        for cap in re.captures_iter(self) {
            let mut content = cap[1].to_string();
            let nested = Regex::new(r"\{\{[^}]+\|([^}]+)\}\}").unwrap();
            content = nested.replace_all(&content, "$1").to_string();
            results.push(content);
        }
        let scaled_input = results.join(" ").replace("{{as|", "");
        Self::from_scaled_string(&scaled_input)
    }

    fn from_scaled_string(&self) -> String {
        let re = Regex::new(r"\([^\)]*\)").unwrap();
        let paren_part = re.find(self).map(|m| m.as_str()).unwrap_or("");
        let base = self.replace(paren_part, "").trim().to_string();
        let scaled = paren_part.get_scalings();
        if !scaled.is_empty() {
            format!("{base} + {scaled}")
        } else {
            base
        }
    }

    fn process_linear_scalings(
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
}
