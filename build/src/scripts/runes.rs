use crate::{
    MayFail,
    generators::parser::{get_identifiers, runes::Rune},
    model::runes::WikiRune,
    scripts::{
        batch::FmtArgs,
        utils::{
            Tag, closures, get_eval, get_fn_names, get_identifiers as repr_identifiers,
            repr_damages,
        },
    },
};
use serde_json::json;
use std::{collections::BTreeSet, fmt::Write, path::PathBuf};
use tutorlolv2_fmt::to_ssnake;
use tutorlolv2_types::{AttackType, DamageIndex};

impl Rune {
    pub fn build(&self, out: PathBuf) -> MayFail {
        let identifiers = core::array::from_fn(|i| {
            let attack_type = match i {
                0 => AttackType::Melee,
                1 => AttackType::Ranged,
                _ => unreachable!(),
            };

            core::array::from_fn(|j| {
                let damage_index = match j {
                    0 => DamageIndex::Min,
                    1 => DamageIndex::Max,
                    _ => unreachable!(),
                };

                get_identifiers(&self[attack_type][damage_index], self.damage_type)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
        });

        let Self {
            riot_id,
            data:
                WikiRune {
                    name,
                    rune_id,
                    custom,
                    ..
                },
            damage_type,
            ranged,
            melee,
        } = self;

        let mut rust = String::new();
        let mut docs = String::new();

        let upper_id = to_ssnake(&rune_id);
        let damage = {
            let dmg = repr_damages(&melee, &ranged);
            if !dmg.is_empty() {
                format!("{dmg} damage_type: {damage_type:?}")
            } else {
                dmg
            }
        };

        let fns = get_fn_names(&rune_id, &melee, &ranged);
        let functions = [[&fns[0], &fns[1]], [&fns[2], &fns[3]]];
        let deals_damage = [melee.deals_damage(), ranged.deals_damage()].concat();

        write!(
            rust,
            "pub static {upper_id}: X = X {{
                name: {name:?},
                metadata: {metadata},
                {fn_names}
                deals_damage: {deals_damage:?},
                riot_id: {riot_id},
                identifiers: {identifiers},
                custom: {custom}
            }};",
            fn_names = {
                let melee_fns = fns[0..2].join(",");
                let ranged_fns = fns[2..4].join(",");

                format!("melee: [{melee_fns}], ranged: [{ranged_fns}],")
            },
            identifiers = repr_identifiers(&identifiers),
            metadata = format_args!(
                "TypeMetadata {{
                    kind: RuneId::{rune_id},
                    damage_type: {damage_type:?},
                    attributes: Undefined,
                }}"
            )
        )?;

        write!(
            docs,
            "#[fmt({fmt})]
            static {upper_id}: X = X {{
                name: {name:?}, {damage}
            }};",
            fmt = json!(FmtArgs {
                target: "formula",
                variant: rune_id,
                meta: (),
                replace: [
                    (": X = X", " ="),
                    ("TypeMetadata ", ""),
                    ("RuneId::", ""),
                    ("ctx.", ""),
                ]
                .into(),
                default: false
            })
        )?;

        for vec in closures(&functions, &melee, &ranged, &rune_id) {
            for (code, doc) in &vec {
                rust.push_str(code);
                docs.push_str(doc);
            }
        }

        let eval = get_eval(Tag::Rune, &rune_id, &deals_damage, &functions);

        crate::write(&out.with_extension("rs"), rust)?;
        crate::write(&out.with_extension("w48"), docs)?;
        crate::write(&out.with_extension("eval"), eval)
    }
}
