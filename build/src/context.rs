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
            "build/src/generators/impls/champions",
            "build/src/generators/impls/items",
            "build/src/generators/impls/runes",
            "cache/wiki/champions/full.json",
            "cache/wiki/items/full.json",
            "cache/wiki/runes/full.json",
            "cache/riot/items.json",
            "cache/riot/runes.json",
            "cache/riot/champion_languages.json",
            "internal/scraper/data.json",
        ]
        .into_iter()
        .map(|path| {
            let mut path = self.root.join(path);

            // Cargo repeatedly rebuilds for nonexistent paths. Watch the nearest
            // existing parent until the optional recommendations file appears.
            if path.ends_with("internal/scraper/data.json") {
                while !path.exists() && path != self.root {
                    path.pop();
                }
            }

            path
        })
        .collect()
    }
}

pub(crate) fn run_formatter(command: &mut Command) -> MayFail {
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
