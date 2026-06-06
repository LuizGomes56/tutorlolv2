use crate::scripts::{
    batch::{Batch, FmtArgs},
    utils::{
        StaticVar, Tag, closures, get_const_eval, get_eval, get_fn_names, get_generator,
        get_id_enum, get_identifiers, get_name_phf, get_static_vars, repr_damages,
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use tutorlolv2_dev::{
    JsonRead, MayFail, gen_factories::wiki_items::ItemBuild,
    generators::gen_factories::wiki_items::Item,
};
use tutorlolv2_fmt::to_ssnake;

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
                        custom,
                    },
                ..
            } = item;

            let fmt_arg = json!(FmtArgs {
                target: "formula",
                variant: item_id,
                meta: (),
                replace: [
                    (": X = X", " ="),
                    ("TypeMetadata ", ""),
                    ("ItemId::", ""),
                    ("ctx.", ""),
                ]
                .into(),
                default: false
            });

            let fns = get_fn_names(item_id, melee, ranged);
            let functions = [[&fns[0], &fns[1]], [&fns[2], &fns[3]]];

            let decl = format!(
                r#"
#[fmt({fmt_arg})]
static {upper_id}: X = X {{
    name: {name:?},
    stats: {stats:?},
    price: {price},
    maps: {maps:?},
    tier: {tier},
    purchasable: {purchasable},{damage}
}};

pub static {upper_id}: X = X {{
    name: {name:?},
    tier: {tier},
    price: {price},
    stats: &[{full_stats}],
    maps: &{maps:?},
    metadata: {metadata},
    {fn_names}
    deals_damage: {deals_damage:?},
    purchasable: {purchasable},
    riot_id: {riot_id},
    identifiers: {identifiers},
    custom: {custom}
}};
                "#,
                upper_id = to_ssnake(item_id),
                damage = {
                    let dmg = repr_damages(melee, ranged, deals_damage);
                    if !dmg.is_empty() {
                        let damage_type = metadata.damage_type;
                        format!("{dmg} damage_type: {damage_type:?}")
                    } else {
                        dmg
                    }
                },
                fn_names = {
                    let melee_fns = fns[0..2].join(",");
                    let ranged_fns = fns[2..4].join(",");

                    format!("melee: [{melee_fns}], ranged: [{ranged_fns}],")
                },
                identifiers = get_identifiers(&identifiers),
                full_stats = stats
                    .iter()
                    .map(|(stat, number)| { format!("(StatName::{stat:?}, {number})") })
                    .collect::<Vec<_>>()
                    .join(", "),
                metadata = format_args!(
                    "TypeMetadata {{
        kind: ItemId::{kind},
        damage_type: {damage_type:?},
        attributes: {attributes:?},
    }}",
                    kind = metadata.kind,
                    damage_type = metadata.damage_type,
                    attributes = metadata.attributes,
                )
            );

            let generator = get_generator(Tag::Item, &item_id, item_id);
            let eval = get_eval(Tag::Item, &item_id, &deals_damage, &functions);
            let fn_closures = closures(&functions, melee, ranged, item_id);

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
        + &cache
        + "type X = Item;";

    Ok((fmt_args, fmt))
}
