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
    let mut meta_items = String::from("#[rustfmt::skip]\npub static META_ITEMS: [CachedMetaItem; ");
    let mut constdecl = String::new();

    let main_map = init_map!(file BTreeMap<String, Positions>, "internal/meta_items.json");
    let len = main_map.len();
    for (_, position) in main_map {
        constdecl.push_str(&format!(
            "CachedMetaItem {{top:&[{}],mid:&[{}],jungle:&[{}],adc:&[{}],support:&[{}],}},",
            position.top.join(","),
            position.mid.join(","),
            position.jungle.join(","),
            position.adc.join(","),
            position.support.join(",")
        ));
    }

    meta_items.push_str(&format!(
        "{}] = [{}];",
        len,
        constdecl.trim_end_matches(',')
    ));

    let _ = fs::write("internal_comptime/src/data/meta.rs", {
        let mut s = String::with_capacity(meta_items.len() + USE_SUPER.len());
        s.push_str(USE_SUPER);
        s.push_str(&meta_items);
        s
    });
}
