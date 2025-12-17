use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{sync::Semaphore, task::JoinHandle};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};
use tutorlolv2_fmt::encode_brotli_11;

mod scripts;

use crate::scripts::{
    BASIC_ATTACK, CRITICAL_STRIKE, ONHIT_EFFECT, StringExt, TOWER_DAMAGE,
    champions::generate_champions, items::generate_items, runes::generate_runes,
};

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub type GeneratorClosure = Box<dyn FnOnce(usize) -> Generated + Send + Sync + 'static>;
pub type GeneratorFn = MayFail<GeneratorClosure>;

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
    #[cfg(debug_assertions)]
    const BASE_PATH: &str = "__build_dbg";
    #[cfg(not(debug_assertions))]
    const BASE_PATH: &str = "tutorlolv2_gen/src/data";

    /// Creates a new resolved path to write at the library that
    /// holds the generated data
    pub fn new(path: impl AsRef<Path>) -> PathBuf {
        Path::new(Self::BASE_PATH).join(path)
    }

    /// Gets a path that can be read to determine to where the
    /// data should be saved at
    pub fn import_file(self) -> PathBuf {
        let base = Path::new(Self::BASE_PATH);
        let file = match self {
            Self::Champions => base.join("champions"),
            Self::Items => base.join("items"),
            Self::Runes => base.join("runes"),
        };
        file.with_extension("rs")
    }

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
    pub fn fwrite<P, D>(path: P, data: D) -> JoinHandle<Result<(), std::io::Error>>
    where
        P: AsRef<Path> + Debug + Send + Sync + 'static,
        D: AsRef<[u8]> + Send + Sync + 'static,
    {
        println!("[write] {path:?}");
        tokio::spawn(tokio::fs::write(CwdPath::new(path), data))
    }

    /// Checks if some path exists, and logs to the console the result
    pub async fn exists(path: impl AsRef<Path> + Debug) -> bool {
        let result = tokio::fs::try_exists(&path).await;
        match result {
            Ok(true) => println!("[ok] {path:?}"),
            Ok(false) => println!("[null] {path:?}"),
            Err(ref e) => println!("[error] {path:?}: {e:?}"),
        }
        result.ok().unwrap_or_default()
    }

    /// Returns the HTML of some generator file that will be exported to the frontend,
    /// already formatted properly, and highlighted
    pub async fn get_generator(gen_folder: SrcFolder, file: impl AsRef<Path>) -> MayFail<String> {
        let folder = gen_folder.gen_folder();
        let path = Self::new("tutorlolv2_dev/src/generators")
            .join(folder)
            .join(file)
            .with_extension("rs");

        #[cfg(debug_assertions)]
        Self::exists(&path).await;

        let data = tokio::fs::read_to_string(path).await?;
        Ok(data.rust_fmt().rust_html())
    }

    /// Returns a `T` that represents a deserialized JSON file, from some `origin` path.
    /// The `.json` extension is added if it is missing in the provided path
    pub async fn deserialize<T: DeserializeOwned>(origin: impl AsRef<Path>) -> MayFail<T> {
        let path = Self::new(origin).with_extension("json");

        #[cfg(debug_assertions)]
        Self::exists(&path).await;

        let bytes = tokio::fs::read(path).await?;
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

/// Begins the execution of every task in parallel, setting a maximum number of
/// tasks that may run concurrently or in parallel.
/// - `permits`: The maximum number of tasks that may run at the same timem
/// - `path`: Path to the folder where the target files are located. They must
/// be `.json` files. Note that every file in that folder should have the same
/// JSON structure, meaning that they will all be deserialized to the same struct
/// - `f`: An async closure that takes in as arguments the name of a file located
/// in the `path` folder, and the deserialized data type of that file, and returns
/// some data that will be inferred by the compiler, and used in the `R` function
/// - `r`: A function that decides what to do with the [`JoinHandle`]'s outputs. In
/// other words, it defines if all tasks will be collected to a [`Vec`],
/// [`std::collections::BTreeMap`], or [`std::collections::HashMap`].
///
/// ```rs
/// let data = parallel_task(
///     128,
///     "internal/champions",
///     async |champion_id, champion: Champion| {
///         // Do some work and return a `MayFail` of some type
///         ..
///     },
///     async |futures| {
///         // In this case, it collects the results to a `BTreeMap`
///         let mut result = BTreeMap::new();
///         for future in futures {
///             let (file_name, data) = future.await?;
///             result.insert(file_name, data);
///         }
///         Ok(result)
///     },
/// ).await?;
/// ```
pub async fn parallel_task<_Path, _FnIn, _Type, _Resp, _Data, _Retn, _Fut1, _Fut2>(
    permits: usize,
    path: _Path,
    f: _FnIn,
    r: _Resp,
) -> MayFail<_Retn>
where
    _Path: AsRef<Path>,
    _Data: Send + Sync + 'static,
    _Fut2: Future<Output = MayFail<_Retn>> + Send + Sync + 'static,
    _Resp: Fn(Vec<JoinHandle<(String, _Data)>>) -> _Fut2,
    _Type: DeserializeOwned,
    _Fut1: Future<Output = MayFail<_Data>> + Send + Sync + 'static,
    _FnIn: Clone + Send + Sync + 'static + Fn(String, _Type) -> _Fut1,
{
    let mut futures = Vec::new();
    let semaphore = Arc::new(Semaphore::new(permits));

    let folder = CwdPath::new(path);
    let read_dir = tokio::fs::read_dir(folder).await?;
    let mut dir_stream = ReadDirStream::new(read_dir);

    while let Some(Ok(entry)) = dir_stream.next().await {
        let f = f.clone();
        let semaphore = semaphore.clone();
        futures.push(tokio::task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let file_name = entry
                .file_name()
                .to_string_lossy()
                .to_string()
                .split(".")
                .next()
                .ok_or(format!("Failed to get file name for entry: {entry:?}"))
                .unwrap()
                .to_string();
            let handler = {
                let file_name = file_name.clone();
                async move || -> MayFail<_Data> {
                    let data = tokio::fs::read(entry.path()).await?;
                    let json = serde_json::from_slice(&data)?;
                    f(file_name, json).await
                }
            };

            (file_name, handler().await.unwrap())
        }))
    }

    r(futures).await
}

/// Spawns some task, given the function and panics on that thread if it fails
/// ```rs
/// let data = gen_task(generate_champions).await?;
/// ```
fn gen_task<F, Fut>(f: F) -> JoinHandle<GeneratorClosure>
where
    Fut: Future<Output = GeneratorFn> + Send + Sync,
    F: Fn() -> Fut + Send + Sync + 'static,
{
    tokio::task::spawn(async move {
        match f().await {
            Ok(value) => value,
            Err(e) => panic!("Error on future [gen]: {e:?}"),
        }
    })
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
pub async fn run() -> MayFail {
    let mut full_block = String::with_capacity(8 * 1024 * 1024);
    let mut full_exports = String::from(
        r#"use crate::*;
        #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Position {
            Top,
            Jungle,
            Middle,
            Bottom,
            Support,
        }"#,
    );
    let mut closures = Vec::new();

    for task in [
        gen_task(generate_champions),
        gen_task(generate_items),
        gen_task(generate_runes),
    ] {
        closures.push(task.await?);
    }

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
        let (start, end) = tracker.record(&value.rust_html());
        full_exports.push_str(&format!(
            "pub static {name}: (u32, u32) = ({start}, {end});"
        ));
    }

    let offset = tracker.offset();
    let compressed_block = encode_brotli_11(full_block.as_bytes());

    full_exports.push_str(&format!("pub const BLOCK_SIZE: usize = {offset};"));
    full_exports.push_str(&format!(
        "pub const BLOCK: [u8; {size}] = [{bytes}];",
        size = compressed_block.len(),
        bytes = compressed_block
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    ));

    for task in [
        CwdPath::fwrite("tutorlolv2_exports/src/block.txt", full_block),
        CwdPath::fwrite("tutorlolv2_exports/src/exports.rs", full_exports.rust_fmt()),
    ] {
        task.await??
    }

    Ok(())
}
