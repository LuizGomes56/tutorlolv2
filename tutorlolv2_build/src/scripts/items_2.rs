use crate::scripts::{
    batch::{FmtArgs, Batch},
    utils::{
        StaticVar, Tag, closures, get_const_eval, get_eval, get_generator, get_id_enum,
        get_name_phf, get_static_vars,
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use tutorlolv2_dev::{
    JsonRead, MayFail, gen_factories::wiki_items::ItemBuild,
    generators::gen_factories::wiki_items::Item,
};

pub fn generate_items() -> MayFail<(HashMap<&'static str, String>, String)> {
    let data = BTreeMap::<String, Item>::from_file("internal/items.json")?;

    let result = data
        .par_iter()
        .map(|(item_id, item)| {
            let Item {
                build:
                    ItemBuild {
                        name,
                        tier,
                        price,
                        stats,
                        maps,
                        metadata,
                        ranged,
                        melee,
                        deals_damage,
                        purchasable,
                        riot_id,
                        identifiers,
                        functions,
                    },
                ..
            } = item;

            let fmt_arg = json!(FmtArgs {
                target: "formula",
                variant: item_id,
                meta: (),
                replace: [(": Item = Item", " ="), ("TypeMetadata ", ""),].into(),
                default: false
            });

            let decl = format!(
                r#"
                #[fmt({fmt_arg})]
                static {upper_id}: Item = Item {{
                    name: {name:?},
                    tier: {tier},
                    price: {price},
                    purchasable: {purchasable:?},
                    maps: {maps:?},
                    stats: {stats:?},
                    metadata: {metadata:?},
                    ranged: {ranged:?},
                    melee: {melee:?},
                    riot_id: {riot_id},
                }};

                #[derive(Clone, Debug, Deserialize, Serialize)]
                pub static {upper_id}: Item = Item {{
                    name: {name:?},
                    tier: {tier},
                    price: {price},
                    stats: &{stats:?},
                    maps: &{maps:?},
                    metadata: {metadata:?},
                    ranged: {ranged:?},
                    melee: {melee:?},
                    deals_damage: {deals_damage:?},
                    purchasable: {purchasable:?},
                    riot_id: {riot_id},
                    identifiers: {identifiers:?},
                }};
                "#,
                upper_id = item_id.to_uppercase(),
            );

            let generator = get_generator(Tag::Item, &item_id, item_id);
            let eval = get_eval(Tag::Item, &item_id, &deals_damage, functions);
            let fn_closures = closures(functions, melee, ranged, item_id);

            (
                item_id,
                Batch {
                    eval,
                    fmt: [decl, generator, fn_closures].concat(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let const_eval = get_const_eval(&result, Tag::Item);
    let item_id_enum = get_id_enum(&data, Tag::Item);
    let item_name_to_id = get_name_phf(&data, Tag::Item, None);

    let (cache, fmt_args) = get_static_vars(
        Tag::Item,
        &data,
        [
            StaticVar {
                attribute: "formula",
                name: "ITEM_FORMULAS",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "generator",
                name: "ITEM_GENERATOR",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "closure",
                name: "ITEM_CLOSURES",
                vtype: "[[Range<usize>; 2]; 2]",
            },
        ],
    );

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<_>>()
        .concat()
        + &item_id_enum
        + &item_name_to_id
        + &const_eval
        + &cache;

    Ok((fmt_args, fmt))
}
