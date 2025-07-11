use crate::{
    ENV_CONFIG,
    setup::{api::fetch_version, helpers::SetupError},
};
use reqwest::Client;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
};

/// Change in `.env` the value of existing environment variable, or creates it if it doesn't exist.
/// Shoud not be used since directly updating `.env` variables are not trivial.
unsafe fn set_env_var(key: &str, value: &str) -> std::io::Result<()> {
    let path = ".env";
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut found = false;
    for line in reader.lines() {
        let line = line?;
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
    let mut out = fs::File::create(path)?;
    for l in lines {
        writeln!(out, "{}", l)?;
    }
    Ok(())
}

/// Update `LOL_VERSION` in `.env`. The route that calls this function
/// will be scheduled to run once a day. Must not be used in production
#[generator_macros::trace_time]
pub async unsafe fn update_env_version(client: Client) -> Result<(), SetupError> {
    let version = fetch_version(client, &ENV_CONFIG.dd_dragon_endpoint).await?;
    Ok(unsafe { set_env_var("LOL_VERSION", &version).unwrap() })
}
