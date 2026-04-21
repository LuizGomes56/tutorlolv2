use crate::{
    champions::template::{ChampionTemplate, SkillSet},
    client::{MayFail, SyncMayFail},
};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub fn download() -> MayFail {
    let result: SyncMayFail = std::fs::read_dir("cache/wiki/champions/parsed_abilities")
        .map_err(|e| format!("[error] Unable to read directory path: {e:?}"))?
        .filter_map(Result::ok)
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let f = || -> MayFail {
                let file = entry.file_name().into_string().map_err(|e| {
                    format!("[error] Failed to get file name for entry: {entry:?}: {e:?}")
                })?;

                let file_name = file.trim_end_matches(".html");

                println!("[parallel] Processing {file_name:?}");

                let bytes = std::fs::read(entry.path())
                    .map_err(|e| format!("[error] Failed to read entry: {entry:?}: {e:?}"))?;

                let data = serde_json::from_slice::<ChampionTemplate>(&bytes).map_err(|e| {
                    format!("[error] Failed to deserialize entry: {entry:?}: {e:?}")
                })?;

                let SkillSet {
                    skill_i,
                    skill_q,
                    skill_w,
                    skill_e,
                    skill_r,
                } = data.skills;

                for (key, ability) in [
                    ('P', skill_i),
                    ('Q', skill_q),
                    ('W', skill_w),
                    ('E', skill_e),
                    ('R', skill_r),
                ] {}

                Ok(())
            };

            f().map_err(|e| e.to_string().into())
        });

    result.map_err(|e| e.to_string().into())
}
