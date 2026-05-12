use crate::scripts::_batch::{Batch, get_aliases, get_arg};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_dev::{
    JsonRead, MayFail, gen_factories::wiki_items::ItemBuild,
    generators::gen_factories::wiki_items::Item,
};

pub fn generate_items() -> MayFail<(String, Vec<(&'static str, String)>)> {
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

            let variant = format!("variant({item_id})");

            let decl = format!(
                "
                #[derive(Clone, Debug, Deserialize, Serialize)]
                #[fmt(formula, keep, {variant})]
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
                    functions: {functions:?},
                }};
                ",
                upper_id = item_id.to_uppercase(),
            );

            let mut generator = tutorlolv2_dev::read_to_string(format!(
                "tutorlolv2_dev/src/generators/gen_items/{file_name}.rs",
                file_name = item_id.to_lowercase()
            ))
            .unwrap_or(
                "impl Generator for Item {
                    fn generate(&mut self) -> MayFail {
                        /* No implementation */
                    }
                }"
                .into(),
            );

            if let Some(pos) = generator.find("impl") {
                generator.drain(..pos);
            }

            generator.insert_str(0, &format!("#[fmt(generator, {variant})]"));

            let eval = format!(
                "
                ItemId::{item_id} => {{
                    match attack_type {{
                        Melee => [{melee_arms}],
                        Ranged => [{ranged_arms}]
                    }}
                }},
                ",
                melee_arms = 0,
                ranged_arms = 0,
            );

            let fn_closures = functions
                .iter()
                .enumerate()
                .map(|(i, function)| {
                    function
                        .iter()
                        .enumerate()
                        .map(|(j, function)| {
                            let array_arg = get_arg(functions.len(), &j);
                            let body = &match i {
                                0 => melee,
                                1 => ranged,
                                _ => unreachable!(),
                            }[j];

                            format!(
                                r#"
                                #[fmt(
                                    closure,
                                    keep,
                                    replace("pub const", ""),
                                    array({array_arg}),
                                    {variant}
                                )]
                                pub const fn {function}(ctx: &Ctx) -> f32 {{{body}}}
                                "#
                            )
                        })
                        .collect::<String>()
                })
                .collect::<String>();

            (
                item_id,
                Batch {
                    eval,
                    fmt: [decl, generator, fn_closures].concat(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let const_eval = format!(
        "
        pub const fn item_const_eval(
            ctx: &Ctx,
            item_id: ItemId,
            attack_type: AttackType
        ) -> [f32; 2] {{
            match item_id {{ {eval} _ => [0.0, 0.0] }}
        }}
        ",
        eval = result
            .values()
            .map(|batch| batch.eval.as_str())
            .collect::<Vec<&str>>()
            .concat()
    );

    let item_id_enum = format!(
        "
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr(u8)]
        pub enum ItemId {{{variants}}}

        impl ItemId {{
            pub const VARIANTS: usize = {len};
            pub const fn debug(&self) -> &'static str {{
                match self {{{debug_arms}}}
            }}
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms} _ => None }}
            }}
        }}
        ",
        variants = data
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(","),
        len = data.len(),
        debug_arms = data
            .keys()
            .map(|name| format!("Self::{name} => {name:?},"))
            .collect::<String>(),
        match_arms = data
            .iter()
            .map(|(item_id, item)| format!(
                "{riot_id} => Some(Self::{item_id}),",
                riot_id = item.data.id
            ))
            .collect::<String>()
    );

    let item_name_to_id = format!(
        "pub static ITEM_NAME_TO_ID: phf::Map<&str, ItemId> = phf::phf_map!({arguments});",
        arguments = data
            .iter()
            .map(|(item_id, item)| {
                let name = &item.data.name;

                let alias = BTreeSet::from_iter(get_aliases(item_id, name))
                    .into_iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{alias} => ItemId::{item_id}")
            })
            .collect::<String>()
    );

    let [mut item_cache, item_formulas, item_generator, item_closures] =
        core::array::from_fn(|i| {
            let (name, vtype) = [
                ("ITEM_CACHE", "&Item"),
                ("ITEM_FORMULAS", "Range<usize>"),
                ("ITEM_GENERATOR", "Range<usize>"),
                ("ITEM_CLOSURES", "&[Range<usize>]"),
            ][i];
            format!("pub static {name}: [{vtype}; ItemId::VARIANTS] = [")
        });

    for item_id in data.keys() {
        let upper_id = item_id.to_uppercase();
        item_cache.push_str(&format!("&{upper_id},"));
    }

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<&str>>()
        .concat()
        + &item_id_enum
        + &item_name_to_id
        + &const_eval;

    let fmt_args = vec![
        ("formula", item_formulas),
        ("generator", item_generator),
        ("closure", item_closures),
    ];

    Ok((fmt, fmt_args))
}
