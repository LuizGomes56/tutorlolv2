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
    let mut runes = init_map!(file BTreeMap<u32, Rune>, "internal/runes.json")
        .into_par_iter()
        .map(|(rune_id, rune)| {
            let ranged_expr = transform_expr(&clean_math_expr(&rune.ranged));
            let melee_expr = transform_expr(&clean_math_expr(&rune.melee));
            let constdecl = format!(
                r#"pub static {}:CachedRune=CachedRune{{damage_type:{},ranged:{},melee:{},}};"#,
                format_args!("{}_{}", rune.name.to_screaming_snake_case(), rune_id),
                format_damage_type(&rune.damage_type),
                format_args!(
                    "|_,{}|{}",
                    if ranged_expr.1 { "ctx" } else { "_" },
                    ranged_expr.0.to_lowercase()
                ),
                format_args!(
                    "|_,{}|{}",
                    if melee_expr.1 { "ctx" } else { "_" },
                    melee_expr.0.to_lowercase()
                ),
            );

            (
                rune_id,
                RuneDetails {
                    rune_name: rune.name.clone(),
                    rune_formula: highlight_rust(&clear_suffixes(&invoke_rustfmt(&constdecl, 80)))
                        .replacen("class=\"type\"", "class=\"constant\"", 1),
                    constdecl,
                },
            )
        })
        .collect::<Vec<(u32, RuneDetails)>>();

    let mut undeclared_runes = init_map!(file BTreeMap<String, u32>, "internal/rune_names.json");

    undeclared_runes.insert("AdaptiveForce".to_string(), 9990);
    undeclared_runes.insert("HealthScaling".to_string(), 9991);
    undeclared_runes.insert("AttackSpeed".to_string(), 9992);
    undeclared_runes.insert("Health".to_string(), 9993);
    undeclared_runes.insert("AbilityHaste".to_string(), 9994);
    undeclared_runes.insert("TenacityandSlowResist".to_string(), 9995);
    undeclared_runes.insert("MoveSpeed".to_string(), 9996);

    for (rune_name, rune_id) in undeclared_runes {
        if runes.iter().any(|(id, _)| *id == rune_id) {
            continue;
        }

        let constdecl = format!(
            r#"pub static {}_{}:CachedRune=CachedRune{{damage_type:{},ranged:zero,melee:zero}};"#,
            rune_name.to_screaming_snake_case(),
            rune_id,
            format_damage_type("_"),
        );

        runes.push((
            rune_id,
            RuneDetails {
                rune_name: rune_name.clone(),
                rune_formula: highlight_rust(&clear_suffixes(&invoke_rustfmt(&constdecl, 80)))
                    .replacen("class=\"type\"", "class=\"constant\"", 1),
                constdecl,
            },
        ));
    }

    runes.sort_by(|a, b| a.1.rune_name.cmp(&b.1.rune_name));
    runes
}
