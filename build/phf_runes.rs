use crate::build::highlight;

use super::{invoke_rustfmt, transform_expr};
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
    let mut damaging_runes_decl = String::from("const DAMAGING_RUNES: [usize; ");
    let mut consts_decl = String::new();
    let mut damaging_runes_vec = Vec::<String>::new();
    let rune_formulas_out_path = Path::new(&out_dir).join("rune_formulas.br");
    let static_runes_out_path = Path::new(&out_dir).join("static_runes.br");
    let mut static_runes_map = HashMap::<String, usize>::new();
    let mut rune_formulas_map = HashMap::<usize, String>::new();

    if let Some(content) = fs::read_to_string("internal/runes.json").ok() {
        let internal_runes_map = serde_json::from_str::<HashMap<usize, Rune>>(&content).unwrap();

        for (key, rune) in &internal_runes_map {
            if !rune.ranged.is_empty() || !rune.melee.is_empty() {
                damaging_runes_vec.push(key.to_string());
            }
            let ranged_expr = transform_expr(&rune.ranged);
            let melee_expr = transform_expr(&rune.melee);
            static_runes_map.insert(rune.name.clone(), *key);
            phf_map_contents.push_str(&format!("\t{}usize => &RUNE_{},\n", key, key));

            let string_content = invoke_rustfmt(&format!(
                r#"pub static RUNE_{}: CachedRune = CachedRune {{
                name: "{}",damage_type: "{}",ranged: {},melee: {},}};"#,
                key,
                rune.name,
                rune.damage_type,
                format!(
                    "|_, {}| {}",
                    if ranged_expr.1 {
                        "ctx: &EvalContext"
                    } else {
                        "_"
                    },
                    ranged_expr.0.to_lowercase()
                ),
                format!(
                    "|_, {}| {}",
                    if melee_expr.1 {
                        "ctx: &EvalContext"
                    } else {
                        "_"
                    },
                    melee_expr.0.to_lowercase()
                ),
            ));
            consts_decl.push_str(&string_content);
            rune_formulas_map.insert(
                *key,
                highlight(&string_content).replacen("class=\"type\"", "class=\"constant\"", 1),
            );
        }
    }

    let static_runes_bytes = compress_bytes!(static_runes_map);
    let rune_formulas_bytes = compress_bytes!(rune_formulas_map);

    fs::write(rune_formulas_out_path, rune_formulas_bytes).unwrap();
    fs::write(static_runes_out_path, static_runes_bytes).unwrap();

    damaging_runes_decl.push_str(&format!(
        "{}] = [{}];",
        damaging_runes_vec.len(),
        damaging_runes_vec.join(",")
    ));

    phf_map_contents.push_str("};\n");

    let final_content = format!(
        "{}{}\n\n{}",
        consts_decl, phf_map_contents, damaging_runes_decl
    );
    fs::write(out_path, invoke_rustfmt(&final_content)).unwrap();
}
