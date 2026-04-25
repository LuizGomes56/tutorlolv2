use crate::{
    champions::template::{ChampionTemplate, ModeStats, Stats},
    client::MayFail,
    is_dir,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tutorlolv2_types::{AdaptiveType, AttackType, DamageType, Key};

pub mod abilities;
pub mod full;
pub mod template;

pub(self) fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/champions")
}

pub async fn run() -> MayFail {
    full::download().await?;
    full::parse()?;
    template::download().await?;
    template::parse()?;
    abilities::download().await?;
    abilities::parse()?;
    concat()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiChampion {
    pub champion_id: String,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub abilities: BTreeMap<Key, Vec<WikiAbility>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiStats {
    pub health: Stat,
    pub mana: Stat,
    pub armor: Stat,
    pub magic_resist: Stat,
    pub attack_damage: Stat,
    pub attack_speed: Stat,
    pub attack_speed_ratio: f32,
    pub crit_modifier: f32,
    pub crit_base: f32,
    pub move_speed: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stat {
    pub base: f32,
    pub per_level: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Modifier {
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiModifiers {
    pub ofa: Modifier,
    pub usb: Modifier,
    pub aram: Modifier,
    pub ar: Modifier,
    pub nb: Modifier,
    pub swift: Modifier,
    pub urf: Modifier,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiAbility {
    pub slot: Key,
    pub name: String,
    pub damage_type: DamageType,
    pub effects: Vec<WikiEffect>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiEffect {
    pub description: String,
    pub levelings: Vec<WikiLeveling>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiLeveling {
    pub attribute: String,
    pub formula_attempt: Vec<String>,
    pub data: String,
}

pub fn concat() -> MayFail {
    println!("[download] champions::concat");

    let dir = Path::new("cache/wiki/champions");

    let champions = crate::read_dir(dir)?
        .filter(is_dir)
        .par_bridge()
        .map(|entry| {
            let path = entry.path();
            let template = crate::read(path.join("template").with_extension("json"))?;
            let champion = serde_json::from_slice::<ChampionTemplate>(&template)?;

            let champion_id = entry.file_name().into_string().map_err(|e| {
                format!("[error] Failed to get file name for path: {path:?}: {e:?}")
            })?;

            println!("[concat] Processing {champion_id:?}");

            let mut abilities = BTreeMap::<Key, Vec<WikiAbility>>::new();

            for entry in crate::read_dir(path.join("abilities"))?.filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            }) {
                let bytes = crate::read(entry.path())?;
                let ability = serde_json::from_slice::<WikiAbility>(&bytes)?;

                abilities
                    .entry(ability.slot)
                    .and_modify(|v| v.push(ability))
                    .or_default();
            }

            let ModeStats {
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
                urf_dmg_taken,
            } = champion.modes;

            let Stats {
                hp_base,
                hp_lvl,
                mp_base,
                mp_lvl,
                arm_base,
                arm_lvl,
                mr_base,
                mr_lvl,
                dam_base,
                dam_lvl,
                as_base,
                as_ratio,
                as_lvl,
                crit_base,
                crit_mod,
                ms,
                ..
            } = champion.stats;

            let wiki_champion = WikiChampion {
                champion_id: champion_id.clone(),
                adaptive_type: champion.adaptive_type.parse()?,
                attack_type: champion.attack_type.parse()?,
                stats: WikiStats {
                    health: Stat {
                        base: hp_base,
                        per_level: hp_lvl,
                    },
                    mana: Stat {
                        base: mp_base,
                        per_level: mp_lvl,
                    },
                    armor: Stat {
                        base: arm_base,
                        per_level: arm_lvl,
                    },
                    magic_resist: Stat {
                        base: mr_base,
                        per_level: mr_lvl,
                    },
                    attack_damage: Stat {
                        base: dam_base,
                        per_level: dam_lvl,
                    },
                    attack_speed: Stat {
                        base: as_base,
                        per_level: as_lvl,
                    },
                    attack_speed_ratio: as_ratio,
                    crit_modifier: crit_mod,
                    crit_base,
                    move_speed: ms,
                },
                modifiers: WikiModifiers {
                    ofa: Modifier {
                        damage_dealt: ofa_dmg_dealt,
                        damage_taken: ofa_dmg_taken,
                    },
                    usb: Modifier {
                        damage_dealt: usb_dmg_dealt,
                        damage_taken: usb_dmg_taken,
                    },
                    aram: Modifier {
                        damage_dealt: aram_dmg_dealt,
                        damage_taken: aram_dmg_taken,
                    },
                    ar: Modifier {
                        damage_dealt: ar_dmg_dealt,
                        damage_taken: ar_dmg_taken,
                    },
                    nb: Modifier {
                        damage_dealt: nb_dmg_dealt,
                        damage_taken: nb_dmg_taken,
                    },
                    swift: Modifier {
                        damage_dealt: swift_dmg_dealt,
                        damage_taken: swift_dmg_taken,
                    },
                    urf: Modifier {
                        damage_dealt: urf_dmg_dealt,
                        damage_taken: urf_dmg_taken,
                    },
                },
                abilities,
            };

            let bytes = serde_json::to_string_pretty(&wiki_champion)?;
            std::fs::write(path.join("data").with_extension("json"), bytes)?;

            MayFail::<(String, WikiChampion)>::Ok((champion_id, wiki_champion))
        })
        .filter_map(Result::ok)
        .collect::<BTreeMap<String, WikiChampion>>();

    let bytes = serde_json::to_string_pretty(&champions)?;
    std::fs::write(dir.join("full").with_extension("json"), bytes)?;

    Ok(())
}

pub fn clean_text(s: &str) -> String {
    s.replace('\u{a0}', " ")
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}
