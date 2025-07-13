use super::{highlight, invoke_rustfmt, transform_expr};
use crate::compress_bytes;
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
                "{}: CachedChampionStatsMap {{
                flat: {}f64,per_level: {}f64}},",
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
    all_stats.join("")
}

fn format_abilities(abilities: &HashMap<String, Ability>) -> HashMap<String, String> {
    let mut formatted_map = HashMap::new();
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
                        "|level: usize, {}| -> f64 {{match level {{",
                        ctx_param
                    ));
                    for (i, (expr, _)) in transformed.into_iter().enumerate() {
                        $var.push_str(&format!("{} => {},", i + 1, expr.to_lowercase()));
                    }
                    $var.push_str("_ => 0.0,}}");
                }
            };
        }
        format_dmg!(min_dmg, minimum_damage);
        format_dmg!(max_dmg, maximum_damage);
        formatted_map.insert(
            name.clone(),
            format!(
                "static __phantom__: CachedChampionAbility = CachedChampionAbility {{name: \"{}\",damage_type: \"{}\",
                damages_in_area: {},minimum_damage: {},
                maximum_damage: {},}};",
                ability.name, ability.damage_type, ability.damages_in_area, min_dmg, max_dmg,
            ),
        );
    }
    formatted_map
}

pub fn global_phf_internal_champions(out_dir: &str) {
    let internal_champions_out_path = Path::new(&out_dir).join("internal_champions.rs");
    let internal_names_out_path = Path::new(&out_dir).join("internal_names.rs");
    let static_champions_out_path = Path::new(&out_dir).join("static_champions.br");
    let champion_formulas_out_path = Path::new(&out_dir).join("champion_formulas.br");
    let ability_formulas_out_path = Path::new(&out_dir).join("ability_formulas.br");
    let champion_generator_out_path = Path::new(&out_dir).join("champion_generator.br");

    let mut map_internal_champions = HashMap::new();
    let mut map_champion_formulas = HashMap::new();
    let mut map_ability_formulas = HashMap::new();
    let mut map_champion_generator = HashMap::new();
    let mut map_static_names = HashMap::<String, String>::new();

    let mut phf_internal_names = String::from(
        "pub static INTERNAL_NAMES: ::phf::Map<&'static str, &'static str> = ::phf::phf_map! {\n",
    );
    let mut phf_internal_champions = String::from(
        "pub static INTERNAL_CHAMPIONS: ::phf::Map<&'static str, 
        &'static CachedChampion> = ::phf::phf_map! {",
    );
    let mut phf_static_bindings = HashMap::<String, String>::new();

    if let Some(dir) = fs::read_dir("internal/champions").ok() {
        for entry in dir {
            let path = entry.unwrap().path();
            let file_stem = path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let content = fs::read_to_string(&path).unwrap();
            let parsed = serde_json::from_str::<Champion>(&content).unwrap();
            map_internal_champions.insert(file_name.to_owned(), parsed);
        }

        for (champion_id, champion) in &map_internal_champions {
            phf_internal_names.push_str(&format!(
                "\t\"{}\" => \"{}\",\n",
                champion.name.clone(),
                champion_id.clone()
            ));
            map_static_names.insert(champion_id.clone(), champion.name.clone());

            phf_internal_champions.push_str(&format!(
                "\"{}\" => &{},",
                champion_id,
                champion_id.to_uppercase()
            ));

            map_champion_generator.insert(
                champion_id.clone(),
                invoke_rustfmt(
                    &fs::read_to_string(format!("src/generators/{}.rs", champion_id)).unwrap(),
                ),
            );

            let formatted_abilities = format_abilities(&champion.abilities);
            let mut map_ability_inner_formulas = HashMap::new();
            let mut decl_parts = Vec::with_capacity(formatted_abilities.len());

            for (key, val) in formatted_abilities.into_iter() {
                let rustfmt_val = invoke_rustfmt(&val)
                    .replace(
                        "static __phantom__: CachedChampionAbility = CachedChampionAbility ",
                        "",
                    )
                    .replace(";", "");
                let highlighted_val = highlight(&format!(
                    "intrinsic {}_{} = {}",
                    champion_id.to_uppercase(),
                    key.to_uppercase(),
                    rustfmt_val
                ))
                .replacen("class=\"type\"", "class=\"constant\"", 1);
                map_ability_inner_formulas.insert(key.clone(), highlighted_val);
                decl_parts.push(format!(
                    "(\"{}\", CachedChampionAbility {})",
                    key, rustfmt_val
                ));
            }

            let abilities_decl = decl_parts.join(",");
            map_ability_formulas.insert(champion_id.clone(), map_ability_inner_formulas);

            let decl = invoke_rustfmt(&format!(
                r#"pub static {}: CachedChampion = CachedChampion {{
                    name: "{}",
                    adaptative_type: "{}",
                    attack_type: "{}",
                    positions: &["{}"],
                    stats: CachedChampionStats {{{}}},
                    abilities: &[{}],
                }};"#,
                champion_id.to_uppercase(),
                champion.name,
                champion.adaptative_type,
                champion.attack_type,
                champion.positions.join("\", \""),
                format_stats(&champion.stats),
                abilities_decl,
            ));

            map_champion_formulas.insert(champion_id.clone(), highlight(&decl));
            phf_static_bindings.insert(champion_id.clone(), decl);
        }
    }

    phf_internal_champions.push_str("};\n");
    phf_internal_names.push_str("};\n");

    let compressed_champion_formulas = compress_bytes!(map_champion_formulas);
    let compressed_ability_formulas = compress_bytes!(map_ability_formulas);
    let compressed_champion_generator = compress_bytes!(map_champion_generator);
    let compressed_internal_names = compress_bytes!(map_static_names);

    fs::write(champion_formulas_out_path, compressed_champion_formulas).unwrap();
    fs::write(ability_formulas_out_path, compressed_ability_formulas).unwrap();
    fs::write(champion_generator_out_path, compressed_champion_generator).unwrap();
    fs::write(static_champions_out_path, compressed_internal_names).unwrap();
    fs::write(internal_names_out_path, phf_internal_names).unwrap();

    let final_content = format!(
        "{}{}",
        phf_static_bindings
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<String>>()
            .join("\n"),
        phf_internal_champions
    );
    fs::write(internal_champions_out_path, &final_content).unwrap();
}
