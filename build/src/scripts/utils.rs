use crate::{
    generators::parser::{
        DamageRange, MapValueExt, ZERO, champions::Champion, items::Item, runes::Rune,
    },
    model::champions::WikiChampion,
    scripts::batch::{Batch, FmtArgs},
};
use regex::{Captures, Regex};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::Range,
    sync::LazyLock,
};
use tutorlolv2_fmt::{pascal_case, to_ssnake};
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex};

#[derive(Debug)]
pub enum Tag {
    Champion,
    Item,
    Rune,
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        match self {
            Tag::Item => "Item",
            Tag::Rune => "Rune",
            Tag::Champion => "Champion",
        }
    }
}

pub fn get_const_eval(data: &BTreeMap<&String, Batch>, tag: Tag) -> String {
    format!(
        "
        pub const fn {ltag}_const_eval(
            ctx: &Ctx,
            {ltag}_id: {tag:?}Id,
            attack_type: AttackType
        ) -> [f32; 2] {{
            match {ltag}_id {{{eval}}}
        }}
        ",
        ltag = tag.as_ref().to_lowercase(),
        eval = data
            .values()
            .map(|batch| batch.eval.as_str())
            .collect::<Vec<&str>>()
            .concat()
    )
}

pub fn get_eval(
    tag: Tag,
    id: &str,
    deals_damage: &[bool],
    functions: &[[&String; 2]; 2],
) -> String {
    let slice = functions.as_flattened();
    let get_arms = |range: Range<_>| {
        deals_damage[range]
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let f = match *v {
                    true => &slice[i],
                    false => ZERO,
                };
                format!("{f}(&ctx)")
            })
            .collect::<Vec<_>>()
            .join(",")
    };

    format!(
        "{tag:?}Id::{id} => {{
            match attack_type {{
                Melee => [{melee_arms}],
                Ranged => [{ranged_arms}]
            }}
        }},",
        melee_arms = get_arms(0..2),
        ranged_arms = get_arms(2..4),
    )
}

pub fn closures(
    functions: &[[&String; 2]; 2],
    melee: &DamageRange,
    ranged: &DamageRange,
    variant: &str,
) -> Vec<Vec<(String, String)>> {
    let mut seen = HashSet::new();

    functions
        .iter()
        .enumerate()
        .map(|(i, function)| {
            let attack_type = match i {
                0 => AttackType::Melee,
                1 => AttackType::Ranged,
                _ => unreachable!(),
            };

            function
                .iter()
                .enumerate()
                .map(|(j, function)| {
                    let damage_index = match j {
                        0 => DamageIndex::Min,
                        1 => DamageIndex::Max,
                        _ => unreachable!(),
                    };

                    let body = match i {
                        0 => &melee[damage_index],
                        1 => &ranged[damage_index],
                        _ => unreachable!(),
                    };

                    let default = is_zero(body);

                    let formula = simplify(body);
                    let rust = if default || seen.contains(function) {
                        format!("")
                    } else {
                        seen.insert(function);
                        let formula_f32 = cast_f32(&formula);
                        let param = ctx_param(&formula_f32);

                        format!("pub const fn {function}({param}: &Ctx) -> f32 {{{formula_f32}}}",)
                    };

                    let docs = format!(
                        "#[fmt({fmt})]
                        fn {function}() {{
                            {formula}
                        }}",
                        fmt = json!(FmtArgs {
                            target: "closure",
                            variant,
                            meta: (attack_type, damage_index),
                            replace: [("ctx.", "")].into(),
                            default
                        })
                    );

                    (rust, docs)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn simplify(formula: &str) -> String {
    static FLOAT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d+\.\d+\b").unwrap());

    static MATCH_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"match\s+[^{]+\{(?:[^{}]|\{[^{}]*\})*\}").unwrap());

    let mut matches = Vec::new();

    let protected = MATCH_RE.replace_all(formula, |caps: &Captures| {
        let idx = matches.len();

        matches.push(caps[0].to_string());

        format!("__MATCH_{idx}__")
    });

    let simplified = symb_anafis::simplify(&protected.replace("ctx.", "ctx_"), &[], None)
        .map(|r| r.replace("ctx_", "ctx."))
        .unwrap_or_else(|_| protected.into_owned());

    let mut restored = simplified;

    fn match_needs_parens(s: &str, placeholder: &str) -> bool {
        const BIN_OPS: &[char] = &['+', '-', '*', '/', '%', '&', '|', '^'];

        let Some(pos) = s.find(placeholder) else {
            return false;
        };

        let before = s[..pos].trim_end();
        let after = s[pos + placeholder.len()..].trim_start();

        let preceded_by_op = before.ends_with(BIN_OPS);
        let followed_by_op = after.starts_with(BIN_OPS);

        preceded_by_op || followed_by_op
    }

    for (i, match_block) in matches.iter().enumerate() {
        let placeholder = format!("__MATCH_{i}__");
        let replacement = if match_needs_parens(&restored, &placeholder) {
            format!("({match_block})")
        } else {
            match_block.clone()
        };
        restored = restored.replace(&placeholder, &replacement);
    }

    let result = FLOAT_RE
        .replace_all(&restored, |caps: &Captures| {
            let original = &caps[0];

            let Ok(value) = original.parse::<f64>() else {
                return original.to_string();
            };

            let mut s = format!("{value:.12}");

            while s.contains('.') && s.ends_with('0') {
                s.pop();
            }

            if s.ends_with('.') {
                s.pop();
            }

            let suspicious = s.contains("999999")
                || s.contains("000000")
                || s.contains("333333")
                || s.contains("666666");

            if suspicious {
                for precision in 2..=6 {
                    let candidate = format!("{value:.precision$}");

                    let reparsed = candidate.parse::<f64>().unwrap();

                    let diff = (value - reparsed).abs();

                    if diff < 1e-9 {
                        return candidate
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string();
                    }
                }
            }

            s
        })
        .into_owned();

    static POW_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?P<base>\([^()]+\)|[\w.]+)\s*\^\s*(?P<exp>\d+)(?:f32)?").unwrap()
    });

    POW_RE
        .replace_all(&result, |caps: &Captures| {
            let base = &caps["base"];
            let exp: usize = caps["exp"].parse().unwrap();

            match exp {
                1 => base.to_string(),
                2 => format!("({base} * {base})"),
                3 => format!("({base} * {base} * {base})"),
                _ => panic!("Unsupported exponent {exp}; maximum supported exponent is 3"),
            }
        })
        .into_owned()
}

pub fn repr_damages(melee: &DamageRange, ranged: &DamageRange) -> String {
    let melee_min = &melee.min_dmg;
    let melee_max = &melee.max_dmg;
    let ranged_min = &ranged.min_dmg;
    let ranged_max = &ranged.max_dmg;

    let same_min = melee_min == ranged_min;
    let same_max = melee_max == ranged_max;

    let mut parts = Vec::new();

    match [melee.deals_damage(), ranged.deals_damage()].concat()[..] {
        [false, false, false, false] => {}
        [true, false, false, false] => {
            parts.push(format!("melee_min_dmg: {}", simplify(melee_min)));
        }
        [false, false, true, false] => {
            parts.push(format!("ranged_min_dmg: {}", simplify(ranged_min)));
        }
        [true, false, true, false] => {
            if same_min {
                parts.push(format!("damage: {}", simplify(melee_min)));
            } else {
                parts.push(format!("melee_min_dmg: {}", simplify(melee_min)));
                parts.push(format!("ranged_min_dmg: {}", simplify(ranged_min)));
            }
        }
        [true, true, false, false] => {
            parts.push(format!("melee_min_dmg: {}", simplify(melee_min)));
            parts.push(format!("melee_max_dmg: {}", simplify(melee_max)));
        }
        [false, false, true, true] => {
            parts.push(format!("ranged_min_dmg: {}", simplify(ranged_min)));
            parts.push(format!("ranged_max_dmg: {}", simplify(ranged_max)));
        }
        [true, true, true, true] => {
            if same_min {
                parts.push(format!("min_dmg: {}", simplify(melee_min)));
            } else {
                parts.push(format!("melee_min_dmg: {}", simplify(melee_min)));
                parts.push(format!("ranged_min_dmg: {}", simplify(ranged_min)));
            }

            if same_max {
                parts.push(format!("max_dmg: {}", simplify(melee_max)));
            } else {
                parts.push(format!("melee_max_dmg: {}", simplify(melee_max)));
                parts.push(format!("ranged_max_dmg: {}", simplify(ranged_max)));
            }
        }
        _ => unreachable!("Invalid deals_damage state. Maybe max without min"),
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("{},", parts.join(", "))
    }
}

pub fn is_zero(value: &str) -> bool {
    value == ZERO || value == "0" || value == "0.0" || value == "(0.0)" || value == "(0)"
}

pub fn get_fn_names(value_id: &str, melee: &DamageRange, ranged: &DamageRange) -> [String; 4] {
    let id = to_ssnake(value_id).to_lowercase();

    let min = DamageIndex::Min;
    let max = DamageIndex::Max;

    let has_max = !is_zero(&melee[max]) || !is_zero(&ranged[max]);

    let min_shared = !is_zero(&melee[min]) && melee[min] == ranged[min];
    let max_shared = !is_zero(&melee[max]) && melee[max] == ranged[max];
    let min_suffix = if has_max { "_min" } else { "" };

    [
        if is_zero(&melee[min]) {
            ZERO.into()
        } else if min_shared {
            format!("{id}{min_suffix}")
        } else {
            format!("{id}_melee{min_suffix}")
        },
        if is_zero(&melee[max]) {
            ZERO.into()
        } else if max_shared {
            format!("{id}_max")
        } else {
            format!("{id}_melee_max")
        },
        if is_zero(&ranged[min]) {
            ZERO.into()
        } else if min_shared {
            format!("{id}{min_suffix}")
        } else {
            format!("{id}_ranged{min_suffix}")
        },
        if is_zero(&ranged[max]) {
            ZERO.into()
        } else if max_shared {
            format!("{id}_max")
        } else {
            format!("{id}_ranged_max")
        },
    ]
}

pub fn get_identifiers(identifiers: &[[BTreeSet<CtxVar>; 2]; 2]) -> String {
    format!(
        "[{}]",
        identifiers
            .iter()
            .map(|slice| format!(
                "[{}]",
                slice
                    .iter()
                    .enumerate()
                    .map(|(i, set)| {
                        let rest = if i == 0 { " as &[_]" } else { "" };
                        let vec = set.iter().collect::<Vec<_>>();
                        format!("&{vec:?}{rest}")
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub fn cast_f32(s: &str) -> String {
    static RE_CAST_F32: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

    RE_CAST_F32
        .replace_all(s, |caps: &Captures| {
            let m = caps.get(0).unwrap();

            let start = m.start();
            let end = m.end();

            let num = m.as_str();

            let before = s[..start].chars().next_back();

            let after = s[end..].chars().next();

            if matches!(before, Some('.')) || matches!(after, Some('.')) {
                return num.to_string();
            }

            let tail = &s[end..];

            let trimmed = tail.trim_start();

            if trimmed.starts_with("=>") || trimmed.starts_with("..") {
                return num.to_string();
            }

            format!("{num}f32")
        })
        .into_owned()
        .replace("match ctx.level", "match ctx.level as u8")
        .replace("match ctx.q_level", "match ctx.q_level as u8")
        .replace("match ctx.w_level", "match ctx.w_level as u8")
        .replace("match ctx.e_level", "match ctx.e_level as u8")
        .replace("match ctx.r_level", "match ctx.r_level as u8")
}

pub fn ctx_param(s: &str) -> &'static str {
    if s.contains("ctx.") { "ctx" } else { "_" }
}
