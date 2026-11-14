use crate::{
    Build, BuildContext, BuildOutput, CLIENT_CFG, DOCS_CFG, MayFail,
    generators::parser::{items::Item, map_changes},
    scripts::utils::{BakeExt, ItemOrRuneExt},
};
use heck::ToShoutySnakeCase;
use std::fmt::Write;
use types::{GameMap, MapMode, StatName, Tag};
use wiki::{
    items::item_parser::WikiItem,
    map_changes::{
        ChangeTarget,
        change::{ChangeKind, Values},
    },
};

struct ItemExt {
    pub tier: u8,
    pub price: u16,
    pub stats: Vec<(StatName, u16)>,
    pub maps: Vec<GameMap>,
}

impl Build for Item {
    fn build(&mut self, context: &BuildContext) -> MayFail<BuildOutput> {
        let ItemExt {
            tier,
            price,
            stats,
            maps,
        } = self.finish();

        let Self {
            data:
                WikiItem {
                    id,
                    name,
                    item_id,
                    purchasable,
                    custom,
                    ..
                },
            ..
        } = &self;

        let mut rust = String::new();
        let plan = self.prepare_formulas();

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                tier: {tier},
                price: {price},
                stats: {full_stats},
                map_stats: &[{map_stats}],
                maps: {maps},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                purchasable: {purchasable},
                riot_id: {id},
                custom: {custom},

                {CLIENT_CFG}
                identifiers: {identifiers},

                {DOCS_CFG}
                builds_from: {builds_from},

                {DOCS_CFG}
                builds_into: {builds_into},
            }};"#,
            upper_id = item_id.to_shouty_snake_case(),
            identifiers = self.identifiers().as_slice().bake(),
            maps = maps.as_slice().bake(),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata(),
            full_stats = stats.as_slice().bake(),
            map_stats = map_stats(&self.data.item_id, &stats)?
                .into_iter()
                .map(|(mode, stats)| {
                    format!("(types::MapMode::{mode:?}, {})", stats.as_slice().bake())
                })
                .collect::<Vec<_>>()
                .join(", "),
            builds_from = item_ids(&self.builds_from),
            builds_into = item_ids(&self.builds_into),
        )?;

        rust.push_str(&self.closures(&plan)?);
        rust.push_str(&self.constants.bake());

        let out = context.out.join(Tag::Items.as_ref()).join(item_id);

        wiki::write(out.with_extension("rs"), rust)?;

        Ok(BuildOutput {
            eval: self.eval(),
            formulas: if context.docs {
                self.formula_docs(&plan)
            } else {
                Vec::new()
            },
        })
    }
}

/// Stats of the item on each mode that changes them, as "Attack damage changed to
/// 45" in Arena: the stats of `stats`, with the ones it changes replaced
/// Stats of an item on each mode that changes them
type MapStats = Vec<(MapMode, Vec<(StatName, u16)>)>;

fn map_stats(item_id: &str, stats: &[(StatName, u16)]) -> MayFail<MapStats> {
    let mut result = Vec::new();

    for changes in map_changes() {
        let target = ChangeTarget::Item {
            item_id: item_id.to_string(),
        };

        let Some(entry) = changes.entries.values().find(|e| e.target == target) else {
            continue;
        };

        let mut changed = stats.to_vec();

        for change in &entry.changes {
            let ChangeKind::ItemStat { stat, amount } = &change.kind else {
                continue;
            };

            // Stats are whole numbers, as in the item data
            let (Values::Scalar(value), None) = (&amount.values, &amount.ranged) else {
                continue;
            };

            match changed.iter_mut().find(|(name, _)| name == stat) {
                Some((_, current)) => *current = *value as u16,
                None => changed.push((*stat, *value as u16)),
            }
        }

        if changed != stats {
            let [mode] = changes
                .maps
                .iter()
                .map(|map| map.mode())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()[..]
            else {
                return Err(
                    format!("[{item_id}] Maps {:?} have different modes", changes.maps).into(),
                );
            };

            result.push((mode, changed));
        }
    }

    Ok(result)
}

fn item_ids(items: &[String]) -> String {
    let variants = items
        .iter()
        .map(|item| format!("crate::libgen::ItemId::{item}"))
        .collect::<Vec<_>>()
        .join(",");

    format!("&[{variants}]")
}

impl Item {
    fn finish(&self) -> ItemExt {
        let data = &self.data;

        ItemExt {
            tier: data.tier.unwrap_or(1),
            price: data.buy.unwrap_or(0),
            stats: data
                .stats
                .iter()
                .map(|(k, v)| {
                    let stat = k
                        .parse::<u8>()
                        .ok()
                        .and_then(StatName::from_repr)
                        .unwrap_or_else(|| {
                            k.parse()
                                .map_err(|e| {
                                    format!(
                                        "[parse] {e}: {k} for {item_id}",
                                        item_id = data.item_id
                                    )
                                })
                                .unwrap()
                        });

                    (stat, *v as _)
                })
                .collect(),
            maps: {
                let mut maps = data
                    .modes
                    .iter()
                    .filter(|(_, v)| **v)
                    .filter_map(|(k, _)| {
                        k.parse::<u8>()
                            .ok()
                            .map(GameMap::from_code)
                            .or(match k.as_str() {
                                "ar" => Some(GameMap::Arena),
                                "aram" => Some(GameMap::Aram),
                                "mayhem" => Some(GameMap::AramMayhem),
                                "classic sr 5v5" => Some(GameMap::SummonersRift),
                                "nb" => Some(GameMap::NexusBlitz),
                                _ => None,
                            })
                    })
                    .collect::<Vec<_>>();

                // The wiki lists Mayhem only for the few items where it differs
                // from ARAM, so every other item follows its ARAM availability
                if !data.modes.contains_key("mayhem") && maps.contains(&GameMap::Aram) {
                    maps.push(GameMap::AramMayhem);
                }

                maps
            },
        }
    }
}
