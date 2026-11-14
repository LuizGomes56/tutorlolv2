use crate::{FORMULA_DOCS_ENV, MayFail};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// The repository containing this build dependency, independent of the launch directory.
pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}

#[derive(Debug, Clone)]
pub struct BuildContext {
    pub root: PathBuf,
    pub out: PathBuf,
    pub docs: bool,
    pub export_json: bool,
}

impl BuildContext {
    pub fn new(root: impl AsRef<Path>, out: impl AsRef<Path>, docs: bool) -> MayFail<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .map_err(|e| format!("Invalid build root {:?}: {e}", root.as_ref()))?;

        let out = root.join(out);

        Ok(Self {
            root,
            out,
            docs,
            export_json: false,
        })
    }

    pub fn from_env() -> MayFail<Self> {
        let out = std::env::var_os("OUT_DIR").ok_or("OUT_DIR is required for a Cargo build")?;

        Self::new(
            project_root(),
            out,
            std::env::var_os(FORMULA_DOCS_ENV).is_some(),
        )
    }

    pub fn inputs(&self) -> Vec<PathBuf> {
        [
            "build/src/generators/champions",
            "build/src/generators/impls/items",
            "build/src/generators/impls/runes",
            "cache/wiki/champions/full.json",
            "cache/wiki/champions/model.json",
            "cache/wiki/items/full.json",
            "cache/wiki/items/data.lua",
            "cache/wiki/runes/full.json",
            "cache/wiki/runes",
            "cache/wiki/spells/full.json",
            "cache/wiki/map_changes/full.json",
            "cache/riot/items.json",
            "cache/riot/runes.json",
            "cache/riot/champion_languages.json",
            "internal/scraper/data.json",
        ]
        .into_iter()
        .map(|path| {
            let mut path = self.root.join(path);

            // Cargo repeatedly rebuilds for nonexistent paths. Watch the nearest
            // existing parent until the optional recommendations and map changes
            // files appear.
            if path.ends_with("internal/scraper/data.json")
                || path.ends_with("cache/wiki/map_changes/full.json")
            {
                while !path.exists() && path != self.root {
                    path.pop();
                }
            }

            path
        })
        .collect()
    }
}

pub fn run_formatter(command: &mut Command) -> MayFail {
    // Windows has a small command-line budget; generated files have long UNC paths.
    // Keep options intact and format file batches without changing their order.
    let args = command
        .get_args()
        .map(std::ffi::OsStr::to_owned)
        .collect::<Vec<_>>();

    let first_file = args
        .iter()
        .position(|arg| Path::new(arg).extension().is_some_and(|ext| ext == "rs"));

    if let Some(first_file) = first_file {
        for files in args[first_file..].chunks(32) {
            let mut batch = Command::new(command.get_program());

            batch.args(&args[..first_file]).args(files);

            if let Some(dir) = command.get_current_dir() {
                batch.current_dir(dir);
            }

            for (key, value) in command.get_envs() {
                if let Some(value) = value {
                    batch.env(key, value);
                } else {
                    batch.env_remove(key);
                }
            }

            run_formatter_once(&mut batch)?;
        }

        Ok(())
    } else {
        run_formatter_once(command)
    }
}

fn run_formatter_once(command: &mut Command) -> MayFail {
    let output = command
        .output()
        .map_err(|e| format!("Failed to start rustfmt: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "rustfmt failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
