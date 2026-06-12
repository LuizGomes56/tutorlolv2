use crate::scripts::batch::{FmtArgs, FmtOutput};
use std::ops::Range;
use tutorlolv2_types::{AbilityId, AttackType, DamageIndex};

pub fn cfinish(target: &str, variable: &mut String, value: &mut [FmtOutput<'_>]) {
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

pub fn ifinish(target: &str, variable: &mut String, value: &mut [FmtOutput<'_>]) {
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
