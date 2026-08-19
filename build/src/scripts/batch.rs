use crate::{
    generators::utils::Tag,
    libfmt::{self, Builder},
    scripts::encoder::{DamageSlot, EntityKind, FormulaDbBuilder, FormulaSource},
};
use heck::ToSnakeCase;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, ops::Range, sync::LazyLock};
use tutorlolv2_wiki::MayFail;

#[derive(Debug, Deserialize, Serialize)]
pub struct FmtArgs<T> {
    pub target: String,
    pub variant: String,
    pub meta: T,
    pub default: bool,
}

#[derive(Debug, Serialize)]
pub struct FmtOutput {
    pub range: Range<usize>,
    pub builder: Builder,
    pub json: FmtArgs<Value>,
    pub delete_range: Range<usize>,
    block: String,
}

pub fn batch(src: String) -> BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>> {
    static FMT_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"#\[fmt\((\{.*?\})\)\]"#).unwrap());

    let result = FMT_RE
        .captures_iter(&src)
        .par_bridge()
        .map(|caps| {
            let full = caps.get(0).unwrap();
            let inner = caps.get(1).unwrap().as_str();

            let start_index = full.start();
            let attr_end = full.end();

            let json = serde_json::from_str::<FmtArgs<Value>>(inner).unwrap();

            let rest = &src[attr_end..];
            let start = rest.find('{').unwrap();

            let mut depth = 0;
            let mut end = 0;

            for (i, ch) in rest[start..].char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;

                        if depth == 0 {
                            end = start + i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let block = rest[..end].trim().to_string();

            let formatter = libfmt::rust_html;
            // formatter had the type: String because both functions returned HTML.
            // Now they return struct Builder with the IR
            // let formatter = if json.target == "json" {
            //     libfmt::json_html
            // } else {
            //     libfmt::rust_html
            // };
            //

            let builder = formatter(&block);

            let mut absolute_end = attr_end + end;

            if rest.get(end..end + 1) == Some(";") {
                absolute_end += 1;
            }

            FmtOutput {
                delete_range: start_index..absolute_end,
                builder,
                range: 0..0,
                json,
                block,
            }
        })
        .collect::<Vec<_>>();

    let mut map = BTreeMap::<_, BTreeMap<_, Vec<_>>>::new();

    for data in result {
        map.entry(data.json.variant.clone())
            .or_default()
            .entry(data.json.target.clone())
            .or_default()
            .push(data);
    }

    map
}

pub fn packb(
    packer: &mut FormulaDbBuilder,
    tag: Tag,
    result: &BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>>,
) -> MayFail {
    for (i, (variant, inner_map)) in result.into_iter().enumerate() {
        let Some(outputs) = inner_map.get("formula") else {
            continue;
        };

        for FmtOutput { block, .. } in outputs {
            let fields = extract_damage_fields(block);
            let id = variant.to_snake_case();
            let refs = fields
                .iter()
                .enumerate()
                .map(|(k, (field, _))| (format!("{id}_{field}").to_lowercase(), k as u8))
                .collect();

            match tag {
                Tag::Champions => {
                    let formulas =
                        fields
                            .iter()
                            .enumerate()
                            .map(|(k, (_, source))| FormulaSource {
                                local: k as u8,
                                source: source.clone(),
                            });

                    packer.push_champion(i as _, formulas, &refs)?
                }
                _ => {
                    let damage_slot = |fn_name: &str| match fn_name {
                        s if s.contains("f0") => DamageSlot::MeleeMin,
                        s if s.contains("f1") => DamageSlot::MeleeMax,
                        s if s.contains("f2") => DamageSlot::RangedMin,
                        s if s.contains("f3") => DamageSlot::RangedMax,
                        _ => unreachable!(
                            "fn_name: {fn_name:?} does not match any known damage slot"
                        ),
                    } as u8;

                    let formulas = fields.into_iter().map(|(fn_name, source)| FormulaSource {
                        local: damage_slot(&fn_name),
                        source,
                    });

                    let refs = refs
                        .into_iter()
                        .map(|(fn_name, _)| {
                            let slot = damage_slot(&fn_name);
                            (fn_name, slot)
                        })
                        .collect();

                    let entity = match tag {
                        Tag::Items => EntityKind::Item,
                        Tag::Runes => EntityKind::Rune,
                        _ => unreachable!(),
                    };

                    let pre = format!(
                        "Error for {entity:?}, block={block} for i={i}, formulas={formulas:?}, refs={refs:?}"
                    );

                    packer
                        .push_item_or_rune(entity, i as _, formulas, &refs)
                        .map_err(|e| format!("{pre}, error={e:?}"))?
                }
            }
        }
    }

    Ok(())
}

pub fn extract_damage_fields(source: &str) -> Vec<(String, String)> {
    let body = extract_outer_body(source).expect("could not find struct body");

    let fields = split_top_level(body);

    let mut result = Vec::new();

    for field in fields {
        let field = field.trim();

        if field.is_empty() {
            continue;
        }

        let Some(colon) = find_top_level_colon(field) else {
            continue;
        };

        let key = field[..colon].trim();
        let value = field[colon + 1..].trim();

        result.push((key.to_owned(), value.to_owned()));
    }

    result
}

fn extract_outer_body(source: &str) -> Option<&str> {
    let bytes = source.as_bytes();

    let mut start = None;
    let mut depth = 0usize;

    let mut in_string = false;
    let mut in_char = false;
    let mut escaped = false;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }

            i += 1;
            continue;
        }

        if in_char {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'\'' {
                in_char = false;
            }

            i += 1;
            continue;
        }

        match b {
            b'"' => in_string = true,
            b'\'' => in_char = true,

            b'{' => {
                if depth == 0 {
                    start = Some(i + 1);
                }

                depth += 1;
            }

            b'}' => {
                depth -= 1;

                if depth == 0 {
                    let start = start?;
                    return Some(&source[start..i]);
                }
            }

            _ => {}
        }

        i += 1;
    }

    None
}

fn split_top_level(source: &str) -> Vec<&str> {
    let bytes = source.as_bytes();

    let mut result = Vec::new();

    let mut start = 0;

    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    let mut in_string = false;
    let mut in_char = false;
    let mut escaped = false;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }

            i += 1;
            continue;
        }

        if in_char {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'\'' {
                in_char = false;
            }

            i += 1;
            continue;
        }

        match b {
            b'"' => in_string = true,
            b'\'' => in_char = true,

            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,

            b'[' => bracket_depth += 1,
            b']' => bracket_depth -= 1,

            b'{' => brace_depth += 1,
            b'}' => brace_depth -= 1,

            b',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                result.push(&source[start..i]);
                start = i + 1;
            }

            _ => {}
        }

        i += 1;
    }

    if start < source.len() {
        result.push(&source[start..]);
    }

    result
}

fn find_top_level_colon(source: &str) -> Option<usize> {
    let bytes = source.as_bytes();

    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    let mut in_string = false;
    let mut in_char = false;
    let mut escaped = false;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }

            i += 1;
            continue;
        }

        if in_char {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'\'' {
                in_char = false;
            }

            i += 1;
            continue;
        }

        match b {
            b'"' => in_string = true,
            b'\'' => in_char = true,

            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,

            b'[' => bracket_depth += 1,
            b']' => bracket_depth -= 1,

            b'{' => brace_depth += 1,
            b'}' => brace_depth -= 1,

            b':' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                return Some(i);
            }

            _ => {}
        }

        i += 1;
    }

    None
}
