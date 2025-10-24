use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::generators::MayFail;

pub trait FilePathExt {
    fn read_dir_json<T: DeserializeOwned>(&self) -> MayFail<HashMap<String, T>>;
    fn json_file_name(&self) -> &str;
    fn read_json<T: DeserializeOwned>(&self) -> MayFail<T>;
    fn write_to_file(&self, bytes: &[u8]) -> MayFail;
}

impl<T: AsRef<Path>> FilePathExt for T {
    fn read_dir_json<U: DeserializeOwned>(&self) -> MayFail<HashMap<String, U>> {
        let mut map = HashMap::new();
        for entry in fs::read_dir(self.as_ref())? {
            let entry = entry?;
            let path = entry.path();
            let data = self.read_json()?;
            map.insert(
                path.file_name().unwrap().to_string_lossy().to_string(),
                data,
            );
        }
        Ok(map)
    }

    fn json_file_name(&self) -> &str {
        self.as_ref()
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.trim_end_matches(".json"))
            .unwrap_or_default()
    }

    fn read_json<U: DeserializeOwned>(&self) -> MayFail<U> {
        let path = self.as_ref();
        println!("read_from_file: {:?}", path);
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse {:?}: {}", path, e).into())
    }

    fn write_to_file(&self, bytes: &[u8]) -> MayFail {
        let path = self.as_ref();
        println!("write_to_file: {:?}", path);
        let mut file = File::create(path)?;
        file.write_all(bytes)?;
        Ok(())
    }
}
