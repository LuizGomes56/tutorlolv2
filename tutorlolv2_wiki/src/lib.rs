use crate::client::MayFail;
use scraper::Selector;
use std::{fs::DirEntry, path::Path};

pub mod champions;
pub mod client;
pub mod formula;
pub mod items;
pub mod parser;
pub mod render;
pub mod runes;

pub fn selector(selectors: &str) -> MayFail<Selector> {
    Selector::parse(selectors)
        .map_err(|e| format!("[selector] Error parsing selector: {selectors:?}: {e:?}").into())
}

pub fn write(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> MayFail<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
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

pub fn is_dir(entry: &DirEntry) -> bool {
    entry
        .file_type()
        .ok()
        .map(|v: std::fs::FileType| v.is_dir())
        .unwrap_or(false)
}

pub fn file_name(entry: &DirEntry) -> MayFail<String> {
    entry
        .file_name()
        .into_string()
        .map_err(|e| format!("[error] Failed to get file name for entry: {entry:?}: {e:?}").into())
}

pub async fn run() -> MayFail {
    champions::run().await?;
    items::run().await?;
    runes::run().await
}
