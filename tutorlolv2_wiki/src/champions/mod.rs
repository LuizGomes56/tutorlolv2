use crate::{
    MayFail,
    champions::{
        abilities::WikiAbility,
        template::{ChampionTemplate, ModeStats, Stats},
    },
    is_dir,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use tutorlolv2_types::{AdaptiveType, AttackType, GameMap, Key, Position};

pub mod abilities;
pub mod exceptions;
pub mod full;
pub mod template;

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/champions")
}

pub async fn run() -> MayFail {
    full::download().await?;
    full::parse()?;
    template::download().await?;
    template::parse()?;
    abilities::download().await?;
    abilities::parse()?;
    exceptions::download().await?;
    exceptions::parse()?;
    concat()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WikiChampion {
    pub name: String,
    pub champion_id: String,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub positions: Vec<Position>,
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub wiki_abilities: BTreeMap<Key, Vec<WikiAbility>>,
    /// Texts of [`exceptions::EXCEPTIONS`] by their name
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub exceptions: BTreeMap<String, String>,
    /// Stats that game modes change, only for the modes that change them
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub map_stats: Vec<WikiMapStats>,
}

/// Stats of a champion that a game mode changes, from `Module:ChampionData`
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WikiMapStats {
    pub maps: Vec<GameMap>,
    /// Multiplier of the total attack speed
    pub total_attack_speed: f32,
    /// Every stat on these maps, with the ones the mode changes replaced
    pub stats: WikiStats,
}

impl WikiMapStats {
    /// Stats that the mode of `code` changes, if any. `values` are the stats the
    /// template gives for the mode, as `hp_lvl`, which replace the ones of `stats`
    pub fn new(
        code: &str,
        total_attack_speed: f32,
        values: &BTreeMap<String, f32>,
        stats: &WikiStats,
    ) -> Option<Self> {
        if total_attack_speed == 1.0 && values.is_empty() {
            return None;
        }

        let maps = crate::map_changes::MAPS
            .iter()
            .find(|source| source.code == code)?
            .maps
            .to_vec();

        let mut stats = *stats;

        for (key, value) in values {
            let stat = match key.as_str() {
                "hp_base" => &mut stats.health.base,
                "hp_lvl" => &mut stats.health.per_level,
                "mp_base" => &mut stats.mana.base,
                "mp_lvl" => &mut stats.mana.per_level,
                "arm_base" => &mut stats.armor.base,
                "arm_lvl" => &mut stats.armor.per_level,
                "mr_base" => &mut stats.magic_resist.base,
                "mr_lvl" => &mut stats.magic_resist.per_level,
                "dam_base" => &mut stats.attack_damage.base,
                "dam_lvl" => &mut stats.attack_damage.per_level,
                "as_base" => &mut stats.attack_speed.base,
                "as_lvl" => &mut stats.attack_speed.per_level,
                "as_ratio" => &mut stats.attack_speed_ratio,
                _ => continue,
            };

            *stat = *value;
        }

        Some(Self {
            maps,
            total_attack_speed,
            stats,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Stat {
    pub base: f32,
    pub per_level: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Modifier {
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct WikiModifiers {
    pub ofa: Modifier,
    pub usb: Modifier,
    pub aram: Modifier,
    pub ar: Modifier,
    pub nb: Modifier,
    pub swift: Modifier,
    pub urf: Modifier,
}

pub fn concat() -> MayFail {
    println!("[fn] champions::concat");

    let champions = crate::read_dir(cache())?
        .filter(is_dir)
        .par_bridge()
        .map(|entry| {
            let path = entry.path();
            let template = crate::read(path.join("template").with_extension("json"))?;
            let ChampionTemplate {
                name,
                champion_id,
                adaptive_type,
                attack_type,
                positions,
                stats:
                    Stats {
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
                    },
                modes:
                    ModeStats {
                        ofa_dmg_dealt,
                        ofa_dmg_taken,
                        usb_dmg_dealt,
                        usb_dmg_taken,
                        aram_dmg_dealt,
                        aram_dmg_taken,
                        aram_total_as,
                        ar_total_as,
                        stats: mode_stats,
                        ar_dmg_dealt,
                        ar_dmg_taken,
                        nb_dmg_dealt,
                        nb_dmg_taken,
                        swift_dmg_dealt,
                        swift_dmg_taken,
                        urf_dmg_dealt,
                        urf_dmg_taken,
                    },
                ..
            } = serde_json::from_slice(&template)?;

            let mut wiki_abilities = BTreeMap::<Key, Vec<_>>::new();

            for entry in crate::read_dir(path.join("abilities"))?.filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            }) {
                let bytes = crate::read(entry.path())?;
                let ability = serde_json::from_slice::<WikiAbility>(&bytes)?;

                wiki_abilities
                    .entry(ability.skill)
                    .or_default()
                    .push(ability);
            }

            wiki_abilities
                .values_mut()
                .for_each(|values| values.dedup_by(|a, b| a.effects == b.effects));

            let mut wiki_champion = WikiChampion {
                name,
                positions: positions
                    .into_iter()
                    .filter_map(|v| v.parse().ok())
                    .collect(),
                champion_id: champion_id.clone(),
                adaptive_type: adaptive_type.parse()?,
                attack_type: attack_type.parse()?,
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
                wiki_abilities,
                exceptions: exceptions::read(&champion_id)?,
                map_stats: Vec::new(),
            };

            let empty = BTreeMap::new();

            wiki_champion.map_stats = [("aram", aram_total_as), ("ar", ar_total_as)]
                .into_iter()
                .filter_map(|(code, total_as)| {
                    let stats = mode_stats.get(code).unwrap_or(&empty);
                    WikiMapStats::new(code, total_as, stats, &wiki_champion.stats)
                })
                .collect();

            let bytes = serde_json::to_string_pretty(&wiki_champion)?;
            crate::write(path.join("data").with_extension("json"), bytes)?;

            MayFail::<(String, WikiChampion)>::Ok((champion_id, wiki_champion))
        })
        .inspect(|v| {
            let _ = v.as_ref().inspect_err(|e| eprintln!("Task error: {e:?}"));
        })
        .filter_map(Result::ok)
        .collect::<BTreeMap<String, WikiChampion>>();

    let bytes = serde_json::to_string_pretty(&champions)?;

    crate::write(cache().join("full").with_extension("json"), bytes)

    /*
    ?;

    let mut formulas = Vec::new();

    for (_, wiki) in &champions {
        for (_, abilities) in &wiki.abilities {
            for ability in abilities {
                formulas.extend_from_slice(&ability.formulas);
            }
        }
    }

    let debug_values = serde_json::json!({
        "formulas": formulas
    });

    let debug_bytes = serde_json::to_string(&debug_values)?;

    crate::write(cache().join("debug").with_extension("json"), debug_bytes)

    */
}
