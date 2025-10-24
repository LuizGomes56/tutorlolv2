pub mod generators;
pub mod init;
pub mod model;
pub mod setup;

pub use generators::*;
pub use init::*;
pub use model::*;
pub use serde::{Serialize, de::DeserializeOwned};
pub use setup::*;

use std::{collections::HashMap, fs, path::Path};

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub trait JsonRead: DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> MayFail<Self> {
        let data = std::fs::read(path)?;
        Ok(serde_json::from_slice(&data)?)
    }

    fn from_dir(path: impl AsRef<Path>) -> MayFail<HashMap<String, Self>> {
        let mut map = HashMap::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            let data = Self::from_file(path)?;
            map.insert(name, data);
        }
        Ok(map)
    }
}

pub trait JsonWrite: Serialize {
    fn into_file(&self, path: impl AsRef<Path>) -> MayFail {
        let data = serde_json::to_string_pretty(self)?;
        Ok(std::fs::write(path, data.as_bytes())?)
    }
}

impl<T> JsonRead for T where T: DeserializeOwned {}
impl<T> JsonWrite for T where T: Serialize {}
