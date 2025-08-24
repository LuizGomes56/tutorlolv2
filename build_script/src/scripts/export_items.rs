use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialStats {
    pub ability_power: Option<f64>,
    pub armor: Option<f64>,
    pub armor_penetration_percent: Option<f64>,
    pub armor_penetration_flat: Option<f64>,
    pub magic_penetration_percent: Option<f64>,
    pub magic_penetration_flat: Option<f64>,
    pub attack_damage: Option<f64>,
    pub attack_speed: Option<f64>,
    pub critical_strike_chance: Option<f64>,
    pub critical_strike_damage: Option<f64>,
    pub health: Option<f64>,
    pub lifesteal: Option<f64>,
    pub magic_resistance: Option<f64>,
    pub mana: Option<f64>,
    pub movespeed: Option<f64>,
    pub omnivamp: Option<f64>,
}

#[derive(Deserialize)]
pub struct DamageObject {
    pub minimum_damage: Option<String>,
    pub maximum_damage: Option<String>,
}

#[derive(Deserialize)]
pub struct Item {
    pub name: String,
    pub gold: u16,
    pub tier: u8,
    pub prettified_stats: BTreeMap<String, f64>,
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub ranged: Option<DamageObject>,
    pub melee: Option<DamageObject>,
    pub attributes: Attrs,
    pub purchasable: bool,
}

pub fn format_damage_object(damage_object: &Option<DamageObject>) -> String {
    macro_rules! assign_value {
        ($field:ident) => {{
            if let Some(damage_object) = damage_object {
                if let Some(raw) = damage_object.$field.as_ref().map(String::as_str) {
                    let expr = clean_math_expr(raw);
                    let (expr, changed) = transform_expr(&expr);
                    let ctx_param = if changed { "ctx" } else { "_" };
                    let lvl_param = if expr.to_lowercase().contains("level") {
                        "level"
                    } else {
                        "_"
                    };
                    format!("|{},{}|{}", lvl_param, ctx_param, expr.to_lowercase())
                } else {
                    String::from("zero")
                }
            } else {
                String::from("zero")
            }
        }};
    }
    format!(
        "CachedItemDamages{{minimum_damage:{},maximum_damage:{}}}",
        assign_value!(minimum_damage),
        assign_value!(maximum_damage),
    )
}

pub fn format_stats(stats: &PartialStats) -> String {
    let mut all_stats = Vec::new();

    macro_rules! insert_stat {
        ($field:ident) => {
            all_stats.push(format!(
                "{}:{}f32,",
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

pub struct ItemDetails {
    pub item_name: String,
    pub item_formula: String,
    pub constdecl: String,
    pub description: String,
    pub is_simulated: bool,
    pub is_damaging: bool,
}

pub fn export_items() -> Vec<(u32, ItemDetails)> {
    let mut items = init_map!(dir Item, "internal/items")
        .into_par_iter()
        .map(|(item_id_str, item)| {
            let item_id = item_id_str.parse::<u32>().unwrap();

            let prettified_stats = item
                .prettified_stats
                .iter()
                .map(|(k, v)| format!("StatName::{}({})",
                {
                    let mut s = k.replace(" ", "");
                    if s == "Lethality" {
                        s = "ArmorPenetration".to_string();
                    } 
                    if s == "HealandShieldPower" {
                        s = "HealAndShieldPower".to_string()
                    }
                    s
                }, v
            ))
                .collect::<Vec<String>>()
                .join(",");

            let constdecl = format!(
                r#"pub static {}:CachedItem=CachedItem {{gold:{},prettified_stats:&[{}],damage_type:{},attributes:Attrs::{},ranged:{},melee:{},stats:CachedItemStats{{{}}},}};"#,
                format!("{}_{}", item.name.to_screaming_snake_case(), item_id),
                item.gold,
                prettified_stats,
                if item.damage_type.is_some() {
                    format_damage_type(&item.damage_type.clone().unwrap_or_default())
                } else {
                    "DamageType::Unknown".to_string()
                },
                item.attributes.stringify(),
                format_damage_object(&item.ranged),
                format_damage_object(&item.melee),
                format_stats(&item.stats),
            );

            (
                item_id,
                ItemDetails {
                    item_name: item.name.clone(),
                    item_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 60)))
                        .replacen("class=\"type\"", "class=\"constant\"", 1),
                    constdecl,
                    description: format!(
                        "ItemDescription{{name:\"{}\",gold_cost:{}u16,prettified_stats:&[{}],}}",
                        item.name, item.gold, prettified_stats
                    ),
                    is_simulated: item.tier >= 3 && item.gold > 0 && item.purchasable,
                    is_damaging: item.ranged.is_some() || item.melee.is_some(),
                },
            )
        })
        .collect::<Vec<(u32, ItemDetails)>>();
    items.sort_by(|a, b| a.1.item_name.cmp(&b.1.item_name));
    items
}
