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
    pub prettified_stats: HashMap<String, f64>,
    pub damage_type: Option<String>,
    pub stats: PartialStats,
    pub builds_from: Vec<u32>,
    pub levelings: Option<Vec<u8>>,
    pub ranged: Option<DamageObject>,
    pub melee: Option<DamageObject>,
    pub damages_onhit: bool,
    pub purchasable: bool,
}

pub fn format_damage_object(damage_object: &Option<DamageObject>) -> String {
    macro_rules! assign_value {
        ($field:ident) => {{
            if let Some(damage_object) = damage_object {
                if let Some(raw) = damage_object.$field.as_ref().map(String::as_str) {
                    let expr = clean_math_expr(raw);
                    let (expr, changed) = transform_expr(&expr);
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
    pub item_generator: String,
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
                "pub static ITEM_FORMULAS: phf::Map<u32, &'static str> = phf::phf_map! {",
            ),
            item_generator: String::from(
                "pub static ITEM_GENERATOR: phf::Map<u32, &'static str> = phf::phf_map! {",
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
        self.item_generator.push_str("};");
        self.item_descriptions.push_str("};");
    }
    pub fn join_fields(self) -> String {
        let mut s = String::with_capacity(
            self.item_name_to_id.len()
                + self.item_id_to_name.len()
                + self.item_formulas.len()
                + self.item_generator.len()
                + self.item_descriptions.len(),
        );
        s.push_str(&self.item_name_to_id);
        s.push_str(&self.item_id_to_name);
        s.push_str(&self.item_formulas);
        s.push_str(&self.item_generator);
        s.push_str(&self.item_descriptions);
        s.push_str(
            "pub struct ItemDescription {
            pub name: &'static str,
            pub gold_cost: u16,
            pub prettified_stats: &'static [(&'static str, f64)],
        }",
        );
        s
    }
}

fn non_void_stats(stats: &PartialStats) -> bool {
    macro_rules! check_val {
        ($($stat:ident),*) => {{
            let mut sum = 0.0;
            $(
                sum += stats.$stat.unwrap_or(0.0);
            )*
            match sum {
                0.0..15.0 => true,
                _ => false
            }
        }};
    }

    check_val!(
        ability_power,
        armor,
        armor_penetration_percent,
        armor_penetration_flat,
        magic_penetration_percent,
        magic_penetration_flat,
        attack_damage,
        attack_speed,
        critical_strike_chance,
        critical_strike_damage,
        health,
        lifesteal,
        magic_resistance,
        mana,
        movespeed,
        omnivamp
    )
}

pub fn export_items(out_dir: &str) {
    let main_map = init_map!(dir Item, "internal/items");

    let mut exported_comptime_phf = ExportedComptimePhfs::new();
    let mut phf_internal_items = String::from(
        "pub static INTERNAL_ITEMS: ::phf::Map<u32, &'static CachedItem> = ::phf::phf_map! {",
    );
    let mut constdecl_items_phf = String::new();
    let mut constdecl_simulated_items = String::from("pub static SIMULATED_ITEMS: [u32; ");
    let mut constdecl_damaging_items = String::from("pub static DAMAGING_ITEMS: [u32; ");

    let mut vec_simulated_items = Vec::new();
    let mut vec_damaging_items = Vec::new();

    for (item_id_str, item) in main_map {
        macro_rules! push_phf_arm {
            (var $varname:ident, $key:expr, $value:expr) => {
                $varname.push_str(&format!("{}u32 => {},", $key, $value))
            };
            ($field:ident, $key:expr, $value:expr) => {
                exported_comptime_phf
                    .$field
                    .push_str(&format!("{} => {},", $key, $value))
            };
        }
        let maybe_id = item_id_str.parse::<u32>();
        if maybe_id.is_err() {
            continue;
        }
        let item_id = maybe_id.unwrap();
        if item.tier >= 3 && item.gold > 0 && item.purchasable && non_void_stats(&item.stats) {
            vec_simulated_items.push(item_id);
        }
        if item.ranged.is_some() || item.melee.is_some() {
            vec_damaging_items.push(item_id);
        }

        let prettified_stats = item
            .prettified_stats
            .iter()
            .map(|(k, v)| format!("(\"{k}\", {v}f64)"))
            .collect::<Vec<String>>()
            .join(",");

        push_phf_arm!(
            item_descriptions,
            format!("{item_id}u32"),
            format!(
                "ItemDescription {{
                    name: \"{}\",
                    gold_cost: {}u16,
                    prettified_stats: &[{}],
                }}",
                item.name, item.gold, prettified_stats
            )
        );
        push_phf_arm!(var phf_internal_items, item_id, format!("&ITEM_{item_id}"));

        let constdecl = format!(
            r#"pub static ITEM_{}: CachedItem = CachedItem {{
            name: "{}",gold: {},tier: {},damage_type: {},
            damages_onhit: {},ranged: {},melee: {},builds_from: 
            & [{}],levelings: {},prettified_stats: &[{}],
            stats: CachedItemStats {{{}}},}};"#,
            item_id,
            item.name,
            item.gold,
            item.tier,
            if item.damage_type.is_some() {
                format!(
                    "Some(\"{}\")",
                    item.damage_type.clone().unwrap_or("UNKNOWN".to_string())
                )
            } else {
                "None".to_string()
            },
            item.damages_onhit,
            format_damage_object(&item.ranged),
            format_damage_object(&item.melee),
            item.builds_from.join(","),
            if item.levelings.is_some() {
                item.levelings.clone().unwrap().join(",")
            } else {
                "None".to_string()
            },
            prettified_stats,
            format_stats(&item.stats),
        );

        push_phf_arm!(
            item_name_to_id,
            format!("\"{}\"", item.name),
            format!("{}u32", item_id)
        );
        push_phf_arm!(
            item_id_to_name,
            format!("{}u32", item_id),
            format!("\"{}\"", item.name)
        );
        push_phf_arm!(
            item_formulas,
            format!("{item_id}u32"),
            format!(
                "r###\"{}\"###",
                highlight(&invoke_rustfmt(&remove_f64_suffix(&constdecl))).replacen(
                    "class=\"type\"",
                    "class=\"constant\"",
                    1,
                )
            )
        );
        constdecl_items_phf.push_str(&constdecl);
    }

    let assign_path = |v: &'static str| Path::new(out_dir).join(v);
    let write_fn = |to: &'static str, content: String| {
        let path = assign_path(to);
        let _ = fs::write(&path, content.as_bytes());
    };
    exported_comptime_phf.add_braces();
    phf_internal_items.push_str("};");

    constdecl_simulated_items.push_str(&format!(
        "{}] = [{}];",
        vec_simulated_items.len(),
        vec_simulated_items.join(",")
    ));

    constdecl_damaging_items.push_str(&format!(
        "{}] = [{}];",
        vec_damaging_items.len(),
        vec_damaging_items.join(",")
    ));

    write_fn("internal_items.rs", {
        let mut s = String::with_capacity(
            phf_internal_items.len()
                + constdecl_items_phf.len()
                + constdecl_simulated_items.len()
                + constdecl_damaging_items.len(),
        );
        s.push_str(&phf_internal_items);
        s.push_str(&constdecl_items_phf);
        s.push_str(&constdecl_simulated_items);
        s.push_str(&constdecl_damaging_items);
        s
    });
    let _ = fs::write(
        "comptime_exports/items.txt",
        exported_comptime_phf.join_fields().as_bytes(),
    );
}
