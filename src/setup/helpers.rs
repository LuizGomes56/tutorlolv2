use serde::de::DeserializeOwned;
use std::{
    fs::{self, File},
    io::{self, Write},
};

#[derive(Debug)]
pub struct SetupError(pub String);

// Helper function to write files
pub fn write_to_file(path_name: &str, bytes: &[u8]) -> Result<(), SetupError> {
    println!("write_to_file: {}", path_name);
    let mut file: fs::File = File::create(path_name).map_err(|e: io::Error| {
        SetupError(format!("Unable to create file '{}': {}", path_name, e))
    })?;
    file.write_all(bytes).map_err(|e: io::Error| {
        SetupError(format!("Unable to write to file '{}': {}", path_name, e))
    })?;
    Ok(())
}

// Helper to read from files and parse the value to a struct
pub fn read_from_file<T: DeserializeOwned>(path_name: &str) -> Result<T, SetupError> {
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
