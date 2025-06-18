use std::{
    fs,
    io::{BufRead, BufReader, Write},
    sync::Arc,
};

use reqwest::Client;

use crate::{
    EnvConfig,
    setup::{api::fetch_version, helpers::SetupError},
};

/// Change in `.env` the value of existing environment variable, or creates it if it doesn't exist.
/// Shoud not be used since directly updating `.env` variables are not trivial.
unsafe fn set_env_var(key: &str, value: &str) -> std::io::Result<()> {
    let path: &'static str = ".env";
    let file: fs::File = fs::File::open(path)?;
    let reader: BufReader<fs::File> = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut found: bool = false;
    for line in reader.lines() {
        let line: String = line?;
        if line.starts_with(&format!("{}=", key)) {
            lines.push(format!("{}={}", key, value));
            found = true;
        } else {
            lines.push(line);
        }
    }
    if !found {
        lines.push(format!("{}={}", key, value));
    }
    let mut out: fs::File = fs::File::create(path)?;
    for l in lines {
        writeln!(out, "{}", l)?;
    }
    Ok(())
}

/// Update `LOL_VERSION` in `.env`. The route that calls this function
/// will be scheduled to run once a day.
#[writer_macros::trace_time]
pub async unsafe fn update_env_version(
    client: Client,
    envcfg: Arc<EnvConfig>,
) -> Result<(), SetupError> {
    let version: &String = &fetch_version(client, envcfg).await?;
    Ok(unsafe { set_env_var("LOL_VERSION", version).unwrap() })
}
