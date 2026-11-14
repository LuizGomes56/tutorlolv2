//! Health, armor and magic resistance of monsters and minions, read from the
//! `{{Infobox unit}}` of their pages. Camp pages hold one infobox per monster.

use crate::{
    HTTP_CLIENT, JsonRead, JsonWrite, MayFail,
    wikitext::{Node, Nodes, Template, expr::Expr},
};
use lazy_regex::regex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

pub const FULL_PATH: &str = "cache/wiki/monsters/full.json";

/// Pages holding the infoboxes of every monster. Camps redirect to their camp page
const PAGES: &[&str] = &[
    "Atakhan",
    "Baron_Nashor",
    "Blue_Sentinel",
    "Caster_minion",
    "Chemtech_Drake",
    "Cloud_Drake",
    "Elder_Dragon",
    "Gromp",
    "Hextech_Drake",
    "Infernal_Drake",
    "Krug_camp",
    "Melee_minion",
    "Mountain_Drake",
    "Murk_Wolf_camp",
    "Ocean_Drake",
    "Raptor_camp",
    "Red_Brambleback",
    "Rift_Herald",
    "Rift_Scuttler_camp",
    "Siege_minion",
    "Super_minion",
    "Voidgrub_camp",
];

/// Highest level a monster scales to
pub const MAX_LEVEL: u8 = 18;

/// How a stat of a monster grows during the game
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MonsterStat {
    Flat(f64),
    /// Value at each level from `start`. Levels outside of the table are clamped to it
    Levels {
        start: u8,
        values: Vec<f64>,
    },
    /// `base + per_upgrade * upgrades`, at most `cap`. Minions gain upgrades over time
    Upgrades {
        base: f64,
        per_upgrade: f64,
        cap: f64,
    },
    /// 0 up to `after` upgrades, then `coefficient * (u - after - 1) / 2 * (u - after)`,
    /// at most `cap`
    TriangularUpgrades {
        coefficient: f64,
        after: u8,
        cap: f64,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MonsterData {
    pub health: MonsterStat,
    pub armor: MonsterStat,
    pub magic_resist: MonsterStat,
}

/// Every monster by its name on the wiki, such as `Baron Nashor`
pub type Monsters = BTreeMap<String, MonsterData>;

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/monsters")
}

pub async fn download() -> MayFail {
    println!("[fn] monsters::download");

    for page in PAGES {
        HTTP_CLIENT
            .fetch_raw(
                &cache().join(page).with_extension("txt"),
                &format!("{page}?action=raw"),
            )
            .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] monsters::parse");

    let mut monsters = Monsters::new();

    for page in PAGES {
        let src = crate::read_to_string(cache().join(page).with_extension("txt"))?;

        for (name, data) in
            MonsterData::from_page(&src).map_err(|e| format!("[monsters] {page}: {e}"))?
        {
            monsters.insert(name, data);
        }
    }

    monsters.write_file(FULL_PATH)
}

pub fn read() -> MayFail<Monsters> {
    Monsters::from_file(FULL_PATH)
}

pub async fn run() -> MayFail {
    download().await?;
    parse()
}

impl MonsterData {
    /// Every `{{Infobox unit}}` of a page, by the name of the unit
    pub fn from_page(src: &str) -> MayFail<Vec<(String, Self)>> {
        Nodes::parse(src)
            .templates()
            .into_iter()
            .filter(|template| template.name.trim() == "Infobox unit")
            .map(|infobox| {
                let field = |key: &str| {
                    infobox
                        .named(key)
                        .ok_or_else(|| format!("Infobox has no `{key}`"))
                };

                let name = field("name")?.to_string().trim().to_string();
                let stat = |key: &str| {
                    MonsterStat::new(field(key)?).map_err(|e| format!("{name} `{key}`: {e}"))
                };

                Ok((
                    name.clone(),
                    Self {
                        health: stat("hp")?,
                        armor: stat("armor")?,
                        magic_resist: stat("magicresist")?,
                    },
                ))
            })
            .collect()
    }
}

impl MonsterStat {
    pub fn new(nodes: &Nodes) -> Result<Self, String> {
        // Rift Scuttler writes the health of its first spawn, then of its respawns
        // as `{{pp|..}} (first spawned) / {{pp|..}} (normal form)`. Respawns are used
        let template = nodes.0.iter().rev().find_map(|node| match node {
            Node::Template(template) => Some(template),
            Node::Text(_) => None,
        });

        let Some(template) = template else {
            let text = nodes.to_string();

            return Expr::new(text.trim())
                .eval()
                .map(Self::Flat)
                .ok_or_else(|| format!("Unknown value `{text}`"));
        };

        match template.name.trim() {
            "#expr" => nodes.number().map(Self::Flat),
            "gs" => Self::growth(template),
            "Jungle monster stat" => Self::jungle_monster(template),
            "pp" if template
                .named("type")
                .is_some_and(|t| t.to_string().trim() == "upgrades") =>
            {
                Self::upgrades(template)
            }
            "pp" => Self::levels(template),
            name => Err(format!("Unknown template `{name}`")),
        }
    }

    /// `{{gs|base|growth|start|end}}`, the value at level `start` growing linearly
    /// until level `end`
    fn growth(template: &Template) -> Result<Self, String> {
        let arg = |index| {
            template
                .positional(index)
                .ok_or_else(|| format!("Missing argument {index} of `{template}`"))
                .and_then(Nodes::number)
        };

        let (base, growth) = (arg(0)?, arg(1)?);
        let (start, end) = (arg(2)? as u8, arg(3)? as u8);

        Ok(Self::Levels {
            start,
            values: (start..=end)
                .map(|level| base + growth * f64::from(level - start))
                .collect(),
        })
    }

    /// Port of `Template:Jungle monster stat`, which writes the health of camps as
    /// `{1}` until level 2, `+10%` of it per level from level 3 and `+5%` from level 12
    fn jungle_monster(template: &Template) -> Result<Self, String> {
        let stat = template.positional(1).map(|s| s.to_string());

        if stat.as_deref().map(str::trim) != Some("hp") {
            return Err(format!("Only the health of `{template}` is known"));
        }

        let base = template
            .positional(0)
            .ok_or_else(|| format!("Missing base of `{template}`"))
            .and_then(Nodes::number)?;

        Ok(Self::Levels {
            start: 1,
            values: (1..=MAX_LEVEL)
                .map(|level| {
                    base * match level {
                        ..=2 => 1.0,
                        3..=11 => 1.0 + 0.1 * f64::from(level - 1),
                        _ => 2.0 + 0.05 * f64::from(level - 11),
                    }
                })
                .collect(),
        })
    }

    /// `{{pp|values|levels}}`. Values are `expr`, `A to B for N` or `A to B by S`,
    /// separated by `;`. Levels are written the same way, or are `1, 2, ..` when
    /// absent. Levels without a value keep the value of the level before them
    fn levels(template: &Template) -> Result<Self, String> {
        let values = template
            .positional(0)
            .ok_or_else(|| format!("Missing values of `{template}`"))
            .and_then(|nodes| sequence(&nodes.to_string()))?;

        let levels = match template.positional(1) {
            Some(levels) => sequence(&levels.to_string())?
                .into_iter()
                .map(|level| level as u8)
                .collect(),
            None => (1..=values.len() as u8).collect::<Vec<_>>(),
        };

        if levels.len() != values.len() || !levels.is_sorted() {
            return Err(format!("Levels do not match the values of `{template}`"));
        }

        let start = levels[0];
        let end = levels[levels.len() - 1];
        let mut table = Vec::new();
        let mut index = 0;

        for level in start..=end {
            while levels.get(index + 1).is_some_and(|&next| next <= level) {
                index += 1;
            }

            table.push(values[index]);
        }

        Ok(Self::Levels {
            start,
            values: table,
        })
    }

    /// Minion stats per upgrade. The values of these templates are only examples,
    /// so the `formula=` describing them is read instead
    fn upgrades(template: &Template) -> Result<Self, String> {
        let formula = template
            .named("formula")
            .map(|nodes| nodes.to_string())
            .ok_or_else(|| format!("Missing formula of `{template}`"))?;

        let float = |text: &str| text.parse::<f64>().map_err(|e| e.to_string());

        if let Some(c) =
            regex!(r"^\s*([\d.]+) \+ ([\d.]+) per upgrade, capped at \(([\d.]+) \+ ([\d.]+)\)")
                .captures(&formula)
        {
            return Ok(Self::Upgrades {
                base: float(&c[1])?,
                per_upgrade: float(&c[2])?,
                cap: float(&c[3])? + float(&c[4])?,
            });
        }

        if let Some(c) = regex!(
            r"^\s*0 for upgrades 1.(\d+); then \+ \(([\d.]+)\*\(upgrade-(\d+)\)/2\*\(upgrade-(\d+)\)\), capped at ([\d.]+)"
        )
        .captures(&formula)
        {
            let after = c[1].parse::<u8>().map_err(|e| e.to_string())?;

            if c[3].parse::<u8>() != Ok(after + 1) || c[4].parse::<u8>() != Ok(after) {
                return Err(format!("Unknown formula `{formula}`"));
            }

            return Ok(Self::TriangularUpgrades {
                coefficient: float(&c[2])?,
                after,
                cap: float(&c[5])?,
            });
        }

        Err(format!("Unknown formula `{formula}`"))
    }
}

impl Nodes {
    /// Number written as arithmetic or as `{{#expr:..}}`
    fn number(&self) -> Result<f64, String> {
        let text = match self.0.as_slice() {
            [Node::Template(template)] if template.name.trim() == "#expr" => template
                .positional(0)
                .map(|nodes| nodes.to_string())
                .unwrap_or_default(),
            _ => self.to_string(),
        };

        Expr::new(text.trim())
            .eval()
            .ok_or_else(|| format!("`{text}` is not a number"))
    }
}

/// Values of `expr;A to B for N;A to B by S;..`
fn sequence(src: &str) -> Result<Vec<f64>, String> {
    let eval = |text: &str| {
        Expr::new(text.trim())
            .eval()
            .ok_or_else(|| format!("`{text}` is not a number"))
    };

    let mut out = Vec::new();

    for part in src.split(';') {
        let Some((start, rest)) = part.split_once(" to ") else {
            out.push(eval(part)?);
            continue;
        };

        let start = eval(start)?;

        if let Some((end, count)) = rest.split_once(" for ") {
            let (end, count) = (eval(end)?, eval(count)? as usize);
            let step = (end - start) / (count as f64 - 1.0);

            out.extend((0..count).map(|i| start + step * i as f64));
        } else if let Some((end, step)) = rest.split_once(" by ") {
            let (end, step) = (eval(end)?, eval(step)?);
            let count = ((end - start) / step + 1e-6).floor() as usize + 1;

            out.extend((0..count).map(|i| start + step * i as f64));
        } else {
            let end = eval(rest)?;

            out.extend((start as i64..=end as i64).map(|i| i as f64));
        }
    }

    Ok(out)
}
