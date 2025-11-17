use crate::MayFail;
use regex::Regex;
use std::fmt::Display;
use tutorlolv2_gen::eval::*;

pub trait F64Ext {
    fn trim(&self) -> String;
}

impl F64Ext for f64 {
    fn trim(&self) -> String {
        if self.fract() == 0.0 {
            format!("{:.0}", self)
        } else {
            format!("{}", self)
        }
    }
}

pub trait RegExtractor {
    fn capture_percent(&self, number: usize) -> MayFail<f64>;
    fn capture_numbers(&self) -> Vec<String>;
    fn capture_parens(&self, number: usize) -> MayFail<String>;
    fn replace_keys(&self) -> String;
    fn remove_parenthesis(&self) -> String;
    fn get_scalings(&self) -> String;
    fn get_interval(&self) -> Option<(f64, f64)>;
    fn get_damage(&self) -> String;
    fn from_scaled_string(&self) -> String;
    fn process_linear_scalings(
        bounds: (f64, f64),
        size: usize,
        postfix: Option<EvalIdent>,
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
                    "There are no parenthesis in #{} for '{}'",
                    number, self
                ))?
                .to_string(),
        )
    }

    fn capture_percent(&self, number: usize) -> MayFail<f64> {
        Ok(Regex::new(&format!(r"^(?:.*?(\d+)%){{{}}}", number + 1))?
            .captures(self)
            .and_then(|cap| cap.get(1).map(|m| m.as_str()))
            .ok_or(format!("No percent value in #{} for '{}'", number, self))?
            .parse::<f64>()
            .map_err(|e| format!("Unable to convert all numbers to f64: {e:?}"))?)
    }

    fn capture_numbers(&self) -> Vec<String> {
        Regex::new(r"\d+")
            .unwrap()
            .find_iter(self)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<String>>()
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
                $(
                    push_tuple(&mut replacements, $from, $to);
                )*
            };
        }
        tuple_rep! {
            "per 100" => "0.01 *",
            "of damage dealt" => "100.0",
            "of damage stored" => "100.0",
            "of expended Grit" => "0.0",
            "of the original damage" => "100.0",
            "per Overwhelm stack on the target" => "1.0",
            "per Soul collected" => format!("*{ThreshStacks}"),
            "of primary target's bonus health" => EnemyBonusHealth,
            "of his bonus health" => BonusHealth,
            "Pantheon's bonus health" => BonusHealth,
            "critical strike chance" => CritChance,
            "of Ivern's AP" => Ap,
            "of Sona's AP" => Ap,
            "per Feast stack" => ChogathStacks,
            "of Siphoning Strike stacks" => NasusStacks,
            "Stardust" => AurelionSolStacks,
            "per mark" => KindredStacks,
            "bonus mana" => BonusMana,
            "bonus AD" => BonusAd,
            "bonus armor" => BonusArmor,
            "bonus magic resistance" => BonusMagicResist,
            "bonus health" => BonusHealth,
            "bonus movement speed" => BonusMoveSpeed,
            "armor" => Armor,
            "of the target's maximum health" => EnemyMaxHealth,
            "of target's maximum health" => EnemyMaxHealth,
            "of Zac's maximum health" => MaxHealth,
            "of Braum's maximum health" => MaxHealth,
            "of her maximum health" => MaxHealth,
            "of his maximum health" => MaxHealth,
            "of maximum health" => MaxHealth,
            "maximum health" => MaxHealth,
            "of the target's current health" => CurrentHealth,
            "of target's current health" => CurrentHealth,
            "target's current health" => CurrentHealth,
            "of the target's missing health" => MissingHealth,
            "of target's missing health" => MissingHealth,
            "target's missing health" => MissingHealth,
            "maximum mana" => MaxMana,
            "AP" => Ap,
            "AD" => Ad,
            "\u{00D7}" => "*",
        };

        replacements
            .into_iter()
            .fold(self.to_string(), |acc, (old, new)| {
                acc.replace(old, &(*new).to_string())
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
                    result.push(format!("({} * {})", decimal, rest));
                    continue;
                }
            }
            result.push(format!("({})", cleaned));
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
        postfix: Option<EvalIdent>,
    ) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let (start, end) = bounds;
        for i in 0..size {
            let value = start + (((end - start) * (i as f64)) / (size as f64 - 1.0));
            if let Some(postfix) = postfix {
                result.push(format!("({value} + {postfix})"));
                continue;
            }
            result.push(value.to_string());
        }
        result
    }
}
