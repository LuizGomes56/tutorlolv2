use serde::de::DeserializeOwned;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub trait FilePathExt {
    fn json_file_name(&self) -> &str;
    fn read_json<T: DeserializeOwned>(&self) -> Result<T, Box<dyn std::error::Error>>;
    fn write_to_file(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

impl<T: AsRef<Path>> FilePathExt for T {
    fn json_file_name(&self) -> &str {
        self.as_ref()
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.trim_end_matches(".json"))
            .unwrap_or_default()
    }

    fn read_json<U: DeserializeOwned>(&self) -> Result<U, Box<dyn std::error::Error>> {
        let path = self.as_ref();
        println!("read_from_file: {:?}", path);
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse {:?}: {}", path, e).into())
    }

    fn write_to_file(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.as_ref();
        println!("write_to_file: {:?}", path);
        let mut file = File::create(path)?;
        file.write_all(bytes)?;
        Ok(())
    }
}
