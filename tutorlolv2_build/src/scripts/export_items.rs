use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_percent: f64,
    pub armor_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    #[serde(rename = "criticalStrikeChance")]
    pub crit_chance: f64,
    #[serde(rename = "criticalStrikeDamage")]
    pub crit_damage: f64,
    pub health: f64,
    pub lifesteal: f64,
    #[serde(rename = "magicResistance")]
    pub magic_resist: f64,
    pub mana: f64,
    pub movespeed: f64,
    pub omnivamp: f64,
}

#[derive(Deserialize)]
pub struct DamageObject {
    pub minimum_damage: String,
    pub maximum_damage: String,
}

#[derive(Deserialize)]

pub struct Item {
    pub riot_id: u32,
    pub name: String,
    pub price: u32,
    pub tier: u8,
    pub prettified_stats: Vec<String>,
    pub damage_type: String,
    pub stats: ItemStats,
    pub ranged: DamageObject,
    pub melee: DamageObject,
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

    let assign_value = |expr: &str| {
        if expr.is_empty() || expr == "zero" {
            None
        } else {
            let (new_expr, changed) = expr.clean_math_expr().transform_expr();
            let ctx_param = if changed { "ctx" } else { "_" };
            Some(format!("|{ctx_param}|{}", new_expr.to_lowercase()))
        }
    };

    let make_closure = |damage_object: &DamageObject| {
        let data = (
            assign_value(&damage_object.minimum_damage),
            assign_value(&damage_object.maximum_damage),
        );
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

pub fn format_stats(stats: &ItemStats) -> String {
    let mut all_stats = Vec::new();

    macro_rules! insert_stat {
        ($field:ident) => {
            all_stats.push(format!("{}:{}f32,", stringify!($field), stats.$field));
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
        .map(|(_, item)| {
            let item_id = item.riot_id;

            let prettified_stats = item.prettified_stats.join(",");

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
                name = format_args!("{}_{item_id}", item.name.to_screaming_snake_case()),
                gold = item.price,
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
                    is_simulated: item.tier >= 3 && item.price > 0 && item.purchasable,
                    is_damaging: {
                        let is_damaging = |expr: &str| expr != "zero" && !expr.is_empty();
                        is_damaging(&item.ranged.minimum_damage)
                            || is_damaging(&item.ranged.maximum_damage)
                            || is_damaging(&item.melee.minimum_damage)
                            || is_damaging(&item.melee.maximum_damage)
                    },
                    item_name: item.name,
                },
            )
        })
        .collect::<Vec<(u32, ItemDetails)>>();
    items.sort_by(|a, b| a.1.item_name.cmp(&b.1.item_name));
    items
}
