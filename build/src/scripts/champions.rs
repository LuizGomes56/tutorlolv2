use {
    crate::{
        Build, BuildContext, BuildOutput, ENGINE_CFG, MayFail, YEW_CFG,
        generators::parser::champions::{Ability, Champion},
        scripts::{
            batch::FormulaDoc,
            simplify::simplify,
            utils::{BakeExt, cast_f32, ctx_param, get_identifiers, probe_ratio},
        },
    },
    heck::{ToShoutySnakeCase, ToSnakeCase},
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Write,
        write,
    },
    tutorlolv2_types::{AbilityId, CtxVar, DevMergeData, MergeData, Tag, TypeMetadata},
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
        } = &self;

        let mut rust = String::new();
        let mut docs = Vec::new();

        let simplified = closures
            .iter()
            .map(|body| simplify(body))
            .collect::<Vec<_>>();

        let mut damage_override = BTreeMap::<AbilityId, String>::new();

        for merge in merge {
            let min = self.get(merge.min)?;
            let max = self.get(merge.max)?;
            let min_i = self.indexof(merge.min)?;
            let max_i = self.indexof(merge.max)?;

            let ratio_sym = simplify(&format!(
                "({}) / ({})",
                simplified[max_i], simplified[min_i],
            ));

            let ratio = if !ratio_sym.contains("ctx") {
                ratio_sym
            } else {
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

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                adaptive_type: {adaptive_type},
                attack_type: {attack_type},
                positions: {positions},
                stats: {stats},
                modifiers: {modifiers},
                metadata: {metadata},

                {ENGINE_CFG}
                closures: &[{fn_names}],

                {YEW_CFG}
                combos: &[{combos}],

                {YEW_CFG}
                merge_data: {merge_data},

                {YEW_CFG}
                identifiers: &[{identifiers}],
            }};"#,
            stats = cast_f32(&format!("{stats:?}"))?,
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
            let mut rust_formula = cast_f32(body)?;

            if crate::scripts::utils::is_zero(&rust_formula) {
                println!("[{champion_id}]: ZeroFormula[{ability_id:?}]; Body = {body:?}");

                rust_formula = "0.0".to_string();
            }

            write!(
                rust,
                "{ENGINE_CFG} pub const fn {function}({param}: &tutorlolv2_types::Ctx) -> f32 {{{rust_formula}}}",
                param = ctx_param(&rust_formula)
            )?;

            if context.docs {
                docs.push(FormulaDoc {
                    local: i.try_into()?,
                    function: function.clone(),
                    source: body.clone(),
                    name: name.clone(),
                    comment: comment.clone(),
                });
            }
        }

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
