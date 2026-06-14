use crate::{
    CPARSER, JsonRead, MayFail,
    generators::{parser::Parser, utils::Tag},
    scripts::batch::{FmtArgs, FmtOutput},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};
use tutorlolv2_types::{AbilityId, AttackType, DamageIndex};

pub fn finish_champions(target: &str, variable: &mut String, value: &mut [FmtOutput<'_>]) {
    value.sort_by(|a, b| match &a.json.meta {
        v if let Ok(ability_a) = serde_json::from_value::<AbilityId>(v.clone())
            && let Ok(ability_b) = serde_json::from_value::<AbilityId>(b.json.meta.clone()) =>
        {
            ability_a.cmp(&ability_b)
        }
        _ => a.json.target.cmp(&b.json.target),
    });

    let ranges = value
        .iter()
        .map(|FmtOutput { html_range, .. }| format!("{html_range:?}"))
        .collect::<Vec<_>>()
        .join(",");

    let push = match target {
        "formula" | "generator" => format!("{ranges},"),
        "ability" | "closure" => format!("&[{ranges},],"),
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}

pub fn finish_items_or_runes(target: &str, variable: &mut String, value: &mut [FmtOutput<'_>]) {
    value.sort_by(|a, b| match &a.json.meta {
        v if let Ok((ata, dia)) =
            serde_json::from_value::<(AttackType, DamageIndex)>(v.clone())
            && let Ok((atb, dib)) =
                serde_json::from_value::<(AttackType, DamageIndex)>(b.json.meta.clone()) =>
        {
            ata.cmp(&atb).then(dia.cmp(&dib))
        }
        _ => a.json.target.cmp(&b.json.target),
    });

    let push = match target {
        "formula" | "generator" => {
            value
                .iter()
                .map(|FmtOutput { html_range, .. }| format!("{html_range:?}"))
                .collect::<Vec<_>>()
                .join(",")
                + ","
        }
        "closure" => {
            let mut ranges: [[Range<usize>; 2]; 2] =
                core::array::from_fn(|_| core::array::from_fn(|_| 0..0));

            for FmtOutput {
                html_range,
                json: FmtArgs { meta, .. },
                ..
            } in value
            {
                let (attack_type, damage_index) =
                    serde_json::from_value::<(AttackType, DamageIndex)>(meta.clone()).unwrap();

                ranges[attack_type as usize][damage_index as usize] = html_range.clone();
            }

            format!("{ranges:?},")
        }
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}

pub fn champion_aliases() -> MayFail<Option<BTreeMap<String, Vec<String>>>> {
    let map = CPARSER.map();
    let languages =
        BTreeMap::<String, BTreeSet<String>>::from_file("internal/champion_languages.json")?;

    let alias = map
        .keys()
        .map(|champion_id| {
            (
                champion_id.clone(),
                languages[champion_id]
                    .iter()
                    .cloned()
                    .chain(
                        (champion_id == "Gnar")
                            .then_some("Mega Gnar".into())
                            .into_iter(),
                    )
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    Ok(Some(alias))
}

pub fn eval_abilities(arms: &str, _: Tag) -> String {
    format!(
        "pub const fn ability_const_eval(
            champion_id: ChampionId,
            ctx: &Ctx,
            kind: AbilityId
        ) -> f32 {{
            match champion_id {{{arms}}}
        }}"
    )
}

pub fn eval_items_or_runes(arms: &str, tag: Tag) -> String {
    format!(
        "pub const fn {ltag}_const_eval(
                {ltag}_id: {enum_name},
                ctx: &Ctx,
                attack_type: AttackType
            ) -> [f32; 2] {{
                match {ltag}_id {{{arms}}}
            }}",
        ltag = tag.singular().to_lowercase(),
        enum_name = tag.enum_name()
    )
}
