pub mod generators;
pub mod init;
pub mod model;
pub mod setup;

pub use generators::*;
pub use init::*;
pub use model::*;
pub use serde::{Serialize, de::DeserializeOwned};
pub use setup::*;

use std::{collections::HashMap, path::Path};

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub trait JsonRead: DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let resolved_path = resolve_path(path)?;
        println!("Trying to read {:?}", resolved_path.as_ref());
        let data = std::fs::read(resolved_path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    fn from_dir(path: impl AsRef<Path>) -> MayFail<HashMap<String, Self>> {
        Ok(get_file_names(path.as_ref())?
            .into_iter()
            .filter_map(|file_name| {
                let data =
                    Self::from_file(path.as_ref().join(&file_name).with_extension("json")).ok()?;
                Some((file_name, data))
            })
            .collect::<HashMap<String, Self>>())
    }
}

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

pub fn resolve_path(path: impl AsRef<Path>) -> MayFail<impl AsRef<Path>> {
    let cwd = std::env::current_dir()?
        .to_str()
        .ok_or("Failed to get current cwd as string")?
        .to_string();
    Ok(Path::new(&cwd).join(path))
}

pub trait JsonWrite: Serialize {
    fn into_file(&self, path: impl AsRef<Path>) -> MayFail {
        let resolved_path = resolve_path(path)?;
        println!("Trying to write {:?}", resolved_path.as_ref());
        let data = serde_json::to_string_pretty(self)?;
        Ok(std::fs::write(resolved_path, data.as_bytes())?)
    }
}

impl<T> JsonRead for T where T: DeserializeOwned {}
impl<T> JsonWrite for T where T: Serialize {}

pub trait FileWrite: AsRef<[u8]> {
    fn write_file(&self, path: impl AsRef<Path>) -> MayFail {
        Ok(std::fs::write(resolve_path(path)?, self)?)
    }
}

impl<T> FileWrite for T where T: AsRef<[u8]> {}

pub fn read_file(path: impl AsRef<Path>) -> MayFail<Vec<u8>> {
    Ok(std::fs::read(resolve_path(path)?)?)
}
