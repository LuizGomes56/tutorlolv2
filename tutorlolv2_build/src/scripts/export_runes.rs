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
        .map(|(riot_id, rune)| {
            let range_expr = rune.ranged.clean_math_expr().transform_expr();
            let melee_expr = rune.melee.clean_math_expr().transform_expr();
            let damage_type = rune.damage_type;

            let metadata = format!(
                "TypeMetadata {{
                    kind: RuneId::{},
                    damage_type: DamageType::{damage_type},
                    attributes: Attrs::Undefined
                }}",
                rune.name.remove_special_chars(),
            );

            let make_closure = |expr: (String, bool)| {
                format!(
                    "|{}| {}",
                    if expr.1 { "ctx" } else { "_" },
                    expr.0.to_lowercase()
                )
            };

            let melee_closure = make_closure(melee_expr);
            let range_closure = make_closure(range_expr);

            let constdecl = format!(
                "pub static {name}: CachedRune = CachedRune {{
                    damage_type: DamageType::{damage_type},
                    metadata: {metadata},
                    melee_closure: {melee_closure},
                    range_closure: {range_closure},
                    riot_id: {riot_id},
                    undeclared: false
                }};",
                name = format_args!(
                    "{}_{riot_id}",
                    rune.name.to_screaming_snake_case().remove_special_chars(),
                ),
            );

            (
                riot_id,
                RuneDetails {
                    rune_name: rune.name,
                    rune_formula: constdecl
                        .invoke_rustfmt(80)
                        .clear_suffixes()
                        .highlight_rust()
                        .replace_const(),
                    constdecl,
                },
            )
        })
        .collect::<Vec<(u32, RuneDetails)>>();

    let mut undeclared_runes = init_map!(file BTreeMap<String, u32>, "internal/rune_names.json");

    [
        ("AdaptiveForce", 9990),
        ("HealthScaling", 9991),
        ("AttackSpeed", 9992),
        ("Health", 9993),
        ("AbilityHaste", 9994),
        ("TenacityandSlowResist", 9995),
        ("MoveSpeed", 9996),
        ("EyeballCollection", 8120),
        ("GhostPoro", 8136),
        ("ZombieWard", 8138),
    ]
    .into_iter()
    .for_each(|(undeclared_name, undeclared_id)| {
        undeclared_runes.insert(undeclared_name.to_string(), undeclared_id);
    });

    for (rune_name, riot_id) in undeclared_runes {
        if runes.iter().any(|(id, _)| *id == riot_id) {
            continue;
        }

        let void_metadata = format!(
            "TypeMetadata {{
                kind: RuneId::{name},
                damage_type: DamageType::Unknown,
                attributes: Attrs::Undefined
            }}",
            name = rune_name.remove_special_chars(),
        );

        let constdecl = format!(
            "pub static {name}: CachedRune = CachedRune {{
                damage_type: DamageType::Unknown,
                metadata: {void_metadata},
                melee_closure: zero,
                range_closure: zero,
                riot_id: {riot_id},
                undeclared: true
            }};",
            name = format_args!(
                "{}_{riot_id}",
                rune_name.to_screaming_snake_case().remove_special_chars(),
            )
        );

        runes.push((
            riot_id,
            RuneDetails {
                rune_name: rune_name.clone(),
                rune_formula: constdecl
                    .invoke_rustfmt(80)
                    .clear_suffixes()
                    .highlight_rust()
                    .replace_const(),
                constdecl,
            },
        ));
    }

    runes.sort_by(|a, b| a.1.rune_name.cmp(&b.1.rune_name));
    runes
}
