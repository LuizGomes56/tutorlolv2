use crate::{
    ONHIT_EFFECT,
    scripts::{
        BASIC_ATTACK, CRITICAL_STRIKE, StringExt, champions::generate_champions,
        items::generate_items, runes::generate_runes,
    },
};
use serde::de::DeserializeOwned;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    sync::{Mutex, Semaphore},
    task::JoinHandle,
};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};
use tutorlolv2_fmt::encode_brotli_11;

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

        Self::exists(&path).await;

        let data = tokio::fs::read_to_string(path).await?;
        Ok(data.invoke_rustfmt(80).highlight_rust())
    }

    pub async fn deserialize<T: DeserializeOwned>(origin: impl AsRef<Path>) -> MayFail<T> {
        let path = Self::new(origin).with_extension("json");
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

pub async fn parallel_task<P, F, T, R, Fut>(
    permits: usize,
    path: P,
    f: F,
) -> MayFail<BTreeMap<String, R>>
where
    P: AsRef<Path>,
    R: Send + Sync + 'static,
    T: DeserializeOwned,
    Fut: Future<Output = MayFail<R>> + Send + Sync + 'static,
    F: Clone + Send + Sync + 'static + Fn(String, T) -> Fut,
{
    let result = Arc::new(Mutex::new(BTreeMap::new()));
    let mut futures = Vec::new();
    let semaphore = Arc::new(Semaphore::new(permits));

    let folder = CwdPath::new(path);
    let read_dir = tokio::fs::read_dir(folder).await?;
    let mut dir_stream = ReadDirStream::new(read_dir);

    while let Some(Ok(entry)) = dir_stream.next().await {
        let f = f.clone();
        let result = result.clone();
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
                async move || -> MayFail<R> {
                    let data = tokio::fs::read(entry.path()).await?;
                    let json = serde_json::from_slice(&data)?;
                    f(file_name, json).await
                }
            };

            let data = handler().await.unwrap();
            result.lock().await.insert(file_name, data);
        }))
    }

    for future in futures {
        future.await.unwrap();
    }

    let response = unsafe { Arc::try_unwrap(result).unwrap_unchecked() };
    Ok(response.into_inner())
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

pub async fn new_runner() -> MayFail {
    let mut full_block = String::with_capacity(8 * 1024 * 1024);
    let mut full_exports = String::new();
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

    let mut offset = full_block.len();

    let mut record_offsets = |value: &str| {
        let start = offset;
        let end = offset + value.len();
        full_block.push_str(value);
        offset = end;
        (start, end)
    };

    for (name, value) in [
        ("ONHIT_EFFECT_OFFSET", ONHIT_EFFECT),
        ("BASIC_ATTACK_OFFSET", BASIC_ATTACK),
        ("CRITICAL_STRIKE_OFFSET", CRITICAL_STRIKE),
    ] {
        let (start, end) = record_offsets(value);
        full_exports.push_str(&format!("pub static {name}: (u32, u32) = ({start}, {end})"));
    }

    let compressed_block = encode_brotli_11(full_block.as_bytes());

    // full_exports.push_str(UNCOMPRESSED_MEGA_BLOCK_SIZE:usize={current_offset};);
    // for task in [
    //     CwdPath::fwrite(SrcFolder::new("full_block.txt"), full_block),
    //     CwdPath::fwrite(SrcFolder::new("full_exports.rs"), full_exports),
    // ] {
    //     task.await??
    // }

    Ok(())
}
