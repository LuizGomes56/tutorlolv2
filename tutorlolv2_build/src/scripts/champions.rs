use crate::{
    parallel_task, push_end,
    scripts::{
        model::{Ability, Champion, MerakiChampionStatMap, MerakiChampionStats},
        simplify, Simplified, StringExt,
    },
    CwdPath, Generated, GeneratorFn, MayFail, SrcFolder, Tracker, EVAL_FEAT, GLOB_FEAT,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
};
use tutorlolv2_types::{AbilityId, DevMergeData};

struct DeclaredAbility {
    ability_id: AbilityId,
    declaration: String,
    metadata: String,
    constfn: ConstFn,
    html_damage: String,
    idents: HashSet<String>,
}

fn declare_abilities(
    champion_id_upper: &str,
    abilities: Vec<(AbilityId, Ability)>,
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

            let match_src = format!("{letter}_level", letter = letter.to_lowercase());

            let (damage, html_damage) = match damage.is_empty() {
                true => ("zero".into(), "zero".into()),
                false => {
                    let comparator = format!(
                        "ctx.{op}",
                        op = match letter {
                            'P' => "level",
                            _ => &match_src,
                        }
                    );
                    let match_closure = format!("|ctx| {{ match {comparator} as u8 {{");

                    let get_arms = |src: &[String]| {
                        src.into_iter()
                            .enumerate()
                            .map(|(i, value)| {
                                let expr = value.clean();
                                let level = i + 1;
                                format!("{level} => {expr},")
                            })
                            .collect::<String>()
                    };

                    let simplified = simplify(&damage);

                    let get_closure = |arm| format!("{match_closure} {arm} {simplified} }}}}");
                    let html_closure = get_closure(get_arms(&damage)).replace("context_level", "n");

                    let closure = match simplified {
                        Simplified::Progression(_) | Simplified::Independent(_) => {
                            let body = simplified
                                .expr()
                                .clean()
                                .cast_f32()
                                .replace("context_level", &comparator);
                            format!("|ctx| {{ {body} }}")
                        }
                        _ => {
                            let damage_f32 = damage
                                .iter()
                                .map(|expr| expr.cast_f32())
                                .collect::<Vec<_>>();

                            get_closure(get_arms(&damage_f32))
                        }
                    };

                    (closure, html_closure)
                }
            };

            let discriminant = format!(
                "{letter}_{ability_name}",
                ability_name = format!("{:?}", ability_id.ability_name()).trim_start_matches('_')
            )
            .to_uppercase();

            let constfn_name = format!(
                "{champion_id}_{discriminant}",
                champion_id = champion_id_upper.to_ssnake()
            )
            .to_lowercase();

            let constfn_declaration = format!(
                "{EVAL_FEAT} pub const fn {constfn_name}(ctx: &Ctx) -> f32 {body}",
                body = damage.trim_start_matches("|ctx|")
            );

            let damage_type = format_args!("DamageType::{damage_type}");
            let attributes = format_args!("Attrs::{attributes:?}");

            let declaration = format!(
                "static {champion_id_upper}_{discriminant}: Ability = Ability {{
                    name: {name:?},
                    damage_type: {damage_type},
                    attributes: {attributes},
                    damage: {html_damage},
                }};"
            )
            .rust_fmt()
            .rust_html()
            .as_const();

            if declaration.is_empty() {
                panic!("[{champion_id_upper}] Empty declaration for {ability_id:?}");
            }

            let kind = ability_id.as_literal();

            let metadata = format!(
                "TypeMetadata {{ 
                    kind: {kind}, 
                    damage_type: {damage_type}, 
                    attributes: {attributes} 
                }}"
            );

            let idents = damage.get_idents();

            DeclaredAbility {
                ability_id,
                declaration,
                metadata,
                constfn: ConstFn {
                    ability_id: kind,
                    declaration: constfn_declaration,
                    name: constfn_name,
                },
                html_damage,
                idents,
            }
        })
        .collect()
}

fn get_stats(stats: &MerakiChampionStats) -> String {
    const NUMBER_OF_STATS: usize =
        size_of::<MerakiChampionStats>() / size_of::<MerakiChampionStatMap>();
    let mut all_stats = Vec::with_capacity(NUMBER_OF_STATS);

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

fn define_merge_indexes(merge_data: Vec<DevMergeData>, ability_data: &[AbilityId]) -> String {
    let mut index = HashMap::with_capacity(ability_data.len());
    for (i, ability_id) in ability_data.into_iter().enumerate() {
        index.entry(ability_id).or_insert(i);
    }

    merge_data
        .into_iter()
        .filter_map(|value| {
            let DevMergeData {
                minimum_damage,
                maximum_damage,
                alias,
            } = value;
            let alias = alias.as_literal();
            match (index.get(&minimum_damage), index.get(&maximum_damage)) {
                (Some(ia), Some(ib)) => Some(format!(
                    "MergeData {{
                        minimum_damage: {ia},
                        maximum_damage: {ib},
                        alias: {alias}
                    }}"
                )),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn sort_abilities(data: &mut [DeclaredAbility]) {
    let priority = |ability_id: AbilityId| match ability_id.as_char() {
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

struct ConstFn {
    ability_id: String,
    declaration: String,
    name: String,
}

struct ChampionResult {
    champion_id_upper: String,
    name: String,
    positions: String,
    base_declaration: String,
    ability_declarations: Vec<(AbilityId, String)>,
    generator: String,
    html_declaration: String,
    const_match_kind: String,
    ability_idents_index: String,
    ability_idents: String,
    damage_def: Vec<(AbilityId, String)>,
}

pub fn generate_champions() -> GeneratorFn {
    let mut data = parallel_task("internal/champions", |champion_id, champion: Champion| {
        println!("[build] ChampionId::{champion_id}");

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

        let mut constfns = Vec::with_capacity(abilities.len());
        let mut fn_names = String::new();
        let mut closures = String::new();
        let mut ability_metadata = String::new();
        let mut ability_names = Vec::with_capacity(abilities.len());
        let mut ability_declarations = Vec::with_capacity(abilities.len());

        let mut ability_idents_index = String::new();
        let mut ability_idents = String::new();
        let mut ability_index = 0;

        let mut damage_def = Vec::new();

        for DeclaredAbility {
            ability_id,
            declaration,
            metadata,
            constfn,
            html_damage,
            idents,
        } in abilities
        {
            let fn_name = &constfn.name;
            fn_names.push_str(&format!("{fn_name},"));
            ability_metadata.push_str(&format!("{metadata},"));
            closures.push_str(&format!("{fn_name}: {html_damage},"));
            ability_names.push(ability_id);
            ability_declarations.push((ability_id, declaration));

            let start = ability_index;
            ability_index += idents.len();
            ability_idents_index.push_str(&format!("{start}..{ability_index},"));
            ability_idents.push_str(&idents.into_iter().collect::<String>());
            damage_def.push((
                ability_id,
                constfn
                    .declaration
                    .trim_start_matches(&format!("{EVAL_FEAT} pub const"))
                    .trim()
                    .rust_fmt()
                    .drop_f32s()
                    .rust_html(),
            ));
            constfns.push(constfn);
        }

        let positions = positions
            .into_iter()
            .map(|pos| format!("Position::{pos}"))
            .collect::<Vec<_>>()
            .join(",");

        let rest = format!(
            "metadata: &[{ability_metadata}],
            stats: CachedChampionStats {{{stats}}},
            merge_data: &[{merge_data}]",
            stats = get_stats(&stats),
            merge_data = define_merge_indexes(merge_data, &ability_names),
        );

        let base_declaration = format!(
            "pub static {champion_id_upper}: CachedChampion = CachedChampion {{
                name: {name:?},
                adaptative_type: AdaptativeType::{adaptative_type},
                attack_type: AttackType::{attack_type},
                positions: &[{positions}],"
        );

        let html_declaration = format!("{base_declaration}{closures}{rest} }};")
            .rust_fmt()
            .drop_f32s()
            .rust_html();

        let mut base_declaration =
            format!("{EVAL_FEAT}{base_declaration}closures: &[{fn_names}], {rest} }};");

        let mut match_arm_kind = Vec::with_capacity(constfns.len());
        for ConstFn {
            ability_id,
            declaration,
            name,
        } in constfns
        {
            base_declaration.push_str(&declaration);
            match_arm_kind.push(format!("{ability_id} => {name}(ctx)"));
        }

        match_arm_kind.push(format!(
            r#"_ => panic!("Invalid AbilityId for '{champion_id}'")"#
        ));

        let const_match_kind = match_arm_kind.join(",");
        let generator = CwdPath::get_generator(SrcFolder::Champions, &champion_id)?;

        /*
        let combos_json = CwdPath::deserialize::<Vec<Vec<String>>>(format!(
            "internal/scraper/combos/{champion_id}.json"
        ))?;

        let mut champion_combos = Vec::<Vec<AbilityId>>::new();

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
        */

        Ok(ChampionResult {
            ability_declarations,
            ability_idents_index: format!("&[{ability_idents_index}]"),
            ability_idents: format!("&[{ability_idents}]"),
            const_match_kind,
            name,
            champion_id_upper,
            base_declaration,
            html_declaration,
            positions: format!("&[{positions}]"),
            damage_def,
            generator,
        })
    });

    data.sort_by(|a, b| a.0.cmp(&b.0));
    build_champions(data)
}

fn build_champions(data: Vec<(String, ChampionResult)>) -> GeneratorFn {
    let len = data.len();
    let recommendations = get_recommendations(len)?;

    let [mut champion_cache, mut champion_positions, mut champion_id_to_name, mut champion_formulas, mut champion_generator, mut champion_abilities, mut ability_ctx_idents_index, mut ability_ctx_idents, mut ability_formulas, mut ability_closures] =
        std::array::from_fn(|i| {
            let (name, vtype, feature) = [
                ("CHAMPION_CACHE", "&CachedChampion", EVAL_FEAT),
                ("CHAMPION_POSITIONS", "&[Position]", GLOB_FEAT),
                ("CHAMPION_ID_TO_NAME", "&str", GLOB_FEAT),
                ("CHAMPION_FORMULAS", "Range<usize>", GLOB_FEAT),
                ("CHAMPION_GENERATOR", "Range<usize>", GLOB_FEAT),
                ("CHAMPION_ABILITIES", "&[AbilityId]", GLOB_FEAT),
                ("ABILITY_IDENTS_INDEX", "&[Range<usize>]", GLOB_FEAT),
                ("ABILITY_IDENTS", "&[EvalIdent]", GLOB_FEAT),
                ("ABILITY_FORMULAS", "&[Range<usize>]", GLOB_FEAT),
                ("ABILITY_CLOSURES", "&[Range<usize>]", GLOB_FEAT),
            ][i];
            format!("{feature} pub static {name}: [{vtype}; ChampionId::VARIANTS] = [")
        });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);
    let mut ability_offsets = Vec::with_capacity(len);
    let mut damage_offsets = Vec::with_capacity(len);

    let mut champion_declarations = String::new();

    let mut champion_id_enum = format!(
        r#"impl ChampionId {{
            pub const VARIANTS: usize = {len};
            pub const unsafe fn from_u8_unchecked(id: u8) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u8(id: u8) -> Option<Self> {{
                if id < Self::VARIANTS as u8 {{
                    Some(unsafe {{ Self::from_u8_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum ChampionId {{"#,
    );

    let languages_map =
        CwdPath::deserialize::<HashMap<String, Vec<String>>>("internal/champion_languages.json")?;
    let mut language_arms = Vec::with_capacity(data.len());
    let mut const_match_arms = String::new();

    for (
        champion_id,
        ChampionResult {
            base_declaration,
            html_declaration,
            name,
            positions,
            const_match_kind,
            ability_declarations,
            generator,
            champion_id_upper,
            ability_idents,
            ability_idents_index,
            damage_def,
        },
    ) in data
    {
        let name_alias = languages_map
            .get(&champion_id)
            .ok_or(format!(
                "Failed to recover {champion_id:?} from languages map"
            ))?
            .into_iter()
            .map(|alias| format!("{alias:?} => ChampionId::{champion_id}"))
            .collect::<Vec<_>>()
            .join(",");
        language_arms.push(name_alias);

        champion_declarations.push_str(&base_declaration);

        let match_arm =
            format!("ChampionId::{champion_id} => {{ match kind {{ {const_match_kind} }} }}");

        const_match_arms.push_str(&match_arm);

        champion_id_to_name.push_str(&format!("{name:?},"));
        champion_positions.push_str(&(positions + ","));
        champion_id_enum.push_str(&(champion_id + ","));
        champion_cache.push_str(&format!("&{champion_id_upper},"));

        ability_ctx_idents.push_str(&(ability_idents + ","));
        ability_ctx_idents_index.push_str(&(ability_idents_index + ","));

        tracker.record_into(&generator, &mut generator_offsets);
        tracker.record_into(&html_declaration, &mut formula_offsets);

        let mut declare_offsets = |list: Vec<(AbilityId, String)>, into: &mut Vec<_>| {
            let offsets = list
                .into_iter()
                .map(|(ability_id, value)| (ability_id, tracker.record(&value)))
                .collect::<Vec<_>>();
            into.push(offsets);
        };

        declare_offsets(ability_declarations, &mut ability_offsets);
        declare_offsets(damage_def, &mut damage_offsets);
    }

    let const_eval = format!(
        "{EVAL_FEAT} pub const fn ability_const_eval(
            ctx: &Ctx, 
            champion_id: ChampionId, 
            kind: AbilityId
        ) -> f32 {{
            match champion_id {{ {const_match_arms} }}
        }}"
    );

    let champion_name_to_id = format!(
        "{EVAL_FEAT} pub static CHAMPION_NAME_TO_ID: phf::Map<&str, ChampionId> = phf::phf_map! {{
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
            &mut ability_ctx_idents_index,
            &mut ability_ctx_idents,
        ],
        "];",
    );

    let callback = move |index: usize| {
        let add_offsets = |(list, target): (Vec<_>, &mut String)| {
            for (start, end) in list {
                let new_start = start + index;
                let new_end = end + index;
                target.push_str(&format!("({new_start}..{new_end}),"));
            }
            push_end([target], "];");
        };

        [
            (generator_offsets, &mut champion_generator),
            (formula_offsets, &mut champion_formulas),
        ]
        .into_iter()
        .for_each(add_offsets);

        assert!(damage_offsets.len() == ability_offsets.len());

        for i in 0..damage_offsets.len() {
            let ability = &ability_offsets[i];
            let closure = &damage_offsets[i];

            let record_and_push = |iterable: &[_], target: &mut String| {
                let mut record = Vec::with_capacity(ability.len());
                for (_, (start, end)) in iterable {
                    let new_start = *start + index;
                    let new_end = *end + index;
                    record.push(format!("({new_start}..{new_end})"));
                }
                target.push_str(&format!("&[{result}],", result = record.join(",")));
            };

            record_and_push(ability, &mut ability_formulas);
            record_and_push(closure, &mut ability_closures);

            champion_abilities.push_str(&{
                let abilities = ability
                    .into_iter()
                    .map(|(ability_id, _)| ability_id.as_literal())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("&[{abilities}],")
            });
        }

        push_end(
            [
                &mut champion_abilities,
                &mut ability_formulas,
                &mut ability_closures,
            ],
            "];",
        );

        let content = [
            champion_id_enum,
            champion_id_to_name,
            champion_positions,
            champion_generator,
            champion_abilities,
            champion_formulas,
            champion_declarations,
            champion_name_to_id,
            champion_cache,
            ability_formulas,
            ability_closures,
            ability_ctx_idents_index,
            ability_ctx_idents,
            recommendations,
            const_eval,
        ]
        .concat()
        .rust_fmt();

        let exports = format!("pub mod champions {{ use super::*; {content} }}");
        Generated { exports, block }
    };

    Ok(Box::new(callback))
}

/// Creates a new string with the definition of recommended items and runes
/// that will be exported to the frontend application
pub fn get_recommendations(len: usize) -> MayFail<String> {
    let enum_ids = ["ItemId", "RuneId"];
    let declaration = ["RECOMMENDED_ITEMS", "RECOMMENDED_RUNES"];

    let mut globals = std::array::from_fn::<_, 2, _>(|i| {
        let enumv = enum_ids[i];
        let var = declaration[i];
        format!("{GLOB_FEAT} pub static {var}: [[&[{enumv}]; 5]; {len}] = [")
    });

    let json = CwdPath::deserialize::<BTreeMap<String, HashMap<String, [Vec<String>; 2]>>>(
        "internal/scraper/data.json",
    )?;

    for (_, data) in json {
        push_end(globals.each_mut(), "[");
        for (_, recommendations) in data.into_iter() {
            for (i, value) in std::array::from_fn::<_, 2, _>(|j| {
                let venum = enum_ids[j];
                let result = recommendations[j]
                    .iter()
                    .map(|element| format!("{venum}::{element}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("&[{result}]")
            })
            .into_iter()
            .enumerate()
            {
                globals[i].push_str(&format!("{value},"));
            }
        }
        push_end(globals.each_mut(), "],");
    }

    push_end(globals.each_mut(), "];");
    Ok(globals.concat())
}
