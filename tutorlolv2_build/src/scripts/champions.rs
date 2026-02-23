use crate::{
    CwdPath, Generated, GeneratorFn, MayFail, SrcFolder, Tracker, parallel_task, push_end,
    scripts::{
        Simplified, StringExt,
        model::{Ability, Champion, MerakiChampionStatMap, MerakiChampionStats},
        rustfmt_batch, simplify,
    },
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{BTreeMap, BTreeSet};
use tutorlolv2_types::{AbilityId, DevMergeData};

struct DeclaredAbility {
    data: AbilityData,
    metadata: String,
    constfn: ConstFn,
    idents: BTreeSet<String>,
}

fn declare_abilities(
    champion_id_upper: &str,
    abilities: Vec<(AbilityId, Ability)>,
) -> BTreeMap<AbilityId, DeclaredAbility> {
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

            let damage = match damage.is_empty() {
                true => "zero".into(),
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
                        src.iter()
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
                    match simplified {
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
                    }
                }
            };

            let discriminant = ability_id.discriminant().to_uppercase();

            let constfn_name = format!(
                "{champion_id}_{discriminant}",
                champion_id = champion_id_upper.to_ssnake()
            )
            .to_lowercase();

            let constfn_declaration = format!(
                "pub const fn {constfn_name}(ctx: &Ctx) -> f32 {body}",
                body = damage.trim_start_matches("|ctx|")
            );

            let idents = damage.get_idents(&damage_type);

            let damage_type = format!("DamageType::{damage_type}");
            let attributes = format!("Attrs::{attributes:?}");

            let metadata = format!(
                "TypeMetadata {{ 
                    kind: {ability_id:?}, 
                    damage_type: {damage_type}, 
                    attributes: {attributes} 
                }}"
            );

            (
                ability_id,
                DeclaredAbility {
                    data: AbilityData {
                        name,
                        damage_type,
                        attributes,
                        damage: damage.replace(" as u8", ""),
                    },
                    metadata,
                    constfn: ConstFn {
                        ability_id: ability_id.as_literal(),
                        declaration: constfn_declaration,
                        name: constfn_name,
                    },
                    idents,
                },
            )
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

fn define_merge_indexes(merge_data: &BTreeSet<DevMergeData>, ability_data: &[AbilityId]) -> String {
    let mut index = BTreeMap::new();
    for (i, &ability_id) in ability_data.iter().enumerate() {
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
            match (index.get(&minimum_damage), index.get(&maximum_damage)) {
                (Some(ia), Some(ib)) => Some(format!(
                    "MergeData {{
                        minimum_damage: {ia},
                        maximum_damage: {ib},
                        alias: {alias:?}
                    }}"
                )),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

struct ConstFn {
    ability_id: String,
    declaration: String,
    name: String,
}

struct AbilityData {
    name: String,
    damage_type: String,
    attributes: String,
    damage: String,
}

struct ChampionResult {
    champion_id_upper: String,
    base_declaration: String,
    ability_declarations: BTreeMap<AbilityId, AbilityData>,
    generator: String,
    html_declaration: String,
    const_match_kind: String,
    ability_idents_index: String,
    ability_idents: String,
    damage_def: Vec<(AbilityId, String)>,
    merge_data: BTreeSet<DevMergeData>,
}

pub fn generate_champions() -> GeneratorFn {
    let mut data = parallel_task("internal/champions", |champion_id, champion: Champion| {
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

        let abilities = declare_abilities(&champion_id_upper, abilities);

        let len = abilities.len();

        let mut constfns = Vec::with_capacity(len);
        let mut fn_names = String::new();
        let mut closures = String::new();
        let mut ability_metadata = String::new();
        let mut ability_names = Vec::with_capacity(len);
        let mut ability_declarations = BTreeMap::new();

        let mut ability_idents_index = String::new();
        let mut ability_idents = String::new();
        let mut ability_index = 0;

        let mut damage_def = Vec::with_capacity(len);

        for (
            ability_id,
            DeclaredAbility {
                data,
                metadata,
                constfn,
                idents,
            },
        ) in abilities
        {
            let fn_name = &constfn.name;
            fn_names.push_str(&format!("{fn_name},"));
            ability_metadata.push_str(&format!("{metadata},"));
            closures.push_str(&format!("{fn_name}: {damage},", damage = data.damage));
            ability_names.push(ability_id);
            ability_declarations.insert(ability_id, data);

            let start = ability_index;
            ability_index += idents.len();
            ability_idents_index.push_str(&format!("{start}..{ability_index},"));
            ability_idents.push_str(&idents.into_iter().collect::<String>());
            damage_def.push((
                ability_id,
                constfn
                    .declaration
                    .trim_start_matches("pub const")
                    .trim()
                    .to_string(),
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
            merge_data = define_merge_indexes(&merge_data, &ability_names),
        );

        let base_declaration = format!(
            "pub static {champion_id_upper}: CachedChampion = CachedChampion {{
                name: {name:?},
                adaptative_type: AdaptativeType::{adaptative_type},
                attack_type: AttackType::{attack_type},
                positions: &[{positions}],"
        );

        let html_declaration = format!("{base_declaration}{closures}{rest} }};");

        let mut base_declaration = format!("{base_declaration}closures: &[{fn_names}], {rest} }};");

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
        let generator = CwdPath::get_generator(SrcFolder::Champions, champion_id)?.rust_html();

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

        println!("[build] ChampionId::{champion_id}");

        Ok(ChampionResult {
            ability_declarations,
            ability_idents_index: format!("&[{ability_idents_index}]"),
            ability_idents: format!("&[{ability_idents}]"),
            const_match_kind,
            champion_id_upper,
            base_declaration,
            html_declaration,
            merge_data,
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

    let [
        mut champion_cache,
        mut champion_formulas,
        mut champion_generator,
        mut champion_abilities,
        mut ability_ctx_idents_index,
        mut ability_ctx_idents,
        mut ability_formulas,
        mut ability_closures,
    ] = std::array::from_fn(|i| {
        let (name, vtype) = [
            ("CHAMPION_CACHE", "&CachedChampion"),
            ("CHAMPION_FORMULAS", "Range<usize>"),
            ("CHAMPION_GENERATOR", "Range<usize>"),
            ("CHAMPION_ABILITIES", "&[AbilityId]"),
            ("ABILITY_IDENTS_INDEX", "&[Range<usize>]"),
            ("ABILITY_IDENTS", "&[CtxVar]"),
            ("ABILITY_FORMULAS", "&[Range<usize>]"),
            ("ABILITY_CLOSURES", "&[Range<usize>]"),
        ][i];
        format!("pub static {name}: [{vtype}; ChampionId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);
    let mut ability_offsets = Vec::with_capacity(len);
    let mut damage_offsets = Vec::with_capacity(len);

    struct FmtArgs {
        html_index: usize,
        damage_start: usize,
        damage_ids: Vec<AbilityId>,
        ability_start: usize,
        ability_ids: Vec<AbilityId>,
    }

    let mut rustfmt_inputs = Vec::<String>::new();
    let mut fmt_args = Vec::<FmtArgs>::with_capacity(len);

    let mut champion_declarations = String::new();

    let mut champion_id_enum = format!(
        r#"impl ChampionId {{
            pub const VARIANTS: usize = {len};
        }}
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum ChampionId {{"#,
    );

    let languages_map = CwdPath::deserialize::<BTreeMap<String, BTreeSet<String>>>(
        "internal/champion_languages.json",
    )?;
    let mut language_arms = Vec::with_capacity(data.len());
    let mut const_match_arms = String::new();

    for (
        champion_id,
        ChampionResult {
            base_declaration,
            html_declaration,
            const_match_kind,
            ability_declarations,
            generator,
            champion_id_upper,
            ability_idents,
            ability_idents_index,
            merge_data,
            damage_def,
        },
    ) in data
    {
        let name_alias = languages_map
            .get(&champion_id)
            .ok_or(format!(
                "Failed to recover {champion_id:?} from languages map"
            ))?
            .iter()
            .map(|alias| format!("{alias:?}"))
            .chain(std::iter::once(format!("{champion_id:?}")))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" | ");
        language_arms.push(format!("{name_alias} => ChampionId::{champion_id}"));

        champion_declarations.push_str(&base_declaration);

        let match_arm =
            format!("ChampionId::{champion_id} => {{ match kind {{ {const_match_kind} }} }}");

        const_match_arms.push_str(&match_arm);
        champion_id_enum.push_str(&format!("{champion_id},"));
        champion_cache.push_str(&format!("&{champion_id_upper},"));

        ability_ctx_idents.push_str(&format!("{ability_idents},"));
        ability_ctx_idents_index.push_str(&format!("{ability_idents_index},"));

        tracker.record_into(&generator, &mut generator_offsets);

        let html_index = rustfmt_inputs.len();
        rustfmt_inputs.push(html_declaration);

        let damage_start = rustfmt_inputs.len();
        let mut damage_ids = Vec::with_capacity(damage_def.len());
        for (ability_id, value) in damage_def {
            damage_ids.push(ability_id);
            rustfmt_inputs.push(value);
        }

        let ability_start = rustfmt_inputs.len();
        let mut ability_ids = Vec::with_capacity(ability_declarations.len());

        let adecl = ability_declarations.into_iter().collect::<Vec<_>>();

        for (ability_id, value) in adecl.iter() {
            let ability_id = *ability_id;
            ability_ids.push(ability_id);

            let AbilityData {
                name,
                damage_type,
                attributes,
                damage,
            } = value;

            let (discriminant, damage) = match merge_data.iter().find(|merge| {
                merge.maximum_damage == ability_id || merge.minimum_damage == ability_id
            }) {
                Some(merge) => {
                    let find_damage = |id| &adecl.iter().find(|(a, _)| *a == id).unwrap().1.damage;
                    let (min, max) = match merge.maximum_damage == ability_id {
                        true => (find_damage(merge.minimum_damage), damage),
                        false => (damage, find_damage(merge.maximum_damage)),
                    };
                    (
                        merge.alias.discriminant(),
                        format!("minimum_damage: {min}, maximum_damage: {max}"),
                    )
                }
                None => (ability_id.discriminant(), format!("damage: {damage}")),
            };

            let discriminant_upper = discriminant.to_uppercase();

            rustfmt_inputs.push(format!(
                "static {champion_id_upper}_{discriminant_upper}: Ability = Ability {{
                    name: {name:?},
                    damage_type: {damage_type},
                    attributes: {attributes},
                    {damage},
                }};"
            ))

            // rustfmt_inputs.push(value);
        }

        fmt_args.push(FmtArgs {
            html_index,
            damage_start,
            damage_ids,
            ability_start,
            ability_ids,
        });
    }

    let formatted = rustfmt_batch(&rustfmt_inputs);

    for plan in fmt_args {
        let html_declaration = formatted[plan.html_index].drop_f32s().rust_html();
        tracker.record_into(&html_declaration, &mut formula_offsets);

        let mut declare_offsets = |list: Vec<(AbilityId, String)>, into: &mut Vec<_>| {
            let offsets = list
                .into_iter()
                .map(|(ability_id, value)| (ability_id, tracker.record(&value)))
                .collect::<Vec<_>>();
            into.push(offsets);
        };

        let damage_def = plan
            .damage_ids
            .iter()
            .enumerate()
            .map(|(j, ability_id)| {
                let index = plan.damage_start + j;
                (*ability_id, formatted[index].drop_f32s().rust_html())
            })
            .collect::<Vec<_>>();

        let ability_declarations = plan
            .ability_ids
            .iter()
            .enumerate()
            .map(|(j, ability_id)| {
                let index = plan.ability_start + j;
                (*ability_id, formatted[index].rust_html().as_const())
            })
            .collect::<Vec<_>>();

        declare_offsets(damage_def, &mut damage_offsets);
        declare_offsets(ability_declarations, &mut ability_offsets);
    }

    let const_eval = format!(
        "pub const fn ability_const_eval(
            ctx: &Ctx, 
            champion_id: ChampionId, 
            kind: AbilityId
        ) -> f32 {{
            match champion_id {{ {const_match_arms} }}
        }}"
    );

    let champion_name_to_id = format!(
        "pub static CHAMPION_NAME_TO_ID: phf::Map<&str, ChampionId> = phf::phf_map! {{
            {arms}
        }};",
        arms = language_arms.join(",\n\t\t\t")
    );

    push_end([&mut champion_id_enum], "}");
    push_end(
        [
            &mut champion_cache,
            &mut ability_ctx_idents_index,
            &mut ability_ctx_idents,
        ],
        "];",
    );

    println!("[ok] Finished building champions");

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
                    .iter()
                    .map(|(ability_id, _)| format!("{ability_id:?}"))
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
        .concat();

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
        format!("pub static {var}: [[&[{enumv}]; 5]; {len}] = [")
    });

    let json = CwdPath::deserialize::<BTreeMap<String, BTreeMap<String, [BTreeSet<String>; 2]>>>(
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
