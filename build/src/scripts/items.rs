use crate::{
    Build, BuildContext, BuildOutput, MayFail,
    generators::{parser::items::Item, utils::Tag},
    scripts::utils::{ItemOrRuneExt, bake},
};
use heck::ToShoutySnakeCase;
use std::fmt::Write;
use tutorlolv2_types::{GameMap, StatName};
use tutorlolv2_wiki::items::item_parser::WikiItem;

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
                maps: {maps},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                purchasable: {purchasable},
                riot_id: {id},
                custom: {custom},

                #[cfg(feature = "yew")]
                identifiers: {identifiers},
            }};"#,
            upper_id = item_id.to_shouty_snake_case(),
            identifiers = bake(&self.identifiers().as_slice()),
            maps = bake(&maps.as_slice()),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata(),
            full_stats = bake(&stats.as_slice()),
        )?;

        rust.push_str(&self.closures(&plan)?);

        let out = context.out.join(Tag::Items.as_ref()).join(item_id);

        tutorlolv2_wiki::write(out.with_extension("rs"), rust)?;

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
            maps: data
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
                            "classic sr 5v5" => Some(GameMap::SummonersRift),
                            "nb" => Some(GameMap::NexusBlitz),
                            _ => None,
                        })
                })
                .collect(),
        }
    }
}
