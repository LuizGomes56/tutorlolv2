use crate::{
    Build, MayFail, OUT_DIR,
    generators::{
        parser::champions::{Ability, Champion},
        utils::Tag,
    },
    model::champions::WikiChampion,
    scripts::{
        batch::FmtArgs,
        utils::{cast_f32, ctx_param, fit_str, get_identifiers, simplify},
    },
};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    write,
};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{AbilityId, CtxVar, DevMergeData, MergeData, TypeMetadata};

struct ChampionExt {
    metadata: Vec<TypeMetadata<AbilityId>>,
    closures: Vec<String>,
    identifiers: Vec<BTreeSet<CtxVar>>,
    functions: Vec<String>,
    merge_data: Vec<MergeData>,
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
            ..
        } = &self;

        println!("Building {champion_id:?}...");

        let mut rust = String::new();
        let mut docs = String::new();

        let upper_id = to_ssnake(&champion_id);
        let damage = abilities
            .iter()
            .map(|(k, v)| {
                let discriminant = k.discriminant().to_lowercase();
                let formula = simplify(&v.damage);
                format!("{discriminant}: {formula},")
            })
            .collect::<String>();

        write!(
            docs,
            "#[fmt({fmt})]
            static {upper_id}: X = X {{
                name: {name:?},
                adaptive_type: {adaptive_type:?},
                attack_type: {attack_type:?},
                positions: {positions:#?}, {damage}
            }};",
            fmt = json!(FmtArgs {
                target: "formula",
                variant: &champion_id,
                meta: (),
                replace: [(": X = X", " ="), ("ctx.", "")].into(),
                default: false
            })
        )?;

        write!(
            rust,
            "pub static {upper_id}: X = X {{
                name: {name:?},
                adaptive_type: AdaptiveType::{adaptive_type:?},
                attack_type: {attack_type:?},
                positions: &{positions:#?},
                stats: {stats:#?},
                modifiers: {modifiers:#?},
                combos: &[{combos}],
                metadata: &{metadata:#?},
                merge_data: &{merge_data:#?},
                identifiers: &[{identifiers}],
                closures: &[{fn_names}],
            }};",
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

        for (i, (((ability_id, ability), function), body)) in
            abilities.iter().zip(&functions).zip(closures).enumerate()
        {
            let Ability {
                name,
                damage_type,
                attributes,
                comment,
                damage,
            } = ability;

            let formula = simplify(&body);
            let formula_f32 = cast_f32(&formula);

            let mut variable = function.to_uppercase();

            let damage_attr = match merge_data
                .iter()
                .find(|merge| merge.min as usize == i || merge.max as usize == i)
            {
                Some(merge) => {
                    let get_ability = |j| abilities.values().nth(j as usize).unwrap();

                    let min_ability = get_ability(merge.min);
                    let max_ability = get_ability(merge.max);

                    let min_damage = simplify(&min_ability.damage);
                    let max_damage = simplify(&max_ability.damage);

                    let alias = merge.alias.discriminant();
                    variable = format!("{champion_id}_{alias}").to_uppercase();

                    format!("min_dmg: {min_damage}, max_dmg: {max_damage}")
                }
                None => {
                    let damage = simplify(damage);
                    format!("damage: {damage}")
                }
            };

            write!(
                rust,
                "pub const fn {function}({param}: &Ctx) -> f32 {{{formula_f32}}}",
                param = ctx_param(&formula_f32)
            )?;

            write!(
                docs,
                "#[fmt({fmt_fn})]
                fn {function}() {{{formula}}}

                #[fmt({fmt_block})]
                static {variable}: Ability = Ability {{
                    name: {name:?},
                    damage_type: {damage_type:?},
                    attributes: {attributes:?},
                    comment: {comment},
                    {damage_attr},
                }};",
                comment = fit_str(comment),
                fmt_fn = json!(FmtArgs {
                    target: "closure",
                    variant: &champion_id,
                    meta: ability_id,
                    replace: [("ctx.", "")].into(),
                    default: false
                }),
                fmt_block = json!(FmtArgs {
                    target: "ability",
                    variant: &champion_id,
                    meta: ability_id,
                    replace: [(": Ability = Ability", " ="), ("ctx.", "")].into(),
                    default: false
                })
            )?;
        }

        let out = OUT_DIR.join(Tag::Champions.plural()).join(champion_id);

        crate::write(out.with_extension("rs"), rust)?;
        crate::write(out.with_extension("w48"), docs)?;

        Ok(format!(
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
                    let module = to_ssnake(champion_id).to_lowercase();
                    format!("{ability_id:?} => {module}::{function}(ctx),")
                })
                .collect::<String>()
        ))
    }
}

impl Champion {
    fn finish(&mut self) -> MayFail<ChampionExt> {
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
                    champion_id = to_ssnake(&self.data.champion_id),
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
