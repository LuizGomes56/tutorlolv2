use crate::{
    champions::{
        abilities::Ability,
        template::{ChampionTemplate, ModeStats, Stats},
    },
    client::MayFail,
    is_dir,
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use tutorlolv2_types::{AdaptiveType, AttackType, Key};

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
    pub name: String,
    pub champion_id: String,
    pub adaptive_type: AdaptiveType,
    pub attack_type: AttackType,
    pub stats: WikiStats,
    pub modifiers: WikiModifiers,
    pub abilities: BTreeMap<Key, Vec<Ability>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stat {
    pub base: f32,
    pub per_level: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Modifier {
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    println!("[download] champions::concat");

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

            let mut abilities = BTreeMap::<Key, Vec<Ability>>::new();

            for entry in crate::read_dir(path.join("abilities"))?.filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            }) {
                let bytes = crate::read(entry.path())?;
                let ability = serde_json::from_slice::<Ability>(&bytes)?;

                abilities.entry(ability.skill).or_default().push(ability);
            }

            let wiki_champion = WikiChampion {
                name,
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
                abilities,
            };

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

    crate::write(cache().join("full").with_extension("json"), bytes)?;

    let mut use_values = Vec::new();
    let mut use_formula = Vec::new();
    let mut base = Vec::new();
    let mut scalings = Vec::new();

    for (_, wiki) in &champions {
        for (_, abilities) in &wiki.abilities {
            for ability in abilities {
                for (_, effect) in &ability.effects {
                    use_values.extend_from_slice(effect.use_values.as_ref().unwrap_or(&vec![]));
                    if let Some(ref formula) = effect.use_formula {
                        use_formula.push(formula);
                    }
                    base.extend_from_slice(effect.base.as_ref().unwrap_or(&vec![]));
                    scalings.extend_from_slice(&effect.scalings);
                }
            }
        }
    }

    let debug_values = serde_json::json!({
        "use_values": use_values,
        "use_formula": use_formula,
        "base": base,
        "scalings": scalings
    });

    let debug_bytes = serde_json::to_string(&debug_values)?;

    crate::write(cache().join("debug").with_extension("json"), debug_bytes)?;

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
