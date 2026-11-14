//! Game version whose data is downloaded and cached. It is read from the
//! `LOL_VERSION` variable in the environment or in `.env`, so a new patch does
//! not require recompiling. When a newer version is released, the cache of the
//! previous one is archived in `cache/{version}`, so that the update downloads
//! everything again while the old data stays available for comparison

use crate::{HTTP_CLIENT, MayFail};
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
};

const ENV_VAR: &str = "LOL_VERSION";

static LOL_VERSION: OnceLock<String> = OnceLock::new();

/// Game version currently in use, such as `16.21.1`. Panics if `LOL_VERSION`
/// is not set in the environment or in `.env`
pub fn lol_version() -> &'static str {
    LOL_VERSION.get_or_init(|| crate::env_var(ENV_VAR).unwrap_or_else(|e| panic!("[version] {e}")))
}

/// Whether a file name is a game version such as `16.21.1`, which is how
/// archived caches are named
pub fn is_version(name: &str) -> bool {
    name.split('.')
        .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// Moves every entry of `cache` into `cache/{version}`, except previously
/// archived versions. `stats` is copied instead of moved, since `update` does
/// not rebuild it and it is only refreshed by calling `stats` manually. The
/// merged recommendations in `app/scraper/data.json` are archived with `app`
/// and copied back to the active cache, since `update` does not regenerate them.
/// The lock file is copied as well, so the generators can be checked against it
/// with `wiki lock-check` after the update
pub fn archive(cache: impl AsRef<Path>, version: &str, lock_file: impl AsRef<Path>) -> MayFail {
    let cache = cache.as_ref();
    let lock_file = lock_file.as_ref();
    let target = cache.join(version);

    if target.try_exists()? {
        return Err(format!("[version] Archive {target:?} already exists").into());
    }

    if !cache.try_exists()? {
        return Ok(());
    }

    let entries = crate::read_dir(cache)?
        .filter(|entry| !entry.file_name().to_str().is_some_and(is_version))
        .collect::<Vec<_>>();

    crate::create_dir_all(&target)?;

    for entry in entries {
        let from = entry.path();
        let to = target.join(entry.file_name());

        match entry.file_name() == "stats" {
            true => {
                println!("[copy] {from:?} -> {to:?}");
                copy_dir(&from, &to)?;
            }
            false => {
                println!("[move] {from:?} -> {to:?}");
                std::fs::rename(&from, &to)
                    .map_err(|e| format!("[version] Error moving {from:?} to {to:?}: {e:?}"))?;
            }
        }
    }

    // Preserve the manually refreshed input without retaining stale generated app data.
    let recommendations = Path::new("app/scraper/data.json");
    let archived_recommendations = target.join(recommendations);

    if archived_recommendations.try_exists()? {
        let active = cache.join(recommendations);

        crate::create_dir_all(active.parent().unwrap())?;
        println!("[copy] {archived_recommendations:?} -> {active:?}");
        std::fs::copy(&archived_recommendations, &active).map_err(|error| {
            format!("[version] Error restoring recommendations {active:?}: {error}")
        })?;
    }

    if lock_file.try_exists()? {
        let to = target.join(crate::lock::FILE_NAME);

        println!("[copy] {lock_file:?} -> {to:?}");
        std::fs::copy(lock_file, &to)
            .map_err(|e| format!("[version] Error copying {lock_file:?} to {to:?}: {e:?}"))?;
    }

    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> MayFail {
    crate::create_dir_all(to)?;

    for entry in crate::read_dir(from)? {
        let path = entry.path();
        let target = to.join(entry.file_name());

        match crate::is_dir(&entry) {
            true => copy_dir(&path, &target)?,
            false => {
                std::fs::copy(&path, &target).map_err(|e| {
                    format!("[version] Error copying {path:?} to {target:?}: {e:?}")
                })?;
            }
        }
    }

    Ok(())
}

fn confirm(question: &str) -> MayFail<bool> {
    print!("{question} [y/N] ");
    std::io::stdout().flush()?;

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;

    Ok(matches!(answer.trim().to_lowercase().as_str(), "y" | "yes"))
}

/// Compares the version in use with the newest one released. If they differ
/// and the update is confirmed, the current cache is archived and `.env` is
/// set to the newest version, which is used for the rest of the run. Must be
/// called before anything reads [`lol_version`]
pub async fn check() -> MayFail {
    let current = crate::env_var(ENV_VAR)?;
    let latest = HTTP_CLIENT.fetch_version().await?;

    let version = match current == latest {
        true => {
            println!("[version] Up to date with game version {current}");
            current
        }
        false => match confirm(&format!(
            "[version] Version {latest} is available, found {current}. Update?"
        ))? {
            true => {
                archive(PathBuf::from("cache"), &current, crate::lock::path())?;
                crate::set_env_var(ENV_VAR, &latest)?;
                println!("[version] {current} -> {latest}");
                latest
            }
            false => current,
        },
    };

    LOL_VERSION
        .set(version)
        .map_err(|_| "[version] LOL_VERSION was read before checking for updates".into())
}
