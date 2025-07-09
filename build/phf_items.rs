use super::{invoke_rustfmt, transform_expr};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability_power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_penetration_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_penetration_flat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_penetration_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_penetration_flat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_damage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_strike_chance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_strike_damage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifesteal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_resistance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movespeed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omnivamp: Option<f64>,
}

#[derive(Deserialize)]
pub struct DamageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_damage: Option<String>,
}

#[derive(Deserialize)]
pub struct Item {
    pub name: String,
    pub gold: usize,
    pub tier: usize,
    pub prettified_stats: HashMap<String, Value>,
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub builds_from: Vec<usize>,
    pub levelings: Option<Vec<usize>>,
    pub ranged: Option<DamageObject>,
    pub melee: Option<DamageObject>,
    pub damages_onhit: bool,
    pub purchasable: bool,
}

fn format_damage_object(damage_object: &Option<DamageObject>) -> String {
    macro_rules! assign_value {
        ($field:ident) => {{
            if let Some(damage_object) = damage_object {
                if let Some(raw) = damage_object.$field.as_ref().map(String::as_str) {
                    let (expr, changed) = transform_expr(raw);
                    let ctx_param = if changed { "ctx: &EvalContext" } else { "_" };
                    format!("|_, {}| {}", ctx_param, expr.to_lowercase())
                } else {
                    String::from("|_, _| 0.0")
                }
            } else {
                String::from("|_, _| 0.0")
            }
        }};
    }
    format!(
        "CachedItemDamages {{ minimum_damage: {}, maximum_damage: {} }}",
        assign_value!(minimum_damage),
        assign_value!(maximum_damage),
    )
}

fn format_prettified_stats(prettified_stats: &HashMap<String, Value>) -> String {
    prettified_stats
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_stats(stats: &PartialStats) -> String {
    let mut all_stats = Vec::new();

    macro_rules! insert_stat {
        ($field:ident) => {
            all_stats.push(format!(
                "{}: {}f64,",
                stringify!($field),
                stats.$field.unwrap_or(0.0)
            ));
        };
    }

    insert_stat!(ability_power);
    insert_stat!(armor);
    insert_stat!(armor_penetration_percent);
    insert_stat!(armor_penetration_flat);
    insert_stat!(magic_penetration_percent);
    insert_stat!(magic_penetration_flat);
    insert_stat!(attack_damage);
    insert_stat!(attack_speed);
    insert_stat!(critical_strike_chance);
    insert_stat!(critical_strike_damage);
    insert_stat!(health);
    insert_stat!(lifesteal);
    insert_stat!(magic_resistance);
    insert_stat!(mana);
    insert_stat!(movespeed);
    insert_stat!(omnivamp);
    all_stats.join("")
}

pub fn global_phf_internal_items(out_dir: &str) {
    let out_path = Path::new(&out_dir).join("internal_items.rs");
    let mut phf_map_contents = String::from(
        "pub static INTERNAL_ITEMS: ::phf::Map<usize, &'static CachedItem> = ::phf::phf_map! {\n",
    );
    let mut siml_items_decl = String::from("const SIMULATED_ITEMS: [usize; ");
    let mut damaging_items_decl = String::from("const DAMAGING_ITEMS: [usize; ");
    let mut siml_items_vec = Vec::<String>::new();
    let mut consts_decl = String::new();
    let mut damaging_items_vec = Vec::<String>::new();

    if let Some(dir) = fs::read_dir("internal/items").ok() {
        let mut internal_items_map = HashMap::new();

        for entry in dir {
            let path = entry.unwrap().path();
            let file_stem = path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let content = fs::read_to_string(&path).unwrap();
            let parsed = serde_json::from_str::<Item>(&content).unwrap();
            internal_items_map.insert(file_name.to_owned(), parsed);
        }

        for (key, item) in &internal_items_map {
            let usize_key = key.parse::<usize>().ok();
            if usize_key.is_none() {
                continue;
            }
            let usize_v = usize_key.unwrap();
            if item.tier >= 3 && item.purchasable {
                siml_items_vec.push(usize_v.to_string());
            }
            if item.ranged.is_some() || item.melee.is_some() {
                damaging_items_vec.push(usize_v.to_string());
            }
            phf_map_contents.push_str(&format!("{}usize => &ITEM_{},", key, key));
            consts_decl.push_str(&format!(
                r#"pub const ITEM_{}: CachedItem = CachedItem {{
                name: "{}",gold: {},tier: {},damage_type: {},
                damages_onhit: {},ranged: {},melee: {},builds_from: 
                &[{}],levelings: {},prettified_stats: &[{}],
                stats: CachedItemStats {{{}}},}};"#,
                key,
                item.name,
                item.gold,
                item.tier,
                if item.damage_type.is_some() {
                    format!("Some(\"{}\")", item.damage_type.clone().unwrap())
                } else {
                    "None".to_string()
                },
                item.damages_onhit,
                format_damage_object(&item.ranged),
                format_damage_object(&item.melee),
                item.builds_from
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                if item.levelings.is_some() {
                    item.levelings
                        .clone()
                        .unwrap()
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                } else {
                    "None".to_string()
                },
                format_prettified_stats(&item.prettified_stats),
                format_stats(&item.stats),
            ));
        }
    }

    siml_items_decl.push_str(&format!(
        "{}] = [{}];",
        siml_items_vec.len(),
        siml_items_vec.join(",")
    ));

    damaging_items_decl.push_str(&format!(
        "{}] = [{}];",
        damaging_items_vec.len(),
        damaging_items_vec.join(",")
    ));

    phf_map_contents.push_str("};\n");

    let final_content = format!(
        "{}{}\n{}\n\n{}",
        consts_decl, phf_map_contents, siml_items_decl, damaging_items_decl
    );
    fs::write(out_path, invoke_rustfmt(&final_content)).unwrap();
}
