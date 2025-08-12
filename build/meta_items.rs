use super::*;

#[derive(Deserialize)]
pub struct Positions {
    pub jungle: Vec<usize>,
    pub top: Vec<usize>,
    pub mid: Vec<usize>,
    pub adc: Vec<usize>,
    pub support: Vec<usize>,
}

pub fn internal_meta_items(_: &str) {
    let mut phf_meta_items = String::from(
        "pub static META_ITEMS: phf::Map<&'static str, &'static CachedMetaItem> = phf::phf_map! {",
    );
    let mut constdecl = String::new();

    for (champion_id, position) in
        init_map!(file HashMap<String, Positions>, "internal/meta_items.json")
    {
        phf_meta_items.push_str(&format!(
            "\t\"{}\" => &META_{},",
            champion_id,
            champion_id.to_uppercase()
        ));
        constdecl.push_str(&format!(
            r#"pub static META_{}: CachedMetaItem = CachedMetaItem {{
                top: &[{}],
                mid: &[{}],
                jungle: &[{}],
                adc: &[{}],
                support: &[{}],
                }};   
            "#,
            champion_id.to_uppercase(),
            position.top.join(","),
            position.mid.join(","),
            position.jungle.join(","),
            position.adc.join(","),
            position.support.join(",")
        ));
    }

    phf_meta_items.push_str("};");

    let _ = fs::write("internal_comptime/src/data/meta.rs", {
        let mut s = String::with_capacity(constdecl.len() + phf_meta_items.len() + USE_SUPER.len());
        s.push_str(USE_SUPER);
        s.push_str(&constdecl);
        s.push_str(&phf_meta_items);
        s
    });
}
