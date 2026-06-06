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
    JsonRead, MayFail, gen_factories::wiki_runes::RuneBuild,
    generators::gen_factories::wiki_runes::Rune,
};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{AttackType, DamageIndex};

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
                        custom,
                    },
                ..
            } = rune;

            let fns = get_fn_names(rune_id, melee, ranged);
            let functions = [[&fns[0], &fns[1]], [&fns[2], &fns[3]]];

            let fmt_arg = json!(FmtArgs {
                target: "formula",
                variant: rune_id,
                meta: (),
                replace: [
                    (": X = X", " ="),
                    ("TypeMetadata ", ""),
                    ("RuneId::", ""),
                    ("ctx.", ""),
                ]
                .into(),
                default: false
            });

            let decl = format!(
                r#"
#[fmt({fmt_arg})]
static {upper_id}: X = X {{
    name: {name:?}, {damage}
}};

pub static {upper_id}: X = X {{
    name: {name:?},
    metadata: {metadata},
    {fn_names}
    deals_damage: {deals_damage:?},
    riot_id: {riot_id},
    identifiers: {identifiers},
    custom: {custom}
}};
                "#,
                upper_id = to_ssnake(rune_id),
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
            let eval = get_eval(Tag::Rune, &rune_id, &deals_damage, &functions);
            let fn_closures = closures(&functions, melee, ranged, rune_id);

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
        + &cache
        + "type X = Rune;";

    Ok((fmt_args, fmt))
}

pub fn finish(target: &str, variable: &mut String, mut value: Vec<FmtOutput<'_>>) {
    value.sort_by(|a, b| match &a.json.meta {
        v if let Ok((ata, dia)) =
            serde_json::from_value::<(AttackType, DamageIndex)>(v.clone())
            && let Ok((atb, dib)) =
                serde_json::from_value::<(AttackType, DamageIndex)>(b.json.meta.clone()) =>
        {
            ata.cmp(&atb).then(dia.cmp(&dib))
        }
        _ => a.json.target.cmp(&b.json.target),
    });

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
