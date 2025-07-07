use super::invoke_rustfmt;
use std::{collections::HashMap, fs, path::Path};

pub fn global_phf_formulas(out_dir: &str) {
    let out_path = Path::new(&out_dir).join("const_formulas.rs");
    let mut phf_map_contents = String::from(
        "pub static WRITER_CONTENT: ::phf::Map<&'static str, &'static str> = ::phf::phf_map! {\n",
    );
    let mut consts_decl = String::new();

    if let Some(content) = fs::read_to_string("internal/champion_names.json").ok() {
        let champion_names = serde_json::from_str::<HashMap<String, String>>(&content).unwrap();

        for (_, champion_id) in &champion_names {
            phf_map_contents.push_str(&format!(
                "\t\"{}\" => &WRITER_{},\n",
                champion_id,
                champion_id.to_uppercase()
            ));
            let text = invoke_rustfmt(
                &fs::read_to_string(format!("src/writers/{}.rs", champion_id)).unwrap(),
            );
            consts_decl.push_str(&format!(
                "pub const WRITER_{}: &'static str = r########\"{}\"########;\n\n",
                champion_id.to_uppercase(),
                text,
            ));
        }
    }

    phf_map_contents.push_str("};\n");

    let final_content = format!("{}{}", consts_decl, phf_map_contents);
    fs::write(out_path, final_content).unwrap();
}
