use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

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

fn format_abilities(abilities: &HashMap<String, Ability>) -> String {
    let mut res = String::new();
    for (name, ability) in abilities {
        res.push_str(&format!(
            "(\n\t\t\t\"{}\",\n\t\t\tCachedChampionAbility {{\n\t\t\t\tname: \"{}\",\n\t\t\t\tdamage_type: \"{}\",\n\t\t\t\tdamages_in_area: {},\n\t\t\t\tminimum_damage: &[{}],\n\t\t\t\tmaximum_damage: &[{}],\n\t\t\t}},\n\t\t),\n\t\t",
            name,
            ability.name,
            ability.damage_type,
            ability.damages_in_area,
            ability
                .minimum_damage
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<String>>()
                .join(", "),
            ability
                .maximum_damage
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<String>>()
                .join(", "),
        ));
    }
    res
}

pub fn global_phf_internal_champions(out_dir: &str) {
    let mut internal_champions_map = HashMap::new();

    for entry in fs::read_dir("internal/champions").unwrap() {
        let path = entry.unwrap().path();
        let file_stem = path.file_stem().unwrap();
        let file_name = file_stem.to_str().unwrap();
        let content = fs::read_to_string(&path).unwrap();
        let parsed = serde_json::from_str::<Champion>(&content).unwrap();
        internal_champions_map.insert(file_name.to_owned(), parsed);
    }

    let out_path = Path::new(&out_dir).join("internal_champions.rs");
    let mut phf_map_contents = String::from(
        "pub static INTERNAL_CHAMPIONS: ::phf::Map<&'static str, &'static CachedChampion> = ::phf::phf_map! {\n",
    );
    let mut consts_decl = String::new();

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

    phf_map_contents.push_str("};\n");

    let final_content = format!("{}{}", consts_decl, phf_map_contents);
    fs::write(out_path, final_content).unwrap();
}
