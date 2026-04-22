use crate::{
    champions::{clean_text, full::ChampionRaw},
    client::{MayFail, SyncMayFail, fetch},
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub async fn download() -> MayFail {
    println!("[fn] champions::template::download");

    let champions = {
        let bytes = std::fs::read("cache/wiki/champions/data.json")?;
        serde_json::from_slice::<BTreeMap<String, ChampionRaw>>(&bytes)?
    };

    for (name, raw) in champions
        .into_iter()
        .filter(|(name, _)| name != "Mega Gnar")
    {
        let ChampionRaw { apiname, .. } = raw;

        fetch(
            format!("cache/wiki/champions/{apiname}/template.html"),
            format!("Template:Data_{name}"),
        )
        .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] champions::template::parse");

    let result: SyncMayFail = std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false))
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let f = || -> MayFail {
                let path = entry.path().join("template");

                let data = std::fs::read_to_string(path.with_extension("html"))
                    .map_err(|e| format!("[error] Failed to read path: {path:?}: {e:?}"))?;

                let champion_id = entry.file_name().into_string().map_err(|e| {
                    format!("[error] Failed to get file name for path: {path:?}: {e:?}")
                })?;

                println!("[parallel] Processing templates for {champion_id:?}");

                let doc = Html::parse_document(&data);

                let table_selector = Selector::parse("table")?;
                let tr_selector = Selector::parse("tr")?;
                let cell_selector = Selector::parse("th, td")?;

                let mut values = BTreeMap::<String, String>::new();
                let mut skill_cells = BTreeMap::<String, String>::new();

                for table in doc.select(&table_selector) {
                    let rows = table.select(&tr_selector).collect::<Vec<_>>();

                    if rows.is_empty() {
                        continue;
                    }

                    let header = rows[0].text().collect::<String>().to_ascii_lowercase();

                    if !(header.contains("parameter") && header.contains("value")) {
                        continue;
                    }

                    for row in rows.into_iter().skip(1) {
                        let cells = row.select(&cell_selector).collect::<Vec<_>>();

                        if cells.len() < 2 {
                            continue;
                        }

                        let key = clean_text(&cells[0].text().collect::<String>());

                        if key.is_empty() {
                            continue;
                        }

                        let value_text = clean_text(&cells[1].text().collect::<String>());

                        if value_text.is_empty() && !is_skill_key(&key) {
                            continue;
                        }

                        values.insert(key.clone(), value_text);

                        if is_skill_key(&key) {
                            skill_cells.insert(key, cells[1].inner_html());
                        }
                    }
                }

                let mut out = ChampionTemplate::default();

                out.general(&values)
                    .skills(skill_cells, &values)
                    .stats(&values)
                    .modes(&values);

                let json = serde_json::to_string_pretty(&out)?;

                std::fs::write(path.with_extension("json"), json)?;

                Ok(())
            };

            f().map_err(|e| e.to_string().into())
        });

    result.map_err(|e| e.to_string().into())
}

fn is_skill_key(key: &str) -> bool {
    matches!(
        key,
        "skill_i" | "skill_q" | "skill_w" | "skill_e" | "skill_r"
    )
}

fn get_f32(map: &BTreeMap<String, String>, key: &str, default: f32) -> f32 {
    map.get(key).and_then(|s| parse_f32(s)).unwrap_or(default)
}

fn parse_f32(s: &str) -> Option<f32> {
    let filtered = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect::<String>();

    match filtered.is_empty() {
        true => None,
        false => filtered.parse::<f32>().ok(),
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChampionTemplate {
    pub adaptive_type: String,
    pub attack_type: String,
    pub skills: SkillSet,
    pub stats: Stats,
    pub modes: ModeStats,
}

impl ChampionTemplate {
    pub fn general(&mut self, values: &BTreeMap<String, String>) -> &mut Self {
        let clean = |key: &str| {
            values
                .get(key)
                .map(String::as_str)
                .map(clean_text)
                .unwrap_or_default()
        };

        self.adaptive_type = clean("adaptivetype");
        self.attack_type = clean("rangetype");
        self
    }

    pub fn skills(
        &mut self,
        skill_cells: BTreeMap<String, String>,
        values: &BTreeMap<String, String>,
    ) -> &mut Self {
        macro_rules! parse {
            ($($field:ident)*) => {
                $(self.skills.$field = parse_skill_list(
                    skill_cells.get(stringify!($field)),
                    values.get(stringify!($field)),
                );)*
            };
        }
        parse!(skill_i skill_q skill_w skill_e skill_r);
        self
    }

    pub fn stats(&mut self, values: &BTreeMap<String, String>) -> &mut Self {
        macro_rules! parse {
            ($($field:ident),*) => {
                $(self.stats.$field = get_f32(values, stringify!($field), 0.0);)*
            };
        }
        parse!(
            hp_base, hp_lvl, hp5_base, hp5_lvl, mp_base, mp_lvl, arm_base, arm_lvl, mr_base,
            mr_lvl, dam_base, dam_lvl, as_base, as_ratio, as_lvl, crit_base
        );
        self
    }

    pub fn modes(&mut self, values: &BTreeMap<String, String>) -> &mut Self {
        macro_rules! parse {
            ($($field:ident),*) => {
                $(self.modes.$field = get_f32(values, stringify!($field), 1.0);)*
            };
        }
        parse!(
            ofa_dmg_dealt,
            ofa_dmg_taken,
            usb_dmg_dealt,
            usb_dmg_taken,
            aram_dmg_dealt,
            aram_dmg_taken,
            ar_dmg_dealt,
            ar_dmg_taken,
            nb_dmg_dealt,
            nb_dmg_taken,
            swift_dmg_dealt,
            swift_dmg_taken,
            urf_dmg_dealt,
            urf_dmg_taken
        );
        self
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SkillSet {
    pub skill_i: Vec<String>,
    pub skill_q: Vec<String>,
    pub skill_w: Vec<String>,
    pub skill_e: Vec<String>,
    pub skill_r: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub hp_base: f32,
    pub hp_lvl: f32,
    pub hp5_base: f32,
    pub hp5_lvl: f32,
    pub mp_base: f32,
    pub mp_lvl: f32,
    pub arm_base: f32,
    pub arm_lvl: f32,
    pub mr_base: f32,
    pub mr_lvl: f32,
    pub dam_base: f32,
    pub dam_lvl: f32,
    pub as_base: f32,
    pub as_ratio: f32,
    pub as_lvl: f32,
    pub crit_base: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModeStats {
    pub ofa_dmg_dealt: f32,
    pub ofa_dmg_taken: f32,
    pub usb_dmg_dealt: f32,
    pub usb_dmg_taken: f32,
    pub aram_dmg_dealt: f32,
    pub aram_dmg_taken: f32,
    pub ar_dmg_dealt: f32,
    pub ar_dmg_taken: f32,
    pub nb_dmg_dealt: f32,
    pub nb_dmg_taken: f32,
    pub swift_dmg_dealt: f32,
    pub swift_dmg_taken: f32,
    pub urf_dmg_dealt: f32,
    pub urf_dmg_taken: f32,
}

fn parse_skill_list(value_html: Option<&String>, value_text: Option<&String>) -> Vec<String> {
    if let Some(html) = value_html {
        let fragment = Html::parse_fragment(html);
        let a_selector = Selector::parse("a").unwrap();

        let from_links = fragment
            .select(&a_selector)
            .map(|a| clean_text(&a.text().collect::<String>()))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if !from_links.is_empty() {
            return from_links;
        }
    }

    if let Some(text) = value_text {
        let trimmed = text.trim();

        if trimmed.is_empty() {
            return Vec::new();
        }

        return trimmed
            .split('\n')
            .flat_map(|x| x.split(','))
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(From::from)
            .collect();
    }

    Vec::new()
}
