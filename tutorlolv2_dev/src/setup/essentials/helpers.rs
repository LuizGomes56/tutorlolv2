use serde::de::DeserializeOwned;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Helper function to write files
pub fn write_to_file(path_name: &str, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("write_to_file: {}", path_name);
    let mut file: fs::File = File::create(path_name)?;
    file.write_all(bytes)?;
    Ok(())
}

/// Helper to read from files and parse the value to a struct
pub fn read_json_file<T: DeserializeOwned>(
    path_name: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    println!("read_from_file: {}", path_name);

    let data: String = fs::read_to_string(path_name)?;

    serde_json::from_str(&data).map_err(|e| format!("Failed to parse {}: {}", path_name, e).into())
}

/// Receives a path and returns its file name without the `.json` extension
/// .trim_end_matches may be called if file does not end with `.json`
pub fn extract_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|os_str: &std::ffi::OsStr| os_str.to_str())
        .map(|s: &str| s.trim_end_matches(".json"))
        .unwrap_or_default()
}
