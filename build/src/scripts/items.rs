use crate::{
    MayFail,
    generators::parser::{get_identifiers, items::Item},
    model::items::WikiItem,
    scripts::{
        batch::FmtArgs,
        utils::{
            Tag, closures, get_eval, get_fn_names, get_identifiers as repr_identifiers,
            repr_damages,
        },
    },
};
use serde_json::json;
use std::{collections::BTreeSet, fmt::Write, path::PathBuf};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex, GameMap, StatName};

struct ItemExt {
    pub tier: u8,
    pub price: u16,
    pub stats: Vec<(StatName, u16)>,
    pub maps: Vec<GameMap>,
    pub identifiers: [[BTreeSet<CtxVar>; 2]; 2],
}

impl Item {
    pub fn build(&self, out: PathBuf) -> MayFail {
        let ItemExt {
            tier,
            price,
            stats,
            maps,
            identifiers,
        } = self.finish();

        let Self {
            attributes,
            data:
                WikiItem {
                    id,
                    name,
                    item_id,
                    purchasable,
                    custom,
                    ..
                },
            damage_type,
            ranged,
            melee,
        } = self;

        let mut rust = String::new();
        let mut docs = String::new();

        let upper_id = to_ssnake(&item_id);
        let damage = {
            let dmg = repr_damages(&melee, &ranged);
            if !dmg.is_empty() {
                format!("{dmg} damage_type: {damage_type:?}")
            } else {
                dmg
            }
        };

        let fns = get_fn_names(&item_id, &melee, &ranged);
        let functions = [[&fns[0], &fns[1]], [&fns[2], &fns[3]]];
        let deals_damage = [melee.deals_damage(), ranged.deals_damage()].concat();

        write!(
            rust,
            "pub static {upper_id}: X = X {{
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
                identifiers: {identifiers},
                custom: {custom}
            }};",
            fn_names = {
                let melee_fns = fns[0..2].join(",");
                let ranged_fns = fns[2..4].join(",");

                format!("melee: [{melee_fns}], ranged: [{ranged_fns}],")
            },
            identifiers = repr_identifiers(&identifiers),
            full_stats = stats
                .iter()
                .map(|(stat, number)| { format!("(StatName::{stat:?}, {number})") })
                .collect::<Vec<_>>()
                .join(", "),
            metadata = format_args!(
                "TypeMetadata {{
                    kind: ItemId::{item_id},
                    damage_type: {damage_type:?},
                    attributes: {attributes:?},
                }}"
            )
        )?;

        write!(
            docs,
            "#[fmt({fmt})]
            static {upper_id}: X = X {{
                name: {name:?},
                stats: {stats:?},
                price: {price},
                maps: {maps:?},
                tier: {tier},
                purchasable: {purchasable},{damage}
            }};",
            fmt = json!(FmtArgs {
                target: "formula",
                variant: &item_id,
                meta: (),
                replace: [
                    (": X = X", " ="),
                    ("TypeMetadata ", ""),
                    ("ItemId::", ""),
                    ("ctx.", ""),
                ]
                .into(),
                default: false
            })
        )?;

        for vec in closures(&functions, &melee, &ranged, &item_id) {
            for (code, doc) in &vec {
                rust.push_str(code);
                docs.push_str(doc);
            }
        }

        let eval = get_eval(Tag::Item, &item_id, &deals_damage, &functions);

        crate::write(&out.with_extension("rs"), rust)?;
        crate::write(&out.with_extension("w48"), docs)?;
        crate::write(&out.with_extension("eval"), eval)
    }

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
            identifiers: core::array::from_fn(|i| {
                let attack_type = match i {
                    0 => AttackType::Melee,
                    1 => AttackType::Ranged,
                    _ => unreachable!(),
                };

                core::array::from_fn(|j| {
                    let damage_index = match j {
                        0 => DamageIndex::Min,
                        1 => DamageIndex::Max,
                        _ => unreachable!(),
                    };

                    get_identifiers(&self[attack_type][damage_index], self.damage_type)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect()
                })
            }),
        }
    }
}
