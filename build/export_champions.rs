use super::*;
use shared_types::AbilityLike;
use std::cmp::Ordering;

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

pub fn sort_pqwer<T: Ord>(data: &mut Vec<(String, T)>) {
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

pub struct ChampionDetails {
    pub champion_name: String,
    pub generator: String,
    pub highlighted_abilities: Vec<(String, String)>,
    pub champion_formula: String,
    pub constdecl: String,
}

pub fn export_champions() -> BTreeMap<String, ChampionDetails> {
    init_map!(dir Champion, "internal/champions")
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
                            AbilityLike::from_str(ability_name),
                            rustfmt_val
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            );

            (
                champion_id.clone(),
                ChampionDetails {
                    champion_name: champion.name.to_string(),
                    champion_formula: highlight(&clear_suffixes(&invoke_rustfmt(&constdecl, 70))),
                    generator: highlight(&invoke_rustfmt(
                        &fs::read_to_string(format!("src/generators/{}.rs", champion_id)).unwrap(),
                        80,
                    )),
                    highlighted_abilities,
                    constdecl,
                },
            )
        })
        .collect::<BTreeMap<String, ChampionDetails>>()
}
