use crate::scripts::_batch::{Batch, get_aliases, get_arg, simplify};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};
use tutorlolv2_dev::{
    JsonRead, MayFail,
    gen_factories::{ZERO, wiki_runes::RuneBuild},
    generators::gen_factories::wiki_runes::Rune,
};

pub fn generate_runes() -> MayFail<(String, Vec<(&'static str, String)>)> {
    let data = BTreeMap::<String, Rune>::from_file("internal/runes.json")?;

    let result = data
        .par_iter()
        .map(|(rune_id, rune)| {
            let Rune {
                build:
                    RuneBuild {
                        name,
                        metadata,
                        melee,
                        ranged,
                        riot_id,
                        deals_damage,
                        identifiers,
                        functions,
                    },
                ..
            } = rune;

            let variant = format_args!("variant({rune_id})");

            let decl = format!(
                "
                #[derive(Clone, Debug, Deserialize, Serialize)]
                #[fmt(formula, keep, {variant})]
                pub static {upper_id}: Rune = Rune {{
                    name: {name:?},
                    metadata: {metadata:?},
                    ranged: {ranged:?},
                    melee: {melee:?},
                    deals_damage: {deals_damage:?},
                    riot_id: {riot_id},
                    identifiers: {identifiers:?},
                    functions: {functions:?},
                }};
                ",
                upper_id = rune_id.to_uppercase(),
            );

            let mut generator = tutorlolv2_dev::read_to_string(format!(
                "tutorlolv2_dev/src/generators/gen_runes/{file_name}.rs",
                file_name = rune_id.to_lowercase()
            ))
            .unwrap_or("impl Generator {}".into());

            if let Some(pos) = generator.find("impl") {
                generator.drain(..pos);
            }

            generator.insert_str(0, &format!("#[fmt(generator, {variant})]"));

            let eval = {
                let get_arms = |range: Range<_>, array: &[String]| {
                    deals_damage[range]
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            let f = match *v {
                                true => &array[i],
                                false => ZERO,
                            };
                            format!("{f}(&ctx)")
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                };

                format!(
                    "
                    RuneId::{rune_id} => {{
                        match attack_type {{
                            Melee => [{melee_arms}],
                            Ranged => [{ranged_arms}]
                        }}
                    }},
                    ",
                    melee_arms = get_arms(0..2, melee),
                    ranged_arms = get_arms(2..4, ranged),
                )
            };

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
                            let formula = simplify(body);

                            format!(
                                r#"
                                #[fmt(
                                    closure,
                                    keep,
                                    replace("pub const", ""),
                                    array({array_arg}),
                                    {variant}
                                )]
                                pub const fn {function}(ctx: &Ctx) -> f32 {{{formula}}}
                                "#
                            )
                        })
                        .collect::<String>()
                })
                .collect::<String>();

            (
                rune_id,
                Batch {
                    eval,
                    fmt: [decl, generator, fn_closures].concat(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let const_eval = format!(
        "
        pub const fn rune_const_eval(
            ctx: &Ctx,
            rune_id: RuneId,
            attack_type: AttackType
        ) -> [f32; 2] {{
            match rune_id {{ {eval} _ => [0.0, 0.0] }}
        }}
        ",
        eval = result
            .values()
            .map(|batch| batch.eval.as_str())
            .collect::<Vec<&str>>()
            .concat()
    );

    let rune_id_enum = format!(
        "
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr(u8)]
        pub enum RuneId {{{variants}}}

        impl RuneId {{
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
            .map(|(rune_id, rune)| format!(
                "{riot_id} => Some(Self::{rune_id}),",
                riot_id = rune.build.riot_id
            ))
            .collect::<String>()
    );

    let rune_name_to_id = format!(
        "pub static RUNE_NAME_TO_ID: phf::Map<&str, RuneId> = phf::phf_map!({arguments});",
        arguments = data
            .iter()
            .map(|(rune_id, rune)| {
                let name = &rune.data.name;

                let alias = BTreeSet::from_iter(get_aliases(rune_id, name))
                    .into_iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{alias} => RuneId::{rune_id}")
            })
            .collect::<String>()
    );

    let [mut rune_cache, rune_formulas, rune_generator, rune_closures] =
        core::array::from_fn(|i| {
            let (name, vtype) = [
                ("RUNE_CACHE", "&Rune"),
                ("RUNE_FORMULAS", "Range<usize>"),
                ("RUNE_GENERATOR", "Range<usize>"),
                ("RUNE_CLOSURES", "&[Range<usize>]"),
            ][i];
            format!("pub static {name}: [{vtype}; RuneId::VARIANTS] = [")
        });

    for rune_id in data.keys() {
        let upper_id = rune_id.to_uppercase();
        rune_cache.push_str(&format!("&{upper_id},"));
    }

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<&str>>()
        .concat()
        + &rune_id_enum
        + &rune_name_to_id
        + &const_eval;

    let fmt_args = vec![
        ("formula", rune_formulas),
        ("generator", rune_generator),
        ("closure", rune_closures),
    ];

    Ok((fmt, fmt_args))
}
