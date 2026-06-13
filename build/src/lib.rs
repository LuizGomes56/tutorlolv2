use crate::{
    generators::{
        parser::{
            MapValueExt, Parser, champions::ChampionParser, items::ItemParser, runes::RuneParser,
        },
        utils::Tag,
    },
    scripts::{
        batch::{FmtOutput, Tracker, batch},
        consts::*,
        finish::{finish_champions, finish_items_or_runes},
        utils::{StaticVar, static_vars},
    },
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs::DirEntry,
    path::{Path, PathBuf},
    process::Command,
};
use tutorlolv2_fmt::{rust_html, to_ssnake};

mod generators;
mod model;
mod scripts;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

pub trait Build {
    fn build(&mut self, out_path: PathBuf) -> MayFail;
}

fn write_module<'a>(
    cmd48: &mut Command,
    cmd80: &mut Command,
    out_dir: impl AsRef<Path>,
    dir: &str,
    iter: &mut dyn Iterator<Item = &'a str>,
) -> MayFail {
    let path = out_dir.as_ref();
    let out = path.join(dir);
    let mut result = String::new();

    cmd80.arg(path.join(format!("{dir}_code")).with_extension("rs"));

    for id in iter.into_iter() {
        let lower_id = to_ssnake(id).to_lowercase();
        let out_path = out.join(id);

        cmd48.arg(out_path.with_extension("w48"));
        cmd80.arg(out_path.with_extension("rs"));

        writeln!(
            result,
            r#"pub mod {lower_id} {{
                include!(concat!(env!("OUT_DIR"), "/{dir}/{id}.rs"));
            }}"#
        )?;
    }

    crate::write(out.with_extension("rs"), result)
}

pub fn run() -> MayFail {
    println!("cargo:rerun-if-changed=build/src/generators/impls/champions");
    println!("cargo:rerun-if-changed=cache/wiki/champions");

    // let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let out_dir = PathBuf::from("build_output");

    let mut cmd48 = Command::new("rustfmt");
    cmd48.args(["--config", "max_width=48"]);

    let mut cmd80 = Command::new("rustfmt");
    cmd80.arg(out_dir.join("docs.rs"));

    let cparser = ChampionParser::new()?;
    let iparser = ItemParser::new()?;
    let rparser = RuneParser::new()?;

    for (dir, iter) in [
        (
            "champions",
            &mut cparser.keys() as &mut dyn Iterator<Item = &str>,
        ),
        ("items", &mut iparser.keys()),
        ("runes", &mut rparser.keys()),
    ] {
        write_module(&mut cmd48, &mut cmd80, &out_dir, dir, iter)?;
    }

    let mut c_result = MayFail::Ok(());
    let mut i_result = MayFail::Ok(());
    let mut r_result = MayFail::Ok(());

    fn build_code<
        T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
        U: Build + TryFrom<T, Error = DynError> + Serialize,
    >(
        parser: &impl Parser<T, U>,
        out_dir: impl AsRef<Path>,
        f: impl Fn() -> MayFail<Option<BTreeMap<String, Vec<String>>>>,
    ) -> MayFail {
        || -> MayFail {
            let out = out_dir.as_ref();
            let tag = parser.tag().plural();
            parser.keys().par_bridge().try_for_each(|id| {
                let out_path = out.join(tag).join(id);
                parser.run_fn(id)?.build(out_path)
            })?;

            write(
                out.join(format!("{tag}_code")).with_extension("rs"),
                [parser.id_enum(), parser.phf(f()?), parser.data_variable()].concat(),
            )
        }()
    }

    rayon::scope(|s| {
        s.spawn(|_| {
            c_result = build_code(&cparser, &out_dir, || {
                let map = cparser.map();
                let languages = BTreeMap::<String, BTreeSet<String>>::from_file(
                    "internal/champion_languages.json",
                )?;

                let alias = map
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

                Ok(Some(alias))
            });
        });
        s.spawn(|_| {
            i_result = build_code(&iparser, &out_dir, || Ok(None));
        });
        s.spawn(|_| {
            r_result = build_code(&rparser, &out_dir, || Ok(None));
        });
    });

    c_result?;
    i_result?;
    r_result?;

    /*
    * pub fn get_const_eval(data: &BTreeMap<&String, Batch>, tag: Tag) -> String {
        format!(
            "
             pub const fn {ltag}_const_eval(
                 ctx: &Ctx,
                 {ltag}_id: {tag:?}Id,
                 attack_type: AttackType
             ) -> [f32; 2] {{
                 match {ltag}_id {{{eval}}}
             }}
             ",
            ltag = tag.singular().to_lowercase(),
            eval = data
                .values()
                .map(|batch| batch.eval.as_str())
                .collect::<Vec<&str>>()
                .concat()
        )
    }
    */

    cmd48.status()?;

    build_docs(out_dir)?;

    cmd80.status()?;

    Ok(())
}

fn build_docs(out_dir: PathBuf) -> MayFail {
    let full = [Tag::Champions, Tag::Items, Tag::Runes]
        .into_par_iter()
        .map(|dir| {
            read_dir(out_dir.join(format!("{}s", dir.singular())))
                .map(|r| {
                    r.filter(|entry| entry.path().extension().map_or(false, |e| e == "w48"))
                        .par_bridge()
                        .into_par_iter()
                        .map(|entry| read_to_string(entry.path()))
                        .collect::<MayFail<String>>()
                })
                .flatten()
                .and_then(|s| Ok((dir, s)))
        })
        .collect::<MayFail<BTreeMap<_, _>>>()?;

    let mut full_block = String::with_capacity(12 * 1024 * 1024);
    let mut exports = String::with_capacity(4 * 1024 * 1024);
    let mut tracker = Tracker::new(&mut full_block);

    for (name, value) in [
        ("IGNITE_OFFSET", IGNITE_FN),
        ("ONHIT_EFFECT_OFFSET", ONHIT_EFFECT),
        ("BASIC_ATTACK_OFFSET", BASIC_ATTACK),
        ("TOWER_DAMAGE_OFFSET", TOWER_DAMAGE),
        ("CRITICAL_STRIKE_OFFSET", CRITICAL_STRIKE),
        ("ONHIT_EFFECT_FN_OFFSET", ONHIT_EFFECT_FN),
        ("TOWER_DAMAGE_FN_OFFSET", TOWER_DAMAGE_FN),
        ("BASIC_ATTACK_FN_OFFSET", BASIC_ATTACK_FN),
        ("CRITICAL_STRIKE_FN_OFFSET", CRITICAL_STRIKE_FN),
    ] {
        let range = tracker.push(&rust_html(value));
        writeln!(exports, "pub static {name}: Range<usize> = {range:?};")?;
    }

    for (dir, src) in full {
        let mut batch = batch(&src);
        tracker.batch(&mut batch);

        let mut fmt_args = match dir {
            Tag::Champions => ChampionParser::static_vars([
                StaticVar {
                    attribute: "formula",
                    name: "CHAMPION_FORMULAS".into(),
                    vtype: "Range<usize>",
                },
                StaticVar {
                    attribute: "generator",
                    name: "CHAMPION_GENERATOR".into(),
                    vtype: "Range<usize>",
                },
                StaticVar {
                    attribute: "ability",
                    name: "ABILITY_FORMULAS".into(),
                    vtype: "&[Range<usize>]",
                },
                StaticVar {
                    attribute: "closure",
                    name: "ABILITY_CLOSURES".into(),
                    vtype: "&[Range<usize>]",
                },
            ]),
            _ => {
                let var = |postfix| format!("{}_{postfix}", dir.singular().to_uppercase());

                static_vars(
                    dir,
                    [
                        StaticVar {
                            attribute: "formula",
                            name: var("FORMULAS"),
                            vtype: "Range<usize>",
                        },
                        StaticVar {
                            attribute: "generator",
                            name: var("GENERATOR"),
                            vtype: "Range<usize>",
                        },
                        StaticVar {
                            attribute: "closure",
                            name: var("CLOSURES"),
                            vtype: "[[Range<usize>; 2]; 2]",
                        },
                    ],
                )
            }
        };

        let finish = match dir {
            Tag::Champions => finish_champions,
            Tag::Items | Tag::Runes => finish_items_or_runes,
        } as fn(&str, &mut String, &mut [FmtOutput]);

        for values in batch.values_mut() {
            for (target, outputs) in values.iter_mut() {
                if let Some(variable) = fmt_args.get_mut(target) {
                    finish(target, variable, outputs);
                }
            }
        }

        let docs = fmt_args
            .values_mut()
            .map(|variable| {
                variable.push_str("];");
                variable.as_str()
            })
            .collect::<String>();

        exports.push_str(&docs);
    }

    write(out_dir.join("docs").with_extension("txt"), full_block)?;
    write(out_dir.join("docs").with_extension("rs"), exports)
}

pub trait JsonRead: DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let data = read(path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    fn from_dir(path: impl AsRef<Path>) -> MayFail<BTreeMap<String, Self>> {
        Ok(read_dir(&path)?
            .into_iter()
            .filter_map(|entry| {
                let entry_name = entry.file_name().to_string_lossy().into_owned();
                let file_name = entry_name
                    .strip_suffix(".json")
                    .unwrap_or(&entry_name)
                    .to_string();

                let data =
                    Self::from_file(path.as_ref().join(&file_name).with_extension("json")).ok()?;
                Some((file_name, data))
            })
            .collect::<BTreeMap<String, Self>>())
    }
}

pub trait JsonWrite: Serialize {
    fn into_file(&self, path: impl AsRef<Path>) -> MayFail {
        let path = path.as_ref();
        println!("[write] {path:?}");

        let data = serde_json::to_string_pretty(self)?;
        Ok(write(path, data.as_bytes())?)
    }
}

impl<T> JsonRead for T where T: DeserializeOwned {}
impl<T> JsonWrite for T where T: Serialize {}

pub fn write(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> MayFail {
    let path = path.as_ref();

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        create_dir_all(parent)?;
    }

    std::fs::write(path, data)
        .map_err(|e| format!("[write] Error writing file: {path:?}: {e:?}").into())
}

pub fn read(path: impl AsRef<Path>) -> MayFail<Vec<u8>> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|e| format!("[read] Error reading file: {path:?}: {e:?}").into())
}

pub fn read_to_string(path: impl AsRef<Path>) -> MayFail<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path)
        .map_err(|e| format!("[read] Error reading file: {path:?}: {e:?}").into())
}

pub fn read_dir(path: impl AsRef<Path>) -> MayFail<impl Iterator<Item = DirEntry>> {
    let path = path.as_ref();
    Ok(std::fs::read_dir(path)
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok))
}

pub fn remove_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Err(e) = std::fs::remove_file(path)
        && !e.kind().eq(&std::io::ErrorKind::NotFound)
    {
        println!("[remove_file] Error removing file: {path:?}: {e:?}");
    }
}

pub fn create_dir_all(path: impl AsRef<Path>) -> MayFail {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .map_err(|e| format!("[create_dir_all] Error creating directory: {path:?}: {e:?}").into())
}
