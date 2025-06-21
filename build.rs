use std::{
    collections::HashMap,
    env,
    fs::{self, DirEntry},
    path::Path,
};

fn main() {
    let out_path: String = env::var("OUT_DIR").unwrap();

    if !Path::new("formulas").exists() {
        fs::create_dir("formulas").unwrap();
    }

    {
        let mut match_arms: String = String::new();
        let mut test_mods: Vec<String> = Vec::new();

        for entry in fs::read_dir("src/writers").unwrap() {
            let entry: DirEntry = entry.unwrap();
            let name: String = entry.file_name().into_string().unwrap();
            if name.ends_with(".rs") && name != "mod.rs" {
                let content: String = fs::read_to_string(entry.path()).unwrap();
                let formulas_path: &String =
                    &format!("formulas/{}.txt", name.trim_end_matches(".rs"));
                if !Path::new(formulas_path).exists() {
                    fs::File::create(formulas_path).unwrap();
                }
                fs::write(formulas_path, content.as_bytes()).unwrap();
                let mod_name: &str = name.trim_end_matches(".rs");
                match_arms += &format!(
                    "\t\t\"{mod_name}\" => Some(writers::{mod_name}::transform(result)),\n",
                );
                test_mods.push(format!(
                    "(\"{}\", Box::new(writers::{}::test))",
                    mod_name, mod_name
                ));
            }
        }

        let writer_transform_code: String = format!(
            "pub fn try_transform(name: &str, result: CdnChampion) -> Option<Champion> {{\n\tmatch name {{\n{match_arms}\t\t_ => None,\n\t}}\n}}"
        );

        let writer_tests_code: String = format!(
            r#"pub fn writer_tests() -> Vec<(&'static str, Box<dyn Fn(Option<CdnChampion>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send>)> {{
    vec![
        {}
    ]
}}"#,
            test_mods.join(",\n\t\t")
        );
        fs::write(format!("{}/writers_test.rs", out_path), writer_tests_code).unwrap();

        fs::write(
            format!("{}/writers_generated.rs", out_path),
            writer_transform_code,
        )
        .unwrap();
    }

    {
        if let Ok(champion_names) = fs::read_to_string("internal/champion_names.json") {
            if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&champion_names) {
                for (_, champion_id) in map.into_iter() {
                    let id = champion_id.to_lowercase();
                    if !Path::new(&format!("src/writers/{}.rs", id)).exists() {
                        let content = r#"use super::{Ability, CdnChampion, Champion,
	                        HashMap, Target, extract_ability_damage}; 
                            // #![auto_generated] 
                            #[writer_macros::writer] pub fn transform(data: CdnChampion) -> Champion {}"#;
                        fs::write(&format!("src/writers/{}.rs", id), content.as_bytes()).unwrap();
                    } else {
                        let mut content =
                            fs::read_to_string(&format!("src/writers/{}.rs", id)).unwrap();
                        if !content.contains("#[writer_macros::test]") {
                            content.push_str(
                                "\n\n#[writer_macros::test]\npub fn test(data: Option<CdnChampion>) {}",
                            );
                        }
                        fs::write(&format!("src/writers/{}.rs", id), content.as_bytes()).unwrap();
                    }
                }
            }
        };
    }
}
