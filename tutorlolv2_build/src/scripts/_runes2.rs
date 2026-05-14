use crate::{
    Tracker,
    scripts::utils::{
        Batch, StaticVar, Tag, closures, get_const_eval, get_eval, get_generator, get_id_enum,
        get_name_phf, get_static_vars,
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::BTreeMap;
use tutorlolv2_dev::{
    JsonRead, MayFail, gen_factories::wiki_runes::RuneBuild,
    generators::gen_factories::wiki_runes::Rune,
};

pub fn generate_runes() -> MayFail<Box<dyn FnOnce(&mut Tracker<'_>) -> MayFail<String>>> {
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

            let decl = format!(
                r#"
                #[fmt(
                    target = formula,
                    variant = {rune_id},
                    replace = [
                        ": Rune = Rune" => " =",
                        "TypeMetadata " => ""
                    ]
                )]
                static {upper_id}: Rune = Rune {{
                    name: {name:?},
                    riot_id: {riot_id},
                    ranged: {ranged:?},
                    melee: {melee:?},
                    metadata: {metadata:?},
                }};

                #[derive(Clone, Debug, Deserialize, Serialize)]
                pub static {upper_id}: Rune = Rune {{
                    name: {name:?},
                    metadata: {metadata:?},
                    ranged: {ranged:?},
                    melee: {melee:?},
                    deals_damage: {deals_damage:?},
                    riot_id: {riot_id},
                    identifiers: {identifiers:?},
                }};
                "#,
                upper_id = rune_id.to_uppercase(),
            );

            let generator = get_generator(Tag::Rune, &rune_id, rune_id);
            let eval = get_eval(Tag::Rune, &rune_id, &deals_damage, melee, ranged);
            let fn_closures = closures(functions, melee, ranged, rune_id);

            (
                rune_id,
                Batch {
                    eval,
                    fmt: [decl, generator, fn_closures].concat(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let const_eval = get_const_eval(&result, Tag::Rune);
    let rune_id_enum = get_id_enum(&data, Tag::Rune);
    let rune_name_to_id = get_name_phf(&data, Tag::Rune, None);

    let (cache, mut fmt_args) = get_static_vars(
        Tag::Rune,
        &data,
        [
            StaticVar {
                attribute: "formula",
                name: "RUNE_FORMULAS",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "generator",
                name: "RUNE_GENERATOR",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "closure",
                name: "RUNE_CLOSURES",
                vtype: "&[Range<usize>]",
            },
        ],
    );

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<_>>()
        .concat()
        + &rune_id_enum
        + &rune_name_to_id
        + &const_eval
        + &cache;

    Ok(Box::new(move |tracker| {
        tracker.batch(fmt, &mut fmt_args)?;
        Ok(String::new())
    }))
}
