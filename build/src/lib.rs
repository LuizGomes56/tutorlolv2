use crate::generators::impls::champions::{champion_gen_fn, champion_ids};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};
use tutorlolv2_dev::JsonRead;
use tutorlolv2_wiki::champions::WikiChampion;

mod generators;
mod scripts;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

pub fn run() -> MayFail {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let mut cmd48 = Command::new("rustfmt");
    cmd48.arg("max_width=48");

    let mut cmd80 = Command::new("rustfmt");

    let mut cmod = String::new();

    for id in champion_ids() {
        let lower_id = id.to_lowercase();
        let out_path = out_dir.join(id);

        cmd48.arg(out_path.with_extension("w48"));
        cmd80.arg(out_path.with_extension("rs"));

        writeln!(
            &mut cmod,
            r#"pub mod {lower_id} {{
            include!(concat!(env!("OUT_DIR"), "/{id}.rs"));
        }}"#
        )?;
    }

    champion_ids()
        .into_par_iter()
        .try_for_each(|id| -> MayFail {
            let lower_id = id.to_lowercase();

            let wiki_path = format!("cache/wiki/champions/{id}/data.json");
            let impl_path = format!("build/src/generators/impls/champions/{lower_id}.rs");
            let out_path = out_dir.join(id);

            let input = PathBuf::from(&impl_path);
            let output = PathBuf::from(&out_path.with_extension("rs"));

            if !needs_regeneration(&input, &output) {
                return Ok(());
            }

            let data = WikiChampion::from_file(&wiki_path)?;
            let function = champion_gen_fn(id).ok_or(format!(
                "[error] Failed to find generator function for {id}"
            ))?;

            function(data)?.call()?.build(&out_dir)
        })?;

    cmd48.status()?;
    cmd80.status()?;

    std::fs::write(out_dir.join("champions.rs"), cmod)?;

    Ok(())
}

fn needs_regeneration(input: &Path, output: &Path) -> bool {
    let input_time = input.metadata().and_then(|m| m.modified()).unwrap();

    let output_time = output
        .metadata()
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    input_time > output_time
}
