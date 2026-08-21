use {
    crate::{
        generators::{
            parser::{
                MapValueExt, Parser, champions::ChampionParser, items::ItemParser,
                runes::RuneParser,
            },
            utils::Tag,
        },
        scripts::{
            batch::{FmtArgs, batch, pack_formulas},
            finish::{champion_aliases, eval_abilities, eval_items_or_runes},
        },
    },
    heck::ToSnakeCase,
    rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator},
    serde::{Serialize, de::DeserializeOwned},
    serde_json::json,
    std::{
        collections::BTreeMap, fmt::Write, path::PathBuf, process::Command, str::FromStr,
        sync::LazyLock,
    },
    tutorlolv2_codec::FormulaDbBuilder,
    tutorlolv2_types::CtxVar,
};

use tutorlolv2_codec::{EntityKind, GeneratorDbBuilder};
pub use tutorlolv2_wiki::{DynError, MayFail};

pub mod generators;
pub mod scripts;

pub trait Build {
    fn build(&mut self) -> MayFail<String>;
}

fn write_module<'a>(
    cmd48: &mut Command,
    cmd80: &mut Command,
    dir: &str,
    iter: &mut dyn Iterator<Item = &'a str>,
) -> MayFail {
    let out = PathBuf::from(dir);
    let mut result = String::new();

    cmd80.arg(PathBuf::from(format!("{dir}_code")).with_extension("rs"));

    for id in iter.into_iter() {
        let module = id.to_snake_case();
        let out_path = out.join(id);

        cmd48.arg(out_path.with_extension("w48"));
        cmd80.arg(out_path.with_extension("rs"));

        writeln!(
            result,
            r#"pub mod {module} {{
                use super::*;
                include!(concat!(env!("OUT_DIR"), "/{dir}/{id}.rs"));
            }}"#
        )?;
    }

    tutorlolv2_wiki::write(OUT_DIR.join(dir).with_extension("rs"), result)
}

pub static CPARSER: LazyLock<ChampionParser> = LazyLock::new(|| ChampionParser::new().unwrap());
pub static IPARSER: LazyLock<ItemParser> = LazyLock::new(|| ItemParser::new().unwrap());
pub static RPARSER: LazyLock<RuneParser> = LazyLock::new(|| RuneParser::new().unwrap());
pub static OUT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));

pub fn run() -> MayFail {
    println!("cargo:rerun-if-changed=build/src/generators/impls/champions");
    println!("cargo:rerun-if-changed=build/src/generators/impls/items");
    println!("cargo:rerun-if-changed=build/src/generators/impls/runes");
    println!("cargo:rerun-if-changed=cache/wiki/champions");
    println!("cargo:rerun-if-changed=cache/wiki/items");
    println!("cargo:rerun-if-changed=cache/wiki/runes");

    rayon::join(
        || LazyLock::force(&CPARSER),
        || rayon::join(|| LazyLock::force(&IPARSER), || LazyLock::force(&RPARSER)),
    );

    let mut cmd48 = Command::new("rustfmt");
    cmd48
        .current_dir(&*OUT_DIR)
        .args(["--config", "max_width=48"]);

    let mut cmd80 = Command::new("rustfmt");
    cmd80.current_dir(&*OUT_DIR).arg(PathBuf::from("docs.rs"));

    for (dir, iter) in [
        (
            "champions",
            &mut CPARSER.keys() as &mut dyn Iterator<Item = &str>,
        ),
        ("items", &mut IPARSER.keys()),
        ("runes", &mut RPARSER.keys()),
    ] {
        write_module(&mut cmd48, &mut cmd80, dir, iter)?;
    }

    let mut c_result = Ok(());
    let mut i_result = Ok(());
    let mut r_result = Ok(());

    fn build_code<
        T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
        U: Build + Serialize + TryFrom<T, Error = DynError>,
    >(
        parser: &impl Parser<T, U>,
        phf_fn: impl Fn() -> MayFail<Option<BTreeMap<String, Vec<String>>>>,
        eval_fn: impl Fn(&str, Tag) -> String,
    ) -> MayFail {
        let tag = parser.tag();
        let plural = tag.plural();

        let eval_arms = parser
            .keys()
            .par_bridge()
            .map(|id| parser.run_fn(id)?.build())
            .collect::<MayFail<String>>()?;

        tutorlolv2_wiki::write(
            OUT_DIR.join(format!("{plural}_code")).with_extension("rs"),
            [
                parser.id_enum(),
                parser.phf(phf_fn()?),
                parser.data_variable(),
                eval_fn(&eval_arms, tag),
            ]
            .concat(),
        )
    }

    rayon::scope(|s| {
        s.spawn(|_| {
            c_result = build_code(&*CPARSER, champion_aliases, eval_abilities);
        });
        s.spawn(|_| {
            i_result = build_code(&*IPARSER, || Ok(None), eval_items_or_runes);
        });
        s.spawn(|_| {
            r_result = build_code(&*RPARSER, || Ok(None), eval_items_or_runes);
        });
    });

    c_result?;
    i_result?;
    r_result?;

    cmd48.status()?;

    build_docs()?;

    cmd80.status()?;

    Ok(())
}

fn build_docs() -> MayFail {
    let mut batches = [Tag::Champions, Tag::Items, Tag::Runes]
        .into_par_iter()
        .map(|tag| -> MayFail<_> {
            let mut src = tutorlolv2_wiki::read_dir(OUT_DIR.join(tag.plural()))?
                .filter(|entry| entry.path().extension().map_or(false, |e| e == "w48"))
                .par_bridge()
                .map(|entry| tutorlolv2_wiki::read_to_string(entry.path()))
                .flatten()
                .collect::<String>();

            let iter = match tag {
                Tag::Champions => &mut CPARSER.keys() as &mut (dyn Iterator<Item = &str> + Send),
                Tag::Items => &mut IPARSER.keys(),
                Tag::Runes => &mut RPARSER.keys(),
            };

            let generators = iter
                .into_iter()
                .enumerate()
                .par_bridge()
                .map(|(i, variant)| {
                    let file_name = variant.to_snake_case();
                    let mut default = false;

                    let mut generator = tutorlolv2_wiki::read_to_string(format!(
                        "build/src/generators/impls/{}/{file_name}.rs",
                        tag.plural()
                    ))
                    .unwrap_or_else(|_| {
                        default = true;
                        "impl Generator {}".into()
                    });

                    if let Some(pos) = generator.find("impl") {
                        generator.drain(..pos);
                    }

                    generator.insert_str(
                        0,
                        &format!(
                            "#[fmt({})]",
                            json!(FmtArgs {
                                target: "generator".into(),
                                variant: variant.into(),
                                meta: (tag as u8, i as u16),
                                default
                            })
                        ),
                    );
                    generator
                })
                .collect::<String>();

            src.push_str(&generators);

            Ok((tag, batch(src)))
        })
        .collect::<MayFail<Vec<_>>>()?;

    let mut generators = GeneratorDbBuilder::new(
        CPARSER.map().len() as _,
        IPARSER.map().len() as _,
        RPARSER.map().len() as _,
        &[],
    );
    let mut formulas = FormulaDbBuilder::new(
        CPARSER.map().len() as _,
        IPARSER.map().len() as _,
        RPARSER.map().len() as _,
        |s| CtxVar::from_str(s).ok().map(|v| v as u8),
    );

    for (tag, batch) in &mut batches {
        pack_formulas(&mut formulas, *tag, batch)?;

        for inner in batch.values_mut() {
            for outputs in inner.values_mut() {
                for output in outputs {
                    if output.json.default || output.json.target != "generator" {
                        continue;
                    }

                    let (kind, index) =
                        serde_json::from_value::<(u8, u16)>(output.json.meta.clone())?;

                    generators.push(EntityKind::from_repr(kind).unwrap(), index, &output.block)?;
                }
            }
        }
    }

    tutorlolv2_wiki::write(
        OUT_DIR.join("formulas").with_extension("bin"),
        formulas.finish()?,
    )?;

    tutorlolv2_wiki::write(
        OUT_DIR.join("generator").with_extension("bin"),
        generators.finish_with_stats()?.bytes,
    )
}
