use heck::ToPascalCase;
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_wiki::items::item_parser::WikiItem;

#[derive(Clone, Debug, Default)]
pub struct ItemRecipe {
    pub builds_from: Vec<String>,
    pub builds_into: Vec<String>,
}

#[derive(Clone)]
pub(super) struct RiotRecipe {
    pub from: Vec<String>,
    pub into: Vec<String>,
}

fn normalized(name: &str) -> String {
    name.to_pascal_case().to_lowercase()
}

/// Resolve only against final enum variants. Numeric redirects retain relationships
/// from mode-specific Riot entries collapsed into an existing wiki item.
pub(super) fn resolve(
    items: &BTreeMap<String, WikiItem>,
    riot: &BTreeMap<String, RiotRecipe>,
    redirects: &BTreeMap<u32, String>,
) -> BTreeMap<String, ItemRecipe> {
    let mut ids = BTreeMap::new();
    let mut names = BTreeMap::<String, (&str, u32)>::new();
    let mut keys = BTreeMap::new();

    for (key, item) in items {
        // Matches ItemId::from_riot_id: first enum variant wins duplicate IDs.
        ids.entry(item.id).or_insert(key.as_str());
        keys.insert(normalized(key), key.as_str());

        let owner = names
            .entry(normalized(&item.name))
            .or_insert((key, item.id));

        if item.id < owner.1 {
            *owner = (key, item.id);
        }
    }

    for (id, key) in redirects {
        ids.entry(*id).or_insert(key);
    }

    let mut recipes = BTreeMap::new();

    for (key, item) in items {
        let (builds_from, upgrades) = if let Some(recipe) = riot.get(key) {
            let resolve_ids = |values: &[String]| {
                values
                    .iter()
                    .filter_map(|value| {
                        let id = value.parse::<u32>().ok()?;
                        let variant = *ids.get(&id)?;

                        (variant != key).then(|| variant.to_owned())
                    })
                    .collect::<Vec<_>>()
            };

            (resolve_ids(&recipe.from), resolve_ids(&recipe.into))
        } else {
            let components = item
                .recipe
                .iter()
                .filter_map(|name| {
                    let name = normalized(name);

                    let variant = keys
                        .get(&name)
                        .copied()
                        .or_else(|| names.get(&name).map(|(key, _)| *key))?;

                    (variant != key).then(|| variant.to_owned())
                })
                .collect();

            (components, Vec::new())
        };

        let mut seen = BTreeSet::new();

        let builds_into = upgrades
            .into_iter()
            .filter(|key| seen.insert(key.clone()))
            .collect();

        recipes.insert(
            key.clone(),
            ItemRecipe {
                builds_from,
                builds_into,
            },
        );
    }

    // Complete inverse edges whenever either side has no Riot record. This also
    // handles a Riot recipe referencing a component available only in the wiki.
    for parent in items.keys() {
        let components = recipes[parent]
            .builds_from
            .iter()
            .filter(|component| !riot.contains_key(parent) || !riot.contains_key(*component))
            .cloned()
            .collect::<BTreeSet<_>>();

        for component in components {
            let upgrades = &mut recipes.get_mut(&component).unwrap().builds_into;

            if !upgrades.contains(parent) {
                upgrades.push(parent.clone());
            }
        }
    }

    recipes
}
