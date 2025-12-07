use serde::de::DeserializeOwned;
use tokio::sync::{Mutex, Semaphore};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

use super::*;
use std::{cmp::Ordering, path::Path, sync::Arc};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStatMap {
    pub flat: f64,
    pub per_level: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerakiChampionStats {
    pub health: MerakiChampionStatMap,
    pub mana: MerakiChampionStatMap,
    pub armor: MerakiChampionStatMap,
    #[serde(rename = "magicResistance")]
    pub magic_resist: MerakiChampionStatMap,
    pub attack_damage: MerakiChampionStatMap,
    pub attack_speed: MerakiChampionStatMap,
    pub movespeed: MerakiChampionStatMap,
    pub critical_strike_damage: MerakiChampionStatMap,
    pub critical_strike_damage_modifier: MerakiChampionStatMap,
    pub attack_speed_ratio: MerakiChampionStatMap,
    pub attack_range: MerakiChampionStatMap,
    pub aram_damage_taken: MerakiChampionStatMap,
    pub aram_damage_dealt: MerakiChampionStatMap,
    pub urf_damage_taken: MerakiChampionStatMap,
    pub urf_damage_dealt: MerakiChampionStatMap,
}

#[derive(Copy, Clone, Debug, Deserialize, Default)]
pub enum Attrs {
    #[default]
    Undefined,
    Onhit,
    OnhitMin,
    OnhitMax,
    Area,
    AreaOnhit,
    AreaOnhitMin,
    AreaOnhitMax,
}

#[derive(Deserialize)]
pub struct Ability {
    pub name: String,
    pub damage_type: String,
    #[serde(default)]
    pub attributes: Attrs,
    pub damage: Vec<String>,
}

#[derive(Deserialize)]
pub struct Champion {
    pub name: String,
    pub adaptative_type: String,
    pub attack_type: String,
    pub positions: Vec<String>,
    pub stats: MerakiChampionStats,
    pub abilities: Vec<(AbilityLike, Ability)>,
    pub merge_data: Vec<(AbilityLike, AbilityLike)>,
}

pub fn format_stats(stats: &MerakiChampionStats) -> String {
    macro_rules! insert_stat {
        ($field:ident) => {
            format!(
                "{}:CachedChampionStatsMap{{flat:{}f32,per_level:{}f32}},",
                stringify!($field),
                stats.$field.flat,
                stats.$field.per_level
            )
        };
        (lone $field:ident) => {
            format!("{}:{}f32,", stringify!($field), stats.$field.flat)
        };
    }
    let mut all_stats = Vec::new();
    all_stats.push(insert_stat!(health));
    all_stats.push(insert_stat!(mana));
    all_stats.push(insert_stat!(armor));
    all_stats.push(insert_stat!(magic_resist));
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

pub fn sort_pqwer(data: &mut [(AbilityLike, String, String, String)]) {
    let priority = |ch| match ch {
        'P' => 0,
        'Q' => 1,
        'W' => 2,
        'E' => 3,
        'R' => 4,
        _ => 5,
    };

    data.sort_by(|a, b| {
        let a_first = a.0.as_char();
        let b_first = b.0.as_char();
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

struct DeclaredAbility {
    declaration: String,
    metadata: String,
    damage: String,
    name: String,
}

fn declare_abilities(upper_id: &str, abilities: &[(AbilityLike, Ability)]) -> Vec<DeclaredAbility> {
    abilities
        .into_iter()
        .map(|(name, ability)| {
            let damage = match ability.damage.is_empty() {
                true => String::from("zero"),
                false => {
                    let mut result = String::new();
                    result
                }
            };

            DeclaredAbility {
                declaration: String::new(),
                metadata: String::new(),
                damage: String::new(),
                name: String::new(),
            }
        })
        .collect()
}

fn get_abilities_decl(
    champion_name: &str,
    abilities: Vec<(AbilityLike, Ability)>,
) -> Vec<(AbilityLike, String, String, String)> {
    let mut result = Vec::new();

    for (name, ability) in abilities {
        let damage = if ability.damage.is_empty() {
            String::from("zero")
        } else {
            let mut target = String::new();
            let transformed = ability
                .damage
                .iter()
                .map(|dmg| dmg.clean_math_expr().transform_expr())
                .collect::<Vec<(_, bool)>>();
            let ability_type = name.as_char();
            let ctx_matcher = match ability_type {
                'P' => "level as u8".into(),
                _ => format!("{}_level", ability_type.to_lowercase()),
            };
            target.push_str(&format!("|ctx| {{ match ctx.{ctx_matcher} {{"));
            for (i, (new_expr, _)) in transformed.into_iter().enumerate() {
                target.push_str(&format!("{} => {},", i + 1, new_expr.to_lowercase()));
            }
            target.push_str("_ => 0.0 }}");
            target
        };

        let const_decl = format!(
            "static {champion_name}_{char_name}: Intrinsic = Intrinsic {{
                name: {ability_name:?},
                damage_type: DamageType::{damage_type},
                attributes: Attrs::{attributes:?},
                damage: {damage},
            }};",
            char_name = format!(
                "{char_disc}{name_disc}",
                char_disc = name.as_char(),
                name_disc = {
                    let ability_name = format!("{:?}", name.ability_name());
                    if !ability_name.starts_with("_") {
                        format!("_{ability_name}")
                    } else {
                        ability_name
                    }
                }
            )
            .to_uppercase(),
            ability_name = ability.name,
            damage_type = ability.damage_type,
            attributes = ability.attributes,
        );

        let metadata = format!(
            "TypeMetadata {{ 
                kind: {kind}, 
                damage_type: DamageType::{damage_type}, 
                attributes: Attrs::{attributes:?} 
            }}",
            kind = name.as_literal(),
            damage_type = ability.damage_type,
            attributes = ability.attributes
        );

        result.push((name, metadata, damage, const_decl));
    }

    result
}

pub struct ChampionDetails {
    pub champion_name: String,
    pub generator: String,
    pub highlighted_abilities: Vec<(AbilityLike, String)>,
    pub combos: Vec<Vec<AbilityLike>>,
    pub champion_formula: String,
    pub constdecl: String,
    pub positions: String,
}

pub fn find_merge_indexes(
    merge_data: &[(AbilityLike, AbilityLike)],
    ability_data: &[(AbilityLike, String, String, String)],
) -> Vec<(usize, usize)> {
    let mut idx: HashMap<AbilityLike, usize> = HashMap::with_capacity(ability_data.len());
    for (i, (al, ..)) in ability_data.iter().enumerate() {
        idx.entry(*al).or_insert(i);
    }

    let mut out = Vec::with_capacity(merge_data.len());
    for &(a, b) in merge_data {
        if let (Some(&ia), Some(&ib)) = (idx.get(&a), idx.get(&b)) {
            out.push((ia, ib));
        }
    }
    out
}

pub type MayFail<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub async fn _parallel<P, F, T, R, Fut>(
    permits: usize,
    path: P,
    f: F,
) -> MayFail<BTreeMap<String, R>>
where
    P: AsRef<Path>,
    R: Send + Sync + 'static,
    T: DeserializeOwned,
    Fut: Future<Output = MayFail<R>> + Send + Sync + 'static,
    F: Clone + Send + Sync + 'static + Fn(String, T) -> Fut,
{
    let result = Arc::new(Mutex::new(BTreeMap::new()));
    let mut futures = Vec::new();
    let semaphore = Arc::new(Semaphore::new(permits));

    let folder = Path::new("../").join(path);
    let read_dir = tokio::fs::read_dir(folder).await?;
    let mut dir_stream = ReadDirStream::new(read_dir);

    while let Some(Ok(entry)) = dir_stream.next().await {
        let f = f.clone();
        let result = result.clone();
        let semaphore = semaphore.clone();
        futures.push(tokio::task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let file_name = entry.file_name().to_string_lossy().to_string();
            let handler = {
                let file_name = file_name.clone();
                async move || -> MayFail<R> {
                    let data = tokio::fs::read(entry.path()).await?;
                    let json = serde_json::from_slice(&data)?;
                    f(file_name, json).await
                }
            };

            let data = handler().await.unwrap();
            result.lock().await.insert(file_name, data);
        }))
    }

    for future in futures {
        future.await.unwrap();
    }

    let response = unsafe { Arc::try_unwrap(result).unwrap_unchecked() };
    Ok(response.into_inner())
}

pub async fn _test() {
    let data = _parallel(
        64,
        "internal/champions",
        async |champion_id, json: Champion| {
            let upper_id = champion_id.to_uppercase();
            Ok(upper_id)
        },
    )
    .await;
}

pub fn export_champions() -> BTreeMap<String, ChampionDetails> {
    init_map!(dir Champion, "internal/champions")
        .into_par_iter()
        .map(|(champion_id, champion)| {
            let champion_name_upper = champion_id.to_uppercase();
            let mut ability_data = get_abilities_decl(&champion_name_upper, champion.abilities)
                .into_par_iter()
                .filter_map(|(name, metadata, closures, ability_decl)| {
                    let rustfmt_val = ability_decl.invoke_rustfmt(80);
                    (!rustfmt_val.is_empty()).then_some((
                        name,
                        metadata,
                        closures,
                        rustfmt_val
                            .highlight_rust()
                            .clear_suffixes()
                            .replace_const(),
                    ))
                })
                .collect::<Vec<(AbilityLike, String, String, String)>>();

            sort_pqwer(&mut ability_data);

            let positions = champion
                .positions
                .into_iter()
                .map(|pos| format!("Position::{pos}"))
                .collect::<Vec<String>>()
                .join(",");

            let constdecl = format!(
                "pub static {champion_name_upper}: CachedChampion = CachedChampion {{
                    name: {true_champion_name:?},
                    adaptative_type: AdaptativeType::{adaptative_type},
                    attack_type: AttackType::{attack_type},
                    positions: &[{positions}],
                    metadata: &[{metadata}],
                    closures: &[{closures}],
                    stats: CachedChampionStats {{{stats}}},
                    merge_data: &[{merge_data}],
                }};",
                true_champion_name = champion.name,
                adaptative_type = champion.adaptative_type,
                attack_type = champion.attack_type,
                metadata = ability_data
                    .iter()
                    .map(|(_, metadata, ..)| metadata)
                    .collect::<Vec<_>>()
                    .join(","),
                closures = ability_data
                    .iter()
                    .map(|(_, _, closures, _)| closures)
                    .collect::<Vec<_>>()
                    .join(","),
                stats = format_stats(&champion.stats),
                merge_data = find_merge_indexes(&champion.merge_data, &ability_data)
                    .iter()
                    .map(|(ia, ib)| format!("({ia}, {ib})"))
                    .collect::<Vec<_>>()
                    .join(","),
            );

            let generator = format!("tutorlolv2_dev/src/generators/gen_champions/{champion_id}.rs")
                .read_as_path()
                .invoke_rustfmt(80)
                .highlight_rust();

            let raw_combos = serde_json::from_str::<Vec<Vec<String>>>(
                &format!("internal/scraper/combos/{champion_id}.json").read_as_path(),
            )
            .unwrap();

            let mut champion_combos = Vec::<Vec<AbilityLike>>::new();

            let ability_names = ability_data
                .iter()
                .map(|(name, ..)| name)
                .collect::<Vec<_>>();
            for combos in raw_combos {
                let mut result = Vec::new();
                for combo in combos {
                    for &&ability_name in &ability_names {
                        if combo == ability_name.as_char().to_string() {
                            result.push(ability_name);
                        }
                    }
                }
                champion_combos.push(result);
            }

            (
                champion_id,
                ChampionDetails {
                    champion_name: champion.name.to_string(),
                    champion_formula: constdecl
                        .invoke_rustfmt(80)
                        .clear_suffixes()
                        .highlight_rust(),
                    generator,
                    combos: champion_combos,
                    highlighted_abilities: ability_data
                        .into_iter()
                        .map(|(name, _, _, ability_decl)| (name, ability_decl))
                        .collect(),
                    constdecl,
                    positions,
                },
            )
        })
        .collect()
}
