use crate::{client::MayFail, selector};
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Range, RangeFrom, RangeTo},
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
    #[serde(default)]
    pub formula: Option<String>,
    pub inner: EffectInner,
    pub scalings: Vec<Scaling>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<f64>>,
    pub base: Option<Vec<f64>>,
}

impl Effect {
    pub fn should_keep(&self) -> bool {
        self.base.is_some()
            || self.use_formula.is_some()
            || self.use_values.is_some()
            || !self.scalings.is_empty()
    }
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
    RangePercentAttr {
        min: f64,
        max: f64,
        debug: String,
        ctx_var: CtxVar,
    },
    Multiplier {
        raw: String,
        base: f64,
        inner: Vec<Scaling>,
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

impl LevelArm {
    pub const fn value(&self) -> f64 {
        match self {
            LevelArm::To { value, .. }
            | LevelArm::Range { value, .. }
            | LevelArm::From { value, .. } => *value,
        }
    }
}

impl Scaling {
    pub fn parse(raw_block: &str) -> Self {
        let inner_raw = raw_block
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim_start_matches('+')
            .trim();

        let nested_blocks = find_scaling_blocks(inner_raw);

        if !nested_blocks.is_empty() {
            let outer_raw = remove_nested_scalings(inner_raw);
            let outer = Self::from_non_nested(&outer_raw);
            let inner = nested_blocks
                .iter()
                .map(String::as_str)
                .map(Self::parse)
                .collect::<Vec<_>>();

            return Self::Nested {
                raw: html_to_text(raw_block),
                outer: Box::new(outer),
                inner,
            };
        }

        Self::from_non_nested(inner_raw)
    }

    pub fn from_non_nested(raw: &str) -> Self {
        let text = normalize_text(&html_to_text(raw));

        if text.is_empty() {
            return Self::Other {
                raw: html_to_text(raw),
            };
        }

        if let Some(scaling) = Self::based_on_level(raw, &text)
            .or_else(|| Self::range_percent_attr_with_nested(&text))
            .or_else(|| Self::range_percent_attr(&text))
            .or_else(|| Self::multiplier_group(&text))
            .or_else(|| Self::nested_percent_attr(&text))
            .or_else(|| Self::ranked_percent_attr(&text))
            .or_else(|| Self::percent_attr(&text))
            .or_else(|| Self::ranked_per100(&text))
            .or_else(|| Self::per100(&text))
            .or_else(|| Self::ranked_per_ctx(&text))
            .or_else(|| Self::per_ctx(&text))
            .or_else(|| Self::ranked_bare_percent(&text))
            .or_else(|| Self::bare_percent(&text))
            .or_else(|| Self::ranked(&text))
            .or_else(|| Self::simple(&text))
            .or_else(|| Self::flat(&text))
        {
            return scaling;
        }

        Self::Other { raw: text }
    }

    pub fn simple(text: &str) -> Option<Self> {
        static SIMPLE_PERCENT_TAIL_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(.+)$").unwrap());

        let caps = SIMPLE_PERCENT_TAIL_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Simple { value, ctx_var })
    }

    pub fn ranked(text: &str) -> Option<Self> {
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

    pub fn ranked_percent_attr(text: &str) -> Option<Self> {
        static RANKED_PERCENT_ATTR_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%\s+(of .+)$").unwrap()
        });

        let caps = RANKED_PERCENT_ATTR_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Ranked { values, ctx_var })
    }

    pub fn per100(text: &str) -> Option<Self> {
        static SIMPLE_PER100_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+per\s+100%?\s+(.+)$").unwrap());

        let caps = SIMPLE_PER100_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Per100 { value, ctx_var })
    }

    pub fn ranked_per100(text: &str) -> Option<Self> {
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

        Some(Self::RankedPer100 { values, ctx_var })
    }

    pub fn percent_attr(text: &str) -> Option<Self> {
        static PERCENT_ATTR_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(of .+)$").unwrap());

        let caps = PERCENT_ATTR_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let debug = caps.get(2)?.as_str().trim().to_string();
        let ctx_var = assign_ctx_var(&debug);

        Some(Self::PercentAttr {
            value,
            debug,
            ctx_var,
        })
    }

    pub fn based_on_level(raw: &str, text: &str) -> Option<Self> {
        Self::based_on_level_all(raw, text).into_iter().next()
    }

    pub fn based_on_level_all(raw: &str, text: &str) -> Vec<Self> {
        if !text.to_ascii_lowercase().contains("based on level") {
            return Vec::new();
        }

        let fragment = Html::parse_fragment(raw);
        let selector = match selector("span.pp-tooltip") {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let mut out = Vec::new();
        let raw_text = normalize_text(&html_to_text(raw));

        for tooltip in fragment.select(&selector) {
            let Some(bot_values) = tooltip.value().attr("data-bot-values") else {
                continue;
            };

            let tooltip_text = normalize_text(&clean_text(&tooltip.text().collect::<String>()));
            if !tooltip_text.to_ascii_lowercase().contains("based on level") {
                continue;
            }

            let mut values = bot_values
                .split(';')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .filter_map(|v| v.parse::<f64>().ok())
                .collect::<Vec<_>>();

            if values.is_empty() {
                continue;
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

            let starts = if let Some(top_values) = tooltip.value().attr("data-top-values") {
                let starts = top_values
                    .split(';')
                    .map(str::trim)
                    .filter_map(|v| v.parse::<u8>().ok())
                    .collect::<Vec<_>>();

                if starts.len() != values.len() {
                    continue;
                }
                starts
            } else {
                match values.len() {
                    18 => (1u8..=18).collect::<Vec<_>>(),
                    20 => (1u8..=20).collect::<Vec<_>>(),
                    _ => continue,
                }
            };

            let debug = local_debug_window(&raw_text, &tooltip_text);
            let ctx_var = assign_ctx_var(&debug);

            if should_drop_based_on_level_as_base_duplicate(&debug) {
                continue;
            }

            out.push(Self::BasedOnLevel {
                level_var: CtxVar::Level,
                arms: build_level_arms(&starts, &values),
                debug,
                ctx_var,
            });
        }

        dedup_scalings_semantic(&mut out);
        out
    }

    pub fn range_percent_attr(text: &str) -> Option<Self> {
        static RANGE_PERCENT_ATTR_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s*-\s*(-?\d+(?:\.\d+)?)%\s*\(based on ([^)]+)\)$")
                .unwrap()
        });

        let caps = RANGE_PERCENT_ATTR_RE.captures(text)?;
        let min = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let max = caps.get(2)?.as_str().parse::<f64>().ok()? / 100.0;
        let based_on = caps.get(3)?.as_str().trim();
        let debug = format!("based on {based_on}");
        let ctx_var = assign_ctx_var(based_on);

        Some(Self::RangePercentAttr {
            min,
            max,
            debug,
            ctx_var,
        })
    }

    pub fn range_percent_attr_anywhere(text: &str) -> Vec<Self> {
        static RANGE_PERCENT_ATTR_ANYWHERE_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(-?\d+(?:\.\d+)?)%\s*-\s*(-?\d+(?:\.\d+)?)%\s*\(based on ([^)]+)\)")
                .unwrap()
        });

        let mut out = Vec::new();

        for caps in RANGE_PERCENT_ATTR_ANYWHERE_RE.captures_iter(text) {
            let Some(min) = caps.get(1).and_then(|v| v.as_str().parse::<f64>().ok()) else {
                continue;
            };
            let Some(max) = caps.get(2).and_then(|v| v.as_str().parse::<f64>().ok()) else {
                continue;
            };
            let Some(based_on) = caps.get(3).map(|v| v.as_str().trim()) else {
                continue;
            };

            let debug = format!("based on {based_on}");
            if should_drop_range_percent_attr(&debug) {
                continue;
            }

            out.push(Self::RangePercentAttr {
                min: min / 100.0,
                max: max / 100.0,
                debug,
                ctx_var: assign_ctx_var(based_on),
            });
        }

        dedup_scalings_semantic(&mut out);
        out
    }

    pub fn range_percent_attr_with_nested(text: &str) -> Option<Self> {
        Self::nested_range_percent_attr_anywhere(text)
            .into_iter()
            .next()
    }

    pub fn nested_range_percent_attr_anywhere(text: &str) -> Vec<Self> {
        static NESTED_RANGE_PERCENT_ATTR_ANYWHERE_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(-?\d+(?:\.\d+)?)%\s*-\s*(-?\d+(?:\.\d+)?)%\s*\(\+\s*(-?\d+(?:\.\d+)?)%\s*-\s*(-?\d+(?:\.\d+)?)%\)\s*\(based on ([^)]+)\)"
            )
            .unwrap()
        });

        let mut out = Vec::new();

        for caps in NESTED_RANGE_PERCENT_ATTR_ANYWHERE_RE.captures_iter(text) {
            let capture = |i| caps.get(i).and_then(|v| v.as_str().parse::<f64>().ok());

            let Some(outer_min) = capture(1) else {
                continue;
            };
            let Some(outer_max) = capture(2) else {
                continue;
            };
            let Some(inner_min) = capture(3) else {
                continue;
            };
            let Some(inner_max) = capture(4) else {
                continue;
            };
            let Some(based_on) = caps.get(5).map(|v| v.as_str().trim()) else {
                continue;
            };

            let debug = format!("based on {based_on}");
            if should_drop_range_percent_attr(&debug) {
                continue;
            }
            let ctx_var = assign_ctx_var(based_on);

            out.push(Self::Nested {
                raw: caps
                    .get(0)
                    .map(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                outer: Box::new(Self::RangePercentAttr {
                    min: outer_min / 100.0,
                    max: outer_max / 100.0,
                    debug: debug.clone(),
                    ctx_var,
                }),
                inner: vec![Self::RangePercentAttr {
                    min: inner_min / 100.0,
                    max: inner_max / 100.0,
                    debug,
                    ctx_var,
                }],
            });
        }

        dedup_scalings_semantic(&mut out);
        out
    }

    pub fn multiplier_group(text: &str) -> Option<Self> {
        static MULTIPLIER_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^1\s*\+\s*(-?\d+(?:\.\d+)?)\s+per\s+100%?\s+(.+)$").unwrap()
        });

        let caps = MULTIPLIER_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Multiplier {
            raw: text.to_string(),
            base: 1.0,
            inner: vec![Self::Per100 { value, ctx_var }],
        })
    }

    pub fn nested_percent_attr(text: &str) -> Option<Self> {
        static NESTED_PERCENT_ATTR_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^\((.+)\)%\s+(of .+)$").unwrap());

        let caps = NESTED_PERCENT_ATTR_RE.captures(text)?;
        let inner_raw = caps.get(1)?.as_str().trim();
        let debug = caps.get(2)?.as_str().trim().to_string();
        let ctx_var = assign_ctx_var(&debug);

        let inner = Self::from_non_nested(inner_raw);
        if matches!(inner, Self::Other { .. }) {
            return None;
        }

        Some(Self::Nested {
            raw: text.to_string(),
            outer: Box::new(Self::PercentAttr {
                value: 0.0,
                debug,
                ctx_var,
            }),
            inner: vec![inner],
        })
    }

    pub fn ranked_per_ctx(text: &str) -> Option<Self> {
        static RANKED_PER_CTX_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)\s+per\s+(.+)$").unwrap()
        });

        let caps = RANKED_PER_CTX_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str());
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Ranked { values, ctx_var })
    }

    pub fn per_ctx(text: &str) -> Option<Self> {
        static PER_CTX_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)\s+per\s+(.+)$").unwrap());

        let caps = PER_CTX_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()?;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Self::Simple { value, ctx_var })
    }

    pub fn ranked_bare_percent(text: &str) -> Option<Self> {
        static RANKED_BARE_PERCENT_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%$").unwrap()
        });

        let caps = RANKED_BARE_PERCENT_RE.captures(text)?;
        let values = parse_slash_f64s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        Some(Self::Flat { values })
    }

    pub fn bare_percent(text: &str) -> Option<Self> {
        static BARE_PERCENT_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%$").unwrap());

        let caps = BARE_PERCENT_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f64>().ok()? / 100.0;

        Some(Self::Flat {
            values: vec![value],
        })
    }

    pub fn flat(text: &str) -> Option<Self> {
        static RANKED_VALUES_ONLY_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+$").unwrap());

        if !RANKED_VALUES_ONLY_RE.is_match(text) {
            return None;
        }

        let values = parse_slash_f64s(text);
        if values.is_empty() {
            return None;
        }

        Some(Self::Flat { values })
    }

    fn should_promote_to_separate_effect(&self, description: &str) -> bool {
        let desc = description.to_ascii_lowercase();
        match self {
            Self::BasedOnLevel { debug, .. } => {
                let d = debug.to_ascii_lowercase();
                d.contains("against monsters")
                    || d.contains("against minions")
                    || d.contains("capped at")
                    || desc.contains("capped at")
            }
            Self::RangePercentAttr { debug, .. } => {
                let d = debug.to_ascii_lowercase();
                d.contains("missing health") || d.contains("critical strike chance")
            }
            Self::Nested { outer, .. } => matches!(
                outer.as_ref(),
                Self::RangePercentAttr { debug, .. }
                    if debug.to_ascii_lowercase().contains("critical strike chance")
            ),
            Self::Multiplier { .. } => true,
            _ => false,
        }
    }

    pub fn scaling_absorbs_scalar(&self, ctx_var: CtxVar, value: f64) -> bool {
        match self {
            Self::Ranked {
                values,
                ctx_var: other,
            } if *other == ctx_var => values.iter().any(|v| approx_eq(*v, value)),
            Self::BasedOnLevel {
                arms,
                ctx_var: other,
                ..
            } if *other == ctx_var => arms
                .iter()
                .map(LevelArm::value)
                .any(|v| approx_eq(v, value)),
            _ => false,
        }
    }

    pub fn scaling_subsumes(&self, older: &Self) -> bool {
        match (self, older) {
            (
                Self::Ranked {
                    values,
                    ctx_var: new_ctx,
                },
                Self::Simple {
                    value,
                    ctx_var: old_ctx,
                },
            ) if new_ctx == old_ctx => values.iter().any(|v| approx_eq(*v, *value)),
            (
                Self::Ranked {
                    values,
                    ctx_var: new_ctx,
                },
                Self::PercentAttr {
                    value,
                    ctx_var: old_ctx,
                    ..
                },
            ) if new_ctx == old_ctx => values.iter().any(|v| approx_eq(*v, *value)),
            (
                Self::BasedOnLevel {
                    arms,
                    ctx_var: new_ctx,
                    ..
                },
                Self::Simple {
                    value,
                    ctx_var: old_ctx,
                },
            ) if new_ctx == old_ctx => arms
                .iter()
                .map(LevelArm::value)
                .any(|v| approx_eq(v, *value)),
            (
                Self::BasedOnLevel {
                    arms,
                    ctx_var: new_ctx,
                    ..
                },
                Self::PercentAttr {
                    value,
                    ctx_var: old_ctx,
                    ..
                },
            ) if new_ctx == old_ctx => arms
                .iter()
                .map(LevelArm::value)
                .any(|v| approx_eq(v, *value)),
            _ => false,
        }
    }

    fn debug_str(&self) -> &str {
        match self {
            Scaling::PercentAttr { debug, .. }
            | Scaling::BasedOnLevel { debug, .. }
            | Scaling::RangePercentAttr { debug, .. } => debug,
            Scaling::Nested { raw, .. }
            | Scaling::Multiplier { raw, .. }
            | Scaling::Other { raw } => raw,
            _ => "",
        }
    }

    fn is_primary_contextual_scaling(&self) -> bool {
        match &self {
            Self::BasedOnLevel { ctx_var, .. } => {
                !matches!(ctx_var, CtxVar::SteelcapsEffect | CtxVar::Stacks)
            }
            Self::PercentAttr { .. }
            | Self::Ranked { .. }
            | Self::RankedPer100 { .. }
            | Self::Per100 { .. }
            | Self::Simple { .. }
            | Self::Flat { .. }
            | Self::RangePercentAttr { .. }
            | Self::Multiplier { .. }
            | Self::Nested { .. } => true,
            Self::Other { .. } => false,
        }
    }
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
            $( $( $method:tt $pat:literal )|+ => $variant:expr );+ $(;)?
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
        in "enchanted target's ad" => CtxVar::SteelcapsEffect;
        in "target's maximum health" | in "of target's maximum health" => CtxVar::EnemyMaxHealth;
        in "target's current health" | in "of target's current health" => CtxVar::EnemyCurrentHealth;
        in "target's missing health" | in "of target's missing health" => CtxVar::EnemyMissingHealth;
        in "his current health" | in "of his current health" | in "current health" => CtxVar::CurrentHealth;
        in "missing health" => CtxVar::MissingHealth;
        in "bonus movement speed" => CtxVar::BonusMoveSpeed;
        in "maximum health" => CtxVar::MaxHealth;
        in "maximum mana" => CtxVar::MaxMana;
        in "missing mana" => CtxVar::SteelcapsEffect; // Not supported yet
        in "bonus health" => CtxVar::BonusHealth;
        in "bonus armor" => CtxVar::BonusArmor;
        in "bonus magic resistance" => CtxVar::BonusMagicResist;
        in "bonus mana" => CtxVar::BonusMana;
        in "critical strike chance" => CtxVar::CritChance;
        in "life steal" => CtxVar::LifeSteal;
        in "stacks" | in " stack" | in "overwhelm" | in "stardust" | in "soul collected" | in "feast stack" | in "mist" | in "grit" | in "mark" | in "of damage stored" => CtxVar::Stacks;
        in "armor" | in "total armor" => CtxVar::Armor;
        in "magic resist" | in "total magic resist" => CtxVar::MagicResist;
        in "lethality" => CtxVar::ArmorPenetrationFlat
    };

    CtxVar::SteelcapsEffect
}

pub fn parse_description_effects(
    index: usize,
    description: &str,
    description_raw: &str,
) -> MayFail<Vec<(String, Effect)>> {
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

    let base_key = fragment
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

    let mut results = Vec::new();
    let mut scalings = parse_scalings(description_raw);
    let base = parse_base_damage(description);

    let mut main_scalings = Vec::new();
    let mut extras = Vec::new();

    let mut kept_primary_contextual = false;
    for scaling in scalings.drain(..) {
        if scaling.should_promote_to_separate_effect(description) {
            extras.push(scaling);
            continue;
        }

        if (use_formula.is_some() || use_values.is_some())
            && matches!(scaling, Scaling::BasedOnLevel { .. })
            && should_drop_based_on_level_as_base_duplicate(scaling.debug_str())
        {
            continue;
        }

        match use_formula.is_some() || use_values.is_some() {
            true => match scaling.is_primary_contextual_scaling() {
                true => match !kept_primary_contextual {
                    true => {
                        main_scalings.push(scaling);
                        kept_primary_contextual = true;
                    }
                    false => extras.push(scaling),
                },
                false => main_scalings.push(scaling),
            },
            false => main_scalings.push(scaling),
        }
    }

    let main_effect = Effect {
        index,
        formula: None,
        inner: EffectInner {
            description: description.to_string(),
            leveling: String::new(),
        },
        scalings: main_scalings,
        use_formula,
        use_values,
        base,
    };

    if main_effect.should_keep() {
        results.push((base_key.clone(), main_effect));
    }

    for (i, scaling) in extras.into_iter().enumerate() {
        let key = format!("{base_key} [{}]", i + 1);
        let effect = Effect {
            index,
            formula: None,
            inner: EffectInner {
                description: description.to_string(),
                leveling: String::new(),
            },
            scalings: vec![scaling],
            use_formula: None,
            use_values: None,
            base: None,
        };

        if effect.should_keep() {
            results.push((key, effect));
        }
    }

    if results.is_empty() {
        results.push((
            base_key,
            Effect {
                index,
                formula: None,
                inner: EffectInner {
                    description: description.to_string(),
                    leveling: String::new(),
                },
                scalings: Vec::new(),
                use_formula: None,
                use_values: None,
                base: None,
            },
        ));
    }

    Ok(results)
}

pub fn parse_base_damage(text: &str) -> Option<Vec<f64>> {
    static LEADING_VALUES_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?%?(?:\s*/\s*-?\d+(?:\.\d+)?%?)+").unwrap());

    let text = normalize_text(text);
    let matched = LEADING_VALUES_RE.find(&text)?;
    let seq = matched.as_str();
    let remainder = strip_leading_non_base_suffixes(text[matched.end()..].trim());

    if sequence_is_pure_ratio(seq, &remainder) || is_contextual_remainder(&remainder) {
        return None;
    }

    let values = seq
        .split('/')
        .map(|v| v.trim().trim_end_matches('%').trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<f64>().ok())
        .collect::<Vec<_>>();

    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn sequence_is_pure_ratio(seq: &str, remainder: &str) -> bool {
    seq.contains('%')
        && [
            "ad",
            "ap",
            "attack damage",
            "ability power",
            "maximum health",
            "current health",
            "missing health",
            "missing mana",
            "bonus movement speed",
            "critical strike chance",
        ]
        .iter()
        .any(|pat| remainder.to_ascii_lowercase().contains(pat))
}

fn strip_leading_non_base_suffixes(input: &str) -> String {
    let mut s = input.trim().to_string();

    loop {
        s = s
            .trim_start_matches(|c: char| c == ',' || c == ';')
            .trim()
            .to_string();

        if let Some(rest) = strip_one_leading_group(&s) {
            s = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = s.strip_prefix('*') {
            s = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = s.strip_prefix('x') {
            s = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = s.strip_prefix('X') {
            s = rest.trim().to_string();
            continue;
        }

        break;
    }

    normalize_text(&s)
}

fn strip_one_leading_group(input: &str) -> Option<String> {
    let (open, close) = if input.starts_with('(') {
        ('(', ')')
    } else if input.starts_with('[') {
        ('[', ']')
    } else {
        return None;
    };

    let mut depth = 0usize;
    for (idx, ch) in input.char_indices() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(input[idx + ch.len_utf8()..].to_string());
            }
        }
    }

    None
}

fn is_contextual_remainder(remainder: &str) -> bool {
    if remainder.is_empty() {
        return false;
    }

    let harmless_exact = [
        "%",
        "seconds",
        "second",
        "units",
        "unit",
        "soldiers",
        "soldier",
        "chunks of ice",
        "chunk of ice",
        "damage",
        "magic damage",
        "physical damage",
        "true damage",
    ];

    if harmless_exact.iter().any(|&s| remainder == s) {
        return false;
    }

    let contextual = [
        " of ",
        "of ",
        " per ",
        "per ",
        "ap",
        "ad",
        "attack damage",
        "ability power",
        "armor",
        "magic resist",
        "magic resistance",
        "health",
        "mana",
        "current",
        "missing",
        "maximum",
        "bonus ",
        "target",
        "stardust",
        "overwhelm",
        "soul",
        "stack",
    ];

    contextual.iter().any(|s| remainder.contains(s))
}

pub fn parse_scalings(input_raw: &str) -> Vec<Scaling> {
    let mut raws = Vec::<String>::new();
    let mut out = Vec::<Scaling>::new();

    raws.extend(find_scaling_blocks(input_raw));

    let stripped_raw = remove_nested_scalings(input_raw);
    let stripped_text = normalize_text(&html_to_text(&stripped_raw));

    if looks_like_scaling_candidate(&stripped_text) {
        raws.push(stripped_raw);
    }

    raws.extend(find_inline_scaling_candidates(&stripped_text));
    raws.retain(|s| !normalize_text(&html_to_text(s)).is_empty());

    for raw in raws {
        let scaling = Scaling::parse(&raw);
        if !matches!(scaling, Scaling::Other { .. }) {
            out.push(scaling);
            continue;
        }

        let fallback = Scaling::from_non_nested(&raw);
        if !matches!(fallback, Scaling::Other { .. }) {
            out.push(fallback);
        }
    }

    out.extend(Scaling::based_on_level_all(input_raw, &stripped_text));
    out.extend(Scaling::nested_range_percent_attr_anywhere(&stripped_text));
    out.extend(Scaling::range_percent_attr_anywhere(&stripped_text));

    vec_dedup(&mut out);
    dedup_scalings_semantic(&mut out);
    out
}

fn looks_like_scaling_candidate(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();

    lower.contains("based on level")
        || lower.contains("% of ")
        || lower.contains("% per ")
        || lower.contains(" per ")
        || lower.contains(" ap")
        || lower.ends_with(" ad")
        || lower.contains(" attack damage")
        || lower.contains(" ability power")
        || lower.contains(" maximum health")
        || lower.contains(" current health")
        || lower.contains(" missing health")
        || lower.contains(" missing mana")
        || lower.contains(" critical strike chance")
        || lower.contains(" target")
}

fn find_inline_scaling_candidates(text: &str) -> Vec<String> {
    static INLINE_SCALING_RES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
        vec![
            Regex::new(r"(?i)-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+%\s+of\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?%\s+of\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+%\s+per\s+100%?\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?%\s+per\s+100%?\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+\s+per\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?\s+per\s+[^,.;)]+").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?%\s+[a-z][^,.;)]*").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?%\s*-\s*-?\d+(?:\.\d+)?%\s*\(\s*based on [^)]+\)").unwrap(),
            Regex::new(r"(?i)-?\d+(?:\.\d+)?%\s*-\s*-?\d+(?:\.\d+)?%\s*\(\+\s*-?\d+(?:\.\d+)?%\s*-\s*-?\d+(?:\.\d+)?%\)\s*\(\s*based on [^)]+\)").unwrap(),
            Regex::new(r"(?i)1\s*\+\s*-?\d+(?:\.\d+)?\s+per\s+100%?\s+[^,.;)]+").unwrap(),
        ]
    });

    let mut out = Vec::new();

    for re in INLINE_SCALING_RES.iter() {
        for m in re.find_iter(text) {
            let candidate = m.as_str().trim().to_string();
            if is_important_candidate(&candidate) {
                out.push(candidate);
            }
        }
    }

    normalize_candidate_list(&mut out);
    out
}

fn is_important_candidate(candidate: &str) -> bool {
    let lower = candidate.to_ascii_lowercase();

    let important = [
        " of ",
        "of ",
        " per ",
        "per ",
        "ap",
        "ad",
        "attack damage",
        "ability power",
        "health",
        "mana",
        "armor",
        "magic resist",
        "critical strike chance",
        "based on target's missing health",
        "based on critical strike chance",
        "bonus attack speed",
        "stardust",
        "overwhelm",
        "soul",
        "stack",
    ];

    important.iter().any(|s| lower.contains(s))
        && !lower.contains("against minions")
        && !lower.contains("against monsters")
        && !lower.contains("reduced to ")
}

fn normalize_candidate_list(list: &mut Vec<String>) {
    list.sort_by_key(|s| std::cmp::Reverse(s.len()));

    let mut out = Vec::<String>::new();
    'outer: for candidate in std::mem::take(list) {
        let norm = normalize_text(&candidate).to_ascii_lowercase();
        for existing in &out {
            let e = normalize_text(existing).to_ascii_lowercase();
            if e.contains(&norm) {
                continue 'outer;
            }
        }
        out.push(candidate);
    }

    *list = out;
}

fn local_debug_window(raw_text: &str, matched_text: &str) -> String {
    let raw_chars = raw_text.chars().collect::<Vec<_>>();
    let matched_chars = matched_text.chars().collect::<Vec<_>>();

    let found = raw_chars
        .windows(matched_chars.len())
        .position(|w| w == matched_chars.as_slice());

    let Some(pos) = found else {
        return raw_text.to_string();
    };

    let start = pos.saturating_sub(80);
    let end = (pos + matched_chars.len() + 80).min(raw_chars.len());
    let local = raw_chars[start..end].iter().collect::<String>();
    normalize_text(&local.replacen(matched_text, "", 1))
}

fn debug_has_real_target(debug: &str) -> bool {
    let text = debug.to_ascii_lowercase();

    [
        "maximum health",
        "current health",
        "missing health",
        "missing mana",
        "ability power",
        "attack damage",
        "bonus ad",
        "bonus attack damage",
        "bonus movement speed",
        "bonus attack speed",
        "critical strike chance",
        "armor",
        "magic resist",
        "magic resistance",
        "of target",
        "of the target",
        "against monsters",
        "against minions",
    ]
    .iter()
    .any(|pat| text.contains(pat))
}

fn should_drop_range_percent_attr(debug: &str) -> bool {
    let text = debug.to_ascii_lowercase();
    text == "based on level" || text.trim().is_empty()
}

fn should_drop_based_on_level_as_base_duplicate(debug: &str) -> bool {
    !debug_has_real_target(debug)
}

fn dedup_scalings_semantic(list: &mut Vec<Scaling>) {
    let mut out = Vec::<Scaling>::new();

    for scaling in std::mem::take(list) {
        if out.contains(&scaling) {
            continue;
        }

        match &scaling {
            Scaling::Simple { value, ctx_var } => {
                if out
                    .iter()
                    .any(|s| s.scaling_absorbs_scalar(*ctx_var, *value))
                {
                    continue;
                }
            }
            Scaling::PercentAttr { value, ctx_var, .. } => {
                if out
                    .iter()
                    .any(|s| s.scaling_absorbs_scalar(*ctx_var, *value))
                {
                    continue;
                }
            }
            Scaling::RangePercentAttr {
                min, max, ctx_var, ..
            } => {
                if out.iter().any(|s| match s {
                    Scaling::BasedOnLevel { ctx_var: other, .. } => {
                        *other == *ctx_var && approx_eq(*min, *max)
                    }
                    _ => false,
                }) {
                    continue;
                }
            }
            _ => {}
        }

        out.retain(|existing| !scaling.scaling_subsumes(existing));
        out.push(scaling);
    }

    *list = out;
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
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

pub fn vec_dedup<T: PartialEq>(list: &mut Vec<T>) {
    let mut out = Vec::new();

    for item in std::mem::take(list) {
        if !out.contains(&item) {
            out.push(item);
        }
    }

    *list = out;
}

pub const SUFFIXES: [&str; 10] = ["", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
