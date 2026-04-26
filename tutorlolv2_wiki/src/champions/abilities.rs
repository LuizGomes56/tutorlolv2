use crate::{
    champions::{
        cache, clean_text,
        template::{ChampionTemplate, SkillSet},
    },
    client::{MayFail, fetch},
    file_name, is_dir,
    parser::get_cells,
    selector,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, str::FromStr, sync::LazyLock};
use tutorlolv2_types::{CtxVar, DamageType, Key};

pub async fn download() -> MayFail {
    println!("[fn] champions::abilities::download");

    for entry in crate::read_dir(cache())?.filter(is_dir) {
        let path = entry.path();

        let bytes = crate::read(path.join("template").with_extension("json"))?;

        let ChampionTemplate {
            name,
            skills:
                SkillSet {
                    skill_i,
                    skill_q,
                    skill_w,
                    skill_e,
                    skill_r,
                },
            ..
        } = serde_json::from_slice(&bytes)
            .map_err(|e| format!("[error] Failed to deserialize entry: {entry:?}: {e:?}"))?;

        println!("[download] Processing {name:?}");

        let save_to = path.join("abilities");

        for (key, skills) in [
            ('P', skill_i),
            ('Q', skill_q),
            ('W', skill_w),
            ('E', skill_e),
            ('R', skill_r),
        ] {
            for (i, skill) in skills.into_iter().enumerate() {
                let url = format!("Template:Data_{name}/{skill}");

                fetch(
                    save_to.join(format!("{key}{i}")).with_extension("html"),
                    &url,
                )
                .await?;
            }
        }
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] champions::abilities::parse");

    crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let path = entry.path();

            let champion_id = file_name(&entry)?;

            println!("[{champion_id:?}] Processing abilities");

            crate::read_dir(path.join("abilities"))?
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|v| v == "html")
                        .unwrap_or(false)
                })
                .par_bridge()
                .into_par_iter()
                .try_for_each(|entry| {
                    let path = entry.path();

                    let bytes = crate::read_to_string(&path)?;
                    let html = Html::parse_document(&bytes);

                    if let Some(element) = html.select(&selector("div.noarticletext > p")?).next()
                        && element
                            .text()
                            .any(|text| text.contains("There is currently no text in this page"))
                    {
                        return Ok(());
                    }

                    let cells = get_cells(&html)?;

                    let mut ability = parse_abilities(&cells)?;
                    ability.champion_id = champion_id.clone();

                    crate::write(
                        path.with_extension("json"),
                        serde_json::to_string_pretty(&ability)?,
                    )?;

                    Ok(())
                })
        })
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Ability {
    pub champion_id: String,
    pub damage_type: DamageType,
    pub name: String,
    pub skill: Key,
    pub spell_effects: Option<String>,
    pub effects: BTreeMap<String, Effect>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AbilityInner {
    pub description: String,
    pub leveling: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Effect {
    pub index: usize,
    pub inner: AbilityInner,
    pub scalings: Vec<Scaling>,
    pub use_formula: Option<String>,
    pub use_values: Option<Vec<String>>,
    pub base: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Scaling {
    Simple {
        value: f32,
        ctx_var: CtxVar,
    },
    Ranked {
        values: Vec<f32>,
        ctx_var: CtxVar,
    },
    RankedPer100 {
        values: Vec<f32>,
        ctx_var: CtxVar,
    },
    Per100 {
        value: f32,
        ctx_var: CtxVar,
    },
    PercentAttr {
        value: f32,
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
        values: Vec<f32>,
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
    To {
        end_exclusive: u8,
        value: f32,
    },
    Range {
        start_inclusive: u8,
        end_exclusive: u8,
        value: f32,
    },
    From {
        start_inclusive: u8,
        value: f32,
    },
}

pub fn parse_abilities(cells: &BTreeMap<String, String>) -> MayFail<Ability> {
    let mut ability = Ability::default();

    ability.name = cells["1"].clone();
    ability.skill = cells["skill"].parse::<char>()?.try_into()?;

    ability.damage_type = cells
        .get("damagetype")
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(DamageType::default);

    ability.spell_effects = cells.get("spelleffects").cloned();

    for i in 0..10 {
        let suffix: &dyn Display = match i {
            0 => &"",
            _ => &i,
        };

        let get_key = |k: &str| format!("{k}{suffix}");

        let description_key = get_key("description");
        let leveling_key = get_key("leveling");

        let description = cells.get(&description_key);
        let description_raw = cells.get(&(description_key + "_raw"));
        let leveling = cells.get(&leveling_key);
        let leveling_raw = cells.get(&(leveling_key + "_raw"));

        if let Some(description) = description {
            if let Some(leveling) = leveling {
                let raw = leveling_raw.map(String::as_str).unwrap_or(leveling);

                if !raw.trim().is_empty() {
                    ability.effects.extend(parse_effects(i, description, raw)?);
                    continue;
                }
            }

            if ability.skill == Key::P {
                let raw = description_raw.map(String::as_str).unwrap_or(description);

                ability
                    .effects
                    .extend([parse_description_effects(i, description, raw)?]);
            }
        }
    }

    Ok(ability)
}

fn parse_effects(
    index: usize,
    description: &str,
    leveling_raw: &str,
) -> MayFail<Vec<(String, Effect)>> {
    let fragment = Html::parse_fragment(leveling_raw);

    let tooltip_selector = selector("span.pp-tooltip")?;

    let dts = fragment.select(&selector("dt")?).collect::<Vec<_>>();
    let dds = fragment.select(&selector("dd")?).collect::<Vec<_>>();

    assert!(
        !dts.is_empty() && dts.len() == dds.len(),
        "No structured dd/dt tags found"
    );

    Ok(dts
        .into_iter()
        .zip(dds.into_iter())
        .filter_map(|(dt, dd)| {
            let key = dt
                .select(&selector("span.template_lc").ok()?)
                .next()
                .map(|e| e.text())
                .unwrap_or(dt.text())
                .collect::<String>()
                .trim()
                .trim_end_matches(':')
                .trim()
                .to_string();

            if key.is_empty() {
                return None;
            }

            let dd_leveling = dd.text().collect::<String>().trim().to_string();
            let dd_leveling_raw = dd.inner_html();
            let tooltip = dd.select(&tooltip_selector).next();

            let use_formula = tooltip
                .and_then(|el| el.value().attr("data-useformula"))
                .map(str::to_string);

            let use_values = tooltip
                .and_then(|el| el.value().attr("data-bot-values"))
                .map(|s| {
                    s.split(';')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty())
                        .collect()
                });

            Some((
                key,
                Effect {
                    index,
                    base: parse_base_damage(&dd_leveling),
                    inner: AbilityInner {
                        description: description.to_string(),
                        leveling: dd_leveling,
                    },
                    use_formula,
                    use_values,
                    scalings: parse_scalings(&dd_leveling_raw),
                },
            ))
        })
        .collect())
}

fn parse_description_effects(
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
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
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
            inner: AbilityInner {
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

fn parse_base_damage(text: &str) -> Option<Vec<String>> {
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
        .map(String::from)
        .collect::<Vec<_>>();

    match values.is_empty() {
        true => None,
        false => Some(values),
    }
}

fn parse_scalings(input_raw: &str) -> Vec<Scaling> {
    let mut raws = extract_plus_paren_blocks(input_raw);

    raws.retain(|s| !html_to_text(s).trim().is_empty());

    let mut out = raws
        .into_iter()
        .map(|raw| parse_scaling_block(&raw))
        .collect::<Vec<_>>();

    fn dedup_scalings<T: PartialEq>(list: &mut Vec<T>) {
        let mut out = Vec::new();

        for item in core::mem::take(list) {
            if !out.contains(&item) {
                out.push(item);
            }
        }

        *list = out;
    }

    dedup_scalings(&mut out);
    out
}

fn parse_scaling_block(raw_block: &str) -> Scaling {
    let inner_raw = raw_block
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim_start_matches('+')
        .trim();

    let nested_blocks = extract_plus_paren_blocks(inner_raw);

    if !nested_blocks.is_empty() {
        let outer_raw = remove_nested_plus_blocks(&inner_raw);
        let outer = parse_non_nested_scaling(&outer_raw);
        let inner = nested_blocks
            .iter()
            .map(String::as_str)
            .map(parse_scaling_block)
            .collect::<Vec<_>>();

        return Scaling::Nested {
            raw: html_to_text(raw_block),
            outer: Box::new(outer),
            inner,
        };
    }

    parse_non_nested_scaling(&inner_raw)
}

fn parse_non_nested_scaling(raw: &str) -> Scaling {
    let text = normalize_text(&html_to_text(raw));

    if text.is_empty() {
        return Scaling::Other {
            raw: html_to_text(raw),
        };
    }

    if let Some(scaling) = Scaling::based_on_level(raw, &text)
        .or_else(|| Scaling::percent_attr(&text))
        .or_else(|| Scaling::ranked_per100(&text))
        .or_else(|| Scaling::ranked(&text))
        .or_else(|| Scaling::per100(&text))
        .or_else(|| Scaling::simple(&text))
        .or_else(|| Scaling::flat(&text))
    {
        return scaling;
    }

    Scaling::Other { raw: text }
}

fn extract_plus_paren_blocks(input: &str) -> Vec<String> {
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

fn remove_nested_plus_blocks(input: &str) -> String {
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

impl Scaling {
    fn simple(text: &str) -> Option<Scaling> {
        static SIMPLE_PERCENT_TAIL_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(.+)$").unwrap());

        let caps = SIMPLE_PERCENT_TAIL_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f32>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Simple { value, ctx_var })
    }

    fn ranked(text: &str) -> Option<Scaling> {
        static RANKED_PERCENT_TAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%\s+(.+)$").unwrap()
        });

        let caps = RANKED_PERCENT_TAIL_RE.captures(text)?;
        let values = parse_slash_f32s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Ranked { values, ctx_var })
    }

    fn per100(text: &str) -> Option<Scaling> {
        static SIMPLE_PER100_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+per\s+100%?\s+(.+)$").unwrap());

        let caps = SIMPLE_PER100_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f32>().ok()? / 100.0;
        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::Per100 { value, ctx_var })
    }

    fn ranked_per100(text: &str) -> Option<Scaling> {
        static RANKED_PER100_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)^(-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+)%\s+per\s+100%?\s+(.+)$")
                .unwrap()
        });

        let caps = RANKED_PER100_RE.captures(text)?;
        let values = parse_slash_f32s(caps.get(1)?.as_str())
            .into_iter()
            .map(|v| v / 100.0)
            .collect::<Vec<_>>();

        let ctx_var = assign_ctx_var(caps.get(2)?.as_str());

        Some(Scaling::RankedPer100 { values, ctx_var })
    }

    fn percent_attr(text: &str) -> Option<Scaling> {
        static PERCENT_ATTR_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(-?\d+(?:\.\d+)?)%\s+(of .+)$").unwrap());

        let caps = PERCENT_ATTR_RE.captures(text)?;
        let value = caps.get(1)?.as_str().parse::<f32>().ok()? / 100.0;
        let debug = caps.get(2)?.as_str().trim().to_string();
        let ctx_var = assign_ctx_var(&debug);

        Some(Scaling::PercentAttr {
            value,
            debug,
            ctx_var,
        })
    }

    fn flat(text: &str) -> Option<Scaling> {
        static RANKED_VALUES_ONLY_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+$").unwrap());

        if !RANKED_VALUES_ONLY_RE.is_match(text) {
            return None;
        }

        let values = parse_slash_f32s(text);
        if values.is_empty() {
            return None;
        }

        Some(Scaling::Flat { values })
    }

    fn based_on_level(raw: &str, text: &str) -> Option<Scaling> {
        if !text.to_ascii_lowercase().contains("based on level") {
            return None;
        }

        let fragment = Html::parse_fragment(raw);
        let tooltip = fragment.select(&selector("span.pp-tooltip").ok()?).next()?;

        let top_values = tooltip.value().attr("data-top-values")?;
        let bot_values = tooltip.value().attr("data-bot-values")?;

        fn get_values<T: FromStr>(text: &str) -> Vec<T> {
            text.split(';')
                .map(str::trim)
                .filter_map(|v| v.parse::<T>().ok())
                .collect::<Vec<_>>()
        }

        let starts = get_values::<u8>(top_values);
        let mut values = get_values::<f32>(bot_values);

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
}

fn build_level_arms(starts: &[u8], values: &[f32]) -> Vec<LevelArm> {
    let mut out = Vec::new();

    for (i, (&start, &value)) in starts.iter().zip(values.iter()).enumerate() {
        match starts.get(i + 1).copied() {
            Some(next) if i == 0 => out.push(LevelArm::To {
                end_exclusive: next,
                value,
            }),
            Some(next) => out.push(LevelArm::Range {
                start_inclusive: start,
                end_exclusive: next,
                value,
            }),
            None => out.push(LevelArm::From {
                start_inclusive: start,
                value,
            }),
        }
    }

    out
}

fn parse_slash_f32s(input: &str) -> Vec<f32> {
    input
        .split('/')
        .map(normalize_text)
        .filter_map(|v| v.parse::<f32>().ok())
        .collect()
}

fn normalize_text(input: &str) -> String {
    clean_text(input)
        .replace('\u{a0}', " ")
        .replace('\u{2013}', "-")
        .replace('\u{2014}', "-")
        .replace('\u{2212}', "-")
        .replace('\u{00D7}', "*")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .trim_start_matches('+')
        .trim()
        .to_string()
}

fn html_to_text(input: &str) -> String {
    clean_text(
        &Html::parse_fragment(input)
            .root_element()
            .text()
            .collect::<String>(),
    )
}

fn assign_ctx_var(input: &str) -> CtxVar {
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
            | in "mark"
            | in "stardust"
            | in "of damage stored"
            | in "mist"
            | in "grit" => CtxVar::Stacks;
        in "armor" | in "total armor" => CtxVar::Armor;
        in "magic resist" | in "total magic resist" => CtxVar::MagicResist;
        in "lethality" => CtxVar::ArmorPenetrationFlat
    };

    CtxVar::SteelcapsEffect
}
