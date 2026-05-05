use crate::{
    client::{MayFail, fetch},
    parser::parse_lua,
};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use std::{collections::BTreeMap, path::PathBuf};

pub mod item_parser;

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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ItemRaw {
    pub id: u32,
    pub tier: Option<u8>,
    #[serde(default)]
    pub modes: BTreeMap<String, bool>,
    #[serde(default)]
    pub stats: BTreeMap<String, f64>,
    #[serde(default)]
    pub effects: ItemEffectsRaw,
    #[serde(default)]
    pub recipe: Vec<String>,
    pub buy: Option<u16>,
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

    let mut raw_map = lua
        .from_value::<JsonValue>(value)
        .map_err(|e| format!("[error] Failed to deserialize item lua: {e:?}"))?;

    fn resolve_redirects_all(value: &mut JsonValue) {
        for _ in 0..32 {
            let snapshot = value.clone();
            let changed = resolve_redirects(value, &snapshot, &mut Vec::new());

            if !changed {
                return;
            }
        }

        panic!("[items] too many redirect passes; possible circular redirect");
    }

    resolve_redirects_all(&mut raw_map);

    if let Some(json_map) = raw_map.as_object_mut() {
        for (_, value) in json_map {
            if let Some(object) = value.as_object_mut()
                && let Some(stats_entry) = object.get_mut("stats")
                && let Some(stats) = stats_entry.as_object_mut()
                && let Some(spec_value) = stats.get_mut("spec")
                && let Some(spec) = spec_value.as_str()
            {
                if let Some((_, part)) = spec.split_once('|')
                    && let Ok(adaptive) = part.trim_end_matches("}}").parse::<f64>()
                    && spec.contains("adaptive")
                {
                    *spec_value = json!(adaptive);
                    continue;
                }

                stats.remove("spec");
            }
        }
    }

    let map = serde_json::from_value(raw_map)?;

    crate::write(
        cache().join("data").with_extension("json"),
        serde_json::to_string_pretty(&map)?,
    )?;

    Ok(map)
}

fn resolve_redirects(
    current: &mut JsonValue,
    snapshot: &JsonValue,
    path: &mut Vec<String>,
) -> bool {
    let mut changed = false;

    match current {
        JsonValue::Object(map) => {
            for (key, child) in map {
                path.push(key.clone());

                if resolve_redirects(child, snapshot, path) {
                    changed = true;
                }

                path.pop();
            }
        }
        JsonValue::Array(arr) => {
            for (i, child) in arr.iter_mut().enumerate() {
                path.push(i.to_string());

                if resolve_redirects(child, snapshot, path) {
                    changed = true;
                }

                path.pop();
            }
        }
        JsonValue::String(s) => {
            if let Some(target) = s.strip_prefix("=>")
                && path.len() >= 2
                && matches!(
                    path[1].as_str(),
                    "id" | "tier" | "modes" | "stats" | "effects" | "recipe" | "buy"
                )
            {
                let mut source_path = path.clone();
                source_path[0] = target.trim().to_string();

                let replacement = get_path(snapshot, &source_path)
                    .unwrap_or_else(|| {
                        panic!("[items] redirect target not found: {path:?} -> {source_path:?}")
                    })
                    .clone();

                *current = replacement;
                changed = true;
            }
        }
        _ => {}
    }

    changed
}

fn get_path<'a>(value: &'a JsonValue, path: &[String]) -> Option<&'a JsonValue> {
    let mut current = value;

    for part in path {
        match current {
            JsonValue::Object(map) => {
                current = map.get(part)?;
            }
            JsonValue::Array(arr) => {
                let index = part.parse::<usize>().ok()?;
                current = arr.get(index)?;
            }
            _ => return None,
        }
    }

    Some(current)
}

pub async fn run() -> MayFail {
    download().await?;
    parse().map(|_| ())?;
    item_parser::parse_items()
}
