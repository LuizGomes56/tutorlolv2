use std::{env, fs, path::Path};

fn main() {
    let out_path: String = env::var("OUT_DIR").unwrap();
    let writer_dir: &Path = Path::new("src/writers");
    let mut match_arms: String = String::new();

    for entry in fs::read_dir(writer_dir).unwrap() {
        let entry = entry.unwrap();
        let name: String = entry.file_name().into_string().unwrap();
        if name.ends_with(".rs") && name != "mod.rs" {
            let mod_name: &str = name.trim_end_matches(".rs");
            match_arms += &format!(
                "\t\t\"{mod}\" => Some(writers::{mod}::transform(result)),\n",
                mod = mod_name
            );
        }
    }

    let writer_transform_code: String = format!(
        "pub fn try_transform(name: &str, result: CdnChampion) -> Option<Champion> {{\n\tmatch name {{\n{match_arms}\t\t_ => None,\n\t}}\n}}"
    );

    fs::write(
        format!("{}/writers_generated.rs", out_path),
        writer_transform_code,
    )
    .unwrap();
}
