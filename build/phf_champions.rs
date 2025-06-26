use super::transform_expr;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionCdnStatsMap {
    pub flat: f64,
    pub per_level: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionCdnStats {
    pub health: ChampionCdnStatsMap,
    pub mana: ChampionCdnStatsMap,
    pub armor: ChampionCdnStatsMap,
    pub magic_resistance: ChampionCdnStatsMap,
    pub attack_damage: ChampionCdnStatsMap,
    pub attack_speed: ChampionCdnStatsMap,
    pub movespeed: ChampionCdnStatsMap,
    pub critical_strike_damage: ChampionCdnStatsMap,
    pub critical_strike_damage_modifier: ChampionCdnStatsMap,
    pub attack_speed_ratio: ChampionCdnStatsMap,
    pub attack_range: ChampionCdnStatsMap,
    pub aram_damage_taken: ChampionCdnStatsMap,
    pub aram_damage_dealt: ChampionCdnStatsMap,
    pub urf_damage_taken: ChampionCdnStatsMap,
    pub urf_damage_dealt: ChampionCdnStatsMap,
}

#[derive(Deserialize)]
pub struct Ability {
    pub name: String,
    pub damage_type: String,
    pub damages_in_area: bool,
    pub minimum_damage: Vec<String>,
    pub maximum_damage: Vec<String>,
}

#[derive(Deserialize)]
pub struct Champion {
    pub name: String,
    pub adaptative_type: String,
    pub attack_type: String,
    pub positions: Vec<String>,
    pub stats: ChampionCdnStats,
    pub abilities: HashMap<String, Ability>,
}

fn format_stats(stats: &ChampionCdnStats) -> String {
    macro_rules! insert_stat {
        ($field:ident) => {
            format!(
                "{}: CachedChampionStatsMap {{\n\t\t\tflat: {}f64,\n\t\t\tper_level: {}f64\n\t\t}},",
                stringify!($field),
                stats.$field.flat,
                stats.$field.per_level
            )
        };
    }
    let mut all_stats = Vec::new();
    all_stats.push(insert_stat!(health));
    all_stats.push(insert_stat!(mana));
    all_stats.push(insert_stat!(armor));
    all_stats.push(insert_stat!(magic_resistance));
    all_stats.push(insert_stat!(attack_damage));
    all_stats.push(insert_stat!(attack_speed));
    all_stats.push(insert_stat!(movespeed));
    all_stats.push(insert_stat!(critical_strike_damage));
    all_stats.push(insert_stat!(critical_strike_damage_modifier));
    all_stats.push(insert_stat!(attack_speed_ratio));
    all_stats.push(insert_stat!(attack_range));
    all_stats.push(insert_stat!(aram_damage_taken));
    all_stats.push(insert_stat!(aram_damage_dealt));
    all_stats.push(insert_stat!(urf_damage_taken));
    all_stats.push(insert_stat!(urf_damage_dealt));
    all_stats.join("\n\t\t")
}

/// ```
/// minimum_damage: |level: usize, ctx: &EvalContext| -> f64 {
/// match level {
///     1 => 75.0 + 0.55 * ctx.ability_power + 0.02 * ctx.bonus_mana,
///     2 => 95.0 + 0.55 * ctx.ability_power + 0.02 * ctx.bonus_mana,
///     3 => 115.0 + 0.55 * ctx.ability_power + 0.02 * ctx.bonus_mana,
///     4 => 135.0 + 0.55 * ctx.ability_power + 0.02 * ctx.bonus_mana,
///     5 => 155.0 + 0.55 * ctx.ability_power + 0.02 * ctx.bonus_mana,
///     _ => 0.0,
/// }
/// maximum_damage: |_, _| 0.0,
/// ```
fn format_abilities(abilities: &HashMap<String, Ability>) -> String {
    let mut res = String::new();
    for (name, ability) in abilities {
        let mut min_dmg = String::new();
        let mut max_dmg = String::new();
        macro_rules! format_dmg {
            ($var:expr, $field:ident) => {
                if ability.$field.is_empty() {
                    $var.push_str("|_, _| 0.0");
                } else {
                    let transformed: Vec<(String, bool)> = ability
                        .$field
                        .iter()
                        .map(|dmg| transform_expr(dmg))
                        .collect();
                    let needs_ctx = transformed.iter().any(|&(_, changed)| changed);
                    let ctx_param = if needs_ctx { "ctx: &EvalContext" } else { "_" };
                    $var.push_str(&format!(
                        "|level: usize, {}| -> f64 {{\n\t\t\t\t\tmatch level {{",
                        ctx_param
                    ));
                    for (i, (expr, _)) in transformed.into_iter().enumerate() {
                        $var.push_str(&format!("\n\t\t\t\t\t\t{} => {},", i + 1, expr));
                    }
                    $var.push_str("\n\t\t\t\t\t\t_ => 0.0,");
                    $var.push_str("\n\t\t\t\t\t}");
                    $var.push_str("\n\t\t\t\t}");
                }
            };
        }
        format_dmg!(min_dmg, minimum_damage);
        format_dmg!(max_dmg, maximum_damage);
        res.push_str(&format!(
            "(\n\t\t\t\"{}\",\n\t\t\tCachedChampionAbility {{\n\t\t\t\tname: \"{}\",\n\t\t\t\tdamage_type: \"{}\",\n\t\t\t\tdamages_in_area: {},\n\t\t\t\tminimum_damage: {},\n\t\t\t\tmaximum_damage: {},\n\t\t\t}},\n\t\t),\n\t\t",
            name,
            ability.name,
            ability.damage_type,
            ability.damages_in_area,
            min_dmg,
            max_dmg,
        ));
    }
    res
}

pub fn global_phf_internal_champions(out_dir: &str) {
    let mut internal_champions_map = HashMap::new();
    let out_path = Path::new(&out_dir).join("internal_champions.rs");
    let mut phf_map_contents = String::from(
        "pub static INTERNAL_CHAMPIONS: ::phf::Map<&'static str, &'static CachedChampion> = ::phf::phf_map! {\n",
    );
    let mut consts_decl = String::new();

    if let Some(dir) = fs::read_dir("internal/champions").ok() {
        for entry in dir {
            let path = entry.unwrap().path();
            let file_stem = path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let content = fs::read_to_string(&path).unwrap();
            let parsed = serde_json::from_str::<Champion>(&content).unwrap();
            internal_champions_map.insert(file_name.to_owned(), parsed);
        }

        for (key, champion) in &internal_champions_map {
            phf_map_contents.push_str(&format!("\t\"{}\" => &{},\n", key, key.to_uppercase()));
            consts_decl.push_str(&format!(
                r#"pub const {}: CachedChampion = CachedChampion {{
    name: "{}",
    adaptative_type: "{}",
    attack_type: "{}",
    positions: &["{}"],
    stats: CachedChampionStats {{
        {}
    }},
    abilities: &[
        {}
    ],
}};

"#,
                key.to_uppercase(),
                champion.name,
                champion.adaptative_type,
                champion.attack_type,
                champion.positions.join("\", \""),
                format_stats(&champion.stats),
                format_abilities(&champion.abilities),
            ));
        }
    }

    phf_map_contents.push_str("};\n");

    let final_content = format!("{}{}", consts_decl, phf_map_contents);
    fs::write(out_path, final_content).unwrap();
}
