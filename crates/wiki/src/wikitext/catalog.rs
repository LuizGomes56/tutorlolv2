//! Raw catalog ingestion for the experimental item/rune generators.
use super::{Nodes, PageContext, SourceDocument, SourceVariant, Variables};
use crate::MayFail;
use mlua::{Lua, LuaSerdeExt};
use serde_json::Value;
use std::collections::BTreeMap;

impl SourceDocument {
    /// Item effects share parser variables, in source slot order. Names come
    /// from the module, rather than the ordinal of a captured scaling value.
    pub fn item_effects(
        module: &str,
        item: &str,
        context: &PageContext,
        variant: SourceVariant,
    ) -> MayFail<BTreeMap<String, Self>> {
        let lua = Lua::new();
        let value = lua.load(module).eval().map_err(|e| e.to_string())?;
        let data: Value = lua.from_value(value).map_err(|e| e.to_string())?;
        let effects = data
            .get(item)
            .and_then(|v| v.get("effects"))
            .ok_or_else(|| format!("Missing raw effects for {item}"))?;
        let mut variables = Variables::default();
        let mut documents = BTreeMap::new();
        for slot in ["pass", "pass2", "pass3", "pass4", "act", "consume"] {
            let Some(mut effect) = effects.get(slot) else {
                continue;
            };
            let mut visited = std::collections::BTreeSet::new();
            while let Some(target) = effect.as_str().and_then(|value| value.strip_prefix("=>")) {
                if !visited.insert(target.to_string()) {
                    return Err(
                        format!("Cyclic raw effect redirect for {item}/{slot}: {target}").into(),
                    );
                }
                effect = data
                    .get(target)
                    .and_then(|v| v.get("effects"))
                    .and_then(|v| v.get(slot))
                    .ok_or_else(|| {
                        format!("Missing raw effect redirect {item}/{slot} => {target}")
                    })?;
            }
            let Some(source) = effect
                .as_str()
                .or_else(|| effect.get("description").and_then(Value::as_str))
            else {
                continue;
            };
            let name = effect.get("name").and_then(Value::as_str).unwrap_or(slot);
            let nodes = Nodes::parse(source).for_variant(variant)?;
            let nodes = variables.expand(&context.lookup.expand(&nodes));
            let mut document = Self::from_nodes(name, &nodes, context, variant)?;
            document.source = source.into();
            if documents.insert(name.into(), document).is_some() {
                return Err(format!("Duplicate raw effect name {name} for {item}").into());
            }
        }
        Ok(documents)
    }

    /// Rune data templates preserve each description boundary. The outer
    /// template name is parameterized on the wiki; named arguments are stable.
    pub fn rune_descriptions(
        source: &str,
        context: &PageContext,
        variant: SourceVariant,
    ) -> MayFail<BTreeMap<String, Self>> {
        let nodes = Nodes::parse(source);
        let template = nodes
            .templates()
            .into_iter()
            .find(|template| template.named("description").is_some())
            .ok_or("Missing Rune data description")?;
        let mut documents = BTreeMap::new();
        for arg in &template.args {
            if let Some(name) = &arg.name
                && name.starts_with("description")
            {
                documents.insert(
                    name.clone(),
                    Self::parse(name, &arg.value.to_string(), context, variant)?,
                );
            }
        }
        Ok(documents)
    }
}
