//! Values that ability pages read from other pages: `{{ccd|Champion|stat}}`
//! (`Template:Current champion data`) and `{{#invoke:ItemData|get|item=…|datatype=…}}`,
//! resolved from `Module:ChampionData/data` and `Module:ItemData/data`, which are
//! already downloaded as `cache/wiki/champions/data.lua` and `cache/wiki/items/data.lua`

use {
    super::{Arg, Node, Nodes, Template, Value},
    crate::MayFail,
    mlua::{Lua, LuaSerdeExt},
    std::{collections::BTreeMap, path::Path},
};

/// Numeric fields of every entry, by name
type Entries = BTreeMap<String, BTreeMap<String, f64>>;

pub struct Lookup {
    champions: Entries,
    items: Entries,
}

impl Lookup {
    /// `crit_base` of the champions that do not set it, as in `Module:ChampionData/getter`:
    /// critical strikes deal 2 times the damage. Champions with lower critical
    /// damage set `crit_mod` instead, such as Jhin, Yasuo and Yone
    const DEFAULT_CRIT_BASE: f64 = 200.0;

    /// Reads the data modules, both written as `return { … }` Lua tables
    pub fn new(champions_lua: &str, items_lua: &str) -> MayFail<Self> {
        Ok(Self {
            champions: Self::entries(champions_lua)?,
            items: Self::entries(items_lua)?,
        })
    }

    /// Reads the data modules from the cache under `root`
    pub fn from_cache_at(root: &Path) -> MayFail<Self> {
        let read = |path: &str| crate::read_to_string(root.join(path));
        Self::new(
            &read("cache/wiki/champions/data.lua")?,
            &read("cache/wiki/items/data.lua")?,
        )
    }

    /// Reads the data modules from the cache, relative to the repository root
    pub fn from_cache() -> MayFail<Self> {
        Self::from_cache_at(Path::new(""))
    }

    /// Numeric fields of each entry, merging its top level with its `stats` table,
    /// whose values take precedence
    fn entries(src: &str) -> MayFail<Entries> {
        let lua = Lua::new();
        let value = lua
            .load(src)
            .eval::<mlua::Value>()
            .map_err(|e| format!("[lookup] Failed to evaluate data module: {e:?}"))?;

        let raw = lua
            .from_value::<BTreeMap<String, serde_json::Value>>(value)
            .map_err(|e| format!("[lookup] Failed to read data module: {e:?}"))?;

        let numbers = |value: Option<&serde_json::Value>| {
            value
                .and_then(serde_json::Value::as_object)
                .into_iter()
                .flatten()
                .filter_map(|(key, value)| Some((key.clone(), value.as_f64()?)))
                .collect::<Vec<_>>()
        };

        Ok(raw
            .into_iter()
            .map(|(name, entry)| {
                let mut fields = numbers(Some(&entry))
                    .into_iter()
                    .collect::<BTreeMap<_, _>>();
                fields.extend(numbers(entry.get("stats")));
                (name, fields)
            })
            .collect())
    }

    /// `stat` of the champion `name`, as `{{ccd|name|stat}}` reads it
    pub fn champion(&self, name: &str, stat: &str) -> Option<f64> {
        let value = self.champions.get(name)?.get(stat).copied();

        match stat {
            "crit_base" => value.or(Some(Self::DEFAULT_CRIT_BASE)),
            _ => value,
        }
    }

    /// `stat` of the item `name`, as `{{#invoke:ItemData|get|item=name|datatype=stat}}` reads it
    pub fn item(&self, name: &str, stat: &str) -> Option<f64> {
        self.items.get(name)?.get(stat).copied()
    }

    /// Replaces the `{{ccd}}` and `{{#invoke:ItemData|get|…}}` templates of `nodes`,
    /// at any depth, by their value. Templates whose value is not found, and other
    /// `#invoke` calls, are kept as they are
    pub fn expand(&self, nodes: &Nodes) -> Nodes {
        Nodes(
            nodes
                .0
                .iter()
                .map(|node| match node {
                    Node::Text(_) => node.clone(),
                    Node::Template(template) => match self.value(template) {
                        Some(value) => Node::Text(Value::Number(value).to_string()),
                        None => Node::Template(Template {
                            name: template.name.clone(),
                            args: template
                                .args
                                .iter()
                                .map(|arg| Arg {
                                    name: arg.name.clone(),
                                    value: self.expand(&arg.value),
                                })
                                .collect(),
                        }),
                    },
                })
                .collect(),
        )
    }

    /// Value that `template` stands for, if it is a lookup that is found
    fn value(&self, template: &Template) -> Option<f64> {
        let text = |nodes: &Nodes| nodes.plain().trim().to_string();
        let positional = |index| template.positional(index).map(text);

        match template.name.as_str() {
            "ccd" => self.champion(&positional(0)?, &positional(1)?),
            "#invoke" if positional(0)? == "ItemData" && positional(1)? == "get" => self.item(
                &text(template.named("item")?),
                &text(template.named("datatype")?),
            ),
            _ => None,
        }
    }
}
