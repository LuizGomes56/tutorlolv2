use crate::scripts::batch::{Batch, FmtArgs};
use regex::{Captures, Regex};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::Range,
    sync::LazyLock,
};
use tutorlolv2_dev::{
    decl_champions::Champion, decl_items::Item, decl_runes::Rune, gen_factories::ZERO,
};
use tutorlolv2_fmt::{pascal_case, to_ssnake};
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex};

pub trait MapValueExt {
    fn riot_id(&self) -> u32;
    fn name(&self) -> &str;
}

impl MapValueExt for Champion {
    fn riot_id(&self) -> u32 {
        panic!("Champions can't have riot_id fields")
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

impl MapValueExt for Item {
    fn riot_id(&self) -> u32 {
        self.build.riot_id
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

impl MapValueExt for Rune {
    fn riot_id(&self) -> u32 {
        self.build.riot_id
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

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

pub fn get_name_phf<T: MapValueExt>(
    data: &BTreeMap<String, T>,
    tag: Tag,
    extras: Option<BTreeMap<String, Vec<String>>>,
) -> String {
    let arguments = data
        .iter()
        .map(|(key, value)| {
            let name = &value.name();

            let mut aliases = get_aliases(key, name);

            if let Some(extra) = &extras
                && let Some(extra_aliases) = extra.get(key)
            {
                extra_aliases.iter().cloned().for_each(|a| aliases.push(a));
            }

            let alias = BTreeSet::from_iter(aliases)
                .into_iter()
                .map(|v| format!("{v:?}"))
                .collect::<Vec<_>>()
                .join(" | ");

            format!("{alias} => {tag:?}Id::{key}")
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "pub static {utag}_NAME_TO_ID: phf::Map<&str, {tag:?}Id> = phf::phf_map!({arguments});",
        utag = tag.as_ref().to_uppercase(),
    )
}

pub fn get_id_enum<T: MapValueExt>(data: &BTreeMap<String, T>, tag: Tag) -> String {
    format!(
        "
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr({repr})]
        pub enum {tag:?}Id {{{variants}}}

        impl {tag:?}Id {{
            pub const VARIANTS: usize = {len};
            pub const fn debug(&self) -> &'static str {{
                match self {{{debug_arms}}}
            }}
            {riot_id_conv}
        }}
        ",
        repr = if matches!(tag, Tag::Champion | Tag::Rune,) {
            "u8"
        } else {
            "u16"
        },
        variants = data
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(","),
        len = data.len(),
        debug_arms = data
            .keys()
            .map(|name| format!("Self::{name} => {name:?},"))
            .collect::<String>(),
        riot_id_conv = if !matches!(tag, Tag::Champion) {
            format!(
                "pub const fn from_riot_id(id: u32) -> Option<Self> {{
                    match id {{ {match_arms} _ => None }}
                }}",
                match_arms = data
                    .iter()
                    .map(|(key, value)| {
                        format!("{riot_id} => Some(Self::{key}),", riot_id = value.riot_id())
                    })
                    .collect::<String>()
            )
        } else {
            String::new()
        }
    )
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

pub fn get_generator(tag: Tag, id: &str, variant: &str) -> String {
    let folder = tag.as_ref().to_lowercase();
    let mut default = false;
    let mut generator = tutorlolv2_dev::read_to_string(format!(
        "tutorlolv2_dev/src/generators/gen_{folder}s/{file_name}.rs",
        file_name = id.to_lowercase()
    ))
    .unwrap_or_else(|_| {
        default = true;
        "impl Generator {}".into()
    });

    if let Some(pos) = generator.find("impl") {
        generator.drain(..pos);
    }

    let fmt_arg = json!(FmtArgs {
        target: "generator",
        variant,
        meta: (),
        replace: Default::default(),
        default
    });

    generator.insert_str(0, &format!("#[fmt({fmt_arg})]"));
    generator
}

pub fn get_eval(
    tag: Tag,
    id: &str,
    deals_damage: &[bool; 4],
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
        "
            {tag:?}Id::{id} => {{
                match attack_type {{
                    Melee => [{melee_arms}],
                    Ranged => [{ranged_arms}]
                }}
            }},
            ",
        melee_arms = get_arms(0..2),
        ranged_arms = get_arms(2..4),
    )
}

pub struct StaticVar {
    pub attribute: &'static str,
    pub name: &'static str,
    pub vtype: &'static str,
}

pub fn get_static_vars<const N: usize, T>(
    tag: Tag,
    data: &BTreeMap<String, T>,
    array: [StaticVar; N],
) -> (String, HashMap<&'static str, String>) {
    let make = |name: &str, vtype: &str| {
        format!(
            "pub static {var}: [{vtype}; {tag:?}Id::VARIANTS] = [",
            var = name.to_uppercase()
        )
    };

    let mut cache = make(&format!("{tag:?}_CACHE"), &format!("&{tag:?}"));

    for id in data.keys() {
        let upper_id = to_ssnake(id);
        cache.push_str(&format!("&{upper_id},"));
    }

    cache.push_str("];");

    let result = array
        .into_iter()
        .map(|static_var| {
            let StaticVar {
                attribute,
                name,
                vtype,
            } = static_var;
            let variable = make(name, vtype);
            (attribute, variable)
        })
        .collect::<HashMap<_, _>>();

    (cache, result)
}

pub fn closures(
    functions: &[[&String; 2]; 2],
    melee: &[String],
    ranged: &[String],
    variant: &str,
) -> String {
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
                        0 => &melee[j],
                        1 => &ranged[j],
                        _ => unreachable!(),
                    };

                    let default = is_zero(body);

                    let formula = simplify(body);
                    let closure = if default || seen.contains(function) {
                        format!("")
                    } else {
                        seen.insert(function);
                        let formula_f32 = cast_f32(&formula);
                        let param = ctx_param(&formula_f32);

                        format!("pub const fn {function}({param}: &Ctx) -> f32 {{{formula_f32}}}",)
                    };

                    let fmt_arg = json!(FmtArgs {
                        target: "closure",
                        variant,
                        meta: (attack_type, damage_index),
                        replace: [("ctx.", "")].into(),
                        default
                    });

                    format!(
                        r#"
                        {closure}

                        #[fmt({fmt_arg})]
                        fn {function}() {{{formula}}}
                        "#
                    )
                })
                .collect::<String>()
        })
        .collect::<String>()
}

pub fn simplify(formula: &str) -> String {
    static FLOAT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d+\.\d+\b").unwrap());

    let simplified = symb_anafis::simplify(&formula.replace("ctx.", "ctx_"), &[], None)
        .map(|r| r.replace("ctx_", "ctx."))
        .unwrap_or(formula.to_string());

    FLOAT_RE
        .replace_all(&simplified, |caps: &Captures| {
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
                        let cleaned = candidate
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string();

                        return cleaned;
                    }
                }
            }

            s
        })
        .into_owned()
}

pub fn get_aliases<'a>(id: &'a str, name: &'a str) -> Vec<String> {
    let get = |s: &str| {
        [
            s.to_string(),
            s.to_lowercase(),
            s.to_uppercase(),
            pascal_case(s),
            pascal_case(s).to_lowercase(),
            pascal_case(s).to_uppercase(),
            to_ssnake(s),
            to_ssnake(s).to_lowercase(),
            to_ssnake(s).to_uppercase(),
        ]
    };

    [get(id), get(name)].concat()
}

pub fn repr_damages(melee: &[String; 2], ranged: &[String; 2], deals_damage: &[bool; 4]) -> String {
    let [melee_min, melee_max] = melee;
    let [ranged_min, ranged_max] = ranged;

    let same_min = melee_min == ranged_min;
    let same_max = melee_max == ranged_max;

    let mut parts = Vec::new();

    match *deals_damage {
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

pub fn get_fn_names(value_id: &str, melee: &[String; 2], ranged: &[String; 2]) -> [String; 4] {
    let id = tutorlolv2_fmt::to_ssnake(value_id).to_lowercase();

    let min_shared = !is_zero(&melee[0]) && melee[0] == ranged[0];
    let max_shared = !is_zero(&melee[1]) && melee[1] == ranged[1];

    [
        if is_zero(&melee[0]) {
            ZERO.into()
        } else if min_shared {
            format!("{id}_min")
        } else {
            format!("{id}_melee_min")
        },
        if is_zero(&melee[1]) {
            ZERO.into()
        } else if max_shared {
            format!("{id}_max")
        } else {
            format!("{id}_melee_max")
        },
        if is_zero(&ranged[0]) {
            ZERO.into()
        } else if min_shared {
            format!("{id}_min")
        } else {
            format!("{id}_ranged_min")
        },
        if is_zero(&ranged[1]) {
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
