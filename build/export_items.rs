use super::*;
use std::collections::HashSet;

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
    pub builds_from: Vec<u32>,
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
                    format!("|{}, {}| {}", lvl_param, ctx_param, expr.to_lowercase())
                } else {
                    String::from("__zero")
                }
            } else {
                String::from("__zero")
            }
        }};
    }
    format!(
        "CachedItemDamages {{ minimum_damage: {}, maximum_damage: {} }}",
        assign_value!(minimum_damage),
        assign_value!(maximum_damage),
    )
}

pub fn format_stats(stats: &PartialStats) -> String {
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

struct ExportedComptimePhfs {
    pub item_name_to_id: String,
    pub item_id_to_name: String,
    pub item_formulas: String,
    pub item_descriptions: String,
}

impl ExportedComptimePhfs {
    pub fn new() -> Self {
        Self {
            item_name_to_id: String::from(
                "pub static ITEM_NAME_TO_ID: phf::OrderedMap<&'static str, u32> = phf::phf_ordered_map! {",
            ),
            item_id_to_name: String::from(
                "pub static ITEM_ID_TO_NAME: phf::OrderedMap<u32, &'static str> = phf::phf_ordered_map! {",
            ),
            item_formulas: String::from(
                "pub static ITEM_FORMULAS: phf::Map<u32, (usize, usize)> = phf::phf_map! {",
            ),
            item_descriptions: String::from(
                "pub static ITEM_DESCRIPTIONS: phf::Map<u32, ItemDescription> = phf::phf_map! {",
            ),
        }
    }
    pub fn add_braces(&mut self) {
        self.item_name_to_id.push_str("};");
        self.item_id_to_name.push_str("};");
        self.item_formulas.push_str("};");
        self.item_descriptions.push_str("};");
    }
    pub fn join_fields(self) -> String {
        let mut s = String::with_capacity(
            self.item_name_to_id.len()
                + self.item_id_to_name.len()
                + self.item_formulas.len()
                + self.item_descriptions.len(),
        );
        s.push_str(&self.item_name_to_id);
        s.push_str(&self.item_id_to_name);
        s.push_str(&self.item_formulas);
        s.push_str(&self.item_descriptions);
        s.push_str(
            "pub struct ItemDescription {pub name: &'static str,pub gold_cost: u16,pub prettified_stats: &'static [(&'static str, f64)],}",
        );
        s
    }
}

pub fn export_items(out_dir: &str, mega_block: &mut String) {
    let main_map = init_map!(dir Item, "internal/items");

    let mut exported_comptime_phf = ExportedComptimePhfs::new();
    let mut phf_internal_items = String::from(
        "pub static INTERNAL_ITEMS: ::phf::Map<u32, &'static CachedItem> = ::phf::phf_map! {",
    );
    let mut constdecl_items_phf = String::new();
    let mut phf_btreemap_name_to_id = BTreeMap::new();
    let mut phf_simulated_items =
        String::from("pub static SIMULATED_ITEMS: phf::OrderedSet<u32> = phf::phf_ordered_set!(");
    let mut phf_damaging_items =
        String::from("pub static DAMAGING_ITEMS: phf::Set<u32> = phf::phf_set!(");

    struct PhfSetSize {
        pub simulated_items: HashSet<u32>,
        pub damaging_items: HashSet<u32>,
    }

    let mut phf_set_size = PhfSetSize {
        simulated_items: HashSet::new(),
        damaging_items: HashSet::new(),
    };

    struct Offsets {
        item_formula: (usize, usize),
    }

    struct Details {
        item_name: String,
        item_formula: String,
        constdecl: String,
        description: String,
        is_simulated: bool,
        is_damaging: bool,
        offsets: Offsets,
    }

    let mut results =
        main_map
            .into_par_iter()
            .map(|(item_id_str, item)| {
                let item_id = item_id_str.parse::<u32>().unwrap();

                let prettified_stats = item
                    .prettified_stats
                    .iter()
                    .map(|(k, v)| format!("(\"{k}\", {v}f64)"))
                    .collect::<Vec<String>>()
                    .join(",");

                let constdecl = format!(
                    r#"pub static ITEM_{}: CachedItem = CachedItem {{
                name: "{}",gold: {},tier: {},damage_type: {},
                attributes: Attrs::{},ranged: {},melee: {},builds_from: 
                & [{}],prettified_stats: &[{}],
                stats: CachedItemStats {{{}}},}};"#,
                    item_id,
                    item.name,
                    item.gold,
                    item.tier,
                    if item.damage_type.is_some() {
                        format!(
                            "Some({})",
                            format_damage_type(&item.damage_type.clone().unwrap_or_default())
                        )
                    } else {
                        "None".to_string()
                    },
                    item.attributes.stringify(),
                    format_damage_object(&item.ranged),
                    format_damage_object(&item.melee),
                    item.builds_from.join(","),
                    prettified_stats,
                    format_stats(&item.stats),
                );

                (
                    item_id,
                    Details {
                        item_name: item.name.clone(),
                        item_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 60)))
                            .replacen("class=\"type\"", "class=\"constant\"", 1),
                        constdecl,
                        description: format!(
                            "ItemDescription {{name: \"{}\",gold_cost: {}u16,prettified_stats: &[{}],}}",
                            item.name, item.gold, prettified_stats
                        ),
                        is_simulated: item.tier >= 3 && item.gold > 0 && item.purchasable,
                        is_damaging: item.ranged.is_some() || item.melee.is_some(),
                        offsets: Offsets {
                            item_formula: (0, 0),
                        },
                    },
                )
            })
            .collect::<BTreeMap<u32, Details>>();

    let mut current_offset = mega_block.len();

    for (_, detail) in results.iter_mut() {
        let formula_start = current_offset;
        mega_block.push_str(&detail.item_formula);
        let formula_end = current_offset + detail.item_formula.len();
        detail.offsets.item_formula = (formula_start, formula_end);
        current_offset = formula_end;
    }

    for (item_id, details) in results.iter() {
        phf_btreemap_name_to_id.insert(details.item_name.clone(), *item_id);

        exported_comptime_phf
            .item_descriptions
            .push_str(&format!("{}u32 => {},", item_id, details.description));
        exported_comptime_phf
            .item_id_to_name
            .push_str(&format!("{}u32 => \"{}\",", item_id, details.item_name));
        exported_comptime_phf.item_formulas.push_str(&format!(
            "{}u32 => ({}, {}),",
            item_id, details.offsets.item_formula.0, details.offsets.item_formula.1
        ));
        constdecl_items_phf.push_str(&details.constdecl);
        phf_internal_items.push_str(&format!("{}u32 => &ITEM_{},", item_id, item_id));

        if details.is_simulated {
            phf_set_size.simulated_items.insert(*item_id);
        }
        if details.is_damaging {
            phf_set_size.damaging_items.insert(*item_id);
        }
    }

    let mut seen = HashSet::new();
    let arms = phf_btreemap_name_to_id
        .iter()
        .filter(|(name, _)| seen.insert((*name).clone()))
        .map(|(name, id)| format!("\"{}\" => {}u32,", name, id))
        .collect::<Vec<_>>()
        .join("");

    exported_comptime_phf.item_name_to_id.push_str(&arms);

    macro_rules! u32_prefix {
        ($field:ident) => {
            &phf_set_size
                .$field
                .iter()
                .map(|value| format!("{value}u32"))
                .collect::<Vec<String>>()
                .join(",")
        };
    }

    phf_simulated_items.push_str(u32_prefix!(simulated_items));
    phf_damaging_items.push_str(u32_prefix!(damaging_items));

    let assign_path = |v: &'static str| Path::new(out_dir).join(v);
    let write_fn = |to: &'static str, content: String| {
        let path = assign_path(to);
        let _ = fs::write(&path, content.as_bytes());
    };
    exported_comptime_phf.add_braces();
    phf_internal_items.push_str("};");
    phf_damaging_items.push_str(");");
    phf_simulated_items.push_str(");");

    write_fn("internal_items.rs", {
        let mut s = String::with_capacity(
            phf_internal_items.len()
                + constdecl_items_phf.len()
                + phf_simulated_items.len()
                + phf_damaging_items.len(),
        );
        s.push_str(&phf_internal_items);
        s.push_str(&constdecl_items_phf);
        s.push_str(&phf_simulated_items);
        s.push_str(&phf_damaging_items);
        s.push_str(&format!(
            "pub const SIZE_SIMULATED_ITEMS: usize = {};
            pub const SIZE_DAMAGING_ITEMS: usize = {};",
            phf_set_size.simulated_items.len(),
            phf_set_size.damaging_items.len()
        ));
        s
    });
    let _ = fs::write(
        "comptime_exports/items.txt",
        exported_comptime_phf.join_fields().as_bytes(),
    );
}
