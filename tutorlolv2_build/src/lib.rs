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

use crate::scripts::{BASIC_ATTACK, CRITICAL_STRIKE, ONHIT_EFFECT, StringExt, TOWER_DAMAGE};
use scripts::{champions::generate_champions, items::generate_items, runes::generate_runes};

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub type GeneratorClosure = Box<dyn FnOnce(usize) -> Generated + Send + Sync + 'static>;
pub type GeneratorFn = MayFail<GeneratorClosure>;

pub struct Generated {
    pub exports: String,
    pub block: String,
}

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

    pub fn new(path: impl AsRef<Path>) -> PathBuf {
        Path::new(Self::BASE_PATH).join(path)
    }

    pub fn import_file(self) -> PathBuf {
        let base = Path::new(Self::BASE_PATH);
        let file = match self {
            Self::Champions => base.join("champions"),
            Self::Items => base.join("items"),
            Self::Runes => base.join("runes"),
        };
        file.with_extension("rs")
    }

    pub fn gen_folder(self) -> impl AsRef<Path> {
        match self {
            SrcFolder::Champions => "gen_champions",
            SrcFolder::Items => "gen_items",
            SrcFolder::Runes => "gen_runes",
        }
    }
}

pub struct CwdPath;

impl CwdPath {
    pub fn new(path: impl AsRef<Path>) -> PathBuf {
        Path::new("../").join(path)
    }

    pub fn fwrite<P, D>(path: P, data: D) -> JoinHandle<Result<(), std::io::Error>>
    where
        P: AsRef<Path> + Debug + Send + Sync + 'static,
        D: AsRef<[u8]> + Send + Sync + 'static,
    {
        println!("[write] {path:?}");
        tokio::spawn(tokio::fs::write(CwdPath::new(path), data))
    }

    pub async fn exists(path: impl AsRef<Path> + Debug) -> bool {
        let result = tokio::fs::try_exists(&path).await;
        match result {
            Ok(true) => println!("[ok] {path:?}"),
            Ok(false) => println!("[null] {path:?}"),
            Err(ref e) => println!("[error] {path:?}: {e:?}"),
        }
        result.ok().unwrap_or_default()
    }

    pub async fn get_generator(gen_folder: SrcFolder, file: impl AsRef<Path>) -> MayFail<String> {
        let folder = gen_folder.gen_folder();
        let path = Self::new("tutorlolv2_dev/src/generators")
            .join(folder)
            .join(file)
            .with_extension("rs");

        #[cfg(debug_assertions)]
        Self::exists(&path).await;

        let data = tokio::fs::read_to_string(path).await?;
        Ok(data.invoke_rustfmt(80).highlight_rust())
    }

    pub async fn deserialize<T: DeserializeOwned>(origin: impl AsRef<Path>) -> MayFail<T> {
        let path = Self::new(origin).with_extension("json");

        #[cfg(debug_assertions)]
        Self::exists(&path).await;

        let bytes = tokio::fs::read(path).await?;
        let data = serde_json::from_slice::<T>(&bytes)?;
        Ok(data)
    }
}

pub fn push_end<const N: usize>(variables: [&mut String; N], end: &str) {
    variables
        .into_iter()
        .for_each(|variable| variable.push_str(end));
}

pub async fn parallel_task<_Path, _FnIn, _Type, _Resp, _Data, V, _Fut1, _Fut2>(
    permits: usize,
    path: _Path,
    f: _FnIn,
    r: _Resp,
) -> MayFail<V>
where
    _Path: AsRef<Path>,
    _Data: Send + Sync + 'static,
    _Fut2: Future<Output = MayFail<V>> + Send + Sync + 'static,
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

pub struct Tracker<'a> {
    inner: &'a mut String,
}

impl<'a> Tracker<'a> {
    pub const fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    pub const fn offset(&self) -> usize {
        self.inner.len()
    }

    pub fn record(&mut self, value: &str) -> (usize, usize) {
        let start = self.offset();
        self.inner.push_str(value);
        let end = self.offset();
        (start, end)
    }

    pub fn record_into(&mut self, value: &str, into: &mut Vec<(usize, usize)>) {
        let (start, end) = self.record(value);
        into.push((start, end));
    }
}

pub async fn new_runner() -> MayFail {
    let mut full_block = String::with_capacity(8 * 1024 * 1024);
    let mut full_exports = format!(
        "use crate::*;
        #[derive(Debug, Copy, Clone, Decode)]
        pub enum Position {{
            Top, Jungle, Middle, Bottom, Support
        }}"
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
        let (start, end) = tracker.record(&value.highlight_rust());
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
        CwdPath::fwrite("tutorlolv2_exports/assets/block.txt", full_block),
        CwdPath::fwrite("tutorlolv2_exports/src/exports.rs", full_exports),
    ] {
        task.await??
    }

    Ok(())
}
