use crate::scripts::{
    batch::{Batch, FmtArgs, FmtOutput},
    utils::{
        StaticVar, Tag, closures, get_const_eval, get_eval, get_fn_names, get_generator,
        get_id_enum, get_identifiers, get_name_phf, get_static_vars, repr_damages,
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};
use tutorlolv2_dev::{
    JsonRead, MayFail,
    gen_factories::{DamageIndex, wiki_runes::RuneBuild},
    generators::gen_factories::wiki_runes::Rune,
};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::AttackType;

pub fn generate_runes() -> MayFail<(HashMap<&'static str, String>, String)> {
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

            let fmt_arg = json!(FmtArgs {
                target: "formula",
                variant: rune_id,
                meta: (),
                replace: [(": Rune = Rune", " ="), ("TypeMetadata ", ""),].into(),
                default: false
            });

            let decl = format!(
                r#"
                #[fmt({fmt_arg})]
                static {upper_id}: Rune = Rune {{
                    name: {name:?},
                    riot_id: {riot_id},
                    ranged: {ranged},
                    melee: {melee},
                    metadata: {metadata:?},
                }};

                pub static {upper_id}: Rune = Rune {{
                    name: {name:?},
                    metadata: {metadata},
                    ranged: {ranged_fns},
                    melee: {melee_fns},
                    deals_damage: {deals_damage:?},
                    riot_id: {riot_id},
                    identifiers: {identifiers},
                }};
                "#,
                upper_id = to_ssnake(rune_id),
                melee = repr_damages(melee),
                ranged = repr_damages(ranged),
                melee_fns = get_fn_names(&functions[AttackType::Melee as usize], melee),
                ranged_fns = get_fn_names(&functions[AttackType::Ranged as usize], ranged),
                identifiers = get_identifiers(&identifiers),
                metadata = format_args!(
                    "TypeMetadata {{
                        kind: RuneId::{kind},
                        damage_type: {damage_type:?},
                        attributes: {attributes:?},
                    }}",
                    kind = metadata.kind,
                    damage_type = metadata.damage_type,
                    attributes = metadata.attributes,
                )
            );

            let generator = get_generator(Tag::Rune, &rune_id, rune_id);
            let eval = get_eval(Tag::Rune, &rune_id, &deals_damage, functions);
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

    let (cache, fmt_args) = get_static_vars(
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
                vtype: "[[Range<usize>; 2]; 2]",
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

    Ok((fmt_args, fmt))
}

pub fn finish(target: &str, variable: &mut String, value: &[FmtOutput<'_>]) {
    let push = match target {
        "formula" | "generator" => {
            value
                .iter()
                .map(|FmtOutput { html_range, .. }| format!("{html_range:?}"))
                .collect::<Vec<_>>()
                .join(",")
                + ","
        }
        "closure" => {
            let mut ranges: [[Range<usize>; 2]; 2] =
                core::array::from_fn(|_| core::array::from_fn(|_| 0..0));

            for FmtOutput {
                html_range,
                json: FmtArgs { meta, .. },
                ..
            } in value
            {
                let (attack_type, damage_index) =
                    serde_json::from_value::<(AttackType, DamageIndex)>(meta.clone()).unwrap();

                ranges[attack_type as usize][damage_index as usize] = html_range.clone();
            }

            format!("{ranges:?},")
        }
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}
