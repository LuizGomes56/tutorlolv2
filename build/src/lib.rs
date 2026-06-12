use crate::{
    generators::{
        impls::champions::{champion_gen_fn, champion_ids},
        parser::{Parser, items::ItemParser, runes::RuneParser},
    },
    model::champions::WikiChampion,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs::DirEntry,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

mod generators;
mod model;
mod scripts;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

fn enqueue<'a>(
    cmd48: &mut Command,
    cmd80: &mut Command,
    out_dir: impl AsRef<Path>,
    dir: &str,
    iter: &mut dyn Iterator<Item = &'a str>,
    target: &mut String,
) -> MayFail {
    for id in iter.into_iter() {
        let lower_id = id.to_lowercase();
        let out_path = out_dir.as_ref().join(dir).join(id);

        cmd48.arg(out_path.with_extension("w48"));
        cmd80.arg(out_path.with_extension("rs"));

        writeln!(
            target,
            r#"pub mod {lower_id} {{
                include!(concat!(env!("OUT_DIR"), "/{dir}/{id}.rs"));
            }}"#
        )?;
    }

    Ok(())
}

pub fn run() -> MayFail {
    println!("cargo:rerun-if-changed=build/src/generators/impls/champions");
    println!("cargo:rerun-if-changed=cache/wiki/champions");

    // let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let out_dir = PathBuf::from("build_output");

    let mut cmd48 = Command::new("rustfmt");
    cmd48.args(["--config", "max_width=48"]);

    let mut cmd80 = Command::new("rustfmt");

    let mut cmod = String::new();
    let mut imod = String::new();
    let mut rmod = String::new();

    let iparser = ItemParser::new()?;
    let rparser = RuneParser::new()?;

    for (dir, iter, target) in [
        (
            "champions",
            &mut champion_ids().iter().copied() as &mut dyn Iterator<Item = &str>,
            &mut cmod,
        ),
        (
            "items",
            &mut iparser.map().keys().map(String::as_str),
            &mut imod,
        ),
        (
            "runes",
            &mut rparser.map().keys().map(String::as_str),
            &mut rmod,
        ),
    ] {
        enqueue(&mut cmd48, &mut cmd80, &out_dir, dir, iter, target)?;
    }

    champion_ids().into_par_iter().try_for_each(|id| {
        let lower_id = id.to_lowercase();

        let wiki_path = format!("cache/wiki/champions/{id}/data.json");
        let impl_path = format!("build/src/generators/impls/champions/{lower_id}.rs");
        let out_path = out_dir.join("champions").join(id);

        let input = PathBuf::from(&impl_path);
        let output = PathBuf::from(&out_path.with_extension("rs"));

        if !needs_regeneration(&input, &output) {
            return Ok(());
        }

        let data = WikiChampion::from_file(&wiki_path)?;
        let function = champion_gen_fn(id).ok_or(format!(
            "[error] Failed to find generator function for {id}"
        ))?;

        function(data)?.call()?.build(&out_path)
    })?;

    iparser.map().keys().par_bridge().try_for_each(|id| {
        let out_path = out_dir.join("items").join(id);
        iparser.run_fn(id)?.build(out_path)
    })?;

    rparser.map().keys().par_bridge().try_for_each(|id| {
        let out_path = out_dir.join("runes").join(id);
        rparser.run_fn(id)?.build(out_path)
    })?;

    cmd48.status()?;
    cmd80.status()?;

    [
        ("champions.rs", cmod),
        ("items.rs", imod),
        ("runes.rs", rmod),
    ]
    .into_iter()
    .try_for_each(|(name, data)| -> MayFail { crate::write(out_dir.join(name), data) })
}

fn needs_regeneration(input: &Path, output: &Path) -> bool {
    let Ok(input_time) = input.metadata().and_then(|m| m.modified()) else {
        return true;
    };
    let output_time = output
        .metadata()
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    input_time > output_time
}

/// Custom trait that allows to deserialize a JSON instance
/// by providing only the file path and the desired type
pub trait JsonRead: DeserializeOwned {
    /// Receives a file path and deserializes the target JSON file into the
    /// struct that called this function as method.
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let data = read(path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    /// Stores the deserialized structs that were succesfully extracted from
    /// `.json` files inside the provided path, which should be a directory.
    /// Returns a [`HashMap`] whose keys are the file name, without the `.json`
    /// extension, and whose values are the deserialized structs. Note that all
    /// files inside the directory should have the same JSON structure, and if the
    /// deserialization fails for some file, it is skipped
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
