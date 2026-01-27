use crate::scripts::{
    BASIC_ATTACK, CRITICAL_STRIKE, ONHIT_EFFECT, StringExt, TOWER_DAMAGE,
    champions::generate_champions, items::generate_items, runes::generate_runes,
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};
use tutorlolv2_fmt::encode_brotli_11;

mod scripts;

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type GeneratorClosure = Box<dyn FnOnce(usize) -> Generated + Send + Sync + 'static>;
pub type GeneratorFn = MayFail<GeneratorClosure>;

const EVAL_FEAT: &str = r#"#[cfg(feature = "eval")]"#;
const GLOB_FEAT: &str = r#"#[cfg(feature = "glob")]"#;

/// Definition of what each generator function should return
pub struct Generated {
    /// Refers to what will be written to the `exports` folder,
    /// and that is useful for the frontend application to use.
    /// This is going to be concatenated with the same field for
    /// other generators, creating a very large String that will
    /// serve as a library to the frontend application
    pub exports: String,
    /// Every HTML that the generator function wants to export to
    /// the `BLOCK` of the frontend application, appearing when
    /// the user hovers over some object. It will be concatenated,
    /// compressed and exported to the frontend at the end
    pub block: String,
}

/// Definition of all available folders to write at, in the target directory
/// of the `exports`
pub enum SrcFolder {
    Champions,
    Items,
    Runes,
}

impl SrcFolder {
    /// Returns the name of the folder holding the generators of some
    /// definition of the enum [`SrcFolder`]
    pub fn gen_folder(self) -> impl AsRef<Path> {
        match self {
            SrcFolder::Champions => "gen_champions",
            SrcFolder::Items => "gen_items",
            SrcFolder::Runes => "gen_runes",
        }
    }
}

/// Helper struct that resolves the paths of the data that will be saved to files
pub struct CwdPath;

impl CwdPath {
    /// Create a new path in the working directory of the main project
    pub fn new(path: impl AsRef<Path>) -> PathBuf {
        Path::new("../").join(path)
    }

    /// Helper function that spawns a new thread to write some data to a file
    pub fn fwrite<P, D>(path: P, data: D) -> Result<(), std::io::Error>
    where
        P: AsRef<Path> + Debug,
        D: AsRef<[u8]>,
    {
        println!("[write] {path:?}");
        std::fs::write(CwdPath::new(path), data)
    }

    /// Checks if some path exists, and logs to the console the result
    pub fn exists(path: impl AsRef<Path> + Debug) -> bool {
        let result = std::fs::exists(&path);
        match result {
            Ok(true) => println!("[ok] {path:?}"),
            Ok(false) => println!("[null] {path:?}"),
            Err(ref e) => println!("[error] {path:?}: {e:?}"),
        }
        result.ok().unwrap_or_default()
    }

    /// Returns the HTML of some generator file that will be exported to the frontend,
    /// already formatted properly, and highlighted
    pub fn get_generator(gen_folder: SrcFolder, file: impl AsRef<Path>) -> MayFail<String> {
        let folder = gen_folder.gen_folder();
        let path = Self::new("tutorlolv2_dev/src/generators")
            .join(folder)
            .join(file)
            .with_extension("rs");

        #[cfg(debug_assertions)]
        Self::exists(&path);

        let data = std::fs::read_to_string(path)?;
        Ok(data.rust_fmt().rust_html())
    }

    /// Returns a `T` that represents a deserialized JSON file, from some `origin` path.
    /// The `.json` extension is added if it is missing in the provided path
    pub fn deserialize<T: DeserializeOwned>(origin: impl AsRef<Path>) -> MayFail<T> {
        let path = Self::new(origin).with_extension("json");

        #[cfg(debug_assertions)]
        Self::exists(&path);

        let bytes = std::fs::read(path)?;
        let data = serde_json::from_slice::<T>(&bytes)?;
        Ok(data)
    }
}

/// Adds some string to the end of each variable in the provided array of mutable
/// references
pub fn push_end<const N: usize>(variables: [&mut String; N], end: &str) {
    variables
        .into_iter()
        .for_each(|variable| variable.push_str(end));
}

pub fn parallel_task<P, F, T, R>(path: P, f: F) -> Vec<(String, R)>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
    F: Fn(&str, T) -> MayFail<R> + Send + Sync,
    R: Send,
{
    let folder = CwdPath::new(path);
    std::fs::read_dir(folder)
        .unwrap()
        .filter_map(Result::ok)
        .par_bridge()
        .into_par_iter()
        .map(|entry| {
            let path = entry.path();
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid file name")
                .unwrap();

            let data = std::fs::read(&path)
                .inspect_err(|e| eprintln!("Failed to read for {path:?}: {e:?}"))
                .unwrap();
            let json = serde_json::from_slice(&data)
                .inspect_err(|e| eprintln!("Failed to deserialize for {path:?}: {e:?}"))
                .unwrap();
            let result = f(name, json).unwrap();

            (name.to_owned(), result)
        })
        .collect::<Vec<_>>()
}

/// Provides functions to help track the current and new offsets of some
/// data inside a very large string
pub struct Tracker<'a> {
    inner: &'a mut String,
}

impl<'a> Tracker<'a> {
    /// Creates a new instance of self, from an existing string that
    /// should live longer than this struct
    pub const fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    /// Get the current length of the string, which represents
    /// the `end` offset of the last record
    pub const fn offset(&self) -> usize {
        self.inner.len()
    }

    /// Returns the start and end offsets of a new record `value`
    /// in the current tracked string. The `value` is added to that
    /// string and the offsets are adjusted properly
    pub fn record(&mut self, value: &str) -> (usize, usize) {
        let start = self.offset();
        self.inner.push_str(value);
        let end = self.offset();
        (start, end)
    }

    /// Adds a new record to the current tracked string, but adds
    /// the result tuple into the provided vector
    pub fn record_into(&mut self, value: &str, into: &mut Vec<(usize, usize)>) {
        let (start, end) = self.record(value);
        into.push((start, end));
    }
}

/// Entry point of the build library. Generates a new library that will be
/// used by both frontend and backend, as well as HTML that represents the
/// internal code that will be shown when hovering over some objects in the
/// frontend application
pub fn run() -> MayFail {
    let mut full_block = String::with_capacity(8 * 1024 * 1024);
    let mut full_exports = String::with_capacity(4 * 1024 * 1024);

    full_exports.push_str("use crate::*;");

    let closures = [generate_champions, generate_items, generate_runes]
        .into_par_iter()
        .map(|task| task().unwrap())
        .collect::<Vec<_>>();

    for closure in closures {
        let Generated { exports, block } = closure(full_block.len());
        full_block.push_str(&block);
        full_exports.push_str(&exports);
    }

    let mut tracker = Tracker::new(&mut full_block);

    for (name, value) in [
        ("ONHIT_EFFECT_OFFSET", ONHIT_EFFECT),
        ("BASIC_ATTACK_OFFSET", BASIC_ATTACK),
        ("TOWER_DAMAGE_OFFSET", TOWER_DAMAGE),
        ("CRITICAL_STRIKE_OFFSET", CRITICAL_STRIKE),
    ] {
        let (start, end) = tracker.record(&value.rust_html().as_const());
        full_exports.push_str(&format!(
            "{GLOB_FEAT} pub static {name}: Range<usize> = {start}..{end};"
        ));
    }

    let offset = tracker.offset();
    let compressed_block = encode_brotli_11(full_block.as_bytes());
    let size = compressed_block.len();

    full_exports.push_str(&format!(
        "{GLOB_FEAT} pub const BLOCK_SIZE: usize = {offset};"
    ));
    full_exports.push_str(&format!("{GLOB_FEAT} pub const BLOCK_LEN: usize = {size};"));
    full_exports.push_str("pub use champions::*; pub use items::*; pub use runes::*;");

    for task in [
        CwdPath::fwrite("tutorlolv2_gen/src/block.br", compressed_block),
        CwdPath::fwrite("tutorlolv2_gen/src/block.txt", full_block),
        CwdPath::fwrite("tutorlolv2_gen/src/data.rs", full_exports.rust_fmt()),
    ] {
        task?
    }

    Ok(())
}
