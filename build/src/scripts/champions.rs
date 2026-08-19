use {
    crate::{
        Build, MayFail, OUT_DIR,
        generators::{
            parser::champions::{Ability, Champion},
            utils::Tag,
        },
        scripts::{
            batch::FmtArgs,
            utils::{cast_f32, ctx_param, fit_str, get_identifiers, probe_ratio, simplify},
        },
    },
    heck::{ToShoutySnakeCase, ToSnakeCase},
    serde_json::json,
    std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Write,
        write,
    },
    tutorlolv2_types::{AbilityId, CtxVar, DevMergeData, MergeData, TypeMetadata},
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
    fn build(&mut self) -> MayFail<String> {
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
        let mut docs = String::new();
        let mut damage_override = BTreeMap::<AbilityId, String>::new();

        for merge in merge {
            let min = self.get(merge.min)?;
            let max = self.get(merge.max)?;
            let min_i = self.indexof(merge.min)?;
            let max_i = self.indexof(merge.max)?;

            let ratio_sym = simplify(&format!(
                "({}) / ({})",
                simplify(&max.damage),
                simplify(&min.damage),
            ));

            let ratio = if !ratio_sym.contains("ctx") {
                ratio_sym
            } else {
                let all_vars = &identifiers[min_i]
                    .iter()
                    .chain(&identifiers[max_i])
                    .copied()
                    .collect::<Vec<_>>();

                let Some(k) = probe_ratio(&min.damage, &max.damage, &all_vars) else {
                    continue;
                };

                k
            };

            let min_fn = &functions[min_i];
            damage_override.insert(merge.max, format!("{ratio} * {min_fn}(ctx)"));
        }

        let resolve_damage = |id: &AbilityId, raw: &str| -> String {
            damage_override
                .get(id)
                .cloned()
                .unwrap_or_else(|| simplify(raw))
        };

        let damage = abilities
            .iter()
            .map(|(id, v)| {
                let discriminant = id.discriminant().to_lowercase();
                let dmg = resolve_damage(id, &v.damage);

                format!("{discriminant}: {dmg},")
            })
            .collect::<String>();

        let upper_id = champion_id.to_shouty_snake_case();

        write!(
            docs,
            "#[fmt({fmt})]
            static {upper_id}: X = X {{{damage}}};",
            fmt = json!(FmtArgs {
                target: "formula".into(),
                variant: champion_id.clone(),
                meta: (),
                default: false
            }),
        )?;

        write!(
            rust,
            r#"pub static {upper_id}: X = X {{
                name: {name:?},
                adaptive_type: AdaptiveType::{adaptive_type:?},
                attack_type: {attack_type:?},
                positions: &{positions:#?},
                stats: {stats:#?},
                modifiers: {modifiers:#?},
                combos: &[{combos}],
                metadata: &{metadata:#?},
                merge_data: &{merge_data:#?},
                closures: &[{fn_names}],
                #[cfg(feature = "docs")]
                identifiers: &[{identifiers}],
            }};"#,
            combos = combo
                .iter()
                .map(|ident| format!("&{ident:#?}"))
                .collect::<Vec<_>>()
                .join(","),
            identifiers = identifiers
                .iter()
                .enumerate()
                .map(|(i, set)| {
                    let rest = if i == 0 { "as &[_]" } else { "" };
                    let vec = set.iter().collect::<Vec<_>>();
                    format!("&{vec:?}{rest}")
                })
                .collect::<Vec<_>>()
                .join(","),
            fn_names = functions.join(",")
        )?;

        for (((ability_id, ability), function), body) in
            abilities.iter().zip(&functions).zip(closures)
        {
            let Ability { name, comment, .. } = ability;

            let formula = simplify(&body);
            let mut rust_formula = formula.clone();
            let mut variable = function.to_uppercase();

            if let Some(merge) = merge
                .iter()
                .find(|merge| merge.min == *ability_id || merge.max == *ability_id)
            {
                let min_damage = resolve_damage(&merge.min, &self.damage_of(merge.min)?);
                let max_damage = resolve_damage(&merge.max, &self.damage_of(merge.max)?);

                if merge.min == *ability_id {
                    rust_formula = min_damage.clone();
                } else {
                    rust_formula = max_damage.clone();
                }

                let alias = merge.alias.discriminant();
                variable = format!("{champion_id}_{alias}").to_uppercase();
            }

            let mut rust_formula = cast_f32(&rust_formula);

            if crate::scripts::utils::is_zero(&rust_formula) {
                println!("[{champion_id}]: ZeroFormula[{ability_id:?}]; Body = {body:?}");
                rust_formula = "0.0".to_string();
            }

            write!(
                rust,
                "pub const fn {function}({param}: &Ctx) -> f32 {{{rust_formula}}}",
                param = ctx_param(&rust_formula)
            )?;

            write!(
                docs,
                "#[fmt({fmt_block})]
                static {variable}: Ability = Ability {{
                    name: {name:?},
                    comment: {comment},
                }};",
                comment = fit_str(comment),
                fmt_block = json!(FmtArgs {
                    target: "ability".into(),
                    variant: champion_id.clone(),
                    meta: ability_id,
                    default: false
                })
            )?;
        }

        let out = OUT_DIR.join(Tag::Champions.plural()).join(champion_id);

        tutorlolv2_wiki::write(out.with_extension("rs"), rust)?;
        tutorlolv2_wiki::write(out.with_extension("w48"), docs)?;

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
                    format!("{ability_id:?} => {module}::{function}(ctx),")
                })
                .collect::<String>()
        );

        Ok(eval)
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
