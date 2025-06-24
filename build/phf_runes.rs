use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize)]
pub struct Rune {
    pub name: String,
    pub damage_type: String,
    pub ranged: String,
    pub melee: String,
}

pub fn global_phf_internal_runes(out_dir: &str) {
    let out_path = Path::new(&out_dir).join("internal_runes.rs");
    let mut phf_map_contents = String::from(
        "pub static INTERNAL_RUNES: ::phf::Map<usize, &'static CachedRune> = ::phf::phf_map! {\n",
    );
    let mut consts_decl = String::new();

    if let Some(content) = fs::read_to_string("internal/runes.json").ok() {
        let internal_runes_map = serde_json::from_str::<HashMap<usize, Rune>>(&content).unwrap();

        for (key, rune) in &internal_runes_map {
            phf_map_contents.push_str(&format!("\t{}usize => &RUNE_{},\n", key, key));
            consts_decl.push_str(&format!(
                r#"pub const RUNE_{}: CachedRune = CachedRune {{
    name: "{}",
    damage_type: "{}",
    ranged: "{}",
    melee: "{}",
}};
            
"#,
                key, rune.name, rune.damage_type, rune.ranged, rune.melee
            ));
        }
    }

    phf_map_contents.push_str("};\n");

    let final_content = format!("{}{}", consts_decl, phf_map_contents);
    fs::write(out_path, final_content).unwrap();
}
