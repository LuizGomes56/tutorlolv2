use serde::de::DeserializeOwned;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{Mutex, Semaphore};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

use crate::scripts::champions::generate_champions;

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub struct Generated {
    pub exports: String,
    pub block: String,
}

pub struct CwdPath;

impl CwdPath {
    pub fn new(path: impl AsRef<Path>) -> PathBuf {
        Path::new("../").join(path)
    }
}

pub fn push_end<const N: usize>(variables: [&mut String; N], end: &str) {
    variables
        .into_iter()
        .for_each(|variable| variable.push_str(end));
}

pub async fn _parallel<P, F, T, R, Fut>(
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

pub async fn new_runner() -> MayFail {
    let mut block = String::with_capacity(8 * 1024 * 1024);

    let closure = generate_champions().await?;
    let data = closure(0);

    // for task in [
    //     tokio::spawn(tokio::fs::write("block.txt", data.block)),
    //     tokio::spawn(tokio::fs::write("exports.rs", data.exports)),
    // ] {
    //     task.await??;
    // }

    Ok(())
}
