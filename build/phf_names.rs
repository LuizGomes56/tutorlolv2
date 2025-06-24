use std::{collections::HashMap, fs, path::Path};

pub fn global_phf_internal_names(out_dir: &str) {
    let out_path = Path::new(&out_dir).join("internal_names.rs");
    let mut phf_map_contents = String::from(
        "pub static INTERNAL_NAMES: ::phf::Map<&'static str, &'static str> = ::phf::phf_map! {\n",
    );

    if let Some(content) = fs::read_to_string("internal/champion_names.json").ok() {
        let names_map = serde_json::from_str::<HashMap<String, String>>(&content).unwrap();
        for (key, value) in &names_map {
            phf_map_contents.push_str(&format!("\t\"{}\" => \"{}\",\n", key, value));
        }
    }

    phf_map_contents.push_str("};\n");
    fs::write(out_path, phf_map_contents).unwrap();
}
