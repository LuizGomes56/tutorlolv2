use super::*;

struct ExportedComptimePhfs {
    pub rune_name_to_id: String,
    pub rune_id_to_name: String,
    pub rune_formulas: String,
    pub rune_generator: String,
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
                "pub static RUNE_FORMULAS: phf::Map<u32, &'static str> = phf::phf_map! {",
            ),
            rune_generator: String::from(
                "pub static RUNE_GENERATOR: phf::Map<u32, &'static str> = phf::phf_map! {",
            ),
        }
    }
    pub fn add_braces(&mut self) {
        self.rune_name_to_id.push_str("};");
        self.rune_id_to_name.push_str("};");
        self.rune_formulas.push_str("};");
        self.rune_generator.push_str("};");
    }
    pub fn join_fields(self) -> String {
        let mut s = String::with_capacity(
            self.rune_name_to_id.len()
                + self.rune_id_to_name.len()
                + self.rune_formulas.len()
                + self.rune_generator.len(),
        );
        s.push_str(&self.rune_name_to_id);
        s.push_str(&self.rune_id_to_name);
        s.push_str(&self.rune_formulas);
        s.push_str(&self.rune_generator);
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

pub fn export_runes(out_dir: &str) {
    let mut phf_internal_runes = String::from(
        "pub static INTERNAL_RUNES: phf::Map<u32, &'static CachedRune> = phf::phf_map! {",
    );
    let mut exported_comptime_phf = ExportedComptimePhfs::new();
    let mut constdecl_internal_runes = String::new();
    let mut constdecl_damaging_runes = String::from("const DAMAGING_RUNES: [u32; ");
    let mut vec_damaging_runes = Vec::new();
    let main_map = init_map!(file HashMap<u32, Rune>, "internal/runes.json");

    for (rune_id, rune) in main_map {
        macro_rules! push_phf_arm {
            (var $varname:ident, $key:expr, $value:expr) => {
                $varname.push_str(&format!("{}u32 => {},", $key, $value))
            };
            ($field:ident, $key:expr, $value:expr) => {
                exported_comptime_phf
                    .$field
                    .push_str(&format!("{} => {},", $key, $value))
            };
        }

        if !rune.ranged.is_empty() || !rune.melee.is_empty() {
            vec_damaging_runes.push(rune_id);
        }
        let ranged_expr = transform_expr(&clean_math_expr(&rune.ranged));
        let melee_expr = transform_expr(&clean_math_expr(&rune.melee));
        push_phf_arm!(
            rune_id_to_name,
            format!("{}u32", rune_id),
            format!("\"{}\"", rune.name)
        );
        push_phf_arm!(rune_name_to_id, format!("\"{}\"", rune.name), rune_id);
        push_phf_arm!(var phf_internal_runes, rune_id, format!("&RUNE_{}", rune_id));

        let constdecl = format!(
            r#"pub static RUNE_{}: CachedRune = CachedRune {{
            name: "{}",damage_type: "{}",ranged: {},melee: {},}};"#,
            rune_id,
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
        );
        constdecl_internal_runes.push_str(&constdecl);
        push_phf_arm!(
            rune_formulas,
            format!("{}u32", rune_id),
            format!(
                "r###\"{}\"###",
                highlight(&remove_f64_suffix(&invoke_rustfmt(&constdecl))).replacen(
                    "class=\"type\"",
                    "class=\"constant\"",
                    1
                )
            )
        );
    }

    exported_comptime_phf.add_braces();
    phf_internal_runes.push_str("};");

    let assign_path = |v: &'static str| Path::new(out_dir).join(v);
    let write_fn = |to: &'static str, content: String| {
        let path = assign_path(to);
        fs::write(&path, content.as_bytes()).unwrap();
    };

    constdecl_damaging_runes.push_str(&format!(
        "{}] = [{}];",
        vec_damaging_runes.len(),
        vec_damaging_runes.join(",")
    ));

    write_fn("internal_runes.rs", {
        let mut s = String::with_capacity(
            phf_internal_runes.len()
                + constdecl_internal_runes.len()
                + constdecl_damaging_runes.len(),
        );
        s.push_str(&phf_internal_runes);
        s.push_str(&constdecl_internal_runes);
        s.push_str(&constdecl_damaging_runes);
        s
    });
    let _ = fs::write(
        "comptime_exports/runes.txt",
        exported_comptime_phf.join_fields().as_bytes(),
    );
}
