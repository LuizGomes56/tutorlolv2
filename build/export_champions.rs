use internal_comptime::AbilityLike;

use super::*;
use std::cmp::Ordering;

struct ExportedComptimePhfs {
    pub champion_name_to_id: String,
    pub champion_id_to_name: String,
    pub champion_formulas: String,
    pub champion_generator: String,
    pub champion_abilities: String,
}

impl ExportedComptimePhfs {
    pub fn new() -> Self {
        Self {
            champion_name_to_id: String::from(
                "pub static CHAMPION_NAME_TO_ID: phf::OrderedMap<&'static str, &'static str> = phf::phf_ordered_map! {",
            ),
            champion_id_to_name: String::from(
                "pub static CHAMPION_ID_TO_NAME: phf::OrderedMap<&'static str, &'static str> = phf::phf_ordered_map! {",
            ),
            champion_formulas: String::from(
                "pub static CHAMPION_FORMULAS: phf::Map<&'static str, (usize, usize)> = phf::phf_map! {",
            ),
            champion_generator: String::from(
                "pub static CHAMPION_GENERATOR: phf::Map<&'static str, (usize, usize)> = phf::phf_map! {",
            ),
            champion_abilities: String::from(
                "pub static CHAMPION_ABILITIES: phf::OrderedMap<&'static str, &'static phf::OrderedMap<&'static str, (usize, usize)>> = phf::phf_ordered_map! {",
            ),
        }
    }
    pub fn add_braces(&mut self) {
        self.champion_name_to_id.push_str("};");
        self.champion_id_to_name.push_str("};");
        self.champion_formulas.push_str("};");
        self.champion_generator.push_str("};");
        self.champion_abilities.push_str("};");
    }
    pub fn join_fields(self) -> String {
        let mut s = String::with_capacity(
            self.champion_name_to_id.len()
                + self.champion_id_to_name.len()
                + self.champion_formulas.len()
                + self.champion_generator.len()
                + self.champion_abilities.len(),
        );
        s.push_str(&self.champion_name_to_id);
        s.push_str(&self.champion_id_to_name);
        s.push_str(&self.champion_formulas);
        s.push_str(&self.champion_generator);
        s.push_str(&self.champion_abilities);
        s
    }
}

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

#[derive(Copy, Clone, Deserialize, Default)]
pub enum Attrs {
    #[default]
    None,
    OnhitMax,
    OnhitMin,
    Onhit,
}

impl Attrs {
    pub fn stringify(&self) -> &'static str {
        match self {
            Attrs::None => "None",
            Attrs::OnhitMax => "OnhitMax",
            Attrs::OnhitMin => "OnhitMin",
            Attrs::Onhit => "Onhit",
        }
    }
}

#[derive(Deserialize)]
pub struct Ability {
    pub damage_type: String,
    #[serde(default)]
    pub attributes: Attrs,
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

pub fn format_stats(stats: &ChampionCdnStats) -> String {
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
        (lone $field:ident) => {
            format!("{}: {}f64,", stringify!($field), stats.$field.flat,)
        };
    }
    let mut all_stats = Vec::new();
    all_stats.push(insert_stat!(health));
    all_stats.push(insert_stat!(mana));
    all_stats.push(insert_stat!(armor));
    all_stats.push(insert_stat!(magic_resistance));
    all_stats.push(insert_stat!(attack_damage));
    all_stats.push(insert_stat!(attack_speed));
    all_stats.push(insert_stat!(lone movespeed));
    all_stats.push(insert_stat!(lone critical_strike_damage));
    all_stats.push(insert_stat!(lone critical_strike_damage_modifier));
    all_stats.push(insert_stat!(lone attack_speed_ratio));
    all_stats.push(insert_stat!(lone attack_range));
    all_stats.push(insert_stat!(lone aram_damage_taken));
    all_stats.push(insert_stat!(lone aram_damage_dealt));
    all_stats.push(insert_stat!(lone urf_damage_taken));
    all_stats.push(insert_stat!(lone urf_damage_dealt));
    all_stats.join("")
}

fn sort_pqwer<T: Ord>(data: &mut Vec<(String, T)>) {
    let priority = |ch: char| match ch {
        'P' => 0,
        'Q' => 1,
        'W' => 2,
        'E' => 3,
        'R' => 4,
        _ => 5,
    };

    data.sort_by(|a, b| {
        let a_first = a.0.chars().next().unwrap_or('Z');
        let b_first = b.0.chars().next().unwrap_or('Z');
        let ord1 = priority(a_first).cmp(&priority(b_first));
        if ord1 != Ordering::Equal {
            return ord1;
        }

        let ord2 = a.0.cmp(&b.0);
        if ord2 != Ordering::Equal {
            return ord2;
        }
        a.1.cmp(&b.1)
    });
}

fn format_abilities(abilities: &HashMap<String, Ability>) -> Vec<(String, String)> {
    let mut formatted_map = Vec::new();
    for (name, ability) in abilities {
        let mut min_dmg = String::new();
        let mut max_dmg = String::new();
        macro_rules! format_dmg {
            ($var:expr, $field:ident) => {
                if ability.$field.is_empty() {
                    $var.push_str("zero");
                } else {
                    let transformed: Vec<(String, bool)> = ability
                        .$field
                        .iter()
                        .map(|dmg| {
                            let expr = clean_math_expr(&dmg);
                            transform_expr(&expr)
                        })
                        .collect();
                    let needs_ctx = transformed.iter().any(|&(_, changed)| changed);
                    let ctx_param = if needs_ctx { "ctx" } else { "_" };
                    $var.push_str(&format!("|level, {}| {{match level {{", ctx_param));
                    for (i, (expr, _)) in transformed.into_iter().enumerate() {
                        $var.push_str(&format!("{} => {},", i + 1, expr.to_lowercase()));
                    }
                    $var.push_str("_ => 0.0,}}");
                }
            };
        }
        format_dmg!(min_dmg, minimum_damage);
        format_dmg!(max_dmg, maximum_damage);
        formatted_map.push((
            name.clone(),
            format!(
                "static __phantom__: CachedChampionAbility = CachedChampionAbility {{damage_type: {},
                attributes: Attrs::{},minimum_damage: {},
                maximum_damage: {},}};",
                format_damage_type(&ability.damage_type), ability.attributes.stringify(), min_dmg, max_dmg,
            )
        ));
    }
    sort_pqwer(&mut formatted_map);
    formatted_map
}

pub fn export_champions(mega_block: &mut String) {
    let main_map = init_map!(dir Champion, "internal/champions");

    let mut exported_comptime_phf = ExportedComptimePhfs::new();
    let mut phf_internal_champions = String::from(
        "pub static INTERNAL_CHAMPIONS: phf::Map<&'static str, &'static CachedChampion> = phf::phf_map! {",
    );
    let mut constdecl_phf_champions = String::new();

    struct Offsets {
        champion_formula: (usize, usize),
        champion_generator: (usize, usize),
        champion_abilities: Vec<(String, (usize, usize))>,
    }

    struct Details {
        champion_name: String,
        generator: String,
        highlighted_abilities: Vec<(String, String)>,
        champion_formula: String,
        constdecl: String,
        offsets: Offsets,
    }

    let mut results = main_map
        .into_par_iter()
        .map(|(champion_id, champion)| {
            let (highlighted_abilities, mut constdecl_abilities) =
                format_abilities(&champion.abilities)
                    .into_par_iter()
                    .filter_map(|(ability_name, ability_formula)| {
                        let rustfmt_val = invoke_rustfmt(&ability_formula, 80)
                        .replace(
                            "static __phantom__: CachedChampionAbility = CachedChampionAbility ",
                            "",
                        )
                        .replace(";", "");
                        if !rustfmt_val.is_empty() {
                            let highlighted_val = clear_suffixes(&highlight(&format!(
                                "intrinsic {}_{} = {}",
                                champion_id.to_uppercase(),
                                ability_name.to_uppercase(),
                                rustfmt_val
                            )))
                            .replacen(
                                "class=\"type\"",
                                "class=\"constant\"",
                                1,
                            );
                            Some((
                                (ability_name.clone(), highlighted_val),
                                (ability_name, rustfmt_val),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<(Vec<_>, Vec<_>)>();

            sort_pqwer(&mut constdecl_abilities);

            let constdecl = format!(
                r#"pub static {}: CachedChampion = CachedChampion {{
                adaptative_type: {},
                attack_type: {},
                positions: &[{}],
                stats: CachedChampionStats {{{}}},
                abilities: &[{}],
                }};"#,
                champion_id.to_uppercase(),
                match champion.adaptative_type.as_str() {
                    "PHYSICAL_DAMAGE" => "AdaptativeType::Physical",
                    _ => "AdaptativeType::Magic",
                },
                match champion.attack_type.as_str() {
                    "MELEE" => {
                        "AttackType::Melee"
                    }
                    _ => {
                        "AttackType::Ranged"
                    }
                },
                champion
                    .positions
                    .iter()
                    .map(|pos| {
                        match pos.as_str() {
                            "TOP" => "Position::Top",
                            "JUNGLE" => "Position::Jungle",
                            "MIDDLE" => "Position::Middle",
                            "BOTTOM" => "Position::Bottom",
                            _ => "Position::Support",
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(","),
                format_stats(&champion.stats),
                constdecl_abilities
                    .iter()
                    .map(|(ability_name, rustfmt_val)| {
                        format!(
                            "({}, CachedChampionAbility {})",
                            match ability_name.chars().next() {
                                Some('P') => {
                                    AbilityLike::from_str_p(&ability_name).to_string()
                                }
                                Some('Q') => {
                                    AbilityLike::from_str_q(&ability_name).to_string()
                                }
                                Some('W') => {
                                    AbilityLike::from_str_w(&ability_name).to_string()
                                }
                                Some('E') => {
                                    AbilityLike::from_str_e(&ability_name).to_string()
                                }
                                Some('R') => {
                                    AbilityLike::from_str_r(&ability_name).to_string()
                                }
                                _ => ability_name.clone(),
                            },
                            rustfmt_val
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            );

            (
                champion_id.clone(),
                Details {
                    champion_name: champion.name.to_string(),
                    champion_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 60))),
                    generator: highlight(&invoke_rustfmt(
                        &fs::read_to_string(format!("src/generators/{}.rs", champion_id)).unwrap(),
                        80,
                    )),
                    offsets: Offsets {
                        champion_formula: (0, 0),
                        champion_generator: (0, 0),
                        champion_abilities: highlighted_abilities
                            .iter()
                            .map(|(name, _)| (name.clone(), (0, 0)))
                            .collect(),
                    },
                    highlighted_abilities,
                    constdecl,
                },
            )
        })
        .collect::<BTreeMap<String, Details>>();

    let mut current_offset = 0usize;

    for (_, detail) in results.iter_mut() {
        let formula_start = current_offset;
        mega_block.push_str(&detail.champion_formula);
        let formula_end = current_offset + detail.champion_formula.len();
        detail.offsets.champion_formula = (formula_start, formula_end);
        current_offset = formula_end;

        let generator_start = current_offset;
        mega_block.push_str(&detail.generator);
        let generator_end = current_offset + detail.generator.len();
        detail.offsets.champion_generator = (generator_start, generator_end);
        current_offset = generator_end;

        for (ability_name, ability_formula) in &detail.highlighted_abilities {
            let ability_start = current_offset;
            mega_block.push_str(ability_formula);
            let ability_end = current_offset + ability_formula.len();
            detail
                .offsets
                .champion_abilities
                .iter_mut()
                .find(|(name, _)| name == ability_name)
                .unwrap()
                .1 = (ability_start, ability_end);
            current_offset = ability_end;
        }
    }

    for (champion_id, mut detail) in results {
        exported_comptime_phf.champion_name_to_id.push_str(&format!(
            "\"{}\" => \"{}\",",
            detail.champion_name, champion_id
        ));

        exported_comptime_phf.champion_id_to_name.push_str(&format!(
            "\"{}\" => \"{}\",",
            champion_id, detail.champion_name
        ));

        exported_comptime_phf.champion_formulas.push_str(&format!(
            "\"{}\" => ({}, {}),",
            champion_id, detail.offsets.champion_formula.0, detail.offsets.champion_formula.1
        ));

        phf_internal_champions.push_str(&format!(
            "\"{}\" => &{},",
            champion_id,
            champion_id.to_uppercase()
        ));

        exported_comptime_phf.champion_generator.push_str(&format!(
            "\"{}\" => ({}, {}),",
            champion_id, detail.offsets.champion_generator.0, detail.offsets.champion_generator.1
        ));

        exported_comptime_phf
            .champion_abilities
            .push_str(&format!("\"{}\" => &phf::phf_ordered_map! {{", champion_id));

        sort_pqwer(&mut detail.highlighted_abilities);

        exported_comptime_phf.champion_abilities.push_str(
            &detail
                .highlighted_abilities
                .iter()
                .map(|(ability_name, _)| {
                    format!(
                        "\"{}\" => ({}, {}),",
                        ability_name,
                        detail
                            .offsets
                            .champion_abilities
                            .iter()
                            .find(|(name, _)| name == ability_name)
                            .unwrap()
                            .1
                            .0,
                        detail
                            .offsets
                            .champion_abilities
                            .iter()
                            .find(|(name, _)| name == ability_name)
                            .unwrap()
                            .1
                            .1
                    )
                })
                .collect::<Vec<String>>()
                .join(""),
        );

        exported_comptime_phf.champion_abilities.push_str("},");
        constdecl_phf_champions.push_str(&detail.constdecl);
    }

    exported_comptime_phf.add_braces();
    phf_internal_champions.push_str("};");

    let _ = fs::write("internal_comptime/src/data/champions.rs", {
        let mut s = String::with_capacity(
            USE_SUPER.len() + constdecl_phf_champions.len() + phf_internal_champions.len(),
        );
        s.push_str(USE_SUPER);
        s.push_str(&constdecl_phf_champions);
        s.push_str(&phf_internal_champions);
        s.push_str(&BASIC_ATTACK);
        s.push_str(&CRITICAL_STRIKE);
        s
    });
    let _ = fs::write(
        "internal_comptime/src/data/names.rs",
        exported_comptime_phf
            .champion_name_to_id
            .clone()
            .replace("OrderedMap", "Map")
            .replace("phf_ordered_map!", "phf_map!"),
    );
    let _ = fs::write(
        "comptime_exports/champions.txt",
        exported_comptime_phf.join_fields().as_bytes(),
    );
}
