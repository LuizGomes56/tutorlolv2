use crate::{
    _lib::{CwdPath, Generated, GeneratorFn, SrcFolder, parallel_task, push_end},
    scripts::{Ability, Champion, MerakiChampionStats, StringExt, USE_SUPER},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{cmp::Ordering, collections::HashMap};
use tutorlolv2_types::AbilityLike;

struct DeclaredAbility {
    ability_id: AbilityLike,
    declaration: String,
    metadata: String,
    damage: String,
}

fn declare_abilities(
    champion_id_upper: &str,
    abilities: Vec<(AbilityLike, Ability)>,
) -> Vec<DeclaredAbility> {
    abilities
        .into_par_iter()
        .map(|(ability_id, ability)| {
            let letter = ability_id.as_char();

            let Ability {
                name,
                damage_type,
                attributes,
                damage,
            } = ability;

            let damage = match damage.is_empty() {
                true => String::from("zero"),
                false => {
                    let mut result = String::new();

                    let expression = damage
                        .iter()
                        .map(|dmg| dmg.clean_math_expr().transform_expr().0)
                        .collect::<Vec<_>>();

                    let ctx_match = match letter {
                        'P' => "level as u8".into(),
                        _ => format!("{letter}_level", letter = letter.to_lowercase()),
                    };

                    result.push_str(&format!("|ctx| {{ match ctx.{ctx_match} {{"));

                    let arms = expression
                        .into_iter()
                        .enumerate()
                        .map(|(i, expr)| format!("{level} => {expr}", level = i + 1))
                        .collect::<Vec<_>>()
                        .join(",");

                    result.push_str(&format!("{arms}, _ => 0.0 }}}}"));
                    result
                }
            };

            let discriminant = format!(
                "{letter}_{ability_name}",
                ability_name = format!("{:?}", ability_id.ability_name()).trim_start_matches('_')
            )
            .to_uppercase();

            let damage_type = format_args!("DamageType::{damage_type}");
            let attributes = format_args!("Attrs::{attributes:?}");

            let declaration = format!(
                "static {champion_id_upper}_{discriminant}: Intrinsic = Intrinsic {{
                    name: {name:?},
                    damage_type: {damage_type},
                    attributes: {attributes},
                    damage: {damage},
                }};"
            )
            .invoke_rustfmt(80)
            .highlight_rust()
            .clear_suffixes()
            .replace_const();

            if declaration.is_empty() {
                panic!("[{champion_id_upper}] Empty declaration for {ability_id:?}");
            }

            let metadata = format!(
                "TypeMetadata {{ 
                    kind: {kind}, 
                    damage_type: {damage_type}, 
                    attributes: {attributes} 
                }}",
                kind = ability_id.as_literal(),
            );

            DeclaredAbility {
                ability_id,
                declaration,
                metadata,
                damage,
            }
        })
        .collect()
}

pub fn get_stats(stats: &MerakiChampionStats) -> String {
    let mut all_stats = Vec::new();
    macro_rules! insert_stats {
        (
            full => [ $($field:ident),+$(,)* ],
            lone => [ $($lone_field:ident),+$(,)* ]
        ) => {
            $(
                all_stats.push(format!(
                    "{}: CachedChampionStatsMap {{ flat: {}f32, per_level: {}f32 }}",
                    stringify!($field),
                    stats.$field.flat,
                    stats.$field.per_level
                ));
            )*
            $(
                all_stats.push(format!("{}: {}f32", stringify!($lone_field), stats.$lone_field.flat));
            )*
        };
    }
    insert_stats!(
        full => [
            health,
            mana,
            armor,
            magic_resist,
            attack_damage,
            attack_speed,
        ],
        lone => [
            movespeed,
            critical_strike_damage,
            critical_strike_damage_modifier,
            attack_speed_ratio,
            attack_range,
            aram_damage_taken,
            aram_damage_dealt,
            urf_damage_taken,
            urf_damage_dealt,
        ]
    );
    all_stats.join(",")
}

pub fn define_merge_indexes(
    merge_data: Vec<(AbilityLike, AbilityLike)>,
    ability_data: &[AbilityLike],
) -> String {
    let mut index = HashMap::with_capacity(ability_data.len());
    for (i, ability_id) in ability_data.into_iter().enumerate() {
        index.entry(ability_id).or_insert(i);
    }

    merge_data
        .into_iter()
        .filter_map(|(a, b)| match (index.get(&a), index.get(&b)) {
            (Some(ia), Some(ib)) => Some(format!("({ia}, {ib})")),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn sort_abilities(data: &mut [DeclaredAbility]) {
    let priority = |ability_id: AbilityLike| match ability_id.as_char() {
        'P' => 0,
        'Q' => 1,
        'W' => 2,
        'E' => 3,
        'R' => 4,
        _ => 5,
    };

    data.sort_by(|a, b| {
        let a_id = a.ability_id;
        let b_id = b.ability_id;
        let a_priority = priority(a_id);
        let b_priority = priority(b_id);
        let a_ord = a_priority.cmp(&b_priority);
        if a_ord != Ordering::Equal {
            return a_ord;
        }
        let b_ord = a_id.cmp(&b_id);
        if b_ord != Ordering::Equal {
            return b_ord;
        }
        a.metadata.cmp(&b.metadata)
    });
}

pub async fn generate_champions() -> GeneratorFn {
    struct ChampionResult {
        champion_id_upper: String,
        name: String,
        positions: String,
        declaration: String,
        ability_declarations: Vec<(AbilityLike, String)>,
        generator: String,
    }

    let data = parallel_task(
        64,
        "internal/champions",
        async |champion_id, champion: Champion| {
            println!("Building: ChampionId({champion_id:?})");

            let champion_id_upper = champion_id.to_uppercase();

            let Champion {
                adaptative_type,
                attack_type,
                merge_data,
                abilities,
                positions,
                stats,
                name,
            } = champion;

            let mut abilities = declare_abilities(&champion_id_upper, abilities);

            sort_abilities(&mut abilities);

            let mut closures = String::new();
            let mut metadata = String::new();
            let mut ability_names = Vec::new();
            let mut ability_declarations = Vec::new();

            for ability in abilities {
                closures.push_str(&ability.damage);
                metadata.push_str(&ability.metadata);
                closures.push(',');
                metadata.push(',');
                ability_names.push(ability.ability_id);
                ability_declarations.push((ability.ability_id, ability.declaration));
            }

            let positions = positions
                .into_iter()
                .map(|pos| format!("Position::{pos}"))
                .collect::<Vec<_>>()
                .join(",");

            let declaration = format!(
                "pub static {champion_id_upper}: CachedChampion = CachedChampion {{
                    name: {name:?},
                    adaptative_type: AdaptativeType::{adaptative_type},
                    attack_type: AttackType::{attack_type},
                    positions: &[{positions}],
                    metadata: &[{metadata}],
                    closures: &[{closures}],
                    stats: CachedChampionStats {{{stats}}},
                    merge_data: &[{merge_data}],
                }};",
                stats = get_stats(&stats),
                merge_data = define_merge_indexes(merge_data, &ability_names),
            );

            let generator = CwdPath::get_generator(SrcFolder::Champions, &champion_id).await?;

            let combos_json = CwdPath::deserialize::<Vec<Vec<String>>>(format!(
                "internal/scraper/combos/{champion_id}.json"
            ))
            .await?;

            let mut champion_combos = Vec::<Vec<AbilityLike>>::new();

            for combos in combos_json {
                let mut result = Vec::new();
                for combo in combos {
                    for &ability_name in &ability_names {
                        if combo == ability_name.as_char().to_string() {
                            result.push(ability_name);
                        }
                    }
                }
                champion_combos.push(result);
            }

            let positions = format!("&[{positions}]");
            let champion_name = format!("{name:?}");
            let champion_id = format!("{champion_id_upper}");

            Ok(ChampionResult {
                ability_declarations,
                name: champion_name,
                champion_id_upper: champion_id,
                declaration,
                positions,
                generator,
            })
        },
    )
    .await?;

    let len = data.len();

    let [
        mut champion_cache,
        mut champion_positions,
        mut champion_id_to_name,
        mut champion_formulas,
        mut champion_generator,
        mut champion_abilities,
    ] = std::array::from_fn(|i| {
        let (name, vtype) = [
            ("CHAMPION_CACHE", "&CachedChampion"),
            ("CHAMPION_POSITIONS", "&[Position]"),
            ("CHAMPION_ID_TO_NAME", "&str"),
            ("CHAMPION_FORMULAS", "(u32,u32)"),
            ("CHAMPION_GENERATOR", "(u32,u32)"),
            ("CHAMPION_ABILITIES", "&[(AbilityLike,(u32,u32))]"),
        ][i];
        format!("pub static {name}: [{vtype}; {len}] = [")
    });

    let mut offset = 0;
    let mut formula_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);
    let mut ability_offsets = Vec::with_capacity(len);

    let mut block = String::new();

    let mut record_offsets = |into: &mut Vec<_>, value: &str| {
        let start = offset;
        let end = offset + value.len();
        block.push_str(value);
        into.push((start, end));
        offset = end;
    };

    let mut champion_declarations = String::new();

    let mut champion_id_enum = format!(
        "impl ChampionId {{
            pub const unsafe fn from_u8_unchecked(id: u8) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u8(id: u8) -> Option<Self> {{
                if id < {len} as u8 {{
                    Some(unsafe {{ Self::from_u8_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}
        #[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Decode, Encode)]
        #[repr(u8)]
        pub enum ChampionId {{",
    );

    let languages_map =
        CwdPath::deserialize::<HashMap<String, Vec<String>>>("internal/champion_languages.json")
            .await?;
    let mut language_arms = Vec::new();

    for (
        champion_id,
        ChampionResult {
            declaration,
            name: champion_name,
            positions,
            ability_declarations,
            generator,
            champion_id_upper,
        },
    ) in data
    {
        let normalized_id = champion_id.remove_special_chars();
        let name_alias = languages_map
            .get(&champion_id)
            .ok_or(format!(
                "Failed to recover {champion_id:?} from languages map"
            ))?
            .into_iter()
            .map(|alias| format!("{alias:?} => ChampionId::{normalized_id}"))
            .collect::<Vec<String>>()
            .join(",");
        language_arms.push(name_alias);

        champion_declarations.push_str(&declaration);
        champion_id_to_name.push_str(&format!("{champion_name},"));
        champion_positions.push_str(&(positions + ","));
        champion_id_enum.push_str(&(normalized_id + ","));
        champion_cache.push_str(&format!("&{champion_id_upper},"));

        record_offsets(&mut generator_offsets, &generator);
        record_offsets(
            &mut formula_offsets,
            &declaration
                .invoke_rustfmt(80)
                .clear_suffixes()
                .highlight_rust(),
        );

        for (_, formula) in ability_declarations {
            record_offsets(&mut ability_offsets, &formula);
        }
    }

    let champion_name_to_id = format!(
        "pub static CHAMPION_NAME_TO_ID: phf::Map<&str, ChampionId> = phf::phf_map! {{
            {arms}
        }};",
        arms = language_arms.join(",")
    );

    push_end([&mut champion_id_enum], "}");
    push_end(
        [
            &mut champion_cache,
            &mut champion_positions,
            &mut champion_id_to_name,
        ],
        "];",
    );

    let imports = [
        USE_SUPER,
        &champion_id_enum,
        &champion_declarations,
        &champion_name_to_id,
        &champion_cache,
    ]
    .concat();
    CwdPath::fwrite(SrcFolder::Champions.import_file(), imports).await??;

    let callback = move |index: usize| {
        let add_offsets = |list: Vec<_>, target: &mut String| {
            for (start, end) in list {
                let new_start = start + index;
                let new_end = end + index;
                target.push_str(&format!("({new_start}, {new_end}),"));
            }
            push_end([target], "];");
        };

        for (list, target) in [
            (ability_offsets, &mut champion_abilities),
            (generator_offsets, &mut champion_generator),
            (formula_offsets, &mut champion_formulas),
        ] {
            add_offsets(list, target);
        }

        let exports = [
            champion_id_enum,
            champion_id_to_name,
            champion_positions,
            champion_generator,
            champion_abilities,
            champion_formulas,
        ]
        .concat();

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
