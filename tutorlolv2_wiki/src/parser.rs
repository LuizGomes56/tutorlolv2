use crate::{MayFail, selector};
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Bound, Range, RangeBounds, RangeFrom, RangeTo},
    sync::LazyLock,
};
use tutorlolv2_types::CtxVar;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EffectInner {
    pub description: String,
    pub leveling: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Effect {
    pub index: usize,
    #[serde(default)]
    pub formula: Option<String>,
    pub inner: EffectInner,
    pub scalings: Vec<Scaling>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<f64>>,
    /// Levels at which each of `use_values` starts, when they are given by level
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_levels: Option<Vec<u8>>,
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

    for tr in html.select(selector!("table.article-table > tbody > tr")) {
        let tds = tr.select(selector!("td")).collect::<Vec<_>>();

        if tds.len() == 3 {
            let first = tds.first().ok_or("Failed to find parameter")?;
            let key = first
                .select(selector!("code"))
                .next()
                .map(|element| element.text())
                .unwrap_or(first.text())
                .collect::<String>();

            let value = tds.get(1).ok_or("Failed to get value")?;

            // Some values have alternative representations that the reader can flip
            // between, such as damage per second and per tick. Only the first one
            // is kept, otherwise both are parsed as separate scalings and added
            let raw = remove_elements_with_class(&value.inner_html(), "flipText2");
            let text = html_to_text(&raw);

            if text.trim().is_empty() {
                continue;
            }

            cells.insert(key.clone() + "_raw", raw);
            cells.insert(key, clean_text(text.trim()));
        }
    }

    Ok(cells)
}

/// Removes every `<span>` with the given class, including its nested elements
fn remove_elements_with_class(html: &str, class: &str) -> String {
    let open = format!("<span class=\"{class}\"");
    let mut out = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(start) = rest.find(&open) {
        out.push_str(&rest[..start]);

        let mut depth = 0usize;
        let mut i = start;

        loop {
            let next_open = rest[i..].find("<span").map(|p| p + i);
            let next_close = rest[i..].find("</span>").map(|p| p + i);

            match (next_open, next_close) {
                (Some(o), Some(c)) if o < c => {
                    depth += 1;
                    i = o + "<span".len();
                }
                (_, Some(c)) => {
                    depth -= 1;
                    i = c + "</span>".len();
                    if depth == 0 {
                        break;
                    }
                }
                _ => {
                    i = rest.len();
                    break;
                }
            }
        }

        rest = &rest[i..];
    }

    out.push_str(rest);
    out
}

pub fn parse_lua(text: &str) -> MayFail<String> {
    Ok(Html::parse_document(text)
        .select(selector!("pre.mw-code.mw-script"))
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
    pub fn flat(level: u8, value: impl Into<f64>) -> Self {
        LevelArm::Range {
            range: level..level + 1,
            value: value.into(),
        }
    }

    pub fn new(range: impl RangeBounds<u8>, value: impl Into<f64>) -> Self {
        let value = value.into();

        match (range.start_bound(), range.end_bound()) {
            (Bound::Unbounded, Bound::Excluded(&end)) => Self::To {
                range: RangeTo { end },
                value,
            },
            (Bound::Included(&start), Bound::Excluded(&end)) => Self::Range {
                range: start..end,
                value,
            },
            (Bound::Included(&start), Bound::Included(&end)) => Self::Range {
                range: start..(end + 1),
                value,
            },
            (Bound::Included(&start), Bound::Unbounded) => Self::From {
                range: RangeFrom { start },
                value,
            },
            (Bound::Unbounded, Bound::Unbounded) => {
                panic!(".. is not a valid interval for LevelArm");
            }
            _ => panic!("Invalid interval format"),
        }
    }

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

    pub fn based_on_level_raw<const N: usize>(ctx_var: CtxVar, arms: [LevelArm; N]) -> Self {
        Self::BasedOnLevel {
            level_var: ctx_var,
            arms: arms.to_vec(),
            debug: "Call to based_on_level_raw".into(),
            ctx_var,
        }
    }

    pub fn based_on_level(raw: &str, text: &str) -> Option<Self> {
        Self::based_on_level_all(raw, text).into_iter().next()
    }

    pub fn based_on_level_all(raw: &str, text: &str) -> Vec<Self> {
        if !text.to_ascii_lowercase().contains("based on level") {
            return Vec::new();
        }

        let fragment = Html::parse_fragment(raw);
        let mut out = Vec::new();
        let raw_text = normalize_text(&html_to_text(raw));

        for tooltip in fragment.select(selector!("span.pp-tooltip")) {
            let Some(bot_values) = tooltip.value().attr("data-bot-values") else {
                continue;
            };

            let tooltip_text = normalize_text(&clean_text(&tooltip.text().collect::<String>()));
            if !tooltip_text.to_ascii_lowercase().contains("based on level") {
                continue;
            }

            let mut values = bot_values
                .split('|')
                .next()
                .unwrap_or_default()
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

            let debug = following_text(&raw_text, &tooltip_text);
            let ctx_var = leading_ctx_var(&debug);

            // Flat base values are read separately. Looking around the tooltip
            // for a stat can bind them to an unrelated neighboring scaling.
            if ctx_var == CtxVar::Unknown {
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
        let target = caps.get(2)?.as_str();

        // "0.3 per 100 bonus health" is 0.3 for every 100 points of that stat
        if let Some(stat) = target.strip_prefix("100 ") {
            return Some(Self::Per100 {
                value,
                ctx_var: assign_ctx_var(stat),
            });
        }

        Some(Self::Simple {
            value,
            ctx_var: assign_ctx_var(target),
        })
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

    /// Whether the scaling's values are `values` multiplied by `factor`
    pub fn has_values(&self, values: &[f64], factor: f64) -> bool {
        let own = match self {
            Self::Ranked { values, .. } | Self::RankedPer100 { values, .. } => values.clone(),
            Self::BasedOnLevel { arms, .. } => arms.iter().map(LevelArm::value).collect(),
            Self::Nested { outer, .. } => return outer.has_values(values, factor),
            _ => return false,
        };

        own.len() == values.len()
            && own
                .iter()
                .zip(values)
                .all(|(a, b)| (a - b * factor).abs() < 1e-6)
    }

    /// Whether both scalings represent the same term, ignoring their debug text
    pub fn same_term(&self, other: &Self) -> bool {
        let values_eq = |a: &[f64], b: &[f64]| {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| approx_eq(*x, *y))
        };

        match (self, other) {
            (
                Self::Simple {
                    value: a,
                    ctx_var: x,
                }
                | Self::Per100 {
                    value: a,
                    ctx_var: x,
                }
                | Self::PercentAttr {
                    value: a,
                    ctx_var: x,
                    ..
                },
                Self::Simple {
                    value: b,
                    ctx_var: y,
                }
                | Self::Per100 {
                    value: b,
                    ctx_var: y,
                }
                | Self::PercentAttr {
                    value: b,
                    ctx_var: y,
                    ..
                },
            ) => {
                core::mem::discriminant(self) == core::mem::discriminant(other)
                    && x == y
                    && approx_eq(*a, *b)
            }
            (
                Self::Ranked {
                    values: a,
                    ctx_var: x,
                },
                Self::Ranked {
                    values: b,
                    ctx_var: y,
                },
            )
            | (
                Self::RankedPer100 {
                    values: a,
                    ctx_var: x,
                },
                Self::RankedPer100 {
                    values: b,
                    ctx_var: y,
                },
            ) => x == y && values_eq(a, b),
            (
                Self::BasedOnLevel {
                    level_var: a_level,
                    arms: a,
                    ctx_var: a_ctx,
                    ..
                },
                Self::BasedOnLevel {
                    level_var: b_level,
                    arms: b,
                    ctx_var: b_ctx,
                    ..
                },
            ) => a_level == b_level && a_ctx == b_ctx && a == b,
            _ => self == other,
        }
    }

    fn is_primary_contextual_scaling(&self) -> bool {
        match &self {
            Self::BasedOnLevel { ctx_var, .. } => !matches!(
                ctx_var,
                CtxVar::SteelcapsEffect | CtxVar::Stacks | CtxVar::Unknown
            ),
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
        .replace(['\u{2013}', '\u{2014}', '\u{2212}'], "-")
        .replace('\u{00D7}', "*")
        .replace("–", "-")
        .replace("'''", "")
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
        // Whole words only, so that "mark" doesn't match "marked" or "market"
        ($text:expr, word $pat:literal) => {
            $text
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| word == $pat)
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
        in "bonus attack speed" => CtxVar::BonusAttackSpeed;
        in "attack speed" => CtxVar::AttackSpeed;
        in "bonus ad" | in "bonus attack damage" => CtxVar::BonusAd;
        in "base ad" | in "base attack damage" => CtxVar::BaseAd;
        == "ad" | ... " ad" | in " attack damage" => CtxVar::AttackDamage;
        == "ap" | ... " ap" | in "ability power" => CtxVar::AbilityPower;
        in "enchanted target's ad" => CtxVar::SteelcapsEffect;
        in "target's maximum health" => CtxVar::EnemyMaxHealth;
        in "target's current health" => CtxVar::EnemyCurrentHealth;
        in "target's missing health" => CtxVar::EnemyMissingHealth;
        in "primary target's bonus health" => CtxVar::EnemyBonusHealth;
        in "current health" => CtxVar::CurrentHealth;
        in "missing health" => CtxVar::MissingHealth;
        in "movement speed" => CtxVar::BonusMoveSpeed;
        // Usually the enemies hit, as in "magic damage equal to 5% of their maximum health"
        in "their maximum health" => CtxVar::EnemyMaxHealth;
        in "their current health" => CtxVar::EnemyCurrentHealth;
        in "their missing health" => CtxVar::EnemyMissingHealth;
        in "maximum health" => CtxVar::MaxHealth;
        in "maximum mana" => CtxVar::MaxMana;
        // There is no variable for missing mana
        in "missing mana" => CtxVar::Unknown;
        in "bonus health" => CtxVar::BonusHealth;
        in "bonus armor" => CtxVar::BonusArmor;
        in "bonus magic resistance" => CtxVar::BonusMagicResist;
        in "bonus mana" => CtxVar::BonusMana;
        in "critical strike chance" => CtxVar::CritChance;
        in "life steal" => CtxVar::LifeSteal;
        word "stack" | word "stacks" | word "overwhelm"
            | word "stardust" | in "soul collected"
            | in "feast stack" | word "mist" | word "grit"
            | word "mark" | word "marks" | in "of damage stored"
            | word "soul" | word "souls" | word "chimes"
            | in "dragon practice" => CtxVar::Stacks;
        in "armor" => CtxVar::Armor;
        in "magic resist" | word "mr" => CtxVar::MagicResist;
        in "lethality" => CtxVar::ArmorPenetrationFlat
    };

    CtxVar::Unknown
}

pub fn parse_description_effects(
    index: usize,
    description: &str,
    description_raw: &str,
) -> MayFail<Vec<(String, Effect)>> {
    let fragment = Html::parse_fragment(description_raw);

    let text = normalize_text(&html_to_text(description_raw));
    let tooltip = fragment.select(selector!("span.pp-tooltip")).next();

    let base_tooltip = tooltip.filter(|el| {
        let value = normalize_text(&el.text().collect::<String>());
        let before = text.split_once(&value).map_or("", |(before, _)| before);
        let clause = before
            .rsplit(['.', ';', ','])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();

        // A monster cap is not flat base damage, even when it happens to be
        // the first tooltip in a passive's description.
        !clause.contains("capped at")
    });

    let mut use_formula = base_tooltip
        .and_then(|el| el.value().attr("data-useformula"))
        .map(str::to_string);

    let mut use_values = base_tooltip
        .and_then(|el| el.value().attr("data-bot-values"))
        .map(|s| {
            // Display options may follow the values, as in "...;20.12|displayMaxColumn=18"
            s.split('|')
                .next()
                .unwrap_or_default()
                .split(';')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<_>>()
        });

    let mut use_levels = base_tooltip.and_then(|el| tooltip_levels(el, use_values.as_deref()?));

    let base_key = fragment
        .select(selector!("span.template_sbc"))
        .next()
        .and_then(|node| {
            node.select(selector!("b"))
                .next()
                .map(|b| b.text().collect::<String>())
        })
        .map(|s| s.trim().trim_end_matches(':').trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(format!("Description {index}"));

    let mut results = Vec::new();
    let mut scalings = parse_scalings(description_raw);
    let base = parse_base_damage(description).or_else(|| inline_damage_base(description));

    // Percentages already parsed as a stat ratio must not also become a flat
    // base value or formula.
    if let (Some(el), Some(values)) = (base_tooltip, &use_values)
        && el.value().attr("data-bot-key") == Some("%")
        && scalings.iter().any(|s| s.has_values(values, 0.01))
    {
        use_formula = None;
        use_values = None;
        use_levels = None;
    }

    let mut main_scalings = Vec::new();
    let mut extras = Vec::new();
    let mut kept_primary_contextual = false;

    for scaling in scalings.drain(..) {
        if scaling.should_promote_to_separate_effect(description) {
            extras.push(scaling);
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
        use_levels,
        base,
    };

    if main_effect.should_keep() {
        results.push((base_key.clone(), main_effect));
    }

    for (i, scaling) in extras.into_iter().enumerate() {
        let key = format!("{base_key} [{}]", i + 1);
        let effect = Effect {
            index,
            inner: EffectInner {
                description: description.to_string(),
                leveling: String::new(),
            },
            scalings: vec![scaling],
            ..Default::default()
        };

        if effect.should_keep() {
            results.push((key, effect));
        }
    }

    if results.is_empty() {
        results.push((
            base_key.replace("–", "-"),
            Effect {
                index,
                inner: EffectInner {
                    description: description.to_string(),
                    leveling: String::new(),
                },
                ..Default::default()
            },
        ));
    }

    Ok(results)
}

/// Flat damage written inside prose, after removing its parenthesized scalings.
/// This avoids taking an earlier movement-speed table as Mordekaiser's base,
/// or a later slow table as Bard's base.
fn inline_damage_base(description: &str) -> Option<Vec<f64>> {
    static DAMAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\b(?:deals?|dealing|dealt)(?:\s+them)?\s*「?\s*(-?\d+(?:\.\d+)?)\s+(?:(?:bonus|total|additional|modified)\s+)*(?:magic|physical|true)\s+damage\b").unwrap()
    });

    let text = normalize_text(&remove_nested_scalings(description));
    let value = DAMAGE_RE.captures(&text)?.get(1)?.as_str().parse().ok()?;
    Some(vec![value])
}

pub fn parse_base_damage(text: &str) -> Option<Vec<f64>> {
    static LEADING_VALUES_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?%?(?:\s*/\s*-?\d+(?:\.\d+)?%?)+").unwrap());

    // A single value is only a base when it is followed by its scalings alone,
    // as in "5 (+ 30% AP)", since numbers also start sentences and durations
    static SINGLE_VALUE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?\s*\(\+").unwrap());

    let text = normalize_text(text);

    if !LEADING_VALUES_RE.is_match(&text) && SINGLE_VALUE_RE.is_match(&text) {
        let (value, rest) = text.split_once(|c: char| c.is_whitespace() || c == '(')?;
        let rest = strip_leading_non_base_suffixes(&format!("({rest}"));

        return match rest.is_empty() {
            true => Some(vec![value.parse().ok()?]),
            false => None,
        };
    }

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

    match values.is_empty() {
        true => None,
        false => Some(values),
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
        s = s.trim_start_matches([',', ';']).trim().to_string();

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

    if harmless_exact.contains(&remainder) {
        return false;
    }

    [
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
    ]
    .iter()
    .any(|s| remainder.contains(s))
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

    // Ratios like "6% (+ 3% per 100 AP) of target's maximum health" are split
    // above into two unrelated terms that get added together. Replace them by
    // a single nested scaling where the inner terms increase the ratio instead
    for (nested, parts) in nested_ratio_scalings(input_raw) {
        out.retain(|scaling| !parts.iter().any(|part| part.same_term(scaling)));
        out.push(nested);
    }

    vec_dedup(&mut out);
    dedup_scalings_semantic(&mut out);
    out
}

/// Finds ratios of a target's health that are increased by scalings placed between
/// the ratio and its target, outside of any parenthesized group, returning the nested
/// scaling and the separate terms that the generic parsing produces for it. The ratio
/// is either given by rank, as in "6 / 7 / 8%", or by level, as in "6% - 12% (based
/// on level)", whose values per level are read from the tooltip in `raw`
fn nested_ratio_scalings(raw: &str) -> Vec<(Scaling, Vec<Scaling>)> {
    static NESTED_RATIO_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?i)(-?\d+(?:\.\d+)?%?\s*-\s*-?\d+(?:\.\d+)?%\s*\(based on level\)|-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)*%)\s*((?:\(\+[^()]*\)\s*)+)(of (?:the )?(?:target's|their|its) [a-z ]*?health)",
        )
        .unwrap()
    });

    let text = normalize_text(&html_to_text(raw));
    let text = text.as_str();
    let mut out = Vec::new();

    for caps in NESTED_RATIO_RE.captures_iter(text) {
        let (Some(whole), Some(values), Some(blocks), Some(target)) =
            (caps.get(0), caps.get(1), caps.get(2), caps.get(3))
        else {
            continue;
        };

        // Matches inside another group are already handled by `Scaling::parse`
        let depth = text[..whole.start()]
            .chars()
            .fold(0i32, |depth, ch| match ch {
                '(' => depth + 1,
                ')' => depth - 1,
                _ => depth,
            });

        if depth != 0 {
            continue;
        }

        let target = target.as_str().replace("their", "target's");

        let outer = match values.as_str().ends_with("(based on level)") {
            true => match level_tooltip_arms(raw, values.as_str()) {
                Some(arms) => Scaling::BasedOnLevel {
                    level_var: CtxVar::Level,
                    arms,
                    debug: target.clone(),
                    ctx_var: assign_ctx_var(&target),
                },
                None => continue,
            },
            false => Scaling::from_non_nested(&format!("{} {target}", values.as_str())),
        };

        let inner = find_scaling_blocks(blocks.as_str())
            .iter()
            .map(|block| Scaling::parse(block))
            .collect::<Vec<_>>();

        if matches!(outer, Scaling::Other { .. })
            || inner.is_empty()
            || inner.iter().any(|s| matches!(s, Scaling::Other { .. }))
        {
            continue;
        }

        let parts = std::iter::once(outer.clone())
            .chain(inner.iter().cloned())
            .collect();

        out.push((
            Scaling::Nested {
                raw: whole.as_str().to_string(),
                outer: Box::new(outer),
                inner,
            },
            parts,
        ));
    }

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

/// Variable of the shortest leading phrase of `text` that names one, so that in
/// "AD bonus physical damage" only "AD" is considered
fn leading_ctx_var(text: &str) -> CtxVar {
    let words = text.split_whitespace().take(6).collect::<Vec<_>>();

    (1..=words.len())
        .map(|n| assign_ctx_var(&words[..n].join(" ")))
        .find(|ctx_var| *ctx_var != CtxVar::Unknown)
        .unwrap_or(CtxVar::Unknown)
}

/// Text that follows `matched_text`, after skipping parenthesized groups. The
/// variable of a value is usually written right after it, as in
/// "60% / 70% / 80% (based on level) (+ 0% - 80% (based on ...)) AD"
/// Values per level of the "based on level" tooltip whose text is `tooltip_text`,
/// as ratios when the tooltip shows percentages
fn level_tooltip_arms(raw: &str, tooltip_text: &str) -> Option<Vec<LevelArm>> {
    let fragment = Html::parse_fragment(raw);
    let wanted = normalize_text(tooltip_text);

    let tooltip = fragment
        .select(selector!("span.pp-tooltip"))
        .find(|el| normalize_text(&clean_text(&el.text().collect::<String>())) == wanted)?;

    let factor = match tooltip.value().attr("data-bot-key") {
        Some("%") => 0.01,
        _ => 1.0,
    };

    // Display options may follow the values, as in "...;20.12|displayMaxColumn=18"
    let values = tooltip
        .value()
        .attr("data-bot-values")?
        .split('|')
        .next()?
        .split(';')
        .map(|v| v.trim().parse::<f64>().ok().map(|v| v * factor))
        .collect::<Option<Vec<_>>>()?;

    let starts = match tooltip_levels(tooltip, &values) {
        Some(levels) => levels,
        None => (1..=values.len() as u8).collect(),
    };

    Some(build_level_arms(&starts, &values))
}

fn following_text(raw_text: &str, matched_text: &str) -> String {
    let Some(pos) = raw_text.find(matched_text) else {
        return String::new();
    };

    let mut rest = raw_text[pos + matched_text.len()..].trim_start();

    loop {
        rest = rest.trim_start_matches(['「', '」', ' ']).trim_start();

        if !rest.starts_with('(') {
            break;
        }

        let mut depth = 0usize;
        let mut end = rest.len();

        for (i, ch) in rest.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        rest = &rest[end..];
    }

    rest.split(['.', ',', ';', '(', ')'])
        .next()
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn should_drop_range_percent_attr(debug: &str) -> bool {
    let text = debug.to_ascii_lowercase();
    text == "based on level" || text.trim().is_empty()
}

fn dedup_scalings_semantic(list: &mut Vec<Scaling>) {
    let mut out = Vec::<Scaling>::new();

    for scaling in std::mem::take(list) {
        if out.iter().any(|existing| existing.same_term(&scaling)) {
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
            } if out.iter().any(|s| match s {
                Scaling::BasedOnLevel { ctx_var: other, .. } => {
                    *other == *ctx_var && approx_eq(*min, *max)
                }
                _ => false,
            }) =>
            {
                continue;
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

/// Levels at which each value of a tooltip starts, if the tooltip is based on
/// level and gives one starting level per value
pub fn tooltip_levels(tooltip: scraper::ElementRef, values: &[f64]) -> Option<Vec<u8>> {
    let text = tooltip.text().collect::<String>().to_ascii_lowercase();

    if !text.contains("based on level") {
        return None;
    }

    let levels = tooltip
        .value()
        .attr("data-top-values")?
        .split(';')
        .map(|v| v.trim().parse::<u8>().ok())
        .collect::<Option<Vec<_>>>()?;

    (levels.len() == values.len() && levels.len() > 1).then_some(levels)
}

pub fn build_level_arms(starts: &[u8], values: &[f64]) -> Vec<LevelArm> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_scalings_bind_to_the_stat_inside_their_group() {
        let raw = r#"deals <span class="pp-tooltip" data-bot-values="68;72;77" data-top-values="1;2;3">68 - 77 (based on level)</span> (+ 60% bonus AD) (+ <span class="pp-tooltip" data-bot-values="70;80;90;100" data-top-values="1;6;11;16" data-bot-key="%">70% / 80% / 90% / 100% (based on level)</span> AP) magic damage"#;
        let scalings = parse_scalings(raw);

        let levels = scalings
            .iter()
            .filter_map(|scaling| match scaling {
                Scaling::BasedOnLevel { arms, ctx_var, .. } => Some((arms, ctx_var)),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(levels.len(), 1, "{scalings:?}");
        assert_eq!(*levels[0].1, CtxVar::AbilityPower);
        assert_eq!(
            levels[0].0.iter().map(LevelArm::value).collect::<Vec<_>>(),
            [0.7, 0.8, 0.9, 1.0]
        );
        assert!(scalings.iter().any(|s| matches!(
            s,
            Scaling::Simple {
                value: 0.6,
                ctx_var: CtxVar::BonusAd
            }
        )));

        let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();
        assert!(
            effects
                .iter()
                .flat_map(|(_, e)| &e.scalings)
                .any(|s| matches!(
                    s,
                    Scaling::BasedOnLevel {
                        ctx_var: CtxVar::AbilityPower,
                        ..
                    }
                )),
            "{effects:?}"
        );
    }

    #[test]
    fn level_scalings_are_not_duplicated_by_other_scaling_blocks() {
        let raw = r#"gains <span class="pp-tooltip" data-bot-values="3;6;9" data-top-values="1;6;11" data-bot-key="%">3% / 6% / 9% (based on level)</span> bonus movement speed and deals 5 (+ 30% AP) magic damage"#;
        let scalings = parse_scalings(raw);

        assert_eq!(
            scalings
                .iter()
                .filter(|s| matches!(s, Scaling::BasedOnLevel { .. }))
                .count(),
            1,
            "{scalings:?}"
        );
    }

    #[test]
    fn monster_caps_are_not_used_as_passive_base_damage() {
        let raw = r#"deals 1% (+ 2.7% per 100 AP) of the target's maximum health, capped at <span class="pp-tooltip" data-useformula="90+10*x" data-bot-values="100;110;120">100 - 120 (based on level)</span> against monsters"#;
        let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();

        assert!(effects[0].1.use_formula.is_none(), "{effects:?}");
        assert!(effects[0].1.use_values.is_none(), "{effects:?}");
        assert!(
            effects[0]
                .1
                .scalings
                .iter()
                .any(|s| matches!(s, Scaling::Nested { .. }))
        );
    }

    #[test]
    fn movement_speed_tooltips_are_not_used_as_passive_base_damage() {
        let raw = r#"gains <span class="pp-tooltip" data-bot-values="3;6;9" data-top-values="1;6;11" data-bot-key="%">3% / 6% / 9% (based on level)</span> bonus movement speed and deals 5 (+ 30% AP) magic damage"#;
        let effects = parse_description_effects(0, &html_to_text(raw), raw).unwrap();

        assert!(effects[0].1.use_values.is_none(), "{effects:?}");
        assert_eq!(effects[0].1.base, Some(vec![5.0]));
    }

    #[test]
    fn per_100_scalings_increase_the_ratio_they_belong_to() {
        let raw = "7 / 7.5 / 8 / 8.5 / 9% <span>(+ 2.5% per 100 <b>bonus</b> AD)</span> of target's <b>maximum</b> health";
        let scalings = parse_scalings(raw);

        assert_eq!(scalings.len(), 1, "{scalings:?}");
        let Scaling::Nested { outer, inner, .. } = &scalings[0] else {
            panic!("expected a nested scaling: {scalings:?}");
        };

        assert!(matches!(
            outer.as_ref(),
            Scaling::Ranked {
                ctx_var: CtxVar::EnemyMaxHealth,
                ..
            }
        ));
        assert!(matches!(
            inner.as_slice(),
            [Scaling::Per100 {
                ctx_var: CtxVar::BonusAd,
                ..
            }]
        ));
    }

    #[test]
    fn level_ratios_use_the_tooltip_values_per_level() {
        let raw = r#"equal to <span class="pp-tooltip" data-bot-values="6;9;12" data-top-values="1;7;13" data-bot-key="%">6% – 12% (based on level)</span> <span>(+ 2% per 100 AP)</span> of their maximum health"#;
        let scalings = parse_scalings(raw);

        let [Scaling::Nested { outer, inner, .. }] = scalings.as_slice() else {
            panic!("expected a single nested scaling: {scalings:?}");
        };

        let Scaling::BasedOnLevel { arms, ctx_var, .. } = outer.as_ref() else {
            panic!("expected a level ratio: {outer:?}");
        };

        assert_eq!(*ctx_var, CtxVar::EnemyMaxHealth);
        assert_eq!(
            arms.iter().map(LevelArm::value).collect::<Vec<_>>(),
            [0.06, 0.09, 0.12]
        );
        assert!(matches!(
            inner.as_slice(),
            [Scaling::Per100 {
                ctx_var: CtxVar::AbilityPower,
                ..
            }]
        ));
    }

    #[test]
    fn ratios_inside_groups_are_left_to_the_group() {
        let text = "50 / 80 (+ 8% (+ 3% per 100 AP) of target's missing health)";
        assert!(nested_ratio_scalings(text).is_empty());
    }

    #[test]
    fn alternative_representations_are_removed() {
        let html = r#"a<span class="flipText1 active">「 x <span>1</span> 」</span><span class="flipText2">「 y <span>2</span> 」</span>b"#;
        let cleaned = remove_elements_with_class(html, "flipText2");

        assert!(cleaned.contains("x <span>1</span>"));
        assert!(!cleaned.contains('y'));
        assert!(cleaned.ends_with('b'));
    }

    #[test]
    fn variables_come_from_the_text_after_the_value() {
        let text = "deal 60% / 70% / 80% (based on level) (+ 0% - 80% (based on critical strike chance)) AD bonus physical damage";
        let following = following_text(text, "60% / 70% / 80% (based on level)");

        assert_eq!(leading_ctx_var(&following), CtxVar::AttackDamage);
    }

    #[test]
    fn unknown_variables_are_explicit() {
        assert_eq!(assign_ctx_var("of target's missing mana"), CtxVar::Unknown);
        assert_eq!(
            assign_ctx_var("bonus attack speed"),
            CtxVar::BonusAttackSpeed
        );
        assert_eq!(assign_ctx_var("the marked enemy"), CtxVar::Unknown);
        assert_eq!(assign_ctx_var("150% base AD"), CtxVar::BaseAd);
    }

    #[test]
    fn values_per_100_without_percent_are_per_100() {
        assert_eq!(
            Scaling::per_ctx("0.3 per 100 bonus health"),
            Some(Scaling::Per100 {
                value: 0.3,
                ctx_var: CtxVar::BonusHealth,
            })
        );
        assert_eq!(
            Scaling::per_ctx("25 per 1 lethality"),
            Some(Scaling::Simple {
                value: 25.0,
                ctx_var: CtxVar::ArmorPenetrationFlat,
            })
        );
    }
}
