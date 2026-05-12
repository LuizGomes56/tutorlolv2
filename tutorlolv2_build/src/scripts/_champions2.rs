use crate::{Tracker, scripts::_batch::fmt_batch};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Debug, Display},
};
use tutorlolv2_dev::{
    JsonRead, MayFail, decl_champions::Ability, gen_factories::wiki_champions::ChampionBuild,
    generators::gen_factories::wiki_champions::Champion,
};
use tutorlolv2_fmt::to_ssnake;

pub fn generate_champions() -> MayFail {
    std::env::set_current_dir("../")?;

    let data = BTreeMap::<String, Champion>::from_file("internal/champions.json")?;
    let languages =
        BTreeMap::<String, BTreeSet<String>>::from_file("internal/champion_languages.json")?;

    struct Batch {
        eval: String,
        fmt: String,
    }

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

            let variant = format!("variant({champion_id})");

            let decl = format!(
                "
                #[derive(Clone, Debug, Deserialize, Serialize)]
                #[fmt(champion_formulas, keep, {variant})]
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
                ",
                upper_id = champion_id.to_uppercase(),
                combos = slice_repr(&combos),
                identifiers = slice_repr(&identifiers),
                fn_names = functions.join(","),
            );

            let mut generator = tutorlolv2_dev::read_to_string(format!(
                "tutorlolv2_dev/src/generators/gen_champions/{file_name}.rs",
                file_name = champion_id.to_lowercase()
            ))
            .unwrap();

            generator.drain(0.."use super::*;\n".len());
            generator.insert_str(0, &format!("#[fmt(generator, {variant})]"));

            let abilities_decl = abilities
                .values()
                .zip(functions)
                .enumerate()
                .map(|(i, (ability, function))| {
                    let array_arg = get_arg(functions.len(), &i);
                    let Ability {
                        name,
                        damage_type,
                        attributes,
                        comment,
                        damage,
                    } = ability;

                    format!(
                        r#"
                        #[fmt(
                            ability_formulas,
                            replace(": Ability", ""),
                            replace("ctx.", ""),
                            array({array_arg}),
                            {variant}
                        )]
                        static {variable}: Ability = Ability {{
                            name: {name:?},
                            damage_type: {damage_type:?},
                            attributes: {attributes:?},
                            comment: {comment:?},
                            damage: {damage},
                        }};
                        "#,
                        variable = function.to_uppercase()
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

            let fn_closures = functions
                .iter()
                .zip(closures)
                .enumerate()
                .map(|(i, (function, body))| {
                    let array_arg = get_arg(functions.len(), &i);

                    format!(
                        r#"
                        #[fmt(
                            ability_closures,
                            keep,
                            replace("pub const", ""),
                            array({array_arg}),
                            {variant}
                        )]
                        pub const fn {function}(ctx: &Ctx) -> f32 {{{body}}}
                        "#
                    )
                })
                .collect::<String>();

            (
                champion_id,
                Batch {
                    eval,
                    fmt: [decl, generator, abilities_decl, fn_closures].concat(),
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
            .collect::<Vec<&str>>()
            .concat()
    );

    let champion_id_enum = format!(
        "
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr(u8)]
        pub enum ChampionId {{{variants}}}

        impl ChampionId {{
            pub const VARIANTS: usize = {len};
            pub const fn debug(&self) -> &'static str {{
                match self {{{arms}}}
            }}
        }}
        ",
        variants = data
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(","),
        len = data.len(),
        arms = data
            .keys()
            .map(|name| format!("Self::{name} => {name:?},"))
            .collect::<String>()
    );

    let champion_name_to_id = format!(
        "pub static CHAMPION_NAME_TO_ID: phf::Map<&str, ChampionId> = phf::phf_map!({arguments});",
        arguments = data
            .keys()
            .map(|champion_id| {
                let alias = languages[champion_id]
                    .iter()
                    .chain(core::iter::once(&champion_id.clone()))
                    .chain(core::iter::once(&champion_id.to_lowercase()))
                    .chain(core::iter::once(&to_ssnake(&champion_id)))
                    .chain(core::iter::once(&to_ssnake(&champion_id).to_lowercase()))
                    .chain(
                        (champion_id == "Gnar")
                            .then_some(&"Mega Gnar".into())
                            .into_iter(),
                    )
                    .map(|alias| format!("{alias:?}"))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{alias} => ChampionId::{champion_id},")
            })
            .collect::<String>()
    );

    let [
        mut champion_cache,
        champion_formulas,
        champion_generator,
        ability_formulas,
        ability_closures,
    ] = core::array::from_fn(|i| {
        let (name, vtype) = [
            ("CHAMPION_CACHE", "&Champion"),
            ("CHAMPION_FORMULAS", "Range<usize>"),
            ("CHAMPION_GENERATOR", "Range<usize>"),
            ("ABILITY_FORMULAS", "&[Range<usize>]"),
            ("ABILITY_CLOSURES", "&[Range<usize>]"),
        ][i];
        format!("pub static {name}: [{vtype}; ChampionId::VARIANTS] = [")
    });

    for champion_id in data.keys() {
        let upper_id = champion_id.to_uppercase();
        champion_cache.push_str(&format!("&{upper_id},"));
    }

    let fmt = result
        .values()
        .map(|batch| batch.fmt.as_str())
        .collect::<Vec<&str>>()
        .concat()
        + &champion_id_enum
        + &champion_name_to_id
        + &const_eval;

    let fmt_args = [
        ("champion_formulas", champion_formulas),
        ("generator", champion_generator),
        ("ability_formulas", ability_formulas),
        ("ability_closures", ability_closures),
    ];

    let mut inner = String::with_capacity(8 * 1024 * 1024);
    let mut tracker = Tracker::new(&mut inner);
    let src = fmt_batch(&mut tracker, fmt, fmt_args).unwrap();

    tutorlolv2_dev::write("src.txt", src).unwrap();
    tutorlolv2_dev::write("tracker.txt", inner).unwrap();

    // let module = format!(
    //     "
    //     pub mod champions {{
    //         {fmt}
    //     }}
    //     ",
    // );

    // println!("{module}");

    Ok(())
}

fn slice_repr<T: Debug>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|ident| format!("&{ident:#?}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn get_arg(len: usize, i: &usize) -> &dyn Display {
    match *i {
        i if i == 0 && i == len - 1 => &"unique",
        i if i == len - 1 => &"last",
        i if i == 0 => &"first",
        _ => i,
    }
}
