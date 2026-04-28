use crate::{client::MayFail, selector};
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Range, RangeFrom, RangeTo},
    str::FromStr,
    sync::LazyLock,
};
use tutorlolv2_types::CtxVar;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EffectInner {
    pub description: String,
    pub leveling: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Effect {
    pub index: usize,
    pub inner: EffectInner,
    pub scalings: Vec<Scaling>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<f64>>,
    pub base: Option<Vec<f64>>,
}

pub fn get_cells(html: &Html) -> MayFail<BTreeMap<String, String>> {
    let mut cells = BTreeMap::new();

    for tr in html.select(&selector("table.article-table > tbody > tr")?) {
        let tds = tr.select(&selector("td")?).collect::<Vec<_>>();

        if tds.len() == 3 {
            let first = tds.first().ok_or("Failed to find parameter")?;
            let key = first
                .select(&selector("code")?)
                .next()
                .map(|element| element.text())
                .unwrap_or(first.text())
                .collect::<String>();

            let value = tds.get(1).ok_or("Failed to get value")?;
            let text = value.text().collect::<String>();

            if text.trim().is_empty() {
                continue;
            }

            cells.insert(key.clone() + "_raw", value.inner_html());
            cells.insert(key, text.trim().to_string());
        }
    }

    Ok(cells)
}

pub fn parse_lua(text: &str) -> MayFail<String> {
    Ok(Html::parse_document(&text)
        .select(&selector("pre.mw-code.mw-script")?)
        .next()
        .ok_or("Failed to select <pre> tag")?
        .text()
        .collect::<String>()
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n"))
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Scaling {
    Simple {
        value: f64,
        ctx_var: CtxVar,
    },
    Ranked {
        values: Vec<f64>,
        ctx_var: CtxVar,
    },
    RankedPer100 {
        values: Vec<f64>,
        ctx_var: CtxVar,
    },
    Per100 {
        value: f64,
        ctx_var: CtxVar,
    },
    PercentAttr {
        value: f64,
        debug: String,
        ctx_var: CtxVar,
    },
    BasedOnLevel {
        level_var: CtxVar,
        arms: Vec<LevelArm>,
        debug: String,
        ctx_var: CtxVar,
    },
    Flat {
        values: Vec<f64>,
    },
    Nested {
        raw: String,
        outer: Box<Scaling>,
        inner: Vec<Scaling>,
    },
    Other {
        raw: String,
    },
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum LevelArm {
    To { range: RangeTo<u8>, value: f64 },
    Range { range: Range<u8>, value: f64 },
    From { range: RangeFrom<u8>, value: f64 },
}

impl Scaling {
    /// Parses raw blocks into an entity of Scaling
    pub fn parse(raw_block: &str) -> Scaling {
        let inner_raw = raw_block
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim_start_matches('+')
            .trim();

        let nested_blocks = find_scaling_blocks(inner_raw);

        if !nested_blocks.is_empty() {
            let outer_raw = remove_nested_scalings(&inner_raw);
            let outer = Scaling::from_non_nested(&outer_raw);
            let inner = nested_blocks
                .iter()
                .map(String::as_str)
                .map(Self::parse)
                .collect::<Vec<_>>();

            return Scaling::Nested {
                raw: html_to_text(raw_block),
                outer: Box::new(outer),
                inner,
            };
        }

        Scaling::from_non_nested(&inner_raw)
    }

    pub fn from_non_nested(raw: &str) -> Self {
        let text = normalize_text(&html_to_text(raw));

        if text.is_empty() {
            return Self::Other {
                raw: html_to_text(raw),
            };
        }

        if let Some(scaling) = Self::based_on_level(raw, &text)
            .or_else(|| Self::nested_percent_attr(&text))
            .or_else(|| Self::percent_attr(&text))
            .or_else(|| Self::ranked_per100(&text))
            .or_else(|| Self::per100(&text))
            .or_else(|| Self::ranked_per_ctx(&text))
            .or_else(|| Self::per_ctx(&text))
            .or_else(|| Self::bare_percent(&text))
            .or_else(|| Self::ranked_bare_percent(&text))
            .or_else(|| Self::ranked(&text))
            .or_else(|| Self::simple(&text))
            .or_else(|| Self::flat(&text))
        {
            return scaling;
        }

        Self::Other { raw: text }
    }

    pub fn simple(text: &str) -> Option<Scaling> {
        static SIMPLE_PERCENT_TAIL_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(.+)$").unwrap());

        let caps = SIMPLE_PERCENT_TAIL_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Simple { value, ctx_var })
    }

    pub fn ranked(text: &str) -> Option<Scaling> {
        static RANKED_PERCENT_TAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%\s+(.+)$").unwrap()
        });

        let caps = RANKED_PERCENT_TAIL_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Ranked { values, ctx_var })
    }

    pub fn per100(text: &str) -> Option<Scaling> {
        static SIMPLE_PER100_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+per\s+100%?\s+(.+)$").unwrap());

        let caps = SIMPLE_PER100_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Per100 { value, ctx_var })
    }

    pub fn ranked_per100(text: &str) -> Option<Scaling> {
        static RANKED_PER100_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%\s+per\s+100%?\s+(.+)$")
                .unwrap()
        });

        let caps = RANKED_PER100_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::RankedPer100 { values, ctx_var })
    }

    pub fn percent_attr(text: &str) -> Option<Scaling> {
        static PERCENT_ATTR_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(of .+)$").unwrap());

        let caps = PERCENT_ATTR_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let debug = caps.get(2)?.as_str().trim().to_string();
        let ctx_var = assign_ctx_var(&debug);

        Some(Scaling::PercentAttr {
            value,
            debug,
            ctx_var,
        })
    }

    pub fn flat(text: &str) -> Option<Scaling> {
        static RANKED_VALUES_ONLY_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+$").unwrap());

        if !RANKED_VALUES_ONLY_RE.is_match(text) {
            return None;
        }

        let values = parse_slash_f64s(text);
        if values.is_empty() {
            return None;
        }

        Some(Scaling::Flat { values })
    }

    pub fn based_on_level(raw: &str, text: &str) -> Option<Scaling> {
        if !text.to_ascii_lowercase().contains("based on level") {
            return None;
        }

        let fragment = Html::parse_fragment(raw);
        let tooltip = fragment.select(&selector("span.pp-tooltip").ok()?).next()?;

        let top_values = tooltip.value().attr("data-top-values")?;
        let bot_values = tooltip.value().attr("data-bot-values")?;

        pub fn get_values<T: FromStr>(text: &str) -> Vec<T> {
            text.split(';')
                .map(str::trim)
                .filter_map(|v| v.parse::<T>().ok())
                .collect::<Vec<_>>()
        }

        let starts = get_values::<u8>(top_values);
        let mut values = get_values::<f64>(bot_values);

        if values.is_empty() || starts.is_empty() || values.len() != starts.len() {
            return None;
        }

        let is_percent = tooltip
            .value()
            .attr("data-bot-key")
            .map(|v| v == "%")
            .unwrap_or(false);

        if is_percent {
            for value in &mut values {
                *value /= 100.0;
            }
        }

        let tooltip_text = normalize_text(&clean_text(&tooltip.text().collect::<String>()));
        let debug = normalize_text(&text.replacen(&tooltip_text, "", 1));
        let ctx_var = assign_ctx_var(&debug);

        let arms = build_level_arms(&starts, &values);

        Some(Scaling::BasedOnLevel {
            level_var: CtxVar::Level,
            arms,
            debug,
            ctx_var,
        })
    }

    pub fn nested_percent_attr(text: &str) -> Option<Scaling> {
        static NESTED_PERCENT_ATTR_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^\((.+)\)%\s+(of .+)$").unwrap());

        let caps = NESTED_PERCENT_ATTR_RE.captures(text)?;
        let inner_raw = caps.get(1)?.as_str().trim();
        let debug = caps.get(2)?.as_str().trim().to_string();
        let ctx_var = assign_ctx_var(&debug);

        let inner = Scaling::from_non_nested(inner_raw);

        if matches!(inner, Scaling::Other { .. }) {
            return None;
        }

        Some(Scaling::Nested {
            raw: text.to_string(),
            outer: Box::new(Scaling::PercentAttr {
                value: 0.0,
                debug,
                ctx_var,
            }),
            inner: vec![inner],
        })
    }

    pub fn ranked_per_ctx(text: &str) -> Option<Scaling> {
        static RANKED_PER_CTX_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)\s+per\s+(.+)$").unwrap()
        });

        let caps = RANKED_PER_CTX_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str());
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Ranked { values, ctx_var })
    }

    pub fn per_ctx(text: &str) -> Option<Scaling> {
        static PER_CTX_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)\s+per\s+(.+)$").unwrap());

        let caps = PER_CTX_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Simple { value, ctx_var })
    }

    pub fn ranked_bare_percent(text: &str) -> Option<Scaling> {
        static RANKED_BARE_PERCENT_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%$").unwrap()
        });

        let caps = RANKED_BARE_PERCENT_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        Some(Scaling::Flat { values })
    }

    pub fn bare_percent(text: &str) -> Option<Scaling> {
        static BARE_PERCENT_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%$").unwrap());

        let caps = BARE_PERCENT_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;

        Some(Scaling::Flat {
            values: vec![value],
        })
    }
}

fn normalize_text(input: &str) -> String {
    clean_text(input)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .trim_start_matches('+')
        .trim()
        .to_string()
}

pub fn html_to_text(input: &str) -> String {
    clean_text(
        &Html::parse_fragment(input)
            .root_element()
            .text()
            .collect::<String>(),
    )
}

pub fn assign_ctx_var(input: &str) -> CtxVar {
    let text = normalize_text(input).to_ascii_lowercase();

    macro_rules! match_str {
        ($text:expr, == $pat:literal) => {
            $text == $pat
        };
        ($text:expr, ... $pat:literal) => {
            $text.ends_with($pat)
        };
        ($text:expr, in $pat:literal) => {
            $text.contains($pat)
        };
    }

    macro_rules! check {
        (
            $text:expr;
            $( $( $method:tt $pat:literal )|+ => $variant:expr );+
            $(;)?
        ) => {
            $(
                if $( match_str!($text, $method $pat) )||+ {
                    return $variant;
                }
            )+
        };
    }

    check! {
        text;
        in "bonus attack speed" => CtxVar::AttackSpeed;
        in "bonus ad" | in "bonus attack damage" => CtxVar::BonusAd;
        == "ad" | ... " ad" | in " attack damage" => CtxVar::AttackDamage;
        == "ap" | ... " ap" | in "ability power" => CtxVar::AbilityPower;
        in "target's maximum health"
            | in "of target's maximum health" => CtxVar::EnemyMaxHealth;
        in "target's current health"
            | in "of target's current health" => CtxVar::EnemyCurrentHealth;
        in "target's missing health"
            | in "of target's missing health" => CtxVar::EnemyMissingHealth;
        in "missing health" => CtxVar::MissingHealth;
        in "bonus movement speed" => CtxVar::BonusMoveSpeed;
        in "maximum health" => CtxVar::MaxHealth;
        in "maximum mana" => CtxVar::MaxMana;
        in "bonus health" => CtxVar::BonusHealth;
        in "bonus armor" => CtxVar::BonusArmor;
        in "bonus magic resistance" => CtxVar::BonusMagicResist;
        in "bonus mana" => CtxVar::BonusMana;
        in "critical strike chance" => CtxVar::CritChance;
        in "life steal" => CtxVar::LifeSteal;
        in "stacks"
            | in " stack"
            | in "overwhelm"
            | in "stardust"
            | in "soul collected"
            | in "feast stack"
            | in "mist"
            | in "grit"
            | in "mark"
            | in "of damage stored" => CtxVar::Stacks;
        in "armor" | in "total armor" => CtxVar::Armor;
        in "magic resist" | in "total magic resist" => CtxVar::MagicResist;
        in "lethality" => CtxVar::ArmorPenetrationFlat
    };

    CtxVar::SteelcapsEffect
}

pub fn find_scaling_blocks(input: &str) -> Vec<String> {
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 1 < chars.len() {
        let (_, c1) = chars[i];
        let (_, c2) = chars[i + 1];

        if c1 == '(' && c2 == '+' {
            let start = chars[i].0;
            let mut depth = 1usize;
            let mut j = i + 1;

            while j + 1 < chars.len() {
                j += 1;
                let (byte, ch) = chars[j];

                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            let end = byte + ch.len_utf8();
                            out.push(input[start..end].trim().to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }

            i = j;
        } else {
            i += 1;
        }
    }

    out
}

fn remove_nested_scalings(input: &str) -> String {
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut out = String::new();
    let mut i = 0usize;

    while i < chars.len() {
        let (_, c1) = chars[i];

        if i + 1 < chars.len() {
            let (_, c2) = chars[i + 1];

            if c1 == '(' && c2 == '+' {
                let mut depth = 1usize;
                let mut j = i + 1;

                while j + 1 < chars.len() {
                    j += 1;
                    let (_, ch) = chars[j];

                    match ch {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                i = j + 1;
                continue;
            }
        }

        out.push(c1);
        i += 1;
    }

    out
}

fn build_level_arms(starts: &[u8], values: &[f64]) -> Vec<LevelArm> {
    let mut out = Vec::new();

    for (i, (&start, &value)) in starts.iter().zip(values.iter()).enumerate() {
        match starts.get(i + 1).copied() {
            Some(next) if i == 0 => out.push(LevelArm::To {
                range: ..next,
                value,
            }),
            Some(next) => out.push(LevelArm::Range {
                range: start..next,
                value,
            }),
            None => out.push(LevelArm::From {
                range: start..,
                value,
            }),
        }
    }

    out
}

fn parse_slash_f64s(input: &str) -> Vec<f64> {
    input
        .split('/')
        .map(normalize_text)
        .filter_map(|v| v.parse::<f64>().ok())
        .collect()
}

pub fn clean_text(s: &str) -> String {
    s.replace('\u{a0}', " ")
        .replace('\u{2013}', "-")
        .replace('\u{2014}', "-")
        .replace('\u{2212}', "-")
        .replace('\u{00D7}', "*")
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub fn parse_description_effects(
    index: usize,
    description: &str,
    description_raw: &str,
) -> MayFail<(String, Effect)> {
    let fragment = Html::parse_fragment(description_raw);

    let tooltip = fragment.select(&selector("span.pp-tooltip")?).next();

    let use_formula = tooltip
        .and_then(|el| el.value().attr("data-useformula"))
        .map(str::to_string);

    let use_values = tooltip
        .and_then(|el| el.value().attr("data-bot-values"))
        .map(|s| {
            s.split(';')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<_>>()
        });

    let key = fragment
        .select(&selector("span.template_sbc")?)
        .next()
        .and_then(|node| {
            node.select(&selector("b").ok()?)
                .next()
                .map(|b| b.text().collect::<String>())
        })
        .map(|s| s.trim().trim_end_matches(':').trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(format!("Description {index}"));

    Ok((
        key,
        Effect {
            index,
            inner: EffectInner {
                description: description.to_string(),
                leveling: String::new(),
            },
            scalings: parse_scalings(description_raw),
            use_formula,
            use_values,
            base: parse_base_damage(description),
        },
    ))
}

pub fn parse_base_damage(text: &str) -> Option<Vec<f64>> {
    static RANKED_VALUES_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"-?\d+(?:\.\d+)?%?(?:\s*/\s*-?\d+(?:\.\d+)?%?)+").unwrap());

    fn get_base_text(s: &str) -> &str {
        for (i, char) in s.char_indices() {
            if char == '(' {
                return &s[..i];
            }
        }
        s
    }

    let base = get_base_text(text).trim();

    let values = RANKED_VALUES_RE
        .find(base)?
        .as_str()
        .split('/')
        .map(|v| v.trim().trim_end_matches('%').trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse().ok())
        .collect::<Vec<_>>();

    match values.is_empty() {
        true => None,
        false => Some(values),
    }
}

pub fn parse_scalings(input_raw: &str) -> Vec<Scaling> {
    let mut raws = find_scaling_blocks(input_raw);

    raws.retain(|s| !html_to_text(s).trim().is_empty());

    let mut out = raws
        .iter()
        .map(String::as_str)
        .map(Scaling::parse)
        .collect::<Vec<_>>();

    vec_dedup(&mut out);
    out
}

pub fn vec_dedup<T: PartialEq>(list: &mut Vec<T>) {
    let mut out = Vec::new();

    for item in core::mem::take(list) {
        if !out.contains(&item) {
            out.push(item);
        }
    }

    *list = out;
}

pub const SUFFIXES: [&str; 10] = ["", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
