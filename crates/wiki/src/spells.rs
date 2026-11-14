//! Summoner spell data from `Module:SpellData/data`. Currently only Ignite's
//! level-dependent true damage is consumed by the engine.

use crate::{
    HTTP_CLIENT, JsonWrite, MayFail,
    wikitext::{Nodes, PassiveProgression},
};
use mlua::{Lua, Table};
use serde::{Deserialize, Serialize};

pub const RAW_PATH: &str = "cache/wiki/spells/data.lua";
pub const FULL_PATH: &str = "cache/wiki/spells/full.json";
const MODULE: &str = "Module:SpellData/data?action=raw";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpellData {
    /// Total true damage, indexed by champion level minus one.
    pub ignite: Vec<i32>,
}

impl SpellData {
    pub fn from_lua(src: &str) -> MayFail<Self> {
        let parse = || -> MayFail<Self> {
            let lua = Lua::new();
            let module: Table = lua.load(src).eval().map_err(|e| e.to_string())?;
            let ignite: Table = module.get("Ignite").map_err(|e| e.to_string())?;
            let effect: String = ignite.get("effect").map_err(|e| e.to_string())?;
            let nodes = Nodes::parse(&effect);
            let templates = nodes.templates();

            // A level-only API cannot express additional stat scalings. Refuse
            // a changed shape rather than silently using only part of its damage.
            if templates.iter().any(|template| {
                template.name == "as"
                    && template
                        .positional(0)
                        .is_some_and(|text| text.plain().trim_start().starts_with("(+"))
            }) {
                return Err("Ignite has a scaling that the level-only API cannot represent".into());
            }

            let damage = templates
                .iter()
                .filter(|template| template.name == "as")
                .filter_map(|template| {
                    let text = template.positional(0)?;
                    let color = template.positional(1).map(Nodes::plain);
                    (text.plain().contains("true damage")
                        || color.as_deref() == Some("true damage"))
                    .then_some(text)
                })
                .collect::<Vec<_>>();

            let [damage] = damage[..] else {
                return Err(format!(
                    "Expected one Ignite true damage value, found {}",
                    damage.len()
                )
                .into());
            };

            // Only the level progression and its damage type belong to this
            // value. An extra multiplier or damage term must not be discarded.
            let mut progression = None;

            for node in &damage.0 {
                match node {
                    crate::wikitext::Node::Text(text)
                        if text.trim().is_empty() || text.trim() == "true damage" => {}
                    crate::wikitext::Node::Template(template)
                        if matches!(template.name.as_str(), "pp" | "pplevel")
                            && progression.is_none() =>
                    {
                        progression = Some(template);
                    }
                    crate::wikitext::Node::Template(template)
                        if template.name == "sti"
                            && template.positionals().count() == 1
                            && template
                                .positional(0)
                                .is_some_and(|text| text.plain() == "true damage") => {}
                    _ => return Err(format!("Unsupported Ignite damage value: {damage}").into()),
                }
            }

            let progression = progression.ok_or("Ignite true damage has no level progression")?;
            let progression = PassiveProgression::new(progression)?;

            if !progression.key.is_empty()
                || progression
                    .axis
                    .as_deref()
                    .is_some_and(|axis| !matches!(axis, "level" | "Level" | "champion level"))
                || progression
                    .keys
                    .iter()
                    .enumerate()
                    .any(|(i, key)| *key != (i + 1) as f64)
            {
                return Err("Ignite damage must be a progression at consecutive champion levels starting at 1".into());
            }

            let ignite = progression
                .values
                .into_iter()
                .map(|value| {
                    if !value.is_finite()
                        || value.fract() != 0.0
                        || !(0.0..=f64::from(i32::MAX)).contains(&value)
                    {
                        return Err(
                            format!("Ignite damage {value} is not a nonnegative i32").into()
                        );
                    }
                    Ok(value as i32)
                })
                .collect::<MayFail<Vec<_>>>()?;

            let data = Self { ignite };
            data.validate()?;

            Ok(data)
        };

        parse().map_err(|e| format!("[spells] Failed to parse Ignite from {MODULE}: {e}").into())
    }

    /// Also checked when the build reads the parsed cache. The final increment
    /// extends the table for levels above those published by the wiki.
    pub fn validate(&self) -> MayFail {
        if !(2..=usize::from(u8::MAX)).contains(&self.ignite.len())
            || self.ignite.iter().any(|value| *value < 0)
        {
            return Err("[spells] Ignite needs 2..=255 nonnegative damage values".into());
        }

        let last = self.ignite.len() - 1;
        let step = i64::from(self.ignite[last]) - i64::from(self.ignite[last - 1]);
        let limit = i64::from(self.ignite[last]) + step * (255 - self.ignite.len()) as i64;

        if !(0..=i64::from(i32::MAX)).contains(&limit) {
            return Err(
                "[spells] Ignite's final increment cannot extend to level 255 as a nonnegative i32"
                    .into(),
            );
        }

        Ok(())
    }
}

pub async fn download() -> MayFail {
    println!("[fn] spells::download");

    HTTP_CLIENT
        .fetch_raw(std::path::Path::new(RAW_PATH), MODULE)
        .await
}

pub fn parse() -> MayFail {
    println!("[fn] spells::parse");
    SpellData::from_lua(&crate::read_to_string(RAW_PATH)?)?.write_file(FULL_PATH)
}

pub async fn run() -> MayFail {
    download().await?;
    parse()
}
