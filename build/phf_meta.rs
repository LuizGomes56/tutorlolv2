use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize)]
pub struct Positions {
    pub jungle: Vec<usize>,
    pub top: Vec<usize>,
    pub mid: Vec<usize>,
    pub adc: Vec<usize>,
    pub support: Vec<usize>,
}

pub fn global_phf_internal_meta_items(out_dir: &str) {
    let out_path = Path::new(&out_dir).join("internal_meta.rs");
    let mut phf_map_contents = String::from(
        "pub static META_ITEMS: ::phf::Map<&'static str, &'static CachedMetaItem> = ::phf::phf_map! {\n",
    );
    let mut consts_decl = String::new();

    if let Some(content) = fs::read_to_string("internal/meta_items.json").ok() {
        let meta_items_map = serde_json::from_str::<HashMap<String, Positions>>(&content).unwrap();
        for (key, item) in &meta_items_map {
            macro_rules! join_usize {
                ($field:ident) => {
                    item.$field
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                };
            }

            phf_map_contents.push_str(&format!("\t\"{}\" => &META_{},\n", key, key.to_uppercase()));
            consts_decl.push_str(&format!(
                r#"pub static META_{}: CachedMetaItem = CachedMetaItem {{
    top: &[{}],
    mid: &[{}],
    jungle: &[{}],
    adc: &[{}],
    support: &[{}],
}};
            
"#,
                key.to_uppercase(),
                join_usize!(top),
                join_usize!(mid),
                join_usize!(jungle),
                join_usize!(adc),
                join_usize!(support)
            ));
        }
    }

    phf_map_contents.push_str("};\n");

    let final_content = format!("{}{}", consts_decl, phf_map_contents);
    fs::write(out_path, final_content).unwrap();
}
