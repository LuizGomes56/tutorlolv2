use super::*;

#[derive(Deserialize)]
pub struct Rune {
    pub name: String,
    pub damage_type: String,
    pub ranged: String,
    pub melee: String,
}

pub struct RuneDetails {
    pub rune_name: String,
    pub rune_formula: String,
    pub constdecl: String,
}

pub fn export_runes() -> Vec<(u32, RuneDetails)> {
    let mut runes = init_map!(file BTreeMap<u32, Rune>, "../internal/runes.json")
        .into_par_iter()
        .map(|(rune_id, rune)| {
            let ranged_expr = transform_expr(&clean_math_expr(&rune.ranged));
            let melee_expr = transform_expr(&clean_math_expr(&rune.melee));
            let constdecl = format!(
                r#"pub static RUNE_{}:CachedRune=CachedRune{{damage_type:{},ranged:{},melee:{},}};"#,
                rune_id,
                format_damage_type(&rune.damage_type),
                format!(
                    "|_,{}|{}",
                    if ranged_expr.1 { "ctx" } else { "_" },
                    ranged_expr.0.to_lowercase()
                ),
                format!(
                    "|_,{}|{}",
                    if melee_expr.1 { "ctx" } else { "_" },
                    melee_expr.0.to_lowercase()
                ),
            );

            (
                rune_id,
                RuneDetails {
                    rune_name: rune.name.clone(),
                    rune_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 80)))
                        .replacen("class=\"type\"", "class=\"constant\"", 1),
                    constdecl,
                },
            )
        })
        .collect::<Vec<(u32, RuneDetails)>>();
    runes.sort_by(|a, b| a.1.rune_name.cmp(&b.1.rune_name));
    runes
}
