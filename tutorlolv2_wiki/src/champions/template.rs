use crate::{
    champions::{cache, clean_text, full::ChampionRaw},
    client::{MayFail, fetch},
    is_dir,
    parser::get_cells,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub async fn download() -> MayFail {
    println!("[fn] champions::template::download");

    let champions = {
        let bytes = crate::read(cache().join("data").with_extension("json"))?;
        serde_json::from_slice::<BTreeMap<String, ChampionRaw>>(&bytes)?
    };

    for (name, raw) in champions
        .into_iter()
        .filter(|(name, _)| name != "Mega Gnar")
    {
        let ChampionRaw { apiname, .. } = raw;

        let path = cache().join(&apiname);
        std::fs::create_dir_all(&path)?;

        fetch(
            path.join("template").with_extension("html"),
            format!("Template:Data_{name}"),
        )
        .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] champions::template::parse");

    crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let path = entry.path().join("template");
            let data = crate::read_to_string(path.with_extension("html"))?;

            let html = Html::parse_document(&data);
            let cells = get_cells(&html)?
                .into_iter()
                .filter(|(key, _)| !key.ends_with("_raw"))
                .collect::<BTreeMap<_, _>>();

            let json = serde_json::to_string_pretty(
                &ChampionTemplate::default()
                    .general(&cells)?
                    .skills(&cells)?
                    .stats(&cells)
                    .modes(&cells),
            )?;

            std::fs::write(path.with_extension("json"), json)?;

            Ok(())
        })
}

fn parse_f32(map: &BTreeMap<String, String>, key: &str) -> Option<f32> {
    map.get(key).and_then(|s| {
        let filtered = s
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect::<String>();

        match filtered.is_empty() {
            true => None,
            false => filtered.parse::<f32>().ok(),
        }
    })
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChampionTemplate {
    pub name: String,
    pub adaptive_type: String,
    pub attack_type: String,
    pub skills: SkillSet,
    pub stats: Stats,
    pub modes: ModeStats,
}

impl ChampionTemplate {
    pub fn general(&mut self, cells: &BTreeMap<String, String>) -> MayFail<&mut Self> {
        let clean = |key: &str| {
            cells
                .get(key)
                .map(String::as_str)
                .map(clean_text)
                .ok_or(format!("Failed to extract key: {key}"))
        };

        self.name = clean("1")?;
        self.adaptive_type = clean("adaptivetype")?;
        self.attack_type = clean("rangetype")?;
        Ok(self)
    }

    pub fn skills(&mut self, cells: &BTreeMap<String, String>) -> MayFail<&mut Self> {
        macro_rules! parse {
            ($($field:ident)*) => {
                $(
                    self.skills.$field = cells.get(stringify!($field))
                        .ok_or(concat!("Failed to get skill: ", stringify!($field)))?
                        .split(',')
                        .map(String::from)
                        .collect::<Vec<_>>();
                )*
            };
        }
        parse!(skill_i skill_q skill_w skill_e skill_r);
        Ok(self)
    }

    pub fn stats(&mut self, cells: &BTreeMap<String, String>) -> &mut Self {
        macro_rules! parse {
            ($($field:ident),*) => {
                $(self.stats.$field = parse_f32(cells, stringify!($field)).unwrap_or(0.0);)*
            };
        }
        parse!(
            hp_base, hp_lvl, hp5_base, hp5_lvl, mp_base, mp_lvl, arm_base, arm_lvl, mr_base,
            mr_lvl, dam_base, dam_lvl, as_base, as_ratio, as_lvl, crit_base, ms
        );
        self.stats.crit_mod = parse_f32(cells, "crit_mod").unwrap_or(1.0);
        self
    }

    pub fn modes(&mut self, cells: &BTreeMap<String, String>) -> &mut Self {
        macro_rules! parse {
            ($($field:ident),*) => {
                $(self.modes.$field = parse_f32(cells, stringify!($field)).unwrap_or(1.0);)*
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
    pub crit_mod: f32,
    pub ms: f32,
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
