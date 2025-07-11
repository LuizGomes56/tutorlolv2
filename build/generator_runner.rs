use std::{collections::HashMap, fs, path::Path};

pub fn generator_runner(out_dir: &str) {
    let mut match_arms = String::new();

    for entry in fs::read_dir("src/generators").unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.ends_with(".rs") && name != "mod.rs" {
            let mod_name = name.trim_end_matches(".rs");
            match_arms += &format!(
                "\t\t\"{mod_name}\" => Some(generators::{mod_name}::gen_{mod_name}(result)),\n",
            );
        }
    }

    let generator_runner_code = format!(
        "pub fn try_run_generator(name: &str, result: CdnChampion) -> Option<Champion> {{\n\tmatch name {{\n{match_arms}\t\t_ => None,\n\t}}\n}}"
    );

    fs::write(
        format!("{}/generator_runner.rs", out_dir),
        generator_runner_code,
    )
    .unwrap();

    if let Ok(champion_names) = fs::read_to_string("internal/champion_names.json") {
        if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&champion_names) {
            for (_, champion_id) in map.into_iter() {
                let id = champion_id.to_lowercase();
                if !Path::new(&format!("src/generators/{}.rs", id)).exists() {
                    let content = format!(
                        r#"use super::*; 
                            // #![auto_generated] 
                            #[generator_macros::generator] pub fn gen_{id}(data: CdnChampion) -> Champion {{}}"#
                    );
                    fs::write(&format!("src/generators/{}.rs", id), content.as_bytes()).unwrap();
                }
            }
        }
    };
}
