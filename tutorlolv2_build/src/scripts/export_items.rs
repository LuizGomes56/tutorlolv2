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
    #[serde(rename = "criticalStrikeChance")]
    pub crit_chance: Option<f64>,
    #[serde(rename = "criticalStrikeDamage")]
    pub crit_damage: Option<f64>,
    pub health: Option<f64>,
    pub lifesteal: Option<f64>,
    #[serde(rename = "magicResistance")]
    pub magic_resist: Option<f64>,
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
    pub damage_type: String,
    pub stats: PartialStats,
    pub ranged: Option<DamageObject>,
    pub melee: Option<DamageObject>,
    pub attributes: Attrs,
    pub purchasable: bool,
}

fn get_items_decl(item_name: &str, item: &Item) -> (String, String, String) {
    let metadata = format!(
        "TypeMetadata {{ 
            kind: ItemId::{name}, 
            damage_type: DamageType::{damage_type}, 
            attributes: Attrs::{attributes:?} 
        }}",
        name = item_name.remove_special_chars(),
        damage_type = item.damage_type,
        attributes = item.attributes
    );

    let assign_value = |expr: &Option<String>| {
        if let Some(raw) = expr.as_ref().map(String::as_str) {
            let (new_expr, changed) = raw.clean_math_expr().transform_expr();
            let ctx_param = if changed { "ctx" } else { "_" };
            Some(format!("|{}|{}", ctx_param, new_expr.to_lowercase()))
        } else {
            None
        }
    };

    let make_closure = |damage_object: &Option<DamageObject>| {
        let data = if let Some(damage) = damage_object {
            (
                assign_value(&damage.minimum_damage),
                assign_value(&damage.maximum_damage),
            )
        } else {
            (None, None)
        };
        let mut closures = Vec::new();
        if let Some(min) = data.0 {
            closures.push(min);
        };
        if let Some(max) = data.1 {
            closures.push(max);
        };
        format!("&[{}]", closures.join(","))
    };

    let range_closure = make_closure(&item.ranged);
    let melee_closure = make_closure(&item.melee);

    (metadata, range_closure, melee_closure)
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
    insert_stat!(crit_chance);
    insert_stat!(crit_damage);
    insert_stat!(health);
    insert_stat!(lifesteal);
    insert_stat!(magic_resist);
    insert_stat!(mana);
    insert_stat!(movespeed);
    insert_stat!(omnivamp);
    all_stats.join("")
}

pub struct ItemDetails {
    pub item_name: String,
    pub item_formula: String,
    pub constdecl: String,
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
                .map(|(k, v)| {
                    format!(
                        "StatName::{}({})",
                        {
                            let mut s = k.replace(" ", "");
                            if s == "Lethality" {
                                s = "ArmorPenetration".to_string();
                            }
                            if s == "HealandShieldPower" {
                                s = "HealAndShieldPower".to_string()
                            }
                            s
                        },
                        v
                    )
                })
                .collect::<Vec<String>>()
                .join(",");

            let (metadata, range_closure, melee_closure) = get_items_decl(&item.name, &item);

            let constdecl = format!(
                "pub static {name}: CachedItem = CachedItem {{
                    gold: {gold},
                    prettified_stats: &[{prettified_stats}],
                    damage_type: DamageType::{damage_type},
                    attributes: Attrs::{attributes:?},
                    metadata: {metadata},
                    range_closure: {range_closure},
                    melee_closure: {melee_closure},
                    stats: CachedItemStats {{{stats}}},
                }};",
                name = format_args!("{}_{}", item.name.to_screaming_snake_case(), item_id),
                gold = item.gold,
                damage_type = item.damage_type,
                attributes = item.attributes,
                stats = format_stats(&item.stats),
            );

            (
                item_id,
                ItemDetails {
                    item_formula: constdecl
                        .invoke_rustfmt(70)
                        .clear_suffixes()
                        .highlight_rust()
                        .replace_const(),
                    constdecl,
                    is_simulated: item.tier >= 3 && item.gold > 0 && item.purchasable,
                    is_damaging: item.ranged.is_some() || item.melee.is_some(),
                    item_name: item.name,
                },
            )
        })
        .collect::<Vec<(u32, ItemDetails)>>();
    items.sort_by(|a, b| a.1.item_name.cmp(&b.1.item_name));
    items
}
