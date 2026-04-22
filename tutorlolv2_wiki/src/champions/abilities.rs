use crate::{
    champions::{
        clean_text,
        template::{ChampionTemplate, SkillSet},
    },
    client::{MayFail, SyncMayFail, fetch},
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use regex::{Regex, RegexBuilder};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, sync::LazyLock};
use tutorlolv2_types::{CtxVar::*, DamageType};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedAbilityPage {
    pub champion_id: String,
    pub slot: char,
    pub variant: Option<usize>,
    pub source_file: String,

    pub name: String,
    pub damage_type: DamageType,
    pub spell_effects: Option<String>,

    pub effects: Vec<ParsedEffectBlock>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedEffectBlock {
    pub index: usize,
    pub suffix: String,
    pub description: Option<String>,
    pub raw_description: Option<String>,
    pub raw_leveling: Option<String>,
    pub levelings: Vec<ParsedLevelingLine>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParsedLevelingLine {
    pub leveling_index: usize,
    pub attribute: String,
    pub data: String,
    pub formula_attempt: Vec<String>,
    pub raw_html: String,
}

pub async fn download() -> MayFail {
    for entry in std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false))
    {
        let path = entry.path();

        let champion_id = entry
            .file_name()
            .into_string()
            .map_err(|e| format!("[error] Failed to get file name for entry: {entry:?}: {e:?}"))?;

        println!("[dir] Processing {champion_id:?}");

        let bytes = std::fs::read(path.join("template").with_extension("json"))
            .map_err(|e| format!("[error] Failed to read entry: {entry:?}: {e:?}"))?;

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
    let result: SyncMayFail = std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false))
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let path = entry.path();

            let champion_id = entry.file_name().into_string().map_err(|e| {
                format!("[error] Failed to get file name for entry: {entry:?}: {e:?}")
            })?;

            println!("[parallel] Processing {champion_id:?}");

            std::fs::read_dir(path.join("abilities"))
                .map_err(|e| format!("[error] Failed to read abilities directory: {e:?}"))?
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().ok().map(|v| v.is_file()).unwrap_or(false)
                        && entry
                            .path()
                            .extension()
                            .map(|v| v == "html")
                            .unwrap_or(false)
                })
                .par_bridge()
                .into_par_iter()
                .try_for_each(|entry| {
                    let f = || -> MayFail {
                        let file_name = entry.file_name().into_string().map_err(|e| {
                            format!("[error] Failed to get file name for entry: {entry:?}: {e:?}")
                        })?;

                        let key = file_name.chars().next().ok_or_else(|| {
                            format!("[error] Failed to get key from file name: {file_name:?}")
                        })?;

                        let path = entry.path();

                        let html = std::fs::read_to_string(&path)?;

                        let title_selector = Selector::parse("title")?;

                        let title = Html::parse_document(&html)
                            .select(&title_selector)
                            .next()
                            .ok_or_else(|| {
                                format!("[error] Failed to get title for file: {file_name:?}")
                            })?
                            .text()
                            .collect::<String>();

                        if title.contains("Too many requests") {
                            std::fs::remove_file(&path)
                                .map_err(|e| format!("[error] Failed to remove file: {e:?}"))?;

                            eprintln!("[{champion_id:?}] Too many requests for {file_name:?}");

                            return Ok(());
                        }

                        let parsed = parse_ability_html(&champion_id, key, &file_name, &html)?;

                        println!("[{champion_id:?}] Processing {file_name:?}");

                        std::fs::write(
                            path.with_extension("json"),
                            serde_json::to_string_pretty(&parsed)?,
                        )?;

                        Ok(())
                    };

                    f().map_err(|e| e.to_string().into())
                })
        });

    result.map_err(|e| e.to_string().into())
}

fn parse_ability_html(
    champion_id: &str,
    slot: char,
    file_name: &str,
    html: &str,
) -> MayFail<ParsedAbilityPage> {
    let doc = Html::parse_document(html);

    let table_selector = Selector::parse("table")?;
    let tr_selector = Selector::parse("tr")?;
    let cell_selector = Selector::parse("th, td")?;

    let mut value_texts = BTreeMap::<String, String>::new();
    let mut value_htmls = BTreeMap::<String, String>::new();

    for table in doc.select(&table_selector) {
        let rows = table.select(&tr_selector).collect::<Vec<_>>();

        if rows.is_empty() {
            continue;
        }

        let header = rows[0].text().collect::<String>().to_ascii_lowercase();

        if !(header.contains("parameter") && header.contains("value")) {
            continue;
        }

        let mut first_data_row = true;

        for row in rows.into_iter().skip(1) {
            let cells = row.select(&cell_selector).collect::<Vec<_>>();
            if cells.len() < 2 {
                continue;
            }

            let mut key = clean_text(&cells[0].text().collect::<String>());
            if key.is_empty() {
                continue;
            }

            if first_data_row && key == "1" {
                key = "name".to_string();
            }

            first_data_row = false;

            let value_text = clean_text(&cells[1].text().collect::<String>());
            let value_html = cells[1].inner_html();

            value_texts.insert(key.clone(), value_text);
            value_htmls.insert(key, value_html);
        }
    }

    let effects = build_effect_blocks(&value_texts, &value_htmls);

    Ok(ParsedAbilityPage {
        champion_id: champion_id.to_string(),
        slot,
        variant: parse_variant(file_name),
        source_file: file_name.to_string(),

        name: value_texts.get("name").cloned().unwrap_or_default(),
        damage_type: value_texts
            .get("damagetype")
            .cloned()
            .filter(|v| !v.is_empty())
            .and_then(|v| v.parse().ok())
            .unwrap_or_default(),
        spell_effects: value_texts
            .get("spelleffects")
            .cloned()
            .filter(|v| !v.is_empty()),

        effects,
    })
}

fn build_effect_blocks(
    value_texts: &BTreeMap<String, String>,
    value_htmls: &BTreeMap<String, String>,
) -> Vec<ParsedEffectBlock> {
    const ENDINGS: [&str; 12] = ["", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    let mut out = Vec::new();

    for suffix in ENDINGS {
        let desc_key = format!("description{suffix}");
        let lvl_key = format!("leveling{suffix}");

        let description = value_texts
            .get(&desc_key)
            .cloned()
            .filter(|v| !v.is_empty());
        let description_raw_html = value_htmls
            .get(&desc_key)
            .cloned()
            .filter(|v| !v.is_empty());

        let leveling_raw_html = value_htmls.get(&lvl_key).cloned().filter(|v| !v.is_empty());

        if description.is_none()
            && match &leveling_raw_html {
                Some(v) if v.trim().is_empty() => true,
                Some(_) => false,
                None => true,
            }
        {
            continue;
        }

        let effect_index = out.len();

        let levelings = leveling_raw_html
            .as_deref()
            .map(parse_leveling_lines)
            .unwrap_or_default();

        out.push(ParsedEffectBlock {
            index: effect_index,
            suffix: suffix.to_string(),
            description,
            raw_description: description_raw_html,
            raw_leveling: leveling_raw_html,
            levelings,
        });
    }

    out
}

fn parse_leveling_lines(raw_html: &str) -> Vec<ParsedLevelingLine> {
    let fragment = Html::parse_fragment(raw_html);
    let selector = Selector::parse("dt, dd").unwrap();

    let mut parts = Vec::<(String, String, String)>::new();

    for element in fragment.select(&selector) {
        let tag = element.value().name().to_string();
        let text = clean_text(&element.text().collect::<String>());
        let inner = element.inner_html();
        parts.push((tag, text, inner));
    }

    if parts.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 1 < parts.len() {
        let (tag_a, attr, _) = &parts[i];
        let (tag_b, data, raw) = &parts[i + 1];

        if tag_a == "dt" && tag_b == "dd" {
            out.push(ParsedLevelingLine {
                leveling_index: out.len(),
                attribute: attr.trim_end_matches(':').trim().to_string(),
                formula_attempt: try_parse_formula_attempt(data),
                data: data.clone(),
                raw_html: raw.clone(),
            });
            i += 2;
        } else {
            i += 1;
        }
    }

    out
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

static SLASH_NUMBERS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-?\d+(?:\.\d+)?(?:\s*/\s*-?\d+(?:\.\d+)?)+").unwrap());

static SINGLE_NUMBER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-?\d+(?:\.\d+)?").unwrap());

static PERCENTAGES_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+(?:\.\d+)?)%").unwrap());

static MULTISPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

const REPLACEMENTS: &[(&str, &dyn Display)] = &[
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
    ("of turret's ", &""),
    ("\u{00D7}", &"*"),
];

fn try_parse_formula_attempt(data: &str) -> Vec<String> {
    let data = normalize_formula_input(data);
    if data.is_empty() {
        return Vec::new();
    }

    let parens = extract_top_level_parens(&data);
    let base = remove_top_level_parens(&data);

    let rank_count = detect_rank_count(&base)
        .or_else(|| parens.iter().find_map(|p| detect_rank_count(p)))
        .unwrap_or(0);

    if rank_count == 0 {
        return Vec::new();
    }

    let mut per_rank = vec![Vec::<String>::new(); rank_count];

    if let Some(terms) = parse_formula_component(&base, rank_count) {
        let terms = normalize_term_count(terms, rank_count);

        for (i, term) in terms.into_iter().enumerate() {
            if !term.is_empty() {
                per_rank[i].push(term);
            }
        }
    }

    for paren in parens {
        let inner = paren
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim();

        if let Some(terms) = parse_formula_component(inner, rank_count) {
            let terms = normalize_term_count(terms, rank_count);

            for (i, term) in terms.into_iter().enumerate() {
                if !term.is_empty() {
                    per_rank[i].push(term);
                }
            }
        }
    }

    per_rank
        .into_iter()
        .map(|parts| {
            if parts.is_empty() {
                return String::new();
            }
            cleanup_formula(parts.join(" + "))
        })
        .collect()
}

fn normalize_term_count(mut terms: Vec<String>, rank_count: usize) -> Vec<String> {
    match (terms.len(), rank_count) {
        (a, b) if a == b => terms,
        (1, n) if n > 1 => vec![terms.remove(0); n],
        (5, 3) => vec![terms[0].clone(), terms[2].clone(), terms[4].clone()],
        (a, b) if a > b => {
            terms.truncate(b);
            terms
        }
        (a, b) if a < b && !terms.is_empty() => {
            let last = terms.last().cloned().unwrap();
            while terms.len() < b {
                terms.push(last.clone());
            }
            terms
        }
        _ => vec![String::new(); rank_count],
    }
}

fn parse_formula_component(chunk: &str, rank_count: usize) -> Option<Vec<String>> {
    let chunk = normalize_formula_input(chunk);
    if chunk.is_empty() {
        return None;
    }

    let out = if !chunk.contains('%') {
        if let Some(values) = parse_ranked_numbers(&chunk) {
            values.into_iter().map(trim_f64).collect::<Vec<_>>()
        } else if let Some(value) = parse_single_number(&chunk) {
            (0..rank_count).map(|_| trim_f64(value)).collect::<Vec<_>>()
        } else {
            return None;
        }
    } else {
        let (left, right) = chunk.split_once('%')?;
        let coeffs = parse_ranked_numbers(left)
            .or_else(|| parse_single_number(left).map(|v| vec![v; rank_count]))?;

        let mut suffix = normalize_formula_input(right);
        suffix = replace_percentages(&suffix);
        suffix = apply_replacements(&suffix);
        suffix = cleanup_formula(suffix);

        let mut out = Vec::with_capacity(coeffs.len());
        for coeff in coeffs {
            let coeff = coeff / 100.0;
            let coeff = trim_f64(coeff);

            if suffix.is_empty() {
                out.push(coeff);
            } else {
                out.push(cleanup_formula(format!("{coeff} * {suffix}")));
            }
        }
        out
    };

    Some(normalize_term_count(out, rank_count))
}

fn parse_ranked_numbers(s: &str) -> Option<Vec<f64>> {
    let m = SLASH_NUMBERS_RE.find(s)?;
    let values = m
        .as_str()
        .split('/')
        .map(|x| x.trim().parse::<f64>().ok())
        .collect::<Option<Vec<_>>>()?;

    Some(values)
}

fn parse_single_number(s: &str) -> Option<f64> {
    let m = SINGLE_NUMBER_RE.find(s)?;
    m.as_str().parse::<f64>().ok()
}

fn detect_rank_count(s: &str) -> Option<usize> {
    parse_ranked_numbers(s).map(|v| v.len())
}

fn normalize_formula_input(s: &str) -> String {
    let s = s
        .replace('\u{00a0}', " ")
        .replace('\u{2013}', "-")
        .replace('\u{2212}', "-")
        .replace('\u{00D7}', "*");

    MULTISPACE_RE.replace_all(s.trim(), " ").to_string()
}

fn replace_percentages(s: &str) -> String {
    PERCENTAGES_RE
        .replace_all(s, |caps: &regex::Captures| {
            let num = caps
                .get(1)
                .and_then(|m| m.as_str().parse::<f64>().ok())
                .unwrap_or(0.0);
            format!("{} *", trim_f64(num / 100.0))
        })
        .to_string()
}

fn apply_replacements(input: &str) -> String {
    let mut pairs = REPLACEMENTS.to_vec();
    pairs.sort_by_key(|(from, _)| usize::MAX - from.len());

    let mut out = input.to_string();

    for (from, to) in pairs {
        let re = RegexBuilder::new(&regex::escape(from))
            .case_insensitive(true)
            .build()
            .unwrap();
        out = re.replace_all(&out, to.to_string()).to_string();
    }

    out
}

fn cleanup_formula(s: String) -> String {
    let mut out = s;

    out = out.replace("( ", "(").replace(" )", ")");
    out = out.replace("+ -", "- ");
    out = out.replace("* +", "* ");
    out = out.replace("  ", " ");

    while out.contains("  ") {
        out = out.replace("  ", " ");
    }

    out.trim().trim_matches('+').trim().to_string()
}

fn extract_top_level_parens(s: &str) -> Vec<String> {
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

fn remove_top_level_parens(s: &str) -> String {
    let mut result = String::new();
    let mut depth = 0usize;

    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth == 0 => result.push(ch),
            _ => {}
        }
    }

    normalize_formula_input(&result)
}

fn trim_f64(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.0}", v)
    } else {
        let s = v.to_string();
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
