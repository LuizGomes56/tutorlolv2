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
use std::{collections::BTreeMap, sync::LazyLock};
use tutorlolv2_types::{CtxVar, DamageType};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedAbilityPage {
    pub champion_id: String,
    pub slot: char,
    pub variant: Option<usize>,
    pub source_file: String,

    pub name: Option<String>,
    pub skill: Option<String>,
    pub damage_type: Option<DamageType>,
    pub spell_effects: Option<String>,

    pub parsed: ParsedAbilityCells,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedAbilityCells {
    pub descriptions: Vec<DescriptionBlock>,
    pub levelings: Vec<LevelingBlock>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DescriptionBlock {
    pub index: usize,
    pub text: String,
    pub raw: String,
    pub tooltips: Vec<TooltipFormula>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LevelingBlock {
    pub index: usize,
    pub raw: String,
    pub rows: Vec<LevelingRow>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LevelingRow {
    pub index: usize,
    pub description: String,
    pub text: String,
    pub raw: String,

    // Primeiro tooltip da linha, quando existir.
    pub formula: Option<String>,
    pub formula_linear: Vec<String>,

    // Valores lineares principais da linha (geralmente os flats rankeados).
    pub values_linear: Vec<String>,

    // Expressões como "0.1 * ctx.ability_power".
    pub scalings: Vec<String>,

    // Todos os tooltips encontrados dentro do dd.
    pub tooltips: Vec<TooltipFormula>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TooltipFormula {
    pub raw: String,
    pub text: String,

    // data-useformula
    pub formula: Option<String>,

    // data-bot-values
    pub formula_linear: Vec<String>,

    // data-bot-key, ex: "%"
    pub bot_key: Option<String>,

    // data-top-*
    pub axis_label: Option<String>,
    pub axis_key: Option<String>,
    pub axis_values: Vec<String>,

    // Quando o tooltip representa algo como "% of target's maximum health",
    // o contexto fica aqui para facilitar a composição posterior.
    pub value_context: Option<String>,

    // Escalings adicionais detectados perto do tooltip.
    pub scalings: Vec<String>,
}

pub async fn download() -> MayFail {
    println!("[fn] champions::abilities::download");

    for entry in crate::read_dir(cache())?.filter(is_dir) {
        let path = entry.path();

        let champion_id = file_name(&entry)?;

        println!("[dir] Processing {champion_id:?}");

        let bytes = crate::read(path.join("template").with_extension("json"))?;

        let data = serde_json::from_slice::<ChampionTemplate>(&bytes)
            .map_err(|e| format!("[error] Failed to deserialize entry: {entry:?}: {e:?}"))?;

        let SkillSet {
            skill_i,
            skill_q,
            skill_w,
            skill_e,
            skill_r,
        } = data.skills;

        let save_to = path.join("abilities");

        std::fs::create_dir_all(&save_to)
            .map_err(|e| format!("[error] Failed to create directory: {e:?}"))?;

        for (key, skills) in [
            ('P', skill_i),
            ('Q', skill_q),
            ('W', skill_w),
            ('E', skill_e),
            ('R', skill_r),
        ] {
            for (i, skill) in skills.into_iter().enumerate() {
                let url = format!("Template:Data_{champion_id}/{skill}");

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
                    let file_name = file_name(&entry)?;

                    let key = file_name.chars().next().ok_or_else(|| {
                        format!("[error] Failed to get key from file name: {file_name:?}")
                    })?;

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
                    let parsed = parse_ability_page(&champion_id, key, &file_name, cells)?;

                    std::fs::write(
                        path.with_extension("json"),
                        serde_json::to_string_pretty(&parsed)?,
                    )?;

                    Ok(())
                })
        })
}

fn parse_ability_page(
    champion_id: &str,
    slot: char,
    file_name: &str,
    cells: BTreeMap<String, String>,
) -> MayFail<ParsedAbilityPage> {
    let parsed = parse_abilities(cells.clone())?;

    Ok(ParsedAbilityPage {
        champion_id: champion_id.to_string(),
        slot: normalize_slot(
            cells
                .get("skill")
                .and_then(|s| s.chars().next())
                .unwrap_or(slot),
        ),
        variant: parse_variant(file_name),
        source_file: file_name.to_string(),

        name: cells.get("1").cloned(),
        skill: cells
            .get("skill")
            .cloned()
            .map(|s| if s == "I" { "P".to_string() } else { s }),
        damage_type: cells
            .get("damagetype")
            .and_then(|v| v.parse::<DamageType>().ok()),
        spell_effects: cells.get("spelleffects").cloned(),

        parsed,
    })
}

fn parse_abilities(cells: BTreeMap<String, String>) -> MayFail<ParsedAbilityCells> {
    let mut descriptions = Vec::new();
    let mut levelings = Vec::new();

    for idx in 0..=10 {
        let suffix = if idx == 0 {
            String::new()
        } else {
            idx.to_string()
        };

        let description_key = format!("description{suffix}");
        let description_raw_key = format!("description{suffix}_raw");

        if let Some(raw) = cells.get(&description_raw_key) {
            descriptions.push(parse_description_block(
                idx,
                cells.get(&description_key).cloned().unwrap_or_default(),
                raw.clone(),
            )?);
        } else if let Some(text) = cells.get(&description_key) {
            descriptions.push(parse_description_block(idx, text.clone(), String::new())?);
        }

        let leveling_key = format!("leveling{suffix}");
        let leveling_raw_key = format!("leveling{suffix}_raw");

        if let Some(raw) = cells.get(&leveling_raw_key) {
            levelings.push(parse_leveling_block(
                idx,
                cells.get(&leveling_key).cloned().unwrap_or_default(),
                raw.clone(),
            )?);
        } else if let Some(text) = cells.get(&leveling_key) {
            levelings.push(parse_leveling_block(idx, text.clone(), String::new())?);
        }
    }

    Ok(ParsedAbilityCells {
        descriptions,
        levelings,
    })
}

fn parse_description_block(index: usize, text: String, raw: String) -> MayFail<DescriptionBlock> {
    Ok(DescriptionBlock {
        index,
        text,
        raw: raw.clone(),
        tooltips: extract_tooltips(&raw),
    })
}

fn parse_leveling_block(index: usize, text: String, raw: String) -> MayFail<LevelingBlock> {
    if raw.trim().is_empty() {
        return Ok(LevelingBlock {
            index,
            raw,
            rows: if text.trim().is_empty() {
                Vec::new()
            } else {
                vec![LevelingRow {
                    index: 0,
                    description: String::new(),
                    text: text.clone(),
                    raw: text.clone(),
                    formula: None,
                    formula_linear: Vec::new(),
                    values_linear: parse_first_ranked_values(&text),
                    scalings: parse_scalings(&text),
                    tooltips: Vec::new(),
                }]
            },
        });
    }

    let fragment = Html::parse_fragment(&raw);

    let dt_selector = selector("dt")?;
    let dd_selector = selector("dd")?;
    let label_selector = selector("span.template_lc")?;

    let dts = fragment.select(&dt_selector).collect::<Vec<_>>();
    let dds = fragment.select(&dd_selector).collect::<Vec<_>>();

    let mut rows = Vec::new();

    for (i, (dt, dd)) in dts.into_iter().zip(dds.into_iter()).enumerate() {
        let description = dt
            .select(&label_selector)
            .next()
            .map(|e| clean_text(&e.text().collect::<String>()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| clean_text(&dt.text().collect::<String>()))
            .trim_end_matches(':')
            .trim()
            .to_string();

        let dd_text = clean_text(&dd.text().collect::<String>());
        let dd_raw = dd.inner_html();

        let tooltips = extract_tooltips(&dd_raw);
        let first_tooltip = tooltips.first();

        rows.push(LevelingRow {
            index: i,
            description,
            text: dd_text.clone(),
            raw: dd_raw.clone(),
            formula: first_tooltip.and_then(|t| t.formula.clone()),
            formula_linear: first_tooltip
                .map(|t| t.formula_linear.clone())
                .unwrap_or_default(),
            values_linear: parse_first_ranked_values(&dd_text),
            scalings: parse_scalings(&dd_text),
            tooltips,
        });
    }

    Ok(LevelingBlock { index, raw, rows })
}

fn extract_tooltips(raw_html: &str) -> Vec<TooltipFormula> {
    let mut out = Vec::new();

    for cap in PP_TOOLTIP_RE.captures_iter(raw_html) {
        let full = cap.get(0).map(|m| m.as_str()).unwrap_or_default();
        let attrs_raw = cap.name("attrs").map(|m| m.as_str()).unwrap_or_default();
        let body_html = cap.name("body").map(|m| m.as_str()).unwrap_or_default();
        let body_text = html_to_text(body_html);

        let attrs = parse_attrs(attrs_raw);

        let match_end = cap.get(0).map(|m| m.end()).unwrap_or(0);
        let context_html = tooltip_context_window(raw_html, match_end);
        let context_text = clean_tooltip_context(context_html);

        let mut scalings = Vec::new();
        scalings.extend(parse_scalings(&body_text));
        scalings.extend(parse_scalings(&context_text));
        dedup_strings(&mut scalings);

        out.push(TooltipFormula {
            raw: full.to_string(),
            text: body_text,
            formula: attrs.get("data-useformula").cloned(),
            formula_linear: attrs
                .get("data-bot-values")
                .map(|v| split_values(v))
                .unwrap_or_default(),
            bot_key: attrs.get("data-bot-key").cloned(),
            axis_label: attrs.get("data-top-label").cloned(),
            axis_key: attrs.get("data-top-key").cloned(),
            axis_values: attrs
                .get("data-top-values")
                .map(|v| split_values(v))
                .unwrap_or_default(),
            value_context: infer_value_context(&context_text),
            scalings,
        });
    }

    out
}

fn parse_first_ranked_values(text: &str) -> Vec<String> {
    SLASH_NUMBERS_RE
        .find(text)
        .map(|m| {
            m.as_str()
                .split('/')
                .map(|s| normalize_num(s.trim()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn parse_scalings(text: &str) -> Vec<String> {
    let normalized = normalize_formula_input(text);
    if normalized.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();

    // 1) Caso composto:
    // 240% (+ 750% per 100% bonus attack speed) bonus AD
    // -> (7.5 * ctx.attack_speed) * ctx.bonus_ad
    for cap in COMPOSITE_SCALING_RE.captures_iter(&normalized) {
        let bonus = cap.name("bonus").map(|m| m.as_str()).unwrap_or_default();
        let driver = cap.name("driver").map(|m| m.as_str()).unwrap_or_default();
        let target = cap.name("target").map(|m| m.as_str()).unwrap_or_default();

        let driver_var = parse_ctx_var(driver);
        let target_var = parse_ctx_var(target);

        if let (Some(driver_var), Some(target_var)) = (driver_var, target_var) {
            for coeff in split_numeric_values(bonus) {
                out.push(format!(
                    "({} * {}) * {}",
                    trim_f64(coeff / 100.0),
                    driver_var,
                    target_var
                ));
            }
        }
    }

    // 2) Analisa conteúdos parentéticos e, se não houver, o texto inteiro só quando
    // parece explicitamente uma escala.
    let mut chunks = extract_parenthetical_chunks(&normalized);

    if chunks.is_empty() && looks_like_scaling_chunk(&normalized) {
        chunks.push(normalized.clone());
    }

    for mut chunk in chunks {
        chunk = chunk
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim()
            .trim_start_matches('+')
            .trim()
            .to_string();

        if chunk.is_empty() {
            continue;
        }

        // (+ 50% per 100% bonus attack speed)
        if let Some(caps) = SIMPLE_PER100_RE.captures(&chunk) {
            let values = caps.name("values").map(|m| m.as_str()).unwrap_or_default();
            let stat = caps.name("stat").map(|m| m.as_str()).unwrap_or_default();

            if let Some(var) = parse_ctx_var(stat) {
                for coeff in split_numeric_values(values) {
                    out.push(format!("{} * {}", trim_f64(coeff / 100.0), var));
                }
            }
            continue;
        }

        // (+ 9% of target's maximum health)
        if let Some(caps) = OF_TARGET_RE.captures(&chunk) {
            let values = caps.name("values").map(|m| m.as_str()).unwrap_or_default();
            let target = caps.name("target").map(|m| m.as_str()).unwrap_or_default();

            if let Some(var) = parse_ctx_var(target) {
                for coeff in split_numeric_values(values) {
                    out.push(format!("{} * {}", trim_f64(coeff / 100.0), var));
                }
            }
            continue;
        }

        // (+ 10% AP), (+ 50% bonus AD)
        if let Some(caps) = SIMPLE_PERCENT_RE.captures(&chunk) {
            let values = caps.name("values").map(|m| m.as_str()).unwrap_or_default();
            let stat = caps.name("stat").map(|m| m.as_str()).unwrap_or_default();

            if let Some(var) = parse_ctx_var(stat) {
                for coeff in split_numeric_values(values) {
                    out.push(format!("{} * {}", trim_f64(coeff / 100.0), var));
                }
            }
        }
    }

    dedup_strings(&mut out);
    out
}

fn normalize_slot(slot: char) -> char {
    if slot == 'I' { 'P' } else { slot }
}

fn parse_variant(file_name: &str) -> Option<usize> {
    let stem = file_name.strip_suffix(".html").unwrap_or(file_name);

    let digits = stem
        .chars()
        .skip(1)
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    digits.parse::<usize>().ok()
}

fn parse_ctx_var(input: &str) -> Option<CtxVar> {
    let s = normalize_formula_input(input).to_ascii_lowercase();

    if s.contains("target's maximum health") || s.contains("of the target's maximum health") {
        return Some(CtxVar::EnemyMaxHealth);
    }
    if s.contains("target's current health") || s.contains("of the target's current health") {
        return Some(CtxVar::EnemyCurrentHealth);
    }
    if s.contains("target's missing health") || s.contains("of the target's missing health") {
        return Some(CtxVar::EnemyMissingHealth);
    }
    if s.contains("bonus attack speed") {
        return Some(CtxVar::AttackSpeed);
    }
    if s.contains("critical strike chance") {
        return Some(CtxVar::CritChance);
    }
    if s.contains("bonus movement speed") {
        return Some(CtxVar::BonusMoveSpeed);
    }
    if s.contains("bonus magic resistance") {
        return Some(CtxVar::BonusMagicResist);
    }
    if s.contains("bonus armor") {
        return Some(CtxVar::BonusArmor);
    }
    if s.contains("bonus mana") {
        return Some(CtxVar::BonusMana);
    }
    if s.contains("bonus health") {
        return Some(CtxVar::BonusHealth);
    }
    if s.contains("base ad") {
        return Some(CtxVar::BaseAd);
    }
    if s == "ad" || s.ends_with(" ad") || s.contains(" attack damage") {
        return Some(CtxVar::AttackDamage);
    }
    if s == "ap" || s.ends_with(" ap") || s.contains("ability power") {
        return Some(CtxVar::AbilityPower);
    }
    if s.contains("maximum mana") {
        return Some(CtxVar::MaxMana);
    }
    if s.contains("maximum health") {
        return Some(CtxVar::MaxHealth);
    }
    if s.contains("current health") {
        return Some(CtxVar::CurrentHealth);
    }
    if s.contains("current mana") {
        return Some(CtxVar::CurrentMana);
    }
    if s.contains("missing health") {
        return Some(CtxVar::MissingHealth);
    }
    if s == "armor" || s.ends_with(" armor") {
        return Some(CtxVar::Armor);
    }
    if s.contains("magic resistance") || s == "mr" {
        return Some(CtxVar::MagicResist);
    }
    if s.contains("stacks") || s.contains(" mark") || s == "mark" || s.contains("stardust") {
        return Some(CtxVar::Stacks);
    }

    None
}

fn infer_value_context(context_text: &str) -> Option<String> {
    parse_ctx_var(context_text).map(|v| v.to_string())
}

fn split_values(s: &str) -> Vec<String> {
    s.split(';')
        .map(|v| normalize_num(v.trim()))
        .filter(|v| !v.is_empty())
        .collect()
}

fn split_numeric_values(s: &str) -> Vec<f64> {
    s.split('/')
        .map(|v| v.trim())
        .filter_map(|v| v.parse::<f64>().ok())
        .collect()
}

fn html_to_text(s: &str) -> String {
    clean_text(&TAG_RE.replace_all(s, " ").to_string())
}

fn parse_attrs(attrs: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    for cap in ATTR_RE.captures_iter(attrs) {
        map.insert(cap[1].to_string(), cap[2].to_string());
    }

    map
}

fn tooltip_context_window(raw_html: &str, match_end: usize) -> &str {
    let rest = raw_html.get(match_end..).unwrap_or("");

    let next_tooltip = rest.find("class=\"pp-tooltip\"");
    let end = next_tooltip.unwrap_or(rest.len());

    &rest[..end]
}

fn clean_tooltip_context(raw_html_after_tooltip: &str) -> String {
    let mut text = html_to_text(raw_html_after_tooltip);
    text = normalize_formula_input(&text);

    text = truncate_case_insensitive(&text, ", capped at").to_string();
    text = truncate_case_insensitive(&text, ";").to_string();
    text = truncate_case_insensitive(&text, ". ").to_string();
    text = truncate_case_insensitive(&text, ".").to_string();

    text.trim_start_matches(|c: char| c == ',' || c.is_whitespace())
        .trim()
        .to_string()
}

fn truncate_case_insensitive<'a>(s: &'a str, needle: &str) -> &'a str {
    let hay = s.to_ascii_lowercase();
    let nee = needle.to_ascii_lowercase();

    if let Some(idx) = hay.find(&nee) {
        &s[..idx]
    } else {
        s
    }
}

fn looks_like_scaling_chunk(s: &str) -> bool {
    s.contains('%') && (s.contains('+') || s.contains("per 100") || s.contains(" of "))
}

fn extract_parenthetical_chunks(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = None;

    for (idx, ch) in s.char_indices() {
        match ch {
            '(' => {
                if depth == 0 {
                    start = Some(idx);
                }
                depth += 1;
            }
            ')' => {
                if depth > 0 {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(begin) = start.take() {
                            out.push(s[begin..=idx].to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    out
}

fn normalize_formula_input(s: &str) -> String {
    let s = s
        .replace('\u{00a0}', " ")
        .replace('\u{2013}', "-")
        .replace('\u{2014}', "-")
        .replace('\u{2212}', "-")
        .replace('\u{00D7}', "*");

    MULTISPACE_RE.replace_all(s.trim(), " ").to_string()
}

fn normalize_num(s: &str) -> String {
    s.parse::<f64>()
        .map(trim_f64)
        .unwrap_or_else(|_| s.trim().to_string())
}

fn trim_f64(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.0}", v)
    } else {
        let s = v.to_string();
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn dedup_strings(v: &mut Vec<String>) {
    v.retain(|s| !s.trim().is_empty());

    let mut out = Vec::new();
    for item in std::mem::take(v) {
        if !out.contains(&item) {
            out.push(item);
        }
    }
    *v = out;
}

static PP_TOOLTIP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<span[^>]*class="[^"]*\bpp-tooltip\b[^"]*"(?P<attrs>[^>]*)>(?P<body>.*?)</span>"#,
    )
    .unwrap()
});

static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"([a-zA-Z0-9_-]+)="([^"]*)""#).unwrap());

static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<[^>]+>").unwrap());

static MULTISPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

static SLASH_NUMBERS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+").unwrap());

// 240% (+ 750% per 100% bonus attack speed) bonus AD
static COMPOSITE_SCALING_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?ix)
        (?P<base>\d+(?:\.\d+)?)%
        \s*
        \(\+\s*
        (?P<bonus>\d+(?:\.\d+)?(?:\s*/\s*\d+(?:\.\d+)?)*)%
        \s*per\s*100%?\s*
        (?P<driver>[^)]+?)
        \)
        \s*
        (?P<target>[A-Za-z][^,;.]*)"#,
    )
    .unwrap()
});

// 50% per 100% bonus attack speed
static SIMPLE_PER100_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?ix)
        (?P<values>\d+(?:\.\d+)?(?:\s*/\s*\d+(?:\.\d+)?)*)%
        \s*per\s*100%?\s*
        (?P<stat>[A-Za-z][^,;.)]*)"#,
    )
    .unwrap()
});

// 9% of target's maximum health
static OF_TARGET_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?ix)
        (?P<values>\d+(?:\.\d+)?(?:\s*/\s*\d+(?:\.\d+)?)*)%
        \s*of\s*
        (?P<target>[A-Za-z][^,;.)]*)"#,
    )
    .unwrap()
});

// 10% AP, 50% bonus AD
static SIMPLE_PERCENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?ix)
        (?P<values>\d+(?:\.\d+)?(?:\s*/\s*\d+(?:\.\d+)?)*)%
        \s*
        (?P<stat>[A-Za-z][^,;.)]*)"#,
    )
    .unwrap()
});
