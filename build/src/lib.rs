use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};
use tutorlolv2_dev::JsonRead;
use tutorlolv2_wiki::champions::WikiChampion;

use crate::generators::impls::champions::{champion_gen_fn, champion_ids};

mod generators;
// mod scripts;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

pub fn s() -> MayFail {
    for id in champion_ids() {
        let lower_id = id.to_lowercase();

        let wiki_path = format!("cache/wiki/champions/{id}/data.json");
        let impl_path = format!("build/src/generators/impls/champions/{lower_id}.rs");
        let out_path = format!("generated/champions/{lower_id}.rs");

        let input = PathBuf::from(&impl_path);
        let output = PathBuf::from(&out_path);

        if !needs_regeneration(&input, &output) {
            continue;
        }

        let data = WikiChampion::from_file(&wiki_path)?;
        let function = champion_gen_fn(id).ok_or(format!(
            "[error] Failed to find generator function for {id}"
        ))?;

        // function(data)?.call()?._end()?;
    }

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
