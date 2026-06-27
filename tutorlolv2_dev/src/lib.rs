use {
    crate::client::HttpClient,
    serde::{Serialize, de::DeserializeOwned},
    std::{collections::BTreeMap, fs::DirEntry, path::Path, sync::LazyLock},
};

pub mod client;
pub mod riot;

pub const DDRAGON_ENDPOINT: &str = "https://ddragon.leagueoflegends.com";
pub const CANISBACK_ENDPOINT: &str = "https://ddragon.canisback.com/img";
pub const LOL_LANGUAGE: &str = "en_US";
pub static LOL_VERSION: &str = "16.13.1";

/// Wrapper around [`reqwest::Client`] which implements methods
/// to download and save files to a local cache and avoids requests
/// to the same URLs
pub static HTTP_CLIENT: LazyLock<HttpClient> = LazyLock::new(HttpClient::new);

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;

/// Alias type for [`Result`] that accepts anything that implements the trait
/// [`std::error::Error`]. Since the application doesn't need detailed errors,
/// this can be used to propagate almost all existing errors
pub type MayFail<T = (), E = DynError> = Result<T, E>;

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

/// Provides a method to convert any type that implements trait [`Serialize`]
/// to a json file, and save to the provided path as a pretty-printed json
pub trait JsonWrite: Serialize {
    /// Saves a struct that implements [`Serialize`] into the provided file path
    /// as a pretty-printed json
    fn into_file(&self, path: impl AsRef<Path>) -> MayFail {
        let path = path.as_ref();
        println!("[write] {path:?}");

        let data = serde_json::to_string_pretty(self)?;
        Ok(write(path, data.as_bytes())?)
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
        Ok(write(path, self)?)
    }
}

impl<T> FileWrite for T where T: AsRef<[u8]> {}

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
