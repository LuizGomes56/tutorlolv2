use crate::{
    MayFail,
    generators::parser::champions::{Champion, ChampionBuild},
};
use serde_json::{Value, json};

impl Champion {
    pub fn _end(&self) -> MayFail {
        let champion_id = &self.champion_id;
        let ChampionBuild {
            abilities,
            name,
            adaptive_type,
            attack_type,
            positions,
            stats,
            modifiers,
            combos,
            metadata,
            closures,
            merge_data,
            identifiers,
            functions,
        } = &self.build;

        let fmt_formula = json!(FmtArgs {
            target: "formula",
            variant: champion_id,
            meta: (),
            replace: [
                (": Champion = Champion", " ="),
                ("DevMergeData ", ""),
                ("WikiStats ", ""),
                ("Stat ", ""),
                ("WikiModifiers ", ""),
                ("Modifier ", ""),
                ("TypeMetadata ", ""),
                ("ctx.", "")
            ]
            .into(),
            default: false
        });

        let decl = format!(
            r#"
            #[fmt({fmt_formula})]
            static {upper_id}: Champion = Champion {{
                name: {name:?},
                adaptive_type: {adaptive_type:?},
                attack_type: {attack_type:?},
                positions: {positions:#?}, {damage}
            }};

            pub static {upper_id}: Champion = Champion {{
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
            }};
            "#,
            upper_id = to_ssnake(champion_id),
            damage = abilities
                .iter()
                .map(|(k, v)| {
                    let discriminant = k.discriminant().to_lowercase();
                    let formula = simplify(&v.damage);
                    format!("{discriminant}: {formula},")
                })
                .collect::<String>(),
            combos = combos
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
            fn_names = functions.join(","),
        );

        let generator = get_generator(Tag::Champion, champion_id, champion_id);

        let abilities_decl = abilities
            .iter()
            .zip(functions)
            .zip(closures)
            .enumerate()
            .map(|(i, (((ability_id, ability), function), body))| {
                let Ability {
                    name,
                    damage_type,
                    attributes,
                    comment,
                    damage,
                } = ability;

                let formula = simplify(body);

                let fmt_closure = json!(FmtArgs {
                    target: "closure",
                    variant: champion_id,
                    meta: ability_id,
                    replace: [("ctx.", "")].into(),
                    default: false
                });

                let fmt_ability = json!(FmtArgs {
                    target: "ability",
                    variant: champion_id,
                    meta: ability_id,
                    replace: [(": Ability = Ability", " ="), ("ctx.", "")].into(),
                    default: false
                });

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

                fn fmt_comment(c: &str) -> String {
                    const CHUNK: usize = 36;
                    let comment = c.replace("  ", " ");
                    if comment.len() <= CHUNK {
                        return format!("{comment:?}");
                    }
                    let mut chunks = Vec::new();
                    let mut current = String::new();
                    for word in comment.split(' ') {
                        if !current.is_empty() && current.len() + 1 + word.len() > CHUNK {
                            chunks.push(format!("{current:?}"));
                            current = word.to_string();
                        } else {
                            if !current.is_empty() {
                                current.push(' ');
                            }
                            current.push_str(word);
                        }
                    }
                    if !current.is_empty() {
                        chunks.push(format!("{current:?}"));
                    }
                    format!("concat!({})", chunks.join(", "))
                }

                let ability_decl = format_args!(
                    "static {variable}: Ability = Ability {{
name: {name:?},
damage_type: {damage_type:?},
attributes: {attributes:?},
comment: {comment},
{damage_attr},
}};",
                    comment = fmt_comment(comment)
                );

                format!(
                    r#"
                    pub const fn {function}({param}: &Ctx) -> f32 {{{formula_f32}}}

                    #[fmt({fmt_closure})]
                    fn {function}() {{{formula}}}

                    #[fmt({fmt_ability})]
                    {ability_decl}
                    "#,
                    param = ctx_param(&formula_f32)
                )
            })
            .collect::<String>();

        let eval = format!(
            r#"
            ChampionId::{champion_id} => {{
                match kind {{
                    {arms}
                    _ => panic!("Invalid AbilityId provided for '{champion_id}'"),
                }}
            }},
            "#,
            arms = functions
                .iter()
                .zip(metadata)
                .map(|(function, metadata)| {
                    let ability_id = metadata.kind;
                    format!("{ability_id:?} => {function}(ctx),")
                })
                .collect::<String>()
        );

        (
            champion_id,
            Batch {
                eval,
                fmt: [decl, generator, abilities_decl].concat(),
            },
        );

        Ok(())
    }
}
