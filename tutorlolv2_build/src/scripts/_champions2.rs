use crate::Tracker;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use tutorlolv2_dev::{
    JsonRead, MayFail, gen_factories::wiki_champions::ChampionBuild,
    generators::gen_factories::wiki_champions::Champion,
};
use tutorlolv2_fmt::to_ssnake;

pub fn generate_champions(tracker: &mut Tracker<'_>) -> MayFail {
    std::env::set_current_dir("../")?;

    let data = BTreeMap::<String, Champion>::from_file("internal/champions.json")?;
    let languages =
        BTreeMap::<String, BTreeSet<String>>::from_file("internal/champion_languages.json")?;

    let champion_id_enum = format!(
        "impl ChampionId {{
            pub const VARIANTS: usize = {len};
            pub const fn debug(&self) -> &'static str {{
                match self {{{arms}}}
            }}
        }}
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr(u8)]
        pub enum ChampionId {{{variants}}}",
        len = data.len(),
        variants = data
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(","),
        arms = data
            .keys()
            .map(|name| format!("Self::{name} => {name:?},"))
            .collect::<String>()
    );

    let [
        mut champion_cache,
        mut champion_formulas,
        mut champion_generator,
        mut ability_formulas,
        mut ability_closures,
    ] = core::array::from_fn(|i| {
        let (name, vtype) = [
            ("CHAMPION_CACHE", "&Champion"),
            ("CHAMPION_FORMULAS", "Range<usize>"),
            ("CHAMPION_GENERATOR", "Range<usize>"),
            ("ABILITY_FORMULAS", "&[&[Range<usize>]]"),
            ("ABILITY_CLOSURES", "&[&[Range<usize>]]"),
        ][i];
        format!("pub static {name}: [{vtype}; ChampionId::VARIANTS] = [")
    });

    let mut const_eval = String::from(
        "pub const fn ability_const_eval(
            ctx: &Ctx,
            champion_id: ChampionId,
            kind: AbilityId
        ) -> f32 {
            match champion_id {",
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

    let result = data
        .into_iter()
        .map(|(champion_id, champion)| {
            let Champion {
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

            let upper_id = champion_id.to_uppercase();

            champion_cache.push_str(&format!("&{upper_id},"));

            let decl = format!(
                "
            #[derive(Clone, Debug, Deserialize, Serialize)]
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
            }};",
                combos = slice_repr(&combos),
                identifiers = slice_repr(&identifiers),
                fn_names = functions.join(",")
            );

            champion_formulas.push_str(&tracker.record_range(&decl));

            let generator = tutorlolv2_dev::read_to_string(format!(
                "tutorlolv2_dev/src/generators/gen_champions/{champion_id}.rs"
            ))
            .unwrap();

            let generator_html = tutorlolv2_fmt::rust_html(&generator);

            champion_generator.push_str(&tracker.record_range(&generator_html));

            const_eval.push_str(&format!(
                r#"ChampionId::{champion_id} => {{
                    match kind {{
                        {arms}
                        _ => panic!("Invalid AbilityId provided for '{champion_id}'"),
                    }}
                }},"#,
                arms = functions
                    .iter()
                    .zip(&metadata)
                    .map(|(function, metadata)| {
                        let ability_id = metadata.kind;
                        format!("{ability_id:?} => {function}(ctx),")
                    })
                    .collect::<String>()
            ));

            let fn_closures = functions
                .iter()
                .zip(&closures)
                .map(|(function, body)| {
                    format!("pub const fn {function}(ctx: &Ctx) -> f32 {{{body}}}")
                })
                .collect::<String>();

            decl + &fn_closures
        })
        .collect::<String>();

    const_eval.push_str("}}");
    champion_cache.push_str("];");
    champion_generator.push_str("];");
    champion_formulas.push_str("];");

    let fmt = tutorlolv2_fmt::rustfmt(
        &[
            result,
            const_eval,
            champion_id_enum,
            champion_name_to_id,
            champion_cache,
            champion_formulas,
            champion_generator,
        ]
        .concat(),
        None,
    );

    println!("{fmt}");

    Ok(())
}

fn slice_repr<T: Debug>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|ident| format!("&{ident:#?}"))
        .collect::<Vec<_>>()
        .join(",")
}
