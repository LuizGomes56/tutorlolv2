use super::*;

struct ExportedComptimePhfs {
    pub rune_name_to_id: String,
    pub rune_id_to_name: String,
    pub rune_formulas: String,
}

impl ExportedComptimePhfs {
    pub fn new() -> Self {
        Self {
            rune_name_to_id: String::from(
                "pub static RUNE_NAME_TO_ID: phf::OrderedMap<&'static str, u32> = phf::phf_ordered_map! {",
            ),
            rune_id_to_name: String::from(
                "pub static RUNE_ID_TO_NAME: phf::OrderedMap<u32, &'static str> = phf::phf_ordered_map! {",
            ),
            rune_formulas: String::from(
                "pub static RUNE_FORMULAS: phf::Map<u32, (usize, usize)> = phf::phf_map! {",
            ),
        }
    }
    pub fn add_braces(&mut self) {
        self.rune_name_to_id.push_str("};");
        self.rune_id_to_name.push_str("};");
        self.rune_formulas.push_str("};");
    }
    pub fn join_fields(self) -> String {
        let mut s = String::with_capacity(
            self.rune_name_to_id.len() + self.rune_id_to_name.len() + self.rune_formulas.len(),
        );
        s.push_str(&self.rune_name_to_id);
        s.push_str(&self.rune_id_to_name);
        s.push_str(&self.rune_formulas);
        s
    }
}

#[derive(Deserialize)]
pub struct Rune {
    pub name: String,
    pub damage_type: String,
    pub ranged: String,
    pub melee: String,
}

pub fn export_runes(out_dir: &str, mega_block: &mut String) {
    let mut phf_internal_runes = String::from(
        "pub static INTERNAL_RUNES: phf::Map<u32, &'static CachedRune> = phf::phf_map! {",
    );
    let mut exported_comptime_phf = ExportedComptimePhfs::new();
    let mut constdecl_internal_runes = String::new();
    let mut phf_damaging_runes =
        String::from("pub static DAMAGING_RUNES: phf::Set<u32> = phf::phf_set!(");
    let mut phf_btree_name_to_id = BTreeMap::new();
    let main_map = init_map!(file BTreeMap<u32, Rune>, "internal/runes.json");
    let damaging_runes = main_map.keys().cloned().collect::<Vec<u32>>();

    struct Offsets {
        rune_formula: (usize, usize),
    }

    struct Details {
        rune_name: String,
        rune_formula: String,
        constdecl: String,
        offsets: Offsets,
    }

    let mut results = main_map
        .into_par_iter()
        .map(|(rune_id, rune)| {
            let ranged_expr = transform_expr(&clean_math_expr(&rune.ranged));
            let melee_expr = transform_expr(&clean_math_expr(&rune.melee));
            let constdecl = format!(
                r#"pub static RUNE_{}: CachedRune = CachedRune {{
            name: "{}",damage_type: {},ranged: {},melee: {},}};"#,
                rune_id,
                rune.name,
                format_damage_type(&rune.damage_type),
                format!(
                    "|_, {}| {}",
                    if ranged_expr.1 { "ctx" } else { "_" },
                    ranged_expr.0.to_lowercase()
                ),
                format!(
                    "|_, {}| {}",
                    if melee_expr.1 { "ctx" } else { "_" },
                    melee_expr.0.to_lowercase()
                ),
            );

            (
                rune_id,
                Details {
                    rune_name: rune.name.clone(),
                    rune_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 80)))
                        .replacen("class=\"type\"", "class=\"constant\"", 1),
                    constdecl,
                    offsets: Offsets {
                        rune_formula: (0, 0),
                    },
                },
            )
        })
        .collect::<BTreeMap<u32, Details>>();

    let mut current_offset = mega_block.len();

    for (_, detail) in results.iter_mut() {
        let formula_start = current_offset;
        mega_block.push_str(&detail.rune_formula);
        let formula_end = current_offset + detail.rune_formula.len();
        detail.offsets.rune_formula = (formula_start, formula_end);
        current_offset = formula_end;
    }

    for (rune_id, details) in results {
        phf_internal_runes.push_str(&format!("{}u32 => &RUNE_{},", rune_id, rune_id));
        constdecl_internal_runes.push_str(&details.constdecl);
        phf_btree_name_to_id.insert(details.rune_name.clone(), rune_id);
        exported_comptime_phf
            .rune_id_to_name
            .push_str(&format!("{}u32 => \"{}\",", rune_id, details.rune_name));
        exported_comptime_phf.rune_formulas.push_str(&format!(
            "{}u32 => ({}, {}),",
            rune_id, details.offsets.rune_formula.0, details.offsets.rune_formula.1
        ));
    }

    exported_comptime_phf.rune_name_to_id.push_str(
        &phf_btree_name_to_id
            .iter()
            .map(|(rune_name, rune_id)| format!("\"{}\" => {}u32,", rune_name, rune_id))
            .collect::<Vec<String>>()
            .join(""),
    );

    phf_damaging_runes.push_str(
        &damaging_runes
            .iter()
            .map(|rune_id| format!("{}u32,", rune_id))
            .collect::<Vec<String>>()
            .join(""),
    );

    exported_comptime_phf.add_braces();
    phf_internal_runes.push_str("};");
    phf_damaging_runes.push_str(");");

    let assign_path = |v: &'static str| Path::new(out_dir).join(v);
    let write_fn = |to: &'static str, content: String| {
        let path = assign_path(to);
        fs::write(&path, content.as_bytes()).unwrap();
    };

    write_fn("internal_runes.rs", {
        let mut s = String::with_capacity(
            phf_internal_runes.len() + constdecl_internal_runes.len() + phf_damaging_runes.len(),
        );
        s.push_str(&phf_internal_runes);
        s.push_str(&constdecl_internal_runes);
        s.push_str(&phf_damaging_runes);
        s.push_str(&format!(
            "pub const SIZE_DAMAGING_RUNES: usize = {};",
            damaging_runes.len()
        ));
        s
    });
    let _ = fs::write(
        "comptime_exports/runes.txt",
        exported_comptime_phf.join_fields().as_bytes(),
    );
}
