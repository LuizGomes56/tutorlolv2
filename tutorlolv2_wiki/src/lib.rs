use {
    crate::client::HttpClient,
    serde::{Serialize, de::DeserializeOwned},
    std::{collections::BTreeMap, fs::DirEntry, path::Path, sync::LazyLock},
};

#[cfg(feature = "avif")]
pub mod avif;
pub mod champions;
pub mod client;
pub mod formula;
pub mod items;
pub mod lock;
pub mod parser;
pub mod render;
pub mod riot;
pub mod runes;
pub mod scraper;
pub mod stats;

pub const DDRAGON_ENDPOINT: &str = "https://ddragon.leagueoflegends.com";
pub const CANISBACK_ENDPOINT: &str = "https://ddragon.canisback.com/img";
pub const MOBALYTICS_ENDPOINT: &str = "https://mobalytics.gg/lol/champions";
pub const LOL_LANGUAGE: &str = "en_US";
pub const LOL_VERSION: &str = "16.21.1";

pub static HTTP_CLIENT: LazyLock<HttpClient> = LazyLock::new(HttpClient::new);

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

pub trait JsonRead: DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let data = read(path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    fn from_dir(path: impl AsRef<Path>) -> MayFail<BTreeMap<String, Self>> {
        Ok(read_dir(&path)?
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

/// Provides a method to convert any type that implements trait [`Serialize`]
/// to a json file, and save to the provided path as a pretty-printed json
pub trait JsonWrite: Serialize {
    /// Saves a struct that implements [`Serialize`] into the provided file path
    /// as a pretty-printed json
    fn write_file(&self, path: impl AsRef<Path>) -> MayFail {
        let path = path.as_ref();
        println!("[write] {path:?}");

        let data = serde_json::to_string_pretty(self)?;
        write(path, data.as_bytes())
    }
}

impl<T> JsonRead for T where T: DeserializeOwned {}
impl<T> JsonWrite for T where T: Serialize {}

/// Wrapper around the standard library [`std::fs::write`], but resolving the path
/// before calling the function, and returning a [`MayFail`] instead of [`std::io::Result`]
pub trait FileWrite: AsRef<[u8]> {
    /// Resolves the provided path and save the contents into the provided
    /// file path
    fn write_file(&self, path: impl AsRef<Path>) -> MayFail {
        write(path, self)
    }
}

impl<T> FileWrite for T where T: AsRef<[u8]> {}

/// Parses a CSS selector once and caches it in a static, returning a
/// `&'static Selector`. Panics on first use if the selector is invalid,
/// which can only happen for a malformed literal
#[macro_export]
macro_rules! selector {
    ($selectors:literal) => {{
        static SELECTOR: ::std::sync::LazyLock<::scraper::Selector> =
            ::std::sync::LazyLock::new(|| {
                ::scraper::Selector::parse($selectors).unwrap_or_else(|e| {
                    panic!("[selector] Error parsing selector: {:?}: {e:?}", $selectors)
                })
            });
        &*SELECTOR
    }};
}

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

pub fn create_dir_all(path: impl AsRef<Path>) -> MayFail {
    let path = path.as_ref();
    std::fs::create_dir_all(path)
        .map_err(|e| format!("[create_dir_all] Error creating directory: {path:?}: {e:?}").into())
}

pub fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().ok().map(|v| v.is_dir()).unwrap_or(false)
}

pub fn file_name(entry: &DirEntry) -> MayFail<String> {
    entry
        .file_name()
        .into_string()
        .map_err(|e| format!("[error] Failed to get file name for entry: {entry:?}: {e:?}").into())
}

pub fn remove_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Err(e) = std::fs::remove_file(path)
        && !e.kind().eq(&std::io::ErrorKind::NotFound)
    {
        println!("[remove_file] Error removing file: {path:?}: {e:?}");
    }
}

pub async fn run() -> MayFail {
    champions::run().await?;
    items::run().await?;
    runes::run().await?;
    lock::write()
}
