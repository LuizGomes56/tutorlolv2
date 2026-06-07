use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, fs::DirEntry, path::Path};
use tutorlolv2_dev::JsonRead;
use tutorlolv2_wiki::champions::WikiChampion;

use crate::generators::impls::champions::{Aatrox, champion_gen_fn, champion_ids};

mod generators;

pub type DynError = Box<dyn core::error::Error + Send + Sync + 'static>;
pub type MayFail<T = (), E = DynError> = Result<T, E>;

pub fn s() -> MayFail {
    for id in champion_ids() {
        let lower_id = id.to_lowercase();
        let generator = format!("build/src/generators/impls/champions/{lower_id}.rs");

        println!("cargo::rerun-if-changed={generator}");

        // if changed:
        {
            let path = format!("cache/wiki/champions/{id}/data.json");
            let data = WikiChampion::from_file(&path)?;
            let function = champion_gen_fn(id).ok_or(format!(
                "[error] Failed to find generator function for {id}"
            ))?;

            function(data)?.call()?._end();
        }
    }

    Ok(())
}
