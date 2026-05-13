use crate::scripts::_batch::{Batch, get_aliases, get_arg, simplify, slice_repr};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_dev::{
    JsonRead, MayFail, decl_champions::Ability, gen_factories::wiki_champions::ChampionBuild,
    generators::gen_factories::wiki_champions::Champion,
};

pub fn generate_champions() -> MayFail<(String, Vec<(&'static str, String)>)> {
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

            let variant = format_args!("variant({champion_id})");

            let decl = format!(
                "
                #[derive(Clone, Debug, Deserialize, Serialize)]
                #[fmt(formula, keep, {variant})]
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

            if let Some(pos) = generator.find("impl") {
                generator.drain(..pos);
            }

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
                            ability,
                            replace(": Ability = Ability", " ="),
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
                        variable = function.to_uppercase(),
                        damage = simplify(damage)
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
            .iter()
            .map(|(champion_id, champion)| {
                let name = &champion.data.name;

                let alias = languages[champion_id]
                    .iter()
                    .chain(get_aliases(champion_id, name).iter())
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

    let fmt_args = vec![
        ("formula", champion_formulas),
        ("generator", champion_generator),
        ("ability", ability_formulas),
        ("closure", ability_closures),
    ];

    Ok((fmt, fmt_args))
}
