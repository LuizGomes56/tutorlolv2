use crate::client::MayFail;
use scraper::Selector;
use std::{fs::DirEntry, path::Path};

pub mod champions;
pub mod client;
pub mod items;
pub mod runes;

pub fn selector(selectors: &str) -> MayFail<Selector> {
    Selector::parse(selectors)
        .map_err(|e| format!("[selector] Error parsing selector: {selectors:?}: {e:?}").into())
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
