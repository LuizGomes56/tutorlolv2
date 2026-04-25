use crate::{
    champions::cache,
    client::{MayFail, fetch},
    parser::parse_lua,
};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChampionRaw {
    pub id: f32,
    pub apiname: String,
    pub title: String,

    #[serde(default)]
    pub skill_i: Vec<String>,
    #[serde(default)]
    pub skill_q: Vec<String>,
    #[serde(default)]
    pub skill_w: Vec<String>,
    #[serde(default)]
    pub skill_e: Vec<String>,
    #[serde(default)]
    pub skill_r: Vec<String>,
}

pub async fn download() -> MayFail<String> {
    println!("[fn] champions::full::download");

    let text = fetch(
        cache().join("data").with_extension("html"),
        "Module:ChampionData/data",
    )
    .await?;

    let pre = parse_lua(&text)?;

    std::fs::write(cache().join("lua").with_extension("txt"), &pre)?;

    Ok(pre)
}

pub fn parse() -> MayFail<BTreeMap<String, ChampionRaw>> {
    println!("[fn] champions::full::parse");

    let src = crate::read_to_string(cache().join("lua").with_extension("txt"))?;
    let lua = Lua::new();

    let value = lua
        .load(&src)
        .eval::<Value>()
        .map_err(|e| format!("[error] Failed to load and eval lua: {e:?}"))?;

    let map = lua
        .from_value::<BTreeMap<String, ChampionRaw>>(value)
        .map_err(|e| format!("[error] Failed to deserialize lua: {e:?}"))?;

    let data = serde_json::to_string_pretty(&map)?;

    std::fs::write(cache().join("data").with_extension("json"), &data)?;

    Ok(map)
}
