use crate::{
    client::{MayFail, fetch},
    parser::parse_lua,
};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

mod item_parser;

fn cache() -> PathBuf {
    PathBuf::from("cache/wiki/items")
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ItemEffectRaw {
    pub name: Option<String>,
    pub unique: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ItemEffectsRaw {
    #[serde(default)]
    pub pass: Option<ItemEffectRaw>,
    #[serde(default)]
    pub act: Option<ItemEffectRaw>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemStatValueRaw {
    Num(f64),
    Text(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemBuyRaw {
    Num(f64),
    Text(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemModeValueRaw {
    Bool(bool),
    Text(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ModesFieldRaw {
    Map(BTreeMap<String, ItemModeValueRaw>),
    Redirect(String),
}

impl Default for ModesFieldRaw {
    fn default() -> Self {
        Self::Map(BTreeMap::new())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemEffectValueRaw {
    Table(ItemEffectRaw),
    Text(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ItemRawLua {
    pub id: f64,
    pub tier: Option<f64>,

    #[serde(default)]
    pub modes: ModesFieldRaw,

    #[serde(default)]
    pub stats: BTreeMap<String, ItemStatValueRaw>,

    #[serde(default)]
    pub effects: BTreeMap<String, ItemEffectValueRaw>,

    #[serde(default)]
    pub recipe: Vec<String>,

    pub buy: Option<ItemBuyRaw>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ItemRaw {
    pub id: f64,
    pub tier: Option<f64>,

    #[serde(default)]
    pub modes: BTreeMap<String, bool>,

    #[serde(default)]
    pub stats: BTreeMap<String, f64>,

    #[serde(default)]
    pub effects: ItemEffectsRaw,

    #[serde(default)]
    pub recipe: Vec<String>,

    pub buy: Option<f64>,
}

pub async fn download() -> MayFail<String> {
    println!("[fn] items::full::download");

    let text = fetch(
        cache().join("data").with_extension("html"),
        "Module:ItemData/data",
    )
    .await?;
    let pre = parse_lua(&text)?;

    crate::write(cache().join("lua").with_extension("txt"), &pre)?;

    Ok(pre)
}

pub fn parse() -> MayFail<BTreeMap<String, ItemRaw>> {
    println!("[fn] items::full::parse");

    let src = crate::read_to_string(cache().join("lua").with_extension("txt"))?;
    let lua = Lua::new();

    let value = lua
        .load(&src)
        .eval::<Value>()
        .map_err(|e| format!("[error] Failed to load and eval item lua: {e:?}"))?;

    let raw_map = lua
        .from_value::<BTreeMap<String, ItemRawLua>>(value)
        .map_err(|e| format!("[error] Failed to deserialize item lua: {e:?}"))?;

    let map = finalize_items(&raw_map);

    crate::write(
        cache().join("data").with_extension("json"),
        serde_json::to_string_pretty(&map)?,
    )?;

    Ok(map)
}

fn finalize_items(raw_map: &BTreeMap<String, ItemRawLua>) -> BTreeMap<String, ItemRaw> {
    raw_map
        .iter()
        .map(|(item_name, raw)| {
            let modes = resolve_modes(item_name, raw_map, &mut BTreeSet::new());
            let stats = filter_numeric_stats(&raw.stats);
            let pass = resolve_effect(item_name, "pass", raw_map, &mut BTreeSet::new());
            let act = resolve_effect(item_name, "act", raw_map, &mut BTreeSet::new());
            let buy = resolve_buy(item_name, raw_map, &mut BTreeSet::new());

            let item = ItemRaw {
                id: raw.id,
                tier: raw.tier,
                modes,
                stats,
                effects: ItemEffectsRaw { pass, act },
                recipe: raw.recipe.clone(),
                buy,
            };

            (item_name.clone(), item)
        })
        .collect()
}

fn filter_numeric_stats(stats: &BTreeMap<String, ItemStatValueRaw>) -> BTreeMap<String, f64> {
    stats
        .iter()
        .filter_map(|(key, value)| match value {
            ItemStatValueRaw::Num(v) => Some((key.clone(), *v)),
            ItemStatValueRaw::Text(_) => None,
        })
        .collect()
}

fn parse_redirect(s: &str) -> Option<&str> {
    let s = s.trim();
    let rest = s.strip_prefix("=>")?.trim();

    if rest.starts_with('{') && rest.ends_with('}') && rest.len() >= 2 {
        Some(rest[1..rest.len() - 1].trim())
    } else {
        Some(rest)
    }
}

fn resolve_buy(
    item_name: &str,
    raw_map: &BTreeMap<String, ItemRawLua>,
    visiting: &mut BTreeSet<String>,
) -> Option<f64> {
    if !visiting.insert(item_name.to_string()) {
        return None;
    }

    let item = raw_map.get(item_name)?;

    let result = match item.buy.as_ref()? {
        ItemBuyRaw::Num(v) => Some(*v),
        ItemBuyRaw::Text(s) => {
            let target = parse_redirect(s)?;
            resolve_buy(target, raw_map, visiting)
        }
    };

    visiting.remove(item_name);
    result
}

fn resolve_effect(
    item_name: &str,
    effect_key: &str,
    raw_map: &BTreeMap<String, ItemRawLua>,
    visiting: &mut BTreeSet<String>,
) -> Option<ItemEffectRaw> {
    let visit_key = format!("{item_name}::{effect_key}");
    if !visiting.insert(visit_key.clone()) {
        return None;
    }

    let item = raw_map.get(item_name)?;
    let effect = item.effects.get(effect_key)?;

    let result = match effect {
        ItemEffectValueRaw::Table(table) => Some(table.clone()),
        ItemEffectValueRaw::Text(s) => {
            let target = parse_redirect(s)?;
            resolve_effect(target, effect_key, raw_map, visiting)
        }
    };

    visiting.remove(&visit_key);
    result
}

fn resolve_modes(
    item_name: &str,
    raw_map: &BTreeMap<String, ItemRawLua>,
    visiting: &mut BTreeSet<String>,
) -> BTreeMap<String, bool> {
    if !visiting.insert(item_name.to_string()) {
        return BTreeMap::new();
    }

    let Some(item) = raw_map.get(item_name) else {
        visiting.remove(item_name);
        return BTreeMap::new();
    };

    let result = match &item.modes {
        ModesFieldRaw::Map(map) => map
            .iter()
            .filter_map(|(mode_key, mode_value)| match mode_value {
                ItemModeValueRaw::Bool(v) => Some((mode_key.clone(), *v)),
                ItemModeValueRaw::Text(s) => {
                    let target = parse_redirect(s)?;
                    resolve_modes(target, raw_map, visiting)
                        .get(mode_key)
                        .copied()
                        .map(|v| (mode_key.clone(), v))
                }
            })
            .collect(),

        ModesFieldRaw::Redirect(s) => {
            let target = parse_redirect(s).and_then(|name| Some(name.to_string()));

            match target {
                Some(target) => resolve_modes(&target, raw_map, visiting),
                None => BTreeMap::new(),
            }
        }
    };

    visiting.remove(item_name);
    result
}

pub async fn run() -> MayFail {
    download().await?;
    parse().map(|_| ())?;
    item_parser::parse_items()
}
