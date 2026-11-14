use {
    crate::{
        Build, BuildContext, BuildOutput, CLIENT_CFG, DOCS_CFG, ENGINE_CFG, MayFail,
        generators::parser::champions::{Ability, Champion},
        scripts::{
            batch::FormulaDoc,
            simplify::format_formula,
            utils::{BakeExt, cast_f32, ctx_param, get_identifiers, probe_ratio},
        },
    },
    heck::{ToShoutySnakeCase, ToSnakeCase},
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Write,
        write,
    },
    tutorlolv2_types::{
        AbilityId, AbilityName, Attrs, CtxVar, DamageType, DevMergeData, MergeData, Tag,
        TypeMetadata,
    },
    tutorlolv2_wiki::champions::WikiChampion,
};

pub struct ChampionExt {
    pub metadata: Vec<TypeMetadata<AbilityId>>,
    pub closures: Vec<String>,
    pub identifiers: Vec<BTreeSet<CtxVar>>,
    pub functions: Vec<String>,
    pub merge_data: Vec<MergeData>,
}

impl Build for Champion {
    fn build(&mut self, context: &BuildContext) -> MayFail<BuildOutput> {
        let ChampionExt {
            metadata,
            closures,
            identifiers,
            functions,
            merge_data,
        } = self.finish()?;

        let Self {
            champion_id,
            data:
                WikiChampion {
                    name,
                    adaptive_type,
                    attack_type,
                    positions,
                    stats,
                    modifiers,
                    ..
                },
            combo,
            abilities,
            merge,
            constants,
            ..
        } = &self;

        let mut rust = String::new();
        let mut docs = Vec::new();

        let simplified = closures
            .iter()
            .map(|body| format_formula(body))
            .collect::<Vec<_>>();

        let mut damage_override = BTreeMap::<AbilityId, String>::new();

        for merge in merge {
            let min = self.get(merge.min)?;
            let max = self.get(merge.max)?;
            let min_i = self.indexof(merge.min)?;
            let max_i = self.indexof(merge.max)?;

            // Sampled numerically: formulas are not simplified, so their ratio is only
            // known when it is the same at every sample
            let ratio = {
                let all_vars = &identifiers[min_i]
                    .iter()
                    .chain(&identifiers[max_i])
                    .copied()
                    .collect::<Vec<_>>();

                let Some(k) = probe_ratio(&min.damage, &max.damage, all_vars) else {
                    continue;
                };

                k
            };

            let min_fn = &functions[min_i];

            damage_override.insert(merge.max, format!("{ratio} * {min_fn}(ctx)"));
        }

        let damages = abilities
            .keys()
            .zip(&simplified)
            .map(|(id, source)| damage_override.get(id).unwrap_or(source).clone())
            .collect::<Vec<_>>();

        let upper_id = champion_id.to_shouty_snake_case();
        let batch_name = format!("{}_batch", champion_id.to_snake_case());

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                adaptive_type: {adaptive_type},
                attack_type: {attack_type},
                positions: {positions},
                stats: {stats},
                modifiers: {modifiers},
                map_stats: &[{map_stats}],
                metadata: {metadata},

                {ENGINE_CFG}
                closures: &[{fn_names}],

                {ENGINE_CFG}
                ability_batch: {batch_name},

                {DOCS_CFG}
                combos: &[{combos}],

                {CLIENT_CFG}
                merge_data: {merge_data},

                {CLIENT_CFG}
                identifiers: &[{identifiers}],
            }};"#,
            stats = cast_f32(&format!("{stats:?}"))?,
            map_stats = self
                .map_stats()
                .iter()
                .map(|stats| -> MayFail<String> {
                    Ok(format!(
                        "crate::libgen::globals::MapStats {{
                            maps: &[{}],
                            total_attack_speed: {:?}f32,
                            stats: {},
                            level_bonuses: &[{}]
                        }}",
                        stats
                            .data
                            .maps
                            .iter()
                            .map(|map| format!("tutorlolv2_types::GameMap::{map:?}"))
                            .collect::<Vec<_>>()
                            .join(", "),
                        stats.data.total_attack_speed,
                        cast_f32(&format!("{:?}", stats.data.stats))?,
                        stats
                            .level_bonuses
                            .iter()
                            .map(|(stat, values)| format!(
                                "crate::libgen::globals::MapStatBonus {{
                                    stat: tutorlolv2_types::StatName::{stat:?},
                                    values: &[{}]
                                }}",
                                values
                                    .iter()
                                    .map(|(level, value)| format!("({level}u8, {value:?}f32)"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                })
                .collect::<MayFail<Vec<_>>>()?
                .join(", "),
            modifiers = cast_f32(&format!("{modifiers:?}"))?,
            adaptive_type = adaptive_type.bake(),
            attack_type = attack_type.bake(),
            positions = positions.as_slice().bake(),
            metadata = metadata.as_slice().bake(),
            merge_data = merge_data.as_slice().bake(),
            combos = combo
                .iter()
                .map(|ident| ident.as_slice().bake())
                .collect::<Vec<_>>()
                .join(","),
            identifiers = identifiers
                .iter()
                .enumerate()
                .map(|(i, set)| {
                    let rest = if i == 0 { "as &[_]" } else { "" };
                    let vec = set.iter().copied().collect::<Vec<_>>();
                    format!("{}{rest}", vec.as_slice().bake())
                })
                .collect::<Vec<_>>()
                .join(","),
            fn_names = functions.join(",")
        )?;

        for (i, ((ability_id, ability), function)) in abilities.iter().zip(&functions).enumerate() {
            let Ability { name, comment, .. } = ability;

            let body = &damages[i];
            let rust_formula = cast_f32(body)?;

            write!(
                rust,
                "{ENGINE_CFG} pub const fn {function}({param}: &tutorlolv2_types::Ctx) -> f32 {{{rust_formula}}}",
                param = ctx_param(&rust_formula)
            )?;

            if context.docs {
                docs.push(FormulaDoc {
                    local: i.try_into()?,
                    function: function.clone(),
                    // The formula the app runs, with the arms of the modes that change it.
                    // v2 generators write it from the same tree as the damage
                    source: self
                        .docs
                        .get(ability_id)
                        .cloned()
                        .unwrap_or_else(|| body.clone()),
                    name: name.clone(),
                    comment: comment.clone(),
                });
            }
        }

        // Champions without onhit abilities never write to it
        let onhit = match metadata.iter().any(|meta| {
            matches!(
                meta.attributes,
                Attrs::Onhit
                    | Attrs::OnhitMin
                    | Attrs::OnhitMax
                    | Attrs::AreaOnhit
                    | Attrs::AreaOnhitMin
                    | Attrs::AreaOnhitMax
            )
        }) {
            true => "onhit",
            false => "_onhit",
        };

        // Emit one direct-call sequence in precisely the metadata/closure order.
        // Keep the multiplication grouping and per-ability integer cast unchanged.
        write!(
            rust,
            "{ENGINE_CFG} pub const fn {batch_name}(
                ctx: &tutorlolv2_types::Ctx,
                modifiers: crate::model::Modifiers,
                {onhit}: &mut crate::model::RangeDamage,
                out: &mut [i32]
            ) {{
                debug_assert!(out.len() == {});
            ",
            functions.len()
        )?;

        for (i, (function, meta)) in functions.iter().zip(&metadata).enumerate() {
            let damage_mod = match meta.damage_type {
                DamageType::Physical => "modifiers.damages.physical_mod",
                DamageType::Magic => "modifiers.damages.magic_mod",
                DamageType::True => "modifiers.damages.true_mod",
                DamageType::Adaptive => {
                    "match modifiers.damages.adaptive_type {
                        tutorlolv2_types::AdaptiveType::Physical => modifiers.damages.physical_mod,
                        tutorlolv2_types::AdaptiveType::Magic => modifiers.damages.magic_mod
                    }"
                }
                _ => "1.0",
            };

            let ability_mod = match meta.kind {
                AbilityId::Q(v) if v as u8 <= AbilityName::Mega as u8 => "modifiers.abilities.q",
                AbilityId::W(v) if v as u8 <= AbilityName::Mega as u8 => "modifiers.abilities.w",
                AbilityId::E(v) if v as u8 <= AbilityName::Mega as u8 => "modifiers.abilities.e",
                AbilityId::R(v) if v as u8 <= AbilityName::Mega as u8 => "modifiers.abilities.r",
                _ => "1.0",
            };

            write!(
                rust,
                "let modifier = (modifiers.damages.global_mod * ({damage_mod})) * {ability_mod};
                let damage = (modifier * {function}(ctx)) as i32;"
            )?;

            match meta.attributes {
                Attrs::OnhitMin | Attrs::AreaOnhitMin => {
                    write!(rust, "onhit.minimum_damage += damage;")?
                }
                Attrs::OnhitMax | Attrs::AreaOnhitMax => {
                    write!(rust, "onhit.maximum_damage += damage;")?
                }
                Attrs::Onhit | Attrs::AreaOnhit => write!(
                    rust,
                    "onhit.minimum_damage += damage; onhit.maximum_damage += damage;"
                )?,
                _ => {}
            }

            write!(rust, "out[{i}] = damage;")?;
        }

        write!(rust, "}}")?;

        rust.push_str(&constants.bake());

        let out = context.out.join(Tag::Champions.as_ref()).join(champion_id);

        tutorlolv2_wiki::write(out.with_extension("rs"), rust)?;

        let eval = format!(
            r#"ChampionId::{champion_id} => {{
                match kind {{
                    {arms}
                    _ => panic!("Invalid AbilityId provided for '{champion_id}'"),
                }}
            }},"#,
            arms = functions
                .iter()
                .zip(metadata)
                .map(|(function, metadata)| {
                    let ability_id = metadata.kind;
                    let module = champion_id.to_snake_case();

                    format!(
                        "{} => crate::libgen::champions::{module}::{function}(ctx),",
                        ability_id.bake()
                    )
                })
                .collect::<String>()
        );

        Ok(BuildOutput {
            eval,
            formulas: docs,
        })
    }
}

impl Champion {
    pub fn finish(&mut self) -> MayFail<ChampionExt> {
        let metadata = self
            .abilities
            .iter()
            .map(|(k, v)| TypeMetadata {
                kind: *k,
                damage_type: v.damage_type,
                attributes: v.attributes,
            })
            .collect::<Vec<_>>();

        let closures = self
            .abilities
            .values()
            .map(|v| v.damage.clone())
            .collect::<Vec<_>>();

        let functions = self
            .abilities
            .keys()
            .map(|ability_id| {
                let discriminant = ability_id.discriminant().to_uppercase();

                format!(
                    "{champion_id}_{discriminant}",
                    champion_id = self.data.champion_id.to_snake_case(),
                )
                .to_lowercase()
            })
            .collect::<Vec<_>>();

        let merge_data = {
            let mut index = BTreeMap::new();

            for (i, &ability_id) in self.abilities.keys().enumerate() {
                index.entry(ability_id).or_insert(i);
            }

            let result = self
                .merge
                .iter()
                .filter_map(|value| {
                    let DevMergeData { min, max, alias } = value;

                    match (index.get(min), index.get(max)) {
                        (Some(ia), Some(ib)) => Some(MergeData {
                            min: *ia as _,
                            max: *ib as _,
                            alias: *alias,
                        }),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();

            for value in result.iter().copied() {
                let MergeData {
                    min: min_damage,
                    max: max_damage,
                    ..
                } = value;

                let min = self.nth(min_damage as _)?;
                let max = self.nth(max_damage as _)?;

                let comment = format!(
                    "{min_c} & {max_c}",
                    min_c = min.comment,
                    max_c = max.comment
                );

                self.nth_mut(min_damage as _)?.comment = comment.clone();
                self.nth_mut(max_damage as _)?.comment = comment;
            }

            result
        };

        let identifiers = self
            .abilities
            .values()
            .map(|ability| {
                get_identifiers(&ability.damage, ability.damage_type).collect::<BTreeSet<_>>()
            })
            .collect::<Vec<_>>();

        Ok(ChampionExt {
            metadata,
            closures,
            identifiers,
            functions,
            merge_data,
        })
    }
}
