use crate::{
    generators::parser::{
        Parser, StaticVar, champions::ChampionParser, items::ItemParser, runes::RuneParser,
    },
    scripts::{
        batch::{FmtOutput, Tracker, batch},
        consts::*,
    },
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs::DirEntry,
    ops::Range,
    path::{Path, PathBuf},
    process::Command,
};
use tutorlolv2_fmt::{rust_html, to_ssnake};
use tutorlolv2_types::AbilityId;

mod generators;
mod model;
mod scripts;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

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

    rayon::scope(|s| {
        s.spawn(|_| {
            let map = cparser.map();

            c_result = || -> MayFail {
                cparser.keys().par_bridge().try_for_each(|id| {
                    let out_path = out_dir.join("champions").join(id);
                    cparser.run_fn(id)?.build(out_path)
                })?;

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

                write(
                    out_dir.join("champions_code").with_extension("rs"),
                    [
                        cparser.id_enum(),
                        cparser.phf(Some(alias)),
                        cparser.data_variable(),
                    ]
                    .concat(),
                )
            }();
        });
        s.spawn(|_| {
            i_result = || -> MayFail {
                iparser.keys().par_bridge().try_for_each(|id| {
                    let out_path = out_dir.join("items").join(id);
                    iparser.run_fn(id)?.build(out_path)
                })?;

                write(
                    out_dir.join("items_code").with_extension("rs"),
                    [
                        iparser.id_enum(),
                        iparser.phf(None),
                        iparser.data_variable(),
                    ]
                    .concat(),
                )
            }();
        });
        s.spawn(|_| {
            r_result = || -> MayFail {
                rparser.keys().par_bridge().try_for_each(|id| {
                    let out_path = out_dir.join("runes").join(id);
                    rparser.run_fn(id)?.build(out_path)
                })?;

                write(
                    out_dir.join("runes_code").with_extension("rs"),
                    [
                        rparser.id_enum(),
                        rparser.phf(None),
                        rparser.data_variable(),
                    ]
                    .concat(),
                )
            }();
        });
    });

    c_result?;
    i_result?;
    r_result?;

    cmd48.status()?;
    cmd80.status()?;

    build_docs(out_dir)
}

fn build_docs(out_dir: PathBuf) -> MayFail {
    pub static mut ZERO_FN_OFFSET: Range<usize> = 0..0;
    pub static mut DEFAULT_ITEM_GENERATOR_OFFSET: Range<usize> = 0..0;

    let full = ["champions", "items", "runes"]
        .into_par_iter()
        .map(|dir| {
            read_dir(out_dir.join(dir))
                .map(|r| {
                    r.filter(|entry| entry.path().extension().map_or(false, |e| e == "w48"))
                        .par_bridge()
                        .into_par_iter()
                        .map(|entry| read_to_string(entry.path()))
                        .collect::<MayFail<String>>()
                })
                .flatten()
        })
        .collect::<MayFail<Vec<String>>>()?
        .concat();

    let mut full_block = String::with_capacity(12 * 1024 * 1024);
    let mut exports = String::with_capacity(4 * 1024 * 1024);
    let mut tracker = Tracker::new(&mut full_block);

    unsafe {
        DEFAULT_ITEM_GENERATOR_OFFSET = tracker.push(&rust_html(DEFAULT_ITEM_GENERATOR));
        ZERO_FN_OFFSET = tracker.push(&rust_html(ZERO_FN));
    }

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

    let mut batch = batch(&full);

    let mut fmt_args = ChampionParser::static_vars([
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
    ]);

    for values in batch.values_mut() {
        for (target, outputs) in values.iter_mut() {
            let variable = fmt_args.get_mut(target).unwrap();
            cfinish(target, variable, outputs);
        }
    }

    pub fn cfinish(target: &str, variable: &mut String, value: &mut [FmtOutput<'_>]) {
        value.sort_by(|a, b| match &a.json.meta {
            v if let Ok(ability_a) = serde_json::from_value::<AbilityId>(v.clone())
                && let Ok(ability_b) = serde_json::from_value::<AbilityId>(b.json.meta.clone()) =>
            {
                ability_a.cmp(&ability_b)
            }
            _ => a.json.target.cmp(&b.json.target),
        });

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
