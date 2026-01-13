pub mod generators;
pub mod init;
pub mod model;
pub mod setup;

pub use generators::*;
pub use init::*;
pub use model::*;
pub use serde::{Serialize, de::DeserializeOwned};
pub use setup::*;

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{collections::HashMap, path::Path};

/// Alias type for [`Result`] that accepts anything that implements the trait
/// [`std::error::Error`]. Since the application doesn't need detailed errors,
/// this can be used to propagate almost all existing errors
pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// Custom trait that allows to deserialize a JSON instance
/// by providing only the file path and the desired type
pub trait JsonRead: DeserializeOwned {
    /// Receives a file path and deserializes the target JSON file into the
    /// struct that called this function as method.
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let resolved_path = resolve_path(path)?;
        println!("[read] {:?}", resolved_path.as_ref());
        let data = std::fs::read(resolved_path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    /// Stores the deserialized structs that were succesfully extracted from
    /// `.json` files inside the provided path, which should be a directory.
    /// Returns a [`HashMap`] whose keys are the file name, without the `.json`
    /// extension, and whose values are the deserialized structs. Note that all
    /// files inside the directory should have the same JSON structure, and if the
    /// deserialization fails for some file, it is skipped
    fn from_dir(path: impl AsRef<Path>) -> MayFail<HashMap<String, Self>> {
        Ok(get_file_names(&path)?
            .into_iter()
            .filter_map(|file_name| {
                let data =
                    Self::from_file(path.as_ref().join(&file_name).with_extension("json")).ok()?;
                Some((file_name, data))
            })
            .collect::<HashMap<String, Self>>())
    }
}

/// Returns a vector containing the absolute file names found in a directory,
/// without their extensions
pub fn get_file_names(path: impl AsRef<Path>) -> MayFail<Vec<String>> {
    let mut result = Vec::new();
    let resolved_path = resolve_path(path)?;
    for entry in std::fs::read_dir(resolved_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_stem()
            .ok_or("Can't recover file_stem")?
            .to_str()
            .ok_or("Could not convert file_stem to str")?
            .to_string();
        result.push(name);
    }
    Ok(result)
}

/// Resolves the path to the current working directory, which should be the parent
/// of this crate
pub fn resolve_path(path: impl AsRef<Path>) -> MayFail<impl AsRef<Path>> {
    let cwd = std::env::current_dir()?
        .to_str()
        .ok_or("Failed to get current cwd as string")?
        .to_string();
    Ok(Path::new(&cwd).join(path))
}

/// Provides a method to convert any type that implements trait [`Serialize`]
/// to a json file, and save to the provided path as a pretty-printed json
pub trait JsonWrite: Serialize {
    /// Saves a struct that implements [`Serialize`] into the provided file path
    /// as a pretty-printed json
    fn into_file(&self, path: impl AsRef<Path>) -> MayFail {
        let resolved_path = resolve_path(path)?;
        println!("[write] {:?}", resolved_path.as_ref());
        let data = serde_json::to_string_pretty(self)?;
        Ok(std::fs::write(resolved_path, data.as_bytes())?)
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
        Ok(std::fs::write(resolve_path(path)?, self)?)
    }
}

impl<T> FileWrite for T where T: AsRef<[u8]> {}

/// Resolves the file path and returns a vector with the bytes found in the provided
/// file path. Wrapper around the standard library [`std::fs::read`]
pub fn read_file(path: impl AsRef<Path>) -> MayFail<Vec<u8>> {
    Ok(std::fs::read(resolve_path(path)?)?)
}

#[track_caller]
pub fn parallel_read<P, F, T>(path: P, f: F) -> MayFail
where
    P: AsRef<Path>,
    T: DeserializeOwned,
    F: FnOnce(&str, T) -> MayFail + Send + Sync + Clone,
{
    std::fs::read_dir(path)
        .map_err(|e| format!("[error] Unable to read directory path in fn [parallel_read]: {e:?}"))?
        .filter_map(Result::ok)
        .par_bridge()
        .into_par_iter()
        .for_each(|entry| {
            let Ok(file_name) = entry.file_name().into_string() else {
                panic!("[error] Failed to get file name for entry: {entry:?}");
            };

            println!("[parallel] Processing {file_name:?}");

            let Ok(bytes) = std::fs::read(entry.path()) else {
                panic!("[error] Failed to read file bytes for entry: {entry:?}");
            };

            let Ok(data) = serde_json::from_slice::<T>(&bytes) else {
                panic!("[error] Failed to deserialize file bytes for entry: {entry:?}");
            };

            if let Err(e) = (f.clone())(&file_name, data) {
                println!("[error] Can't process {file_name:?}: {e:?}");
            }
        });
    Ok(())
}
