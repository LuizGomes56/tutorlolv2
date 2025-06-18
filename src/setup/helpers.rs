use serde::de::DeserializeOwned;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub struct SetupError(pub String);

/// Helper function to write files
pub fn write_to_file(path_name: &str, bytes: &[u8]) -> Result<(), SetupError> {
    #[cfg(debug_assertions)]
    println!("write_to_file: {}", path_name);
    let mut file: fs::File = File::create(path_name).map_err(|e: io::Error| {
        SetupError(format!("Unable to create file '{}': {}", path_name, e))
    })?;
    file.write_all(bytes).map_err(|e: io::Error| {
        SetupError(format!("Unable to write to file '{}': {}", path_name, e))
    })?;
    Ok(())
}

/// Helper to read from files and parse the value to a struct
pub fn read_json_file<T: DeserializeOwned>(path_name: &str) -> Result<T, SetupError> {
    println!("read_from_file: {}", path_name);

    let data: String = fs::read_to_string(path_name).map_err(|e: io::Error| {
        SetupError(format!(
            "Unable to read file fn[read_from_file]: PathName: {}, Error: {}",
            path_name, e
        ))
    })?;

    serde_json::from_str(&data).map_err(|e: serde_json::Error| {
        SetupError(format!(
            "Failed to parse JSON fn[read_from_file]: PathName: {}, Error: {}",
            path_name, e
        ))
    })
}

/// Receives a path and returns its file name without the `.json` extension
/// .trim_end_matches may be called if file does not end with `.json`
pub fn extract_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|os_str: &std::ffi::OsStr| os_str.to_str())
        .map(|s: &str| s.trim_end_matches(".json"))
        .unwrap_or_default()
}
