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
    pub from_description: bool,
    pub level_ranges: Vec<(usize, usize)>,
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

        let mut levelings = leveling_raw_html
            .as_deref()
            .map(parse_leveling_lines)
            .unwrap_or_default();

        if levelings.is_empty() {
            if let Some(raw_desc) = description_raw_html.as_deref() {
                levelings = parse_description_tooltips(raw_desc);
            }
        }

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
                data: data.clone(),
                formula_attempt: try_parse_formula_attempt(data),
                raw_html: raw.clone(),
                from_description: false,
                level_ranges: Vec::new(),
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

static PP_TOOLTIP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<span[^>]*class="[^"]*\bpp-tooltip\b[^"]*"(?P<attrs>[^>]*)>(?P<body>.*?)</span>"#,
    )
    .unwrap()
});

static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"([a-zA-Z0-9_-]+)="([^"]*)""#).unwrap());

static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<[^>]+>").unwrap());

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
    ("of expended Grit", &"* (ctx.current_mana / ctx.max_mana)"),
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
    ("bonus magic damage", &""),
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

    // exceções primeiro
    if let Some(result) = try_parse_formula_exception(&data, rank_count) {
        return result;
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
    let mut placeholders = Vec::<(String, String)>::new();

    for (idx, (from, to)) in pairs.into_iter().enumerate() {
        let escaped = regex::escape(from);

        let starts_word = from
            .chars()
            .next()
            .map(|c| c.is_ascii_alphanumeric())
            .unwrap_or(false);

        let ends_word = from
            .chars()
            .last()
            .map(|c| c.is_ascii_alphanumeric())
            .unwrap_or(false);

        let pattern = match (starts_word, ends_word) {
            (true, true) => format!(r"\b{escaped}\b"),
            (true, false) => format!(r"\b{escaped}"),
            (false, true) => format!(r"{escaped}\b"),
            (false, false) => escaped,
        };

        let re = RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .unwrap();

        if !re.is_match(&out) {
            continue;
        }

        let placeholder = format!("__CTX_REPL_{idx}__");
        out = re.replace_all(&out, placeholder.as_str()).to_string();
        placeholders.push((placeholder, to.to_string()));
    }

    for (placeholder, value) in placeholders {
        out = out.replace(&placeholder, &value);
    }

    out
}

fn cleanup_formula(s: String) -> String {
    let mut out = s
        .replace("( ", "(")
        .replace(" )", ")")
        .replace("+ -", "- ")
        .replace("* +", "* ")
        .replace("  ", " ");

    out = normalize_inline_percent_products(&out);

    while out.contains("  ") {
        out = out.replace("  ", " ");
    }

    out.trim().trim_matches('+').trim().replace("(+ ", "(")
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

fn parse_semicolon_f64s(s: &str) -> Vec<f64> {
    s.split(';')
        .filter_map(|x| x.trim().parse::<f64>().ok())
        .collect()
}

fn parse_semicolon_usizes(s: &str) -> Vec<usize> {
    s.split(';')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect()
}

fn build_level_ranges(starts: &[usize], max_level: usize) -> Vec<(usize, usize)> {
    if starts.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for (i, start) in starts.iter().copied().enumerate() {
        let end = starts
            .get(i + 1)
            .copied()
            .map(|v| v.saturating_sub(1))
            .unwrap_or(max_level);
        out.push((start, end));
    }
    out
}

fn infer_suffix_from_context(context_html: &str) -> String {
    let text = clean_tooltip_context(context_html);

    if text.is_empty() {
        return String::new();
    }

    // "against monsters" é qualificador do cap, não escala
    if text.to_ascii_lowercase().starts_with("against ") {
        return String::new();
    }

    let replaced = cleanup_formula(apply_replacements(&text));

    if replaced.contains("ctx.") {
        return replaced;
    }

    String::new()
}

fn parse_description_tooltips(raw_description: &str) -> Vec<ParsedLevelingLine> {
    let mut out = Vec::new();

    for cap in PP_TOOLTIP_RE.captures_iter(raw_description) {
        let full = cap.get(0).map(|m| m.as_str()).unwrap_or_default();
        let attrs_raw = cap.name("attrs").map(|m| m.as_str()).unwrap_or_default();
        let body_html = cap.name("body").map(|m| m.as_str()).unwrap_or_default();
        let body_text = html_to_text(body_html);

        let attrs = parse_attrs(attrs_raw);

        let match_end = cap.get(0).map(|m| m.end()).unwrap_or(0);

        let context_html = tooltip_context_window(raw_description, match_end);
        let suffix = infer_suffix_from_context(context_html);
        // let attribute =
        //     infer_description_attribute(raw_description, &body_text, &suffix, context_html);

        let bot_values = attrs
            .get("data-bot-values")
            .map(|s| parse_semicolon_f64s(s))
            .unwrap_or_default();

        let top_values = attrs
            .get("data-top-values")
            .map(|s| parse_semicolon_usizes(s))
            .unwrap_or_default();

        let is_percent = attrs
            .get("data-bot-key")
            .map(|v| v == "%")
            .unwrap_or_else(|| body_text.contains('%'));

        let (formula_attempt, level_ranges) = if !top_values.is_empty() && !bot_values.is_empty() {
            let ranges = build_level_ranges(&top_values, 20);
            let formulas = bot_values
                .into_iter()
                .map(|v| {
                    if !suffix.is_empty() {
                        if is_percent {
                            format!("{} * {}", trim_f64(v / 100.0), suffix)
                        } else {
                            format!("{} * {}", trim_f64(v), suffix)
                        }
                    } else if is_percent {
                        trim_f64(v / 100.0)
                    } else {
                        trim_f64(v)
                    }
                })
                .collect::<Vec<_>>();

            (formulas, ranges)
        } else if !bot_values.is_empty() {
            let formulas = bot_values
                .into_iter()
                .map(|v| {
                    if !suffix.is_empty() {
                        if is_percent {
                            format!("{} * {}", trim_f64(v / 100.0), suffix)
                        } else {
                            format!("{} * {}", trim_f64(v), suffix)
                        }
                    } else if is_percent {
                        trim_f64(v / 100.0)
                    } else {
                        trim_f64(v)
                    }
                })
                .collect::<Vec<_>>();

            (formulas, Vec::new())
        } else {
            (Vec::new(), Vec::new())
        };

        if formula_attempt.is_empty() {
            continue;
        }

        let attribute =
            infer_description_attribute(raw_description, &body_text, &suffix, context_html);

        out.push(ParsedLevelingLine {
            leveling_index: out.len(),
            attribute,
            data: body_text,
            formula_attempt: formula_attempt.into_iter().map(cleanup_formula).collect(),
            raw_html: full.to_string(),
            from_description: true,
            level_ranges,
        });
    }

    out
}

fn infer_description_attribute(
    raw_description: &str,
    body_text: &str,
    suffix: &str,
    context_html: &str,
) -> String {
    let desc = normalize_formula_input(&html_to_text(raw_description)).to_ascii_lowercase();
    let body = body_text.to_ascii_lowercase();
    let context = clean_tooltip_context(context_html).to_ascii_lowercase();

    if context.starts_with("against monsters") {
        return "Description Cap Against Monsters".to_string();
    }

    if desc.contains("damage") && suffix.contains("ctx.enemy_max_health") {
        return "Description Damage".to_string();
    }

    if desc.contains("capped at") {
        return "Description Cap".to_string();
    }

    if body.contains("based on level") {
        return "Description Scaling".to_string();
    }

    "Description Tooltip".to_string()
}

fn tooltip_context_window(raw_description: &str, match_end: usize) -> &str {
    let rest = raw_description.get(match_end..).unwrap_or("");

    let next_tooltip = rest.find("class=\"pp-tooltip\"");
    let end = next_tooltip.unwrap_or(rest.len());

    &rest[..end]
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

fn clean_tooltip_context(raw_html_after_tooltip: &str) -> String {
    let mut text = html_to_text(raw_html_after_tooltip);
    text = normalize_formula_input(&text);

    text = truncate_case_insensitive(&text, ", capped at").to_string();
    text = truncate_case_insensitive(&text, " capped at ").to_string();
    text = truncate_case_insensitive(&text, ". ").to_string();
    text = truncate_case_insensitive(&text, ".").to_string();

    text = text
        .trim_start_matches(|c: char| c == ',' || c.is_whitespace())
        .trim()
        .to_string();

    text
}

static NESTED_PERCENT_OF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        ^
        (?P<base>
            -?\d+(?:\.\d+)?
            (?:\s*/\s*-?\d+(?:\.\d+)?)*
        )
        \s*%
        \s*\(\+\s*
        (?P<bonus>
            -?\d+(?:\.\d+)?
            (?:\s*/\s*-?\d+(?:\.\d+)?)*
        )
        \s*%
        \s*per\s*100\s*
        (?P<stat>.+?)
        \)
        \s*
        (?P<target>of .+)
        $
        ",
    )
    .unwrap()
});

fn parse_ranked_numbers_or_single(s: &str, rank_count: usize) -> Option<Vec<f64>> {
    parse_ranked_numbers(s).or_else(|| parse_single_number(s).map(|v| vec![v; rank_count]))
}

fn try_parse_formula_exception(data: &str, rank_count: usize) -> Option<Vec<String>> {
    let data = normalize_formula_input(data);

    let caps = NESTED_PERCENT_OF_RE.captures(&data)?;

    let base = parse_ranked_numbers_or_single(caps.name("base")?.as_str(), rank_count)?;
    let bonus = parse_ranked_numbers_or_single(caps.name("bonus")?.as_str(), rank_count)?;

    let stat = cleanup_formula(apply_replacements(caps.name("stat")?.as_str()));
    let target = cleanup_formula(apply_replacements(caps.name("target")?.as_str()));

    if !target.contains("ctx.") {
        return Some(Vec::new());
    }

    let out = (0..rank_count)
        .map(|i| {
            let base_coeff = trim_f64(base[i] / 100.0);
            let bonus_coeff = trim_f64((bonus[i] / 100.0) * 0.01);

            cleanup_formula(format!(
                "({base_coeff} + {bonus_coeff} * {stat}) * {target}"
            ))
        })
        .collect::<Vec<_>>();

    Some(out)
}

static INLINE_PERCENT_PRODUCT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (\d+(?:\.\d+)?)
        %
        \s*
        (
            ctx\.[a-zA-Z_][a-zA-Z0-9_]*
            |
            -?\d+(?:\.\d+)?
        )
        ",
    )
    .unwrap()
});

fn normalize_inline_percent_products(s: &str) -> String {
    INLINE_PERCENT_PRODUCT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let lhs = caps
                .get(1)
                .and_then(|m| m.as_str().parse::<f64>().ok())
                .unwrap_or(0.0);

            let rhs = caps.get(2).map(|m| m.as_str()).unwrap_or_default();

            format!("{} * {rhs}", trim_f64(lhs / 100.0))
        })
        .to_string()
}
