use crate::{
    Build, MayFail, OUT_DIR,
    generators::{parser::items::Item, utils::Tag},
    scripts::utils::{ItemOrRuneExt, fit_str},
};
use std::fmt::Write;
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{GameMap, StatName};
use tutorlolv2_wiki::items::item_parser::WikiItem;

struct ItemExt {
    pub tier: u8,
    pub price: u16,
    pub stats: Vec<(StatName, u16)>,
    pub maps: Vec<GameMap>,
}

impl Build for Item {
    fn build(&mut self) -> MayFail<String> {
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
        let mut docs = String::new();

        let upper_id = to_ssnake(&item_id);

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                tier: {tier},
                price: {price},
                stats: &[{full_stats}],
                maps: &{maps:?},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                purchasable: {purchasable},
                riot_id: {id},
                custom: {custom},
                #[cfg(feature = "docs")]
                identifiers: &{identifiers:?},
            }};"#,
            identifiers = self.identifiers(),
            deals_damage = self.deals_damage(),
            fn_names = self.function_names(),
            metadata = self.repr_metadata(),
            full_stats = stats
                .iter()
                .map(|(stat, number)| { format!("(StatName::{stat:?}, {number})") })
                .collect::<Vec<_>>()
                .join(", "),
        )?;

        write!(
            docs,
            "#[fmt({fmt})]
            static {var_name}: X = X {{
                name: {name},
                price: {price},
                stats: {stats:?},
                maps: {maps:?},
                tier: {tier},
                purchasable: {purchasable},
                {damage}
            }};",
            damage = self.repr_damages(),
            fmt = self.formula_fmt(),
            name = fit_str(&name),
            var_name = {
                let max_len = 28;
                if upper_id.len() > max_len {
                    upper_id[..max_len].to_string()
                } else {
                    upper_id
                }
            }
        )?;

        let (code, doc) = &self.closures()?;
        rust.push_str(code);
        docs.push_str(doc);

        let out = OUT_DIR.join(Tag::Items.plural()).join(item_id);

        crate::write(out.with_extension("rs"), rust)?;
        crate::write(out.with_extension("w48"), docs)?;

        Ok(self.eval())
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
                .filter_map(|(k, v)| {
                    let stat = k
                        .parse::<u8>()
                        .ok()
                        .map(StatName::from_u8)
                        .flatten()
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

                    Some((stat, *v as _))
                })
                .collect(),
            maps: data
                .modes
                .iter()
                .filter(|(_, v)| **v)
                .filter_map(|(k, _)| {
                    k.parse::<u8>()
                        .ok()
                        .map(GameMap::from_u8)
                        .or_else(|| match k.as_str() {
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
