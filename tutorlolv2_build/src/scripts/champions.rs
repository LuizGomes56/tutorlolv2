use crate::scripts::{
    batch::{Batch, FmtArgs, FmtOutput},
    utils::{
        StaticVar, Tag, cast_f32, ctx_param, get_generator, get_id_enum, get_name_phf,
        get_static_vars, simplify,
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tutorlolv2_dev::{
    JsonRead, MayFail, decl_champions::Ability, gen_factories::wiki_champions::ChampionBuild,
    generators::gen_factories::wiki_champions::Champion,
};
use tutorlolv2_fmt::to_ssnake;

pub fn generate_champions() -> MayFail<(HashMap<&'static str, String>, String)> {
    let data = BTreeMap::<String, Champion>::from_file("internal/champions.json")?;
    let languages =
        BTreeMap::<String, BTreeSet<String>>::from_file("internal/champion_languages.json")?;

    let result = data
        .par_iter()
        .map(|(champion_id, champion)| {
            let Champion {
                abilities,
                build:
                    ChampionBuild {
                        name,
                        adaptive_type,
                        attack_type,
                        positions,
                        stats,
                        modifiers,
                        combos,
                        metadata,
                        closures,
                        merge_data,
                        identifiers,
                        functions,
                    },
                ..
            } = champion;

            let fmt_formula = json!(FmtArgs {
                target: "formula",
                variant: champion_id,
                meta: (),
                replace: [
                    (": Champion = Champion", " ="),
                    ("MergeData ", ""),
                    ("WikiStats ", ""),
                    ("Stats ", ""),
                    ("WikiModifiers ", ""),
                    ("Modifiers ", ""),
                    ("TypeMetadata ", ""),
                ]
                .into(),
                default: false
            });

            let decl = format!(
                r#"
                #[fmt({fmt_formula})]
                static {upper_id}: Champion = Champion {{
                    name: {name:?},
                    adaptive_type: {adaptive_type:?},
                    attack_type: {attack_type:?},
                    positions: {positions:#?},
                    stats: {stats:#?},
                    modifiers: {modifiers:#?},
                    combos: [{combos}],
                    metadata: {metadata:#?},
                    merge_data: {merge_data:#?}
                }};

                pub static {upper_id}: Champion = Champion {{
                    name: {name:?},
                    adaptive_type: AdaptiveType::{adaptive_type:?},
                    attack_type: {attack_type:?},
                    positions: &{positions:#?},
                    stats: {stats:#?},
                    modifiers: {modifiers:#?},
                    combos: &[{combos}],
                    metadata: &{metadata:#?},
                    merge_data: &{merge_data:#?},
                    identifiers: &[{identifiers}],
                    closures: &[{fn_names}],
                }};
                "#,
                upper_id = to_ssnake(champion_id),
                combos = combos
                    .iter()
                    .map(|ident| format!("&{ident:#?}"))
                    .collect::<Vec<_>>()
                    .join(","),
                identifiers = identifiers
                    .iter()
                    .enumerate()
                    .map(|(i, slice)| {
                        let rest = if i == 0 { "as &[_]" } else { "" };
                        format!("&{slice:?}{rest}")
                    })
                    .collect::<Vec<_>>()
                    .join(","),
                fn_names = functions.join(","),
            );

            let generator = get_generator(Tag::Champion, champion_id, champion_id);

            let abilities_decl = abilities
                .iter()
                .zip(functions)
                .zip(closures)
                .map(|(((ability_id, ability), function), body)| {
                    let Ability {
                        name,
                        damage_type,
                        attributes,
                        comment,
                        damage,
                    } = ability;

                    let formula = simplify(body);

                    let fmt_closure = json!(FmtArgs {
                        target: "closure",
                        variant: champion_id,
                        meta: ability_id,
                        replace: [("ctx.", "")].into(),
                        default: false
                    });

                    let fmt_ability = json!(FmtArgs {
                        target: "ability",
                        variant: champion_id,
                        meta: ability_id,
                        replace: [(": Ability = Ability", " = Ability"), ("ctx.", "")].into(),
                        default: false
                    });

                    let formula_f32 = cast_f32(&formula);

                    format!(
                        r#"
                        pub const fn {function}({param}: &Ctx) -> f32 {{{formula_f32}}}

                        #[fmt({fmt_closure})]
                        fn {function}() {{{formula}}}

                        #[fmt({fmt_ability})]
                        static {variable}: Ability = Ability {{
                            name: {name:?},
                            damage_type: {damage_type:?},
                            attributes: {attributes:?},
                            comment: {comment:?},
                            damage: {damage},
                        }};
                        "#,
                        variable = function.to_uppercase(),
                        damage = simplify(damage),
                        param = ctx_param(&formula_f32)
                    )
                })
                .collect::<String>();

            let eval = format!(
                r#"
                ChampionId::{champion_id} => {{
                    match kind {{
                        {arms}
                        _ => panic!("Invalid AbilityId provided for '{champion_id}'"),
                    }}
                }},
                "#,
                arms = functions
                    .iter()
                    .zip(metadata)
                    .map(|(function, metadata)| {
                        let ability_id = metadata.kind;
                        format!("{ability_id:?} => {function}(ctx),")
                    })
                    .collect::<String>()
            );

            (
                champion_id,
                Batch {
                    eval,
                    fmt: [decl, generator, abilities_decl].concat(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let const_eval = format!(
        "pub const fn ability_const_eval(
            ctx: &Ctx,
            champion_id: ChampionId,
            kind: AbilityId
        ) -> f32 {{
            match champion_id {{{eval}}}
        }}",
        eval = result
            .values()
            .map(|batch| batch.eval.as_str())
            .collect::<Vec<_>>()
            .concat()
    );

    let champion_id_enum = get_id_enum(&data, Tag::Champion);

    let alias = data
        .keys()
        .map(|champion_id| {
            (
                champion_id.clone(),
                languages[champion_id]
                    .iter()
                    .cloned()
                    .chain(
                        (champion_id == "Gnar")
                            .then_some("Mega Gnar".into())
                            .into_iter(),
                    )
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let champion_name_to_id = get_name_phf(&data, Tag::Champion, Some(alias));
    let recommendations = get_recommendations(data.len())?;

    let (cache, fmt_args) = get_static_vars(
        Tag::Champion,
        &data,
        [
            StaticVar {
                attribute: "formula",
                name: "CHAMPION_FORMULAS",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "generator",
                name: "CHAMPION_GENERATOR",
                vtype: "Range<usize>",
            },
            StaticVar {
                attribute: "ability",
                name: "ABILITY_FORMULAS",
                vtype: "&[Range<usize>]",
            },
            StaticVar {
                attribute: "closure",
                name: "ABILITY_CLOSURES",
                vtype: "&[Range<usize>]",
            },
        ],
    );

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<&str>>()
        .concat()
        + &champion_id_enum
        + &champion_name_to_id
        + &const_eval
        + &cache
        + &recommendations;

    Ok((fmt_args, fmt))
}

pub fn get_recommendations(len: usize) -> MayFail<String> {
    let enum_ids = ["ItemId", "RuneId"];
    let declaration = ["RECOMMENDED_ITEMS", "RECOMMENDED_RUNES"];

    let mut globals = core::array::from_fn::<_, 2, _>(|i| {
        let enumv = enum_ids[i];
        let var = declaration[i];
        format!("pub static {var}: [[&[crate::{enumv}]; 5]; {len}] = [")
    });

    let json = BTreeMap::<String, BTreeMap<String, [BTreeSet<String>; 2]>>::from_file(
        "internal/scraper/data.json",
    )
    .unwrap_or_default();

    let push_end = |globals: &mut [String; 2], str| {
        for value in globals.each_mut() {
            value.push_str(str);
        }
    };

    for data in json.values() {
        push_end(&mut globals, "[");
        for recommendations in data.values() {
            for (i, value) in core::array::from_fn::<_, 2, _>(|j| {
                let venum = enum_ids[j];
                let result = recommendations[j]
                    .iter()
                    .map(|element| format!("{venum}::{element}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("&[{result}]")
            })
            .into_iter()
            .enumerate()
            {
                globals[i].push_str(&format!("{value},"));
            }
        }
        push_end(&mut globals, "],");
    }

    push_end(&mut globals, "];");
    Ok(globals.concat())
}

pub fn finish(target: &str, variable: &mut String, value: &[FmtOutput<'_>]) {
    let ranges = value
        .iter()
        .map(|FmtOutput { html_range, .. }| format!("{html_range:?}"))
        .collect::<Vec<_>>()
        .join(",");

    let push = match target {
        "formula" | "generator" => format!("{ranges},"),
        "ability" | "closure" => format!("&[{ranges},],"),
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}
