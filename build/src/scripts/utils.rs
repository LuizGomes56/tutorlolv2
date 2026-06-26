use crate::{
    MayFail,
    generators::{
        parser::{DamageRange, ZERO, items::Item, runes::Rune},
        utils::Tag,
    },
    scripts::batch::FmtArgs,
};
use heck::{ToPascalCase, ToSnakeCase};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use regex::{Captures, Regex};
use serde_json::{Value, json};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{Debug, Write},
    ops::{Index, Range},
    sync::LazyLock,
};
use tutorlolv2_types::{AttackType, Attrs, CtxVar, DamageIndex, DamageType};

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

pub fn is_zero(value: &str) -> bool {
    value == ZERO || value == "0" || value == "0.0" || value == "(0.0)" || value == "(0)"
}

pub trait ItemOrRune {
    const TAG: Tag;

    fn id(&self) -> &str;
    fn damage_type(&self) -> DamageType;
    fn attributes(&self) -> Attrs;
}

impl ItemOrRune for Item {
    const TAG: Tag = Tag::Items;

    fn id(&self) -> &str {
        &self.data.item_id
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn attributes(&self) -> Attrs {
        self.attributes
    }
}

impl ItemOrRune for Rune {
    const TAG: Tag = Tag::Runes;

    fn id(&self) -> &str {
        &self.data.rune_id
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn attributes(&self) -> Attrs {
        Attrs::Undefined
    }
}

impl ItemOrRuneExt for Item {}
impl ItemOrRuneExt for Rune {}

pub trait ItemOrRuneExt: Index<AttackType, Output = DamageRange> + ItemOrRune + Debug {
    fn identifiers(&self) -> Vec<CtxVar> {
        let mut set = BTreeSet::new();

        let mut add_set = |attack_type: AttackType, damage_index| {
            for element in get_identifiers(&self[attack_type][damage_index], self.damage_type()) {
                set.insert(element);
            }
        };

        add_set(AttackType::Melee, DamageIndex::Min);
        add_set(AttackType::Melee, DamageIndex::Max);
        add_set(AttackType::Ranged, DamageIndex::Min);
        add_set(AttackType::Ranged, DamageIndex::Max);

        set.into_iter().collect()
    }

    fn deals_damage(&self) -> Vec<bool> {
        [
            self[AttackType::Melee].deals_damage(),
            self[AttackType::Ranged].deals_damage(),
        ]
        .concat()
    }

    fn repr_metadata(&self) -> String {
        format!(
            "TypeMetadata {{
                kind: {enum_name}::{id},
                damage_type: {damage_type:?},
                attributes: {attributes:?},
            }}",
            enum_name = Self::TAG.enum_name(),
            id = self.id(),
            damage_type = self.damage_type(),
            attributes = self.attributes(),
        )
    }

    fn function_names(&self) -> String {
        let functions = self.functions();
        let melee_fns = functions[0..2].join(",");
        let ranged_fns = functions[2..4].join(",");

        format!("melee: [{melee_fns}], ranged: [{ranged_fns}],")
    }

    fn eval(&self) -> String {
        let id = self.id();
        let module = id.to_snake_case();

        let deals_damage = [
            self[AttackType::Melee].deals_damage(),
            self[AttackType::Ranged].deals_damage(),
        ]
        .concat();

        let functions = self.functions();

        let get_arms = |range: Range<_>| {
            deals_damage[range]
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let f = match *v {
                        true => &format!("{module}::{}", functions[i]),
                        false => ZERO,
                    };
                    format!("{f}(&ctx)")
                })
                .collect::<Vec<_>>()
                .join(",")
        };

        format!(
            "{enum_name}::{id} => {{
                match attack_type {{
                    Melee => [{melee_arms}],
                    Ranged => [{ranged_arms}]
                }}
            }},",
            enum_name = Self::TAG.enum_name(),
            melee_arms = get_arms(0..2),
            ranged_arms = get_arms(2..4),
        )
    }

    fn build_aliases(&self) -> [Option<(usize, String)>; 4] {
        let melee = &self[AttackType::Melee];
        let ranged = &self[AttackType::Ranged];

        let raw = [
            &melee.min_dmg,
            &melee.max_dmg,
            &ranged.min_dmg,
            &ranged.max_dmg,
        ];

        let all_vars = self.identifiers();

        let mut aliases = [None, None, None, None];

        for i in 0..4 {
            if is_zero(raw[i]) {
                continue;
            }

            for j in 0..i {
                if is_zero(raw[j]) {
                    continue;
                }

                let ratio_sym = simplify(&format!("({}) / ({})", raw[i], raw[j]));
                let ratio = if !ratio_sym.contains("ctx") {
                    ratio_sym
                } else {
                    let Some(k) = probe_ratio(raw[j], raw[i], &all_vars) else {
                        continue;
                    };

                    k
                };

                aliases[i] = Some((j, ratio));
                break;
            }
        }

        aliases
    }

    fn closures(&self) -> MayFail<(String, String)> {
        let variant = self.id();
        let functions = self.functions();
        let aliases = self.build_aliases();

        let mut seen = HashSet::new();

        let mut rust = String::new();
        let mut docs = String::new();

        for i in 0..4 {
            let attack_type = unsafe { AttackType::from_u8_unchecked(i / 2) };
            let damage_index = unsafe { DamageIndex::from_u8_unchecked(i % 2) };

            let body = &self[attack_type][damage_index];
            let f = &functions[i as usize];

            let default = is_zero(body);
            let formula = simplify(body);

            write!(
                docs,
                "#[fmt({fmt})]
                fn {f}() {{{formula}}}",
                fmt = json!(FmtArgs {
                    target: "closure".into(),
                    variant: variant.into(),
                    meta: (attack_type, damage_index),
                    replace: [("ctx.", ""), ("(ctx)", "__fn__")]
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .into(),
                    default
                })
            )?;

            let formula_f32 = if default {
                continue;
            } else if let Some((base_idx, ref ratio)) = aliases[i as usize] {
                let base_fn = &functions[base_idx];

                cast_f32(&format!("{ratio} * {base_fn}(ctx)"))
            } else {
                cast_f32(&formula)
            };

            if !seen.contains(f) {
                seen.insert(f);

                let param = ctx_param(&formula_f32);

                write!(
                    rust,
                    "pub const fn {f}({param}: &Ctx) -> f32 {{{formula_f32}}}"
                )?;
            }
        }

        Ok((rust, docs))
    }

    fn repr_damages(&self) -> String {
        let melee = &self[AttackType::Melee];
        let ranged = &self[AttackType::Ranged];

        let aliases = self.build_aliases();

        let raw = [
            &melee.min_dmg,
            &melee.max_dmg,
            &ranged.min_dmg,
            &ranged.max_dmg,
        ];

        let labels = [
            "melee_min_dmg",
            "melee_max_dmg",
            "ranged_min_dmg",
            "ranged_max_dmg",
        ];

        let mut parts = Vec::new();

        let deals_damage = [melee.deals_damage(), ranged.deals_damage()].concat();

        for i in 0..4 {
            if !deals_damage[i] {
                continue;
            }

            let value = if let Some((base_idx, ref ratio)) = aliases[i] {
                let postfix = labels[base_idx];
                let function = format_args!("{postfix}__fn__");

                if ratio == "1" {
                    format!("{function}")
                } else {
                    format!("{ratio} * {function}")
                }
            } else {
                simplify(raw[i])
            };

            parts.push(format!("{}: {value}", labels[i]));
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!(
                "damage_type: {:?}, {}",
                self.damage_type(),
                parts.join(", ")
            )
        }
    }

    fn formula_fmt(&self) -> Value {
        json!(FmtArgs {
            target: "formula".into(),
            variant: self.id().into(),
            meta: (),
            replace: [
                (": X = X", " ="),
                ("TypeMetadata ", ""),
                (&format!("{}::", Self::TAG.enum_name()), ""),
                ("ctx.", ""),
                ("(ctx)", "__fn__")
            ]
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .into(),
            default: false
        })
    }

    fn functions(&self) -> [String; 4] {
        let value_id = self.id();
        let melee = &self[AttackType::Melee];
        let ranged = &self[AttackType::Ranged];

        let id = value_id.to_snake_case();

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
}

pub fn get_identifiers(damage: &str, damage_type: DamageType) -> impl Iterator<Item = CtxVar> + '_ {
    static RE_IDENTS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"ctx\.([a-z_][a-z0-9_]*)").unwrap());

    RE_IDENTS
        .captures_iter(&damage)
        .filter_map(|cap| cap[1].to_pascal_case().parse().ok())
        .chain(match damage_type {
            DamageType::Physical => Some(CtxVar::PhysicalMultiplier),
            DamageType::Magic => Some(CtxVar::MagicMultiplier),
            _ => None,
        })
}

pub fn fit_str(c: &str) -> String {
    const CHUNK: usize = 30;
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

pub fn cast_f32(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < s.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;

            while i < s.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }

            let num = &s[start..i];

            let prev_ok =
                start == 0 || !matches!(bytes[start - 1], b'a'..=b'z' | b'A'..=b'Z' | b'_');
            let next_ok = i >= s.len() || !matches!(bytes[i], b'a'..=b'z' | b'A'..=b'Z' | b'_');

            let tail = s[i..].trim_start();
            let is_pattern = tail.starts_with("=>") || tail.starts_with("..");

            out.push_str(num);
            if prev_ok && next_ok && !is_pattern {
                out.push_str("f32");
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }

    out.replace("match ctx.level", "match ctx.level as u8")
        .replace("match ctx.q_level", "match ctx.q_level as u8")
        .replace("match ctx.w_level", "match ctx.w_level as u8")
        .replace("match ctx.e_level", "match ctx.e_level as u8")
        .replace("match ctx.r_level", "match ctx.r_level as u8")
}

pub fn ctx_param(s: &str) -> &'static str {
    if s.contains("ctx") { "ctx" } else { "_" }
}

pub struct StaticVar<'a> {
    pub name: String,
    pub attribute: &'a str,
    pub vtype: &'a str,
}

pub fn static_vars<'a, const N: usize>(
    tag: Tag,
    array: [StaticVar<'a>; N],
) -> HashMap<&'a str, String> {
    array
        .into_iter()
        .map(|static_var| {
            let StaticVar {
                attribute,
                name,
                vtype,
            } = static_var;

            (attribute, variable(tag, &name, vtype))
        })
        .collect::<HashMap<_, _>>()
}

pub fn variable(tag: Tag, var: &str, vtype: &str) -> String {
    let enum_name = tag.enum_name();
    format!(
        "pub static {var}: [{vtype}; {enum_name}::VARIANTS] = [",
        var = var.to_uppercase()
    )
}

pub fn probe_ratio(min_expr: &str, max_expr: &str, all_vars: &[CtxVar]) -> Option<String> {
    const N_PROBES: usize = 20;
    const MIN_VALID: usize = 8;
    const TOL: f64 = 1e-9;

    let mut rng = SmallRng::seed_from_u64(0x4c4f_4c42_5544_494c);
    let mut k_candidate: Option<f64> = None;
    let mut valid = 0usize;

    for _ in 0..N_PROBES {
        let mut min_s = min_expr.to_owned();
        let mut max_s = max_expr.to_owned();

        for var in all_vars {
            let v = rng.random_range(2..=998).to_string();
            min_s = min_s.replace(var.as_var(), &v);
            max_s = max_s.replace(var.as_var(), &v);
        }

        let Ok(min_v) = simplify(&min_s).trim().parse::<f64>() else {
            continue;
        };

        let Ok(max_v) = simplify(&max_s).trim().parse::<f64>() else {
            continue;
        };

        if min_v.abs() < 1e-10 {
            continue;
        }

        let ratio = max_v / min_v;
        valid += 1;

        match k_candidate {
            None => k_candidate = Some(ratio),
            Some(k) => {
                if (ratio - k).abs() / (1.0 + k.abs()) > TOL {
                    return None;
                }
            }
        }
    }

    (valid >= MIN_VALID)
        .then(|| k_candidate)
        .flatten()
        .map(|k| {
            let r = k.round();
            if (k - r).abs() < 1e-9 {
                return format!("{}", r as i64);
            }

            format!("{k:.10}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_owned()
        })
}
